//! # telemetry-kit
//!
//! Privacy-first, batteries-included telemetry for Rust applications.
//!
//! ## Quick Start
//!
//! ```rust,no_run
//! use telemetry_kit::prelude::*;
//!
//! #[tokio::main]
//! async fn main() -> telemetry_kit::Result<()> {
//!     // Initialize telemetry with sensible defaults
//!     let telemetry = TelemetryKit::builder()
//!         .service_name("my-app")?
//!         .service_version(env!("CARGO_PKG_VERSION"))
//!         .build()?;
//!
//!     // Track events
//!     telemetry.track_command("build", |event| {
//!         event
//!             .flag("--release")
//!             .duration_ms(1234)
//!             .success(true)
//!     }).await?;
//!
//!     Ok(())
//! }
//! ```
//!
//! ## Features
//!
//! - **Zero-config**: Sensible defaults, minimal boilerplate
//! - **Privacy-first**: Built-in anonymization and GDPR compliance
//! - **CLI-optimized**: Perfect for command-line applications
//! - **Self-hostable**: Simple collection server included
//!
//! ## Sync Protocol
//!
//! The SDK automatically syncs events to telemetry-kit.dev using HMAC-SHA256
//! authentication. Events are buffered locally in SQLite and synced in batches.

#![warn(missing_docs)]
#![warn(rustdoc::missing_crate_level_docs)]

pub mod error;
pub mod event;
pub mod storage;
pub mod user;

#[cfg(feature = "sync")]
pub mod sync;

mod builder;
mod telemetry;

pub use builder::TelemetryBuilder;
pub use error::{Result, TelemetryError};
pub use telemetry::TelemetryKit;

/// Prelude module for convenient imports
pub mod prelude {
    //! Commonly used imports for telemetry-kit
    //!
    //! ```rust
    //! use telemetry_kit::prelude::*;
    //! ```
    pub use crate::builder::TelemetryBuilder;
    pub use crate::error::{Result, TelemetryError};
    pub use crate::event::*;
    pub use crate::telemetry::TelemetryKit;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_initialization() {
        let builder = TelemetryKit::builder();
        assert!(builder.service_name("test").is_ok());
    }
}
