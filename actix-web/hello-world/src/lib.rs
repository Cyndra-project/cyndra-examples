use actix_web::web::{resource, ServiceConfig};
use cyndra_service::CyndraActixWeb;

async fn hello_world() -> &'static str {
    "Hello World!"
}

#[cyndra_service::main]
async fn actix_web(
) -> CyndraActixWeb<impl FnOnce(&mut ServiceConfig) + Sync + Send + Copy + Clone + 'static> {
    Ok(move |cfg: &mut ServiceConfig| {
        cfg.service(resource("/hello").to(hello_world));
    })
}
