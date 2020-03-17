use std::fmt::Display;

pub mod console_color;
pub mod console_key;
pub mod keycode;

#[cfg(windows)]
mod windows;

#[cfg(windows)]
use windows::WindowsTerminal as Console;
use crate::console::console_key::ConsoleKey;
use crate::console::console_color::ConsoleColor;

/// Reads a key press from the console.
#[inline]
pub fn read() -> ConsoleKey{
    Console::read()
}

/// Writes a value to the console.
#[inline]
pub fn write<T: Display>(value: T){
    Console::write(value)
}

/// Reads a key press from the console using the specified color.
#[inline]
pub fn read_with_color(color: ConsoleColor) -> ConsoleKey{
    Console::read_with_color(color)
}

/// Writes a value to the console with the specified color.
#[inline]
pub fn write_with_color<T: Display>(value: T, color: ConsoleColor){
    Console::write_with_color(value, color)
}

pub(crate) trait Terminal {
    fn read() -> ConsoleKey;
    fn write<T: Display>(value: T);
    fn read_with_color(color: ConsoleColor) -> ConsoleKey;
    fn write_with_color<T: Display>(value: T, color: ConsoleColor);
    fn read_string() -> String;
    fn read_string_with_color(color: ConsoleColor) -> String;
}