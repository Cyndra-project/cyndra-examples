use axum::Router;
use tower_http::services::{ServeDir, ServeFile};

#[cyndra_runtime::main]
async fn axum() -> cyndra_axum::CyndraAxum {
    let router = Router::new().nest_service(
        "/",
        ServeDir::new("static").not_found_service(ServeFile::new("static/index.html")),
    );

    Ok(router.into())
}
