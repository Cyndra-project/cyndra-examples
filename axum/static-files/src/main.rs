use axum::{routing::get, Router};
use tower_http::services::ServeDir;

async fn hello_world() -> &'static str {
    "Hello, world!"
}

#[cyndra_runtime::main]
async fn main() -> cyndra_axum::CyndraAxum {
    let router = Router::new()
        .route("/", get(hello_world))
        .nest_service("/assets", ServeDir::new("assets"));

    Ok(router.into())
}
