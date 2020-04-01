use std::convert::TryFrom;
use bigdecimal::BigDecimal;
use math_engine::context::{Config, DefaultContext};
use math_engine::evaluator::Evaluator;
use crate::cmd::Command;
use crate::cmd::error::*;
use crate::cmd::commands::eval::utils::StringIterExt;
use crate::cmd::commands::eval_type::EvalType;

pub struct EvalCommand;
impl Command<String, Result> for EvalCommand {
    fn name(&self) -> &str {
        ""
    }

    fn alias(&self) -> Option<&str> {
        None
    }

    fn execute(&self, args: &[String]) -> Result {
        // eval --[option] [expression]
        if args.len() == 0 {
            return Err(Error::new("Empty, expected: eval [expression]"));
        }

        let eval_type = EvalType::try_from(args[0].as_str());

        let buffer = if eval_type.is_ok() {
            args.iter().skip(1).join_by(" ")
        } else {
            args.iter().join_by(" ")
        };

        match eval_type.unwrap_or_default() {
            EvalType::Decimal => {
                let config = Config::new().with_implicit_mul(true);
                let context = DefaultContext::new_decimal_with_config(config);
                let evaluator = Evaluator::with_context(context);
                match evaluator.eval(&buffer) {
                    Ok(n) => println!("{}", n),
                    Err(e) => eprintln!("{}", e),
                }
            }
            EvalType::BigDecimal => {
                let config = Config::new().with_implicit_mul(true);
                let context: DefaultContext<BigDecimal> =
                    DefaultContext::new_unchecked_with_config(config);
                let evaluator = Evaluator::with_context(context);
                match evaluator.eval(&buffer) {
                    Ok(n) => println!("{}", n),
                    Err(e) => eprintln!("{}", e),
                }
            }
            EvalType::Complex => {
                let config = Config::new()
                    .with_implicit_mul(true)
                    .with_complex_number(true);

                let context = DefaultContext::new_complex_with_config(config);
                let evaluator = Evaluator::with_context(context);
                match evaluator.eval(&buffer) {
                    Ok(n) => println!("{}", n),
                    Err(e) => eprintln!("{}", e),
                }
            }
        }

        Ok(())
    }
}

mod utils {
    pub trait StringIterExt {
        fn join_by(self, separator: &str) -> String;
    }

    impl<'a, I: Iterator<Item = &'a String>> StringIterExt for I {
        fn join_by(self, separator: &str) -> String {
            let mut result = String::new();
            let mut peekable = self.peekable();

            loop {
                if let Some(s) = peekable.next() {
                    result.push_str(s)
                } else {
                    break;
                }

                if peekable.peek().is_some() {
                    result.push_str(separator)
                }
            }

            result
        }
    }
}