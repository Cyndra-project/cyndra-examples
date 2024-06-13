use axum::{routing::get, Router};

#[cfg(not(feature = "cyndra"))]
async fn hello() -> &'static str {
    "Hello, world!"
}

#[cfg(feature = "cyndra")]
async fn hello() -> &'static str {
    "Hello, Cyndra!"
}

#[cyndra_runtime::main]
async fn main() -> cyndra_axum::CyndraAxum {
    let router = Router::new().route("/", get(hello));

    Ok(router.into())
}
