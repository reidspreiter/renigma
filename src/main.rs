mod enigma;
use enigma::Enigma;

use actix_files::NamedFile;
use actix_web::{get, post, web, App, HttpResponse, HttpServer};
use std::path::PathBuf;

#[get("/")]
async fn index() -> actix_web::Result<NamedFile> {
    let path: PathBuf = "./src/static/index.html".parse().unwrap();
    Ok(NamedFile::open(path)?)
}

#[post("/encode")]
async fn encode(data: web::Json<String>) -> HttpResponse {
    println!("POSTING YAY");
    println!("{}", data);
    HttpResponse::Ok().json("Recieved")
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