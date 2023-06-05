use std::sync::Arc;

use axum::{extract::State, response::IntoResponse, routing::get, Router};

async fn hello_world(State(state): State<Arc<AppState>>) -> impl IntoResponse {
    state.msg
}

#[derive(Clone)]
struct AppState {
    msg: &'static str,
}

#[cyndra_runtime::main]
async fn axum() -> cyndra_axum::CyndraAxum {
    let state = Arc::new(AppState {
        msg: "Hello, world!",
    });

    let router = Router::new().route("/", get(hello_world)).with_state(state);

    Ok(router.into())
}
