/// Extension trait for `Option<T>`.
pub trait OptionExt<T> {
    /// Checks if the `Option` contains the specified value.
    fn contains_some<U>(&self, x: &U) -> bool
    where
        U: PartialEq<T>;
}

/// Extension trait for `Option` for strings operations.
pub trait OptionStrExt {
    /// Checks if the `Option` contains the specified `&str`.
    fn contains_str(&self, value: &str) -> bool;
}

impl<T> OptionExt<T> for Option<T> {
    #[inline]
    fn contains_some<U>(&self, x: &U) -> bool
    where
        U: PartialEq<T>,
    {
        // Copied from `option.rs`.
        match self {
            Some(y) => x == y,
            None => false,
        }
    }
}

impl OptionStrExt for Option<String> {
    #[inline]
    fn contains_str(&self, value: &str) -> bool {
        match self {
            Some(s) => s == value,
            None => false,
        }
    }
}

impl<'a> OptionStrExt for Option<&'a String> {
    #[inline]
    fn contains_str(&self, value: &str) -> bool {
        match self {
            Some(s) => *s == value,
            None => false,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn contains_some_test() {
        let x = Some(10_i32);

        assert!(x.contains_some(&10_i32));
        assert!(!x.contains_some(&15_i32));
    }

    #[test]
    fn contains_str_test() {
        let x = Some(String::from("Hello"));

        assert!(x.contains_str("Hello"));
        assert!(!x.contains_str("hello"));
    }

    #[test]
    fn contains_str_ref_test() {
        let x = Some(String::from("Hello"));

        assert!(x.contains_str("Hello"));
        assert!(!x.contains_str("hello"));
    }
}
