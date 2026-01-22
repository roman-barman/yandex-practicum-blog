use tracing::subscriber::set_global_default;
use tracing_log::LogTracer;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::{EnvFilter, Registry, fmt};

pub(crate) fn initialize_tracing_subscribe(env_filter: &str) -> anyhow::Result<()> {
    let env_filter =
        EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new(env_filter));
    let json_layer = fmt::layer()
        .json()
        .with_timer(fmt::time::ChronoUtc::new("%Y-%m-%dT%H:%M:%S%.3fZ".into()))
        .with_level(true)
        .with_target(true)
        .with_current_span(true)
        .flatten_event(true);
    let subscriber = Registry::default().with(env_filter).with(json_layer);
    LogTracer::init()?;
    set_global_default(subscriber)?;
    Ok(())
}
