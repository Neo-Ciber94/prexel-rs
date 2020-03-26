use crate::error::{Result};

/// Represents the precedence of an operator.
#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Precedence(pub u32);

/// Represents the associativity of an operator.
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum Associativity { Left, Right }

/// Represents the notation of an unary operator.
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum Notation { Prefix, Postfix }

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

/// A trait for a function that takes 2 arguments.
pub trait InfixFunction<N> : BinaryFunction<N>{
}

impl Precedence{
    pub const VERY_LOW : Precedence = Precedence::from(0);
    pub const LOW : Precedence = Precedence::from(1);
    pub const MEDIUM : Precedence = Precedence::from(2);
    pub const HIGH : Precedence = Precedence::from(3);
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