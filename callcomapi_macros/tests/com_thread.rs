use callcomapi_macros::com_thread;

mod common;

#[com_thread]
fn sync_fn1(x: i32) -> (i32, std::thread::ThreadId) {
    common::call_com_api().unwrap();
    (x, std::thread::current().id())
}

#[com_thread]
fn sync_fn2(x: i32) -> (i32, std::thread::ThreadId) {
    common::call_com_api().unwrap();
    (x, std::thread::current().id())
}

#[test]
fn test_com_thread_sync() {
    let (v1, t1) = sync_fn1(1);
    let (v2, t2) = sync_fn2(2);
    assert_eq!(v1, 1);
    assert_eq!(v2, 2);
    assert_eq!(t1, t2);
}

#[com_thread]
async fn async_fn1(x: i32) -> (i32, std::thread::ThreadId) {
    common::call_com_api().unwrap();
    (x, std::thread::current().id())
}
#[com_thread]
async fn async_fn2(x: i32) -> (i32, std::thread::ThreadId) {
    common::call_com_api().unwrap();
    (x, std::thread::current().id())
}

#[tokio::test]
async fn test_com_thread_async() {
    let (v1, t1) = async_fn1(1).await;
    let (v2, t2) = async_fn2(2).await;
    assert_eq!(v1, 1);
    assert_eq!(v2, 2);
    assert_eq!(t1, t2);
}

#[com_thread(MTA)]
fn sync_fn_mta(x: i32) -> (i32, std::thread::ThreadId) {
    common::call_com_api().unwrap();
    (x, std::thread::current().id())
}

#[test]
fn test_com_thread_sync_mta() {
    let (v1, t1) = sync_fn_mta(1);
    let (v2, t2) = sync_fn_mta(2);
    assert_eq!(v1, 1);
    assert_eq!(v2, 2);
    assert_eq!(t1, t2);
}

#[com_thread(MTA)]
async fn async_fn_mta(x: i32) -> (i32, std::thread::ThreadId) {
    common::call_com_api().unwrap();
    (x, std::thread::current().id())
}

#[tokio::test]
async fn test_com_thread_async_mta() {
    let (v1, t1) = async_fn_mta(1).await;
    let (v2, t2) = async_fn_mta(2).await;
    assert_eq!(v1, 1);
    assert_eq!(v2, 2);
    assert_eq!(t1, t2);
}
// Multi-parameter function tests
#[com_thread]
fn sync_fn_multi(x: i32, y: i32, z: String) -> (i32, i32, String, std::thread::ThreadId) {
    common::call_com_api().unwrap();
    (x, y, z, std::thread::current().id())
}

#[test]
fn test_com_thread_sync_multi() {
    let (a, b, s, t1) = sync_fn_multi(10, 20, "hello".to_string());
    let (_, _, _, t2) = sync_fn_multi(30, 40, "world".to_string());
    assert_eq!(a, 10);
    assert_eq!(b, 20);
    assert_eq!(s, "hello");
    assert_eq!(t1, t2);
}

#[com_thread]
async fn async_fn_multi(x: i32, y: String) -> (i32, String, std::thread::ThreadId) {
    common::call_com_api().unwrap();
    (x, y, std::thread::current().id())
}

#[tokio::test]
async fn test_com_thread_async_multi() {
    let (a, s, t1) = async_fn_multi(99, "async".to_string()).await;
    let (_, _, t2) = async_fn_multi(100, "test".to_string()).await;
    assert_eq!(a, 99);
    assert_eq!(s, "async");
    assert_eq!(t1, t2);
}
