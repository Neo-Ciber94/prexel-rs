use std::result;
use std::fmt::{Display, Formatter, Debug};

/// A convenient error type.
pub type Result<T> = result::Result<T, Error>;

/// Represents an error.
#[derive(Debug)]
pub struct Error{
    /// Detail information of the error.
    detail: Detail
}

/// The detail information of an error.
enum Detail{
    Simple(ErrorKind),
    Custom(Box<Custom>)
}

/// Represents a custom error.
#[derive(Debug)]
pub struct Custom{
    kind: ErrorKind,
    error: Box<dyn std::error::Error + Send + Sync>
}

/// A list of general errors.
#[derive(Clone, Copy, Debug, Hash, Eq, PartialEq)]
pub enum ErrorKind{
    /// The value overflow.
    Overflow,
    /// The value is zero.
    Zero,
    /// The value is `Not a Number`.
    NAN,
    /// The provided input is invalid.
    InvalidInput,
    /// The provided number of arguments is invalid.
    InvalidArgumentCount,
    /// Performed a division by zero.
    DivisionByZero,
    /// The value is negative.
    NegativeValue,
    /// The value is positive.
    PositiveValue,
    /// The expression is invalid.
    InvalidExpression,
    /// Other type of error.
    Other
}

impl ErrorKind{
    pub fn as_str(&self) -> &'static str{
        match *self{
            ErrorKind::Overflow => "Value has overflow",
            ErrorKind::Zero => "Value is zero",
            ErrorKind::NAN => "Value is 'Not a Number'",
            ErrorKind::InvalidInput => "Invalid input",
            ErrorKind::InvalidArgumentCount => "Invalid number of arguments",
            ErrorKind::DivisionByZero => "Cannot divide by zero",
            ErrorKind::NegativeValue => "Value is negative",
            ErrorKind::PositiveValue => "Value is positive",
            ErrorKind::InvalidExpression => "Invalid expression",
            ErrorKind::Other => "Other error",
        }
    }
}

impl Eq for Error{}

impl PartialEq for Error{
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        self.kind() == other.kind()
    }
}

impl Debug for Detail{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match *self{
            Detail::Simple(ref kind) => f.write_str(kind.as_str()),
            Detail::Custom(ref custom) => Debug::fmt(custom.error.as_ref(), f)
        }
    }
}

impl Error{
    /// Creates a new `Error` an `ErrorKind` and inner error.
    ///
    /// # Example
    /// ```
    /// use math_engine::error::{Error, ErrorKind};
    ///
    /// let custom_error = Error::new(ErrorKind::Other, "My error");
    /// assert_eq!(ErrorKind::Other, custom_error.kind());
    /// assert_eq!("My error", custom_error.to_string());
    /// ```
    #[inline]
    pub fn new<E>(kind: ErrorKind, error: E) -> Error
        where E : Into<Box<dyn std::error::Error + Send + Sync>> {
        Error{ detail: Detail::Custom(Box::from(Custom { kind, error: error.into() })) }
    }

    /// Creates an error with the specified message and `ErrorKind::Other`.
    ///
    /// # Example
    /// ```
    /// use math_engine::error::Error;
    /// let error = Error::other("Custom error");
    /// assert_eq!("Custom error", error.to_string());
    /// ```
    #[inline]
    pub fn other(msg: &str) -> Error{
        Self::new(ErrorKind::Other, msg)
    }

    /// Gets the `ErrorKind` of this error.
    ///
    /// # Example
    /// ```
    /// use math_engine::error::{Error, ErrorKind};
    /// let error = Error::from(ErrorKind::InvalidInput);
    /// assert_eq!(ErrorKind::InvalidInput, error.kind());
    /// ```
    pub fn kind(&self) -> ErrorKind{
        match self.detail{
            Detail::Simple(ref kind) => *kind,
            Detail::Custom(ref custom) => custom.kind,
        }
    }

    ///Consumes the `Error`, returning its inner error (if any).
    ///
    /// # Example
    /// ```
    /// use math_engine::error::Error;
    /// use math_engine::error::ErrorKind;
    ///
    /// fn print_error(error: Error){
    ///     if let Some(inner_error) = error.into_inner(){
    ///         println!("Inner error: {}", inner_error)
    ///     }
    ///     else{
    ///         println!("No inner error");
    ///     }
    /// }
    ///
    /// fn main(){
    ///     // No inner error
    ///     print_error(Error::from(ErrorKind::InvalidInput));
    ///     // With inner error
    ///     print_error(Error::new(ErrorKind::Other, "Custom error"))
    /// }
    /// ```
    pub fn into_inner(self) -> Option<Box<dyn std::error::Error + Send + Sync>>{
        match self.detail{
            Detail::Simple(_) => None,
            Detail::Custom(custom) => Some(custom.error),
        }
    }

    /// Gets a reference to the inner error (if any).
    ///
    /// # Example
    /// ```
    /// use math_engine::error::{Error, ErrorKind};
    /// let error = Error::new(ErrorKind::Overflow, "Value has overflow");
    /// let inner_error = error.get_ref().unwrap();
    /// ```
    pub fn get_ref(&self) -> Option<&Box<dyn std::error::Error + Send + Sync>>{
        match self.detail{
            Detail::Simple(_) => None,
            Detail::Custom(ref custom) => Some(&custom.error),
        }
    }

    /// Gets a mutable reference to the inner error (if any).
    ///
    /// # Example
    /// ```
    /// use math_engine::error::{Error, ErrorKind};
    /// let mut error = Error::new(ErrorKind::Overflow, "Value has overflow");
    /// let inner_error = error.get_mut().unwrap();
    /// ```
    pub fn get_mut(&mut self) -> Option<&mut Box<dyn std::error::Error + Send + Sync + 'static>>{
        match self.detail{
            Detail::Simple(_) => None,
            Detail::Custom(ref mut custom) => Some(&mut custom.error),
        }
    }
}

impl From<ErrorKind> for Error{
    #[inline]
    fn from(kind: ErrorKind) -> Self {
        Error{ detail: Detail::Simple(kind) }
    }
}

impl Display for Error{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self.detail{
            Detail::Simple(ref kind) => f.write_str(kind.as_str()),
            Detail::Custom(ref custom) => Display::fmt(custom.error.as_ref(), f)
        }
    }
}

impl std::error::Error for Error{
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self.detail{
            Detail::Simple(_) => None,
            Detail::Custom(ref custom) => custom.error.source(),
        }
    }

    #[allow(deprecated)]
    fn cause(&self) -> Option<&dyn std::error::Error> {
        match self.detail{
            Detail::Simple(_) => None,
            Detail::Custom(ref custom) => custom.error.cause(),
        }
    }
}

#[cfg(test)]
mod tests{
    use super::*;

    #[test]
    fn custom_error_test(){
        let error = Error::new(ErrorKind::Other,"Just a test");
        if let Detail::Custom(e) = error.detail{
            assert_eq!(ErrorKind::Other, e.kind);
            assert_eq!("Just a test", e.error.to_string())
        }
        else{
            unreachable!()
        }
    }
}