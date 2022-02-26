use crate::eval_expr::CONFIG;
use crate::{eprint_colored, print_colored, readln, EvalType};
use prexel::complex::Complex;
use prexel::context::{Context, DefaultContext};
use prexel::evaluator::Evaluator;
use prexel::num_traits::Zero;
use prexel::tokenizer::Tokenizer;
use prexel::utils::splitter::{DefaultSplitter, Outcome};
use std::fmt::{Debug, Display};
use std::str::FromStr;
use termcolor::Color;

pub fn run_repl(eval_type: EvalType) {
    match eval_type {
        EvalType::Decimal => {
            let context = DefaultContext::with_config_decimal(CONFIG.lock().unwrap().clone());
            eval_loop(move || context)
        }
        EvalType::Float => {
            let context =
                DefaultContext::<f64>::with_config_unchecked(CONFIG.lock().unwrap().clone());
            eval_loop(move || context)
        }
        EvalType::Integer => {
            let context =
                DefaultContext::<i128>::with_config_checked(CONFIG.lock().unwrap().clone());
            eval_loop(move || context)
        }
        EvalType::Complex => {
            let context = DefaultContext::<Complex<f64>>::with_config_complex(
                CONFIG.lock().unwrap().clone().with_complex_number(true),
            );
            eval_loop(move || context)
        }
    }
}

fn eval_loop<'a, N, F>(factory: F)
where
    N: FromStr + Clone + Display + Debug + Zero + 'a,
    F: FnOnce() -> DefaultContext<'a, N>,
    <N as FromStr>::Err: Display,
{
    const RESULT: &str = "$result";

    let mut context = factory();
    context.set_variable(RESULT, N::zero()).unwrap();

    let tokenizer = repl_tokenizer();
    let mut evaluator = Evaluator::with_context_and_tokenizer(context, tokenizer);
    loop {
        print_prompt();
        let input = readln!();
        let expression = input.trim();

        print_prompt();

        match expression {
            // Exits the REPL
            "exit" => {
                break;
            }
            // Assign a variable
            _ if expression.contains('=') => {
                let parts = expression
                    .split('=')
                    .map(|s| s.trim().to_owned())
                    .collect::<Vec<_>>();

                if parts.len() != 2 {
                    eprint_colored!(Color::Red, "Invalid assignment");
                } else {
                    // TODO: Check if the variable is a valid identifier
                    let variable = &parts[0];
                    match N::from_str(&parts[1]) {
                        Ok(value) => {
                            if let Err(err) = evaluator.mut_context().set_variable(variable, value)
                            {
                                eprint_colored!(Color::Red, "{}", err);
                            }
                        }
                        Err(err) => {
                            eprint_colored!(Color::Red, "{}", err);
                        }
                    }
                }
            }
            // Evaluates the expression
            _ => match evaluator.eval(&input) {
                Ok(result) => {
                    print_colored!(Color::Green, "{}", result);
                    if let Err(err) = evaluator.mut_context().set_variable(RESULT, result) {
                        eprint_colored!(Color::Red, "{}", err);
                    }
                }
                Err(err) => {
                    eprint_colored!(Color::Red, "{}", err);
                }
            },
        }

        println!();
    }
}

fn repl_tokenizer<'a, N>() -> Tokenizer<'a, N>
where
    N: FromStr + Clone + Display + Debug + 'a,
    <N as FromStr>::Err: Display,
{
    let splitter = DefaultSplitter::with_identifier_rule(|c, rest| {
        #[inline]
        fn is_valid_char(c: &char) -> bool {
            matches!(c, 'a'..='z' | 'A'..='Z' | '0'..='9' | '_')
        }

        if c == '$' {
            let mut buf = String::new();
            buf.push(c);

            while let Some(c) = rest.next_if(is_valid_char) {
                buf.push(c);
            }

            Outcome::Data(buf)
        } else {
            Outcome::Continue
        }
    });

    Tokenizer::with_splitter(splitter)
}

fn print_prompt() {
    print_colored!(Color::Cyan, ">>> ");
}

/*

*/
