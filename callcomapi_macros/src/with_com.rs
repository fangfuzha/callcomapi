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
        quote! { windows::Win32::System::Com::COINIT_APARTMENTTHREADED }
    } else {
        let lit = parse_macro_input!(attr as syn::LitStr);
        let s = lit.value();
        match s.to_uppercase().as_str() {
            "MTA" | "MULTI" | "MULTITHREADED" => {
                // Multi-threaded apartment
                quote! { windows::Win32::System::Com::COINIT_MULTITHREADED }
            }
            "STA" | "APARTMENT" | "APARTMENTTHREADED" => {
                // Single-threaded apartment
                quote! { windows::Win32::System::Com::COINIT_APARTMENTTHREADED }
            }
            _ => {
                // No support for custom constant paths: treat unknown values as STA
                quote! { windows::Win32::System::Com::COINIT_APARTMENTTHREADED }
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
            // Initialize COM (Parameter 1: NULL, Parameter 2: threading model)
            // Both S_OK and S_FALSE are considered success; ignore error codes
            unsafe {
                let _ = windows::Win32::System::Com::CoInitializeEx(
                    None,
                    #model,
                );
            }

            // Define a local Guard struct that implements Drop for COM cleanup
            struct __ComGuard;
            impl Drop for __ComGuard {
                fn drop(&mut self) {
                    // Cleanup COM, revoke initialization
                    unsafe { windows::Win32::System::Com::CoUninitialize(); }
                }
            }

            // Create guard instance (bound to this scope)
            // Drop is automatically called when function returns or panics
            let _com_guard = __ComGuard;

            // Execute original function body (COM is now initialized and guard is in scope)
            #block
        }
    };

    expanded.into()
}
