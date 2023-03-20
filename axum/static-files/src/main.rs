use std::path::PathBuf;

use axum::{routing::get, Router};
use axum_extra::routing::SpaRouter;

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
        .route("/hello", get(hello_world))
        .merge(SpaRouter::new("/assets", static_folder).index_file("index.html"));

    Ok(router.into())
}
