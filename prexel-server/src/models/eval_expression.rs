use std::{collections::HashMap, fmt::Display};
use serde::{Deserialize, Serialize};

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

/// Represents an expression to be evaluated.
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct EvalExpression {
    pub expression: String,
    pub variables: Option<HashMap<String, Variable>>,
}