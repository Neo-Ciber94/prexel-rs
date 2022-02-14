use std::iter::Peekable;
use std::str::Chars;
use crate::context::Context;

/// A trait that provides a method to convert a string into a sequence of tokens.
pub trait Splitter {
    /// Converts a string into a sequence of tokens.
    fn split_into_tokens(&self, expression: &str) -> Vec<String>;
}

/// Defines the method used to split a string.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum SplitStrategy {
    /// All the tokens will be retrieve including whitespaces.
    None,
    /// All the tokens will be retrieve ignoring whitespaces.
    RemoveWhiteSpaces,
}

/// Provides a way to extract tokens from a `str`.
///
/// # Example
/// ```
/// use prexel::utils::splitter::{DefaultSplitter, Splitter};
///
/// let splitter = DefaultSplitter::default();
/// let tokens = splitter.split_into_tokens("2 + 3");
/// assert_eq!(["2", "+", "3"].to_vec(), tokens);
/// ```
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct DefaultSplitter(pub SplitStrategy);

impl DefaultSplitter {
    #[inline]
    pub const fn new(kind: SplitStrategy) -> DefaultSplitter {
        DefaultSplitter(kind)
    }
}

impl Splitter for DefaultSplitter {
    fn split_into_tokens(&self, expression: &str) -> Vec<String> {
        let mut tokens = Vec::new();
        let mut iterator = expression.chars().peekable();

        while let Some(next) = iterator.next() {
            match next {
                'a'..='z' | 'A'..='Z' => {
                    let mut temp = next.to_string();
                    next_alphanumeric(&mut temp, &mut iterator);
                    tokens.push(temp);
                }
                '0'..='9' => {
                    let mut temp = next.to_string();
                    next_numeric(&mut temp, &mut iterator);
                    tokens.push(temp);
                }
                ' ' => match self.0 {
                    SplitStrategy::None => tokens.push(String::from(" ")),
                    SplitStrategy::RemoveWhiteSpaces => {}
                },
                c => tokens.push(c.to_string()),
            }
        }

        tokens
    }
}

impl Default for DefaultSplitter {
    fn default() -> Self {
        DefaultSplitter(SplitStrategy::RemoveWhiteSpaces)
    }
}

/// A `Splitter` implementation that accepts a custom interceptor.
///
/// An `interceptor` is a function that will be called for each token
/// and allow to include custom logic for split the string.
pub struct SplitterWithInterceptor<F>
where
    F: Fn(char, &mut Peekable<Chars>) -> Option<String>,
{
    interceptor: F,
    strategy: SplitStrategy,
}

impl<F> SplitterWithInterceptor<F>
where
    F: Fn(char, &mut Peekable<Chars>) -> Option<String>,
{
    /// Creates a new `SplitterWithInterceptor` with the given interceptor and `SplitStrategy::RemoveWhiteSpaces`.
    pub fn new(interceptor: F) -> Self {
        Self::with_strategy(interceptor, SplitStrategy::RemoveWhiteSpaces)
    }

    /// Creates a new `SplitterWithInterceptor` with the given interceptor and `SplitStrategy`.
    pub fn with_strategy(interceptor: F, strategy: SplitStrategy) -> Self {
        SplitterWithInterceptor { interceptor, strategy }
    }

    /// Returns the `SplitStrategy` used by the `SplitterWithInterceptor`.
    pub fn strategy(&self) -> SplitStrategy {
        self.strategy
    }
}

impl<F> Splitter for SplitterWithInterceptor<F>
where
    F: Fn(char, &mut Peekable<Chars>) -> Option<String>,
{
    fn split_into_tokens(&self, expression: &str) -> Vec<String> {
        let mut tokens = Vec::new();
        let mut iterator = expression.chars().peekable();

        while let Some(next) = iterator.next() {
            if let Some(token) = (self.interceptor)(next, &mut iterator) {
                tokens.push(token);
            } else {
                match next {
                    'a'..='z' | 'A'..='Z' => {
                        let mut temp = next.to_string();
                        next_alphanumeric(&mut temp, &mut iterator);
                        tokens.push(temp);
                    }
                    '0'..='9' => {
                        let mut temp = next.to_string();
                        next_numeric(&mut temp, &mut iterator);
                        tokens.push(temp);
                    }
                    ' ' => match self.strategy {
                        SplitStrategy::None => tokens.push(String::from(" ")),
                        SplitStrategy::RemoveWhiteSpaces => {}
                    },
                    c => tokens.push(c.to_string()),
                }
            }
        }

        tokens
    }
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

#[cfg(test)]
mod tests {
    use crate::utils::splitter::SplitterWithInterceptor;
    use super::DefaultSplitter;
    use super::{SplitStrategy, Splitter};

    #[test]
    fn split_into_tokens() {
        let splitter = DefaultSplitter::default();
        assert_eq!(
            ["10", "+", "-", "2", "*", "Sin", "(", "45", ")"].to_vec(),
            splitter.split_into_tokens("10 + -2 * Sin(45)")
        );
        assert_eq!(
            ["10", "+", "(", "-", "3", ")", "*", "0.25"].to_vec(),
            splitter.split_into_tokens("10 + (-3) * 0.25")
        );
        assert_eq!(
            ["(", "x", "+", "y", ")", "-", "2", "^", "10"].to_vec(),
            splitter.split_into_tokens("(x+y)-2^10")
        );
        assert_eq!(
            ["Log2", "(", "25", ")", "*", "PI", "-", "2"].to_vec(),
            splitter.split_into_tokens("Log2(25) * PI - 2")
        );
        assert_eq!(
            ["2", "PI", "+", "10"].to_vec(),
            splitter.split_into_tokens("2PI + 10")
        );
        assert_eq!(
            ["x", "=", "10"].to_vec(),
            splitter.split_into_tokens("x = 10")
        );

        assert_eq!(
            ["5", " ", "*", " ", "2"].to_vec(),
            DefaultSplitter::new(SplitStrategy::None).split_into_tokens("5 * 2")
        );
    }

    #[test]
    fn split_with_interceptor_test() {
        let splitter = SplitterWithInterceptor::new(|c, rest| {
            if c == '@' {
                let mut temp = String::new();
                temp.push(c);
                while let Some(c) = rest.peek() {
                    if c.is_alphanumeric() {
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
        });

        let tokens = splitter.split_into_tokens("@125 + -@2 * Sin(@45)");
        assert_eq!(
            ["@125", "+", "-", "@2", "*", "Sin", "(", "@45", ")"].to_vec(),
            tokens
        );
    }
}
