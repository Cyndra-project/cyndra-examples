use axum::{routing::get, Router, extract::State};

async fn hello_world(State(state): State<AppState>,) -> String {
    let body = format!("Hello from {}", state.id);
    body
}

#[derive(Clone)]
struct AppState {
    pub id: ulid::Ulid
}

#[cyndra_runtime::main]
async fn main() -> cyndra_axum::CyndraAxum {
    let state = AppState {id: ulid::Ulid::new()};
    let router = Router::new().route("/", get(hello_world)).with_state(state);

    Ok(router.into())
}
