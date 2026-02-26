# callcomapi

`callcomapi` 是一个简化从 Rust 调用 Windows COM API 的库。它提供了一个外观 crate，通过过程宏自动处理 COM 的初始化、反初始化以及线程管理。

## 主要特性

- **callcomapi** – 统一入口 crate，导出所有宏和运行时支持。
- `#[with_com]` – 在函数执行期间自动初始化/反初始化 COM。
- `#[com_thread]` – 在专用的后台 COM 线程上运行同步或异步函数。

## 快速开始

1. 在 `Cargo.toml` 中添加依赖：

```toml
[dependencies]
callcomapi = { version = "0.1" }
```

2. 在代码中使用：

```rust
use callcomapi::prelude::*;

#[with_com]
fn example_with_com() {
    // COM 将在此函数作用域内自动初始化
}

#[com_thread]
fn run_on_com_thread(x: i32) -> i32 {
    // 在后台 COM 线程运行（默认为 STA）
    x * 2
}

#[com_thread(MTA)]
async fn run_on_mta_thread(x: String) -> String {
    // 在后台 MTA 线程运行
    x.to_uppercase()
}
```

## 设计优势

- **单一导入**：只需依赖 `callcomapi` 即可获得宏和运行时，无需引入多个 crate。
- **减少样板代码**：自动管理 COM 生命周期。
- **线程处理**：对后台 COM 线程的集中控制，确保任务在正确的套间模型（Apartment Model）中运行。
- `callcomapi_runtime` 为每个套间模型维持一个小规模的线程池（每个模型一个线程），并通过通道分发任务。
- 任务必须满足 `Send + 'static` 约束，因为参数和返回值需要跨线程边界移动。
- 如果 COM 线程意外退出，运行时会尝试重新创建线程并重试一次任务发送。

### 构建与测试

在仓库根目录下执行：

```powershell
# 构建整个工作区
cargo build

# 运行宏 crate 的测试
cargo test -p callcomapi_macros
```

### 说明与后续工作

- 本仓库专注于 Windows 平台下的 COM API，使用 `windows` crate 实现。
- 为了实现更健壮的错误处理（避免发送失败时发生 panic），后续可以将运行时 API 修改为返回 `Result` 并相应调整宏实现。

## 开源协议

MIT
