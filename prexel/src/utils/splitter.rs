use std::iter::Peekable;
use std::marker::PhantomData;
use std::str::Chars;

/// The result of a split rule.
pub enum Outcome {
    /// The result of a split.
    Data(String),
    /// Continue to the next rule.
    Continue,
    /// Skips the current `char`.
    Skip,
}

/// A rule for splitting a string into tokens.
pub type SplitRule<'a> = Box<dyn Fn(char, &mut Peekable<Chars>) -> Outcome + 'a>;

/// A trait that provides a method to convert a string into a sequence of tokens.
pub trait Splitter {
    /// Converts a string into a sequence of tokens.
    fn split_into_tokens(&self, expression: &str) -> Vec<String>;
}

/// Options used for whitespaces.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum SplitWhitespaceOption {
    /// All the tokens will be retrieve including whitespaces.
    None,
    /// All the tokens will be retrieve ignoring whitespaces.
    Remove,
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
pub struct DefaultSplitter<'a> {
    rules: Vec<SplitRule<'a>>,
}

impl<'a> DefaultSplitter<'a> {
    #[inline]
    pub fn new(kind: SplitWhitespaceOption) -> DefaultSplitter<'a> {
        DefaultSplitterBuilder::default()
            .rule(rules::next_numeric)
            .rule(rules::next_identifier)
            .rule(rules::next_operator)
            .whitespace(kind)
            .build()
    }

    pub fn with_numeric_rule<F: 'a>(rule: F) -> Self
    where
        F: Fn(char, &mut Peekable<Chars>) -> Outcome,
    {
        DefaultSplitterBuilder::default()
            .rule(rule)
            .rule(rules::next_identifier)
            .rule(rules::next_operator)
            .whitespace(SplitWhitespaceOption::Remove)
            .build()
    }

    #[inline]
    pub fn builder() -> DefaultSplitterBuilder<'a> {
        DefaultSplitterBuilder::default()
    }

    #[inline]
    pub fn rules(&self) -> &[SplitRule] {
        &self.rules
    }
}

impl Splitter for DefaultSplitter<'_> {
    fn split_into_tokens(&self, expression: &str) -> Vec<String> {
        let mut tokens = Vec::new();
        let mut iterator = expression.chars().peekable();

        while let Some(c) = iterator.peek().cloned() {
            iterator.next();

            let mut next = false;

            for rule in &self.rules {
                match rule(c, &mut iterator) {
                    Outcome::Data(s) => {
                        tokens.push(s);
                        next = true;
                        break;
                    }
                    Outcome::Continue => {
                        continue;
                    }
                    Outcome::Skip => {
                        next = true;
                        break;
                    }
                }
            }

            if !next {
                tokens.push(c.to_string());
            }
        }

        tokens
    }
}

impl Default for DefaultSplitter<'_> {
    fn default() -> Self {
        DefaultSplitter::new(SplitWhitespaceOption::Remove)
    }
}

pub struct DefaultSplitterBuilder<'a> {
    rules: Vec<SplitRule<'a>>,
    whitespace_option: Option<SplitWhitespaceOption>,
    _marker: &'a PhantomData<()>,
}

impl<'a> DefaultSplitterBuilder<'a> {
    pub fn new() -> Self {
        DefaultSplitterBuilder {
            rules: Vec::new(),
            whitespace_option: None,
            _marker: &PhantomData,
        }
    }

    pub fn rule<F: 'a>(mut self, rule: F) -> Self
    where
        F: Fn(char, &mut Peekable<Chars>) -> Outcome,
    {
        self.rules.push(Box::new(rule));
        self
    }

    pub fn whitespace(mut self, option: SplitWhitespaceOption) -> Self {
        self.whitespace_option = Some(option);
        self
    }

    pub fn build(self) -> DefaultSplitter<'a> {
        let DefaultSplitterBuilder {
            rules,
            whitespace_option,
            ..
        } = self;

        let mut rules = rules;
        let whitespace_option = whitespace_option.unwrap_or(SplitWhitespaceOption::None);

        match whitespace_option {
            SplitWhitespaceOption::None => {},
            SplitWhitespaceOption::Remove => rules.push(Box::new(rules::skip_whitespace)),
        };

        DefaultSplitter { rules }
    }
}

impl Default for DefaultSplitterBuilder<'_> {
    fn default() -> Self {
        DefaultSplitterBuilder::new()
    }
}

pub mod rules {
    use crate::utils::splitter::Outcome;
    use std::iter::Peekable;
    use std::str::Chars;

    pub fn next_identifier(c: char, rest: &mut Peekable<Chars>) -> Outcome {
        #[inline]
        fn is_valid_char(c: &char) -> bool {
            matches!(c, 'a'..='z' | 'A'..='Z' | '0'..='9' | '_')
        }

        match c {
            'a'..='z' | 'A'..='Z' | '_' => {
                let mut temp = String::new();
                temp.push(c);

                while let Some(c) = rest.next_if(is_valid_char) {
                    temp.push(c);
                }

                Outcome::Data(temp)
            }
            _ => Outcome::Continue,
        }
    }

    pub fn next_numeric(c: char, rest: &mut Peekable<Chars>) -> Outcome {
        #[inline]
        fn is_valid_char(c: &char) -> bool {
            matches!(c, '0'..='9' | '.')
        }

        match c {
            '0'..='9' => {
                let mut temp = String::new();
                temp.push(c);

                let mut has_decimal_point = false;

                while let Some(c) = rest.next_if(is_valid_char) {
                    if c == '.' {
                        if has_decimal_point {
                            break;
                        }

                        has_decimal_point = true;
                    }

                    temp.push(c);
                }

                Outcome::Data(temp)
            }
            _ => Outcome::Continue,
        }
    }

    pub fn next_operator(c: char, rest: &mut Peekable<Chars>) -> Outcome {
        fn is_valid_char(c: &char) -> bool {
            matches!(
                c, '~'
                | '`'
                | '!'
                | '@'
                | '#'
                | '$'
                | '%'
                | '^'
                | '&'
                | '*'
                | '-'
                | '+'
                | '_'
                | ':'
                | ';'
                | '"'
                | '\''
                | '|'
                | '\\'
                | '?'
                | '.'
                | '<'
                | '>'
                | '/'
                | '='
                | ','
            )
        }

        match c {
            _ if is_valid_char(&c) => {
                let mut temp = String::new();
                temp.push(c);

                while let Some(c) = rest.next_if(is_valid_char) {
                    temp.push(c);
                }

                Outcome::Data(temp)
            }
            _ => Outcome::Continue,
        }
    }

    pub fn skip_whitespace(c: char, _: &mut Peekable<Chars>) -> Outcome {
        if c.is_whitespace() {
            return Outcome::Skip;
        }

        Outcome::Continue
    }
}

#[cfg(test)]
mod tests {
    use super::DefaultSplitter;
    use super::{SplitWhitespaceOption, Splitter};

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
            DefaultSplitter::new(SplitWhitespaceOption::None).split_into_tokens("5 * 2")
        );

        assert_eq!(
            ["256", ">>", "3"].to_vec(),
            DefaultSplitter::default().split_into_tokens("256 >> 3")
        );
    }
}
