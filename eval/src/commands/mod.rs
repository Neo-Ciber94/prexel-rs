mod eval;
mod run;
mod context;

pub use self::eval::EvalCommand;
pub use self::run::RunCommand;
pub use self::context::ContextCommand;

mod info {
    use std::convert::TryFrom;
    use std::result;

    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub enum CommandInfo {
        Eval,
        Run,
        Context,
    }

    impl CommandInfo {
        pub fn name(&self) -> &str{
            match self{
                CommandInfo::Eval => "",
                CommandInfo::Run => "--run",
                CommandInfo::Context => "--context",
            }
        }

        pub fn alias(&self) -> Option<&str>{
            match self{
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
           match value{
                "--decimal" | "--d" => Ok(NumberType::Decimal),
                "--bigdecimal" | "--b" => Ok(NumberType::BigDecimal),
                "--complex" | "--c" => Ok(NumberType::Complex),
                _ => Err(())
            }
        }
    }
}
