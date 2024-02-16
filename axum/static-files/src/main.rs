use axum::Router;
use tower_http::services::ServeDir;

#[cyndra_runtime::main]
async fn main() -> cyndra_axum::CyndraAxum {
    // ServeDir falls back to serve index.html when requesting a directory
    let router = Router::new().nest_service("/", ServeDir::new("assets"));

    Ok(router.into())
}
