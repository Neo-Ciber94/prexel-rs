use evaluator::{
    eval_complex_expression, eval_decimal_expression, eval_float_expression,
    eval_integer_expression,
};
use models::{EvalExpression, EvalResult};
use rocket::serde::{json::Json, Deserialize, Serialize};
mod evaluator;
mod models;

#[macro_use]
extern crate rocket;

/// Represents the type of the numbers of an expression.
#[derive(Debug, PartialEq, FromFormField)]
enum NumberType {
    /// Decimal numbers. (default)
    Decimal,

    /// Floating point numbers.
    Float,

    /// Complex numbers.
    Complex,

    /// Integer numbers
    Integer,
}

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

#[post("/eval?<type>&<only_result>", data = "<expr>")]
fn eval(
    expr: Json<EvalExpression>,
    r#type: Option<NumberType>,
    only_result: Option<bool>,
) -> Json<EvalResponse> {
    let r#type = r#type.unwrap_or(NumberType::Decimal);
    let expression = expr.into_inner();

    let eval_result = match r#type {
        NumberType::Decimal => eval_decimal_expression(expression),
        NumberType::Float => eval_float_expression(expression),
        NumberType::Complex => eval_complex_expression(expression),
        NumberType::Integer => eval_integer_expression(expression),
    };

    let only_result = only_result.unwrap_or(false);

    if !only_result || eval_result.is_err() {
        Json(EvalResponse::Result(EvaluatedExpression::from(eval_result)))
    } else {
        Json(EvalResponse::Number(eval_result.ok().unwrap().to_string()))
    }
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![eval])
}
