use std::sync::Mutex;

use actix_web::{web, App, HttpServer};

use crate::apps;
use crate::config;
use crate::db;

pub fn routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/users")
            .route("", web::get().to(apps::user::views::list))
            .route("", web::post().to(apps::user::views::create))
            .route("/{id}", web::post().to(apps::user::views::update))
            .route("/{id}", web::get().to(apps::user::views::retrieve))
            .route("/{id}", web::delete().to(apps::user::views::destroy)),
    )
    .route("/countup", web::get().to(apps::countup))
    .route("/echo", web::get().to(apps::echo))
    .route("/", web::get().to(apps::hello));
}

#[actix_web::main]
pub async fn run() -> std::io::Result<()> {
    let counter = web::Data::new(apps::AppStateWithCounter {
        counter: Mutex::new(0),
    });
    HttpServer::new(move || {
        App::new()
            .data(config::AppState {
                app_name: String::from("Actic-web"),
            })
            .data(db::create_connection_pool().clone())
            .app_data(counter.clone())
            .configure(routes)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}