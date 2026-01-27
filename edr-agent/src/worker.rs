use std::sync::mpsc::Receiver;
use crate::model::{Event, EventType, normalize_path};
use crate::process::cache::ProcessCache;

use windows::Win32::System::Threading::OpenProcess;
use windows::Win32::System::Threading::PROCESS_QUERY_INFORMATION;
use windows::Win32::Foundation::{CloseHandle, HANDLE};
use ntapi::ntpsapi::{NtQueryInformationProcess, PROCESS_BASIC_INFORMATION};
use ntapi::winapi::ctypes::c_void;
use std::mem::{zeroed, size_of};

use crate::process::{enrich};

pub fn run(event_rx: Receiver<Event>, stop_rx: Receiver<()>) {
    let mut cache = ProcessCache::new();

    loop {
        if let Ok(_) = stop_rx.try_recv() {
            break;
        }

        match event_rx.recv() {
            Ok(event) => {
                handle_event(event, &mut cache);
            }
            Err(_) => {
                break;
            }
        }
    }
}

fn handle_event(mut event: Event, cache: &mut ProcessCache) {
    match event.event_type {
        EventType::ProcessStart => {
            if event.process.ppid.is_none() {
                event.process.ppid = Some(get_parent_process_id(event.process.pid).unwrap_or(0));
            }
            let image_path_raw = enrich::get_image_path(event.process.pid);
            event.process.image_path_raw = image_path_raw.clone();
            event.process.image_path = image_path_raw.map(|p| normalize_path(p.as_str()));

            cache.insert(event.process.pid, event.process.clone());
        }
        EventType::ProcessStop => {
            cache.remove(&event.process.pid);
        }
        EventType::ImageLoad => {
        }
    }
}

fn get_parent_process_id(pid: u32) -> Option<u32> {
    unsafe {
        let handle = match OpenProcess(PROCESS_QUERY_INFORMATION, false, pid) {
            Ok(h) => h,
            Err(_) => {
                eprintln!("impossible to find ppid for {}", pid);
                return None;
            }
        };

        let mut pbi: PROCESS_BASIC_INFORMATION = zeroed();
        let mut return_length: u32 = 0;


        let status = NtQueryInformationProcess(
            handle.0 as *mut c_void,
            0, // ProcessBasicInformation
            &mut pbi as *mut _ as *mut c_void,
            size_of::<PROCESS_BASIC_INFORMATION>() as u32,
            &mut return_length,
        );

        let _ = CloseHandle(handle);

        if status == 0 {

            Some(pbi.InheritedFromUniqueProcessId as u32)
        } else {
            eprintln!("NtQueryInformationProcess failed with status: 0x{:X}", status);
            None
        }
    }
}
