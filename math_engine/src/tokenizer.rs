use std::marker::PhantomData;
use std::str::FromStr;

use crate::context::{Context, DefaultContext};
use crate::error::{Error, ErrorKind};
use crate::function::Notation;
use crate::num::checked::CheckedNum;
use crate::token::Token;
use crate::utils::option_ext::OptionStrExt;
use crate::utils::string_tokenizer::{StringTokenizer, TokenizeKind};
use crate::Result;

/// Provides a way to retrieve the tokens of an expression.
pub trait Tokenize<N> {
    /// Gets the tokens of the specified expression.
    fn tokenize(&self, expression: &str) -> Result<Vec<Token<N>>>;
}

/// The default `Tokenizer`.
///
/// # Example
/// ```
/// use math_engine::tokenizer::{Tokenizer, Tokenize};
/// use math_engine::token::Token::*;
///
/// let t : Tokenizer<i32> = Tokenizer::new();
/// let tokens = t.tokenize("2 + 3").unwrap();
/// assert_eq!(&[Number(2_i32), BinaryOperator('+'), Number(3_i32)], tokens.as_slice());
/// ```
pub struct Tokenizer<'a, N, C = DefaultContext<'a, N>>
where
    C: Context<'a, N>,
{
    /// The context which contains the variables, constants and functions used
    /// for tokenize and expression.
    context: &'a C,
    _marker: PhantomData<N>,
}

impl<'a, N> Tokenizer<'a, N, DefaultContext<'a, N>>
where
    N: CheckedNum + 'static,
{
    /// Constructs a new `Tokenizer` using the default checked context.
    #[inline]
    pub fn new() -> Self {
        Tokenizer {
            context: unsafe { DefaultContext::instance() },
            _marker: PhantomData,
        }
    }
}

impl<'a, N, C> Tokenizer<'a, N, C>
where
    C: Context<'a, N>,
    N: FromStr,
{
    /// Constructs a new `Tokenizer` with the given `Context`.
    #[inline]
    pub fn with_context(context: &'a C) -> Self {
        Tokenizer {
            context,
            _marker: PhantomData,
        }
    }
}

impl<'a, N, C> Tokenize<N> for Tokenizer<'a, N, C>
where
    C: Context<'a, N>,
    N: FromStr,
{
    fn tokenize(&self, expression: &str) -> Result<Vec<Token<N>>> {
        const STRING_TOKENIZER: StringTokenizer =
            StringTokenizer::new(TokenizeKind::RemoveWhiteSpaces);
        const COMMA: &str = ",";
        const WHITESPACE: &str = " ";

        if expression.is_empty() {
            return Err(Error::new(
                ErrorKind::Empty,
                "Expression is empty",
            ));
        }

        // `Vec` used for fast access indexing, Iterator.nth(..) could be O(N)
        let raw_tokens = STRING_TOKENIZER.get_tokens(expression);
        // Actual iterator over the string tokens.
        let mut iter = raw_tokens.iter().enumerate().peekable();
        // Stores the tokens to return.
        let mut tokens = Vec::new();
        // Context that contains the variables, constants and functions.
        let context = self.context;

        while let Some((pos, string)) = iter.next() {
            if is_number(string) {
                // `complex_number` is enable in the context, check the next value and
                // if is the imaginary unit append it to the current number.
                if context.config().complex_number && iter.peek().map(|s| s.1).contains_str("i") {
                    let mut temp = string.clone();
                    let im = iter.next().unwrap().1;
                    temp.push_str(im);

                    let n = N::from_str(&temp).map_err(|_| {
                        Error::new(
                            ErrorKind::InvalidInput,
                            format!(
                                "failed to parse `{}` to `{}`.",
                                temp,
                                std::any::type_name::<N>()
                            ),
                        )
                    })?;
                    tokens.push(Token::Number(n));
                } else {
                    let n = N::from_str(string).map_err(|_| {
                        Error::new(
                            ErrorKind::InvalidInput,
                            format!(
                                "failed to parse `{}` to `{}`.",
                                string,
                                std::any::type_name::<N>()
                            ),
                        )
                    })?;
                    tokens.push(Token::Number(n));
                }
            } else if context.is_variable(string) {
                tokens.push(Token::Variable(string.clone()));
            } else if context.is_constant(string) {
                tokens.push(Token::Constant(string.clone()));
            } else if context.is_function(string) {
                tokens.push(Token::Function(string.clone()));
            } else if context.is_binary_function(string) || context.is_unary_function(string) {
                let prev = if pos == 0 {
                    None
                } else {
                    Some(raw_tokens[pos - 1].as_str())
                };
                let next = if pos == raw_tokens.len() - 1 {
                    None
                } else {
                    Some(raw_tokens[pos].as_str())
                };

                if is_unary(prev, string, next, context) {
                    let operator = string.chars().next().unwrap();
                    tokens.push(Token::UnaryOperator(operator));
                } else {
                    // If the operator is not unary, could be binary so need 2 operands.
                    if prev.is_none() || next.is_none() {
                        return Err(Error::new(
                            ErrorKind::InvalidExpression,
                            format!(
                                "binary operations need 2 operands: {:?} {} {:?}",
                                prev, string, next
                            ),
                        ));
                    }

                    // If the current string value length is 1 we assume is a symbol
                    // for a binary operator, otherwise is an infix function.
                    if string.len() == 1 {
                        let operator = string.chars().next().unwrap();
                        tokens.push(Token::BinaryOperator(operator));
                    } else {
                        tokens.push(Token::InfixFunction(string.clone()));
                    }
                }
            } else if string == COMMA {
                tokens.push(Token::Comma);
            } else if string == WHITESPACE {
                // Ignore whitespaces
            } else {
                if string.len() == 1 {
                    // If string token length is 1 and its not considered a binary operator, unary operator
                    // or a function we check if is a grouping symbol in the context `Config`.
                    let c = string.chars().next().unwrap();
                    if let Some(symbol) = context.config().get_group_symbol(c) {
                        if c == symbol.group_open {
                            tokens.push(Token::GroupingOpen(c));
                        } else {
                            tokens.push(Token::GroupingClose(c));
                        }
                        continue;
                    }
                }

                tokens.push(Token::Unknown(string.clone()));
            }
        }

        Ok(tokens)
    }
}

