//! Facade crate for `callcomapi`.
//!
//! This crate provides a unified entry point for using COM API macros and utilities.

pub use callcomapi_macros::{com_thread, with_com};

/// Common types and traits for COM operations.
pub mod prelude {
    pub use crate::com_thread;
    pub use crate::with_com;
}
