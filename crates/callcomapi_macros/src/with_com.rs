use proc_macro::TokenStream;
use quote::quote;
use syn::{ItemFn, parse_macro_input};

/// Implementation of the COM initialization/cleanup attribute macro
///
/// Implementation flow:
/// 1. Parse the attribute parameter and determine the threading model (MTA or STA)
/// 2. Extract function signature and function body
/// 3. Generate wrapper code:
///    a. CoInitializeEx initializes COM
///    b. RAII Guard ensures cleanup
///    c. Execute original function body
pub fn inner_with_com(attr: TokenStream, item: TokenStream) -> TokenStream {
    // Step 1: Parse the threading model attribute
    // Supported formats: no param (default STA), "MTA", "STA", or full path
    let model = if attr.is_empty() {
        // Default: single-threaded apartment
        quote! { ::callcomapi_runtime::ComModel::STA }
    } else {
        let lit = parse_macro_input!(attr as syn::LitStr);
        let s = lit.value();
        match s.to_uppercase().as_str() {
            "MTA" | "MULTI" | "MULTITHREADED" => {
                // Multi-threaded apartment
                quote! { ::callcomapi_runtime::ComModel::MTA }
            }
            "STA" | "APARTMENT" | "APARTMENTTHREADED" => {
                // Single-threaded apartment
                quote! { ::callcomapi_runtime::ComModel::STA }
            }
            _ => {
                // No support for custom constant paths: treat unknown values as STA
                quote! { ::callcomapi_runtime::ComModel::STA }
            }
        }
    };

    // Step 2: Parse function definition
    let func = parse_macro_input!(item as ItemFn);
    let vis = &func.vis;
    let sig = &func.sig;
    let block = &func.block;

    // Step 3: Generate wrapper code
    // Uses Guard struct to implement RAII pattern, ensuring COM cleanup even on panic/drop
    let expanded = quote! {
        #vis #sig {
            // Initialize COM and get a guard that uninitializes on drop
            let _com_guard = unsafe {
                ::callcomapi_runtime::init_com(#model)
            };

            // Execute original function body
            #block
        }
    };

    expanded.into()
}
