use actix_web::{get, Responder, HttpServer, App, HttpResponse, web};
use lib::Weight;
use std::io::Result;
use chrono::*;
use std::fs::OpenOptions;

async fn manage_file() -> impl Responder {
    let mut file = OpenOptions::new()
        .write(true)
        .append(true)
        .create(false)
        .open("../public/weight_statistics.txt")
        .expect("File not found");

    let content = file;

    HttpResponse::Ok().body(file)
}

async fn get_weight() -> impl Responder {
    let weight = vec![
        Weight {
            id: 1,
            weight: 80.5,
            date: Utc::now(),
        },

        Weight {
            id: 2,
            weight: 77.4,
            date: Utc::now(),
        },

        Weight {
            id: 3,
            weight: 78.2,
            date: Utc::now(),
        },
    ];

    HttpResponse::Ok().json(weight)
}

#[get("/")]
async fn app() -> impl Responder {
    HttpResponse::Ok().body("Hello, world!")
}

#[actix_web::main]
async fn main() -> Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(app)
            .route("/", web::get().to(manage_file))
            .route("/", web::get().to(get_weight))

    })
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}
