use axum::Json;
use serde_json::json;

/// Health check handler to verify the server is running.
///
/// Returns a JSON response with status "ok".
pub async fn health_check() -> Json<serde_json::Value> {
    Json(json!({ "status": "ok" }))
}
