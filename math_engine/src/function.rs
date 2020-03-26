use crate::error::{Result};

/// A trait for a function that take a variable number of arguments.
pub trait Function<N>{
    /// Gets the name of the function.
    fn name(&self) -> &str;
    /// Calls this function with the given number of arguments.
    fn call(&self, args: &[N]) -> Result<N>;
}

/// A trait for a function that takes 2 arguments.
pub trait BinaryFunction<N>{
    /// Gets the name of the function.
    fn name(&self) -> &str;
    /// Gets the `Precedence` of the function.
    fn precedence(&self) -> Precedence;
    /// Gets the `Associativity` of the function.
    fn associativity(&self) -> Associativity;
    /// Calls this function with the given arguments.
    fn call(&self, left: N, right: N) -> Result<N>;
}

/// A trait a function that takes 1 argument.
pub trait UnaryFunction<N>{
    /// Gets the name of the function.
    fn name(&self) -> &str;
    /// Gets the `Notation` of this function.
    fn notation(&self) -> Notation;
    /// Calls this function with the given argument.
    fn call(&self, value: N) -> Result<N>;
}

/// Represents the associativity of an operator.
///
/// See: `https://en.wikipedia.org/wiki/Operator_associativity`
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum Associativity {
    /// Left to right.
    Left,
    /// Right to left.
    Right
}

/// Represents the notation of an unary operator.
///
/// See: `https://en.wikipedia.org/wiki/Unary_operation`
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum Notation {
    /// The operator is before the value, eg: -10.
    Prefix,
    /// The operator is after the value, eg: 25!.
    Postfix
}

/// Represents the precedence of an operator.
///
/// See: `https://en.wikipedia.org/wiki/Order_of_operations`
#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Precedence(pub u32);

impl Precedence{
    /// Very low precedence.
    pub const VERY_LOW : Precedence = Precedence::from(0);
    /// Low precedence, used for addition (+) and subtraction (-) operators.
    pub const LOW : Precedence = Precedence::from(1);
    /// Medium precedence, used for multiplication (*) and division (/) operators.
    pub const MEDIUM : Precedence = Precedence::from(2);
    /// High precedence, used for power (^) operator.
    pub const HIGH : Precedence = Precedence::from(3);
    /// Very high precedence.
    pub const VERY_HIGH : Precedence = Precedence::from(4);

    /// Constructs a `Precedence` from the given value.
    #[inline]
    pub const fn from(value: u32) -> Self {
        Precedence(value)
    }
}

impl Into<u32> for Precedence{
    #[inline]
    fn into(self) -> u32 {
        self.0
    }
}