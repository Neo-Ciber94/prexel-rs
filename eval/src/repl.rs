use crate::eval_expr::CONFIG;
use crate::{readln, ColorWriter, EvalType, Intense};
use prexel::complex::Complex;
use prexel::context::{Context, DefaultContext};
use prexel::evaluator::Evaluator;
use prexel::num_traits::Zero;
use prexel::tokenizer::Tokenizer;
use prexel::utils::splitter::{DefaultSplitter, Outcome};
use std::fmt::{Debug, Display};
use std::io::Write;
use std::ops::ControlFlow;
use std::str::FromStr;
use crossterm::event::{self, Event, KeyCode};
use crossterm::style::{Stylize, Color, StyledContent, Attribute};

#[derive(Debug, Clone, Default)]
struct TextStyling {
    fg: Option<Color>,
    bg: Option<Color>,
}

impl TextStyling {
    pub fn new() -> Self {
        Self {
            fg: None,
            bg: None,
        }
    }

    pub fn fg(mut self, fg: Color) -> Self {
        self.fg = Some(fg);
        self
    }

    pub fn bg(mut self, bg: Color) -> Self {
        self.bg = Some(bg);
        self
    }

    pub fn styled<D: Display>(&self, s: D) -> StyledContent<String> {
        let mut stylized = StyledContent::new(Default::default(), s.to_string());

        if let Some(fg) = &self.fg {
            stylized = stylized.with(fg.clone());
        }

        if let Some(bg) = &self.bg {
            stylized = stylized.with(bg.clone());
        }

        stylized
    }
}

struct Repl<'a> {
    on_enter: Box<dyn Fn(String) -> Option<ControlFlow<()>> + 'a>,
    start_text: Option<String>,
    prompt: Option<String>,
    prompt_style: Option<TextStyling>,
    text_style: Option<TextStyling>
}

impl<'a> Repl<'a> {
    fn new(on_enter: Box<dyn Fn(String) -> Option<ControlFlow<()>> + 'a>) -> Self {
        Self {
            on_enter,
            prompt: None,
            start_text: None,
            prompt_style: None,
            text_style: None
        }
    }

    fn print_prompt(&self) {
        if let Some(prompt) = self.prompt.as_deref() {
            if let Some(prompt_style) = &self.prompt_style {
                print!("{}", prompt_style.styled(prompt));
            } else {
                print!("{}", prompt);
            }
            std::io::stdout().flush().unwrap();
        }
    }

    pub fn run(self)  {
        let mut buf = String::new();

        self.print_prompt();

        loop {
            // `read()` blocks until an `Event` is available
            match event::read().unwrap() {
                Event::Key(event) => {
                    match event.code {
                        KeyCode::Backspace => {
                            if buf.len() > 0 {
                                buf.pop();
                                print!("\x08 \x08");
                            }
                        }
                        KeyCode::Enter => {
                            let s = buf.drain(..).collect::<String>();

                            match (self.on_enter)(s) {
                                Some(ControlFlow::Break(_)) => break,
                                Some(ControlFlow::Continue(_)) => continue,
                                None => {
                                    println!();
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

                            if let Some(ref style) = self.text_style {
                                print!("{}", style.styled(c));
                            } else {
                                print!("{}", c);
                            }
                        }
                        KeyCode::Esc => {
                            break;
                        },
                        _ => {}
                    }
                },
                _ => {}
            }

            // Flush the output
            std::io::stdout().flush().unwrap();
        }
    }
}

struct ReplBuilder<'a> {
    prompt: Option<String>,
    on_enter: Box<dyn Fn(String) -> Option<ControlFlow<()>> + 'a>,
    start_text: Option<String>,
    prompt_style: Option<TextStyling>,
    text_style: Option<TextStyling>
}

impl<'a> ReplBuilder<'a> {
    pub fn new<F>(f: F) -> Self
    where
        F: Fn(String) -> Option<ControlFlow<()>> + 'a,
    {
        Self {
            prompt: None,
            on_enter: Box::new(f),
            start_text: None,
            prompt_style: None,
            text_style: None,
        }
    }

    pub fn prompt(mut self, prompt: &str) -> Self {
        self.prompt = Some(prompt.to_string());
        self
    }

    pub fn start_text(mut self, s: &str) -> Self {
        self.start_text = Some(s.to_string());
        self
    }

