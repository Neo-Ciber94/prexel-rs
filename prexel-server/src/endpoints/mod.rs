use crate::evaluator::eval_expression;
use crate::models::EvaluatedExpression;
use crate::ApiResponse;
use actix_web::web::Query;
use actix_web::{post, web, HttpRequest, HttpResponse};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
struct Params {
    only_result: Option<bool>,
}

#[post("/eval")]
pub async fn eval(req: HttpRequest, body: web::Bytes) -> ApiResponse {
    let body = match String::from_utf8(body.to_vec()) {
        Ok(s) => s,
        Err(_) => {
            return Ok(HttpResponse::BadRequest()
                .json(EvaluatedExpression::with_error("Invalid JSON".to_string())));
        }
    };

    let deserializer = &mut serde_json::Deserializer::from_str(&body);
    let expr = match serde_path_to_error::deserialize(deserializer) {
        Ok(expr) => expr,
        Err(err) => {
            let path = err.path().to_string();
            let error_message = match path.as_str() {
                "." => "Invalid json".to_string(),
                _ => format!("Invalid value type at path: {}", path),
            };
            return Ok(
                HttpResponse::BadRequest().json(EvaluatedExpression::with_error(error_message))
            );
        }
    };

    let expression = expr;
    let eval_result = eval_expression(expression);
    let query = Query::<Params>::from_query(req.query_string()).ok();
    let only_result = query.and_then(|q| q.only_result).unwrap_or(false);

    if !only_result || eval_result.is_err() {
        Ok(HttpResponse::Ok().json(EvaluatedExpression::from(eval_result)))
    } else {
        let text = eval_result.unwrap().to_string();
        Ok(HttpResponse::Ok().body(text))
    }
}
