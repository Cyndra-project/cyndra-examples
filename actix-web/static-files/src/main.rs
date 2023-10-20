use actix_files::NamedFile;
use actix_web::{get, web::ServiceConfig, Responder};
use cyndra_actix_web::CyndraActixWeb;

#[get("/")]
async fn index() -> impl Responder {
    NamedFile::open_async("assets/index.html").await
}

#[cyndra_runtime::main]
async fn actix_web() -> CyndraActixWeb<impl FnOnce(&mut ServiceConfig) + Send + Clone + 'static> {
    let config = move |cfg: &mut ServiceConfig| {
        cfg.service(index);
    };

    Ok(config.into())
}
