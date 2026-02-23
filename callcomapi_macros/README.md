# callcomapi_macros

`callcomapi_macros` is a Rust procedural macro crate that simplifies calling COM APIs. It reduces the boilerplate for Windows COM initialization and cleanup.

This crate is intended for Rust projects that need to call Windows COM APIs frequently.

## Usage

Add `callcomapi_macros` as a dependency in your `Cargo.toml`,
Then you can use the provided macros in your Rust code.

```rust
use callcomapi_macros::{with_com, com_thread};

// 1. Automatically initialize COM and ensure cleanup when the function returns
#[with_com]
fn my_com_function() {
    // Call COM APIs here
}

// 2. Convert a synchronous or asynchronous function to run on a dedicated
//    background COM thread that stays alive and is reused.
#[com_thread]
fn run_on_com_thread(x: i32) -> i32 {
    x * 2
}
```

## 示例

You can check the [examples](examples/) folder for more usage details:

- [with_com.rs](examples/with_com.rs): demonstrates scoping COM lifetime within a function.
- [com_thread.rs](examples/com_thread.rs): demonstrates using a persistent background thread to call COM.
