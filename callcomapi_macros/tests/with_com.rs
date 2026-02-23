use windows::Win32::System::Com::{COINIT_MULTITHREADED, CoUninitialize};
use windows::core::Result;
use callcomapi_macros::with_com;

mod common;

// raw binding so we can observe the HRESULT exactly instead of having
// the automatic `Result<()>` wrapper which treats S_FALSE as success.
#[link(name = "ole32")]
unsafe extern "system" {
    fn CoInitializeEx(
        pvreserved: *const core::ffi::c_void,
        dwcoinit: windows::Win32::System::Com::COINIT,
    ) -> windows::core::HRESULT;
}

// legacy probe that only succeeds when CoInitializeEx returns S_OK.
// Any other HRESULT (including S_FALSE) is treated as an error, letting tests
// verify that COM truly became uninitialized.
fn check_com() -> Result<()> {
    use windows::Win32::Foundation::S_OK;

    // call the raw function directly
    let hr = unsafe { CoInitializeEx(core::ptr::null(), COINIT_MULTITHREADED) };
    if hr != S_OK {
        return Err(windows::core::Error::from(hr));
    }
    unsafe { CoUninitialize() };
    Ok(())
}

#[with_com]
fn foo() -> i32 {
    // when executed the macro should have initialized COM once.  calling a
    // real COM API should succeed.
    common::call_com_api().unwrap();
    42
}

#[test]
fn test_macro_wraps_function() {
    assert_eq!(foo(), 42);
}

// explicitly specify apartment-threaded model using the new string syntax;
// the macro internally maps "STA" to the correct constant.
#[with_com("STA")]
fn sta_foo() -> i32 {
    common::call_com_api().unwrap();
    99
}

#[test]
fn test_sta_model() {
    assert_eq!(sta_foo(), 99);
}

#[with_com]
fn generic<T>(x: T) -> T {
    x
}

#[test]
fn test_generic() {
    assert_eq!(generic(10u8), 10);
}

#[with_com]
async fn async_fn() -> i32 {
    // still should be inside COM-initialized context
    common::call_com_api().unwrap();
    7
}

#[test]
fn test_async() {
    let val = futures::executor::block_on(async_fn());
    assert_eq!(val, 7);
}

// another variant: call from within an async block and await the function
#[tokio::test]
async fn test_async_await() {
    let val = async_fn().await;
    assert_eq!(val, 7);
}

// panic safety: function panics but COM should be uninitialized afterwards
#[with_com]
fn will_panic() {
    panic!("oops");
}

#[test]
fn test_panic_safety() {
    let _ = std::panic::catch_unwind(|| will_panic());
    // if the guard ran, COM isn't still marked as initialized; a simple probe
    // should succeed (return S_OK rather than S_FALSE).
    check_com().unwrap();
}
