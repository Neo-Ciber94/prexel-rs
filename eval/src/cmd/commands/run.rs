use std::convert::TryFrom;
use std::fmt::{Debug, Display, Formatter};
use std::io::{stdout, Write};
use std::iter::Iterator;
use std::rc::Rc;
use std::str::FromStr;
use bigdecimal::BigDecimal;
use crossterm::event::{self, Event, KeyCode};
use crossterm::execute;
use crossterm::style::{Color, Print, ResetColor, SetForegroundColor};
use math_engine::context::{Config, Context, DefaultContext};
use math_engine::error::{Error as MathError, ErrorKind};
use math_engine::evaluator::Evaluator;
use crate::cmd::Command;
use crate::cmd::commands::eval_type::EvalType;
use crate::cmd::error::*;
use crate::custom_function::CustomFunction;

pub struct RunCommand;
impl RunCommand{
    const TEXT_COLOR: Color = Color::Cyan;
    const RESULT_COLOR: Color = Color::White;
    const NEWLINE_COLOR: Color = Color::Green;
    const ERROR_COLOR: Color = Color::Red;
    const RESULT_VAR_NAME : &'static str  = "result";
    const BACKSPACE: &'static str = "\x08 \x08";

    fn eval_expr<N>(buffer: &mut String, evaluator: &mut Rc<Evaluator<'_, N>>) where N: FromStr + Debug + Display + Clone {
        if buffer.contains("="){
            match Self::eval_assign(buffer, evaluator){
                Ok(()) => {},
                Err(e) => print_color(format!(" [Error] {}", e), Self::ERROR_COLOR)
            }
        }
        else{
            match evaluator.eval(buffer) {
                Ok(n) => {
                    print_color(format!(" = {}", n), Self::RESULT_COLOR);
                    Rc::make_mut(evaluator)
                        .mut_context()
                        .set_variable(Self::RESULT_VAR_NAME, n);
                }
                Err(e) => {
                    print_color(format!(" [Error] {}", e), Self::ERROR_COLOR)
                }
            }
        }

        print_color("\n>> ", Self::NEWLINE_COLOR);
        buffer.clear();
    }

    fn eval_assign<N>(expression: &str, evaluator: &mut Rc<Evaluator<'_, N>>) -> math_engine::Result<()>
        where N: FromStr + Debug + Display + Clone
    {
        // Could be a variable assignment or a function assignment
        // * Variable Assignment: `VariableName = expression`.
        //      Eg.: `x = 10`, `y = x ^ 2`
        // * Function Assignment: `FunctionName(...args) = expression`.
        //      Eg.: `Double(x) = x * 2`
        let assignment: Vec<&str> = expression.split("=").collect::<Vec<&str>>();

        // We only need 2 parts: `Variable` = `Expression`
        if assignment.len() != 2{
            return Err(MathError::new(
                ErrorKind::InvalidExpression,
                "Invalid assignment expression"
            ));
        }

        let var = assignment[0].trim();
        let expr = assignment[1].trim();

        // If variable name contains parentheses we assume is a function
        if var.contains('(') && var.contains(')') {
            match CustomFunction::from_str(evaluator.clone(), var){
                Ok(f) => {
                    let ev = Rc::make_mut(evaluator);
                    let context = ev.mut_context();

                    // Checks the function do not exists
                    // `DefaultContext` panics if tries to add a function that exists.
                    if context.is_function(f.name()){
                        return Err(MathError::new(
                            ErrorKind::Other,
                            format!("Function `{}` already exists in the context", f.name())
                        ));
                    }
                    else{
                        context.add_function(f)
                    }
                },
                // Function parse failed
                Err(e) => {
                    return Err(MathError::new(
                        ErrorKind::InvalidInput,
                        e
                    ));
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

impl Command<String, Result> for RunCommand{
    fn name(&self) -> &str {
        "--run"
    }

    fn alias(&self) -> Option<&str> {
        Some("--r")
    }

    fn execute(&self, args: &[String]) -> Result {
        // Actual loop
        fn run<N: FromStr + Display + Debug + Clone>(mut evaluator: Rc<Evaluator<'_, N>>){
            let mut buffer = String::new();
            print_color(">> ", RunCommand::NEWLINE_COLOR);

            while let Event::Key(key) = event::read().unwrap() {
                match key.code {
                    KeyCode::Char(c) => if c.is_ascii_punctuation() || c.is_alphanumeric() || c == ' '{
                        if c == ' ' && buffer.ends_with(' '){
                            continue;
                        }

                        print_color(c, RunCommand::TEXT_COLOR);
                        buffer.push(c);
                    },
                    KeyCode::Backspace => {
                        if !buffer.is_empty() {
                            print_color(RunCommand::BACKSPACE, Color::White);
                            buffer.pop();
                        }
                    }
                    KeyCode::Enter => RunCommand::eval_expr(&mut buffer, &mut evaluator),
                    KeyCode::Esc => break,
                    _ => {}
                }
            }
        }

        // eval --run | --r --[option]
        if args.len() > 1 {
            return Err(Error::new(
                format!(
                    "Invalid arguments: expected: eval {} | {} --[option]",
                    self.name(),
                    self.alias().unwrap()))
            );
        }

        let eval_type = if args.is_empty(){
            EvalType::default()
        }
        else{
            EvalType::try_from(args[0].as_str())
                .unwrap_or_default()
        };

        match eval_type {
            EvalType::Decimal => {
                let config = Config::new().with_implicit_mul(true);
                let context = DefaultContext::new_decimal_with_config(config);
                run(Rc::new(Evaluator::with_context(context)));
            }
            EvalType::BigDecimal => {
                let config = Config::new().with_implicit_mul(true);
                let context: DefaultContext<BigDecimal> =
                    DefaultContext::new_unchecked_with_config(config);
                run(Rc::new(Evaluator::with_context(context)));
            }
            EvalType::Complex => {
                let config = Config::new()
                    .with_implicit_mul(true)
                    .with_complex_number(true);
                let context = DefaultContext::new_complex_with_config(config);
                run(Rc::new(Evaluator::with_context(context)));
            }
        }

        Ok(())
    }
}

#[inline]
fn print_color<T: Display + Clone>(value: T, color: Color) {
    execute!(
            stdout(),
            SetForegroundColor(color),
            Print(value),
            ResetColor
        ).unwrap();
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
enum TokenKind{ Variable, Function }
impl Display for TokenKind{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match *self{
            TokenKind::Variable => write!(f, "Variable"),
            TokenKind::Function => write!(f, "Function"),
        }
    }
}

fn check_token_name(kind: TokenKind, name: &str) -> math_engine::Result<()>{
    if name.is_empty(){
        return Err(MathError::new(
            ErrorKind::InvalidInput,
            format!("{} names cannot be empty", kind)
        ));
    }

    if name.chars().any(char::is_whitespace){
        return Err(MathError::new(
            ErrorKind::InvalidInput,
            format!("{} names cannot contains whitespaces", kind)
        ));
    }

    if !name.chars().all(char::is_alphanumeric){
        return Err(MathError::new(
            ErrorKind::InvalidInput,
            format!("{}s can only contain alphanumeric characters", kind)
        ));
    }

    if name.chars().next().unwrap().is_alphabetic(){
        return Err(MathError::new(
            ErrorKind::InvalidInput,
            format!("{} names should start if a letter", kind)
        ));
    }

    Ok(())
}