use rama::http::service::fs::ServeDir;

#[cyndra_runtime::main]
async fn main() -> Result<impl cyndra_rama::CyndraService, cyndra_rama::CyndraError> {
    Ok(cyndra_rama::RamaService::application(ServeDir::new(
        "assets",
    )))
}
