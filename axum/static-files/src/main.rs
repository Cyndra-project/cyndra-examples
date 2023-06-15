use std::path::PathBuf;

use axum::{routing::get, Router};
use tower_http::services::ServeDir;

async fn hello_world() -> &'static str {
    "Hello, world!"
}

#[cyndra_runtime::main]
async fn axum(
    // Name your static assets folder by passing `folder = <name>` to `StaticFolder`
    // If you don't pass a name, it will default to `static`.
    #[cyndra_static_folder::StaticFolder(folder = "assets")] static_folder: PathBuf,
) -> cyndra_axum::CyndraAxum {
    let router = Router::new()
        .route("/", get(hello_world))
        .nest_service("/assets", ServeDir::new(static_folder));

    Ok(router.into())
}
