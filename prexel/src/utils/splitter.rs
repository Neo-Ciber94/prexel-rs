use crate::utils::splitter::rules::{Outcome, SplitRule};

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
    rules: Vec<Box<dyn SplitRule + 'a>>,
}

impl<'a> DefaultSplitter<'a> {
    #[inline]
    pub fn new(kind: SplitWhitespaceOption) -> DefaultSplitter<'a> {
        DefaultSplitterBuilder::default()
            .rule(rules::SplitNumeric)
            .rule(rules::SplitIdentifier)
            .rule(rules::SplitOperator)
            .whitespace(kind)
            .build()
    }

    pub fn with_numeric_rule<F: 'a>(rule: F) -> Self
    where
        F: SplitRule + 'a,
    {
        DefaultSplitterBuilder::default()
            .rule(rule)
            .rule(rules::SplitIdentifier)
            .rule(rules::SplitOperator)
            .whitespace(SplitWhitespaceOption::Remove)
            .build()
    }

    pub fn with_identifier_rule<F: 'a>(rule: F) -> Self
    where
        F: SplitRule + 'a,
    {
        DefaultSplitterBuilder::default()
            .rule(rules::SplitNumeric)
            .rule(rule)
            .rule(rules::SplitIdentifier)
            .rule(rules::SplitOperator)
            .whitespace(SplitWhitespaceOption::Remove)
            .build()
    }

    #[inline]
    pub fn builder() -> DefaultSplitterBuilder<'a> {
        DefaultSplitterBuilder::default()
    }

    #[inline]
    pub fn rules(&self) -> &[Box<dyn SplitRule + 'a>] {
        self.rules.as_slice()
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
                match rule.split(c, &mut iterator) {
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
    rules: Vec<Box<dyn SplitRule + 'a>>,
    whitespace_option: Option<SplitWhitespaceOption>,
}

impl<'a> DefaultSplitterBuilder<'a> {
    pub fn new() -> Self {
        DefaultSplitterBuilder {
            rules: Vec::new(),
            whitespace_option: None,
        }
    }

    pub fn insert_rule<F: 'a>(mut self, index: usize, rule: F) -> Self
    where
        F: SplitRule + 'a,
    {
        self.rules.insert(index, Box::new(rule));
        self
    }

    pub fn rule<F: 'a>(mut self, rule: F) -> Self
    where
        F: SplitRule + 'a,
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
            SplitWhitespaceOption::None => {}
            SplitWhitespaceOption::Remove => rules.push(Box::new(rules::SkipWhitespace)),
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
    use std::collections::HashSet;
    use std::iter::Peekable;
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

    pub trait SplitRule {
        fn split(&self, c: char, rest: &mut Peekable<Chars>) -> Outcome;
    }

    pub struct SplitIdentifier;
    impl SplitRule for SplitIdentifier {
        fn split(&self, c: char, rest: &mut Peekable<Chars>) -> Outcome {
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
    }

    pub struct SplitNumeric;
    impl SplitRule for SplitNumeric {
        fn split(&self, c: char, rest: &mut Peekable<Chars>) -> Outcome {
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
    }

    pub struct SplitOperator;
    impl SplitRule for SplitOperator {
        fn split(&self, c: char, rest: &mut Peekable<Chars>) -> Outcome {
            fn is_valid_char(c: &char) -> bool {
                matches!(
                    c,
                    '~' | '`'
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
    }

    pub struct SplitWithOperatorsBuilder {
        operators: HashSet<char>,
    }
    impl SplitWithOperatorsBuilder {
        pub fn new() -> Self {
            SplitWithOperatorsBuilder {
                operators: HashSet::new(),
            }
        }

        pub fn with_default_operators() -> Self {
            let operators = HashSet::from([
                '~', '`', '!', '@', '#', '$', '%', '^', '&', '*', '-', '+', '_', ':', ';', '"',
                '\'', '|', '\\', '?', '.', '<', '>', '/', '=', ',',
            ]);
            SplitWithOperatorsBuilder { operators }
        }

        pub fn except(mut self, operator: char) -> Self {
            self.operators.remove(&operator);
            self
        }

        pub fn build(self) -> SplitWithOperators {
            SplitWithOperators {
                operators: self.operators,
            }
        }
    }

    pub struct SplitWithOperators {
        operators: HashSet<char>,
    }
    impl SplitWithOperators {
        pub fn new() -> Self {
            SplitWithOperatorsBuilder::with_default_operators().build()
        }

        pub fn is_valid(&self, c: &char) -> bool {
            self.operators.contains(c)
        }
    }
    impl SplitRule for SplitWithOperators {
        fn split(&self, c: char, rest: &mut Peekable<Chars>) -> Outcome {
            match c {
                _ if self.is_valid(&c) => {
                    let mut temp = String::new();
                    temp.push(c);

                    while let Some(c) = rest.next_if(|c| self.is_valid(c)) {
                        temp.push(c);
                    }

                    Outcome::Data(temp)
                }
                _ => Outcome::Continue,
            }
        }
    }

    pub struct SkipWhitespace;
    impl SplitRule for SkipWhitespace {
        fn split(&self, c: char, _: &mut Peekable<Chars>) -> Outcome {
            if c.is_whitespace() {
                return Outcome::Skip;
            }

            Outcome::Continue
        }
    }

    #[cfg(feature = "binary")]
    pub struct SplitBinary;

    #[cfg(feature = "binary")]
    impl SplitRule for SplitBinary {
        fn split(&self, c: char, rest: &mut Peekable<Chars>) -> Outcome {
            fn is_next_binary(chars: &mut Peekable<Chars>) -> bool {
                chars.peek() == Some(&'1') || chars.peek() == Some(&'0')
            }

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
        }
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
