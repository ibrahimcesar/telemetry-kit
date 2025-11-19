//! Basic example showing how telemetry-kit will work
//!
//! Run with:
//! ```
//! cargo run --example basic
//! ```

use telemetry_kit::prelude::*;

fn main() {
    println!("telemetry-kit v{}", env!("CARGO_PKG_VERSION"));
    println!();
    println!("This is a placeholder example.");
    println!("The actual implementation is under development.");
    println!();
    
    // This is how it will work in v0.1.0+:
    println!("Example of planned API:");
    println!();
    println!(r#"
    use telemetry_kit::prelude::*;

    #[tokio::main]
    async fn main() -> Result<(), Box<dyn std::error::Error>> {{
        let _guard = telemetry_kit::init()
            .service_name("my-app")
            .endpoint("https://telemetry.myapp.com")
            .anonymous()
            .init()?;
        
        do_work().await?;
        
        Ok(())
    }}

    #[instrument]
    async fn do_work() -> Result<(), Box<dyn std::error::Error>> {{
        // Your code here - automatically instrumented!
        Ok(())
    }}
    "#);
    
    // Current minimal implementation:
    let _guard = telemetry_kit::init()
        .service_name("basic-example")
        .init();
    
    println!();
    println!("✓ Basic initialization successful (placeholder)");
}
