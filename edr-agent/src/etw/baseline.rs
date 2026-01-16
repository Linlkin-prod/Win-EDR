use crate::process::{self, cache::ProcessCache, enrich};
use crate::model::{ProcessContext, Event, EventData, normalize_path, EventType, HostId, ProcessStatus};
use std::sync::mpsc;


pub fn run_baseline(_tx: mpsc::Sender<Event>) {
    let snapshot = process::snapshot::snapshot();

    for proc in snapshot {
        let image_path_raw = enrich::get_image_path(proc.pid);

        let ctx: ProcessContext = ProcessContext {
            pid: proc.pid,
            ppid: proc.ppid,
            image: Some(proc.image),
            image_path_raw: image_path_raw.clone(),
            image_path: image_path_raw.map(|p| normalize_path(p.as_str())),
            cmdline: None,
            user_sid: None,
            integrity_level: None,
            session_id: None,
            status: Some(ProcessStatus::Running),
        };

        let event = Event {
            timestamp: 0,
            event_type: EventType::ProcessStart,
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

        let _ = _tx.send(event);
    }
}