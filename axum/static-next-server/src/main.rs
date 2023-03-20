use std::path::PathBuf;

use axum::Router;
use axum_extra::routing::SpaRouter;

#[cyndra_runtime::main]
async fn axum(
    #[cyndra_static_folder::StaticFolder] static_folder: PathBuf,
) -> cyndra_axum::CyndraAxum {
    let router =
        Router::new().merge(SpaRouter::new("/", static_folder).index_file("index.html"));

    Ok(router.into())
}
