use std::sync::Mutex;

use actix_web::{get, post, web, HttpResponse, HttpServer, Responder};
use shared::EncryptedData;

struct GlobalState {
    data: Vec<EncryptedData>,
    counter: u32,
}

impl GlobalState {
    fn new() -> Self {
        GlobalState {
            data: Vec::new(),
            counter: 0,
        }
    }

    fn append(&mut self, encdata: EncryptedData) -> u32 {
        self.data.push(encdata);
        self.counter += 1;
        self.counter - 1
    }
}

type AppState = web::Data<Mutex<GlobalState>>;

#[post("/upload")]
async fn upload(
    state: AppState,
    data: web::Json<EncryptedData>,
) -> Result<impl Responder, actix_web::Error> {
    let mut state = state.lock().unwrap();

    let id = state.append(data.into_inner());

    println!("Uploaded data with id {}", id);
    println!("Data: {:?}", state.data);

    Ok(HttpResponse::Ok().json(id))
}

#[get("/download/{id}")]
async fn download(state: AppState, id: web::Path<i32>) -> Result<impl Responder, actix_web::Error> {
    let state = state.lock().unwrap();
    let id = *id;

    println!("Getting data with id {}", id);

    if id as usize >= state.data.len() {
        return Ok(HttpResponse::NotFound().finish());
    }

    let data = &state.data[id as usize];

    println!("Data: {:?}", data);

    Ok(HttpResponse::Ok().json(data))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let state = web::Data::new(Mutex::new(GlobalState::new()));
    HttpServer::new(move || {
        actix_web::App::new()
            .app_data(state.clone())
            .service(upload)
            .service(download)
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}
