/// Context with functions, variables and constants for evaluating expressions.
pub mod context;

/// Common errors for prexel.
pub mod error;

/// Evaluator for the math expressions.
pub mod evaluator;

/// Traits for functions.
pub mod function;

/// Common traits for numeric types.
pub mod num;

/// Implementations of the function traits for common math operations.
pub mod ops;

/// Tokens used for prexel.
pub mod token;

/// The tokenizer for prexel.
pub mod tokenizer;

/// Common utilities.
pub mod utils;

/// Extra num traits
mod numeric_traits;

/// Reexports of the `num_traits` crate.
pub mod num_traits {
    pub use num_traits::*;
    pub use crate::numeric_traits::*;
}

/// An convenient result type used for returning the result of evaluations.
pub type Result<T> = std::result::Result<T, error::Error>;

/// Support for decimal numbers.
#[cfg(feature = "decimal")]
pub mod decimal;

/// Support for complex numbers.
#[cfg(feature = "complex")]
pub mod complex;

#[doc(hidden)]
#[cfg(feature = "docs")]
pub mod descriptions;

/// Support for binary numbers.
#[cfg(feature = "binary")]
pub mod binary;