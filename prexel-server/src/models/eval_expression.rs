use std::collections::HashMap;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EvalExpression<N> {
    pub expression: String,
    pub variables: Option<HashMap<String, N>>
}