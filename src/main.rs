mod config;
mod models;
use crate::models::Status;
use actix_web::{web, App, HttpServer, Responder};
use dotenv::dotenv;
use std::io;

async fn status() -> impl Responder {
    web::HttpResponse::Ok().json(Status {
        status: "Ok".to_string(),
    })
}

#[actix_rt::main]
async fn main() -> io::Result<()> {
    dotenv().ok();

    let config = config::Config::from_env().unwrap();

    println!(
        "Server Starting at https://{}:{}",
        config.server.host, config.server.port
    );

    HttpServer::new(|| App::new().route("/", web::get().to(status)))
        .bind(format!("{}:{}", config.server.host, config.server.port))?
        .run()
        .await
}
