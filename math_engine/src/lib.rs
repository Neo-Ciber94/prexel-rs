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

#[cfg(feature = "decimal")]
pub mod decimal;

#[cfg(feature = "complex")]
pub mod complex;