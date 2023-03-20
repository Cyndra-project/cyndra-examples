use warp::Filter;
use warp::Reply;

#[cyndra_runtime::main]
async fn warp() -> cyndra_warp::CyndraWarp<(impl Reply,)> {
    let route = warp::any().map(|| "Hello, World!");
    Ok(route.boxed().into())
}
