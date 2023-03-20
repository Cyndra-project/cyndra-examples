cyndra_next::app! {
    use futures::TryStreamExt;
    use tracing::debug;
    use cyndra_next::body::StreamBody;
    use cyndra_next::extract::BodyStream;
    use cyndra_next::response::{Response, IntoResponse};

    #[cyndra_next::endpoint(method = get, route = "/hello")]
    async fn hello() -> &'static str {
        "Hello, World!"
    }

    // We can also use tracing/log macros directly:
    #[cyndra_next::endpoint(method = get, route = "/goodbye")]
    async fn goodbye() -> &'static str {
        debug!("goodbye endpoint called");
        "Goodbye, World!"
    }

    // We can also extract the http body in our handlers.
    // The endpoint below takes the body from the request using the axum `BodyStream`
    // extractor, lazily maps its bytes to uppercase and streams it back in our response:
    #[cyndra_next::endpoint(method = post, route = "/uppercase")]
    async fn uppercase(body: BodyStream) -> impl IntoResponse {
        let chunk_stream = body.map_ok(|chunk| {
            chunk
                .iter()
                .map(|byte| byte.to_ascii_uppercase())
                .collect::<Vec<u8>>()
        });
        Response::new(StreamBody::new(chunk_stream))
    }
}
