use std::{collections::HashMap, fmt::Display};
use serde::{Deserialize, Serialize};

/// Represents the result of an evaluation.
pub type EvalResult = Result<String, String>;

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(untagged)]
pub enum Variable {
    String(String),
    Number(f64),
}

impl Display for Variable {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Variable::String(s) => write!(f, "{}", s),
            Variable::Number(n) => write!(f, "{}", n),
        }
    }
}

/// Represents the type of the numbers of an expression.
#[derive(Debug, PartialEq, Clone, Deserialize, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum NumberType {
    /// Decimal numbers. (default)
    Decimal,

    /// Floating point numbers.
    Float,

    /// Complex numbers.
    Complex,

    /// Integer numbers
    Integer,

    /// Binary
    Binary,
}

/// Represents an expression to be evaluated.
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct EvalExpression {
    pub expression: String,
    pub r#type: Option<NumberType>,
    pub variables: Option<HashMap<String, Variable>>,
}

/// Represents the result to evaluate an expression.
#[derive(Debug, Serialize, Deserialize)]
pub struct EvaluatedExpression {
    result: Option<String>,
    error: Option<String>,
}

impl EvaluatedExpression {
    pub fn with_error(error: String) -> Self {
        EvaluatedExpression {
            result: None,
            error: Some(error),
        }
    }
}

impl From<EvalResult> for EvaluatedExpression {
    fn from(result: EvalResult) -> Self {
        match result {
            EvalResult::Ok(result) => EvaluatedExpression {
                result: Some(result),
                error: None,
            },
            EvalResult::Err(error) => EvaluatedExpression {
                result: None,
                error: Some(error),
            },
        }
    }
}