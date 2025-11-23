import * as vscode from 'vscode';
import * as cp from 'child_process';
import * as path from 'path';
import { promisify } from 'util';

const execFile = promisify(cp.execFile);

interface TelemetryRecommendation {
    file: string;
    line: number;
    function_name: string;
    pattern: string;
    suggestion: string;
    code_snippet: string;
    priority: 'High' | 'Medium' | 'Low';
}

interface AnalyzeResult {
    recommendations: TelemetryRecommendation[];
}

export function activate(context: vscode.ExtensionContext) {
    console.log('telemetry-kit extension activated');

    const diagnosticCollection = vscode.languages.createDiagnosticCollection('telemetry-kit');
    context.subscriptions.push(diagnosticCollection);

    // Command: Analyze Project
    const analyzeCommand = vscode.commands.registerCommand('telemetry-kit.analyzeProject', async () => {
        await analyzeProject(diagnosticCollection);
    });

    // Command: Add Telemetry Here
    const addTelemetryCommand = vscode.commands.registerCommand('telemetry-kit.addTelemetry', async () => {
        await addTelemetryAtCursor();
    });

    // Command: Initialize Project
    const initCommand = vscode.commands.registerCommand('telemetry-kit.initProject', async () => {
        await initializeProject();
    });

    // Code Action Provider for quick fixes
    const codeActionProvider = vscode.languages.registerCodeActionsProvider(
        { language: 'rust' },
        new TelemetryCodeActionProvider(),
        {
            providedCodeActionKinds: [vscode.CodeActionKind.QuickFix]
        }
    );

    context.subscriptions.push(
        analyzeCommand,
        addTelemetryCommand,
        initCommand,
        codeActionProvider
    );

    // Auto-analyze on file save if enabled
    const config = vscode.workspace.getConfiguration('telemetry-kit');
    if (config.get('autoSuggest')) {
        const onSave = vscode.workspace.onDidSaveTextDocument((document) => {
            if (document.languageId === 'rust') {
                analyzeSingleFile(document, diagnosticCollection);
            }
        });
        context.subscriptions.push(onSave);
    }

    // Show welcome message on first activation
    const hasShownWelcome = context.globalState.get('telemetry-kit.hasShownWelcome', false);
    if (!hasShownWelcome) {
        showWelcomeMessage();
        context.globalState.update('telemetry-kit.hasShownWelcome', true);
    }
}

async function analyzeProject(diagnosticCollection: vscode.DiagnosticCollection) {
    const workspaceFolders = vscode.workspace.workspaceFolders;
    if (!workspaceFolders) {
        vscode.window.showErrorMessage('No workspace folder open');
        return;
    }

    const workspaceRoot = workspaceFolders[0].uri.fsPath;
    const config = vscode.workspace.getConfiguration('telemetry-kit');
    const cliPath = config.get('cliPath', 'telemetry-kit');

    try {
        await vscode.window.withProgress(
            {
                location: vscode.ProgressLocation.Notification,
                title: 'Analyzing project for telemetry opportunities...',
                cancellable: false
            },
            async () => {
                const { stdout } = await execFile(cliPath, ['analyze', '--format', 'json', workspaceRoot]);
                const result: AnalyzeResult = JSON.parse(stdout);

                // Clear existing diagnostics
                diagnosticCollection.clear();

                // Group recommendations by file
                const diagnosticsByFile = new Map<string, vscode.Diagnostic[]>();

                for (const rec of result.recommendations) {
                    const filePath = path.isAbsolute(rec.file) ? rec.file : path.join(workspaceRoot, rec.file);
                    const uri = vscode.Uri.file(filePath);

                    if (!diagnosticsByFile.has(filePath)) {
                        diagnosticsByFile.set(filePath, []);
                    }

                    const diagnostic = createDiagnostic(rec);
                    diagnosticsByFile.get(filePath)!.push(diagnostic);
                }

                // Set diagnostics for each file
                for (const [filePath, diagnostics] of diagnosticsByFile) {
                    diagnosticCollection.set(vscode.Uri.file(filePath), diagnostics);
                }

                const totalRecs = result.recommendations.length;
                vscode.window.showInformationMessage(
                    `Found ${totalRecs} telemetry opportunity${totalRecs !== 1 ? 's' : ''}`
                );
            }
        );
    } catch (error) {
        const err = error as Error;
        if (err.message.includes('ENOENT')) {
            vscode.window.showErrorMessage(
                'telemetry-kit CLI not found. Install it with: cargo install telemetry-kit-cli'
            );
        } else {
            vscode.window.showErrorMessage(`Analysis failed: ${err.message}`);
        }
    }
}

