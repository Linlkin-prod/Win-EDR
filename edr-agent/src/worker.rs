use std::sync::mpsc::Receiver;
use std::time::Duration;
use crate::process::{self, cache::ProcessCache, enrich};
use crate::model::{ProcessContext, Event, EventData, normalize_path, EventType, HostId};
use std::time::SystemTime;

pub fn run(_stop_rx: Receiver<()>) {
    let snapshot = process::snapshot::snapshot();
    let mut cache = ProcessCache::new();

    for proc in snapshot {
        let image_path_raw = enrich::get_image_path(proc.pid);

        let ctx = ProcessContext {
            pid: proc.pid,
            ppid: proc.ppid,
            image: proc.image,
            image_path_raw: image_path_raw.clone().unwrap_or_default(),
            image_path: image_path_raw.map(|p| normalize_path(p.as_str())).unwrap_or_default(),
            cmdline: None,
            user_sid: None,
            integrity_level: None,
            session_id: None,
        };

        cache.insert(proc.pid, ctx.clone());

        let event = Event {
            timestamp: SystemTime::now()
                .duration_since(SystemTime::UNIX_EPOCH)
                .unwrap_or(Duration::from_secs(0))
                .as_secs(),
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

        tracing::info!("Baseline event: {:?}", event);
    }

}
