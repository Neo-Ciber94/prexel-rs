use crate::context::{Binary, binary_number_splitter, BinaryContext};
use crate::models::{EvalExpression, EvalResult, NumberType};
use once_cell::sync::Lazy;
use prexel::complex;
use prexel::context::{Config, Grouping};
use prexel::tokenizer::Tokenizer;
use prexel::{context::Context, context::DefaultContext, decimal::Decimal, evaluator::Evaluator};
use std::str::FromStr;
use std::string::ToString;

static CONFIG: Lazy<Config> = Lazy::new(|| {
    Config::default()
        .with_grouping(Grouping::Parenthesis)
        .with_grouping(Grouping::Bracket)
        .with_implicit_mul(true)
});

pub fn eval_expression(expression: EvalExpression) -> EvalResult {
    let r#type = expression.r#type.clone().unwrap_or(NumberType::Decimal);

    match r#type {
        NumberType::Decimal => eval_decimal_expression(expression),
        NumberType::Float => eval_float_expression(expression),
        NumberType::Integer => eval_integer_expression(expression),
        NumberType::Complex => eval_complex_expression(expression),
        NumberType::Binary => eval_binary_expression(expression),
    }
}

fn eval_decimal_expression(expression: EvalExpression) -> EvalResult {
    let mut context = DefaultContext::with_config_decimal(CONFIG.clone());

    // Set variables
    if let Some(variables) = &expression.variables {
        for (name, value) in variables {
            match Decimal::from_str(&value.to_string()) {
                Ok(value) => {
                    context
                        .set_variable(name, value)
                        .map_err(|err| err.to_string())?;
                }
                Err(err) => return EvalResult::Err(format!("{}", err)),
            }
        }
    }

    // Evaluate expression
    let evaluator = Evaluator::with_context(context);
    let result = evaluator.eval(&expression.expression);

    match result {
        Ok(result) => EvalResult::Ok(result.to_string()),
        Err(error) => EvalResult::Err(error.to_string()),
    }
}

fn eval_float_expression(expression: EvalExpression) -> EvalResult {
    let mut context = DefaultContext::with_config_checked(CONFIG.clone());

    // Set variables
    if let Some(variables) = &expression.variables {
        for (name, value) in variables {
            match f64::from_str(&value.to_string()) {
                Ok(value) => {
                    context
                        .set_variable(name, value)
                        .map_err(|err| err.to_string())?;
                }
                Err(err) => return EvalResult::Err(format!("{}", err)),
            }
        }
    }

    // Evaluate expression
    let evaluator = Evaluator::with_context(context);
    let result = evaluator.eval(&expression.expression);

    match result {
        Ok(result) => EvalResult::Ok(result.to_string()),
        Err(error) => EvalResult::Err(error.to_string()),
    }
}

fn eval_complex_expression(expression: EvalExpression) -> EvalResult {
    let mut context = DefaultContext::with_config_complex(CONFIG.clone().with_complex_number(true));

    // Set variables
    if let Some(variables) = &expression.variables {
        for (name, value) in variables {
            match complex::Complex::<f64>::from_str(&value.to_string()) {
                Ok(value) => {
                    context
                        .set_variable(name, value)
                        .map_err(|err| err.to_string())?;
                }
                Err(err) => return EvalResult::Err(format!("{}", err)),
            }
        }
    }

    // Evaluate expression
    let evaluator = Evaluator::with_context(context);
    let result = evaluator.eval(&expression.expression);

    match result {
        Ok(result) => EvalResult::Ok(result.to_string()),
        Err(error) => EvalResult::Err(error.to_string()),
    }
}

fn eval_integer_expression(expression: EvalExpression) -> EvalResult {
    let mut context = DefaultContext::with_config_checked(CONFIG.clone());

    // Set variables
    if let Some(variables) = &expression.variables {
        for (name, value) in variables {
            match i128::from_str(&value.to_string()) {
                Ok(value) => {
                    context
                        .set_variable(name, value)
                        .map_err(|err| err.to_string())?;
                }
                Err(err) => return EvalResult::Err(format!("{}", err)),
            }
        }
    }

    // Evaluate expression
    let evaluator = Evaluator::with_context(context);
    let result = evaluator.eval(&expression.expression);

    match result {
        Ok(result) => EvalResult::Ok(result.to_string()),
        Err(error) => EvalResult::Err(error.to_string()),
    }
}

fn eval_binary_expression(expression: EvalExpression) -> EvalResult {
    let mut context = DefaultContext::with_config_binary(CONFIG.clone());

    // Set variables
    if let Some(variables) = &expression.variables {
        for (name, value) in variables {
            match i128::from_str(&value.to_string()) {
                Ok(value) => {
                    context
                        .set_variable(name, Binary(value))
                        .map_err(|err| err.to_string())?;
                }
                Err(err) => return EvalResult::Err(format!("{}", err)),
            }
        }
    }

    // Evaluate expression
    let tokenizer = Tokenizer::with_splitter(binary_number_splitter());
    let evaluator = Evaluator::with_context_and_tokenizer(context, tokenizer);
    let result = evaluator.eval(&expression.expression);

    match result {
        Ok(result) => EvalResult::Ok(result.to_string()),
        Err(error) => EvalResult::Err(error.to_string()),
    }
}
