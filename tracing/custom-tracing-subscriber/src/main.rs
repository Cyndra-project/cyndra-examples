use actix_web::{get, web::ServiceConfig};
use cyndra_actix_web::CyndraActixWeb;
use tracing_subscriber::filter::{EnvFilter, LevelFilter};

#[tracing::instrument(name = "hello")]
#[get("/")]
async fn hello_world() -> &'static str {
    let msg = "Hello World!";
    tracing::info!("{msg}");
    msg
}

#[cyndra_runtime::main]
async fn actix_web() -> CyndraActixWeb<impl FnOnce(&mut ServiceConfig) + Send + Clone + 'static> {
    // We need to write to stdout for Cyndra to record our logs, so we use the
    // tracing::fmt subscriber which has sane defaults for applications and also
    // writes to stdout.
    tracing_subscriber::fmt()
        .with_env_filter(
            EnvFilter::builder()
                .with_default_directive(LevelFilter::INFO.into())
                .from_env_lossy(),
        )
        .init();
    tracing::info!("tracing is initialized");

    let config = move |cfg: &mut ServiceConfig| {
        cfg.service(hello_world);
    };

    Ok(config.into())
}
