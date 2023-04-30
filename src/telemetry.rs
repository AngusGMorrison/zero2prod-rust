use tracing::Subscriber;
use tracing_bunyan_formatter::{BunyanFormattingLayer, JsonStorageLayer};
use tracing_subscriber::fmt::MakeWriter;
use tracing_subscriber::{layer::SubscriberExt, EnvFilter, Registry};

type AsyncSubscriber = Box<dyn Subscriber + Sync + Send>;

/// Compose `tracing` layers into a subscriber to handle asynchronous tracing
/// events.
pub fn new_subscriber<Sink>(name: String, default_log_level: String, sink: Sink) -> AsyncSubscriber
where
    Sink: for<'a> MakeWriter<'a> + Send + Sync + 'static,
{
    let env_filter =
        EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new(default_log_level));
    let formatting_layer = BunyanFormattingLayer::new(name, sink);
    let subscriber = Registry::default()
        .with(env_filter)
        .with(JsonStorageLayer)
        .with(formatting_layer);

    Box::new(subscriber)
}

/// Register a subscriber as the global default for span processing.
///
/// MUST be called only once.
pub fn init_subscriber(subscriber: AsyncSubscriber) {
    // Redirect all `log` events to our tracing subscriber.
    tracing_log::LogTracer::init().expect("Failed to redirect logs to tracer");
    tracing::subscriber::set_global_default(subscriber)
        .expect("Failed to set global subscriber default");
}
