use crate::route::StreamRouter;
use std::collections::HashMap;
use std::sync::Arc;
use warp::http::StatusCode;
use warp::{Rejection, Reply};

pub async fn handle_status(
    id: String,
    stations: Arc<HashMap<String, Arc<StreamRouter>>>, // ← new
) -> Result<impl Reply, Rejection> {
    if let Some(router) = stations.get(&id) {
        let now_playing = router
            .now_playing
            .read()
            .unwrap()
            .clone()
            .unwrap_or_else(|| "Nothing playing".to_string());

        let json = warp::reply::json(&serde_json::json!({
            "id": router.config.id, // Changed from name to id
            // "mount": router.config.mount, // Removed: no such field
            "now_playing": now_playing,
        }));
        Ok(warp::reply::with_status(json, StatusCode::OK))
    } else {
        Err(warp::reject::not_found())
    }
}
