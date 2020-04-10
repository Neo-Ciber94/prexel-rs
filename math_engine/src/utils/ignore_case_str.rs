use std::cmp::Ordering;
use std::fmt::{Debug, Display, Formatter};
use std::hash::{Hash, Hasher};

/// Represents `str` that will ignore case when comparing.
#[derive(Copy, Clone)]
pub struct IgnoreCaseStr<'a>(&'a str);

impl<'a> IgnoreCaseStr<'a> {
    /// Creates a new `IgnoreCaseStr`.
    #[inline]
    pub fn new(value: &'a str) -> Self {
        IgnoreCaseStr(value)
    }

    /// Gets a reference to the inner value.
    #[inline]
    pub fn as_raw_str(&self) -> &'a str {
        self.0
    }

    /// Gets the inner `str` of this instance.
    #[inline]
    pub fn into_inner(self) -> &'a str {
        self.0
    }
}

impl<'a> From<&'a str> for IgnoreCaseStr<'a> {
    #[inline]
    fn from(value: &'a str) -> Self {
        IgnoreCaseStr(value)
    }
}

impl<'a> Display for IgnoreCaseStr<'a> {
    #[inline]
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.0)
    }
}

impl<'a> Debug for IgnoreCaseStr<'a> {
    #[inline]
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.0)
    }
}

impl<'a> Eq for IgnoreCaseStr<'a> {}

impl<'a> Ord for IgnoreCaseStr<'a> {
    #[inline]
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}

impl<'a> PartialEq for IgnoreCaseStr<'a> {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        self.partial_cmp(other).unwrap() == Ordering::Equal
    }
}

impl<'a> PartialOrd for IgnoreCaseStr<'a> {
    #[inline]
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        partial_cmp_by(self.0.chars(), other.0.chars(), |a, b| {
            a.to_lowercase().partial_cmp(b.to_lowercase())
        })
    }
}

impl<'a> Hash for IgnoreCaseStr<'a> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        for c in self.0.as_bytes() {
            c.to_ascii_lowercase().hash(state);
        }
    }
}

impl<'a> AsRef<IgnoreCaseStr<'a>> for IgnoreCaseStr<'a> {
    fn as_ref(&self) -> &IgnoreCaseStr<'a> {
        self
    }
}

unsafe impl<'a> Send for IgnoreCaseStr<'a> {}

unsafe impl<'a> Sync for IgnoreCaseStr<'a> {}

// Copied from iterator.rs
pub(crate) fn partial_cmp_by<I, O, F>(mut iterator: I, other: O, mut partial_cmp: F) -> Option<Ordering>
where
    I: Iterator + Sized,
    O: IntoIterator,
    F: FnMut(I::Item, O::Item) -> Option<Ordering>,
{
    let mut other = other.into_iter();

    loop {
        let x = match iterator.next() {
            None => {
                if other.next().is_none() {
                    return Some(Ordering::Equal);
                } else {
                    return Some(Ordering::Less);
                }
            }
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
mod tests {
    use super::*;

    #[test]
    fn eq_test() {
        let a = IgnoreCaseStr::new("HEllO wOrLd");
        let b = IgnoreCaseStr::new("hello world");

        assert!(a.eq(&b))
    }

    #[test]
    fn cmp_test() {
        let a = IgnoreCaseStr::new("hoLA MUNdo");
        let b = IgnoreCaseStr::new("hola Mundo");

        assert_eq!(a.cmp(&b), Ordering::Equal)
    }

    #[test]
    fn cmp_non_ascii_test() {
        let a = IgnoreCaseStr::new("ωΠΦΛδ");
        let b = IgnoreCaseStr::new("ΩπφλΔ");

        assert_eq!(a.cmp(&b), Ordering::Equal)
    }
}
