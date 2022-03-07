use std::fmt::{Debug, Display};
use std::iter::Peekable;
use std::str::{Chars, FromStr};
use crate::context::{Config, Context, DefaultContext};
use crate::utils::splitter::{DefaultSplitter, DefaultSplitterBuilder, Outcome, rules, SplitWhitespaceOption};

/// Represents a binary number.
#[derive(Clone, Debug, Copy, PartialEq, Eq, Ord, PartialOrd)]
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

/// Returns a splitter for binary data.
///
/// # Example
/// ```
/// use prexel::binary::{Binary, binary_number_splitter};
/// use prexel::context::DefaultContext;
/// use prexel::evaluator::Evaluator;
/// use prexel::tokenizer::Tokenizer;
///
/// let tokenizer = Tokenizer::with_splitter(binary_number_splitter());
/// let context = DefaultContext::new_binary();
/// let evaluator = Evaluator::with_context_and_tokenizer(context, tokenizer);
///
/// let result = evaluator.eval("b1000 > b0111");
/// assert_eq!(result, Ok(Binary(1)));
/// ```
pub fn binary_number_splitter<'a>() -> DefaultSplitter<'a> {
    fn is_next_binary(chars: &mut Peekable<Chars>) -> bool {
        chars.peek() == Some(&'1') || chars.peek() == Some(&'0')
    }

    DefaultSplitterBuilder::new()
        .rule(|c, rest| {
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

                Outcome::Data(temp)
            } else {
                Outcome::Continue
            }
        })
        .rule(rules::next_numeric)
        .rule(rules::next_identifier)
        .rule(rules::next_operator)
        .whitespace(SplitWhitespaceOption::Remove)
        .build()
}

impl<'a> DefaultContext<'a, Binary> {
    /// Constructs a new binary context.
    pub fn new_binary() -> Self {
        Self::with_config_binary(Config::new())
    }

    /// Constructs a new binary context with the given configuration.
    pub fn with_config_binary(config: Config) -> Self {
        use crate::binary::math::*;

        let mut context = DefaultContext::<Binary>::with_config(config);
        context.add_unary_function(NotFunction).unwrap();
        context.add_binary_function(AndFunction).unwrap();
        context.add_binary_function(OrFunction).unwrap();
        context.add_binary_function(XorFunction).unwrap();
        context.add_binary_function(EqFunction).unwrap();
        context.add_binary_function(NeFunction).unwrap();
        context.add_binary_function(GtFunction).unwrap();
        context.add_binary_function(LtFunction).unwrap();
        context.add_binary_function(GteFunction).unwrap();
        context.add_binary_function(LteFunction).unwrap();
        context.add_binary_function(ShrFunction).unwrap();
        context.add_binary_function(ShlFunction).unwrap();
        context
    }
}

mod math {
    use crate::binary::Binary;
    use crate::function::{Associativity, BinaryFunction, Notation, Precedence, UnaryFunction};

    pub struct NotFunction;
    impl UnaryFunction<Binary> for NotFunction {
        fn name(&self) -> &str {
            "~"
        }

        fn aliases(&self) -> Option<&[&str]> {
            Some(&["not"])
        }

        fn notation(&self) -> Notation {
            Notation::Prefix
        }

        fn call(&self, value: Binary) -> crate::Result<Binary> {
            Ok(Binary(!value.0))
        }

        fn description(&self) -> Option<&str> {
            Some("Returns the bitwise NOT of the given value.")
        }
    }

    pub struct AndFunction;
    impl BinaryFunction<Binary> for AndFunction {
        fn name(&self) -> &str {
            "&"
        }

        fn aliases(&self) -> Option<&[&str]> {
            Some(&["and"])
        }

        fn precedence(&self) -> Precedence {
            Precedence(11)
        }

        fn associativity(&self) -> Associativity {
            Associativity::Left
        }

        fn call(&self, left: Binary, right: Binary) -> crate::Result<Binary> {
            Ok(Binary(left.0 & right.0))
        }

        fn description(&self) -> Option<&str> {
            Some("Returns the bitwise AND of the given values.")
        }
    }

