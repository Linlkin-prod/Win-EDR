use tracing_subscriber::fmt::Subscriber;

pub fn init_logging() {
    let subscriber = Subscriber::builder()
        .with_max_level(tracing::Level::INFO)
        .with_thread_ids(true)
        .with_thread_names(true)
        .finish();

    tracing::subscriber::set_global_default(subscriber)
        .expect("Failed to set global default subscriber");
}