mod endpoints;
mod services;
mod models;

use actix_ratelimit::{MemoryStore, MemoryStoreActor, RateLimiter};
use actix_web::middleware::Logger;
use actix_web::{ App, HttpResponse, HttpServer};
use std::env;
use std::time::Duration;

pub type ApiResponse = Result<HttpResponse, HttpResponse>;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    const MAX_REQUESTS : usize = 100;
    const TIME_BETWEEN_REQUESTS : Duration = Duration::from_secs(60);

    let store = MemoryStore::new();
    let port = env::var("PORT")
        .map(|s| s.parse::<u16>().ok())
        .ok()
        .flatten()
        .unwrap_or(8000);

    HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .wrap(
                RateLimiter::new(MemoryStoreActor::from(store.clone()).start())
                    .with_interval(TIME_BETWEEN_REQUESTS)
                    .with_max_requests(MAX_REQUESTS),
            )
            .service(endpoints::eval)
            .service(endpoints::get_operators)
            .service(endpoints::get_functions)
            .service(endpoints::get_constants)
    })
    .bind(format!("0.0.0.0:{}", port))?
    .run()
    .await
}
