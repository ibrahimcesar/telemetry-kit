//! Procedural macros for telemetry-kit
//!
//! This crate provides the `#[instrument]` macro for automatic function instrumentation.

use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, ItemFn, ReturnType};

/// Automatically instrument a function with telemetry tracking
///
/// This macro wraps a function to track:
/// - Execution duration
/// - Success/failure (for functions returning Result)
/// - Function name as the command/feature name
///
/// # Examples
///
/// ```rust,ignore
/// use telemetry_kit::instrument;
///
/// #[instrument]
/// async fn fetch_data(url: &str) -> Result<Data, Error> {
///     // Function body
/// }
///
/// #[instrument]
/// fn process_data(data: &Data) -> Result<(), Error> {
///     // Function body
/// }
/// ```
///
/// # Generated Code
///
/// The macro generates code that:
/// 1. Records the start time
/// 2. Executes the original function
/// 3. Calculates duration
/// 4. Tracks the command with telemetry-kit
/// 5. Returns the original result
///
/// For async functions, it wraps the future appropriately.
#[proc_macro_attribute]
pub fn instrument(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as ItemFn);

    let attrs = &input.attrs;
    let vis = &input.vis;
    let sig = &input.sig;
    let block = &input.block;
    let _fn_name = &sig.ident;
    let _fn_name_str = _fn_name.to_string();

    // Check if function is async
    let is_async = sig.asyncness.is_some();

    // Check if function returns Result
    let _returns_result = match &sig.output {
        ReturnType::Type(_, ty) => {
            if let syn::Type::Path(type_path) = &**ty {
                type_path.path.segments.last()
                    .map(|seg| seg.ident == "Result")
                    .unwrap_or(false)
            } else {
                false
            }
        }
        _ => false,
    };

    // Generate the instrumented function
    // Note: Currently this macro only wraps the function and measures timing
    // without sending telemetry. Full integration will be added in a future version
    // when we have a proper global telemetry instance pattern.
    let instrumented = if is_async {
        // Async function - just wrap and measure
        quote! {
            #(#attrs)*
            #vis #sig {
                let __start = ::std::time::Instant::now();
                let __result = async move #block.await;
                let _duration = __start.elapsed();

                // TODO: Send telemetry when global instance is available
                // For now, timing is measured but not recorded

                __result
            }
        }
    } else {
        // Sync function - just wrap and measure
        quote! {
            #(#attrs)*
            #vis #sig {
                let __start = ::std::time::Instant::now();
                let __result = (|| #block)();
                let _duration = __start.elapsed();

                // TODO: Send telemetry when global instance is available
                // For now, timing is measured but not recorded

                __result
            }
        }
    };

    TokenStream::from(instrumented)
}
