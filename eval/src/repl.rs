use crate::collections::carray::CircularArray;
use crate::eval_expr::CONFIG;
use crate::repl_writer::ReplWriter;
use crate::style::{ColoredText, TextStyling};
use crate::{ColorWriter, EvalType};
use crossterm::event::{self, Event, KeyCode};
use crossterm::style::Color;
use prexel::complex::Complex;
use prexel::context::{Context, DefaultContext};
use prexel::evaluator::Evaluator;
use prexel::num_traits::Zero;
use prexel::tokenizer::Tokenizer;
use prexel::utils::splitter::{DefaultSplitter, Outcome};
use std::fmt::{Debug, Display};
use std::ops::ControlFlow;
use std::str::FromStr;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::time::Duration;

pub struct ReplConfig {
    pub history_size: Option<usize>,
    pub writer: ColorWriter,
    pub eval_type: EvalType,
}

struct Repl {
    writer: ReplWriter,
    history_size: usize,
    text: Option<ColoredText<String>>,
    pre_text: Option<ColoredText<String>>,
    exit_text: Option<ColoredText<String>>,
}

#[allow(unused)]
impl Repl {
    fn print_prompt(&mut self) {
        self.writer.write_prompt();
    }

    fn print_pre_text(&mut self) {
        if let Some(pre_text) = &self.pre_text {
            self.writer
                .fg(pre_text.fg)
                .bg(pre_text.bg)
                .writeln(&pre_text.content);
        }
    }

    fn print_exit_text(&mut self) {
        if let Some(exit_text) = &self.exit_text {
            self.writer
                .fg(exit_text.fg)
                .bg(exit_text.bg)
                .rewrite(&exit_text.content);

            self.writer.write("\n");
        }
    }

    pub fn run<F>(mut self, mut f: F)
    where
        F: FnMut(String, &mut ReplWriter) -> Option<ControlFlow<()>>,
    {
        let running = Arc::new(AtomicBool::new(true));
        let notifier = running.clone();
        let mut history = CircularArray::<String>::new(self.history_size);
        let mut history_cursor = 0_usize;

        ctrlc::set_handler(move || {
            notifier.store(false, Ordering::SeqCst);
        })
        .expect("Error setting Ctrl-C handler");

        let mut buf = String::new();
        self.print_pre_text();

        if self.pre_text.is_none() {
            self.print_prompt();
        }

        self.writer.flush();

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
                            self.writer.write("\x08 \x08");
                        }
                    }
                    KeyCode::Delete => {}
                    KeyCode::Enter => {
                        let s = buf.drain(..).collect::<String>();
                        self.writer.writeln("");

                        match f(s.clone(), &mut self.writer) {
                            Some(ControlFlow::Break(_)) => {
                                break;
                            }
                            Some(ControlFlow::Continue(_)) => {
                                continue;
                            }
                            None => {}
                        }

                        if s.trim().len() > 0 {
                            if history.last() != Some(&s) {
                                history.push(s);
                            }
                            history_cursor = history.len();
                        }
                    }
                    KeyCode::Up => {
                        if history_cursor > 0 {
                            history_cursor -= 1;
                            buf.clear();
                            buf.push_str(&history[history_cursor]);
                            self.writer.rewrite(&buf);
                        }
                    }
                    KeyCode::Down => {
                        if history_cursor < history.len() {
                            history_cursor += 1;
                            if history_cursor == history.len() {
                                buf.clear();
                            } else {
                                buf.clear();
                                buf.push_str(&history[history_cursor]);
                            }
                            self.writer.rewrite(&buf);
                        }
                    }
                    KeyCode::Left => {}
                    KeyCode::Right => {}
                    KeyCode::Char(c) => {
                        buf.push(c);

                        if let Some(style) = &self.text {
                            self.writer.fg(style.fg).bg(style.bg).write(c);
                        } else {
                            self.writer.write(c);
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
            self.writer.flush();
        }

        self.print_exit_text();
    }
}

struct ReplBuilder {
    writer: Option<ReplWriter>,
    history_size: Option<usize>,
    prompt: Option<ColoredText<String>>,
    text: Option<ColoredText<String>>,
    pre_text: Option<ColoredText<String>>,
    exit_text: Option<ColoredText<String>>,
}

#[allow(unused)]
impl ReplBuilder {
    pub fn new() -> Self {
        ReplBuilder {
            writer: None,
            history_size: None,
            text: None,
            prompt: None,
            pre_text: None,
            exit_text: None,
        }
    }

    pub fn writer(mut self, writer: ReplWriter) -> Self {
        self.writer = Some(writer);
        self
    }

    pub fn history_size(mut self, size: Option<usize>) -> Self {
        self.history_size = size;
        self
    }

    pub fn prompt(mut self, prompt: &str) -> Self {
        if let Some(colored) = &mut self.prompt {
            colored.content = prompt.into();
        } else {
            self.prompt = Some(ColoredText::new(prompt.into()));
        }

        self
    }

    pub fn prompt_style(mut self, style: TextStyling) -> Self {
        let mut colored = self
            .prompt
            .get_or_insert_with(|| ColoredText::new("".into()));
        colored.fg = style.fg;
        colored.bg = style.bg;
        self
    }

    pub fn text_style(mut self, style: TextStyling) -> Self {
        let mut colored = self.text.get_or_insert_with(|| ColoredText::new("".into()));
        colored.fg = style.fg;
        colored.bg = style.bg;
        self
    }

    pub fn pre_text(mut self, pre_text: &str) -> Self {
        if let Some(colored) = &mut self.pre_text {
            colored.content = pre_text.into();
        } else {
            self.pre_text = Some(ColoredText::new(pre_text.into()));
        }

        self
    }

    pub fn pre_text_style(mut self, style: TextStyling) -> Self {
        let mut colored = self
            .pre_text
            .get_or_insert_with(|| ColoredText::new("".into()));
        colored.fg = style.fg;
        colored.bg = style.bg;
        self
    }

    pub fn exit_text(mut self, exit_text: &str) -> Self {
        if let Some(colored) = &mut self.exit_text {
            colored.content = exit_text.into();
        } else {
            self.exit_text = Some(ColoredText::new(exit_text.into()));
        }

        self
    }

    pub fn exit_text_style(mut self, style: TextStyling) -> Self {
        let mut colored = self
            .exit_text
            .get_or_insert_with(|| ColoredText::new("".into()));
        colored.fg = style.fg;
        colored.bg = style.bg;
        self
    }

    pub fn build(self) -> Repl {
        let mut writer = self.writer.unwrap_or_else(|| ReplWriter::colored());
        let history_size = self.history_size.unwrap_or(100);

        if let Some(prompt) = self.prompt {
            writer.set_prompt(Some(prompt));
        }

        Repl {
            writer,
            history_size,
            text: self.text,
            pre_text: self.pre_text,
            exit_text: self.exit_text,
        }
    }
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
