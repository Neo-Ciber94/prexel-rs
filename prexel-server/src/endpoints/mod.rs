use crate::models::{EvaluatedExpression, NumberType};
use crate::{ApiResponse, services};
use actix_web::web::Query;
use actix_web::{get, post, web, HttpRequest, HttpResponse, Responder, Result};
use prexel::complex::Complex;
use prexel::context::DefaultContext;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
struct EvalParams {
    only_result: Option<bool>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct GetFunctions {
    pub r#type: Option<NumberType>,
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
    let eval_result = services::eval_expression(expression);
    let query = Query::<EvalParams>::from_query(req.query_string()).ok();
    let only_result = query.and_then(|q| q.only_result).unwrap_or(false);

    if !only_result || eval_result.is_err() {
        Ok(HttpResponse::Ok().json(EvaluatedExpression::from(eval_result)))
    } else {
        let text = eval_result.unwrap();
        Ok(HttpResponse::Ok().body(text))
    }
}

#[get("/operators")]
pub async fn get_operators(req: HttpRequest) -> Result<impl Responder> {
    let query = Query::<GetFunctions>::from_query(req.query_string()).ok();
    let t = query.and_then(|q| q.r#type).unwrap_or(NumberType::Decimal);

    match t {
        NumberType::Decimal => {
            let context = DefaultContext::new_decimal();
            let operators = services::get_operators(&context);
            Ok(HttpResponse::Ok().json(operators))
        }
        NumberType::Float => {
            let context = DefaultContext::new_decimal();
            let operators = services::get_operators(&context);
            Ok(HttpResponse::Ok().json(operators))
        }
        NumberType::Complex => {
            let context = DefaultContext::<Complex<f64>>::new_complex();
            let operators = services::get_operators(&context);
            Ok(HttpResponse::Ok().json(operators))
        }
        NumberType::Integer => {
            let context = DefaultContext::<i128>::new_checked();
            let operators = services::get_operators(&context);
            Ok(HttpResponse::Ok().json(operators))
        }
        NumberType::Binary => {
            let context = DefaultContext::new_binary();
            let operators = services::get_operators(&context);
            Ok(HttpResponse::Ok().json(operators))
        }
    }
}

#[get("/constants")]
pub async fn get_constants(req: HttpRequest) -> Result<impl Responder> {
    let query = Query::<GetFunctions>::from_query(req.query_string()).ok();
    let t = query.and_then(|q| q.r#type).unwrap_or(NumberType::Decimal);

    match t {
        NumberType::Decimal => {
            let context = DefaultContext::new_decimal();
            let constants = services::get_constants(&context);
            Ok(HttpResponse::Ok().json(constants))
        }
        NumberType::Float => {
            let context = DefaultContext::new_decimal();
            let constants = services::get_constants(&context);
            Ok(HttpResponse::Ok().json(constants))
        }
        NumberType::Complex => {
            let context = DefaultContext::<Complex<f64>>::new_complex();
            let constants = services::get_constants(&context);
            Ok(HttpResponse::Ok().json(constants))
        }
        NumberType::Integer => {
            let context = DefaultContext::<i128>::new_checked();
            let constants = services::get_constants(&context);
            Ok(HttpResponse::Ok().json(constants))
        }
        NumberType::Binary => {
            let context = DefaultContext::new_binary();
            let constants = services::get_constants(&context);
            Ok(HttpResponse::Ok().json(constants))
        }
    }
}

#[get("/functions")]
pub async fn get_functions(req: HttpRequest) -> Result<impl Responder> {
    let query = Query::<GetFunctions>::from_query(req.query_string()).ok();
    let t = query.and_then(|q| q.r#type).unwrap_or(NumberType::Decimal);

    match t {
        NumberType::Decimal => {
            let context = DefaultContext::new_decimal();
            let functions = services::get_functions(&context);
            Ok(HttpResponse::Ok().json(functions))
        }
        NumberType::Float => {
            let context = DefaultContext::new_decimal();
            let functions = services::get_functions(&context);
            Ok(HttpResponse::Ok().json(functions))
        }
        NumberType::Complex => {
            let context = DefaultContext::<Complex<f64>>::new_complex();
            let functions = services::get_functions(&context);
            Ok(HttpResponse::Ok().json(functions))
        }
        NumberType::Integer => {
            let context = DefaultContext::<i128>::new_checked();
            let functions = services::get_functions(&context);
            Ok(HttpResponse::Ok().json(functions))
        }
        NumberType::Binary => {
            let context = DefaultContext::new_binary();
            let functions = services::get_functions(&context);
            Ok(HttpResponse::Ok().json(functions))
        }
    }
}