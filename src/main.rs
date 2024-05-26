mod enigma;
use enigma::Enigma;

use actix_files::NamedFile;
use actix_web::{get, App, HttpRequest, HttpServer};
use std::path::PathBuf;

#[get("/")]
async fn index(_req: HttpRequest) -> actix_web::Result<NamedFile> {
    let path: PathBuf = "./index.html".parse().unwrap();
    Ok(NamedFile::open(path)?)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(index)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}