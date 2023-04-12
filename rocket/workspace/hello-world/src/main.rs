#[macro_use]
extern crate rocket;

#[get("/")]
fn index() -> &'static str {
    shared::hello()
}

#[cyndra_runtime::main]
async fn rocket() -> cyndra_rocket::CyndraRocket {
    let rocket = rocket::build().mount("/hello", routes![index]);

    Ok(rocket.into())
}
