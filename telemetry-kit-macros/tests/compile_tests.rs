//! Compile-time tests for #[instrument] macro
//!
//! These tests verify that the macro generates valid Rust code
//! that compiles successfully.

#[test]
fn test_macro_compilation() {
    let t = trybuild::TestCases::new();
    t.pass("tests/ui/pass/*.rs");
}
