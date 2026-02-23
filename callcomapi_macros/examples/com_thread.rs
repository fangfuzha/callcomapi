use callcomapi_macros::com_thread;
use std::thread;

/// Demonstrates the `#[com_thread]` macro. It creates a background thread on first call,
/// initializes COM in that thread, and reuses the thread for subsequent calls.
#[tokio::main]
async fn main() {
    println!("Main thread ID: {:?}", thread::current().id());

    // 1. Run a synchronous function on the background thread
    let result1 = sync_task(10);
    println!(
        "Task 1 result (sync): {}, thread ID: {:?}",
        result1.0, result1.1
    );

    // 2. Call again; it should use the same background thread
    let result2 = sync_task(20);
    println!(
        "Task 2 result (sync): {}, thread ID: {:?}",
        result2.0, result2.1
    );

    if result1.1 == result2.1 {
        println!("Check: both tasks ran on the same background thread.");
    }

    // 3. Run an asynchronous function on the background thread
    let (v, tid) = async_task("hello async".to_string()).await;
    println!("Task 3 result (async): '{}', thread ID: {:?}", v, tid);

    if tid == result1.1 {
        println!("Check: async task also used the same background thread.");
    }

    println!("All functions running on background threads have COM initialized automatically.");
}

#[com_thread]
fn sync_task(val: i32) -> (i32, thread::ThreadId) {
    // Call any COM APIs here
    (val * 2, thread::current().id())
}

#[com_thread(MTA)]
async fn async_task(s: String) -> (String, thread::ThreadId) {
    // Specify MTA run mode
    (s.to_uppercase(), thread::current().id())
}
