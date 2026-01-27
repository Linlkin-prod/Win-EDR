use windows::Win32::System::Diagnostics::Etw::{
    StartTraceW, EnableTraceEx2, EVENT_TRACE_PROPERTIES, EVENT_TRACE_FLAG_PROCESS,
    TRACE_LEVEL_INFORMATION, CONTROLTRACE_HANDLE,
};
use windows::core::GUID;
use windows::core::PWSTR;
use std::sync::mpsc::Sender;
use std::time::{SystemTime, UNIX_EPOCH};
use crate::model::{Event, EventType, EventData, ProcessContext, HostId, normalize_path, ProcessStatus};

#[repr(C)]
pub struct ProcessEvent {
    pub pid: u32,
    pub ppid: u32,
}


pub fn run(tx: Sender<Event>) -> windows::core::Result<()> {
    unsafe {
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

        

        Ok(())
    }
}

fn on_process_event(ev: ProcessEvent, tx: &Sender<Event>) {
    let ctx = ProcessContext {
        pid: ev.pid,
        ppid: ev.ppid,
        image: Some(String::new()),          
        image_path_raw: Some(String::new()), 
        image_path: Some(String::new()),
        cmdline: None,
        user_sid: None,
        integrity_level: None,
        session_id: None,
        status: Some(ProcessStatus::Running),
    };

    let event = Event {
        timestamp: SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs(),
        event_type: EventType::ProcessStart, // ou ProcessStop selon opcode
        host_id: HostId {
            agent_id: "agent-1234".to_string(),
            hostname: "host.local".to_string(),
        },
        process: ctx,
        data: EventData::ProcessStart {
            parent_image: String::new(),
            parent_cmdline: None,
        },
    };

    let _ = tx.send(event);
}

