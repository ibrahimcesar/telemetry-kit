//! Test: Sync function without Result compiles

use telemetry_kit_macros::instrument;

#[instrument]
fn calculate(x: i32, y: i32) -> i32 {
    x + y
}

fn main() {}
