# callcomapi

`callcomapi` is a crate that simplifies calling Windows COM APIs from Rust. It provides
procedural macros that manage COM initialization, uninitialization, and thread dispatch.

## Features

- **Single dependency**: import only `callcomapi` to get macros and runtime support.
- `#[with_com]` – initialize and uninitialize COM around a function.
- `#[com_thread]` – schedule functions on a dedicated background COM thread, for both sync and
  async code.

## Quick start

Add to `Cargo.toml`:

```toml
[dependencies]
callcomapi = "0.1"
```

Example usage:

```rust
use callcomapi::prelude::*;

#[with_com]
fn example() {
    // COM is initialized automatically and uninitialized on scope exit.
}

#[com_thread]
fn on_bg_thread() {
    // executed on a dedicated STA thread.
}
```

## Examples

See the `examples/` directory:

- `with_com.rs`: basic usage
- `com_thread.rs`: cross-thread invocation