async function analyzeSingleFile(document: vscode.TextDocument, diagnosticCollection: vscode.DiagnosticCollection) {
    const config = vscode.workspace.getConfiguration('telemetry-kit');
    const cliPath = config.get('cliPath', 'telemetry-kit');

    try {
        const filePath = document.uri.fsPath;
        const { stdout } = await execFile(cliPath, ['analyze', '--format', 'json', filePath]);
        const result: AnalyzeResult = JSON.parse(stdout);

        const diagnostics = result.recommendations.map(rec => createDiagnostic(rec));
        diagnosticCollection.set(document.uri, diagnostics);
    } catch (error) {
        // Silently fail for auto-analysis
        console.error('Auto-analysis failed:', error);
    }
}

function createDiagnostic(rec: TelemetryRecommendation): vscode.Diagnostic {
    const config = vscode.workspace.getConfiguration('telemetry-kit');
    const severityConfig = config.get('diagnosticSeverity', 'Hint');

    const severityMap: Record<string, vscode.DiagnosticSeverity> = {
        'Error': vscode.DiagnosticSeverity.Error,
        'Warning': vscode.DiagnosticSeverity.Warning,
        'Information': vscode.DiagnosticSeverity.Information,
        'Hint': vscode.DiagnosticSeverity.Hint
    };

    const severity = severityMap[severityConfig] || vscode.DiagnosticSeverity.Hint;
    const line = rec.line - 1; // VS Code is 0-indexed
    const range = new vscode.Range(line, 0, line, 100);

    const diagnostic = new vscode.Diagnostic(
        range,
        `[${rec.priority}] ${rec.suggestion}`,
        severity
    );

    diagnostic.code = 'telemetry-suggestion';
    diagnostic.source = 'telemetry-kit';
    diagnostic.relatedInformation = [
        new vscode.DiagnosticRelatedInformation(
            new vscode.Location(vscode.Uri.file(rec.file), range),
            `Pattern: ${rec.pattern}\n\nExample:\n${rec.code_snippet}`
        )
    ];

    return diagnostic;
}

async function addTelemetryAtCursor() {
    const editor = vscode.window.activeTextEditor;
    if (!editor) {
        return;
    }

    const position = editor.selection.active;
    const document = editor.document;
    const line = document.lineAt(position.line);

    // Detect context
    const lineText = line.text.trim();
    let snippet: string;

    if (lineText.includes('fn main()') || lineText.includes('async fn main()')) {
        snippet = 'telemetry.track_event("${1:app_start}", |e| e.success(true)).await?;';
    } else if (lineText.includes('fn ') || lineText.includes('async fn ')) {
        snippet = 'telemetry.track_command("${1:command_name}", |e| {\n\te.success(${2:true}).duration_ms(${3:0})\n}).await?;';
    } else {
        snippet = 'telemetry.track_event("${1:event_name}", |e| e.success(${2:true})).await?;';
    }

    editor.insertSnippet(new vscode.SnippetString(snippet));
}

async function initializeProject() {
    const workspaceFolders = vscode.workspace.workspaceFolders;
    if (!workspaceFolders) {
        vscode.window.showErrorMessage('No workspace folder open');
        return;
    }

    const workspaceRoot = workspaceFolders[0].uri.fsPath;
    const config = vscode.workspace.getConfiguration('telemetry-kit');
    const cliPath = config.get('cliPath', 'telemetry-kit');

    const terminal = vscode.window.createTerminal({
        name: 'telemetry-kit init',
        cwd: workspaceRoot
    });

    terminal.show();
    terminal.sendText(`${cliPath} init`);
}

function showWelcomeMessage() {
    vscode.window.showInformationMessage(
        'Welcome to telemetry-kit! Run "telemetry-kit: Analyze Project" to find instrumentation opportunities.',
        'Analyze Now',
        'Learn More'
    ).then(selection => {
        if (selection === 'Analyze Now') {
            vscode.commands.executeCommand('telemetry-kit.analyzeProject');
        } else if (selection === 'Learn More') {
            vscode.env.openExternal(vscode.Uri.parse('https://github.com/ibrahimcesar/telemetry-kit'));
        }
    });
}

class TelemetryCodeActionProvider implements vscode.CodeActionProvider {
    provideCodeActions(
        document: vscode.TextDocument,
        range: vscode.Range | vscode.Selection,
        context: vscode.CodeActionContext
    ): vscode.CodeAction[] {
        const actions: vscode.CodeAction[] = [];

        for (const diagnostic of context.diagnostics) {
            if (diagnostic.source === 'telemetry-kit') {
                const action = new vscode.CodeAction(
                    'Add telemetry tracking',
                    vscode.CodeActionKind.QuickFix
                );

                action.diagnostics = [diagnostic];
                action.command = {
                    command: 'telemetry-kit.addTelemetry',
                    title: 'Add telemetry tracking'
                };

                actions.push(action);
            }
        }

        return actions;
    }
}

export function deactivate() {
    console.log('telemetry-kit extension deactivated');
}
