use crate::models::{EvalExpression, EvalResult};
use prexel::complex;
use prexel::{context::Context, context::DefaultContext, decimal::Decimal, evaluator::Evaluator};
use std::str::FromStr;
use std::string::ToString;

pub fn eval_decimal_expression(expression: EvalExpression) -> EvalResult {
    let mut context = DefaultContext::new_decimal();

    // Set variables
    if let Some(variables) = &expression.variables {
        for (name, value) in variables {
            match Decimal::from_str(value) {
                Ok(value) => {
                    context.set_variable(name, value);
                }
                Err(err) => return EvalResult::with_error(format!("{}", err)),
            }
        }
    }

    // Evaluate expression
    let evaluator = Evaluator::with_context(context);
    let result = evaluator.eval(&expression.expression);

    match result {
        Ok(result) => EvalResult::new(result.to_string()),
        Err(error) => EvalResult::with_error(error.to_string()),
    }
}

pub fn eval_float_expression(expression: EvalExpression) -> EvalResult {
    let mut context = DefaultContext::new_checked();

    // Set variables
    if let Some(variables) = &expression.variables {
        for (name, value) in variables {
            match f64::from_str(value) {
                Ok(value) => {
                    context.set_variable(name, value);
                }
                Err(err) => return EvalResult::with_error(format!("{}", err)),
            }
        }
    }

    // Evaluate expression
    let evaluator = Evaluator::with_context(context);
    let result = evaluator.eval(&expression.expression);

    match result {
        Ok(result) => EvalResult::new(result.to_string()),
        Err(error) => EvalResult::with_error(error.to_string()),
    }
}

pub fn eval_complex_expression(expression: EvalExpression) -> EvalResult {
    let mut context = DefaultContext::new_complex();

    // Set variables
    if let Some(variables) = &expression.variables {
        for (name, value) in variables {
            match complex::Complex::<f64>::from_str(value) {
                Ok(value) => {
                    context.set_variable(name, value);
                }
                Err(err) => return EvalResult::with_error(format!("{}", err)),
            }
        }
    }

    // Evaluate expression
    let evaluator = Evaluator::with_context(context);
    let result = evaluator.eval(&expression.expression);

    match result {
        Ok(result) => EvalResult::new(result.to_string()),
        Err(error) => EvalResult::with_error(error.to_string()),
    }
}