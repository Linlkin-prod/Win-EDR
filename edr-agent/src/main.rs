mod service;
mod logging;
pub mod worker;
pub mod model;
mod process;
mod etw;

use windows::core::PWSTR;
use windows::Win32::System::Services::*;

fn main() -> windows::core::Result<()> {
    logging::init_logging();

    // UTF-16 service name with trailing NUL so Windows APIs see a proper LPWSTR
    let mut service_name: Vec<u16> = "EDRAgent"
        .encode_utf16()
        .chain(std::iter::once(0))
        .collect();

    let service_table = [
        SERVICE_TABLE_ENTRYW {
            lpServiceName: PWSTR(service_name.as_mut_ptr()),
            lpServiceProc: Some(service_main),
        },
        SERVICE_TABLE_ENTRYW::default(), // null terminator entry
    ];

    unsafe {
        StartServiceCtrlDispatcherW(service_table.as_ptr())?;
    }

    Ok(())
}

unsafe extern "system" fn service_main(_argc: u32, _argv: *mut PWSTR) {
    if let Err(e) = crate::service::run() {
        tracing::error!("Service failed: {:?}", e);
    }
}