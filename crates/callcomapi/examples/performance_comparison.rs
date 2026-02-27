use callcomapi::{ComModel, com_thread, init_com, with_com};
use std::time::Instant;
use windows::Win32::System::Com::{
    CLSCTX_INPROC_SERVER, CoCreateInstance, CoSetProxyBlanket, EOAC_NONE, RPC_C_AUTHN_LEVEL_CALL,
    RPC_C_IMP_LEVEL_IMPERSONATE,
};
use windows::Win32::System::Rpc::{RPC_C_AUTHN_WINNT, RPC_C_AUTHZ_NONE};
use windows::Win32::System::Wmi::{
    IWbemClassObject, IWbemLocator, WBEM_FLAG_FORWARD_ONLY, WBEM_FLAG_RETURN_IMMEDIATELY,
    WBEM_GENERIC_FLAG_TYPE, WBEM_INFINITE, WbemLocator,
};
use windows::core::{BSTR, Result};

/// helper that does a real COM operation to ensure COM is initialized
pub fn call_com_api() -> Result<()> {
    unsafe {
        let locator: IWbemLocator = CoCreateInstance(&WbemLocator, None, CLSCTX_INPROC_SERVER)?;
        let services = locator.ConnectServer(
            &BSTR::from("ROOT\\CIMV2"),
            &BSTR::new(),
            &BSTR::new(),
            &BSTR::new(),
            0,
            &BSTR::new(),
            None,
        )?;

        CoSetProxyBlanket(
            &services,
            RPC_C_AUTHN_WINNT,
            RPC_C_AUTHZ_NONE,
            None,
            RPC_C_AUTHN_LEVEL_CALL,
            RPC_C_IMP_LEVEL_IMPERSONATE,
            None,
            EOAC_NONE,
        )?;

        let enumerator = services.ExecQuery(
            &BSTR::from("WQL"),
            &BSTR::from("SELECT Name FROM Win32_Processor"),
            WBEM_GENERIC_FLAG_TYPE(
                (WBEM_FLAG_FORWARD_ONLY.0 | WBEM_FLAG_RETURN_IMMEDIATELY.0) as i32,
            ),
            None,
        )?;

        let mut cpu_objects: [Option<IWbemClassObject>; 1] = [None];
        let mut returned_count: u32 = 0;
        enumerator
            .Next(WBEM_INFINITE as i32, &mut cpu_objects, &mut returned_count)
            .ok()?;

        // To keep the benchmark clean, we don't print on every call here
        if returned_count > 0 {
            let _ = cpu_objects[0].take();
        }
    }
    Ok(())
}

/// helper used by the `#[with_com]` section
#[with_com]
fn macro_with() {
    call_com_api().unwrap();
}

/// async helper used by `#[with_com]` to measure its async performance
#[with_com]
async fn macro_with_async() {
    call_com_api().unwrap();
}

/// helper used by the `#[com_thread]` section (sync)
#[com_thread]
fn macro_thread() -> std::thread::ThreadId {
    call_com_api().unwrap();
    std::thread::current().id()
}

/// helper used by the `#[with_com]` section (sync, MTA)
#[with_com("MTA")]
fn macro_with_mta() {
    call_com_api().unwrap();
}

/// async helper used by `#[with_com]` (MTA)
#[with_com("MTA")]
async fn macro_with_async_mta() {
    call_com_api().unwrap();
}

/// helper used by the `#[com_thread]` section (sync, MTA)
#[com_thread(MTA)]
fn macro_thread_mta() -> std::thread::ThreadId {
    call_com_api().unwrap();
    std::thread::current().id()
}

/// async helper used by `#[com_thread]` (MTA)
#[com_thread(MTA)]
async fn macro_thread_async_mta() -> std::thread::ThreadId {
    call_com_api().unwrap();
    std::thread::current().id()
}

/// helper used by `#[com_thread]` for async measurement
#[com_thread]
async fn macro_thread_async() -> std::thread::ThreadId {
    call_com_api().unwrap();
    std::thread::current().id()
}

fn main() {
    const ITER: usize = 1_00;
    println!("Starting performance comparison (ITER={})...", ITER);

    // 1a. 手动初始化一次, 在循环后卸载
    let start = Instant::now();
    let _guard = unsafe { init_com(ComModel::STA) };
    for _ in 0..ITER {
        call_com_api().unwrap();
    }
    drop(_guard);
    let single_init_dur = start.elapsed();

    // 1b. 手动初始化和卸载（每次都初始化）
    let start = Instant::now();
    for _ in 0..ITER {
        let _guard = unsafe { init_com(ComModel::STA) };
        call_com_api().unwrap();
    }
    let manual_dur = start.elapsed();

    // 1c. 手动初始化一次 (MTA)
    let start = Instant::now();
    let _guard = unsafe { init_com(ComModel::MTA) };
    for _ in 0..ITER {
        call_com_api().unwrap();
    }
    drop(_guard);
    let single_init_mta_dur = start.elapsed();

    // 1d. 手动初始化和卸载 (每次都初始化 MTA)
    let start = Instant::now();
    for _ in 0..ITER {
        let _guard = unsafe { init_com(ComModel::MTA) };
        call_com_api().unwrap();
    }
    let manual_mta_dur = start.elapsed();

    // 2a. 使用 #[with_com] 宏（同步版本）
    let start = Instant::now();
    for _ in 0..ITER {
        macro_with();
    }
    let with_com_sync_dur = start.elapsed();

    // 2b. 使用 #[with_com] 宏（异步版本）
    let start = Instant::now();
    for _ in 0..ITER {
        futures::executor::block_on(macro_with_async());
    }
    let with_com_async_dur = start.elapsed();

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

    // 4. MTA 模式对比
    // 4a. with_com(MTA sync)
    let start = Instant::now();
    for _ in 0..ITER {
        macro_with_mta();
    }
    let with_com_sync_mta_dur = start.elapsed();

    // 4b. with_com(MTA async)
    let start = Instant::now();
    for _ in 0..ITER {
        futures::executor::block_on(macro_with_async_mta());
    }
    let with_com_async_mta_dur = start.elapsed();

    // 4c. com_thread(MTA sync)
    let start = Instant::now();
    for _ in 0..ITER {
        macro_thread_mta();
    }
    let thread_sync_mta_dur = start.elapsed();

    // 4d. com_thread(MTA async)
    let start = Instant::now();
    for _ in 0..ITER {
        futures::executor::block_on(macro_thread_async_mta());
    }
    let thread_async_mta_dur = start.elapsed();

    println!(
        "--- STA Results ---\nsingle init: {:?}\nmanual init: {:?}\nwith_com(sync): {:?}\nwith_com(async): {:?}\ncom_thread(sync): {:?}\ncom_thread(async): {:?}",
        single_init_dur,
        manual_dur,
        with_com_sync_dur,
        with_com_async_dur,
        thread_sync_dur,
        thread_async_dur
    );
    println!(
        "--- MTA Results ---\nsingle init: {:?}\nmanual init: {:?}\nwith_com(sync): {:?}\nwith_com(async): {:?}\ncom_thread(sync): {:?}\ncom_thread(async): {:?}",
        single_init_mta_dur,
        manual_mta_dur,
        with_com_sync_mta_dur,
        with_com_async_mta_dur,
        thread_sync_mta_dur,
        thread_async_mta_dur
    );
}
