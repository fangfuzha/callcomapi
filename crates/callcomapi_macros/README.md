# callcomapi_macros

> **注意**：这是 `callcomapi` 的内部过程宏 crate。通常情况下，你应该依赖更高层级的 [callcomapi](https://crates.io/crates/callcomapi) crate。

`callcomapi_macros` 提供了 `#[with_com]` 和 `#[com_thread]` 宏，以简化 Windows COM 的开发工作。

## 宏

- `#[with_com]`：在函数作用域周围执行 `CoInitializeEx` 和 `CoUninitialize`。
- `#[com_thread]`：将函数分发到专用的后台 COM 线程执行；支持同步和异步函数。
