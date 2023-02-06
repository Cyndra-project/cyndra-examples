use std::path::PathBuf;

use axum::Router;
use axum_extra::routing::SpaRouter;
use sync_wrapper::SyncWrapper;

#[cyndra_service::main]
async fn axum(
    #[cyndra_static_folder::StaticFolder] static_folder: PathBuf,
) -> cyndra_service::CyndraAxum {
    let router =
        Router::new().merge(SpaRouter::new("/assets", static_folder).index_file("index.html"));

    let sync_wrapper = SyncWrapper::new(router);

    Ok(sync_wrapper)
}
