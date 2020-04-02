use std::borrow::Borrow;

/// Extensions methods for strings.
pub trait StrExt{
    /// Compares the string with the specified `char`.
    fn eq_char(&self, ch: char) -> bool;

    /// Gets the single `char` of the string or `None` if the string have more than 1 char.
    fn single_char(&self) -> Option<char>;

    /// Converts the string to capital case.
    ///
    /// # Remarks
    /// For an input `hello world` should produce `Hello world`
    fn to_sentence_case(&self) -> String;

    /// Converts the string to capital case.
    ///
    /// # Remarks
    /// For an input `hello world` should produce `Hello World`
    fn to_capital_case(&self) -> String;
}

impl<'a, S: Borrow<&'a str>> StrExt for S{
    #[inline]
    fn eq_char(&self, ch: char) -> bool {
        self.borrow().chars().next().map_or(false, |c| c == ch)
            && self.borrow().chars().count() == 1
    }

    #[inline]
    fn single_char(&self) -> Option<char> {
        if self.borrow().chars().count() != 1{
            return None;
        }

        return self.borrow()
            .chars()
            .next();
    }

    fn to_sentence_case(&self) -> String {
        let mut temp = String::new();
        let mut is_uppercase = false;

        for c in self.borrow().chars(){
            if !is_uppercase && c.is_alphanumeric(){
                temp.push(c.to_ascii_uppercase());
                is_uppercase = true;
            }
            else{
                temp.push(c.to_ascii_lowercase());
            }
        }

        temp
    }

    fn to_capital_case(&self) -> String {
        let mut temp = String::new();
        let mut next_uppercase = true;

        for c in self.borrow().chars(){
            if next_uppercase && c.is_alphanumeric(){
                temp.push(c.to_ascii_uppercase());
                next_uppercase = false;
            }
            else if c.is_whitespace(){
                temp.push(c);
                next_uppercase = true;
            }
            else{
                temp.push(c.to_ascii_lowercase());
            }
        }

        temp
    }
}

#[cfg(test)]
mod tests{
    use super::*;

    #[test]
    fn eq_char_test(){
        assert!(!"hi".eq_char('h'));
        assert!("a".eq_char('a'));
    }

    #[test]
    fn single_char_test(){
        assert_eq!("hi".single_char(), None);
        assert_eq!("a".single_char(), Some('a'));
    }

    #[test]
    fn to_sentence_case_test(){
        assert_eq!("hello world".to_sentence_case(), "Hello world".to_string());
        assert_eq!("  hello world".to_sentence_case(), "  Hello world".to_string());
        assert_eq!("1hello world".to_sentence_case(), "1hello world".to_string());
    }

    #[test]
    fn to_capital_case_test(){
        assert_eq!("hello world".to_capital_case(), "Hello World".to_string());
        assert_eq!("  hello world".to_capital_case(), "  Hello World".to_string());
        assert_eq!("1hello world".to_capital_case(), "1hello World".to_string());
    }
}