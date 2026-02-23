# callcomapi

callcomapi is a small workspace that provides two cooperating crates to simplify calling Windows COM APIs from Rust:

- `callcomapi_macros` — a procedural macro crate providing `#[with_com]` and `#[com_thread]` helpers to automatically initialize/uninitialize COM and run functions on a dedicated COM thread.
- `callcomapi_runtime` — a runtime helper library used by the macros that manages background COM threads, message passing, and task execution.

Goals

- Reduce boilerplate for COM initialization and cleanup.
- Provide a safe, ergonomic way to run both sync and async code on threads pre-initialized for a target COM apartment model (STA/MTA).
- Centralize thread/handle management in a runtime crate so macros remain lightweight.

Quick start

1. Add the crates as workspace members (already done in this repo). From your project include the macros crate as a dependency in `Cargo.toml`.

2. Use the macros in your code:

```rust
use callcomapi_macros::{with_com, com_thread};

#[with_com]
fn example_with_com() {
    // COM is initialized Automatically for the duration of this function
}

#[com_thread]
fn run_on_com_thread(x: i32) -> i32 {
    // Runs on a background thread with COM initialized (default STA)
    x * 2
}

#[com_thread(MTA)]
async fn run_on_mta_thread(x: String) -> String {
    // Runs on a background thread initialized for MTA
    x.to_uppercase()
}
```

Examples

See the `callcomapi_macros/examples` folder for working examples:

- `with_com.rs` — shows using `#[with_com]` to scope COM lifetime.
- `com_thread.rs` — demonstrates `#[com_thread]` for sync and async functions and different apartment models.

Design notes

- Macros generate lightweight wrappers that delegate execution to `callcomapi_runtime`.
- `callcomapi_runtime` maintains a small pool (one thread per COM model) and delivers tasks via channels.
- Tasks must be `Send + 'static` because parameters and return values move across thread boundaries.
- Runtime retries sending a task once if the background thread has died unexpectedly and recreates the thread.

Building and testing

From the repository root:

```powershell
# build workspace
cargo build

# run tests for the macro crate
cargo test -p callcomapi_macros
```

Notes and next steps

- The repository aims to be minimal and platform-specific (Windows COM). The `windows` crate is used for COM APIs.
- If you want more robust error handling (avoid panics on send failures), we can change the runtime APIs to return `Result` and adjust the macros accordingly.

License

MIT
