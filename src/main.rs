mod enigma;
use enigma::{Enigma, EnigmaSettings};

use actix_files::Files;
use actix_web::{post, web, App, HttpServer, Responder, Result};


#[post("/encode")]
async fn encode(settings: web::Json<EnigmaSettings>) -> Result<impl Responder> {
    let mut enigma = Enigma::new(&settings);
    let ciphertext = enigma.encode(&settings);
    Ok(web::Json(ciphertext))
}


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(encode)
            .service(Files::new("/static", "static/").show_files_listing())
            .service(Files::new("/", "./static/").index_file("index.html"))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}