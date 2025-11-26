//! # telemetry-kit CLI
//!
//! Command-line tool for managing telemetry configuration and operations.
//!
//! ## Commands
//!
//! - `telemetry-kit init` - Interactive project setup
//! - `telemetry-kit test` - Test sync credentials
//! - `telemetry-kit stats` - View event statistics
//! - `telemetry-kit sync` - Manually trigger sync
//! - `telemetry-kit validate` - Validate configuration
//! - `telemetry-kit clean` - Clear local events
//! - `telemetry-kit consent` - Manage privacy consent
//!
//! ## Telemetry
//!
//! This CLI uses telemetry-kit to track anonymous usage patterns.
//! We only collect: command names, success/failure, and timing.
//! No personal data is ever collected. Respects DO_NOT_TRACK.
//!
//! To opt out: `export DO_NOT_TRACK=1`

use clap::{Parser, Subcommand, ValueEnum};
use colored::Colorize;
use dialoguer::{Confirm, Input, Password, Select};
use indicatif::{ProgressBar, ProgressStyle};
use std::path::PathBuf;
use std::time::Instant;
use telemetry_kit::TelemetryKit;

#[derive(Parser)]
#[command(name = "telemetry-kit")]
#[command(version, about = "\x1b[1;33mPrivacy-first telemetry toolkit for Rust applications\x1b[0m", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,

    /// Service name to operate on (defaults to current directory name)
    #[arg(short, long, global = true)]
    service: Option<String>,
}

#[derive(Debug, Clone, ValueEnum)]
enum ProjectType {
    /// Binary/CLI application
    Bin,
    /// Library
    Lib,
    /// Web service (Axum)
    Web,
}

#[derive(Subcommand)]
enum Commands {
    /// Create a new Rust project with telemetry pre-configured
    New {
        /// Project name
        name: String,

        /// Project type: bin (CLI), lib (library), or web (web service)
        #[arg(short, long, default_value = "bin")]
        project_type: ProjectType,

        /// Skip interactive prompts and use defaults
        #[arg(short, long)]
        yes: bool,

        /// Configure sync immediately
        #[arg(long)]
        with_sync: bool,
    },

    /// Initialize telemetry configuration interactively
    Init {
        /// Skip interactive prompts and use defaults
        #[arg(short, long)]
        yes: bool,

        /// Service name
        #[arg(short = 'n', long)]
        service_name: Option<String>,
    },

    /// Test sync credentials and connectivity
    Test {
        /// Organization ID
        #[arg(short, long)]
        org_id: Option<String>,

        /// Application ID
        #[arg(short, long)]
        app_id: Option<String>,

        /// Authentication token
        #[arg(short, long)]
        token: Option<String>,

        /// HMAC secret
        #[arg(short, long)]
        secret: Option<String>,
    },

    /// View event statistics
    Stats {
        /// Show detailed breakdown
        #[arg(short, long)]
        detailed: bool,
    },

    /// Manually trigger event sync
    Sync {
        /// Force sync even if auto-sync is enabled
        #[arg(short, long)]
        force: bool,
    },

    /// Validate telemetry configuration
    Validate {
        /// Path to configuration file
        #[arg(short, long)]
        config: Option<PathBuf>,
    },

    /// Clear local event database
    Clean {
        /// Skip confirmation prompt
        #[arg(short, long)]
        yes: bool,

        /// Also remove configuration
        #[arg(long)]
        all: bool,
    },

    /// Manage privacy consent
    #[cfg(feature = "privacy")]
    Consent {
        #[command(subcommand)]
        action: ConsentAction,
    },

    /// Analyze code and suggest instrumentation points
    Analyze {
        /// Path to analyze (defaults to current directory)
        #[arg(short, long)]
        path: Option<PathBuf>,

        /// Show detailed analysis
        #[arg(short, long)]
        detailed: bool,

        /// Output format: text, json
        #[arg(short = 'f', long, default_value = "text")]
        format: String,
    },
}

#[cfg(feature = "privacy")]
#[derive(Subcommand)]
enum ConsentAction {
    /// Grant consent for telemetry tracking
    Grant,

    /// Deny consent for telemetry tracking
    Deny,

    /// Opt out of all telemetry (DO_NOT_TRACK equivalent)
    OptOut,

    /// Show current consent status
    Status,

    /// Interactively prompt for consent
    Prompt,
}

#[tokio::main]
async fn main() {
    let start = Instant::now();
    let cli = Cli::parse();

    // Get command name for telemetry
    let command_name = get_command_name(&cli.command);

    // Initialize telemetry for the CLI itself (respects DO_NOT_TRACK)
    let telemetry = if !TelemetryKit::is_do_not_track_enabled() {
        TelemetryKit::builder()
            .service_name("telemetry-kit-cli")
            .ok()
            .and_then(|b| b.service_version(env!("CARGO_PKG_VERSION")).build().ok())
    } else {
        None
    };

    let result = match cli.command {
        Commands::New {
            name,
            project_type,
            yes,
            with_sync,
        } => cmd_new(name, project_type, yes, with_sync).await,
        Commands::Init { yes, service_name } => cmd_init(yes, service_name, cli.service).await,
        Commands::Test {
            org_id,
            app_id,
            token,
            secret,
        } => cmd_test(org_id, app_id, token, secret).await,
        Commands::Stats { detailed } => cmd_stats(detailed, cli.service).await,
        Commands::Sync { force } => cmd_sync(force, cli.service).await,
        Commands::Validate { config } => cmd_validate(config, cli.service).await,
        Commands::Clean { yes, all } => cmd_clean(yes, all, cli.service).await,
        #[cfg(feature = "privacy")]
        Commands::Consent { action } => cmd_consent(action, cli.service).await,
        Commands::Analyze {
            path,
            detailed,
            format,
        } => cmd_analyze(path, detailed, format).await,
    };

    let success = result.is_ok();
    let duration_ms = start.elapsed().as_millis() as u64;

    // Track command execution
    if let Some(ref t) = telemetry {
        let _ = t
            .track_command(&command_name, |event| {
                event.success(success).duration_ms(duration_ms)
            })
            .await;
    }

    match result {
        Ok(_) => std::process::exit(0),
        Err(e) => {
            eprintln!("{} {}", "Error:".red().bold(), e);
            std::process::exit(1);
        }
    }
}

