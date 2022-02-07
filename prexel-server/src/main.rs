mod evaluator;
mod models;
mod context;

use std::env;
use crate::models::{EvalResponse, EvaluatedExpression};
use actix_ratelimit::{MemoryStore, MemoryStoreActor, RateLimiter};
use actix_web::web::{Json, Query};
use actix_web::{post, App, HttpRequest, HttpResponse, HttpServer, Responder};
use evaluator::eval_expression;
use models::{EvalExpression};
use serde::{Deserialize, Serialize};
use std::time::Duration;
use actix_web::middleware::Logger;

#[derive(Debug, Serialize, Deserialize)]
struct Params {
    only_result: Option<bool>,
}

#[post("/eval")]
async fn eval(req: HttpRequest, expr: Json<EvalExpression>) -> impl Responder {
    let expression = expr.into_inner();
    let eval_result = eval_expression(expression);
    let query = Query::<Params>::from_query(req.query_string()).ok();
    let only_result = query.and_then(|q| q.only_result).unwrap_or(false);

    if !only_result || eval_result.is_err() {
        HttpResponse::Ok().json(EvalResponse::Result(EvaluatedExpression::from(eval_result)))
    } else {
        HttpResponse::Ok().json(EvalResponse::Number(eval_result.ok().unwrap().to_string()))
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    let store = MemoryStore::new();
    let port = env::var("PORT")
        .map(|s| s.parse::<u16>().ok()).ok()
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
            .service(eval)
    })
    .bind(format!("0.0.0.0:{}", port))?
    .run()
    .await
}