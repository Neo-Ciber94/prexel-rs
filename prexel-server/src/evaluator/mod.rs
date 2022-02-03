use crate::models::{EvalExpression, EvalResult};
use once_cell::sync::Lazy;
use prexel::complex;
use prexel::context::Config;
use prexel::{context::Context, context::DefaultContext, decimal::Decimal, evaluator::Evaluator};
use std::str::FromStr;
use std::string::ToString;

static CONFIG: Lazy<Config> = Lazy::new(|| {
    Config::default()
        .with_group_symbol('[', ']')
        .with_group_symbol('(', ')')
        .with_implicit_mul(true)
});

pub fn eval_decimal_expression(expression: EvalExpression) -> EvalResult {
    let mut context = DefaultContext::new_decimal_with_config(CONFIG.clone());

    // Set variables
    if let Some(variables) = &expression.variables {
        for (name, value) in variables {
            match Decimal::from_str(value) {
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

pub fn eval_float_expression(expression: EvalExpression) -> EvalResult {
    let mut context = DefaultContext::with_config_checked(CONFIG.clone());

    // Set variables
    if let Some(variables) = &expression.variables {
        for (name, value) in variables {
            match f64::from_str(value) {
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

pub fn eval_complex_expression(expression: EvalExpression) -> EvalResult {
    let mut context =
        DefaultContext::new_complex_with_config(CONFIG.clone().with_complex_number(true));

    // Set variables
    if let Some(variables) = &expression.variables {
        for (name, value) in variables {
            match complex::Complex::<f64>::from_str(value) {
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
