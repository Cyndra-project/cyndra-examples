use actix_web::{get, web::ServiceConfig};
use cyndra_service::CyndraActixWeb;

#[get("/hello")]
async fn hello_world() -> &'static str {
    "Hello World!"
}

#[cyndra_service::main]
async fn actix_web(
) -> CyndraActixWeb<impl FnOnce(&mut ServiceConfig) + Sync + Send + Clone + 'static> {
    Ok(move |cfg: &mut ServiceConfig| {
        cfg.service(hello_world);
    })
}
