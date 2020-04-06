use std::cmp::Ordering;
use std::iter::FromIterator;
use std::vec::IntoIter;

/// Extensions methods for `Iterator`
pub trait IteratorExt: Iterator {
    /// Sorts the elements of the iterator.
    ///
    /// # Remarks
    /// This method is O(n) in space, the elements of the iterator are store in a `Vec`
    /// and then sorted.
    fn sorted(self) -> IntoIter<Self::Item>
    where
        Self: Sized,
        Self::Item: Ord,
    {
        let mut vec = Vec::from_iter(self.into_iter());
        vec.sort();
        vec.into_iter()
    }

    /// Sorts the elements of the iterator using the specified function.
    ///
    /// # Remarks
    /// This method is O(n) in space, the elements of the iterator are store in a `Vec`
    /// and then sorted.
    fn sorted_by<F>(self, compare: F) -> IntoIter<Self::Item>
    where
        Self: Sized,
        F: FnMut(&Self::Item, &Self::Item) -> Ordering,
    {
        let mut vec = Vec::from_iter(self.into_iter());
        vec.sort_by(compare);
        vec.into_iter()
    }

    /// Sorts the elements of the iterator by the specified key.
    ///
    /// # Remarks
    /// This method is O(n) in space, the elements of the iterator are store in a `Vec`
    /// and then sorted.
    fn sorted_by_key<F, R>(self, f: F) -> IntoIter<Self::Item>
    where
        Self: Sized,
        R: Ord,
        F: FnMut(&Self::Item) -> R,
    {
        let mut vec = Vec::from_iter(self.into_iter());
        vec.sort_by_key(f);
        vec.into_iter()
    }
}

impl<I: Iterator> IteratorExt for I{}

#[cfg(test)]
mod tests{
    use super::*;

    #[test]
    fn sorted_test(){
        let items = [4, 1, 3, 2];
        let sorted = items.iter()
            .copied()
            .sorted()
            .collect::<Vec<i32>>();

        assert_eq!(&[1_i32, 2_i32, 3_i32, 4_i32], sorted.as_slice());
    }

    #[test]
    fn sorted_by_test(){
        let items = [4, 1, 3, 2];
        let sorted = items.iter()
            .copied()
            .sorted_by(|a, b| a.cmp(b))
            .collect::<Vec<i32>>();

        assert_eq!(&[1_i32, 2_i32, 3_i32, 4_i32], sorted.as_slice());
    }

    #[test]
    fn sorted_by_key_test(){
        let items = ["4a", "2b", "1c", "3d"];
        let sorted = items.iter()
            .copied()
            .sorted_by_key(|c| c.chars().nth(0).map(|c| c as i32).unwrap())
            .collect::<Vec<&str>>();

        assert_eq!(&["1c", "2b", "3d", "4a"], sorted.as_slice());
    }
}