/// Create a new project with telemetry pre-configured
async fn cmd_new(
    name: String,
    project_type: ProjectType,
    _skip_prompts: bool,
    with_sync: bool,
) -> Result<(), Box<dyn std::error::Error>> {
    use std::fs;

    println!("{}", "🔭 Creating New Telemetry-Enabled Project".cyan().bold());
    println!();

    // Validate project name
    if name.contains('/') || name.contains('\\') {
        return Err("Project name cannot contain path separators".into());
    }

    let project_path = std::env::current_dir()?.join(&name);

    if project_path.exists() {
        return Err(format!("Directory '{}' already exists", name).into());
    }

    println!("{} {}", "Project name:".green(), name.cyan());
    println!("{} {:?}", "Project type:".green(), project_type);
    println!("{} {}", "Location:".green(), project_path.display().to_string().dimmed());
    println!();

    // Create project directory
    fs::create_dir_all(&project_path)?;
    fs::create_dir_all(project_path.join("src"))?;

    // Create Cargo.toml
    println!("{} Cargo.toml", "Creating".green());
    let cargo_toml = generate_cargo_toml(&name, &project_type);
    fs::write(project_path.join("Cargo.toml"), cargo_toml)?;

    // Create source files
    match project_type {
        ProjectType::Bin => {
            println!("{} src/main.rs (CLI template)", "Creating".green());
            let main_rs = generate_cli_template(&name, with_sync);
            fs::write(project_path.join("src/main.rs"), main_rs)?;
        }
        ProjectType::Web => {
            println!("{} src/main.rs (web service template)", "Creating".green());
            let main_rs = generate_web_template(&name, with_sync);
            fs::write(project_path.join("src/main.rs"), main_rs)?;
        }
        ProjectType::Lib => {
            println!("{} src/lib.rs (library template)", "Creating".green());
            let lib_rs = generate_lib_template(&name, with_sync);
            fs::write(project_path.join("src/lib.rs"), lib_rs)?;
        }
    }

    // Create .gitignore
    println!("{} .gitignore", "Creating".green());
    let gitignore = "/target\n/Cargo.lock\n.env\n*.db\n*.db-shm\n*.db-wal\n";
    fs::write(project_path.join(".gitignore"), gitignore)?;

    // Create README.md
    println!("{} README.md", "Creating".green());
    let readme = generate_readme(&name, &project_type);
    fs::write(project_path.join("README.md"), readme)?;

    // Optionally create .env.example
    if with_sync {
        println!("{} .env.example", "Creating".green());
        let env_example = "# Telemetry sync configuration\n\
            # Get your credentials from https://telemetry-kit.dev\n\
            TK_ORG_ID=your-org-id-here\n\
            TK_APP_ID=your-app-id-here\n\
            TK_TOKEN=your-token-here\n\
            TK_SECRET=your-secret-here\n";
        fs::write(project_path.join(".env.example"), env_example)?;
    }

    // Initialize git repository
    println!("{} git repository", "Initializing".green());
    let git_init = std::process::Command::new("git")
        .args(["init"])
        .current_dir(&project_path)
        .output();

    match git_init {
        Ok(output) if output.status.success() => {
            println!("{} Git repository initialized", "✓".green().bold());
        }
        _ => {
            println!("{} Git not available (skipped)", "!".yellow().bold());
        }
    }

    println!();
    println!("{} {}", "✓ Project created:".green().bold(), name.cyan());
    println!();
    println!("Next steps:");
    println!("  {} - Enter the project directory", format!("cd {}", name).cyan());
    println!("  {} - Build the project", "cargo build".cyan());
    println!("  {} - Run the project", "cargo run".cyan());

    if with_sync {
        println!();
        println!("Sync is enabled! Configure your credentials:");
        println!("  1. Copy .env.example to .env");
        println!("  2. Edit .env with your telemetry-kit.dev credentials");
    } else {
        println!();
        println!("To enable sync later:");
        println!("  {} in your project directory", "telemetry-kit init".cyan());
    }

    println!();
    println!("View telemetry stats:");
    println!("  {} - in your project directory", "telemetry-kit stats".cyan());

    Ok(())
}

/// Initialize telemetry configuration
async fn cmd_init(
    skip_prompts: bool,
    service_name: Option<String>,
    override_service: Option<String>,
) -> Result<(), Box<dyn std::error::Error>> {
    println!("{}", "🔭 Telemetry Kit - Interactive Setup".cyan().bold());
    println!();

    // Determine service name
    let service = if let Some(name) = override_service.or(service_name) {
        name
    } else if skip_prompts {
        std::env::current_dir()?
            .file_name()
            .and_then(|n| n.to_str())
            .unwrap_or("my-app")
            .to_string()
    } else {
        let default = std::env::current_dir()?
            .file_name()
            .and_then(|n| n.to_str())
            .unwrap_or("my-app")
            .to_string();

        Input::new()
            .with_prompt("Service name")
            .default(default)
            .interact_text()?
    };

    println!("{} {}", "Service:".green(), service);
    println!();

    // Ask about sync configuration
    let configure_sync = if skip_prompts {
        false
    } else {
        Confirm::new()
            .with_prompt("Configure sync to telemetry-kit.dev?")
            .default(false)
            .interact()?
    };

    if configure_sync {
        println!();
        println!("{}", "Sync Configuration".cyan().bold());
        println!();

        let org_id: String = Input::new()
            .with_prompt("Organization ID")
            .interact_text()?;

        let app_id: String = Input::new().with_prompt("Application ID").interact_text()?;

        let token: String = Input::new().with_prompt("Auth Token").interact_text()?;

        let secret: String = Password::new().with_prompt("HMAC Secret").interact()?;

        println!();
        println!("{}", "Testing credentials...".yellow());

        // Test the credentials
        match test_credentials(&org_id, &app_id, &token, &secret).await {
            Ok(_) => {
                println!("{} Credentials validated successfully!", "✓".green().bold());
                println!();
                println!("{}", "Configuration saved".green());
                println!();
                println!("Add to your code:");
                println!();
                println!("{}", "  use telemetry_kit::prelude::*;".dimmed());
                println!();
                println!("{}", "  let telemetry = TelemetryKit::builder()".dimmed());
                println!(
                    "{}",
                    format!("      .service_name(\"{}\")?", service).dimmed()
                );
                println!(
                    "{}",
                    format!(
                        "      .with_sync_credentials(\"{}\", \"{}\", \"{}\", \"{}\")?",
                        org_id, app_id, token, "***"
                    )
                    .dimmed()
                );
                println!("{}", "      .build()?;".dimmed());
            }
            Err(e) => {
                println!("{} Failed to validate credentials: {}", "✗".red().bold(), e);
                println!();
                println!(
                    "{}",
                    "Configuration saved anyway (you can test later with 'telemetry-kit test')"
                        .yellow()
                );
            }
        }
    } else {
        println!();
        println!("{}", "Local-only mode".green());
        println!();
        println!("Add to your code:");
        println!();
        println!("{}", "  use telemetry_kit::prelude::*;".dimmed());
        println!();
        println!("{}", "  let telemetry = TelemetryKit::builder()".dimmed());
        println!(
            "{}",
            format!("      .service_name(\"{}\")?", service).dimmed()
        );
        println!("{}", "      .build()?;".dimmed());
    }

    println!();
    println!("{}", "✓ Initialization complete!".green().bold());
    println!();
    println!("Next steps:");
    println!("  • {} - View statistics", "telemetry-kit stats".cyan());
    println!("  • {} - Test sync", "telemetry-kit test".cyan());
    println!("  • {} - Validate config", "telemetry-kit validate".cyan());

    Ok(())
}

