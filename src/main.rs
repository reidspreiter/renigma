// mod enigma;
// use enigma::Enigma;

use actix_files::NamedFile;
use actix_web::{get, web, App, HttpRequest, HttpServer};
use std::path::PathBuf;

// fn main() {
//     let mut enigma = Enigma::new(
//         "abghfi",
//         "UKWA",
//         "II", 23,
//         "I", 1,
//         "V", 0,
//     );
//     let plaintext = "She sat deep in thought, for the room was on fire!";
//     let ciphertext = enigma.encode(plaintext, false, true, true);
//     println!("{}", ciphertext);

//     let mut enigma = Enigma::new(
//         "abghfi",
//         "UKWA",
//         "II", 23,
//         "I", 1,
//         "V", 0,
//     );
//     let should_be_plain = enigma.encode(&ciphertext, false, true, true);
//     assert_eq!(plaintext, should_be_plain);
//     println!("{}", should_be_plain);
// }

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