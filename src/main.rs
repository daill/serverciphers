use reqwest::{get, Response};
use rocket::fs::NamedFile;
use rocket::futures::TryFutureExt;
use rocket::response::status::NotFound;
use rocket::{get, launch, routes};
use std::path::{Path, PathBuf};

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![files, get_ciphers, home])
}

#[get("/api/ciphers")]
async fn get_ciphers() -> String {
    let res = reqwest::get("https://ciphersuite.info/api/cs/security/recommended/")
        .await
        .unwrap()
        .text()
        .await
        .unwrap();
    return res;
}

#[get("/")]
async fn home() -> Option<NamedFile> {
    NamedFile::open(Path::new("web").join("index.html"))
        .await
        .ok()
}

#[get("/<file..>")]
async fn files(file: PathBuf) -> Option<NamedFile> {
    let ext = file.extension().unwrap().to_str().unwrap();
    match ext {
        "css" | "js" | "png" | "webp" | "ico" | "svg" => {
            return NamedFile::open(Path::new("web").join(file)).await.ok();
        }
        _ => return None,
    }
}