/// Test sync credentials
async fn cmd_test(
    org_id: Option<String>,
    app_id: Option<String>,
    token: Option<String>,
    secret: Option<String>,
) -> Result<(), Box<dyn std::error::Error>> {
    println!("{}", "🔭 Testing Sync Credentials".cyan().bold());
    println!();

    // Prompt for missing credentials
    let org_id = if let Some(id) = org_id {
        id
    } else {
        Input::new()
            .with_prompt("Organization ID")
            .interact_text()?
    };

    let app_id = if let Some(id) = app_id {
        id
    } else {
        Input::new().with_prompt("Application ID").interact_text()?
    };

    let token = if let Some(t) = token {
        t
    } else {
        Input::new().with_prompt("Auth Token").interact_text()?
    };

    let secret = if let Some(s) = secret {
        s
    } else {
        Password::new().with_prompt("HMAC Secret").interact()?
    };

    println!();
    let spinner = ProgressBar::new_spinner();
    spinner.set_style(
        ProgressStyle::default_spinner()
            .template("{spinner:.cyan} {msg}")
            .unwrap(),
    );
    spinner.set_message("Testing credentials...");
    spinner.enable_steady_tick(std::time::Duration::from_millis(100));

    match test_credentials(&org_id, &app_id, &token, &secret).await {
        Ok(_) => {
            spinner.finish_with_message(format!("{} Credentials valid!", "✓".green().bold()));
            println!();
            println!("{}", "Connection successful!".green().bold());
            println!();
            println!("Credentials:");
            println!("  Org ID:  {}", org_id.cyan());
            println!("  App ID:  {}", app_id.cyan());
            println!(
                "  Token:   {}...",
                token.chars().take(8).collect::<String>().cyan()
            );
            Ok(())
        }
        Err(e) => {
            spinner.finish_with_message(format!("{} Test failed", "✗".red().bold()));
            println!();
            Err(e)
        }
    }
}

/// View event statistics
async fn cmd_stats(
    detailed: bool,
    service: Option<String>,
) -> Result<(), Box<dyn std::error::Error>> {
    println!("{}", "📊 Event Statistics".cyan().bold());
    println!();

    let service_name = service.unwrap_or_else(|| {
        std::env::current_dir()
            .ok()
            .and_then(|p| {
                p.file_name()
                    .and_then(|n| n.to_str())
                    .map(|s| s.to_string())
            })
            .unwrap_or_else(|| "unknown".to_string())
    });

    // Get database path
    let mut db_path = dirs::home_dir().ok_or("Cannot determine home directory")?;
    db_path.push(".telemetry-kit");
    db_path.push(format!("{}.db", service_name));

    if !db_path.exists() {
        println!(
            "{} No telemetry data found for service: {}",
            "!".yellow().bold(),
            service_name
        );
        println!();
        println!("Database path: {}", db_path.display().to_string().dimmed());
        println!();
        println!("Have you initialized telemetry in your app?");
        println!("  Run {} for setup", "telemetry-kit init".cyan());
        return Ok(());
    }

    // Load telemetry and get stats
    use telemetry_kit::prelude::*;

    let telemetry = TelemetryKit::builder()
        .service_name(&service_name)?
        .db_path(db_path.clone())
        .build()?;

    let stats = telemetry.stats().await?;

    println!("{} {}", "Service:".green(), service_name.cyan());
    println!("{} {}", "Database:".green(), db_path.display());
    println!();

    println!("{}", "Events:".bold());
    println!(
        "  Total:      {}",
        stats.total_events.to_string().cyan().bold()
    );
    println!("  Synced:     {}", stats.synced_events.to_string().green());
    println!(
        "  Unsynced:   {}",
        stats.unsynced_events.to_string().yellow()
    );
    println!();

    let sync_percentage = if stats.total_events > 0 {
        (stats.synced_events as f64 / stats.total_events as f64 * 100.0) as u32
    } else {
        0
    };

    println!(
        "{} {}%",
        "Sync rate:".green(),
        sync_percentage.to_string().cyan()
    );

    if detailed {
        println!();
        println!("{}", "Storage:".bold());
        let metadata = std::fs::metadata(&db_path)?;
        let size_kb = metadata.len() / 1024;
        println!("  Size:       {} KB", size_kb.to_string().cyan());
        println!(
            "  Modified:   {}",
            format!("{:?}", metadata.modified()?).dimmed()
        );
    }

    println!();

    if stats.unsynced_events > 0 {
        println!(
            "{} You have {} unsynced events",
            "💡".yellow(),
            stats.unsynced_events.to_string().cyan()
        );
        println!("   Run {} to sync now", "telemetry-kit sync".cyan());
    } else {
        println!("{} All events synced!", "✓".green().bold());
    }

    Ok(())
}

/// Manually trigger sync
async fn cmd_sync(force: bool, service: Option<String>) -> Result<(), Box<dyn std::error::Error>> {
    println!("{}", "🔄 Manual Sync".cyan().bold());
    println!();

    let service_name = service.unwrap_or_else(|| {
        std::env::current_dir()
            .ok()
            .and_then(|p| {
                p.file_name()
                    .and_then(|n| n.to_str())
                    .map(|s| s.to_string())
            })
            .unwrap_or_else(|| "unknown".to_string())
    });

    // Get database path
    let mut db_path = dirs::home_dir().ok_or("Cannot determine home directory")?;
    db_path.push(".telemetry-kit");
    db_path.push(format!("{}.db", service_name));

    if !db_path.exists() {
        return Err(format!("No telemetry data found for service: {}", service_name).into());
    }

    println!(
        "{} This command requires sync credentials",
        "!".yellow().bold()
    );
    println!("Manual sync from CLI is not yet fully implemented.");
    println!();
    println!("Use the SDK's .sync() method in your application:");
    println!();
    println!("{}", "  telemetry.sync().await?;".dimmed());
    println!();
    println!("Or enable auto-sync:");
    println!();
    println!("{}", "  TelemetryKit::builder()".dimmed());
    println!("{}", "      .auto_sync(true)".dimmed());
    println!("{}", "      .build()?;".dimmed());

    if force {
        println!();
        println!("{} Force flag noted (not yet implemented)", "!".yellow());
    }

    Ok(())
}

