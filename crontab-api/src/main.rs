use axum::{response::IntoResponse, routing::get, Router};
use cyndra_crontab::{CrontabService, CyndraCrontab};
use cyndra_persist::{Persist, PersistInstance};

async fn hello_crontab() -> impl IntoResponse {
    "Hello there, try making a POST request to '/crontab/set' to create a new job.".to_string()
}

#[cyndra_runtime::main]
async fn crontab(#[Persist] persist: PersistInstance) -> CyndraCrontab {
    let router = Router::new().route("/", get(hello_crontab));
    CrontabService::new(persist, router)
}
