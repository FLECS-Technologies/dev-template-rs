use crate::config;
use tracing::info;
use tracing_subscriber::{EnvFilter, layer::SubscriberExt, util::SubscriberInitExt};

const DEFAULT_TRACING_FILTER: &str = "info";
fn filter() -> EnvFilter {
    EnvFilter::try_from_default_env().unwrap_or_else(|_| {
        config::get()
            .trace_filter
            .as_deref()
            .unwrap_or(DEFAULT_TRACING_FILTER)
            .into()
    })
}

pub fn init() {
    tracing_subscriber::registry()
        .with(filter())
        .with(tracing_subscriber::fmt::layer())
        .init();
    info!("Tracing initialized");
}
