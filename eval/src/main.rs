use std::env::args;

fn main() {
    if args().count() <= 1 {
        println!("No arguments passed, usage: eval --[options] [args]");
        return;
    }

    let arguments = args()
        .skip(1)
        .collect::<Vec<String>>();

    match arguments[0].as_str() {
        "--run" | "--r" => {
            if arguments.len() == 1 {
                eval_internal::run();
                println!();
            } else {
                println!("Invalid arguments, expected format: eval --[options] [args]");
            }
        }
        _ => {
            let buffer = arguments.iter()
                .fold(&mut String::new(), |string, s| {
                    string.push_str(s);
                    string
                }).clone();

            eval_single_arg(&buffer)
        }
    }
}

fn eval_single_arg(expression: &str) {
    use math_engine::evaluator::Evaluator;
    use math_engine::decimal::Decimal;
    use math_engine::context::DefaultContext;
    use math_engine::context::Config;

    let config = Config::new()
        .with_group_symbol('[', ']')
        .with_implicit_mul();

    let context = DefaultContext::new_decimal_with_config(config);
    let evaluator: Evaluator<Decimal> = Evaluator::with_context(context);

    match evaluator.eval(expression) {
        Ok(n) => println!("{}", n),
        Err(e) => println!("{:?}", e)
    }
}

mod eval_internal {
    use std::fmt::Display;
    use std::io::{stdout, Write};

    use crossterm::{execute, style::Print};
    use crossterm::event::{self, Event, KeyCode};
    use crossterm::style::Color;
    use crossterm::style::ResetColor;
    use crossterm::style::SetForegroundColor;

    use math_engine::context::{Config, Context};
    use math_engine::context::DefaultContext;
    use math_engine::decimal::Decimal;
    use math_engine::error::{Error, ErrorKind};
    use math_engine::evaluator::Evaluator;
    use math_engine::Result;

    const BACKSPACE: &str = "\x08 \x08";

    const TEXT_COLOR: Color = Color::Cyan;
    const RESULT_COLOR: Color = Color::White;
    const NEWLINE_COLOR: Color = Color::Green;
    const ERROR_COLOR: Color = Color::Red;

    pub(crate) fn run() {
        let config = Config::new().with_implicit_mul();
        let context = DefaultContext::new_decimal_with_config(config);
        let mut evaluator: Evaluator<Decimal> = Evaluator::with_context(context);
        let mut buffer = String::new();

        print_color(">> ", NEWLINE_COLOR);

        while let Event::Key(key) = event::read().unwrap() {
            match key.code {
                KeyCode::Char(c) => if c.is_ascii_punctuation() || c.is_alphanumeric() || c == ' '{
                    if c == ' ' && buffer.ends_with(' '){
                        continue;
                    }

                    print_color(c, TEXT_COLOR);
                    buffer.push(c);
                },
                KeyCode::Backspace => {
                    if !buffer.is_empty() {
                        print_color(BACKSPACE, Color::White);
                        buffer.pop();
                    }
                }
                KeyCode::Enter => eval_expr(&mut buffer, &mut evaluator),
                KeyCode::Esc => break,
                _ => {}
            }
        }
    }

    fn eval_expr(buffer: &mut String, mut evaluator: &mut Evaluator<'_, Decimal>) {
        if buffer.contains("=") {
            match eval_assign(&buffer, &mut evaluator) {
                Ok(()) => {}
                Err(e) => {
                    print_color(
                        format!(" [Error] {}", e),
                        ERROR_COLOR,
                    );
                }
            }
        } else {
            match evaluator.eval(buffer.as_str()) {
                Ok(n) => {
                    print_color(
                        format!(" = {}", n),
                        RESULT_COLOR,
                    );
                    evaluator.mut_context().set_variable("result", n);
                }
                Err(e) => {
                    print_color(
                        format!(" [Error] {}", e),
                        ERROR_COLOR,
                    );
                }
            }
        }

        print_color("\n>> ", NEWLINE_COLOR);
        buffer.clear();
    }

    fn eval_assign(buffer: &String, evaluator: &mut Evaluator<'_, Decimal>) -> Result<()> {
        let assignment: Vec<&str> = buffer.split("=").collect::<Vec<&str>>();
        debug_assert!(assignment.len() == 2);

        let var = assignment[0].trim();
        let expr = assignment[1].trim();

        let first_char = var.chars().next()
            .ok_or(Error::new(ErrorKind::InvalidInput, "Empty variable"))?;

        if !first_char.is_ascii_alphabetic() {
            return Err(
                Error::new(
                    ErrorKind::InvalidExpression,
                    "Variable names must start with a letter")
            );
        }

        let value = evaluator.eval(expr)?;
        evaluator.mut_context().set_variable(var, value);
        Ok(())
    }

    fn print_color<T: Display + Clone>(value: T, color: Color) {
        execute!(
            stdout(),
            SetForegroundColor(color),
            Print(value),
            ResetColor
        ).unwrap();
    }
}