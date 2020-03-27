use crate::context::{Config, DefaultContext};

pub mod num;
pub mod token;
pub mod tokenizer;
pub mod context;
pub mod evaluator;
pub mod function;
pub mod error;
pub mod utils;
pub mod ops;

/// An convenient result type used for returning the result of evaluations.
pub type Result<T> = std::result::Result<T, error::Error>;

pub fn eval<'a, T>(expression: &str) -> Result<T> where T: num::checked::CheckedNum{
    let config = Config::new()
        .with_group_symbol('[', ']')
        .with_implicit_mul();

    let context = DefaultContext::new_checked_with_config(config);
    let evaluator : evaluator::Evaluator<T> = evaluator::Evaluator::with_context(context);
    evaluator.eval(expression)
}

#[cfg(feature = "decimal")]
pub mod decimal;

#[cfg(feature = "complex")]
pub mod complex;