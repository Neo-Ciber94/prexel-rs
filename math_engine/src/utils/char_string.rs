use std::ops::{IndexMut, Index};

pub struct CharString(Vec<char>);

impl CharString {
    #[inline]
    pub fn new() -> CharString{
        CharString(Vec::new())
    }

    #[inline]
    pub fn with_capacity(capacity: usize) -> CharString{
        CharString(Vec::with_capacity(capacity))
    }

    #[inline]
    pub fn push(&mut self, value: char){
        self.0.push(value)
    }

    #[inline]
    pub fn push_str(&mut self, str: &str){
        for c in str.chars(){
            self.0.push(c)
        }
    }

    #[inline]
    pub fn pop(&mut self) -> Option<char>{
        self.0.pop()
    }

    #[inline]
    pub fn len(&self) -> usize{
        self.0.len()
    }
}

impl From<&str> for CharString{
    fn from(str: &str) -> Self {
        CharString(str.chars().collect::<Vec<char>>())
    }
}

impl From<String> for CharString{
    fn from(string: String) -> Self {
        CharString(string.chars().collect::<Vec<char>>())
    }
}

impl Index<usize> for CharString{
    type Output = char;

    fn index(&self, index: usize) -> &Self::Output {
        &self.0[index]
    }
}

impl IndexMut<usize> for CharString{
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.0[index]
    }
}