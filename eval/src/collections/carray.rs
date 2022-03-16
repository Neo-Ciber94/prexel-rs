use std::ops::{Deref, DerefMut, Index, IndexMut};
use std::slice::Iter;
use std::vec::IntoIter;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct CircularArray<T> {
    data: Vec<T>,
    cap: usize
}

#[allow(unused)]
impl<T> CircularArray<T> {
    pub fn new(capacity: usize) -> CircularArray<T> {
        CircularArray {
            data: Vec::with_capacity(capacity),
            cap: capacity
        }
    }

    pub fn push(&mut self, item: T) {
        if self.data.len() == self.cap {
            self.data.remove(0);
        }
        self.data.push(item);
    }

    pub fn pop(&mut self) -> Option<T> {
        self.data.pop()
    }

    pub fn remove(&mut self, index: usize) -> Option<T> {
        if index >= self.data.len() {
            return None;
        }

        Some(self.data.remove(index))
    }

    pub fn len(&self) -> usize {
        self.data.len()
    }

    pub fn capacity(&self) -> usize {
        self.cap
    }

    pub fn get(&self, index: usize) -> Option<&T> {
        self.data.get(index)
    }

    pub fn get_mut(&mut self, index: usize) -> Option<&mut T> {
        self.data.get_mut(index)
    }

    pub fn iter(&self) -> Iter<'_, T> {
        self.data.iter()
    }
}

impl<T> Index<usize> for CircularArray<T> {
    type Output = T;

    fn index(&self, index: usize) -> &T {
        self.data.index(index)
    }
}

impl<T> IndexMut<usize> for CircularArray<T> {
    fn index_mut(&mut self, index: usize) -> &mut T {
        self.data.index_mut(index)
    }
}

impl<T> IntoIterator for CircularArray<T> {
    type Item = T;
    type IntoIter = IntoIter<T>;

    fn into_iter(self) -> Self::IntoIter {
        self.data.into_iter()
    }
}

impl<T> Deref for CircularArray<T> {
    type Target = [T];

    fn deref(&self) -> &Self::Target {
        &self.data
    }
}

impl<T> DerefMut for CircularArray<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.data
    }
}