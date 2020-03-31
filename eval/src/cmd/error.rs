use std::error;
use std::fmt::{Display, Formatter};

pub type Result = std::result::Result<(), Error>;

#[derive(Debug)]
pub struct Error {
    detail: Box<dyn error::Error + Sync + Send>,
}

impl Error {
    #[inline]
    pub fn new<E>(error: E) -> Self
    where
        E: Into<Box<dyn error::Error + Sync + Send>>,
    {
        Error {
            detail: error.into(),
        }
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        Display::fmt(self.detail.as_ref(), f)
    }
}

impl error::Error for Error {}
