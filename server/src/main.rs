use actix_web::{post, web, HttpResponse, HttpServer, Responder};
use shared::FileUpload;
use shared::EncryptedData;

#[post("/upload")]
async fn upload(_data: web::Json<FileUpload>) -> Result<impl Responder, actix_web::Error> {
    Ok(HttpResponse::Ok().body("File uploaded"))
}

let global_data: Vec<EncryptedData> = Vec::new();
let global_counter: i32 = -1;

fn add_data(encdata: EncryptedData) -> i32 {
    global_data.append(encdata);
    global_counter += 1;
    return global_counter;
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| actix_web::App::new().service(upload))
        .bind(("0.0.0.0", 8080))?
        .run()
        .await
}

