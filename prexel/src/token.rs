use std::fmt::{Debug, Display, Formatter};
use Token::*;

/// Represents a token in an expression.
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Token<N> {
    /// A number
    Number(N),
    /// A variable
    Variable(String),
    /// A constant
    Constant(String),
    /// A function
    Function(String),
    /// A binary operator
    BinaryOperator(String),
    /// An unary operator
    UnaryOperator(String),
    /// The argument count
    ArgCount(usize),
    /// An open grouping symbol
    GroupingOpen(char),
    /// A close grouping symbol
    GroupingClose(char),
    /// An unknown value
    Unknown(String),
    /// A comma
    Comma,
}

impl<N: Display> Display for Token<N> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Number(n) => write!(f, "Number({})", n),
            Variable(name) => write!(f, "Variable({})", name),
            Constant(name) => write!(f, "Constant({})", name),
            Function(name) => write!(f, "Function({})", name),
            //InfixFunction(name) => write!(f, "InfixFunction('{}')", name),
            BinaryOperator(name) => write!(f, "BinaryOperator('{}')", name),
            UnaryOperator(name) => write!(f, "UnaryOperator('{}')", name),
            ArgCount(n) => write!(f, "ArgCount({})", n),
            GroupingOpen(c) => write!(f, "ParenthesisOpen('{}')", c),
            GroupingClose(c) => write!(f, "ParenthesisClose('{}')", c),
            Unknown(name) => write!(f, "Unknown({})", name),
            Comma => write!(f, "Comma"),
        }
    }
}

impl<N: Debug> Debug for Token<N> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Number(n) => write!(f, "Number({:?})", n),
            Variable(name) => write!(f, "Variable({:?})", name),
            Constant(name) => write!(f, "Constant({:?})", name),
            Function(name) => write!(f, "Function({:?})", name),
            //InfixFunction(name) => write!(f, "InfixFunction('{:?}')", name),
            BinaryOperator(name) => write!(f, "BinaryOperator('{:?}')", name),
            UnaryOperator(name) => write!(f, "UnaryOperator('{:?}')", name),
            ArgCount(n) => write!(f, "ArgCount({:?})", n),
            GroupingOpen(c) => write!(f, "ParenthesisOpen('{:?}')", c),
            GroupingClose(c) => write!(f, "ParenthesisClose('{:?}')", c),
            Unknown(name) => write!(f, "Unknown({:?})", name),
            Comma => write!(f, "Comma"),
        }
    }
}

impl<N> Token<N> {
    /// Checks if the token is a number.
    #[inline]
    pub fn is_number(&self) -> bool {
        matches!(self, Token::Number(_))
    }

    /// Checks if the token is a variable.
    #[inline]
    pub fn is_variable(&self) -> bool {
        matches!(self, Token::Variable(_))
    }

    /// Checks if the token is a constant.
    #[inline]
    pub fn is_constant(&self) -> bool {
        matches!(self, Token::Constant(_))
    }

    /// Checks if the token is a function.
    #[inline]
    pub fn is_function(&self) -> bool {
        matches!(self, Token::Function(_))
    }

    /// Checks if the token is an unary operator.
    #[inline]
    pub fn is_unary_operator(&self) -> bool {
        matches!(self, Token::UnaryOperator(_))
    }

    /// Checks if the token is a binary operator.
    #[inline]
    pub fn is_binary_operator(&self) -> bool {
        matches!(self, Token::BinaryOperator(_))
    }

    /// Checks if the token represents an argument count.
    ///
    /// # Remarks
    /// This is used internally to insert the argument count of a function.
    #[inline]
    pub fn is_arg_count(&self) -> bool {
        matches!(self, Token::ArgCount(_))
    }

    /// Checks if the token is a grouping open.
    #[inline]
    pub fn is_grouping_open(&self) -> bool {
        matches!(self, Token::GroupingOpen(_))
    }

    /// Checks if the token is a grouping close.
    #[inline]
    pub fn is_grouping_close(&self) -> bool {
        matches!(self, Token::GroupingClose(_))
    }

    /// Checks if the token is a comma.
    #[inline]
    pub fn is_comma(&self) -> bool {
        matches!(self, Token::Comma)
    }

    /// Checks if the token is an unknown value.
    ///
    /// # Remarks
    /// A token is considered unknown if is not a number and its value
    /// cannot be found in the context used for tokenize an expression.
    #[inline]
    pub fn is_unknown(&self) -> bool {
        matches!(self, Token::Unknown(_))
    }

    /// Checks if the token contains a symbol with the specified value.
    ///
    /// # Remarks
    /// If the token is a named token with a `char` eg: grouping and operators will compare its values,
    /// otherwise returns `false`.
    #[inline]
    pub fn contains_symbol(&self, name: char) -> bool {
        match self {
            Token::GroupingClose(c) | Token::GroupingOpen(c) => *c == name,
            _ => false,
        }
    }
}
