use std::fmt::{Debug, Display};
use std::ops::ControlFlow;
use std::str::FromStr;
use crossterm::style::Color;
use prexel::complex::Complex;
use prexel::context::{Context, DefaultContext};
use prexel::evaluator::Evaluator;
use prexel::num_traits::Zero;
use prexel::tokenizer::Tokenizer;
use prexel::utils::splitter::{DefaultSplitter, Outcome};
use crate::eval_expr::CONFIG;
use crate::{ColorWriter, EvalType};
use crate::repl::repl::ReplBuilder;
use crate::repl::repl_writer::ReplWriter;
use crate::style::TextStyling;

pub mod repl;
pub mod repl_writer;

pub struct ReplConfig {
    pub history_size: Option<usize>,
    pub writer: ColorWriter,
    pub eval_type: EvalType,
}

pub fn run_repl(config: ReplConfig) {
    match config.eval_type {
        EvalType::Decimal => {
            let context = DefaultContext::with_config_decimal(CONFIG.lock().unwrap().clone());
            eval_loop(config, move || context)
        }
        EvalType::Float => {
            let context =
                DefaultContext::<f64>::with_config_unchecked(CONFIG.lock().unwrap().clone());
            eval_loop(config, move || context)
        }
        EvalType::Integer => {
            let context =
                DefaultContext::<i128>::with_config_checked(CONFIG.lock().unwrap().clone());
            eval_loop(config, move || context)
        }
        EvalType::Complex => {
            let context = DefaultContext::<Complex<f64>>::with_config_complex(
                CONFIG.lock().unwrap().clone().with_complex_number(true),
            );
            eval_loop(config, move || context)
        }
    }
}

fn eval_loop<'a, N, F>(config: ReplConfig, factory: F)
    where
        N: FromStr + Clone + Display + Debug + Zero + 'a,
        F: FnOnce() -> DefaultContext<'a, N>,
        <N as FromStr>::Err: Display,
{
    const RESULT: &str = "$result";

    let ReplConfig {
        writer,
        history_size,
        ..
    } = config;

    let mut context = factory();
    context.set_variable(RESULT, N::zero()).unwrap();

    let tokenizer = repl_tokenizer();
    let mut evaluator = Evaluator::with_context_and_tokenizer(context, tokenizer);

    let repl = ReplBuilder::new()
        .prompt(">>> ")
        .prompt_style(TextStyling::new().fg(Some(Color::Cyan)))
        .pre_text("Press CTRL+C or type 'exit' to Exit")
        .pre_text_style(TextStyling::new().fg(Some(Color::Blue)))
        .exit_text("Bye bye!")
        .exit_text_style(TextStyling::new().fg(Some(Color::Yellow)))
        .history_size(history_size)
        .writer(ReplWriter::new(writer))
        .build();

    repl.run(|s, writer| {
        let expression = s.trim();

        match expression {
            // Exits the REPL
            "exit" => {
                return Some(ControlFlow::Break(()));
            }

            // Assign a variable
            _ if expression.contains('=') => {
                let parts = expression
                    .split('=')
                    .map(|s| s.trim().to_owned())
                    .collect::<Vec<_>>();

                if parts.len() != 2 {
                    writer.red().writeln("Invalid assignment");
                } else {
                    let variable = &parts[0];
                    match N::from_str(&parts[1]) {
                        Ok(value) => {
                            if let Err(err) = evaluator.mut_context().set_variable(variable, value)
                            {
                                writer.red().writeln(err);
                            }
                        }
                        Err(err) => {
                            writer.red().writeln(err);
                        }
                    }
                }
            }

            // Evaluates the expression
            _ => match evaluator.eval(expression) {
                Ok(result) => {
                    writer.green().writeln(&result);
                    if let Err(err) = evaluator.mut_context().set_variable(RESULT, result) {
                        writer.red().writeln(err);
                    }
                }
                Err(err) => {
                    writer.red().writeln(err);
                }
            },
        }

        None
    });
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