fn is_unary<'a, N>(
    prev: Option<&str>,
    cur: &str,
    next: Option<&str>,
    context: &impl Context<'a, N>,
) -> bool {
    if let Some(op) = context.get_unary_function(cur) {
        if op.notation() == Notation::Postfix {
            prev.map_or(false, |s| {
                s == ")" || is_number(s) || context.is_constant(s) || context.is_variable(s)
            })
        } else {
            if next.is_none() {
                // 10-, (24)+
                return false;
            }

            if let Some(prev_str) = prev {
                // 10+, 2+(2), (4)-10
                if prev_str == ")"
                    || prev_str == "]"
                    || is_number(prev_str)
                    || context.is_variable(prev_str)
                    || context.is_constant(prev_str)
                {
                    return false;
                }

                // 10! - 2
                if context.is_unary_function(&prev_str[..1])
                    && !context.is_binary_function(&prev_str[..1])
                {
                    return false;
                }

                // +-, (-, !+
                if prev_str.len() == 1 {
                    let c = prev_str.chars().last().unwrap();
                    c.is_ascii_punctuation()
                } else {
                    true
                }
            } else {
                // -10, +(25)
                true
            }
        }
    } else {
        false
    }
}

fn is_number(value: &str) -> bool {
    if value == "0" {
        return true;
    }

    if value.is_empty() {
        return false;
    }

    let mut has_decimal_point = false;
    let is_signed = value.starts_with('+') || value.starts_with('-');
    let mut iterator = value.chars().enumerate();

    if is_signed && value.len() == 1 {
        return false;
    }

    if is_signed {
        iterator.next();
    }

    for item in iterator {
        match item {
            (n, '0') => {
                let starts_with_zero = if is_signed {
                    value[1..].starts_with('0')
                } else {
                    value.starts_with('0')
                };

                if !has_decimal_point && starts_with_zero && n > 0 {
                    //+00, 00
                    if let Some(c) = value.chars().nth(n - 1) {
                        if c == '0' {
                            return false;
                        }
                    }
                }
            }
            (_, '1'..='9') => {}
            (n, '.') if n < value.len() - 1 => {
                if !is_signed || n > 1 {
                    if has_decimal_point {
                        return false;
                    } else {
                        has_decimal_point = true;
                    }
                } else {
                    return false;
                }
            }
            _ => {
                return false;
            }
        }
    }

    true
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::token::Token::*;

    #[test]
    fn is_number_test() {
        assert!(is_number("0"));
        assert!(is_number("6"));
        assert!(is_number("700"));
        assert!(is_number("567"));
        assert!(is_number("1000000.05"));
        assert!(is_number("10.0"));
        assert!(is_number("-102"));
        assert!(is_number("+55"));
        assert!(is_number("+66.4"));
        assert!(is_number("-45.90"));
        assert!(is_number("0.0001"));
        assert!(is_number(".10"));

        assert!(!is_number("00"));
        assert!(!is_number("+00"));
        assert!(!is_number(""));
        assert!(!is_number(" "));
        assert!(!is_number("."));
        assert!(!is_number("+"));
        assert!(!is_number("-"));
        assert!(!is_number("89."));
        assert!(!is_number("10+"));
        assert!(!is_number("20-"));
        assert!(!is_number("+.10"));
        assert!(!is_number("-.25"));
        assert!(!is_number("1..2"));
    }

    #[test]
    fn is_unary_test() {
        let context: &DefaultContext<i64> = &DefaultContext::new_checked();
        assert!(is_unary(None, "-", Some("5"), context));
        assert!(is_unary(None, "-", Some("Pi"), context));
        assert!(is_unary(Some("("), "-", Some("5"), context));
        assert!(is_unary(Some("("), "-", Some("Pi"), context));
        assert!(is_unary(Some("+"), "-", Some("5"), context));
        assert!(is_unary(Some("+"), "-", Some("E"), context));
        assert!(is_unary(Some("5"), "!", None, context));
        assert!(is_unary(Some("E"), "!", None, context));
        assert!(is_unary(Some(","), "-", Some("5"), context));
        assert!(is_unary(Some("@"), "-", Some("5"), context));

        assert!(!is_unary(Some("3"), "-", Some("5"), context));
        assert!(!is_unary(Some(")"), "-", Some("5"), context));
        assert!(!is_unary(Some("E"), "-", Some("Pi"), context));
        assert!(!is_unary(Some(")"), "-", Some("E"), context));
        assert!(!is_unary(Some(")"), "-", Some("("), context));
    }

    #[test]
    fn tokenize_test() {
        let context: &DefaultContext<i64> = &DefaultContext::new_checked();
        let tokenizer: Tokenizer<i64> = Tokenizer::with_context(context);
        assert_eq!(
            &tokenizer.tokenize("2 + 3").unwrap(),
            &[Number(2), BinaryOperator('+'), Number(3)]
        );

        assert_eq!(
            &tokenizer.tokenize("5 * Sin(pi)").unwrap(),
            &[
                Number(5),
                BinaryOperator('*'),
                Function(String::from("Sin")),
                GroupingOpen('('),
                Constant(String::from("pi")),
                GroupingClose(')')
            ]
        );

        assert_eq!(
            &tokenizer.tokenize("10/2 mod 3^2").unwrap(),
            &[
                Number(10),
                BinaryOperator('/'),
                Number(2),
                InfixFunction(String::from("mod")),
                Number(3),
                BinaryOperator('^'),
                Number(2)
            ]
        );

        assert_eq!(
            &tokenizer.tokenize("10! + 2").unwrap(),
            &[
                Number(10),
                UnaryOperator('!'),
                BinaryOperator('+'),
                Number(2)
            ]
        );

        assert_eq!(
            &tokenizer.tokenize("600!").unwrap(),
            &[Number(600), UnaryOperator('!')]
        );

        assert_eq!(
            &tokenizer.tokenize("10 2").unwrap(),
            &[Number(10), Number(2)]
        );

        #[cfg(feature = "complex")]
        {
            use num_complex::Complex;
            use num_complex::Complex64;
            let context: &DefaultContext<Complex64> = &DefaultContext::new_complex();
            let complex_tokenizer = Tokenizer::with_context(context);

            assert_eq!(
                &complex_tokenizer.tokenize("5 + 3i").unwrap(),
                &[
                    Number(Complex64::new(5_f64, 0_f64)),
                    BinaryOperator('+'),
                    Number(Complex64::new(0_f64, 3_f64)),
                ]
            );
        }
    }
}
