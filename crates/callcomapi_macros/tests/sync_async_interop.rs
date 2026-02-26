use callcomapi_macros::com_thread;
use std::thread;

mod common;

#[com_thread]
fn sync_com_task(val: i32) -> (i32, thread::ThreadId) {
    // verify COM is initialized
    common::call_com_api().expect("COM should be initialized in sync task");
    (val * 2, thread::current().id())
}

#[com_thread]
async fn async_com_task(val: i32) -> (i32, thread::ThreadId) {
    // verify COM is initialized
    common::call_com_api().expect("COM should be initialized in async task");
    (val + 10, thread::current().id())
}

#[tokio::test]
async fn test_sync_then_async_interop() {
    println!("Main thread ID: {:?}", thread::current().id());

    // 1. Execute synchronous macro function
    let (res_sync, tid_sync) = sync_com_task(5);
    assert_eq!(res_sync, 10);
    println!("Sync task finished on thread: {:?}", tid_sync);

    // 2. Execute asynchronous macro function
    let (res_async, tid_async) = async_com_task(5).await;
    assert_eq!(res_async, 15);
    println!("Async task finished on thread: {:?}", tid_async);

    // Runtime allocates one background thread per COM model (STA/MTA).
    // Both sync and async tasks with the same model share the same thread.
    assert_eq!(
        tid_sync, tid_async,
        "Sync and async tasks with the same COM model should run on the same background thread"
    );
    assert_ne!(
        tid_sync,
        thread::current().id(),
        "Sync task should not run on main thread"
    );
    assert_ne!(
        tid_async,
        thread::current().id(),
        "Async task should not run on main thread"
    );
}

#[tokio::test]
async fn test_multiple_interleaved_calls() {
    // Interleaved calls to verify stability
    for i in 0..3 {
        let (s_v, _) = sync_com_task(i);
        let (a_v, _) = async_com_task(i).await;
        assert_eq!(s_v, i * 2);
        assert_eq!(a_v, i + 10);
    }
}
