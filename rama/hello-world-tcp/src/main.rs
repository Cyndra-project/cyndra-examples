use rama::{net, service::service_fn};
use std::convert::Infallible;
use tokio::io::AsyncWriteExt;

async fn hello_world<S>(mut stream: S) -> Result<(), Infallible>
where
    S: net::stream::Stream + Unpin,
{
    const TEXT: &str = "Hello, Cyndra!";

    let resp = [
        "HTTP/1.1 200 OK",
        "Content-Type: text/plain",
        format!("Content-Length: {}", TEXT.len()).as_str(),
        "",
        TEXT,
        "",
    ]
    .join("\r\n");

    stream
        .write_all(resp.as_bytes())
        .await
        .expect("write to stream");

    Ok::<_, std::convert::Infallible>(())
}

#[cyndra_runtime::main]
async fn main() -> Result<impl cyndra_rama::CyndraService, cyndra_rama::CyndraError> {
    Ok(cyndra_rama::RamaService::transport(service_fn(
        hello_world,
    )))
}
