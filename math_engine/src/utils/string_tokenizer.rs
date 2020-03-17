use std::iter::Peekable;
use std::str::CharIndices;

/// Defines the method of the `StringTokenizer` to extract the tokens.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum TokenizeKind{
    /// All the tokens will be retrieve including whitespaces.
    None,
    /// All the tokens will be retrieve ignoring whitespaces.
    RemoveWhiteSpaces
}

/// Provides a way to extract tokens from an `str`.
///
/// # Example
/// ```
/// use math_engine::utils::string_tokenizer::StringTokenizer;
/// let tokenizer = StringTokenizer::default();
/// let tokens = tokenizer.get_tokens("2 + 3");
/// assert_eq!(["2", "+", "3"].to_vec(), tokens);
/// ```
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct StringTokenizer(TokenizeKind);

impl StringTokenizer{
    #[inline]
    pub const fn new(kind: TokenizeKind) -> StringTokenizer{
        StringTokenizer(kind)
    }

    pub fn get_tokens(&self, expression: &str) -> Vec<String>{
        let mut tokens = Vec::new();
        let mut iterator = expression.char_indices().peekable();

        while let Some((_, next)) = iterator.next(){
            match next{
                'a'..='z' | 'A'..='Z' => {
                    let mut temp = String::from(next.to_string());
                    Self::next_alphanumeric(&mut temp, &mut iterator);
                    tokens.push(temp);
                },
                '0'..='9' => {
                    let mut temp = String::from(next.to_string());
                    Self::next_numeric(&mut temp, &mut iterator);
                    tokens.push(temp);
                },
                ' ' => {
                    match self.0{
                        TokenizeKind::None => { tokens.push(String::from(" "))},
                        TokenizeKind::RemoveWhiteSpaces => {},
                    }
                },
                c => tokens.push(String::from(c.to_string()))
            }
        }

        tokens
    }

    fn next_alphanumeric(dest: &mut String, iterator: &mut Peekable<CharIndices>){
        while let Some((_, c)) = iterator.peek(){
            if c.is_alphanumeric(){
                dest.push(*c);
                iterator.next();
            }
            else{
                break;
            }
        }
    }

    fn next_numeric(dest: &mut String, iterator: &mut Peekable<CharIndices>){
        let mut has_decimal_point = false;

        while let Some((_, c)) = iterator.peek() {
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
            }
            else {
                break;
            }
        }
    }
}

impl Default for StringTokenizer{
    fn default() -> Self {
        StringTokenizer(TokenizeKind::RemoveWhiteSpaces)
    }
}

#[cfg(test)]
mod tests{
    use super::StringTokenizer;
    use crate::utils::string_tokenizer::TokenizeKind;

    #[test]
    fn get_tokens_test(){
        let tokenizer = StringTokenizer::default();
        assert_eq!(["10", "+", "-", "2", "*", "Sin", "(", "45", ")"].to_vec(), tokenizer.get_tokens("10 + -2 * Sin(45)"));
        assert_eq!(["10", "+", "(", "-", "3", ")", "*", "0.25"].to_vec(), tokenizer.get_tokens("10 + (-3) * 0.25"));
        assert_eq!(["(", "x", "+", "y", ")", "-", "2", "^", "10"].to_vec(), tokenizer.get_tokens("(x+y)-2^10"));
        assert_eq!(["Log2", "(", "25", ")", "*", "PI", "-", "2"].to_vec(), tokenizer.get_tokens("Log2(25) * PI - 2"));
        assert_eq!(["2", "PI", "+", "10"].to_vec(), tokenizer.get_tokens("2PI + 10"));
        assert_eq!(["x", "=", "10"].to_vec(), tokenizer.get_tokens("x = 10"));

        assert_eq!(["5", " ", "*", " ", "2"].to_vec(), StringTokenizer::new(TokenizeKind::None).get_tokens("5 * 2"));
    }
}