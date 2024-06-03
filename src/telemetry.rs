use tracing::Subscriber;
use tracing_bunyan_formatter::{BunyanFormattingLayer, JsonStorageLayer};
use tracing_log::LogTracer;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::{EnvFilter, Registry};

pub fn get_subscriber(
    name: String,
    env_filter: String,
    sink: bool,
) -> Box<dyn Subscriber + Send + Sync> {
    if sink {
        let formatting_layer = BunyanFormattingLayer::new(name, std::io::sink);
        let env_filter =
            EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new(env_filter));
        Box::new(
            Registry::default()
                .with(env_filter)
                .with(JsonStorageLayer)
                .with(formatting_layer),
        )
    } else {
        let formatting_layer = BunyanFormattingLayer::new(name, std::io::stdout);
        let env_filter =
            EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new(env_filter));
        Box::new(
            Registry::default()
                .with(env_filter)
                .with(JsonStorageLayer)
                .with(formatting_layer),
        )
    }
}

pub fn init_subscriber(subscriber: impl Subscriber + Send + Sync) {
    LogTracer::init().expect("Could not set global logger");
    tracing::subscriber::set_global_default(subscriber)
        .expect("Could not to set global subscriber");
}
