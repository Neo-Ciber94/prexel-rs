use prexel::{context::Context, context::DefaultContext, decimal::Decimal, evaluator::Evaluator};

use crate::models::{EvalExpression, EvalResult};

pub fn eval_with_decimal(expression: EvalExpression<Decimal>) -> EvalResult<Decimal> {
    let mut context = DefaultContext::new_decimal();

    // Set variables
    if let Some(variables) = &expression.variables {
        for (name, value) in variables {
            context.set_variable(name, value.clone());
        }
    }

    // Evaluate expression
    let evaluator = Evaluator::with_context(context);
    let result = evaluator.eval(&expression.expression);
    
    match result {
        Ok(result) => EvalResult::new(result),
        Err(error) => EvalResult::with_error(error.to_string()),
    }
}