    pub struct OrFunction;
    impl BinaryFunction<Binary> for OrFunction {
        fn name(&self) -> &str {
            "|"
        }

        fn aliases(&self) -> Option<&[&str]> {
            Some(&["or"])
        }

        fn precedence(&self) -> Precedence {
            Precedence(13)
        }

        fn associativity(&self) -> Associativity {
            Associativity::Left
        }

        fn call(&self, left: Binary, right: Binary) -> crate::Result<Binary> {
            Ok(Binary(left.0 | right.0))
        }

        fn description(&self) -> Option<&str> {
            Some("Returns the bitwise OR of the given values.")
        }
    }

    pub struct XorFunction;
    impl BinaryFunction<Binary> for XorFunction {
        fn name(&self) -> &str {
            "^"
        }

        fn aliases(&self) -> Option<&[&str]> {
            Some(&["xor"])
        }

        fn precedence(&self) -> Precedence {
            Precedence(12)
        }

        fn associativity(&self) -> Associativity {
            Associativity::Left
        }

        fn call(&self, left: Binary, right: Binary) -> crate::Result<Binary> {
            Ok(Binary(left.0 ^ right.0))
        }

        fn description(&self) -> Option<&str> {
            Some("Returns the bitwise XOR of the given values.")
        }
    }

    pub struct EqFunction;
    impl BinaryFunction<Binary> for EqFunction {
        fn name(&self) -> &str {
            "=="
        }

        fn aliases(&self) -> Option<&[&str]> {
            Some(&["eq", "equal"])
        }

        fn precedence(&self) -> Precedence {
            Precedence(10)
        }

        fn associativity(&self) -> Associativity {
            Associativity::Left
        }

        fn call(&self, left: Binary, right: Binary) -> crate::Result<Binary> {
            let result = if left.0 == right.0 { 1 } else { 0 };
            Ok(Binary(result))
        }

        fn description(&self) -> Option<&str> {
            Some("Returns 1 if the given values are equal, 0 otherwise.")
        }
    }

    pub struct NeFunction;
    impl BinaryFunction<Binary> for NeFunction {
        fn name(&self) -> &str {
            "!="
        }

        fn aliases(&self) -> Option<&[&str]> {
            Some(&["ne", "no_equal"])
        }

        fn precedence(&self) -> Precedence {
            Precedence(10)
        }

        fn associativity(&self) -> Associativity {
            Associativity::Left
        }

        fn call(&self, left: Binary, right: Binary) -> crate::Result<Binary> {
            let result = if left.0 != right.0 { 1 } else { 0 };
            Ok(Binary(result))
        }

        fn description(&self) -> Option<&str> {
            Some("Returns 1 if the given values are not equal, 0 otherwise.")
        }
    }

    pub struct GtFunction;
    impl BinaryFunction<Binary> for GtFunction {
        fn name(&self) -> &str {
            ">"
        }

        fn aliases(&self) -> Option<&[&str]> {
            Some(&["gt"])
        }

        fn precedence(&self) -> Precedence {
            Precedence(9)
        }

        fn associativity(&self) -> Associativity {
            Associativity::Left
        }

        fn call(&self, left: Binary, right: Binary) -> crate::Result<Binary> {
            let result = if left.0 > right.0 { 1 } else { 0 };
            Ok(Binary(result))
        }

        fn description(&self) -> Option<&str> {
            Some("Returns 1 if the left value is greater than the right value, 0 otherwise.")
        }
    }

    pub struct LtFunction;
    impl BinaryFunction<Binary> for LtFunction {
        fn name(&self) -> &str {
            "<"
        }

        fn aliases(&self) -> Option<&[&str]> {
            Some(&["lt"])
        }

        fn precedence(&self) -> Precedence {
            Precedence(9)
        }

        fn associativity(&self) -> Associativity {
            Associativity::Left
        }

        fn call(&self, left: Binary, right: Binary) -> crate::Result<Binary> {
            let result = if left.0 < right.0 { 1 } else { 0 };
            Ok(Binary(result))
        }

