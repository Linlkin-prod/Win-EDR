use crate::process::{self, cache::ProcessCache, enrich};
use crate::model::{ProcessContext, Event, EventData, normalize_path, EventType, HostId, ProcessStatus};
use std::sync::mpsc;


pub fn run_baseline(_tx: mpsc::Sender<Event>) {
    let snapshot = process::snapshot::snapshot();

    for proc in snapshot {

        let ctx: ProcessContext = ProcessContext {
            pid: proc.pid,
            ppid: Some(proc.ppid),
            image: Some(proc.image),
            image_path_raw: None,
            image_path: None,
            cmdline: None,
            user_sid: None,
            integrity_level: None,
            session_id: None,
            status: Some(ProcessStatus::Running),
        };

        let event = Event {
            timestamp: 0,
            event_type: EventType::ProcessStart,
            host_id: None,
            process: ctx,
            data: EventData::ProcessStart {
                parent_image: String::new(),
                parent_cmdline: None,
            },
        };

        let _ = _tx.send(event);
    }
}