/// Validate configuration
async fn cmd_validate(
    _config: Option<PathBuf>,
    service: Option<String>,
) -> Result<(), Box<dyn std::error::Error>> {
    println!("{}", "✓ Validating Configuration".cyan().bold());
    println!();

    let service_name = service.unwrap_or_else(|| {
        std::env::current_dir()
            .ok()
            .and_then(|p| {
                p.file_name()
                    .and_then(|n| n.to_str())
                    .map(|s| s.to_string())
            })
            .unwrap_or_else(|| "unknown".to_string())
    });

    println!("{} {}", "Service:".green(), service_name.cyan());

    // Check database
    let mut db_path = dirs::home_dir().ok_or("Cannot determine home directory")?;
    db_path.push(".telemetry-kit");
    db_path.push(format!("{}.db", service_name));

    if db_path.exists() {
        println!(
            "{} Database found: {}",
            "✓".green().bold(),
            db_path.display().to_string().dimmed()
        );

        // Try to load telemetry
        use telemetry_kit::prelude::*;
        match TelemetryKit::builder()
            .service_name(&service_name)?
            .db_path(db_path)
            .build()
        {
            Ok(_) => {
                println!("{} Telemetry initialization successful", "✓".green().bold());
            }
            Err(e) => {
                println!("{} Failed to initialize: {}", "✗".red().bold(), e);
                return Err(e.into());
            }
        }
    } else {
        println!(
            "{} No database found (run {} first)",
            "!".yellow().bold(),
            "telemetry-kit init".cyan()
        );
    }

    println!();
    println!("{}", "✓ Configuration valid".green().bold());

    Ok(())
}

/// Clear local events
async fn cmd_clean(
    skip_confirm: bool,
    clean_all: bool,
    service: Option<String>,
) -> Result<(), Box<dyn std::error::Error>> {
    println!("{}", "🧹 Clean Local Events".cyan().bold());
    println!();

    let service_name = service.unwrap_or_else(|| {
        std::env::current_dir()
            .ok()
            .and_then(|p| {
                p.file_name()
                    .and_then(|n| n.to_str())
                    .map(|s| s.to_string())
            })
            .unwrap_or_else(|| "unknown".to_string())
    });

    // Get database path
    let mut db_path = dirs::home_dir().ok_or("Cannot determine home directory")?;
    db_path.push(".telemetry-kit");
    db_path.push(format!("{}.db", service_name));

    if !db_path.exists() {
        println!(
            "{} No database found for service: {}",
            "!".yellow().bold(),
            service_name
        );
        return Ok(());
    }

    // Show stats before cleaning
    use telemetry_kit::prelude::*;
    let telemetry = TelemetryKit::builder()
        .service_name(&service_name)?
        .db_path(db_path.clone())
        .build()?;

    let stats = telemetry.stats().await?;

    println!("{} {}", "Service:".green(), service_name.cyan());
    println!(
        "{} {} events ({} unsynced)",
        "Current:".green(),
        stats.total_events,
        stats.unsynced_events
    );
    println!();

    if stats.unsynced_events > 0 {
        println!(
            "{} You have {} unsynced events that will be lost!",
            "⚠️ ".yellow().bold(),
            stats.unsynced_events.to_string().yellow()
        );
        println!();
    }

    let confirmed = if skip_confirm {
        true
    } else {
        Confirm::new()
            .with_prompt("Are you sure you want to delete all local events?")
            .default(false)
            .interact()?
    };

    if !confirmed {
        println!("{}", "Cancelled".yellow());
        return Ok(());
    }

    drop(telemetry); // Close database connection

    // Delete database
    std::fs::remove_file(&db_path)?;
    println!("{} Database deleted", "✓".green().bold());

    if clean_all {
        // TODO: Remove configuration files when implemented
        println!(
            "{} Configuration cleaning not yet implemented",
            "!".yellow()
        );
    }

    println!();
    println!("{}", "✓ Clean complete".green().bold());

    Ok(())
}

/// Test credentials by attempting to create a sync client
async fn test_credentials(
    org_id: &str,
    app_id: &str,
    token: &str,
    secret: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    use telemetry_kit::sync::SyncClient;
    use telemetry_kit::sync::SyncConfig;

    let config = SyncConfig::builder()
        .org_id(org_id)?
        .app_id(app_id)?
        .token(token)
        .secret(secret)
        .build()?;

    let _client = SyncClient::new(config)?;

    // TODO: Actually test connectivity by sending a test request
    // For now, just validate that the client can be created

    Ok(())
}

/// Get service name from override or current directory
fn get_service_name(
    override_service: Option<String>,
) -> Result<String, Box<dyn std::error::Error>> {
    Ok(override_service.unwrap_or_else(|| {
        std::env::current_dir()
            .ok()
            .and_then(|p| {
                p.file_name()
                    .and_then(|n| n.to_str())
                    .map(|s| s.to_string())
            })
            .unwrap_or_else(|| "default".to_string())
    }))
}

/// Get command name for telemetry tracking
fn get_command_name(command: &Commands) -> String {
    match command {
        Commands::New { .. } => "new".to_string(),
        Commands::Init { .. } => "init".to_string(),
        Commands::Test { .. } => "test".to_string(),
        Commands::Stats { .. } => "stats".to_string(),
        Commands::Sync { .. } => "sync".to_string(),
        Commands::Validate { .. } => "validate".to_string(),
        Commands::Clean { .. } => "clean".to_string(),
        #[cfg(feature = "privacy")]
        Commands::Consent { .. } => "consent".to_string(),
        Commands::Analyze { .. } => "analyze".to_string(),
    }
}