        fn description(&self) -> Option<&str> {
            Some("Returns 1 if the left value is less than the right value, 0 otherwise.")
        }
    }

    pub struct GteFunction;
    impl BinaryFunction<Binary> for GteFunction {
        fn name(&self) -> &str {
            ">="
        }

        fn aliases(&self) -> Option<&[&str]> {
            Some(&["gte"])
        }

        fn precedence(&self) -> Precedence {
            Precedence(9)
        }

        fn associativity(&self) -> Associativity {
            Associativity::Left
        }

        fn call(&self, left: Binary, right: Binary) -> crate::Result<Binary> {
            let result = if left.0 >= right.0 { 1 } else { 0 };
            Ok(Binary(result))
        }

        fn description(&self) -> Option<&str> {
            Some("Returns 1 if the left value is greater than or equal to the right value, 0 otherwise.")
        }
    }

    pub struct LteFunction;
    impl BinaryFunction<Binary> for LteFunction {
        fn name(&self) -> &str {
            "<="
        }

        fn aliases(&self) -> Option<&[&str]> {
            Some(&["lte"])
        }

        fn precedence(&self) -> Precedence {
            Precedence(9)
        }

        fn associativity(&self) -> Associativity {
            Associativity::Left
        }

        fn call(&self, left: Binary, right: Binary) -> crate::Result<Binary> {
            let result = if left.0 <= right.0 { 1 } else { 0 };
            Ok(Binary(result))
        }

        fn description(&self) -> Option<&str> {
            Some("Returns 1 if the left value is less than or equal to the right value, 0 otherwise.")
        }
    }

    pub struct ShrFunction;
    impl BinaryFunction<Binary> for ShrFunction {
        fn name(&self) -> &str {
            ">>"
        }

        fn aliases(&self) -> Option<&[&str]> {
            Some(&["shr"])
        }

        fn precedence(&self) -> Precedence {
            Precedence(7)
        }

        fn associativity(&self) -> Associativity {
            Associativity::Left
        }

        fn call(&self, left: Binary, right: Binary) -> crate::Result<Binary> {
            Ok(Binary(left.0 >> right.0))
        }

        fn description(&self) -> Option<&str> {
            Some("Returns the left value bits shifted right by the right value.")
        }
    }

    pub struct ShlFunction;
    impl BinaryFunction<Binary> for ShlFunction {
        fn name(&self) -> &str {
            "<<"
        }

        fn aliases(&self) -> Option<&[&str]> {
            Some(&["shl"])
        }

        fn precedence(&self) -> Precedence {
            Precedence(7)
        }

        fn associativity(&self) -> Associativity {
            Associativity::Left
        }

        fn call(&self, left: Binary, right: Binary) -> crate::Result<Binary> {
            Ok(Binary(left.0 << right.0))
        }

