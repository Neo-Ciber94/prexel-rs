/// Extensions methods for strings.
pub trait StrExt{
    /// Compares the string with the specified `char`.
    fn eq_char(&self, ch: char) -> bool;
    /// Gets the single `char` of the string or `None` if the string have more than 1 char.
    fn as_single_char(&self) -> Option<char>;
}

impl StrExt for &str{
    fn eq_char(&self, ch: char) -> bool {
        self.chars().next().map_or(false, |c| c == ch) && self.chars().count() == 1
    }

    fn as_single_char(&self) -> Option<char> {
        if self.chars().count() != 1{
            return None;
        }

        return self.chars().next();
    }
}