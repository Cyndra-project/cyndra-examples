use axum::{routing::get, Router};
use cyndra_metadata::Metadata;

#[cyndra_runtime::main]
async fn axum(
    #[cyndra_metadata::CyndraMetadata] metadata: Metadata,
) -> cyndra_axum::CyndraAxum {
    let router = Router::new().route("/", get(format!("{:?}", metadata)));

    Ok(router.into())
}
