use std::collections::HashMap;
use serde::{Deserialize, Serialize};

/// Represents an expression to be evaluated.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EvalExpression {
    pub expression: String,
    pub variables: Option<HashMap<String, String>>,
}
