#[macro_use]
extern crate rocket;

use anyhow::Context;
use rocket::response::status::BadRequest;
use rocket::State;
use cyndra_runtime::SecretStore;

#[get("/secret")]
async fn secret(state: &State<MyState>) -> Result<String, BadRequest<String>> {
    Ok(state.secret.clone())
}

struct MyState {
    secret: String,
}

#[cyndra_runtime::main]
async fn rocket(
    #[cyndra_runtime::Secrets] secrets: SecretStore,
) -> cyndra_rocket::CyndraRocket {
    // get secret defined in `Secrets.toml` file.
    let secret = secrets.get("MY_API_KEY").context("secret was not found")?;

    let state = MyState { secret };
    let rocket = rocket::build().mount("/", routes![secret]).manage(state);

    Ok(rocket.into())
}
