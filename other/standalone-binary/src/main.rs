use cyndra_axum::CyndraAxum;
use cyndra_secrets::SecretStore;

use multi_binary::build_router;

#[cyndra_runtime::main]
async fn axum(#[cyndra_secrets::Secrets] secret_store: SecretStore) -> CyndraAxum {
    // Get all resources 'the Cyndra way'
    let my_secret = secret_store.get("SOME_API_KEY").unwrap();

    // Use the shared build function
    let router = build_router(my_secret);

    // Let Cyndra do the serving
    Ok(router.into())
}
