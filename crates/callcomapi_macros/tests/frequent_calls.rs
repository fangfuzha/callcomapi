use callcomapi_macros::{com_thread, with_com};
use callcomapi_runtime::{ComModel, init_com};
use std::time::Instant;

mod common;

/// helper used by the `#[with_com]` section
#[with_com]
fn macro_with() {
    common::call_com_api().unwrap();
}

/// helper used by the `#[com_thread]` section (sync)
#[com_thread]
fn macro_thread() -> std::thread::ThreadId {
    common::call_com_api().unwrap();
    std::thread::current().id()
}

/// helper used by `#[com_thread]` for async measurement
#[com_thread]
async fn macro_thread_async() -> std::thread::ThreadId {
    common::call_com_api().unwrap();
    std::thread::current().id()
}

/// 性能对比：手动初始化/卸载 vs `#[with_com]` vs `#[com_thread]`。
///
/// 这个测试仅用于观察相对耗时。每个阶段都会执行大量轻量 COM 调用并测量用时，
/// 并在终端输出结果。并非精确基准，仅供参考。
#[test]
fn frequent_calls_comparison() {
    const ITER: usize = 1_00;

    // 1. 手动初始化和卸载
    let start = Instant::now();
    for _ in 0..ITER {
        let _guard = unsafe { init_com(ComModel::STA) };
        common::call_com_api().unwrap();
    }
    let manual_dur = start.elapsed();

    // 2. 使用 #[with_com] 宏
    let start = Instant::now();
    for _ in 0..ITER {
        macro_with();
    }
    let with_com_dur = start.elapsed();

    // 3a. 使用 #[com_thread] 宏（同步版本）
    let start = Instant::now();
    let mut first_tid = None;
    for _ in 0..ITER {
        let tid = macro_thread();
        if let Some(ft) = first_tid {
            assert_eq!(tid, ft, "com_thread 应该在相同线程上执行");
        } else {
            first_tid = Some(tid);
        }
    }
    let thread_sync_dur = start.elapsed();

    // 3b. 使用 #[com_thread] 宏（异步版本）
    let start = Instant::now();
    let mut first_tid_async = None;
    for _ in 0..ITER {
        let tid = futures::executor::block_on(macro_thread_async());
        if let Some(ft) = first_tid_async {
            assert_eq!(tid, ft, "com_thread async 应该在相同线程上执行");
        } else {
            first_tid_async = Some(tid);
        }
    }
    let thread_async_dur = start.elapsed();

    println!(
        "manual init: {:?}\nwith_com: {:?}\ncom_thread(sync): {:?}\ncom_thread(async): {:?}",
        manual_dur, with_com_dur, thread_sync_dur, thread_async_dur
    );
}
