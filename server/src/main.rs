use actix_web::{post, web, HttpResponse, HttpServer, Responder};
use shared::FileUpload;

#[post("/upload")]
async fn upload(_data: web::Json<FileUpload>) -> Result<impl Responder, actix_web::Error> {
    Ok(HttpResponse::Ok().body("File uploaded"))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| actix_web::App::new().service(upload))
        .bind(("0.0.0.0", 8080))?
        .run()
        .await
}
