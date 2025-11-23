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

use clap::{Parser, Subcommand};
use colored::Colorize;
use dialoguer::{Confirm, Input, Password, Select};
use indicatif::{ProgressBar, ProgressStyle};
use std::path::PathBuf;

#[derive(Parser)]
#[command(name = "telemetry-kit")]
#[command(version, about = "Privacy-first telemetry toolkit for Rust applications", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,

    /// Service name to operate on (defaults to current directory name)
    #[arg(short, long, global = true)]
    service: Option<String>,
}

#[derive(Subcommand)]
enum Commands {
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
    let cli = Cli::parse();

    let result = match cli.command {
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
    };

    match result {
        Ok(_) => std::process::exit(0),
        Err(e) => {
            eprintln!("{} {}", "Error:".red().bold(), e);
            std::process::exit(1);
        }
    }
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
