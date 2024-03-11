use axum::Router;
use tower_http::services::{ServeDir, ServeFile};

#[cyndra_runtime::main]
async fn main() -> cyndra_axum::CyndraAxum {
    let router = Router::new().nest_service(
        "/",
        ServeDir::new("dist").not_found_service(ServeFile::new("dist/index.html")),
    );
    Ok(router.into())
}