        fn description(&self) -> Option<&str> {
            Some("Returns the left value bits shifted left by the right value.")
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::evaluator::Evaluator;
    use crate::tokenizer::Tokenizer;
    use super::*;

    fn eval(expr: &str) -> crate::Result<Binary> {
        let tokenizer = Tokenizer::with_splitter(binary_number_splitter());
        let context = DefaultContext::new_binary();
        let evaluator = Evaluator::with_context_and_tokenizer(context, tokenizer);
        evaluator.eval(expr)
    }

    #[test]
    fn not_test() {
        assert_eq!(eval("~b1").unwrap(), Binary(-2));
        assert_eq!(eval("not b1").unwrap(), Binary(-2));
    }

    #[test]
    fn and_test() {
        assert_eq!(eval("b101 & b110").unwrap(), Binary(4));
        assert_eq!(eval("b101 and b110").unwrap(), Binary(4));
    }

    #[test]
    fn or_test() {
        assert_eq!( eval("b101 | b110").unwrap(), Binary(7));
        assert_eq!( eval("b101 or b110").unwrap(), Binary(7));
    }

    #[test]
    fn xor_test() {
        assert_eq!(eval("b101 ^ b110").unwrap(), Binary(3));
        assert_eq!(eval("b101 xor b110").unwrap(), Binary(3));
    }

    #[test]
    fn equal_test() {
        assert_eq!(eval("b101 == b110"), Ok(Binary(0)));
        assert_eq!(eval("b101 == b101"), Ok(Binary(1)));
        assert_eq!(eval("b101 equal b110"), Ok(Binary(0)));
        assert_eq!(eval("b101 equal b101"), Ok(Binary(1)));
        assert_eq!(eval("b101 eq b110"), Ok(Binary(0)));
        assert_eq!(eval("b101 eq b101"), Ok(Binary(1)));
    }

    #[test]
    fn not_equal_test() {
        assert_eq!(eval("b101 != b110"), Ok(Binary(1)));
        assert_eq!(eval("b101 != b101"), Ok(Binary(0)));

        assert_eq!(eval("b101 no_equal b110"), Ok(Binary(1)));
        assert_eq!(eval("b101 no_equal b101"), Ok(Binary(0)));

        assert_eq!(eval("b101 ne b110"), Ok(Binary(1)));
        assert_eq!(eval("b101 ne b101"), Ok(Binary(0)));
    }

    #[test]
    fn greater_test() {
        assert_eq!(eval("b101 > b110"), Ok(Binary(0)));
        assert_eq!(eval("b101 > b101"), Ok(Binary(0)));
        assert_eq!(eval("b110 > b101"), Ok(Binary(1)));

        assert_eq!(eval("b101 gt b110"), Ok(Binary(0)));
        assert_eq!(eval("b101 gt b101"), Ok(Binary(0)));
        assert_eq!(eval("b110 gt b101"), Ok(Binary(1)));
    }

    #[test]
    fn less_test() {
        assert_eq!(eval("b101 < b110"), Ok(Binary(1)));
        assert_eq!(eval("b101 < b101"), Ok(Binary(0)));
        assert_eq!(eval("b110 < b101"), Ok(Binary(0)));

        assert_eq!(eval("b101 lt b110"), Ok(Binary(1)));
        assert_eq!(eval("b101 lt b101"), Ok(Binary(0)));
        assert_eq!(eval("b110 lt b101"), Ok(Binary(0)));
    }

    #[test]
    fn greater_equal_test() {
        assert_eq!(eval("b101 >= b110"), Ok(Binary(0)));
        assert_eq!(eval("b101 >= b101"), Ok(Binary(1)));
        assert_eq!(eval("b110 >= b101"), Ok(Binary(1)));

        assert_eq!(eval("b101 gte b110"), Ok(Binary(0)));
        assert_eq!(eval("b101 gte b101"), Ok(Binary(1)));
        assert_eq!(eval("b110 gte b101"), Ok(Binary(1)));
    }

    #[test]
    fn less_equal_test() {
        assert_eq!(eval("b101 <= b110"), Ok(Binary(1)));
        assert_eq!(eval("b101 <= b101"), Ok(Binary(1)));
        assert_eq!(eval("b110 <= b101"), Ok(Binary(0)));

        assert_eq!(eval("b101 lte b110"), Ok(Binary(1)));
        assert_eq!(eval("b101 lte b101"), Ok(Binary(1)));
        assert_eq!(eval("b110 lte b101"), Ok(Binary(0)));
    }

    #[test]
    fn shift_right_test() {
        assert_eq!(eval("b101 >> 1"), Ok(Binary(2)));
        assert_eq!(eval("b101 >> 0"), Ok(Binary(5)));

        assert_eq!(eval("b101 shr 1"), Ok(Binary(2)));
        assert_eq!(eval("b101 shr 0"), Ok(Binary(5)));
    }

    #[test]
    fn shift_left_test() {
        assert_eq!(eval("b101 << 1"), Ok(Binary(10)));
        assert_eq!(eval("b101 << 0"), Ok(Binary(5)));

        assert_eq!(eval("b101 shl 1"), Ok(Binary(10)));
        assert_eq!(eval("b101 shl 0"), Ok(Binary(5)));
    }
}