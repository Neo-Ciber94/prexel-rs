use std::i128;
use crate::models::{EvalExpression, EvalResult, NumberType};
use once_cell::sync::Lazy;
use prexel::complex;
use prexel::context::Config;
use prexel::{context::Context, context::DefaultContext, decimal::Decimal, evaluator::Evaluator};
use std::str::FromStr;
use std::string::ToString;
use prexel::tokenizer::Tokenizer;
use crate::context::{Binary, binary_number_splitter, BinaryContext};

static CONFIG: Lazy<Config> = Lazy::new(|| {
    Config::default()
        .with_group_symbol('[', ']')
        .with_group_symbol('(', ')')
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
    let mut context = DefaultContext::new_decimal_with_config(CONFIG.clone());

    // Set variables
    if let Some(variables) = &expression.variables {
        for (name, value) in variables {
            match Decimal::from_str(&value.to_string()) {
                Ok(value) => {
                    context.set_variable(name, value);
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
                    context.set_variable(name, value);
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
    let mut context =
        DefaultContext::new_complex_with_config(CONFIG.clone().with_complex_number(true));

    // Set variables
    if let Some(variables) = &expression.variables {
        for (name, value) in variables {
            match complex::Complex::<f64>::from_str(&value.to_string()) {
                Ok(value) => {
                    context.set_variable(name, value);
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
    let mut context =
        DefaultContext::with_config_checked(CONFIG.clone());

    // Set variables
    if let Some(variables) = &expression.variables {
        for (name, value) in variables {
            match i64::from_str(&value.to_string()) {
                Ok(value) => {
                    context.set_variable(name, value);
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
    let mut context =
        DefaultContext::with_config_binary(CONFIG.clone());

    // Set variables
    if let Some(variables) = &expression.variables {
        for (name, value) in variables {
            match i128::from_str(&value.to_string()) {
                Ok(value) => {
                    context.set_variable(name, Binary(value));
                }
                Err(err) => return EvalResult::Err(format!("{}", err)),
            }
        }
    }

    // Evaluate expression
    let tokenizer = Tokenizer::with_splitter(&context, binary_number_splitter());
    let tokens = tokenizer.tokenize(&expression.expression).map_err(|err| format!("{}", err))?;
    let evaluator = Evaluator::with_context(context);
    let result = evaluator.eval_tokens(&tokens);

    match result {
        Ok(result) => EvalResult::Ok(result.to_string()),
        Err(error) => EvalResult::Err(error.to_string()),
    }
}