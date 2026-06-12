mod bindings;
mod http;
mod plugin_manager;

use std::net::SocketAddr;
use tower_http::trace::TraceLayer;
use tracing::info;
use tracing_subscriber::{EnvFilter, layer::SubscriberExt, util::SubscriberInitExt};

use crate::plugin_manager::PluginManager;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::registry()
        .with(EnvFilter::try_from_default_env().unwrap_or_else(|_| "info,tower_http=info".into()))
        .with(tracing_subscriber::fmt::layer())
        .init();

    let plugin_manager = PluginManager::new()?;
    let app = http::router(plugin_manager).layer(TraceLayer::new_for_http());
    let addr = SocketAddr::from(([127, 0, 0, 1], 3012));

    info!(%addr, "starting plugin host service");
    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, app).await?;

    Ok(())
}
