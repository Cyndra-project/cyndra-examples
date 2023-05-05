use salvo::prelude::*;

#[handler]
async fn hello_world(res: &mut Response) {
    res.render(Text::Plain("Hello, world!"));
}

#[cyndra_runtime::main]
async fn salvo() -> cyndra_salvo::CyndraSalvo {
    let router = Router::new().get(hello_world);

    Ok(router.into())
}
