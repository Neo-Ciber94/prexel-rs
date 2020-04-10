use std::convert::TryFrom;
use bigdecimal::BigDecimal;
use math_engine::context::{Config, DefaultContext};
use math_engine::evaluator::Evaluator;
use math_engine::Result;
use math_engine::error::{Error, ErrorKind};
use math_engine::complex::Complex;
use crate::cli::{Command, CommandArgs};
use crate::commands::internal::{NumberType, CommandInfo, StdKind};
use crate::commands::eval::utils::StringIterExt;
use crate::commands::internal;
use crossterm::style::Color;

pub struct EvalCommand;
impl Command<String, Result<()>> for EvalCommand {
    fn name(&self) -> &str {
        CommandInfo::Eval.name()
    }

    fn alias(&self) -> Option<&str> {
        CommandInfo::Eval.alias()
    }

    fn help_info(&self) -> &str {
        "\
Evaluates a math expression

USAGE:
    eval [EXPRESSION]
    eval [--OPTION] [EXPRESSION]

OPTIONS:
    --decimal, --d          Evaluates using a 128 bits decimal number. Used by default
    --bigdecimal, --b       Evaluates using an arbitrary decimal number
    --complex, --c          Evaluates using a complex number

EXAMPLES:
    eval 10 + 2
    eval Sin(45) * 2
    eval --complex (5+3i) + (7+5i)
    eval --b 100!"
    }

    fn execute(&self, args: CommandArgs<'_, String>) -> Result<()> {
        if args.len() == 0 {
            return Err(Error::new(
                ErrorKind::InvalidExpression,
                "Empty expression. See `eval --help` for help information")
            );
        }

        let number_type = NumberType::try_from(args[0].as_str());

        let buffer = if number_type.is_ok() {
            args.iter().skip(1).join_by(" ")
        } else {
            args.iter().join_by(" ")
        };

        match number_type.unwrap_or_default() {
            NumberType::Decimal => {
                let config = Config::new().with_implicit_mul(true);
                let context = DefaultContext::new_decimal_with_config(config);
                let evaluator = Evaluator::with_context(context);
                match evaluator.eval(&buffer) {
                    Ok(n) => println!("{}", n),
                    Err(e) => eprintln!("{}", e)
                }
            }
            NumberType::BigDecimal => {
                let config = Config::new().with_implicit_mul(true);
                let context: DefaultContext<BigDecimal> =
                    DefaultContext::new_unchecked_with_config(config);
                let evaluator = Evaluator::with_context(context);
                match evaluator.eval(&buffer) {
                    Ok(n) => {
                        internal::print_color(n, Color::Green, StdKind::Output)
                    },
                    Err(e) => {
                        internal::print_color(e.to_string(), Color::Red, StdKind::Error)
                    },
                }
            }
            NumberType::Complex => {
                let config = Config::new()
                    .with_implicit_mul(true)
                    .with_complex_number(true);

                let context = DefaultContext::<Complex<f64>>::new_complex_with_config(config);
                let evaluator = Evaluator::with_context(context);
                match evaluator.eval(&buffer) {
                    Ok(n) => {
                        internal::print_color(n, Color::Green, StdKind::Output)
                    },
                    Err(e) => {
                        internal::print_color(e.to_string(), Color::Red, StdKind::Error)
                    },
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