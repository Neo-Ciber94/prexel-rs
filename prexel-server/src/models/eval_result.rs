use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EvalResult<N> {
    result: Option<N>,
    error: Option<String>,
}

impl<N> EvalResult<N> {
    pub fn new(result: N) -> Self {
        EvalResult {
            result: Some(result),
            error: None,
        }
    }

    pub fn with_error(error: String) -> Self {
        EvalResult {
            result: None,
            error: Some(error),
        }
    }
}
