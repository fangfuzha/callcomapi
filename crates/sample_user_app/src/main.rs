use anyhow::{Context, Result};
use callcomapi::prelude::{com_thread, with_com};
use std::thread;
use windows::Win32::System::Com::{CLSCTX_INPROC_SERVER, CoCreateInstance};
use windows::Win32::System::Wmi::{IWbemLocator, WbemLocator};

/// This is a sample user application to simulate how an external developer
/// might use the `callcomapi` library in their project.
#[tokio::main]
async fn main() -> Result<()> {
    println!("Starting user simulation application...");
    println!("Main thread ID: {:?}", thread::current().id());

    // Scenario 1: Using #[with_com] on a function
    println!("\n--- Scenario 1: Using #[with_com] ---");
    run_with_com_example().context("Failed scenario 1")?;

    // Scenario 2: Using #[com_thread] for background tasks
    println!("\n--- Scenario 2: Using #[com_thread] ---");
    let (res, tid) = background_task(42).await;
    println!("Background task result: {}, run on thread: {:?}", res, tid);

    let (res2, tid2) = background_task(100).await;
    println!("Second call result: {}, run on thread: {:?}", res2, tid2);

    if tid == tid2 {
        println!("Success: Both background tasks reused the same COM-initialized thread.");
    }

    println!("\nSimulation complete.");
    Ok(())
}

#[with_com("MTA")]
fn run_with_com_example() -> Result<()> {
    println!(
        "Inside #[with_com(MTA)] function, thread ID: {:?}",
        thread::current().id()
    );

    // Simulate COM call
    unsafe {
        let _locator: IWbemLocator = CoCreateInstance(&WbemLocator, None, CLSCTX_INPROC_SERVER)
            .context("Failed to create WbemLocator")?;
        println!("WbemLocator created successfully within #[with_com] context.");
    }

    Ok(())
}

#[com_thread]
async fn background_task(val: i32) -> (i32, thread::ThreadId) {
    // This runs on a dedicated COM-initialized background thread
    let current_tid = thread::current().id();
    (val * 2, current_tid)
}