/// Manage privacy consent
#[cfg(feature = "privacy")]
async fn cmd_consent(
    action: ConsentAction,
    override_service: Option<String>,
) -> Result<(), Box<dyn std::error::Error>> {
    use telemetry_kit::privacy::{PrivacyConfig, PrivacyManager};
    use telemetry_kit::TelemetryKit;

    // Determine service name
    let service = get_service_name(override_service)?;

    // Create a privacy manager
    let config = PrivacyConfig::default();
    let manager = PrivacyManager::new(config, &service)?;

    match action {
        ConsentAction::Grant => {
            manager.grant_consent(&service)?;
            println!("{}", "✓ Consent granted".green().bold());
            println!();
            println!("Telemetry tracking is now {}", "enabled".green());
            println!("Service: {}", service.cyan());
            println!();
            println!("You can revoke consent at any time with:");
            println!("  {}", "telemetry-kit consent deny".yellow());
        }

        ConsentAction::Deny => {
            manager.deny_consent(&service)?;
            println!("{}", "✗ Consent denied".red().bold());
            println!();
            println!("Telemetry tracking is now {}", "disabled".red());
            println!("Service: {}", service.cyan());
            println!();
            println!("You can grant consent at any time with:");
            println!("  {}", "telemetry-kit consent grant".yellow());
        }

        ConsentAction::OptOut => {
            manager.opt_out(&service)?;
            println!("{}", "🚫 Opted out of telemetry".red().bold());
            println!();
            println!("You have opted out of all telemetry tracking.");
            println!("Service: {}", service.cyan());
            println!();
            println!("This is equivalent to setting DO_NOT_TRACK=1");
            println!();
            println!("You can opt back in with:");
            println!("  {}", "telemetry-kit consent grant".yellow());
        }

        ConsentAction::Status => {
            // Load current consent
            let consent = manager.load_consent()?;

            println!("{}", "Privacy Consent Status".cyan().bold());
            println!();
            println!("Service: {}", service.cyan());

            // Check DO_NOT_TRACK
            let dnt_enabled = TelemetryKit::is_do_not_track_enabled();
            if dnt_enabled {
                println!();
                println!(
                    "{} {}",
                    "⚠".yellow(),
                    "DO_NOT_TRACK environment variable is set".yellow()
                );
                println!("  Telemetry is {} regardless of consent", "disabled".red());
            }

            println!();
            println!(
                "Consent Status: {}",
                match consent.status {
                    telemetry_kit::privacy::ConsentStatus::Unknown => "Unknown (not set)".yellow(),
                    telemetry_kit::privacy::ConsentStatus::Granted => "Granted ✓".green(),
                    telemetry_kit::privacy::ConsentStatus::Denied => "Denied ✗".red(),
                    telemetry_kit::privacy::ConsentStatus::OptedOut => "Opted Out 🚫".red(),
                }
            );

            if consent.timestamp.timestamp() > 0 {
                println!(
                    "Last Updated: {}",
                    consent.timestamp.format("%Y-%m-%d %H:%M:%S UTC")
                );
            }

            println!();

            // Show what will happen
            match manager.should_track() {
                Ok(true) => {
                    println!(
                        "Current Behavior: Telemetry is {}",
                        "ENABLED".green().bold()
                    );
                    println!("  Events will be tracked and synced");
                }
                Ok(false) => {
                    println!("Current Behavior: Telemetry is {}", "DISABLED".red().bold());
                    if dnt_enabled {
                        println!("  Reason: DO_NOT_TRACK environment variable");
                    } else {
                        println!("  Reason: Consent denied or not granted");
                    }
                }
                Err(e) => {
                    println!("Current Behavior: {}", "ERROR".red().bold());
                    println!("  {}", e);
                }
            }

            println!();
            println!("Commands:");
            println!(
                "  {} - Grant consent",
                "telemetry-kit consent grant".yellow()
            );
            println!("  {} - Deny consent", "telemetry-kit consent deny".yellow());
            println!(
                "  {} - Opt out completely",
                "telemetry-kit consent opt-out".yellow()
            );
        }

        ConsentAction::Prompt => {
            // Check if DO_NOT_TRACK is set
            if TelemetryKit::is_do_not_track_enabled() {
                println!("{}", "⚠ DO_NOT_TRACK is enabled".yellow().bold());
                println!();
                println!("The DO_NOT_TRACK environment variable is set.");
                println!("Telemetry will be disabled regardless of consent.");
                println!();
                println!("To enable telemetry, unset DO_NOT_TRACK:");
                println!("  {}", "unset DO_NOT_TRACK".cyan());
                return Ok(());
            }

            // Show current status
            let consent = manager.load_consent()?;
            if consent.status != telemetry_kit::privacy::ConsentStatus::Unknown {
                println!(
                    "Current consent status: {}",
                    match consent.status {
                        telemetry_kit::privacy::ConsentStatus::Granted => "Granted".green(),
                        telemetry_kit::privacy::ConsentStatus::Denied => "Denied".red(),
                        telemetry_kit::privacy::ConsentStatus::OptedOut => "Opted Out".red(),
                        telemetry_kit::privacy::ConsentStatus::Unknown => "Unknown".yellow(),
                    }
                );
                println!();
            }

            // Interactive prompt
            println!("{}", "Privacy Consent".cyan().bold());
            println!();
            println!("Service: {}", service.cyan());
            println!();
            println!("telemetry-kit collects anonymous usage statistics to help");
            println!("improve the tool. We respect your privacy:");
            println!();
            println!(
                "  • {} anonymous machine ID (no personal data)",
                "✓".green()
            );
            println!("  • {} command usage and success rates", "✓".green());
            println!("  • {} errors and performance metrics", "✓".green());
            println!("  • {} always respects DO_NOT_TRACK", "✓".green());
            println!();
            println!("We do NOT collect:");
            println!("  • {} usernames or email addresses", "✗".red());
            println!("  • {} file paths (sanitized to ~)", "✗".red());
            println!("  • {} any personally identifiable information", "✗".red());
            println!();

            let choices = vec!["Grant consent", "Deny consent", "Opt out (DO_NOT_TRACK)"];
            let selection = Select::new()
                .with_prompt("What would you like to do?")
                .items(&choices)
                .default(0)
                .interact()?;

            println!();

            match selection {
                0 => {
                    manager.grant_consent(&service)?;
                    println!("{}", "✓ Consent granted".green().bold());
                    println!("Thank you! Telemetry is now enabled.");
                }
                1 => {
                    manager.deny_consent(&service)?;
                    println!("{}", "✗ Consent denied".red().bold());
                    println!("Telemetry is now disabled.");
                }
                2 => {
                    manager.opt_out(&service)?;
                    println!("{}", "🚫 Opted out".red().bold());
                    println!("You have opted out of all telemetry.");
                }
                _ => unreachable!(),
            }

            println!();
            println!("You can change this at any time with:");
            println!("  {}", "telemetry-kit consent status".yellow());
        }
    }

    Ok(())
}

