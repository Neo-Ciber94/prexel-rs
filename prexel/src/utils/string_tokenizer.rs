use std::iter::Peekable;
use std::str::Chars;

/// A trait that provides a method to convert a string into a sequence of tokens.
pub trait StringTokenizer {
    /// Converts a string into a sequence of tokens.
    fn get_tokens(&self, expression: &str) -> Vec<String>;
}

/// Defines the method of the `StringTokenizer` to extract the tokens.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum TokenizeStrategy {
    /// All the tokens will be retrieve including whitespaces.
    None,
    /// All the tokens will be retrieve ignoring whitespaces.
    RemoveWhiteSpaces,
}

/// Provides a way to extract tokens from an `str`.
///
/// # Example
/// ```
/// use prexel::utils::string_tokenizer::{DefaultStringTokenizer, StringTokenizer};
///
/// let tokenizer = DefaultStringTokenizer::default();
/// let tokens = tokenizer.get_tokens("2 + 3");
/// assert_eq!(["2", "+", "3"].to_vec(), tokens);
/// ```
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct DefaultStringTokenizer(pub TokenizeStrategy);

impl DefaultStringTokenizer {
    #[inline]
    pub const fn new(kind: TokenizeStrategy) -> DefaultStringTokenizer {
        DefaultStringTokenizer(kind)
    }

    fn next_alphanumeric(dest: &mut String, iterator: &mut Peekable<Chars>) {
        while let Some(c) = iterator.peek() {
            if c.is_alphanumeric() {
                dest.push(*c);
                iterator.next();
            } else {
                break;
            }
        }
    }

    fn next_numeric(dest: &mut String, iterator: &mut Peekable<Chars>) {
        let mut has_decimal_point = false;

        while let Some(c) = iterator.peek() {
            if *c == '.' || c.is_ascii_digit() {
                if *c == '.' {
                    if has_decimal_point {
                        break;
                    } else {
                        has_decimal_point = true;
                    }
                }

                dest.push(*c);
                iterator.next();
            } else {
                break;
            }
        }
    }
}

impl StringTokenizer for DefaultStringTokenizer {
    fn get_tokens(&self, expression: &str) -> Vec<String> {
        let mut tokens = Vec::new();
        let mut iterator = expression.chars().peekable();

        while let Some(next) = iterator.next() {
            match next {
                'a'..='z' | 'A'..='Z' => {
                    let mut temp = next.to_string();
                    Self::next_alphanumeric(&mut temp, &mut iterator);
                    tokens.push(temp);
                }
                '0'..='9' => {
                    let mut temp = next.to_string();
                    Self::next_numeric(&mut temp, &mut iterator);
                    tokens.push(temp);
                }
                ' ' => match self.0 {
                    TokenizeStrategy::None => tokens.push(String::from(" ")),
                    TokenizeStrategy::RemoveWhiteSpaces => {}
                },
                c => tokens.push(c.to_string()),
            }
        }

        tokens
    }
}

impl Default for DefaultStringTokenizer {
    fn default() -> Self {
        DefaultStringTokenizer(TokenizeStrategy::RemoveWhiteSpaces)
    }
}

#[cfg(test)]
mod tests {
    use super::DefaultStringTokenizer;
    use super::{TokenizeStrategy, StringTokenizer};

    #[test]
    fn get_tokens_test() {
        let tokenizer = DefaultStringTokenizer::default();
        assert_eq!(
            ["10", "+", "-", "2", "*", "Sin", "(", "45", ")"].to_vec(),
            tokenizer.get_tokens("10 + -2 * Sin(45)")
        );
        assert_eq!(
            ["10", "+", "(", "-", "3", ")", "*", "0.25"].to_vec(),
            tokenizer.get_tokens("10 + (-3) * 0.25")
        );
        assert_eq!(
            ["(", "x", "+", "y", ")", "-", "2", "^", "10"].to_vec(),
            tokenizer.get_tokens("(x+y)-2^10")
        );
        assert_eq!(
            ["Log2", "(", "25", ")", "*", "PI", "-", "2"].to_vec(),
            tokenizer.get_tokens("Log2(25) * PI - 2")
        );
        assert_eq!(
            ["2", "PI", "+", "10"].to_vec(),
            tokenizer.get_tokens("2PI + 10")
        );
        assert_eq!(["x", "=", "10"].to_vec(), tokenizer.get_tokens("x = 10"));

        assert_eq!(
            ["5", " ", "*", " ", "2"].to_vec(),
            DefaultStringTokenizer::new(TokenizeStrategy::None).get_tokens("5 * 2")
        );
    }
}
