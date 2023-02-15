use actix_web::{get, post, web, App, HttpRequest, HttpResponse, HttpServer, Responder, Result};
use actix_files as fs;

#[get("/")]
async fn index() -> Result<fs::NamedFile> {
    Ok(fs::NamedFile::open("./frontend/dist/index.html")?)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(index)
            .service(fs::Files::new("/", "./frontend/dist"))
    })
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}
