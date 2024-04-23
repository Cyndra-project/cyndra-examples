use axum::{routing::get, Router};
use cyndra_runtime::DeploymentMetadata;

#[cyndra_runtime::main]
async fn main(
    #[cyndra_runtime::Metadata] metadata: DeploymentMetadata,
) -> cyndra_axum::CyndraAxum {
    let router = Router::new().route("/", get(format!("{:?}", metadata)));

    Ok(router.into())
}
