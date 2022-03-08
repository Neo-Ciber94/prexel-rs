use crate::EvalType;
use once_cell::sync::Lazy;
use prexel::complex::Complex;
use prexel::context::{Config, DefaultContext};
use prexel::evaluator::Evaluator;
use std::sync::Mutex;
use prexel::binary::binary_number_splitter;
use prexel::tokenizer::Tokenizer;

pub static CONFIG: Lazy<Mutex<Config>> = Lazy::new(|| {
    let config = Config::new().with_implicit_mul(true);
    Mutex::new(config)
});

#[derive(Debug, Clone)]
pub struct EvalExpr {
    r#type: EvalType,
}

impl EvalExpr {
    pub fn new(r#type: EvalType) -> Self {
        EvalExpr { r#type }
    }

    pub fn eval(&self, expr: &str) -> prexel::Result<String> {
        match self.r#type {
            EvalType::Decimal => eval_decimal(expr),
            EvalType::Complex => eval_complex(expr),
            EvalType::Float => eval_float(expr),
            EvalType::Integer => eval_integer(expr),
            EvalType::Binary => eval_binary(expr),
        }
    }
}

pub fn eval_decimal(expr: &str) -> prexel::Result<String> {
    let context = DefaultContext::with_config_decimal(CONFIG.lock().unwrap().clone());
    let evaluator = Evaluator::with_context(context);
    evaluator.eval(expr).map(|v| v.to_string())
}

pub fn eval_float(expr: &str) -> prexel::Result<String> {
    let context = DefaultContext::with_config_unchecked(CONFIG.lock().unwrap().clone());
    let evaluator = Evaluator::<f64>::with_context(context);
    evaluator.eval(expr).map(|v| v.to_string())
}

pub fn eval_integer(expr: &str) -> prexel::Result<String> {
    let context = DefaultContext::with_config_checked(CONFIG.lock().unwrap().clone());
    let evaluator = Evaluator::<i128>::with_context(context);
    evaluator.eval(expr).map(|v| v.to_string())
}

pub fn eval_complex(expr: &str) -> prexel::Result<String> {
    let context = DefaultContext::with_config_complex(
        CONFIG.lock().unwrap().clone().with_complex_number(true),
    );
    let evaluator = Evaluator::<Complex<f64>>::with_context(context);
    evaluator.eval(expr).map(|v| v.to_string())
}

pub fn eval_binary(expr: &str) -> prexel::Result<String> {
    let context = DefaultContext::with_config_binary(CONFIG.lock().unwrap().clone());
    let tokenizer = Tokenizer::with_splitter(binary_number_splitter());
    let evaluator = Evaluator::with_context_and_tokenizer(context, tokenizer);
    evaluator.eval(expr).map(|v| v.to_string())
}