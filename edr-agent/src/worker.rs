use std::sync::mpsc::Receiver;
use std::time::Duration;

pub fn run(stop_rx: Receiver<()>) {
    tracing::info!("Worker started");

    loop {
        if stop_rx.try_recv().is_ok() {
            break;
        }

        tracing::info!("EDR heartbeat");
        std::thread::sleep(Duration::from_secs(5));
    }

    tracing::info!("Worker stopping");
}
