use std::convert::TryFrom;
use std::result;
use math_engine::context::{Config, DefaultContext};
use math_engine::evaluator::Evaluator;
use crate::cmd::error::{Error, Result};
use crate::cmd::Command;
use bigdecimal::BigDecimal;
use utils::StringIterExt;

pub enum NumberKind {
    Decimal,
    BigDecimal,
    Complex,
}

impl TryFrom<&str> for NumberKind {
    type Error = ();

    fn try_from(value: &str) -> result::Result<Self, Self::Error> {
        match value {
            "--decimal" | "--d" => Ok(NumberKind::Decimal),
            "--bigdecimal" | "--b" => Ok(NumberKind::BigDecimal),
            "--complex" | "--c" => Ok(NumberKind::Complex),
            _ => Err(()),
        }
    }
}

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

        let kind = NumberKind::try_from(args[0].as_str());

        let buffer = if kind.is_ok() {
            args.iter().skip(1).join_by(" ")
        } else {
            args.iter().join_by(" ")
        };

        match kind.unwrap_or(NumberKind::Decimal) {
            NumberKind::Decimal => {
                let config = Config::new().with_implicit_mul();
                let context = DefaultContext::new_decimal_with_config(config);
                let evaluator = Evaluator::with_context(context);
                match evaluator.eval(&buffer) {
                    Ok(n) => println!("{}", n),
                    Err(e) => eprintln!("{:?}", e),
                }
            }
            NumberKind::BigDecimal => {
                let config = Config::new().with_implicit_mul();
                let context: DefaultContext<BigDecimal> =
                    DefaultContext::new_unchecked_with_config(config);
                let evaluator = Evaluator::with_context(context);
                match evaluator.eval(&buffer) {
                    Ok(n) => println!("{}", n),
                    Err(e) => eprintln!("{:?}", e),
                }
            }
            NumberKind::Complex => {
                let config = Config::new().with_implicit_mul().with_complex_number();

                let context = DefaultContext::new_complex_with_config(config);
                let evaluator = Evaluator::with_context(context);
                match evaluator.eval(&buffer) {
                    Ok(n) => println!("{}", n),
                    Err(e) => eprintln!("{:?}", e),
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