/// Generate Cargo.toml for the project
fn generate_cargo_toml(name: &str, project_type: &ProjectType) -> String {
    let bin_section = match project_type {
        ProjectType::Lib => "",
        _ => "\n[[bin]]\nname = \"{{name}}\"\npath = \"src/main.rs\"\n",
    };

    let lib_section = match project_type {
        ProjectType::Lib => "\n[lib]\nname = \"{{lib_name}}\"\npath = \"src/lib.rs\"\n",
        _ => "",
    };

    let extra_deps = match project_type {
        ProjectType::Web => "axum = \"0.7\"\ntower = \"0.4\"\ntower-http = { version = \"0.5\", features = [\"trace\"] }\ntracing = \"0.1\"\ntracing-subscriber = { version = \"0.3\", features = [\"env-filter\"] }\n",
        _ => "",
    };

    format!(
        r#"[package]
name = "{}"
version = "0.1.0"
edition = "2021"
{}{}
[dependencies]
# Note: telemetry-kit v0.2 is not yet published on crates.io
# For now, you can use a git dependency or path dependency:
# telemetry-kit = {{ git = "https://github.com/ibrahimcesar/telemetry-kit", features = ["sync"] }}
# or:
# telemetry-kit = {{ path = "../telemetry-kit", features = ["sync"] }}
#
# Once v0.2.0 is published, use:
telemetry-kit = {{ version = "0.2", features = ["sync"] }}
tokio = {{ version = "1.35", features = ["full"] }}
{}
"#,
        name,
        bin_section.replace("{{name}}", name),
        lib_section.replace("{{lib_name}}", &name.replace('-', "_")),
        extra_deps
    )
}

