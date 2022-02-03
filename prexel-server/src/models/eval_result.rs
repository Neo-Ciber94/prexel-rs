use serde::{Deserialize, Serialize};

/// Represents the result of an evaluation.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EvalResult {
    result: Option<String>,
    error: Option<String>,
}

impl EvalResult {
    /// Creates a new `EvalResult` with the given result.
    pub fn new(result: String) -> Self {
        EvalResult {
            result: Some(result),
            error: None,
        }
    }

    /// Creates a new `EvalResult` with the given error.
    pub fn with_error(error: String) -> Self {
        EvalResult {
            result: None,
            error: Some(error),
        }
    }
}
