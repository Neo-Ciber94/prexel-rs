use crate::cli::{Command, CommandArgs};
use crate::commands::internal::{CommandInfo, NumberType, StdKind};
use crate::custom_function::CustomFunction;
use bigdecimal::BigDecimal;
use crossterm::event::{self, Event, KeyCode};
use math_engine::context::validate::{check_token_name, TokenKind};
use math_engine::context::{Config, Context, DefaultContext};
use math_engine::error::{Error, ErrorKind};
use math_engine::evaluator::Evaluator;
use math_engine::Result;
use std::convert::TryFrom;
use std::fmt::{Debug, Display};
use std::iter::Iterator;
use std::rc::Rc;
use std::str::FromStr;
use math_engine::complex::Complex;
use crate::commands::internal;
use crossterm::style::Color;

pub struct RunCommand;
impl RunCommand {
    const TEXT_COLOR: Color = Color::Cyan;
    const RESULT_COLOR: Color = Color::White;
    const NEWLINE_COLOR: Color = Color::Green;
    const ERROR_COLOR: Color = Color::Red;
    const RESULT_VAR_NAME: &'static str = "result";
    const BACKSPACE: &'static str = "\x08 \x08";

    fn eval_expr<N>(buffer: &mut String, evaluator: &mut Rc<Evaluator<'_, N>>)
    where
        N: FromStr + Debug + Display + Clone,
    {
        if buffer.contains("=") {
            match Self::eval_assign(buffer, evaluator) {
                Ok(()) => {}
                Err(e) => {
                    internal::print_color(format!(" [Error] {}", e), Self::ERROR_COLOR, StdKind::Error)
                },
            }
        } else {
            match evaluator.eval(buffer) {
                Ok(n) => {
                    internal::print_color(format!(" = {}", n), Self::RESULT_COLOR, StdKind::Output);
                    Rc::make_mut(evaluator)
                        .mut_context()
                        .set_variable(Self::RESULT_VAR_NAME, n);
                }
                Err(e) => internal::print_color(format!(" [Error] {}", e), Self::ERROR_COLOR, StdKind::Error),
            }
        }

        internal::print_color("\n>> ", Self::NEWLINE_COLOR, StdKind::Output);
        buffer.clear();
    }

    fn eval_assign<N>(expression: &str, evaluator: &mut Rc<Evaluator<'_, N>>) -> Result<()>
    where N: FromStr + Debug + Display + Clone, {
        // Could be a variable assignment or a function assignment
        // * Variable Assignment: `variable_name = expression`.
        //      Eg.: `x = 10`, `y = x ^ 2`
        // * Function Assignment: `function_name(...args) = expression`.
        //      Eg.: `Double(x) = x * 2`
        let assignment: Vec<&str> = expression.split("=").collect::<Vec<&str>>();

        // We only need 2 parts: `variable_name` = `expression`
        if assignment.len() != 2 {
            return Err(Error::new(
                ErrorKind::InvalidExpression,
                "Invalid assignment expression",
            ));
        }

        let var = assignment[0].trim();
        let expr = assignment[1].trim();

        // If variable name contains parentheses we assume is a function
        if var.contains('(') && var.contains(')') {
            // Takes the entire expression
            match CustomFunction::from_str(evaluator.clone(), expression) {
                Ok(f) => {
                    let ev = Rc::make_mut(evaluator);
                    let context = ev.mut_context();

                    // Checks the function do not exists
                    // `DefaultContext` panics if tries to add a function that exists.
                    if context.is_function(f.name()) {
                        return Err(Error::new(
                            ErrorKind::Other,
                            format!("Function `{}` already exists in the context", f.name()),
                        ));
                    } else {
                        // Checks the function name is valid
                        check_token_name(TokenKind::Function, f.name())?;
                        context.add_function(f)
                    }
                }
                // Function parse failed
                Err(e) => {
                    return Err(Error::new(ErrorKind::InvalidInput, e));
                }
            }
        } else {
            // Checks the variable name is valid
            check_token_name(TokenKind::Variable, var)?;

            let value = evaluator.eval(expr)?;

            Rc::make_mut(evaluator)
                .mut_context()
                .set_variable(var, value);
        }

        Ok(())
    }
}

impl Command<String, Result<()>> for RunCommand {
    fn name(&self) -> &str {
        CommandInfo::Run.name()
    }

    fn alias(&self) -> Option<&str> {
        CommandInfo::Run.alias()
    }

    fn help_info(&self) -> &str {
        "\
Executes the evaluator in a loop

USAGE:
    eval --run | --r
    eval [--OPTION] --run | --r

OPTIONS:
    --decimal, --d          Evaluates using a 128 bits decimal number. Used by default
    --bigdecimal, --b       Evaluates using an arbitrary decimal number
    --complex, --c          Evaluates using a complex number

EXAMPLES:
    eval --run
    eval --r
    eval --bigdecimal --run
    eval --c --r

REMARKS:
    There are some unique behaviours when the evaluator runs using `--r | --run`:
    - The result of last operation is store in a variable called `result`.
    - You can assign variables: `variable_name = expression`.
        Eg.: x = 10, y = Sin(45)
    - You can create functions: `function_name(arguments) = expression`.
        Eg.: double(x) = x * 2, getThree() = 3"
    }

    fn execute(&self, args: CommandArgs<'_, String>) -> Result<()> {
        // Actual loop
        fn run<N: FromStr + Display + Debug + Clone>(mut evaluator: Rc<Evaluator<'_, N>>) {
            let mut buffer = String::new();
            internal::print_color(">> ", RunCommand::NEWLINE_COLOR, StdKind::Output);

            while let Event::Key(key) = event::read().unwrap() {
                match key.code {
                    KeyCode::Char(c) => {
                        if c.is_ascii_punctuation() || c.is_alphanumeric() || c == ' ' {
                            if c == ' ' && buffer.ends_with(' ') {
                                continue;
                            }

                            internal::print_color(c, RunCommand::TEXT_COLOR, StdKind::Output);
                            buffer.push(c);
                        }
                    }
                    KeyCode::Backspace => {
                        if !buffer.is_empty() {
                            internal::print_color(RunCommand::BACKSPACE, Color::White, StdKind::Output);
                            buffer.pop();
                        }
                    }
                    KeyCode::Enter => RunCommand::eval_expr(&mut buffer, &mut evaluator),
                    KeyCode::Esc => break,
                    _ => {}
                }
            }
        }

        // eval [--run | --r] [--OPTION]
        if args.len() > 1 {
            return Err(Error::new(
                ErrorKind::InvalidExpression,
                format!(
                    "Invalid arguments: expected: `eval [--option] [{}]`",
                    self.name()
                ),
            ));
        }

        let number_type = if args.is_empty() {
            NumberType::default()
        } else {
            NumberType::try_from(args[0].as_str()).unwrap_or_default()
        };

        match number_type {
            NumberType::Decimal => {
                let config = Config::new().with_implicit_mul(true);
                let context = DefaultContext::new_decimal_with_config(config);
                run(Rc::new(Evaluator::with_context(context)));
            }
            NumberType::BigDecimal => {
                let config = Config::new().with_implicit_mul(true);
                let context: DefaultContext<BigDecimal> =
                    DefaultContext::new_unchecked_with_config(config);
                run(Rc::new(Evaluator::with_context(context)));
            }
            NumberType::Complex => {
                let config = Config::new()
                    .with_implicit_mul(true)
                    .with_complex_number(true);
                let context = DefaultContext::<Complex<f64>>::new_complex_with_config(config);
                run(Rc::new(Evaluator::with_context(context)));
            }
        }

        // Just add a newline at the end
        println!();

        Ok(())
    }
}