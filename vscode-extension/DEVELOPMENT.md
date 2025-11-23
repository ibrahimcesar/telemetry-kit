# telemetry-kit VS Code Extension - Development Guide

## Setup

### Prerequisites

- Node.js 20+
- VS Code 1.85+
- TypeScript 5.3+

### Installation

```bash
cd vscode-extension
npm install
```

## Development Workflow

### 1. Compile TypeScript

```bash
npm run compile
```

Or watch mode for development:

```bash
npm run watch
```

### 2. Run Extension in Development Mode

1. Open `vscode-extension` folder in VS Code
2. Press `F5` to launch Extension Development Host
3. A new VS Code window opens with the extension loaded
4. Open a Rust project to test

### 3. Testing the Extension

#### Test Analysis Command

1. Open a Rust project with `Cargo.toml`
2. Ensure telemetry-kit CLI is installed: `cargo install telemetry-kit-cli`
3. Run: `Cmd+Shift+P` → "telemetry-kit: Analyze Project"
4. Check Problems panel for suggestions

#### Test Snippets

1. Open a `.rs` file
2. Type `tk-init` and press `Tab`
3. Verify snippet expansion
4. Test all 17+ snippets

#### Test Quick Fixes

1. Run analysis to generate diagnostics
2. Click on a suggestion in Problems panel
3. Click the light bulb (💡) icon
4. Select "Add telemetry tracking"
5. Verify code insertion

#### Test Context Menu

1. Right-click in a Rust file
2. Select "telemetry-kit: Add Telemetry Here"
3. Verify appropriate snippet is inserted

## Building for Distribution

### Create VSIX Package

```bash
npm run package
```

This creates `telemetry-kit-0.1.0.vsix`

### Install Locally

```bash
code --install-extension telemetry-kit-0.1.0.vsix
```

### Test in Clean Environment

```bash
code --disable-extensions --install-extension telemetry-kit-0.1.0.vsix
```

## Publishing

### First-Time Setup

1. **Create Publisher Account:**
   - Go to https://marketplace.visualstudio.com/manage
   - Create organization "telemetry-kit"

2. **Generate Personal Access Token:**
   - Go to https://dev.azure.com/[your-org]/_usersSettings/tokens
   - Create token with **Marketplace** > **Manage** scope
   - Save token securely

3. **Login to vsce:**
   ```bash
   npx vsce login telemetry-kit
   ```

### Publishing Steps

1. **Update version in package.json:**
   ```json
   {
     "version": "0.1.1"
   }
   ```

2. **Update CHANGELOG.md:**
   ```markdown
   ## [0.1.1] - 2025-01-24
   ### Fixed
   - Bug fixes...
   ```

3. **Publish:**
   ```bash
   npx vsce publish
   ```

Or publish specific version:

```bash
npx vsce publish minor  # 0.1.0 → 0.2.0
npx vsce publish patch  # 0.1.0 → 0.1.1
npx vsce publish major  # 0.1.0 → 1.0.0
```

## Architecture

### File Structure

```
vscode-extension/
├── src/
│   └── extension.ts          # Main extension logic
├── snippets/
│   └── telemetry.json         # 17+ code snippets
├── package.json               # Extension manifest
├── tsconfig.json              # TypeScript configuration
├── README.md                  # User documentation
├── CHANGELOG.md               # Version history
└── DEVELOPMENT.md             # This file
```

### Key Components

#### extension.ts

Main extension file with:

- **`activate()`** - Extension activation
- **`analyzeProject()`** - Run CLI analysis
- **`analyzeSingleFile()`** - Auto-analyze on save
- **`createDiagnostic()`** - Create VS Code diagnostics
- **`TelemetryCodeActionProvider`** - Quick fix provider

#### Snippets

17+ snippets in `snippets/telemetry.json`:

- Initialization patterns
- Event tracking
- Privacy controls
- Framework integration

#### Commands

3 commands registered:

1. `telemetry-kit.analyzeProject` - Full project analysis
2. `telemetry-kit.addTelemetry` - Context-aware insertion
3. `telemetry-kit.initProject` - Interactive setup

