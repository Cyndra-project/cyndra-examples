use actix_files::Files;
use actix_web::web::ServiceConfig;
use cyndra_actix_web::CyndraActixWeb;

#[cyndra_runtime::main]
async fn main() -> CyndraActixWeb<impl FnOnce(&mut ServiceConfig) + Send + Clone + 'static> {
    let config = move |cfg: &mut ServiceConfig| {
        cfg.service(Files::new("/", "assets"));
    };

    Ok(config.into())
}
