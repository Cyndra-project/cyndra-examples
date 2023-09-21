use std::path::PathBuf;

use axum::Router;
use axum_extra::routing::SpaRouter;

#[cyndra_runtime::main]
async fn axum() -> cyndra_axum::CyndraAxum {
    let router =
        Router::new().merge(SpaRouter::new("/", PathBuf::from("static")).index_file("index.html"));

    Ok(router.into())
}
