use windows::Win32::System::Diagnostics::Etw::{
    StartTraceW, EnableTraceEx2, EVENT_TRACE_PROPERTIES, EVENT_TRACE_FLAG_PROCESS,
    TRACE_LEVEL_INFORMATION, CONTROLTRACE_HANDLE,
};
use windows::core::GUID;
use windows::core::PWSTR;
use std::sync::mpsc::Sender;

use crate::model::Event;


pub fn run_etw_listener (_tx: Sender<Event>) -> windows::core::Result<CONTROLTRACE_HANDLE> {

    unsafe {
        let mut properties = EVENT_TRACE_PROPERTIES::default();
        properties.EnableFlags = EVENT_TRACE_FLAG_PROCESS;

        let mut session_name: Vec<u16> = "EDRKernelSession\0".encode_utf16().collect();
        let mut session_handle: CONTROLTRACE_HANDLE = CONTROLTRACE_HANDLE::default();

        let status = StartTraceW(
            &mut session_handle,
            PWSTR(session_name.as_mut_ptr()),
            &mut properties,
        );

        if status.is_err() {
            return Err(status.into());
        }

        let trace_guid = GUID {
            data1: 0x9e814f68,
            data2: 0x3f86,
            data3: 0x4c43,
            data4: [0x96, 0x70, 0xf5, 0xeb, 0x6c, 0xec, 0x60, 0xa6],
        };

        let handle_ptr = &session_handle as *const CONTROLTRACE_HANDLE;

        let status = EnableTraceEx2(
            session_handle,
            &trace_guid,
            0,
            TRACE_LEVEL_INFORMATION.try_into().unwrap(),
            EVENT_TRACE_FLAG_PROCESS.0 as u64,
            0,
            0,
            None,
        );

        if status.is_err() {
            return Err(status.into());
        }

        Ok(session_handle)
    }
}