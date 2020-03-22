use std::convert::TryFrom;
use std::fmt::Display;

use win32console::console::WinConsole;
use win32console::input::InputRecord::KeyEvent;

use crate::console::console_color::ConsoleColor;
use crate::console::console_key::ConsoleKey;
use crate::console::keycode::KeyCode;
use crate::console::keycode::KeyModifierState;
use crate::console::Terminal;

type WinConsoleColor = win32console::structs::console_color::ConsoleColor;

pub struct WindowsTerminal;

impl Terminal for WindowsTerminal {
    fn read() -> ConsoleKey {
        while let KeyEvent(key_event) = WinConsole::input().read_single_input().unwrap() {
            if !key_event.key_down{
                continue;
            }

            let char_value = key_event.u_char;

            let modifier = KeyModifierState::from(key_event.control_key_state);
            let key_code = KeyCode::from_event(key_event);

            return ConsoleKey::new(char_value, key_code, modifier);
        }

        unreachable!()
    }

    #[inline]
    fn write<T: Display>(value: T) {
        let s = value.to_string();
        WinConsole::output().write_utf8(s.as_bytes()).unwrap();
    }

    #[inline]
    fn read_with_color(color: ConsoleColor) -> ConsoleKey {
        with_color(color, Self::read)
    }

    #[inline]
    fn write_with_color<T: Display>(value: T, color: ConsoleColor) {
        with_color(color, || Self::write(value))
    }

    #[inline]
    fn read_string() -> String {
        WinConsole::output().read_string().unwrap()
    }

    #[inline]
    fn read_string_with_color(color: ConsoleColor) -> String {
        with_color(color, Self::read_string)
    }
}

fn with_color<T, F: FnOnce() -> T>(color: ConsoleColor, func: F) -> T {
    let fg = WinConsole::output().get_foreground_color().unwrap();
    let bg = WinConsole::output().get_background_color().unwrap();

    color.foreground().map(|c| {
        let ascii_color = WinConsoleColor::try_from(c as u16).ok().unwrap();
        WinConsole::output().set_foreground_color(ascii_color).unwrap();
    });

    color.background().map(|c| {
        let ascii_color = WinConsoleColor::try_from(c as u16).ok().unwrap();
        WinConsole::output().set_background_color(ascii_color).unwrap();
    });

    let ret = func();

    WinConsole::output().set_foreground_color(fg).unwrap();
    WinConsole::output().set_background_color(bg).unwrap();

    return ret;
}