mod eval;
mod run;

pub use eval::EvalCommand;
pub use run::RunCommand;

mod eval_type{
    use std::convert::TryFrom;
    use std::result;

    pub enum EvalType {
        Decimal,
        BigDecimal,
        Complex,
    }

    impl Default for EvalType {
        fn default() -> Self {
            EvalType::Decimal
        }
    }

    impl TryFrom<&str> for EvalType {
        type Error = ();

        fn try_from(value: &str) -> result::Result<Self, Self::Error> {
            match value {
                "--decimal" | "--d" => Ok(EvalType::Decimal),
                "--bigdecimal" | "--b" => Ok(EvalType::BigDecimal),
                "--complex" | "--c" => Ok(EvalType::Complex),
                _ => Err(()),
            }
        }
    }
}
