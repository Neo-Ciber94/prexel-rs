use crate::collections::carray::CircularArray;
use crate::repl::repl_writer::ReplWriter;
use crossterm::event;
use crossterm::event::{Event, KeyCode};
use crossterm::style::Color;
use std::ops::ControlFlow;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::time::Duration;

pub struct Repl {
    writer: ReplWriter,
    history_size: usize,
    pre_text: Option<String>,
    exit_text: Option<String>,
}

#[allow(unused)]
impl Repl {
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

        if let Some(pre_text) = self.pre_text {
            self.writer.green().writeln(&pre_text);
        } else {
            self.writer.write_prompt_prefix();
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
                        self.writer.flush();

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
                        self.writer.write(c);
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

        if let Some(exit_text) = self.exit_text {
            self.writer.fg(Some(Color::Yellow)).rewrite(&exit_text);
        }
    }
}

pub struct ReplBuilder {
    prompt_prefix: Option<String>,
    history_size: Option<usize>,
    pre_text: Option<String>,
    exit_text: Option<String>,
}

impl ReplBuilder {
    pub fn new() -> Self {
        ReplBuilder {
            prompt_prefix: None,
            history_size: None,
            pre_text: None,
            exit_text: None,
        }
    }

    pub fn prompt_prefix(mut self, prefix: &str) -> Self {
        self.prompt_prefix = Some(prefix.to_string());
        self
    }

    pub fn history_size(mut self, size: Option<usize>) -> Self {
        self.history_size = size;
        self
    }

    pub fn pre_text(mut self, pre_text: &str) -> Self {
        self.pre_text = Some(pre_text.to_owned());
        self
    }

    pub fn exit_text(mut self, exit_text: &str) -> Self {
        self.exit_text = Some(exit_text.to_owned());
        self
    }

    pub fn build(self) -> Repl {
        let ReplBuilder {
            history_size,
            prompt_prefix,
            pre_text,
            exit_text,
        } = self;

        let history_size = history_size.unwrap_or(100);

        let writer = if let Some(prefix) = prompt_prefix {
            ReplWriter::with_prompt_prefix(prefix)
        } else {
            ReplWriter::new()
        };

        Repl {
            writer,
            history_size,
            pre_text,
            exit_text,
        }
    }
}
