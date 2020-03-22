use win32console::structs::input_event::{ControlKeyState, KeyEventRecord};

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum KeyCode {
    Backspace,
    Tap,
    Clear,
    Enter,
    Shift,
    Control,
    Alt,
    Pause,
    CapsLock,
    Escape,
    Space,
    PageUp,
    PageDown,
    End,
    Home,
    Arrow(Direction),
    Select,
    Print,
    Execute,
    PrintScreen,
    Insert,
    Delete,
    Help,
    Char(char),
    LeftWindows,
    RightWindows,
    Apps,
    Sleep,
    NumPad(u8),
    Multiply,
    Add,
    Separator,
    Subtract,
    Decimal,
    Divide,
    F(u8),
    NumLock,
    ScrollLock,
    LeftShift,
    RightShift,
    LeftControl,
    RightControl,
    LeftMenu,
    RightMenu,
    BrowserBack,
    BrowserForward,
    BrowserRefresh,
    BrowserStop,
    BrowserSearch,
    BrowserFavorites,
    BrowserHome,
    VolumeMute,
    VolumeDown,
    VolumeUp,
    MediaNext,
    MediaPrev,
    MediaStop,
    MediaPlayPause,
    LaunchMail,
    MediaSelect,
    LaunchApp1,
    LaunchApp2,
    Ome1,
    OmePlus,
    OmeComma,
    OmeMinus,
    OmePeriod,
    Ome2,
    Ome3,
    Ome4,
    Ome5,
    Ome6,
    Ome7,
    Ome8,
    Ome102,
    Process,
    Packet,
    Attention,
    CrSel,
    ExSel,
    EraseEndOfFile,
    Play,
    Zoom,
    NoName,
    PA1,
    OmeClear,
    Undefined,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct KeyModifierState(u32);

impl KeyCode {
    pub fn from_event(event: KeyEventRecord) -> KeyCode {
        match event.virtual_key_code {
            0x08 => KeyCode::Backspace,
            0x09 => KeyCode::Tap,
            0x0C => KeyCode::Clear,
            0x0D => KeyCode::Enter,
            0x10 => KeyCode::Shift,
            0x11 => KeyCode::Control,
            0x12 => KeyCode::Alt,
            0x13 => KeyCode::Pause,
            0x14 => KeyCode::CapsLock,
            0x1B => KeyCode::Escape,
            0x20 => KeyCode::Space,
            0x21 => KeyCode::PageUp,
            0x22 => KeyCode::PageDown,
            0x23 => KeyCode::End,
            0x24 => KeyCode::Home,
            0x25 => KeyCode::Arrow(Direction::Left),
            0x26 => KeyCode::Arrow(Direction::Up),
            0x27 => KeyCode::Arrow(Direction::Right),
            0x28 => KeyCode::Arrow(Direction::Down),
            0x29 => KeyCode::Select,
            0x2A => KeyCode::Print,
            0x2B => KeyCode::Execute,
            0x2C => KeyCode::PrintScreen,
            0x2D => KeyCode::Insert,
            0x2E => KeyCode::Delete,
            0x2F => KeyCode::Help,
            0x30..=0x39 => KeyCode::Char(event.u_char),
            0x41..=0x5A => KeyCode::Char(event.u_char),
            0x5B => KeyCode::LeftWindows,
            0x5C => KeyCode::RightWindows,
            0x5D => KeyCode::Apps,
            0x5F => KeyCode::Sleep,
            0x60..=0x69 => KeyCode::NumPad((event.virtual_key_code - 0x60) as u8),
            0x6A => KeyCode::Multiply,
            0x6C => KeyCode::Separator,
            0x6D => KeyCode::Subtract,
            0x6E => KeyCode::Decimal,
            0x6F => KeyCode::Divide,
            0x70..=0x87 => KeyCode::F((event.virtual_key_code - 0x69) as u8),
            0x90 => KeyCode::NumLock,
            0x91 => KeyCode::ScrollLock,
            0xA0 => KeyCode::LeftShift,
            0xA1 => KeyCode::RightShift,
            0xA2 => KeyCode::LeftControl,
            0xA3 => KeyCode::RightControl,
            0xA4 => KeyCode::LeftMenu, //Alt?
            0xA5 => KeyCode::RightMenu,
            0xA6 => KeyCode::BrowserBack,
            0xA7 => KeyCode::BrowserForward,
            0xA8 => KeyCode::BrowserRefresh,
            0xA9 => KeyCode::BrowserStop,
            0xAA => KeyCode::BrowserSearch,
            0xAB => KeyCode::BrowserFavorites,
            0xAC => KeyCode::BrowserHome,
            0xAD => KeyCode::VolumeMute,
            0xAE => KeyCode::VolumeDown,
            0xAF => KeyCode::VolumeUp,
            0xB0 => KeyCode::MediaNext,
            0xB1 => KeyCode::MediaPrev,
            0xB2 => KeyCode::MediaStop,
            0xB3 => KeyCode::MediaPlayPause,
            0xB4 => KeyCode::LaunchMail,
            0xB5 => KeyCode::MediaSelect,
            0xB6 => KeyCode::LaunchApp1,
            0xB7 => KeyCode::LaunchApp2,
            0xBA => KeyCode::Ome1,
            0xBB => KeyCode::OmePlus,
            0xBC => KeyCode::OmeComma,
            0xBD => KeyCode::OmeMinus,
            0xBE => KeyCode::OmePeriod,
            0xBF => KeyCode::Ome2,
            0xC0 => KeyCode::Ome3,
            0xDB => KeyCode::Ome4,
            0xDC => KeyCode::Ome5,
            0xDD => KeyCode::Ome6,
            0xDE => KeyCode::Ome7,
            0xDF => KeyCode::Ome8,
            0xE2 => KeyCode::Ome102,
            0xE5 => KeyCode::Process,
            0xE7 => KeyCode::Packet,
            0xF6 => KeyCode::Attention,
            0xF7 => KeyCode::CrSel,
            0xF8 => KeyCode::ExSel,
            0xF9 => KeyCode::EraseEndOfFile,
            0xFA => KeyCode::Play,
            0xFB => KeyCode::Zoom,
            0xFC => KeyCode::NoName,
            0xFD => KeyCode::PA1,
            0xFE => KeyCode::OmeClear,
            _ => KeyCode::Undefined,
        }
    }
}

impl KeyModifierState {
    pub const RIGHT_ALT_PRESSED: u32 = 0x0001;
    pub const LEFT_ALT_PRESSED: u32 = 0x0002;
    pub const RIGHT_CTRL_PRESSED: u32 = 0x0004;
    pub const LEFT_CTRL_PRESSED: u32 = 0x0008;
    pub const SHIFT_PRESSED: u32 = 0x0010;
    pub const NUM_LOCK_ON: u32 = 0x0020;
    pub const SCROLL_LOCK_ON: u32 = 0x0040;
    pub const CAPS_LOCK_ON: u32 = 0x0080;
    pub const ENHANCED_KEY: u32 = 0x0100;

    #[inline]
    pub fn new(state: u32) -> Self{
        KeyModifierState(state)
    }

    #[inline]
    pub fn has_state(&self, state: u32) -> bool {
        (self.0 & state) != 0
    }

    #[inline]
    pub fn has_alt(&self) -> bool {
        self.has_state(KeyModifierState::LEFT_ALT_PRESSED) || self.has_state(KeyModifierState::RIGHT_ALT_PRESSED)
    }

    #[inline]
    pub fn has_ctrl(&self) -> bool {
        self.has_state(KeyModifierState::RIGHT_CTRL_PRESSED) || self.has_state(KeyModifierState::LEFT_CTRL_PRESSED)
    }

    #[inline]
    pub fn has_shift(&self) -> bool {
        self.has_state(KeyModifierState::SHIFT_PRESSED)
    }
}

impl From<ControlKeyState> for KeyModifierState{
    #[inline]
    fn from(state: ControlKeyState) -> Self {
        KeyModifierState(state.get_state())
    }
}