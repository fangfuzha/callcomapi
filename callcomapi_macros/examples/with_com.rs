use callcomapi_macros::with_com;
use windows::Win32::System::Com::{CLSCTX_INPROC_SERVER, CoCreateInstance};
use windows::Win32::System::Wmi::{IWbemLocator, WbemLocator};
use windows::core::BSTR;

/// Demonstrates using the `#[with_com]` macro which automatically initializes and uninitializes COM.
#[with_com]
fn main() {
    println!("Using #[with_com] macro to auto-handle COM initialization...");

    // Try calling a real COM API (WMI)
    unsafe {
        let result = call_wmi_sample();
        match result {
            Ok(_) => println!("Successfully called COM API!"),
            Err(e) => eprintln!("Call failed: {}", e),
        }
    }

    println!("COM will be automatically uninitialized when the function exits.");
}

unsafe fn call_wmi_sample() -> windows::core::Result<()> {
    unsafe {
        // Create WbemLocator
        let locator: IWbemLocator = CoCreateInstance(&WbemLocator, None, CLSCTX_INPROC_SERVER)?;

        // Connect to the WMI service
        let _service = locator.ConnectServer(
            &BSTR::from("ROOT\\CIMV2"),
            &BSTR::new(),
            &BSTR::new(),
            &BSTR::new(),
            0,
            &BSTR::new(),
            None,
        )?;

        println!("Successfully connected to WMI service (ROOT\\CIMV2).");
        Ok(())
    }
}
