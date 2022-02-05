use std::time::Duration;
use evaluator::{
    eval_expression,
};
use models::{EvalExpression, EvalResult};
use rocket::serde::{json::Json, Deserialize, Serialize};
mod evaluator;
mod models;
mod middlewares;

#[macro_use]
extern crate rocket;

/// Represents the result to evaluate an expression.
#[derive(Debug, Serialize, Deserialize)]
struct EvaluatedExpression {
    result: Option<String>,
    error: Option<String>,
}

impl From<EvalResult> for EvaluatedExpression {
    fn from(result: EvalResult) -> Self {
        match result {
            EvalResult::Ok(result) => EvaluatedExpression {
                result: Some(result),
                error: None,
            },
            EvalResult::Err(error) => EvaluatedExpression {
                result: None,
                error: Some(error),
            },
        }
    }
}

/// Represents a response object.
#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
enum EvalResponse {
    Result(EvaluatedExpression),
    Number(String),
}

#[post("/eval?<only_result>", data = "<expr>")]
fn eval(
    expr: Json<EvalExpression>,
    only_result: Option<bool>,
) -> Json<EvalResponse> {
    let expression = expr.into_inner();
    let eval_result = eval_expression(expression);
    let only_result = only_result.unwrap_or(false);

    if !only_result || eval_result.is_err() {
        Json(EvalResponse::Result(EvaluatedExpression::from(eval_result)))
    } else {
        Json(EvalResponse::Number(eval_result.ok().unwrap().to_string()))
    }
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![eval])
        .attach(middlewares::RateLimiter::new(10, Duration::from_secs(10)))
}
