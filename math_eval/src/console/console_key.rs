use crate::console::keycode::{KeyCode, KeyModifierState};

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct ConsoleKey {
    char_value: char,
    key_code: KeyCode,
    modifier: KeyModifierState,
}

impl ConsoleKey {
    #[inline]
    pub fn new(char_value: char, key_code: KeyCode, modifier: KeyModifierState) -> Self{
        ConsoleKey {
            char_value,
            key_code,
            modifier
        }
    }

    #[inline]
    pub fn key_code(&self) -> &KeyCode {
        &self.key_code
    }

    #[inline]
    pub fn char_value(&self) -> char {
        self.char_value
    }

    #[inline]
    pub fn modifier(&self) -> KeyModifierState{
        self.modifier
    }
}