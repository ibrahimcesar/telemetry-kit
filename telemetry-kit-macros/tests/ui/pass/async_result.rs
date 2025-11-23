//! Test: Async function returning Result compiles

use telemetry_kit_macros::instrument;

#[instrument]
async fn fetch_data(url: &str) -> Result<String, String> {
    if url.is_empty() {
        Err("empty URL".to_string())
    } else {
        Ok(format!("data from {}", url))
    }
}

fn main() {}
