use crate::context::{Config, DefaultContext};

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

/// Evaluates the specified math expression.
pub fn eval<'a, T>(expression: &str) -> Result<T>
    where T: num::unchecked::UncheckedNum
    + std::panic::RefUnwindSafe
    + std::panic::UnwindSafe
    + 'static,
{
    use evaluator::Evaluator;
    use std::any::TypeId;
    use std::cell::RefCell;
    use std::collections::HashMap;
    use std::panic::*;

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

    /// Gets a `static` value initialized with the given function.
    fn load_static<T: UnwindSafe, F: FnOnce() -> T>(f: F) -> &'static T {
        thread_local! {
            static STATIC_DATA: RefCell<HashMap<TypeId, *const ()>> = RefCell::new(HashMap::new());
        }

        STATIC_DATA.with(|map| {
            let raw = map
                .borrow_mut()
                .entry(TypeId::of::<T>())
                .or_insert(Box::into_raw(Box::new(f())) as *const ())
                .clone();

            unsafe { &*(raw as *const T) }
        })
    }

    let result = catch_panic(move || {
        let evaluator = load_static(move || {
            let config = Config::new()
                .with_group_symbol('[', ']')
                .with_implicit_mul();

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