mod endpoints;
mod evaluator;
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
                    .with_interval(Duration::from_secs(60))
                    .with_max_requests(100),
            )
            .service(endpoints::eval)
    })
    .bind(format!("0.0.0.0:{}", port))?
    .run()
    .await
}
