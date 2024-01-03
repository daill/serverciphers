use std::path::{Path, PathBuf};
use reqwest::{get, Response};
use rocket::fs::NamedFile;
use rocket::{get, launch, routes};
use rocket::futures::TryFutureExt;
use rocket::response::status::NotFound;

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![files, get_ciphers])
}

#[get("/api/ciphers")]
async fn get_ciphers() -> String {
    let res = reqwest::get("https://ciphersuite.info/api/cs/security/recommended/").await.unwrap().text().await.unwrap();
    return res

}

#[get("/<file..>")]
async fn files(file: PathBuf) -> Option<NamedFile> {
    NamedFile::open(Path::new("web").join(file)).await.ok()
}