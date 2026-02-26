use windows::core::Result;

/// helper that does a real COM operation to ensure COM is initialized
pub fn call_com_api() -> Result<()> {
    use windows::Win32::System::Com::{
        CLSCTX_INPROC_SERVER, CoCreateInstance, CoSetProxyBlanket, EOAC_NONE,
        RPC_C_AUTHN_LEVEL_CALL, RPC_C_IMP_LEVEL_IMPERSONATE,
    };
    use windows::Win32::System::Rpc::{RPC_C_AUTHN_WINNT, RPC_C_AUTHZ_NONE};
    use windows::Win32::System::Wmi::{
        IWbemClassObject, IWbemLocator, WBEM_FLAG_FORWARD_ONLY, WBEM_FLAG_RETURN_IMMEDIATELY,
        WBEM_GENERIC_FLAG_TYPE, WBEM_INFINITE, WbemLocator,
    };
    use windows::core::BSTR;

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

        if returned_count > 0 {
            if let Some(cpu_object) = cpu_objects[0].take() {
                let cpu_text = cpu_object.GetObjectText(0)?.to_string();
                let cpu_model = cpu_text
                    .lines()
                    .find_map(|line| {
                        let trimmed = line.trim();
                        if trimmed.starts_with("Name =") {
                            Some(
                                trimmed
                                    .trim_start_matches("Name =")
                                    .trim()
                                    .trim_end_matches(';')
                                    .trim_matches('"')
                                    .to_string(),
                            )
                        } else {
                            None
                        }
                    })
                    .unwrap_or_else(|| "<unknown>".to_string());
                println!("CPU Model: {}", cpu_model);
            } else {
                println!("CPU Model: <unknown>");
            }
        } else {
            println!("CPU Model: <unknown>");
        }
    }
    Ok(())
}
