//! COM attribute macro library providing convenient integration between Rust and Windows COM.
//!
//! # Macro Overview
//!
//! This library provides two main attribute macros for simplifying COM initialization,
//! thread management, and resource cleanup:
//!
//! ## `with_com` - Function-level COM initialization
//!
//! Automatically initializes and cleans up COM for the duration of a function.
//!
//! ### Usage
//!
//! ```ignore
//! // Default STA mode
//! #[with_com]
//! fn my_function() {
//!     // COM is initialized here
//!     // ... COM calls ...
//! }
//!
//! // Explicitly specify MTA mode
//! #[with_com("MTA")]
//! fn mta_function() {
//!     // COM initialized in multi-threaded apartment mode
//! }
//! ```
//!
//! ### Threading Model Parameter
//!
//! - `#[with_com]` or `#[with_com("STA")]` - Single-threaded apartment (default)
//! - `#[with_com("MTA")]` - Multi-threaded apartment
//! - Supports full path: `#[with_com("windows::Win32::System::Com::COINIT_APARTMENTTHREADED")]`
//!
//! ## `com_thread` - Background thread COM operations
//!
//! Transforms a function to execute on a background thread with COM initialized.
//! Uses channels for inter-thread communication.
//! The thread is created on first call and reused for subsequent calls.
//!
//! ### Usage
//!
//! ```ignore
//! // Sync function - default STA mode
//! #[com_thread]
//! fn sync_com_operation(param: i32) -> i32 {
//!     // Executes on background thread (COM initialized)
//!     param * 2
//! }
//!
//! // Async function - MTA mode
//! #[com_thread(MTA)]
//! async fn async_com_operation(param: i32) -> i32 {
//!     // Executes on background thread (MTA mode)
//!     param * 2
//! }
//! ```
//!
//! ### Threading Model Parameter
//!
//! - `#[com_thread]` or `#[com_thread(STA)]` - Single-threaded apartment (default)
//! - `#[com_thread(MTA)]` - Multi-threaded apartment
//!
//! ### Workflow
//!
//! 1. **First call**: Spawns background thread, initializes COM, establishes message channel
//! 2. **Parameter passing**: Sends parameters and response channel via message channel
//! 3. **Execution**: Background thread executes function body
//! 4. **Result return**: Returns result via response channel
//! 5. **Thread reuse**: Subsequent calls reuse the same background thread

use proc_macro::TokenStream;

mod com_thread;
mod with_com;

// Wrapper at crate root because proc-macro attributes must be defined at root
/// Initialize COM, execute function, cleanup COM
///
/// Optional parameter specifies threading model: `"MTA"` or `"STA"` (default).
/// Uses RAII pattern to ensure proper cleanup even if the function panics.
///
/// See module documentation for details.
#[proc_macro_attribute]
pub fn with_com(attr: TokenStream, item: TokenStream) -> TokenStream {
    with_com::inner_with_com(attr, item)
}

/// Transform function to execute as a COM operation on a background thread
///
/// Creates a dedicated background thread on first call with COM initialized.
/// Subsequent calls reuse the thread via message channels.
/// Supports both sync and async functions.
///
/// Optional parameter specifies threading model: `"MTA"` or `"STA"` (default).
///
/// See module documentation for details.
#[proc_macro_attribute]
pub fn com_thread(attr: TokenStream, item: TokenStream) -> TokenStream {
    com_thread::inner_com_thread(attr, item)
}
