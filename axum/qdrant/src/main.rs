use axum::{extract::State, routing::get, Router};
use qdrant_client::prelude::*;
use std::sync::Arc;

struct AppState {
    qdrant: QdrantClient,
}

async fn list_collections(State(state): State<Arc<AppState>>) -> String {
    format!("{:?}\n", state.qdrant.list_collections().await)
}

#[cyndra_runtime::main]
async fn main(
    #[cyndra_qdrant::Qdrant(cloud_url = "{secrets.CLOUD_URL}", api_key = "{secrets.API_KEY}")]
    qdrant: QdrantClient,
) -> cyndra_axum::CyndraAxum {
    let state = Arc::new(AppState { qdrant });

    let router = Router::new()
        .route("/", get(list_collections))
        .with_state(state);

    Ok(router.into())
}
