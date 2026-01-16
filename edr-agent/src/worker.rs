use std::sync::mpsc::Receiver;
use crate::model::{Event, EventType};
use crate::process::cache::ProcessCache;

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

fn handle_event(event: Event, cache: &mut ProcessCache) {
    match event.event_type {
        EventType::ProcessStart => {
            cache.insert(event.process.pid, event.process.clone());
        }
        EventType::ProcessStop => {
            
        }
        EventType::ImageLoad => {
        }
    }
}
