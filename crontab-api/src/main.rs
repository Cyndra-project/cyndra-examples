use axum::{response::IntoResponse, routing::get, Router};
use cyndra_persist::{Persist, PersistInstance};

use crontab_api::{CrontabService, CyndraCrontab};

async fn hello_crontab() -> impl IntoResponse {
    "Hello there, try making a POST request to '/crontab/set' to create a new job.".to_string()
}

async fn trigger_me() -> impl IntoResponse {
    "Triggered by the crontab service".to_string()
}

#[cyndra_runtime::main]
async fn crontab(#[Persist] persist: PersistInstance) -> CyndraCrontab {
    let router = Router::new()
        .route("/", get(hello_crontab))
        .route("/trigger-me", get(trigger_me));
    CrontabService::new(persist, router)
}
