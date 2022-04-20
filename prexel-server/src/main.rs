mod endpoints;
mod middlewares;
mod models;
mod services;

use crate::middlewares::rate_limiter::{
    generate_rate_limit_id, get_rate_limit_identifier, RATE_LIMIT_ID,
};
use actix_ratelimit::errors::ARError;
use actix_ratelimit::{MemoryStore, MemoryStoreActor, RateLimiter};
use actix_web::cookie::{Cookie, CookieBuilder};
use actix_web::dev::Service;
use actix_web::http::header::{COOKIE, SET_COOKIE};
use actix_web::http::{HeaderMap, HeaderValue};
use actix_web::middleware::normalize::TrailingSlash;
use actix_web::middleware::{Logger, NormalizePath};
use actix_web::{App, HttpResponse, HttpServer};
use std::env;
use std::time::Duration;

pub type ApiResponse = Result<HttpResponse, HttpResponse>;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("debug"));

    let port = env::var("PORT")
        .map(|s| s.parse::<u16>().ok())
        .ok()
        .flatten()
        .unwrap_or(8000);

    HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .wrap(NormalizePath::new(TrailingSlash::Trim))
            .wrap(get_rate_limit_middleware())
            .wrap_fn(|mut req, srv| {
                let has_id = get_rate_limit_identifier(&req).is_some();
                let mut cookie: Option<Cookie> = None;

                if !has_id {
                    let new_id = generate_rate_limit_id();
                    log::debug!("Generating new rate limiter id for unknown ip: {}", new_id);
                    let temp_cookie = CookieBuilder::new(RATE_LIMIT_ID, new_id)
                        .http_only(true)
                        .path("/")
                        .permanent()
                        .finish();

                    // We need set the cookie to the request, to allow other middlewares to read it
                    req.headers_mut().insert(
                        COOKIE,
                        HeaderValue::from_str(&temp_cookie.to_string()).unwrap(),
                    );

                    cookie = Some(temp_cookie);
                }

                let fut = srv.call(req);
                async move {
                    let mut res = fut.await?;

                    if let Some(cookie) = cookie {
                        let headers: &mut HeaderMap = res.headers_mut();

                        // Set the cookie to the response
                        headers.insert(
                            SET_COOKIE,
                            HeaderValue::from_str(&cookie.to_string()).unwrap(),
                        );
                    }

                    Ok(res)
                }
            })
            .service(
                actix_web::web::scope("")
                    .service(endpoints::eval)
                    .service(endpoints::get_operators)
                    .service(endpoints::get_functions)
                    .service(endpoints::get_constants),
            )
    })
    .bind(format!("0.0.0.0:{}", port))?
    .run()
    .await
}

fn get_rate_limit_middleware() -> RateLimiter<MemoryStoreActor> {
    const MAX_REQUESTS: usize = 100;
    const TIME_BETWEEN_REQUESTS: Duration = Duration::from_secs(60);

    let store = MemoryStore::new();

    RateLimiter::new(MemoryStoreActor::from(store).start())
        .with_interval(TIME_BETWEEN_REQUESTS)
        .with_max_requests(MAX_REQUESTS)
        .with_identifier(|req| match get_rate_limit_identifier(req) {
            Some(id) => Ok(id),
            None => Err(ARError::IdentificationError),
        })
}
