use std::cmp::Ordering;
use std::hash::{Hash, Hasher};
use std::fmt::{Display, Debug, Formatter};

/// Represents `String` that will ignore case when comparing.
#[derive(Clone)]
pub struct IgnoreCaseString(String);

impl IgnoreCaseString{
    /// Creates a new `IgnoreCaseString`.
    #[inline]
    pub fn new(value: String) -> IgnoreCaseString{
        IgnoreCaseString(value)
    }

    /// Gets the inner value of this `IgnoreCaseString`.
    #[inline]
    pub fn into_inner(self) -> String{
        self.0
    }

    /// Gets this instance value as `str`.
    #[inline]
    pub fn as_str(&self) -> &str{
        self.0.as_str()
    }

    /// Gets this instance `String` as mutable.
    #[inline]
    pub fn get_mut(&mut self) -> &mut String{
        &mut self.0
    }
}

impl From<&str> for IgnoreCaseString{
    #[inline]
    fn from(value: &str) -> Self {
        IgnoreCaseString(value.to_string())
    }
}

impl Display for IgnoreCaseString{
    #[inline]
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.0.as_str())
    }
}

impl Debug for IgnoreCaseString{
    #[inline]
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.0.as_str())
    }
}

impl Eq for IgnoreCaseString {}

impl PartialEq for IgnoreCaseString{
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        self.partial_cmp(other).unwrap() == Ordering::Equal
    }
}

impl Ord for IgnoreCaseString{
    #[inline]
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}

impl PartialOrd for IgnoreCaseString{
    #[inline]
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        partial_cmp_by(self.0.chars(), other.0.chars(), |a,b|{
            a.to_lowercase().partial_cmp(b.to_lowercase())
        })
    }
}

impl Hash for IgnoreCaseString{
    #[inline]
    fn hash<H: Hasher>(&self, state: &mut H) {
        for c in self.0.bytes(){
            c.to_ascii_lowercase().hash(state);
        }
    }
}

impl AsRef<IgnoreCaseString> for IgnoreCaseString{
    fn as_ref(&self) -> &IgnoreCaseString {
        self
    }
}

unsafe impl Send for IgnoreCaseString{}

unsafe impl Sync for IgnoreCaseString{}

// Copied from iterator.rs
fn partial_cmp_by<I, O, F>(mut iterator: I, other: O, mut partial_cmp: F) -> Option<Ordering>
    where
        I: Iterator + Sized,
        O: IntoIterator,
        F: FnMut(I::Item, O::Item) -> Option<Ordering>,
{
    let mut other = other.into_iter();

    loop {
        let x = match iterator.next() {
            None => if other.next().is_none() {
                return Some(Ordering::Equal)
            } else {
                return Some(Ordering::Less)
            },
            Some(val) => val,
        };

        let y = match other.next() {
            None => return Some(Ordering::Greater),
            Some(val) => val,
        };

        match partial_cmp(x, y) {
            Some(Ordering::Equal) => (),
            non_eq => return non_eq,
        }
    }
}

#[cfg(test)]
mod tests{
    use super::*;

    #[test]
    fn eq_test(){
        let a = IgnoreCaseString::from("HeLLo WOrlD");
        let b = IgnoreCaseString::from("hello wOrld");
        assert_eq!(a, b);
    }

    #[test]
    fn cmp_test(){
        let a = IgnoreCaseString::from("HeLLo WOrlD");
        let b = IgnoreCaseString::from("hello wOrld");
        assert_eq!(a.cmp(&b), Ordering::Equal);
    }
}