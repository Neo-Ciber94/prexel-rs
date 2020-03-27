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
    ArgCount(usize),
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
    /// Checks if the token is a number.
    #[inline]
    pub fn is_number(&self) -> bool {
        match self{
            Token::Number(_) => true,
            _ => false
        }
    }

    /// Checks if the token is a variable.
    #[inline]
    pub fn is_variable(&self) -> bool {
        match self{
            Token::Variable(_) => true,
            _ => false
        }
    }

    /// Checks if the token is a constant.
    #[inline]
    pub fn is_constant(&self) -> bool {
        match self{
            Token::Constant(_) => true,
            _ => false
        }
    }

    /// Checks if the token is a function.
    #[inline]
    pub fn is_function(&self) -> bool {
        match self{
            Token::Function(_) => true,
            _ => false
        }
    }

    /// Checks if the token is an infix operator.
    #[inline]
    pub fn is_infix_function(&self) -> bool {
        match self{
            Token::InfixFunction(_) => true,
            _ => false
        }
    }

    /// Checks if the token is an unary operator.
    #[inline]
    pub fn is_unary_operator(&self) -> bool {
        match self{
            Token::UnaryOperator(_) => true,
            _ => false
        }
    }

    /// Checks if the token is a binary operator.
    #[inline]
    pub fn is_binary_operator(&self) -> bool {
        match self{
            Token::BinaryOperator(_) => true,
            _ => false
        }
    }

    /// Checks if the token represents an argument count.
    ///
    /// # Remarks
    /// This is used internally to insert the argument count of a function.
    #[inline]
    pub fn is_arg_count(&self) -> bool {
        match self{
            Token::ArgCount(_) => true,
            _ => false
        }
    }

    /// Checks if the token is a grouping open.
    #[inline]
    pub fn is_grouping_open(&self) -> bool {
        match self{
            Token::GroupingOpen(_) => true,
            _ => false
        }
    }

    /// Checks if the token is a grouping close.
    #[inline]
    pub fn is_grouping_close(&self) -> bool {
        match self{
            Token::GroupingClose(_) => true,
            _ => false
        }
    }

    /// Checks if the token is a comma.
    #[inline]
    pub fn is_comma(&self) -> bool {
        match self{
            Token::Comma => true,
            _ => false
        }
    }

    /// Checks if the token is an unknown value.
    ///
    /// # Remarks
    /// A token is considered unknown if is not a number and its value
    /// cannot be found in the context used for tokenize an expression.
    #[inline]
    pub fn is_unknown(&self) -> bool {
        match self{
            Token::Unknown(_) => true,
            _ => false
        }
    }

    /// Checks if the token contains the specified number value.
    ///
    /// # Remarks
    /// This check the token value if is a `Token::Number` otherwise returns false.
    #[inline]
    pub fn contains_number<U: PartialEq<N>>(&self, value: &U) -> bool{
        match self{
            Token::Number(n) => value == n,
            _ => false
        }
    }

    /// Checks if the token contains a value with the specified name.
    ///
    /// # Remarks
    /// If the token is a named token eg: variable, constant, function, infix function or unknown token,
    /// will compare its `String` with the specified `str` otherwise returns `false`.
    #[inline]
    pub fn contains_name(&self, name: &str) -> bool{
        match self{
            Token::Variable(s) |
            Token::Constant(s) |
            Token::Function(s) |
            Token::InfixFunction(s) |
            Token::Unknown(s) => {
                s == name
            }
            _ => false
        }
    }

    /// Checks if the token contains a symbol with the specified value.
    ///
    /// # Remarks
    /// If the token is a named token with a `char` eg: grouping and operators will compare its values,
    /// otherwise returns `false`.
    #[inline]
    pub fn contains_symbol(&self, name: char) -> bool{
        match *self{
            Token::GroupingClose(c) |
            Token::GroupingOpen(c) |
            Token::UnaryOperator(c) |
            Token::BinaryOperator(c) => {
                c == name
            }
            _ => false
        }
    }
}