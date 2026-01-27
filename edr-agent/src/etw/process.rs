use windows::Win32::System::Diagnostics::Etw::{
    CONTROLTRACE_HANDLE, EVENT_RECORD, EVENT_TRACE_FLAG_PROCESS, EVENT_TRACE_PROPERTIES,
    EVENT_TRACE_LOGFILEW, EnableTraceEx2, OpenTraceW, ProcessTrace,
    PROCESS_TRACE_MODE_EVENT_RECORD, PROCESS_TRACE_MODE_REAL_TIME, StartTraceW,
    TRACE_LEVEL_INFORMATION, PROCESSTRACE_HANDLE,
};

use windows::core::GUID;
use windows::core::PWSTR;
use std::sync::{mpsc::Sender, OnceLock};
use std::time::{SystemTime, UNIX_EPOCH};
use std::thread;
use crate::model::{Event, EventType, EventData, ProcessContext, ProcessStatus};

static ETW_TX: OnceLock<Sender<Event>> = OnceLock::new();


pub fn run(tx: Sender<Event>) -> windows::core::Result<()> {
    unsafe {
        let _ = ETW_TX.set(tx);
        let mut properties = EVENT_TRACE_PROPERTIES::default();
        properties.EnableFlags = EVENT_TRACE_FLAG_PROCESS;

        let mut session_name: Vec<u16> = "EDRProcessSession\0".encode_utf16().collect();
        let mut session_handle: CONTROLTRACE_HANDLE = CONTROLTRACE_HANDLE::default();

        let status = StartTraceW(
            &mut session_handle,
            PWSTR(session_name.as_mut_ptr()),
            &mut properties,
        );

        if status.is_err() {
            return Err(status.into());
        }

        let process_guid = GUID {
            data1: 0x9e814f68,
            data2: 0x3f86,
            data3: 0x4c43,
            data4: [0x96, 0x70, 0xf5, 0xeb, 0x6c, 0xec, 0x60, 0xa6],
        };

        let status = EnableTraceEx2(
            session_handle,
            &process_guid,
            0,
            TRACE_LEVEL_INFORMATION as u8,
            EVENT_TRACE_FLAG_PROCESS.0 as u64,
            0,
            0,
            None,
        );

        if status.is_err() {
            return Err(status.into());
        }

        let trace_thread_name: Vec<u16> = "EDRProcessSession\0".encode_utf16().collect();
        thread::spawn(move || {
            unsafe {
                let mut logfile = EVENT_TRACE_LOGFILEW::default();
                logfile.LoggerName = PWSTR(trace_thread_name.as_ptr() as *mut u16);
                logfile.Anonymous1.ProcessTraceMode =
                    PROCESS_TRACE_MODE_REAL_TIME | PROCESS_TRACE_MODE_EVENT_RECORD;
                logfile.Anonymous2.EventRecordCallback = Some(event_callback);

                let trace_handle: PROCESSTRACE_HANDLE = OpenTraceW(&mut logfile);

                if trace_handle.Value == 0 || trace_handle.Value == u64::MAX {
                    tracing::error!("OpenTraceW failed for EDRProcessSession");
                    return;
                }

                let handles = [trace_handle];
                let _ = ProcessTrace(&handles, None, None);
            }
        });

        Ok(())
    }
}





unsafe extern "system" fn event_callback(record : *mut EVENT_RECORD) {
    let header = &(*record).EventHeader;
    let opcode = header.EventDescriptor.Opcode;
    let event_type = match opcode {
        1 | 3 => EventType::ProcessStart,
        2 | 4 => EventType::ProcessStop,
        _ => return,
    };

    let pid = header.ProcessId;

    let status = match event_type {
        EventType::ProcessStart => Some(ProcessStatus::Running),
        EventType::ProcessStop => Some(ProcessStatus::Terminated),
        _ => None,
    };

    let data = match event_type {
        EventType::ProcessStart => EventData::ProcessStart {
            parent_image: String::new(),
            parent_cmdline: None,
        },
        EventType::ProcessStop => EventData::ProcessStop { exit_code: 0 },
        _ => return,
    };


    let event = Event {
        timestamp: SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs(),
        event_type,
        host_id: None,
        process: ProcessContext {
            pid,
            ppid: None,
            image: None,          
            image_path_raw: None, 
            image_path: None,
            cmdline: None,
            user_sid: None,
            integrity_level: None,
            session_id: None,
            status,
        },
        data,
    };

    if let Some(tx) = ETW_TX.get() {
        let _ = tx.send(event);
    }

}
