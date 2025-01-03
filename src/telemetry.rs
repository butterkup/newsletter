use tracing_subscriber::{fmt::MakeWriter, layer::SubscriberExt};

pub fn get_subscriber<Sink>(
  name: String,
  log_level: String,
  sink: Sink,
) -> impl tracing::Subscriber + Send + Sync
where
  Sink: for<'a> MakeWriter<'a> + Send + Sync + 'static,
{
  let env_filter = tracing_subscriber::EnvFilter::try_from_default_env()
    .unwrap_or_else(|_| tracing_subscriber::EnvFilter::new(name));
  let formatting_layer = tracing_bunyan_formatter::BunyanFormattingLayer::new(log_level, sink);
  tracing_subscriber::Registry::default()
    .with(env_filter)
    .with(tracing_bunyan_formatter::JsonStorageLayer)
    .with(formatting_layer)
}

pub fn init_subscriber(subscriber: impl tracing::Subscriber + Send + Sync) {
  tracing_log::LogTracer::init().expect("Failed to set logger");
  tracing::subscriber::set_global_default(subscriber).expect("Failed to set subscriber");
}

pub fn setup_subscriber<Sink>(name: String, log_level: String, sink: Sink)
where
  Sink: for<'a> MakeWriter<'a> + Send + Sync + 'static,
{
  init_subscriber(get_subscriber(name, log_level, sink));
}
