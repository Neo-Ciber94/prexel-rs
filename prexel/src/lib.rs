pub mod context;
pub mod error;
pub mod evaluator;
pub mod function;
pub mod num;
pub mod ops;
pub mod token;
pub mod tokenizer;
pub mod utils;

/// Reexports of the `num_traits` crate.
pub mod num_traits {
    pub use num_traits::*;
}

/// An convenient result type used for returning the result of evaluations.
pub type Result<T> = std::result::Result<T, error::Error>;

#[cfg(feature = "decimal")]
pub mod decimal;

#[cfg(feature = "complex")]
pub mod complex;