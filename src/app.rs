use crate::palidator_cache::PalidatorCache;
use axum::routing::get;
use axum::{Json, Router};
use serde::{Deserialize, Serialize};
use std::sync::atomic::AtomicU64;
use std::sync::{Arc, RwLock};
use axum::extract::Path;
use tracing::info;

pub struct AppState {
    palidator_cache: Arc<RwLock<PalidatorCache>>,
    slot_cache: Arc<AtomicU64>,
}

impl AppState {
    pub fn new(palidator_cache: Arc<RwLock<PalidatorCache>>, slot_cache: Arc<AtomicU64>) -> Self {
        Self {
            palidator_cache,
            slot_cache,
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct NextPalidator {
    pub pubkey: String,
    pub slot: u64,
}

pub fn app_router() -> axum::Router {
    Router::new()
        .route("/paladin/palidators", get(get_all_validators))
        .route("/paladin/next_palidator", get(get_next_validator))
        .route("/paladin/next_palidator/{slot}", get(get_next_with_slot))
}

#[axum::debug_handler]
pub async fn get_all_validators(
    ctx: axum::Extension<Arc<AppState>>,
) -> Result<Json<Vec<String>>, &'static str> {
    info!("call get all validators");
    let pal_cache = ctx.palidator_cache.read().unwrap();
    let palidators = pal_cache.get_all_palidator_keys();
    Ok(Json(palidators))
}

#[axum::debug_handler]
pub async fn get_next_validator(
    ctx: axum::Extension<Arc<AppState>>,
) -> Result<Json<NextPalidator>, &'static str> {
    info!("call get next validator");
    let current_slot = ctx.slot_cache.load(std::sync::atomic::Ordering::SeqCst);
    let pal_cache = ctx.palidator_cache.read().unwrap();
    let (slot, pubkey) = pal_cache
        .get_next_palidator_with_slot(current_slot)
        .ok_or("slot not found")?;
    Ok(Json(NextPalidator { pubkey, slot }))
}

#[axum::debug_handler]
pub async fn get_next_with_slot(
    ctx: axum::Extension<Arc<AppState>>,
    Path(slot): Path<u64>,
) -> Result<Json<NextPalidator>, &'static str> {
    info!("call get next validator with slot");
    let pal_cache = ctx.palidator_cache.read().unwrap();
    let (slot, pubkey) = pal_cache
        .get_next_palidator_with_slot(slot)
        .ok_or("slot not found")?;
    Ok(Json(NextPalidator { pubkey, slot }))
}
