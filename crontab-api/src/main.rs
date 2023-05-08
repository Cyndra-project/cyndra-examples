use std::sync::Arc;

use axum::Router;
use cyndra_persist::PersistInstance;

mod router;

pub struct CrontabService {
    router: Router,
    persist: PersistInstance,
}

pub struct AppState {
    persist: PersistInstance,
}

#[cyndra_runtime::async_trait]
impl cyndra_runtime::Service for CrontabService {
    async fn bind(mut self, addr: std::net::SocketAddr) -> Result<(), cyndra_runtime::Error> {
        let router = self.router;

        let serve_router = axum::Server::bind(&addr).serve(router.into_make_service());

        tokio::select!(
            // _ = self.discord_bot.run() => {},
            _ = serve_router => {}
        );

        Ok(())
    }
}

#[cyndra_runtime::main]
async fn init(
    #[cyndra_persist::Persist] persist: PersistInstance,
) -> Result<CrontabService, cyndra_runtime::Error> {
    let app_state = Arc::new(AppState {
        persist: persist.clone(),
    });
    let router = router::build_router(app_state);

    Ok(CrontabService { router, persist })
}
