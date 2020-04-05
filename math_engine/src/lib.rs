pub mod context;
pub mod error;
pub mod evaluator;
pub mod function;
pub mod num;
pub mod ops;
pub mod token;
pub mod tokenizer;
pub mod utils;

/// An convenient result type used for returning the result of evaluations.
pub type Result<T> = std::result::Result<T, error::Error>;

#[cfg(feature = "decimal")]
pub mod decimal;

#[cfg(feature = "complex")]
pub mod complex;

/// Evaluates the specified math expression and gets the result as `Result<T>`.
///
/// # Remarks
/// Type `T` must implement:
/// - Basic numeric operations using: `Add`, `Sub`, `Mul`, `Div`, `Rem` and `Neg` traits.
/// - Conversion from `0` and `1` with the traits `Zero` and `One`.
/// - Conversion from and to primitive types by: `FromPrimitive` and `ToPrimitive`.
/// - Ordering using: `PartialOrd`.
/// - Conversion form `&str` using `FromStr` trait.
/// - Provides display and debug using: `Debug` and `Display`
/// - Provides cloning by: `Clone`
///
/// Primitive types as: `i8`, `i16`, `i32`, `i64`, `i128`, `f32`, `f64` meet all those conditions.
///
/// NOTE: Unsigned types don't implement `Neg`.
///
/// If a more complex behaviour is needed as custom functions, operators, variables, constants, etc.
/// [`DefaultContext`] and [`Evaluator`] should be used diretly.
///
/// # Example
/// ```
/// assert_eq!(Ok(17), math_engine::eval::<i32>("2 + 3 * 5"));
/// assert_eq!(Ok(100_f32), math_engine::eval::<f32>("10^2"));
/// assert!(math_engine::eval::<f64>("10/0").is_err())
/// ```
///
/// [`DefaultContext`]: context/struct.DefaultContext.html
/// [`Evaluator`]: evaluator/struct.Evaluator.html
pub fn eval<T>(expression: &str) -> Result<T>
where
    T: num::unchecked::UncheckedNum + std::panic::RefUnwindSafe + std::panic::UnwindSafe + 'static,
{
    use context::Config;
    use context::DefaultContext;
    use evaluator::Evaluator;
    use std::panic::*;
    use crate::utils::static_store::StaticStore;

    /// Allow to catch `panic`s without print error messages.
    fn catch_panic<F: FnOnce() -> R + UnwindSafe, R>(
        f: F,
    ) -> std::result::Result<R, Box<dyn std::any::Any + Send + 'static>> {
        let prev_hook = std::panic::take_hook();
        set_hook(Box::new(|_| {}));
        let result = std::panic::catch_unwind(f);
        set_hook(prev_hook);
        result
    }

    // Holds the data of the evaluators
    static STATIC_EVALUATOR : StaticStore = StaticStore::new();

    let result = catch_panic(move || {
        let evaluator = STATIC_EVALUATOR.load(move || {
            let config = Config::new()
                .with_implicit_mul(true)
                .with_group_symbol('[', ']');

            let context = DefaultContext::new_unchecked_with_config(config);
            let temp: Evaluator<T> = Evaluator::with_context(context);
            temp
        });

        evaluator.eval(expression)
    });

    match result {
        Ok(n) => n,
        Err(_) => Err(error::Error::from(error::ErrorKind::Overflow)),
    }
}
