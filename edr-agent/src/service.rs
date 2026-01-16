use std::ffi::OsString;
use std::os::windows::ffi::OsStrExt;
use std::sync::{Mutex, OnceLock, mpsc};
use std::{thread};

use windows::Win32::System::Services::*;
use windows::core::PCWSTR;

use crate::worker;
use crate::etw;
use crate::model;

const SERVICE_NAME: &str = "EDRAgent";

static STOP_SIGNAL: OnceLock<Mutex<mpsc::Sender<()>>> = OnceLock::new();

pub fn run() -> windows::core::Result<()> {
    unsafe {
        let service_name = OsString::from(SERVICE_NAME);
        let service_name_w: Vec<u16> = service_name.encode_wide().chain(std::iter::once(0)).collect();

        let status_handle = RegisterServiceCtrlHandlerW(
            PCWSTR(service_name_w.as_ptr()),
            Some(service_ctrl_handler),
        )?;

        set_service_status(status_handle, SERVICE_START_PENDING)?;

        let (stop_tx, stop_rx) = mpsc::channel();
        STOP_SIGNAL.set(Mutex::new(stop_tx)).unwrap();


        let (tx, rx) = mpsc::channel();
        let event_pipeline = model::EventPipeline {
            tx: tx,
            rx: rx,
        };

        let worker_handle = thread::spawn(move || {
            worker::run(event_pipeline.rx, stop_rx);
        });

        let baseline_handle = thread::spawn({
            let tx = event_pipeline.tx.clone();
            move || {
                etw::baseline::run_baseline(tx);
            }
        });

        let etw_process_handle = etw::process::run_etw_listener(event_pipeline.tx.clone())
            .expect("Failed to start ETW process listener");

        set_service_status(status_handle, SERVICE_RUNNING)?;

        // Block here until STOP is requested
        worker_handle.join().ok();

        set_service_status(status_handle, SERVICE_STOPPED)?;
    }

    Ok(())
}

extern "system" fn service_ctrl_handler(ctrl: u32) {
    match ctrl {
        SERVICE_CONTROL_STOP | SERVICE_CONTROL_SHUTDOWN => {
            tracing::info!("Service stop requested");

            if let Some(lock) = STOP_SIGNAL.get() {
                let _ = lock.lock().unwrap().send(());
            }
        }
        _ => {tracing::warn!("Unknown service control request: {}", ctrl);}
    }
}

unsafe fn set_service_status(
    status_handle: SERVICE_STATUS_HANDLE,
    state: SERVICE_STATUS_CURRENT_STATE,
) -> windows::core::Result<()> {
    let status = SERVICE_STATUS {
        dwServiceType: SERVICE_WIN32_OWN_PROCESS,
        dwCurrentState: state,
        dwControlsAccepted: if state == SERVICE_RUNNING {
            SERVICE_ACCEPT_STOP | SERVICE_ACCEPT_SHUTDOWN
        } else {
            0
        },
        dwWin32ExitCode: 0,
        dwServiceSpecificExitCode: 0,
        dwCheckPoint: 0,
        dwWaitHint: 0,
    };
    unsafe {
        SetServiceStatus(status_handle, &status)?;
    }
    
    Ok(())
}