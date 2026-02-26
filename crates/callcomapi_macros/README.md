# callcomapi_macros

> **Note**: This is the internal procedural macro crate for `callcomapi`. You should normally
> depend on the higher-level [callcomapi](https://crates.io/crates/callcomapi) crate.

`callcomapi_macros` provides the `#[with_com]` and `#[com_thread]` macros to simplify working
with Windows COM.

## Macros

- `#[with_com]`: performs `CoInitializeEx` and `CoUninitialize` around a function scope.
- `#[com_thread]`: dispatches a function to a dedicated background COM thread; supports both
  synchronous and asynchronous functions.
