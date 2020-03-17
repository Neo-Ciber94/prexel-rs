use std::fmt::{Display, Formatter, Debug};
use Token::*;

/// Represents a token in an expression.
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Token<N>{
    /// A number
    Number(N),
    /// A variable
    Variable(String),
    /// A constant
    Constant(String),
    /// A function
    Function(String),
    /// An infix function
    InfixFunction(String),
    /// A binary operator
    BinaryOperator(char),
    /// An unary operator
    UnaryOperator(char),
    /// The argument count
    ArgCount(u32),
    /// An open grouping symbol
    GroupingOpen(char),
    /// A close grouping symbol
    GroupingClose(char),
    /// An unknown value
    Unknown(String),
    /// A comma
    Comma
}

impl <N : Display> Display for Token<N>{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self{
            Number(n) => write!(f, "Number({})", n),
            Variable(name) => write!(f, "Variable({})", name),
            Constant(name) => write!(f, "Constant({})", name),
            Function(name) => write!(f, "Function({})", name),
            InfixFunction(c) => write!(f, "InfixFunction('{}')", c),
            BinaryOperator(c) => write!(f, "BinaryOperator('{}')", c),
            UnaryOperator(c) => write!(f, "UnaryOperator('{}')", c),
            ArgCount(n) => write!(f, "ArgCount({})", n),
            GroupingOpen(c) => write!(f, "ParenthesisOpen('{}')", c),
            GroupingClose(c) => write!(f, "ParenthesisClose('{}')", c),
            Unknown(name) => write!(f, "Unknown({})", name),
            Comma => write!(f, "Comma"),
        }
    }
}

impl <N : Debug> Debug for Token<N>{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self{
            Number(n) => write!(f, "Number({:?})", n),
            Variable(name) => write!(f, "Variable({:?})", name),
            Constant(name) => write!(f, "Constant({:?})", name),
            Function(name) => write!(f, "Function({:?})", name),
            InfixFunction(c) => write!(f, "InfixFunction('{:?}')", c),
            BinaryOperator(c) => write!(f, "BinaryOperator('{:?}')", c),
            UnaryOperator(c) => write!(f, "UnaryOperator('{:?}')", c),
            ArgCount(n) => write!(f, "ArgCount({:?})", n),
            GroupingOpen(c) => write!(f, "ParenthesisOpen('{:?}')", c),
            GroupingClose(c) => write!(f, "ParenthesisClose('{:?}')", c),
            Unknown(name) => write!(f, "Unknown({:?})", name),
            Comma => write!(f, "Comma"),
        }
    }
}

impl <N> Token<N> {
    #[inline]
    pub fn is_number(&self) -> bool {
        match self{
            Token::Number(_) => true,
            _ => false
        }
    }

    #[inline]
    pub fn is_variable(&self) -> bool {
        match self{
            Token::Variable(_) => true,
            _ => false
        }
    }

    #[inline]
    pub fn is_constant(&self) -> bool {
        match self{
            Token::Constant(_) => true,
            _ => false
        }
    }

    #[inline]
    pub fn is_function(&self) -> bool {
        match self{
            Token::Function(_) => true,
            _ => false
        }
    }

    #[inline]
    pub fn is_unary_operator(&self) -> bool {
        match self{
            Token::UnaryOperator(_) => true,
            _ => false
        }
    }

    #[inline]
    pub fn is_binary_operator(&self) -> bool {
        match self{
            Token::BinaryOperator(_) => true,
            _ => false
        }
    }

    #[inline]
    pub fn is_infix_operator(&self) -> bool {
        match self{
            Token::InfixFunction(_) => true,
            _ => false
        }
    }

    #[inline]
    pub fn is_arg_count(&self) -> bool {
        match self{
            Token::ArgCount(_) => true,
            _ => false
        }
    }

    #[inline]
    pub fn is_grouping_open(&self) -> bool {
        match self{
            Token::GroupingOpen(_) => true,
            _ => false
        }
    }

    #[inline]
    pub fn is_grouping_close(&self) -> bool {
        match self{
            Token::GroupingClose(_) => true,
            _ => false
        }
    }

    #[inline]
    pub fn is_comma(&self) -> bool {
        match self{
            Token::Comma => true,
            _ => false
        }
    }

    #[inline]
    pub fn is_unknown(&self) -> bool {
        match self{
            Token::Unknown(_) => true,
            _ => false
        }
    }
}