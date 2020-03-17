use math_eval::console;
use math_eval::console::keycode::*;
use math_eval::console::console_color::{ConsoleColor, Color};
use math_engine::evaluator::Evaluator;
use math_engine::context::{Context, Config, DefaultContext};
use math_engine::error::{Result, Error, ErrorKind};

const WHITE_SPACE: &str = " ";
const BACKSPACE: &str = "\x08 \x08";

fn main() {
    let config = Config::new().with_implicit_mul();
    let context = DefaultContext::new_checked_with_config(config);
    let mut evaluator: Evaluator<f64> = Evaluator::with_context(context);
    let mut buffer = String::new();

    console::write_with_color("> ", ConsoleColor::with_foreground(Color::Yellow));

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
                    console::write(ch);
                    buffer.push(ch);
                }
            }
        }
    }
}

fn eval_expr(buffer: &mut String, mut evaluator: &mut Evaluator<'_, f64>) {
    if buffer.contains("="){
        match eval_assign(&buffer, &mut evaluator){
            Ok(()) => {},
            Err(e) => {
                console::write_with_color(
                    format!(" [Error] {}", e),
                    ConsoleColor::with_foreground(Color::Red)
                );
            }
        }
    }
    else{
        match evaluator.eval(buffer.as_str()) {
            Ok(n) => {
                console::write(format!(" = {}", n));
            },
            Err(e) => {
                console::write_with_color(
                    format!(" [Error] {}", e),
                    ConsoleColor::with_foreground(Color::Red)
                );
            }
        }
    }

    console::write_with_color("\n> ", ConsoleColor::with_foreground(Color::Yellow));
    buffer.clear();
}

fn eval_assign(buffer: &String, evaluator: &mut Evaluator<'_, f64>) -> Result<()>{
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