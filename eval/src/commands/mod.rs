mod context;
mod eval;
mod run;

pub use self::context::ContextCommand;
pub use self::eval::EvalCommand;
pub use self::run::RunCommand;

mod internal {
    use std::convert::TryFrom;
    use std::fmt::Display;
    use std::result;
    use crossterm::style::Color;

    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub enum CommandInfo {
        Eval,
        Run,
        Context,
    }

    impl CommandInfo {
        pub fn name(&self) -> &str {
            match self {
                CommandInfo::Eval => "",
                CommandInfo::Run => "--run",
                CommandInfo::Context => "--context",
            }
        }

        pub fn alias(&self) -> Option<&str> {
            match self {
                CommandInfo::Eval => None,
                CommandInfo::Run => Some("--r"),
                CommandInfo::Context => Some("--ctx"),
            }
        }
    }

    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub enum NumberType {
        Decimal,
        BigDecimal,
        Complex,
    }

    impl Default for NumberType {
        fn default() -> Self {
            NumberType::Decimal
        }
    }

    impl TryFrom<&str> for NumberType {
        type Error = ();

        fn try_from(value: &str) -> result::Result<Self, Self::Error> {
            match value {
                "--decimal" | "--d" => Ok(NumberType::Decimal),
                "--bigdecimal" | "--b" => Ok(NumberType::BigDecimal),
                "--complex" | "--c" => Ok(NumberType::Complex),
                _ => Err(()),
            }
        }
    }

    pub enum StdKind {
        Output,
        Error,
    }

    pub(crate) fn print_color<T: Display + Clone>(value: T, color: Color, std_kind: StdKind) {
        use crossterm::execute;
        use crossterm::style::{Print, ResetColor, SetForegroundColor};
        use std::io::{Write, stderr, stdout};

        match std_kind {
            StdKind::Output => {
                execute!(
                    stdout(),
                    SetForegroundColor(color),
                    Print(value),
                    ResetColor
                ).unwrap();
            }
            StdKind::Error => {
                execute!(
                    stderr(),
                    SetForegroundColor(color),
                    Print(value),
                    ResetColor
                ).unwrap();
            }
        }
    }
}
