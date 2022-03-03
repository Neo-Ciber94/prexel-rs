use std::fmt::{Debug, Display};
use std::io::Write;
use std::ops::ControlFlow;
use std::str::FromStr;
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use std::time::Duration;
use crossterm::event::{self, Event, KeyCode};
use crossterm::style::Color;
use prexel::complex::Complex;
use prexel::context::{Context, DefaultContext};
use prexel::evaluator::Evaluator;
use prexel::num_traits::Zero;
use prexel::tokenizer::Tokenizer;
use prexel::utils::splitter::{DefaultSplitter, Outcome};
use crate::{ColorWriter, EvalType};
use crate::eval_expr::CONFIG;
use crate::writer::TextStyling;

struct Repl<'a> {
    writer: ColorWriter<'a>,
    prompt: Option<String>,
    prompt_style: Option<TextStyling>,
    pre_text: Option<String>,
    exit_text: Option<String>,
    text_style: Option<TextStyling>,
    pre_text_style: Option<TextStyling>,
    exit_text_style: Option<TextStyling>,
}

impl<'a> Repl<'a> {
    fn print_pre_text(&mut self) {
        if let Some(pre_text) = &self.pre_text {
            if let Some(style) = self.pre_text_style {
                self.writer.styled(style).write(pre_text);
            } else {
                self.writer.write(pre_text, None);
            }

            std::io::stdout().flush().unwrap();
        }
    }

    fn print_exit_text(&mut self) {
        if let Some(exit_text) = &self.exit_text {
            if let Some(style) = self.exit_text_style {
                self.writer.styled(style).write(exit_text);
            } else {
                self.writer.write(exit_text, None);
            }

            std::io::stdout().flush().unwrap();
        }
    }

    fn print_prompt(&mut self) {
        if let Some(prompt) = &self.prompt {
            if let Some(style) = self.prompt_style {
                self.writer.styled(style).write(prompt);
            } else {
                self.writer.write(prompt, None);
            }

            std::io::stdout().flush().unwrap();
        }
    }

    pub fn run<F>(mut self, mut f: F)
    where
        F: FnMut(String, &mut ColorWriter) -> Option<ControlFlow<()>>,
    {
        let running = Arc::new(AtomicBool::new(true));
        let notifier = running.clone();

        ctrlc::set_handler(move || {
            notifier.store(false, Ordering::SeqCst);
        })
        .expect("Error setting Ctrl-C handler");

        let mut buf = String::new();
        self.print_pre_text();
        self.print_prompt();

        while running.load(Ordering::SeqCst) {
            // Checks for an event each 100ms
            if !event::poll(Duration::from_millis(100)).unwrap() {
                continue;
            }

            match event::read().unwrap() {
                Event::Key(event) => match event.code {
                    KeyCode::Backspace => {
                        if buf.len() > 0 {
                            buf.pop();
                            print!("\x08 \x08");
                        }
                    }
                    KeyCode::Enter => {
                        let s = buf.drain(..).collect::<String>();
                        println!();

                        match f(s, &mut self.writer) {
                            Some(ControlFlow::Break(_)) => {
                                println!();
                                break;
                            }
                            Some(ControlFlow::Continue(_)) => {
                                self.print_prompt();
                                continue;
                            }
                            None => {
                                self.print_prompt();
                            }
                        }
                    }
                    KeyCode::Up => {}
                    KeyCode::Down => {}
                    KeyCode::Tab => {}
                    KeyCode::BackTab => {}
                    KeyCode::Delete => {}
                    KeyCode::Char(c) => {
                        buf.push(c);

                        if let Some(style) = self.text_style {
                            self.writer.styled(style).write(c);
                        } else {
                            self.writer.write(c, None);
                        }
                    }
                    KeyCode::Esc => {
                        break;
                    }
                    _ => {}
                },
                _ => {}
            }

            // Flush the output
            std::io::stdout().flush().unwrap();
        }

        self.print_exit_text();
    }
}

struct ReplBuilder<'a> {
    writer: Option<ColorWriter<'a>>,
    prompt: Option<String>,
    pre_text: Option<String>,
    exit_text: Option<String>,
    prompt_style: Option<TextStyling>,
    text_style: Option<TextStyling>,
    pre_text_style: Option<TextStyling>,
    exit_text_style: Option<TextStyling>,
}

impl<'a> ReplBuilder<'a> {
    pub fn new() -> Self {
        ReplBuilder {
            writer: None,
            prompt: None,
            pre_text: None,
            exit_text: None,
            prompt_style: None,
            text_style: None,
            pre_text_style: None,
            exit_text_style: None,
        }
    }

    pub fn writer(mut self, writer: ColorWriter<'a>) -> Self {
        self.writer = Some(writer);
        self
    }

    pub fn prompt(mut self, prompt: &str) -> Self {
        self.prompt = Some(prompt.into());
        self
    }

    pub fn prompt_style(mut self, style: TextStyling) -> Self {
        self.prompt_style = Some(style);
        self
    }

    pub fn text_style(mut self, style: TextStyling) -> Self {
        self.text_style = Some(style);
        self
    }

    pub fn pre_text(mut self, pre_text: &str) -> Self {
        self.pre_text = Some(pre_text.into());
        self
    }

    pub fn pre_text_style(mut self, style: TextStyling) -> Self {
        self.pre_text_style = Some(style);
        self
    }

    pub fn exit_text(mut self, exit_text: &str) -> Self {
        self.prompt = Some(exit_text.into());
        self
    }

    pub fn exit_text_style(mut self, style: TextStyling) -> Self {
        self.exit_text_style = Some(style);
        self
    }

    pub fn build(self) -> Repl<'a> {
        let writer = self.writer.unwrap_or_else(|| ColorWriter::colorized());

        Repl {
            writer,
            prompt: self.prompt,
            prompt_style: self.prompt_style,
            text_style: self.text_style,
            pre_text: self.pre_text,
            pre_text_style: self.pre_text_style,
            exit_text: self.exit_text,
            exit_text_style: self.exit_text_style,
        }
    }
}

pub fn run_repl(writer: ColorWriter, eval_type: EvalType) {
    match eval_type {
        EvalType::Decimal => {
            let context = DefaultContext::with_config_decimal(CONFIG.lock().unwrap().clone());
            eval_loop(writer, move || context)
        }
        EvalType::Float => {
            let context =
                DefaultContext::<f64>::with_config_unchecked(CONFIG.lock().unwrap().clone());
            eval_loop(writer, move || context)
        }
        EvalType::Integer => {
            let context =
                DefaultContext::<i128>::with_config_checked(CONFIG.lock().unwrap().clone());
            eval_loop(writer, move || context)
        }
        EvalType::Complex => {
            let context = DefaultContext::<Complex<f64>>::with_config_complex(
                CONFIG.lock().unwrap().clone().with_complex_number(true),
            );
            eval_loop(writer, move || context)
        }
    }
}

fn eval_loop<'a, N, F>(writer: ColorWriter, factory: F)
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

    let repl = ReplBuilder::new()
        .prompt(">>> ")
        .prompt_style(TextStyling::new().fg(Color::Cyan))
        .text_style(TextStyling::new().fg(Color::Yellow))
        .pre_text("Press CTRL+C or type 'exit' to Exit\n")
        .pre_text_style(TextStyling::new().fg(Color::Blue))
        .exit_text("Bye bye!")
        .exit_text_style(TextStyling::new().fg(Color::Yellow))
        .writer(writer)
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