## Testing Scenarios

### Scenario 1: New Rust Project

1. Create new Rust project:
   ```bash
   cargo new test-project
   cd test-project
   code .
   ```

2. Run analysis (should find `main` function)
3. Use `tk-init` snippet
4. Add telemetry tracking
5. Verify no errors

### Scenario 2: Existing Project

1. Open existing Rust project
2. Run analysis
3. Review High priority suggestions
4. Use quick fixes
5. Verify code compiles

### Scenario 3: Web Service (Axum)

1. Create Axum project
2. Run analysis
3. Use `tk-axum` snippet
4. Add telemetry to handlers
5. Test tracking

## Debugging

### Enable Debug Logging

1. Open Output panel: View → Output
2. Select "telemetry-kit" from dropdown
3. View extension logs

### Common Issues

#### CLI Not Found

```typescript
// In extension.ts, check:
const cliPath = config.get('cliPath', 'telemetry-kit');
// Verify this path exists
```

#### Diagnostics Not Appearing

1. Check if `analyze` command returns JSON
2. Verify file paths are absolute
3. Check diagnostic severity setting

#### Snippets Not Working

1. Verify `snippets/telemetry.json` is valid JSON
2. Check prefix matches what you're typing
3. Ensure file is in `package.json` → `contributes.snippets`

## Code Quality

### Linting

```bash
npm run lint
```

### Type Checking

```bash
tsc --noEmit
```

### Format Code

```bash
npx prettier --write src/**/*.ts
```

## Performance

### Optimization Tips

1. **Debounce auto-analysis:**
   - Don't analyze on every keystroke
   - Only on save

2. **Limit file scanning:**
   - Skip `target/` directory
   - Only analyze `.rs` files

3. **Cache results:**
   - Store analysis results
   - Invalidate on file changes

## Release Checklist

Before releasing a new version:

- [ ] Update version in `package.json`
- [ ] Update `CHANGELOG.md`
- [ ] Test all commands
- [ ] Test all snippets
- [ ] Test quick fixes
- [ ] Test configuration options
- [ ] Build VSIX: `npm run package`
- [ ] Test VSIX locally
- [ ] Publish: `npx vsce publish`
- [ ] Create GitHub release
- [ ] Update documentation

## Future Enhancements

### Planned Features

1. **CodeLens Integration:**
   ```typescript
   // Show inline metrics
   class TelemetryCodeLensProvider implements vscode.CodeLensProvider {
     provideCodeLenses(document) {
       // Show "Add telemetry" lens above functions
     }
   }
   ```

2. **Hover Information:**
   ```typescript
   // Show documentation on hover
   class TelemetryHoverProvider implements vscode.HoverProvider {
     provideHover(document, position) {
       // Show telemetry pattern docs
     }
   }
   ```

3. **Tree View:**
   ```typescript
   // Show telemetry coverage in sidebar
   class TelemetryCoverageProvider implements vscode.TreeDataProvider {
     getTreeItem(element) {
       // Show instrumented vs uninstrumented functions
     }
   }
   ```

4. **Webview Dashboard:**
   ```typescript
   // Show telemetry stats in webview
   const panel = vscode.window.createWebviewPanel(
     'telemetryDashboard',
     'Telemetry Dashboard',
     vscode.ViewColumn.One,
     {}
   );
   ```

## Contributing

1. Fork the repository
2. Create feature branch
3. Make changes
4. Test thoroughly
5. Submit pull request

## Resources

- [VS Code Extension API](https://code.visualstudio.com/api)
- [Extension Guidelines](https://code.visualstudio.com/api/references/extension-guidelines)
- [Publishing Extensions](https://code.visualstudio.com/api/working-with-extensions/publishing-extension)
- [vsce Documentation](https://github.com/microsoft/vscode-vsce)

## Support

- GitHub Issues: https://github.com/ibrahimcesar/telemetry-kit/issues
- Documentation: https://github.com/ibrahimcesar/telemetry-kit#readme
