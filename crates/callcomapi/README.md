# callcomapi

`callcomapi` 是一个简化从 Rust 调用 Windows COM API 的库。它提供了一些过程宏，用于管理 COM 的初始化、反初始化以及线程分发。

## 特性

- **单一依赖**：只需导入 `callcomapi` 即可获得宏和运行时支持。
- `#[with_com]` – 在函数周围初始化和反初始化 COM。
- `#[com_thread]` – 将函数调度到专用的后台 COM 线程上执行，支持同步和异步代码。

## 快速开始

在 `Cargo.toml` 中添加：

```toml
[dependencies]
callcomapi = "0.1"
```

使用示例：

```rust
use callcomapi::prelude::*;

#[with_com]
fn example() {
    // COM 将自动初始化，并在退出作用域时反初始化。
}

#[com_thread]
fn on_bg_thread() {
    // 在专用的 STA(默认) 线程(可重用)上执行。
}
```

## 示例

请参阅 `examples/` 目录：

- `with_com.rs`: 基础用法
- `com_thread.rs`: 跨线程调用
