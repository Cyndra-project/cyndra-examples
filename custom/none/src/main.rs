#[cyndra_runtime::main]
async fn cyndra_main() -> Result<MyService, cyndra_runtime::Error> {
    Ok(MyService {})
}

// Customize this struct with things from `cyndra_main` needed in `bind`,
// such as secrets or database connections
struct MyService {}

#[cyndra_runtime::async_trait]
impl cyndra_runtime::Service for MyService {
    async fn bind(self, _addr: std::net::SocketAddr) -> Result<(), cyndra_runtime::Error> {
        // Start your service and bind to the socket address
        Ok(())
    }
}
