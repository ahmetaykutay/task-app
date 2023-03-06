use actix_web::{web, App, HttpServer};
use mongodb::{options::ClientOptions, Client, Database};
use std::sync::{Arc, Mutex};
mod task;

#[macro_use]
extern crate dotenv_codegen;

pub struct AppState {
    pub db: Arc<Mutex<Database>>,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let db_uri = dotenv!("DB_URI");
    let mut client_options = ClientOptions::parse(&db_uri)
        .await
        .expect("clint options could not be parsed");
    client_options.app_name = Some("Task App".to_string());
    let client = Client::with_options(client_options).expect("db connection failed");
    let db = client.database("task_app");
    let db = Arc::new(Mutex::new(db));

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(AppState { db: db.clone() }))
            .service(web::scope("/tasks").configure(task::api::config))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