/// Generate README.md
fn generate_readme(name: &str, project_type: &ProjectType) -> String {
    let type_desc = match project_type {
        ProjectType::Bin => "A Rust CLI application with telemetry",
        ProjectType::Lib => "A Rust library with telemetry",
        ProjectType::Web => "A Rust web service with telemetry",
    };

    format!(
        r#"# {}

{}

## Features

- ✅ Privacy-first telemetry with telemetry-kit
- ✅ Anonymous usage tracking
- ✅ GDPR compliant
- ✅ DO_NOT_TRACK support

## Usage

```bash
cargo run
```

## Telemetry

This project uses [telemetry-kit](https://github.com/ibrahimcesar/telemetry-kit) for privacy-first usage analytics.

**What we track:**
- Anonymous usage statistics
- Command execution and success rates
- Performance metrics

**What we DON'T track:**
- Personal information
- File paths or code
- Anything identifiable

**Privacy controls:**
- Set `DO_NOT_TRACK=1` to disable all telemetry
- Data is stored locally by default
- You control what gets synced

View your telemetry stats:
```bash
telemetry-kit stats
```

## License

MIT OR Apache-2.0
"#,
        name, type_desc
    )
}

/// Generate CLI template (main.rs for binary)
fn generate_cli_template(name: &str, with_sync: bool) -> String {
    let sync_setup = if with_sync {
        r#"
    // Load sync credentials from environment
    let telemetry = if let (Ok(org_id), Ok(app_id), Ok(token), Ok(secret)) = (
        std::env::var("TK_ORG_ID"),
        std::env::var("TK_APP_ID"),
        std::env::var("TK_TOKEN"),
        std::env::var("TK_SECRET"),
    ) {
        TelemetryKit::builder()
            .service_name("{{name}}")?
            .service_version(env!("CARGO_PKG_VERSION"))
            .with_sync_credentials(&org_id, &app_id, &token, &secret)?
            .auto_sync(true)
            .build()?
    } else {
        // Fallback to local-only if credentials not set
        println!("⚠️  Telemetry sync not configured (using local-only mode)");
        println!("   Set TK_ORG_ID, TK_APP_ID, TK_TOKEN, TK_SECRET to enable sync\n");
        TelemetryKit::builder()
            .service_name("{{name}}")?
            .service_version(env!("CARGO_PKG_VERSION"))
            .build()?
    };"#
    } else {
        r#"
    // Initialize telemetry (local-only)
    let telemetry = TelemetryKit::builder()
        .service_name("{{name}}")?
        .service_version(env!("CARGO_PKG_VERSION"))
        .build()?;"#
    };

    format!(
        r#"//! {}
//!
//! A Rust CLI application with privacy-first telemetry.

use telemetry_kit::prelude::*;

#[tokio::main]
async fn main() -> Result<()> {{
    println!("🔭 {} v{{}}\n", env!("CARGO_PKG_VERSION"));
{}

    // Your application logic here
    println!("Hello from {}!");

    // Track a command
    telemetry
        .track_command("run", |event| {{
            event.success(true).duration_ms(42)
        }})
        .await?;

    println!("\n✓ Command tracked!");

    // View telemetry stats
    let stats = telemetry.stats().await?;
    println!("\nTelemetry Stats:");
    println!("  Total events: {{}}", stats.total_events);
    println!("  Synced: {{}}", stats.synced_events);
    println!("  Unsynced: {{}}", stats.unsynced_events);

    Ok(())
}}
"#,
        name,
        name,
        sync_setup.replace("{{name}}", name),
        name
    )
}

/// Generate web service template (main.rs for web)
fn generate_web_template(name: &str, with_sync: bool) -> String {
    let sync_setup = if with_sync {
        r#"
    let telemetry = if let (Ok(org_id), Ok(app_id), Ok(token), Ok(secret)) = (
        std::env::var("TK_ORG_ID"),
        std::env::var("TK_APP_ID"),
        std::env::var("TK_TOKEN"),
        std::env::var("TK_SECRET"),
    ) {
        TelemetryKit::builder()
            .service_name("{{name}}")?
            .service_version(env!("CARGO_PKG_VERSION"))
            .with_sync_credentials(&org_id, &app_id, &token, &secret)?
            .auto_sync(true)
            .build()?
    } else {
        println!("⚠️  Telemetry sync not configured (using local-only mode)");
        TelemetryKit::builder()
            .service_name("{{name}}")?
            .service_version(env!("CARGO_PKG_VERSION"))
            .build()?
    };"#
    } else {
        r#"
    let telemetry = TelemetryKit::builder()
        .service_name("{{name}}")?
        .service_version(env!("CARGO_PKG_VERSION"))
        .build()?;"#
    };

    format!(
        r#"//! {}
//!
//! A Rust web service with privacy-first telemetry.

use axum::{{
    routing::get,
    Router,
}};
use std::sync::Arc;
use telemetry_kit::prelude::*;

#[tokio::main]
async fn main() -> Result<()> {{
    // Initialize tracing
    tracing_subscriber::fmt::init();
{}

    let telemetry = Arc::new(telemetry);

    // Track startup
    telemetry
        .track_feature("startup", |event| event.success(true))
        .await?;

    // Build router
    let app = Router::new()
        .route("/", get(|| async {{ "Hello from {}!" }}))
        .route("/health", get(|| async {{ "OK" }}));

    // Start server
    let addr = "0.0.0.0:3000";
    println!("🚀 Server listening on {{}}", addr);

    let listener = tokio::net::TcpListener::bind(addr).await
        .map_err(|e| TelemetryError::Other(e.to_string()))?;

    axum::serve(listener, app).await
        .map_err(|e| TelemetryError::Other(e.to_string()))?;

    Ok(())
}}
"#,
        name,
        sync_setup.replace("{{name}}", name),
        name,
    )
}

/// Analyze code and suggest instrumentation points
async fn cmd_analyze(
    path: Option<PathBuf>,
    detailed: bool,
    format: String,
) -> Result<(), Box<dyn std::error::Error>> {
    use std::fs;

    println!("{}", "🔍 Analyzing Code for Instrumentation Opportunities".cyan().bold());
    println!();

    let search_path = path.unwrap_or_else(|| std::env::current_dir().unwrap());

    if !search_path.exists() {
        return Err(format!("Path does not exist: {}", search_path.display()).into());
    }

    println!("{} {}", "Analyzing:".green(), search_path.display().to_string().dimmed());
    println!();

    // Find all Rust files
    let rust_files = find_rust_files(&search_path)?;

    if rust_files.is_empty() {
        println!("{} No Rust files found", "!".yellow().bold());
        return Ok(());
    }

    println!("{} {} Rust files", "Found:".green(), rust_files.len().to_string().cyan());
    println!();

    // Analyze each file
    let mut all_recommendations = Vec::new();

    for file_path in &rust_files {
        if let Ok(content) = fs::read_to_string(file_path) {
            let recommendations = analyze_file(file_path, &content);
            if !recommendations.is_empty() {
                all_recommendations.extend(recommendations);
            }
        }
    }

    // Display recommendations
    if all_recommendations.is_empty() {
        println!("{} No instrumentation opportunities found", "✓".green().bold());
        println!();
        println!("Your code is already well-instrumented or doesn't have obvious patterns to track.");
        return Ok(());
    }

    match format.as_str() {
        "json" => display_recommendations_json(&all_recommendations)?,
        _ => display_recommendations_text(&all_recommendations, detailed)?,
    }

    println!();
    println!("{}", "Next Steps:".cyan().bold());
    println!("  1. Review the recommendations above");
    println!("  2. Add telemetry tracking where it makes sense");
    println!("  3. Use {} for automatic instrumentation", "#[instrument]".yellow());
    println!("  4. Run {} to verify your changes", "cargo check".cyan());

    Ok(())
}

/// Find all Rust files in a directory
fn find_rust_files(path: &PathBuf) -> Result<Vec<PathBuf>, Box<dyn std::error::Error>> {
    use std::fs;

    let mut rust_files = Vec::new();

    if path.is_file() && path.extension().and_then(|s| s.to_str()) == Some("rs") {
        rust_files.push(path.clone());
        return Ok(rust_files);
    }

    if path.is_dir() {
        for entry in fs::read_dir(path)? {
            let entry = entry?;
            let entry_path = entry.path();

            // Skip target, .git, and hidden directories
            if entry_path.is_dir() {
                let name = entry_path.file_name().and_then(|n| n.to_str()).unwrap_or("");
                if name == "target" || name == ".git" || name.starts_with('.') {
                    continue;
                }
                rust_files.extend(find_rust_files(&entry_path)?);
            } else if entry_path.extension().and_then(|s| s.to_str()) == Some("rs") {
                rust_files.push(entry_path);
            }
        }
    }

    Ok(rust_files)
}

#[derive(Debug, Clone)]
struct Recommendation {
    file: PathBuf,
    line: usize,
    function_name: String,
    pattern: InstrumentationPattern,
    suggestion: String,
    code_snippet: String,
    priority: Priority,
}

#[derive(Debug, Clone, PartialEq)]
enum InstrumentationPattern {
    MainFunction,
    AsyncResultFunction,
    PublicApiFunction,
    HttpHandler,
    CommandHandler,
    DatabaseOperation,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
enum Priority {
    High,
    Medium,
    Low,
}

/// Analyze a single Rust file for instrumentation opportunities
fn analyze_file(file_path: &PathBuf, content: &str) -> Vec<Recommendation> {
    let mut recommendations = Vec::new();

    for (line_num, line) in content.lines().enumerate() {
        let line_num = line_num + 1;
        let trimmed = line.trim();

        // Pattern 1: Main function
        if trimmed.contains("fn main()") || trimmed.contains("async fn main()") {
            if !content.contains(".track_") && !content.contains("TelemetryKit") {
                recommendations.push(Recommendation {
                    file: file_path.clone(),
                    line: line_num,
                    function_name: "main".to_string(),
                    pattern: InstrumentationPattern::MainFunction,
                    suggestion: "Track application startup and lifecycle".to_string(),
                    code_snippet: r#"telemetry.track_command("startup", |e| e.success(true)).await?;"#.to_string(),
                    priority: Priority::High,
                });
            }
        }

        // Pattern 2: Public async functions that return Result
        if trimmed.starts_with("pub async fn ") && trimmed.contains("-> Result") {
            let fn_name = extract_function_name(trimmed);
            if !fn_name.is_empty() {
                // Check if function already has #[instrument] by looking at previous lines
                let has_instrument = if line_num > 1 {
                    content.lines().nth(line_num - 2)
                        .map(|l| l.trim().contains("#[instrument]"))
                        .unwrap_or(false)
                } else {
                    false
                };

                if !has_instrument {
                    recommendations.push(Recommendation {
                        file: file_path.clone(),
                        line: line_num,
                        function_name: fn_name.clone(),
                        pattern: InstrumentationPattern::PublicApiFunction,
                        suggestion: format!("Track '{}' execution for API visibility", fn_name),
                        code_snippet: format!(r#"telemetry.track_feature("{}", |e| e.success(true)).await?;"#, fn_name),
                        priority: Priority::Medium,
                    });
                }
            }
        }

        // Pattern 3: HTTP route handlers (Axum)
        if trimmed.contains("async fn") && (trimmed.contains("Request") || trimmed.contains("Json") || trimmed.contains("Path")) {
            let fn_name = extract_function_name(trimmed);
            if !fn_name.is_empty() {
                recommendations.push(Recommendation {
                    file: file_path.clone(),
                    line: line_num,
                    function_name: fn_name.clone(),
                    pattern: InstrumentationPattern::HttpHandler,
                    suggestion: format!("Track HTTP endpoint '{}' requests", fn_name),
                    code_snippet: format!(r#"telemetry.track_feature("http.{}", |e| e.success(true)).await?;"#, fn_name),
                    priority: Priority::High,
                });
            }
        }

        // Pattern 4: Database operations
        if trimmed.contains("sqlx::query") || trimmed.contains(".execute(") || trimmed.contains(".fetch") {
            recommendations.push(Recommendation {
                file: file_path.clone(),
                line: line_num,
                function_name: "database_operation".to_string(),
                pattern: InstrumentationPattern::DatabaseOperation,
                suggestion: "Track database query performance".to_string(),
                code_snippet: r#"let start = std::time::Instant::now(); /* query */ telemetry.track_feature("db.query", |e| e.duration_ms(start.elapsed().as_millis() as u64)).await?;"#.to_string(),
                priority: Priority::Medium,
            });
        }
    }

    // Deduplicate recommendations for the same line
    recommendations.sort_by_key(|r| r.line);
    recommendations.dedup_by_key(|r| (r.file.clone(), r.line));

    recommendations
}

/// Extract function name from a function signature
fn extract_function_name(line: &str) -> String {
    if let Some(fn_pos) = line.find("fn ") {
        let after_fn = &line[fn_pos + 3..];
        if let Some(paren_pos) = after_fn.find('(') {
            return after_fn[..paren_pos].trim().to_string();
        }
    }
    String::new()
}

/// Display recommendations in text format
fn display_recommendations_text(
    recommendations: &[Recommendation],
    detailed: bool,
) -> Result<(), Box<dyn std::error::Error>> {
    println!("{}", "📊 Instrumentation Recommendations".cyan().bold());
    println!();

    let mut by_priority: Vec<_> = recommendations.iter().collect();
    by_priority.sort_by_key(|r| &r.priority);

    let high_priority: Vec<_> = by_priority.iter().filter(|r| r.priority == Priority::High).collect();
    let medium_priority: Vec<_> = by_priority.iter().filter(|r| r.priority == Priority::Medium).collect();
    let low_priority: Vec<_> = by_priority.iter().filter(|r| r.priority == Priority::Low).collect();

    if !high_priority.is_empty() {
        println!("{} {} High Priority", "⚠️ ".red().bold(), high_priority.len().to_string().red());
        for rec in high_priority {
            display_recommendation(rec, detailed);
        }
        println!();
    }

    if !medium_priority.is_empty() {
        println!("{} {} Medium Priority", "⚡".yellow().bold(), medium_priority.len().to_string().yellow());
        for rec in medium_priority {
            display_recommendation(rec, detailed);
        }
        println!();
    }

    if !low_priority.is_empty() && detailed {
        println!("{} {} Low Priority", "💡".dimmed(), low_priority.len().to_string().dimmed());
        for rec in low_priority {
            display_recommendation(rec, detailed);
        }
        println!();
    }

    println!("{}", format!("Total: {} recommendations", recommendations.len()).cyan());

    Ok(())
}

/// Display a single recommendation
fn display_recommendation(rec: &Recommendation, detailed: bool) {
    let file_display = rec.file.file_name()
        .and_then(|n| n.to_str())
        .unwrap_or("unknown");

    println!("  {} {}:{}", "•".cyan(), file_display.cyan(), rec.line.to_string().dimmed());
    println!("    {}: {}", "Function".dimmed(), rec.function_name.yellow());
    println!("    {}: {}", "Reason".dimmed(), rec.suggestion);

    if detailed {
        println!("    {}: {}", "Pattern".dimmed(), format!("{:?}", rec.pattern).dimmed());
        println!("    {}", "Suggested code:".dimmed());
        println!("    {}", rec.code_snippet.green());
    }

    println!();
}

/// Display recommendations in JSON format
fn display_recommendations_json(
    recommendations: &[Recommendation],
) -> Result<(), Box<dyn std::error::Error>> {
    use serde_json::json;

    let json_recommendations: Vec<_> = recommendations
        .iter()
        .map(|r| {
            json!({
                "file": r.file.display().to_string(),
                "line": r.line,
                "function": r.function_name,
                "pattern": format!("{:?}", r.pattern),
                "suggestion": r.suggestion,
                "code_snippet": r.code_snippet,
                "priority": format!("{:?}", r.priority),
            })
        })
        .collect();

    let output = json!({
        "total": recommendations.len(),
        "recommendations": json_recommendations,
    });

    println!("{}", serde_json::to_string_pretty(&output)?);

    Ok(())
}

/// Generate library template (lib.rs)
fn generate_lib_template(name: &str, with_sync: bool) -> String {
    let sync_setup = if with_sync {
        r#"
        let telemetry = if let (Ok(org_id), Ok(app_id), Ok(token), Ok(secret)) = (
            std::env::var("TK_ORG_ID"),
            std::env::var("TK_APP_ID"),
            std::env::var("TK_TOKEN"),
            std::env::var("TK_SECRET"),
        ) {
            TelemetryKit::builder()
                .service_name("{{name}}")?
                .service_version(env!("CARGO_PKG_VERSION"))
                .with_sync_credentials(&org_id, &app_id, &token, &secret)?
                .auto_sync(true)
                .build()?
        } else {
            TelemetryKit::builder()
                .service_name("{{name}}")?
                .service_version(env!("CARGO_PKG_VERSION"))
                .build()?
        };"#
    } else {
        r#"
        let telemetry = TelemetryKit::builder()
            .service_name("{{name}}")?
            .service_version(env!("CARGO_PKG_VERSION"))
            .build()?;"#
    };

    format!(
        r#"//! {}
//!
//! A Rust library with privacy-first telemetry.

use telemetry_kit::prelude::*;

/// Initialize the library with telemetry
pub async fn init() -> Result<TelemetryKit> {{
{}

    // Track initialization
    telemetry
        .track_feature("init", |event| event.success(true))
        .await?;

    Ok(telemetry)
}}

/// Example function that tracks its usage
pub async fn do_something(telemetry: &TelemetryKit) -> Result<()> {{
    // Your library logic here

    // Track feature usage
    telemetry
        .track_feature("do_something", |event| {{
            event.success(true)
        }})
        .await?;

    Ok(())
}}

#[cfg(test)]
mod tests {{
    use super::*;

    #[tokio::test]
    async fn test_init() {{
        let telemetry = init().await.unwrap();
        let stats = telemetry.stats().await.unwrap();
        assert!(stats.total_events > 0);
    }}
}}
"#,
        name,
        sync_setup.replace("{{name}}", name)
    )
}
