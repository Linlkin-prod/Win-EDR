use std::sync::mpsc::{self, Receiver};
use std::thread;
use std::time::Duration;
use crate::process::{self, cache::ProcessCache, enrich};
use crate::model::{ProcessContext, Event, EventData, normalize_path, EventType, HostId, EventPipeline};
use std::time::SystemTime;
use crate::etw;



pub fn run(_rx: Receiver<Event>) {

    let mut cache = ProcessCache::new();


    loop {
        if _rx.try_recv().is_ok() {
            break;
        }
    }

    if let Ok(event) = _rx.recv() {
        tracing::info!("ETW Process Event: PID={}, PPID={}", event.process.pid, event.process.ppid);

        //enrich and process the event as needed

        if event.event_type == EventType::ProcessStart {
            let ctx = event.process.clone();
            cache.insert(event.process.pid, ctx);
        }
    }


}
