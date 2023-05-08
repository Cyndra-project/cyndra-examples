use axum::Router;
use cyndra_secrets::SecretStore;

mod router;

pub struct CrontabService {
    router: Router,
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
    #[cyndra_secrets::Secrets] _secrets: SecretStore,
) -> Result<CrontabService, cyndra_runtime::Error> {
    let router = router::build_router();

    Ok(CrontabService { router })
}
