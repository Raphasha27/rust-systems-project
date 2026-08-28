use axum::Json;
use serde::{Deserialize, Serialize};

#[derive(Serialize)]
pub struct HealthResponse {
    status: String,
    version: String,
}

pub async fn health_check() -> Json<HealthResponse> {
    Json(HealthResponse {
        status: "healthy".to_string(),
        version: env!("CARGO_PKG_VERSION").to_string(),
    })
}

#[derive(Serialize, Deserialize)]
pub struct DataItem {
    id: u32,
    name: String,
    value: f64,
}

pub async fn get_data() -> Json<Vec<DataItem>> {
    Json(vec![
        DataItem { id: 1, name: "Item 1".to_string(), value: 100.0 },
        DataItem { id: 2, name: "Item 2".to_string(), value: 200.0 },
    ])
}
