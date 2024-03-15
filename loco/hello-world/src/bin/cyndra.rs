use hello_world::app::App;
use loco_rs::boot::{create_app, StartMode};
use loco_rs::environment::Environment;

#[cyndra_runtime::main]
async fn main(
    #[cyndra_runtime::Metadata] meta: cyndra_runtime::DeploymentMetadata,
) -> cyndra_axum::CyndraAxum {
    let environment = match meta.env {
        cyndra_runtime::Environment::Local => Environment::Development,
        cyndra_runtime::Environment::Deployment => Environment::Production,
    };
    let boot_result = create_app::<App>(StartMode::ServerOnly, &environment)
        .await
        .unwrap();

    let router = boot_result.router.unwrap();
    Ok(router.into())
}
