//! Test: Function with generics compiles

use telemetry_kit_macros::instrument;

#[instrument]
async fn process<T: std::fmt::Display>(value: T) -> Result<String, String> {
    Ok(format!("{}", value))
}

fn main() {}
