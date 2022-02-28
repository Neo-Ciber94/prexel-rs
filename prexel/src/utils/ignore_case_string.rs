use std::cmp::Ordering;
use std::fmt::{Debug, Display, Formatter};
use std::hash::{Hash, Hasher};
use std::ops::Deref;
use super::ignore_case_str::partial_cmp_by;

/// Represents `String` that will ignore case when comparing.
#[derive(Clone)]
pub struct IgnoreCaseString(String);

impl IgnoreCaseString {
    /// Creates a new `IgnoreCaseString`.
    #[inline]
    pub fn new(value: String) -> IgnoreCaseString {
        IgnoreCaseString(value)
    }

    /// Gets the inner value of this `IgnoreCaseString`.
    #[inline]
    pub fn into_inner(self) -> String {
        self.0
    }

    /// Gets this instance inner value as a `&str`.
    #[inline]
    pub fn as_str(&self) -> &str {
        self.0.as_str()
    }

    /// Gets a reference to this instance inner value.
    #[inline]
    pub fn get_mut(&mut self) -> &mut String {
        &mut self.0
    }
}

impl Deref for IgnoreCaseString {
    type Target = str;

    #[inline]
    fn deref(&self) -> &str {
        self.0.as_str()
    }
}

impl From<&str> for IgnoreCaseString {
    #[inline]
    fn from(value: &str) -> Self {
        IgnoreCaseString(value.to_string())
    }
}

impl Display for IgnoreCaseString {
    #[inline]
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.0.as_str())
    }
}

impl Debug for IgnoreCaseString {
    #[inline]
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.0.as_str())
    }
}

impl Eq for IgnoreCaseString {}

impl PartialEq for IgnoreCaseString {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        self.partial_cmp(other).unwrap() == Ordering::Equal
    }
}

impl Ord for IgnoreCaseString {
    #[inline]
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}

impl PartialOrd for IgnoreCaseString {
    #[inline]
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        partial_cmp_by(self.0.chars(), other.0.chars(), |a, b| {
            a.to_lowercase().partial_cmp(b.to_lowercase())
        })
    }
}

impl Hash for IgnoreCaseString {
    #[inline]
    fn hash<H: Hasher>(&self, state: &mut H) {
        for c in self.0.bytes() {
            c.to_ascii_lowercase().hash(state);
        }
    }
}

impl AsRef<IgnoreCaseString> for IgnoreCaseString {
    fn as_ref(&self) -> &IgnoreCaseString {
        self
    }
}

unsafe impl Send for IgnoreCaseString {}

unsafe impl Sync for IgnoreCaseString {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn eq_test() {
        let a = IgnoreCaseString::from("HeLLo WOrlD");
        let b = IgnoreCaseString::from("hello wOrld");
        assert_eq!(a, b);
    }

    #[test]
    fn cmp_test() {
        let a = IgnoreCaseString::from("HeLLo WOrlD");
        let b = IgnoreCaseString::from("hello wOrld");
        assert_eq!(a.cmp(&b), Ordering::Equal);
    }
}