    pub fn text_style(mut self, style: TextStyling) -> Self {
        self.text_style = Some(style);
        self
    }

    pub fn text_fg(mut self, fg: Color) -> Self {
        self.text_style = Some(self.text_style.unwrap_or_default().fg(fg));
        self
    }

    pub fn text_bg(mut self, bg: Color) -> Self {
        self.text_style = Some(self.text_style.unwrap_or_default().bg(bg));
        self
    }

    pub fn prompt_style(mut self, style: TextStyling) -> Self {
        self.prompt_style = Some(style);
        self
    }

    pub fn prompt_fg(mut self, fg: Color) -> Self {
        self.prompt_style = Some(self.prompt_style.unwrap_or_default().fg(fg));
        self
    }

    pub fn prompt_bg(mut self, bg: Color) -> Self {
        self.prompt_style = Some(self.prompt_style.unwrap_or_default().bg(bg));
        self
    }

    pub fn build(self) -> Repl<'a> {
        Repl {
            prompt: self.prompt,
            on_enter: self.on_enter,
            start_text: self.start_text,
            prompt_style: self.prompt_style,
            text_style: self.text_style,
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

fn eval_loop<'a, N, F>(mut writer: ColorWriter, factory: F)
    where
        N: FromStr + Clone + Display + Debug + Zero + 'a,
        F: FnOnce() -> DefaultContext<'a, N>,
        <N as FromStr>::Err: Display,
{
    let repl = ReplBuilder::new(|s| {
        // print!("{}{}", ">>> ".blue(), s.reverse());
        None
    })
        .prompt(">>> ")
        .prompt_fg(Color::Cyan)
        .text_fg(Color::Yellow)
        .build();

    repl.run();
}

fn ___eval_loop<'a, N, F>(mut writer: ColorWriter, factory: F)
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

    print_prompt(&mut writer);

    let mut buf = String::new();

    loop {
        // `read()` blocks until an `Event` is available
        match event::read().unwrap() {
            Event::Key(event) => {
                match event.code {
                    KeyCode::Backspace => {
                        if buf.len() > 0 {
                            buf.pop();
                            print!("\x08 \x08");
                        }
                    }
                    KeyCode::Enter => {
                        let _ = buf.drain(..).collect::<String>();
                        println!();
                        print_prompt(&mut writer);
                    }
                    KeyCode::Up => {}
                    KeyCode::Down => {}
                    KeyCode::Tab => {}
                    KeyCode::BackTab => {}
                    KeyCode::Delete => {}
                    KeyCode::Char(c) => {
                        buf.push(c);
                        print!("{}", c);
                        //writer.write_white(Intense::No, c);
                    }
                    KeyCode::Esc => {
                        break;
                    },
                    _ => {}
                }
            },
            _ => {}
        }

        // Flush the output
        std::io::stdout().flush().unwrap();
    }
}

fn __eval_loop<'a, N, F>(mut writer: ColorWriter, factory: F)
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
        print_prompt(&mut writer);
        let input = readln!();
        let expression = input.trim();

        print_prompt(&mut writer);

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
                    writer.write_err_red(Intense::Yes, "Invalid assignment");
                } else {
                    // TODO: Check if the variable is a valid identifier
                    let variable = &parts[0];
                    match N::from_str(&parts[1]) {
                        Ok(value) => {
                            if let Err(err) = evaluator.mut_context().set_variable(variable, value)
                            {
                                writer.write_err_red(Intense::Yes, format!("{}", err));
                            }
                        }
                        Err(err) => {
                            writer.write_err_red(Intense::Yes, format!("{}", err));
                        }
                    }
                }
            }
            // Evaluates the expression
            _ => match evaluator.eval(&input) {
                Ok(result) => {
                    writer.write_green(Intense::Yes, format!("{}", result));
                    if let Err(err) = evaluator.mut_context().set_variable(RESULT, result) {
                        writer.write_err_red(Intense::Yes, format!("{}", err));
                    }
                }
                Err(err) => {
                    writer.write_err_red(Intense::Yes, format!("{}", err));
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

fn print_prompt(writer: &mut ColorWriter) {
    //writer.write_cyan(Intense::Yes, ">>> ");
    print!(">>> ");
    std::io::stdout().flush().unwrap();
}
