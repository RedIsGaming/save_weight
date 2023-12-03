use actix_web::{ get, Responder, HttpResponse, HttpServer, App, http };
use lib::Weight;
use std::io::Result;
use chrono::*;
use actix_cors::Cors;
use std::vec;
use actix_web::middleware::Logger;
use std::net::{ IpAddr, Ipv4Addr };

#[allow(unused)]
async fn get_weight() -> impl Responder {
    let _weight = vec![
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

    HttpResponse::Ok().body("weight")
}

#[get("/")]
async fn app() -> impl Responder {
    HttpResponse::Ok().body("Hello, world!")
}

#[actix_web::main]
async fn main() -> Result<()> {
    HttpServer::new(|| {
        let cors = Cors::default()
            .allowed_origin("localhost:5173")
            .allowed_methods(vec!["GET", "POST"])
            .allowed_headers(vec![http::header::AUTHORIZATION, http::header::ACCEPT])
            .allowed_header(http::header::CONTENT_TYPE)
            .max_age(3600);

        App::new()
            .service(app)
            .wrap(cors)
            .wrap(Logger::default())
    })
    .bind((IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 8080))?
    .run()
    .await
}
