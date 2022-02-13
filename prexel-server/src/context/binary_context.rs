use prexel::context::{Config, Context, DefaultContext};
use prexel::function::{Associativity, BinaryFunction, Notation, Precedence, UnaryFunction};
use std::fmt::Display;
use std::iter::Peekable;
use std::str::{Chars, FromStr};
use prexel::utils::splitter::SplitterWithInterceptor;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Ord, PartialOrd)]
pub struct Binary(pub i128);

impl FromStr for Binary {
    type Err = <i128 as FromStr>::Err;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Some(stripped) = s.strip_prefix('b') {
            return i128::from_str_radix(stripped, 2).map(Binary);
        }

        i128::from_str(s).map(Binary)
    }
}

impl Display for Binary {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{:b}", self.0)
    }
}

pub type BinaryNumberSplitter = SplitterWithInterceptor<fn(char, &mut Peekable<Chars>) -> Option<String>>;

pub fn binary_number_splitter() -> BinaryNumberSplitter {
    fn is_next_binary(chars: &mut Peekable<Chars>) -> bool {
        chars.peek() == Some(&'1') || chars.peek() == Some(&'0')
    }

    SplitterWithInterceptor::new(|c, rest| {
        if c == 'b' && is_next_binary(rest) {
            let mut temp = String::new();
            temp.push(c);
            while let Some(c) = rest.peek() {
                if c.is_ascii_digit() {
                    temp.push(*c);
                    rest.next();
                } else {
                    break;
                }
            }

            Some(temp)
        } else {
            None
        }
    })
}

pub trait BinaryContext {
    fn new_binary() -> Self;
    fn with_config_binary(config: Config) -> Self;
}

impl<'a> BinaryContext for DefaultContext<'a, Binary> {
    fn new_binary() -> Self {
        Self::with_config_binary(Config::new())
    }

    fn with_config_binary(config: Config) -> Self {
        let mut context = DefaultContext::<Binary>::with_config(config);
        context.add_unary_function(NotFunction);
        context.add_binary_function(AndFunction);
        context.add_binary_function(OrFunction);
        context.add_binary_function(XorFunction);
        context.add_binary_function(EqFunction);
        context.add_binary_function(NeFunction);
        context.add_binary_function(GtFunction);
        context.add_binary_function(LtFunction);
        context.add_binary_function(GteFunction);
        context.add_binary_function(LteFunction);
        context
    }
}

struct NotFunction;
impl UnaryFunction<Binary> for NotFunction {
    fn name(&self) -> &str {
        "~"
    }

    fn notation(&self) -> Notation {
        Notation::Prefix
    }

    fn call(&self, value: Binary) -> prexel::Result<Binary> {
        Ok(Binary(!value.0))
    }
}

struct AndFunction;
impl BinaryFunction<Binary> for AndFunction {
    fn name(&self) -> &str {
        "and"
    }

    fn precedence(&self) -> Precedence {
        Precedence(11)
    }

    fn associativity(&self) -> Associativity {
        Associativity::Left
    }

    fn call(&self, left: Binary, right: Binary) -> prexel::Result<Binary> {
        Ok(Binary(left.0 & right.0))
    }
}

struct OrFunction;
impl BinaryFunction<Binary> for OrFunction {
    fn name(&self) -> &str {
        "or"
    }

    fn precedence(&self) -> Precedence {
        Precedence(13)
    }

    fn associativity(&self) -> Associativity {
        Associativity::Left
    }

    fn call(&self, left: Binary, right: Binary) -> prexel::Result<Binary> {
        Ok(Binary(left.0 | right.0))
    }
}

struct XorFunction;
impl BinaryFunction<Binary> for XorFunction {
    fn name(&self) -> &str {
        "xor"
    }

    fn precedence(&self) -> Precedence {
        Precedence(12)
    }

    fn associativity(&self) -> Associativity {
        Associativity::Left
    }

    fn call(&self, left: Binary, right: Binary) -> prexel::Result<Binary> {
        Ok(Binary(left.0 ^ right.0))
    }
}

struct EqFunction;
impl BinaryFunction<Binary> for EqFunction {
    fn name(&self) -> &str {
        "=="
    }

    fn precedence(&self) -> Precedence {
        Precedence(10)
    }

    fn associativity(&self) -> Associativity {
        Associativity::Left
    }

    fn call(&self, left: Binary, right: Binary) -> prexel::Result<Binary> {
        let result = if left.0 == right.0 { 1 } else { 0 };
        Ok(Binary(result))
    }
}

struct NeFunction;
impl BinaryFunction<Binary> for NeFunction {
    fn name(&self) -> &str {
        "!="
    }

    fn precedence(&self) -> Precedence {
        Precedence(10)
    }

    fn associativity(&self) -> Associativity {
        Associativity::Left
    }

    fn call(&self, left: Binary, right: Binary) -> prexel::Result<Binary> {
        let result = if left.0 != right.0 { 1 } else { 0 };
        Ok(Binary(result))
    }
}

struct GtFunction;
impl BinaryFunction<Binary> for GtFunction {
    fn name(&self) -> &str {
        ">"
    }

    fn precedence(&self) -> Precedence {
        Precedence(9)
    }

    fn associativity(&self) -> Associativity {
        Associativity::Left
    }

    fn call(&self, left: Binary, right: Binary) -> prexel::Result<Binary> {
        let result = if left.0 > right.0 { 1 } else { 0 };
        Ok(Binary(result))
    }
}

struct LtFunction;
impl BinaryFunction<Binary> for LtFunction {
    fn name(&self) -> &str {
        "<"
    }

    fn precedence(&self) -> Precedence {
        Precedence(9)
    }

    fn associativity(&self) -> Associativity {
        Associativity::Left
    }

    fn call(&self, left: Binary, right: Binary) -> prexel::Result<Binary> {
        let result = if left.0 < right.0 { 1 } else { 0 };
        Ok(Binary(result))
    }
}

struct GteFunction;
impl BinaryFunction<Binary> for GteFunction {
    fn name(&self) -> &str {
        ">="
    }

    fn precedence(&self) -> Precedence {
        Precedence(9)
    }

    fn associativity(&self) -> Associativity {
        Associativity::Left
    }

    fn call(&self, left: Binary, right: Binary) -> prexel::Result<Binary> {
        let result = if left.0 >= right.0 { 1 } else { 0 };
        Ok(Binary(result))
    }
}

struct LteFunction;
impl BinaryFunction<Binary> for LteFunction {
    fn name(&self) -> &str {
        "<="
    }

    fn precedence(&self) -> Precedence {
        Precedence(9)
    }

    fn associativity(&self) -> Associativity {
        Associativity::Left
    }

    fn call(&self, left: Binary, right: Binary) -> prexel::Result<Binary> {
        let result = if left.0 <= right.0 { 1 } else { 0 };
        Ok(Binary(result))
    }
}