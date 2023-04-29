use tracing::Subscriber;
use tracing_bunyan_formatter::{BunyanFormattingLayer, JsonStorageLayer};
use tracing_subscriber::{layer::SubscriberExt, EnvFilter, Registry};

/// Compose `tracing` layers into a subscriber to handle asynchronous tracing
/// events.
pub fn new_subscriber(name: String, default_log_level: String) -> impl Subscriber + Send + Sync {
    let env_filter =
        EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new(default_log_level));
    let formatting_layer = BunyanFormattingLayer::new(name, std::io::stdout);
    Registry::default()
        .with(env_filter)
        .with(JsonStorageLayer)
        .with(formatting_layer)
}

/// Register a subscriber as the global default for span processing.
///
/// MUST be called only once.
pub fn init_subscriber(subscriber: impl Subscriber + Send + Sync) {
    // Redirect all `log` events to our tracing subscriber.
    tracing_log::LogTracer::init().expect("Failed to redirect logs to tracer");
    tracing::subscriber::set_global_default(subscriber)
        .expect("Failed to set global subscriber default");
}
