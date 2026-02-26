# callcomapi

callcomapi is a small library that makes it easier to call Windows COM APIs from Rust. It
provides a single facade crate that automatically handles COM initialization, uninitialization,
and thread management via procedural macros.

## Key features

- **callcomapi** – the single entry-point crate that exports all macros and runtime support.
- `#[with_com]` – automatically initializes/uninitializes COM for the duration of a function.
- `#[com_thread]` – run sync or async functions on a dedicated background COM thread.

## Quick start

1. Add the dependency to your `Cargo.toml`:

```toml
[dependencies]
callcomapi = { version = "0.1" }
```

2. Use it in code:

```rust
use callcomapi::prelude::*;

#[with_com]
fn example_with_com() {
    // COM is initialized automatically for the scope of this function
}

#[com_thread]
fn run_on_com_thread(x: i32) -> i32 {
    // runs on a background COM thread (STA by default)
    x * 2
}

#[com_thread(MTA)]
async fn run_on_mta_thread(x: String) -> String {
    // runs on a background MTA thread
    x.to_uppercase()
}
```

## Design advantages

- **Single import**: just depend on `callcomapi` and you get macros and runtime without pulling in
  multiple crates.
- **Less boilerplate**: automatic COM lifecycle management.
- **Thread handling**: centralized control of background COM threads ensures tasks run in the correct
  apartment model.
- `callcomapi_runtime` keeps a tiny pool (one thread per apartment) and dispatches tasks through channels.
- Tasks must be `Send + 'static` since arguments/returns cross thread boundaries.
- The runtime retries once if a COM thread unexpectedly exits and recreates it.

### Building and testing

From the repository root:

```powershell
# build the workspace
cargo build

# run tests for the macro crate
cargo test -p callcomapi_macros
```

### Notes and future work

- This repo is intentionally minimal and Windows-specific. It uses the `windows` crate for COM
  APIs.
- For more robust error handling (avoiding panics on send failures), the runtime APIs could be changed to return a `Result` and the macros updated accordingly.

## License

MIT
