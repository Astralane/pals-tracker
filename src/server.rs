use crate::app::{AppState, app_router};
use crate::palidator_cache::PalidatorCache;
use anyhow::Context;
use axum::Extension;
use std::net::SocketAddr;
use std::sync::atomic::AtomicU64;
use std::sync::{Arc, RwLock};
use tracing::info;

pub async fn serve(
    address: SocketAddr,
    palidator_cache: Arc<RwLock<PalidatorCache>>,
    slot_cache: Arc<AtomicU64>,
) -> anyhow::Result<()> {
    let app_state = Arc::new(AppState::new(palidator_cache, slot_cache));
    let app = app_router().layer(Extension(app_state));
    let listener = tokio::net::TcpListener::bind(address)
        .await
        .context(format!("Failed to bind to address {:?}", address))?;
    info!("Listening on {:?}", address);
    let _hdl = axum::serve(listener, app.into_make_service())
        .await
        .context("Failed to start server")?;
    Ok(())
}
