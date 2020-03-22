use math_engine::evaluator::Evaluator;
use math_engine::decimal::Decimal;
use math_engine::context::DefaultContext;
use std::env::args;

fn main(){
    if args().count() <= 1{
        println!("No arguments passed, usage: eval --[options] [args]");
        return;
    }

    let arguments = args().skip(1).collect::<Vec<String>>();
    match arguments[0].as_str() {
        "--run" | "--r" => {
            if arguments.len() == 1{
                evaluator::run();
            }
            else{
                println!("Invalid arguments, expected format: eval --[options] [args]");
            }
        }
        _ => {
            let buffer = arguments.iter().fold(&mut String::new(), |buf, s|{
                buf.push_str(s);
                buf
            }).clone();

            eval(&buffer)
        }
    }
}

fn eval(expression: &str){
    let context = DefaultContext::new_decimal();
    let evaluator : Evaluator<Decimal> = Evaluator::with_context(context);
    match evaluator.eval(expression){
        Ok(n) => println!("{}", n),
        Err(e) => println!("{:?}", e)
    }
}

mod evaluator {
    use eval::console;
    use eval::console::keycode::*;
    use eval::console::console_color::{ConsoleColor, Color};
    use math_engine::evaluator::Evaluator;
    use math_engine::context::{Context, Config, DefaultContext};
    use math_engine::error::{Result, Error, ErrorKind};
    use math_engine::decimal::Decimal;

    const WHITE_SPACE: &str = " ";
    const BACKSPACE: &str = "\x08 \x08";

    const TEXT_COLOR: Color = Color::BrightCyan;
    const RESULT_COLOR: Color = Color::BrightCyan;
    const NEWLINE_COLOR : Color = Color::BrightGreen;
    const ERROR_COLOR : Color = Color::BrightRed;

    pub fn run() {
        let config = Config::new().with_implicit_mul();
        let context = DefaultContext::new_decimal_with_config(config);
        let mut evaluator: Evaluator<Decimal> = Evaluator::with_context(context);
        let mut buffer = String::new();

        console::write_with_color(">> ", ConsoleColor::with_foreground(NEWLINE_COLOR));

        loop {
            let key = console::read();

            match key.key_code() {
                KeyCode::Space => {
                    console::write(WHITE_SPACE);
                    buffer.push_str(WHITE_SPACE);
                }
                KeyCode::Backspace => {
                    if !buffer.is_empty() {
                        console::write(BACKSPACE);
                        buffer.pop();
                    }
                }
                KeyCode::Escape => { return; }
                KeyCode::Enter => { eval_expr(&mut buffer, &mut evaluator) }
                _ => {
                    let ch = key.char_value();
                    if ch.is_alphanumeric() || ch.is_ascii_punctuation() {
                        console::write_with_color(ch, ConsoleColor::with_foreground(TEXT_COLOR));
                        buffer.push(ch);
                    }
                }
            }
        }
    }

    fn eval_expr(buffer: &mut String, mut evaluator: &mut Evaluator<'_, Decimal>) {
        if buffer.contains("="){
            match eval_assign(&buffer, &mut evaluator){
                Ok(()) => {},
                Err(e) => {
                    console::write_with_color(
                        format!(" [Error] {}", e),
                        ConsoleColor::with_foreground(ERROR_COLOR)
                    );
                }
            }
        }
        else{
            match evaluator.eval(buffer.as_str()) {
                Ok(n) => {
                    console::write_with_color(
                        format!(" = {}", n),
                        ConsoleColor::with_foreground(RESULT_COLOR)
                    );
                    evaluator.mut_context().set_variable("result", n);
                },
                Err(e) => {
                    console::write_with_color(
                        format!(" [Error] {}", e),
                        ConsoleColor::with_foreground(ERROR_COLOR)
                    );
                }
            }
        }

        console::write_with_color("\n>> ", ConsoleColor::with_foreground(NEWLINE_COLOR));
        buffer.clear();
    }

    fn eval_assign(buffer: &String, evaluator: &mut Evaluator<'_, Decimal>) -> Result<()>{
        let assignment: Vec<&str> = buffer.split("=").collect::<Vec<&str>>();
        debug_assert!(assignment.len() == 2);

        let var = assignment[0].trim();
        let expr = assignment[1].trim();

        let first_char = var.chars().nth(0)
            .ok_or(Error::new(ErrorKind::InvalidInput,"Empty variable"))?;

        if !first_char.is_ascii_alphabetic(){
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
}