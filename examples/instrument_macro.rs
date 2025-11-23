//! Example: Using the #[instrument] macro for automatic function instrumentation
//!
//! This example demonstrates how to use the #[instrument] procedural macro
//! to automatically track function execution timing.
//!
//! Run with:
//! ```bash
//! cargo run --example instrument_macro --features macros
//! ```

#[cfg(feature = "macros")]
use telemetry_kit::instrument;

/// Example async function that fetches data
#[cfg(feature = "macros")]
#[instrument]
async fn fetch_user_data(user_id: u64) -> Result<String, String> {
    // Simulate some work
    tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;

    if user_id > 0 {
        Ok(format!("User data for ID: {}", user_id))
    } else {
        Err("Invalid user ID".to_string())
    }
}

/// Example sync function for calculations
#[cfg(feature = "macros")]
#[instrument]
fn calculate_total(items: &[i32]) -> Result<i32, String> {
    if items.is_empty() {
        Err("Empty items list".to_string())
    } else {
        Ok(items.iter().sum())
    }
}

/// Example async function without Result
#[cfg(feature = "macros")]
#[instrument]
async fn process_data(data: &str) -> usize {
    // Simulate processing
    tokio::time::sleep(tokio::time::Duration::from_millis(50)).await;
    data.len()
}

#[cfg(feature = "macros")]
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🔭 telemetry-kit #[instrument] Macro Example\n");

    // Test async function with Result
    println!("Testing async function with Result...");
    match fetch_user_data(123).await {
        Ok(data) => println!("  ✅ Success: {}", data),
        Err(e) => println!("  ❌ Error: {}", e),
    }

    match fetch_user_data(0).await {
        Ok(data) => println!("  ✅ Success: {}", data),
        Err(e) => println!("  ❌ Error: {}", e),
    }

    // Test sync function with Result
    println!("\nTesting sync function with Result...");
    match calculate_total(&[10, 20, 30]) {
        Ok(total) => println!("  ✅ Total: {}", total),
        Err(e) => println!("  ❌ Error: {}", e),
    }

    match calculate_total(&[]) {
        Ok(total) => println!("  ✅ Total: {}", total),
        Err(e) => println!("  ❌ Error: {}", e),
    }

    // Test async function without Result
    println!("\nTesting async function without Result...");
    let length = process_data("Hello, telemetry!").await;
    println!("  ✅ Processed {} characters", length);

    println!("\n✅ All tests completed!");
    println!("\nNote: The #[instrument] macro currently measures execution time");
    println!("      but doesn't send telemetry yet. Full integration coming soon!");

    Ok(())
}

#[cfg(not(feature = "macros"))]
fn main() {
    eprintln!("This example requires the 'macros' feature.");
    eprintln!("Run with: cargo run --example instrument_macro --features macros");
    std::process::exit(1);
}
