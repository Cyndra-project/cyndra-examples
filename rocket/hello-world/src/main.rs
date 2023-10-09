use rocket::{get, routes};

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[cyndra_runtime::main]
async fn main() -> cyndra_rocket::CyndraRocket {
    let rocket = rocket::build().mount("/", routes![index]);

    Ok(rocket.into())
}
