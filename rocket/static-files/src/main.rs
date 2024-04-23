use rocket::fs::relative;
use rocket::fs::NamedFile;
use std::path::{Path, PathBuf};

#[rocket::get("/<path..>")]
pub async fn serve(mut path: PathBuf) -> Option<NamedFile> {
    path.set_extension("html");
    let mut path = Path::new(relative!("assets")).join(path);
    if path.is_dir() {
        path.push("index.html");
    }

    NamedFile::open(path).await.ok()
}

#[cyndra_runtime::main]
async fn main() -> cyndra_rocket::CyndraRocket {
    let rocket = rocket::build().mount("/", rocket::routes![serve]);

    Ok(rocket.into())
}
