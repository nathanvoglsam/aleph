//
//
// This file is a part of Aleph
//
// https://github.com/nathanvoglsam/aleph
//
// MIT License
//
// Copyright (c) 2020 Aleph Engine
//
// Permission is hereby granted, free of charge, to any person obtaining a copy
// of this software and associated documentation files (the "Software"), to deal
// in the Software without restriction, including without limitation the rights
// to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
// copies of the Software, and to permit persons to whom the Software is
// furnished to do so, subject to the following conditions:
//
// The above copyright notice and this permission notice shall be included in all
// copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
// AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
// LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
// OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
// SOFTWARE.
//

use std::fmt::Display;

use any::*;
use smallbox::SmallBox;
use smallbox::space::S1;

///
/// This interface represents the API expected of something that gives the engine access to a
/// device's keyboard.
///
pub trait IKeyboard: IAny + Send + Sync + 'static {
    ///
    /// Get the current state of the mouse, last updated at the beginning of the frame
    ///
    /// # Warning
    ///
    /// This will likely lock an RwLock so trying to hold on to this between frames will deadlock
    /// the engine.
    ///
    fn get_state<'a>(&'a self) -> SmallBox<dyn IKeyboardStateLock + 'a, S1>;

    ///
    /// Get read only access to this frame's list of mouse events.
    ///
    /// # Warning
    ///
    /// This will likely lock an RwLock so trying to hold on to this between frames will deadlock
    /// the engine.
    ///
    fn events<'a>(&'a self) -> SmallBox<dyn IKeyboardEventsLock + 'a, S1>;
}

///
/// This interface is used to provide access to the list of keyboard events for the current frame.
///
/// Some implementations may need to lock a mutex or read/write lock to provide access to the list
/// safely so this interface is passed to wrap the lock guard
///
pub trait IKeyboardEventsLock {
    fn events(&self) -> &[KeyboardEvent];
}

///
/// This interface is used to provide access to the list of keyboard events for the current frame.
///
/// Some implementations may need to lock a mutex or read/write lock to provide access to the list
/// safely so this interface is passed to wrap the lock guard
///
pub trait IKeyboardStateLock {
    /// Takes the given scan code and attempts to translate it into a key code.
    ///
    /// This value returning `None` should not be an expected response so shouldn't be treated like
    /// a standard error. Typical usage would be to just call `unwrap`.
    fn translate_scan_code(&self, scan_code: ScanCode) -> Option<KeyCode>;

    /// Takes the given key code and attempts to translate it into a scan code.
    ///
    /// This value returning `None` should not be an expected response so shouldn't be treated like
    /// a standard error. Typical usage would be to just call `unwrap`.
    fn translate_key_code(&self, key_code: KeyCode) -> Option<ScanCode>;

    /// Returns whether the provided key code is currently pressed down
    fn key_code_down(&self, key_code: KeyCode) -> bool;

    /// Returns whether the provided scan code is currently pressed down
    fn scan_code_down(&self, scan_code: ScanCode) -> bool;
}

#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub enum KeyCode {
    // Character keys
    A,
    B,
    C,
    D,
    E,
    F,
    G,
    H,
    I,
    J,
    K,
    L,
    M,
    N,
    O,
    P,
    Q,
    R,
    S,
    T,
    U,
    V,
    W,
    X,
    Y,
    Z,

    // Other main keys
    BackQuote,
    Tilde,
    LeftBrace,
    LeftBracket,
    RightBrace,
    RightBracket,
    BackSlash,
    VerticalBar,
    Semicolon,
    Colon,
    Apostrophe,
    Quote,
    Comma,
    LessThan,
    Period,
    GreaterThan,
    ForwardSlash,
    QuestionMark,

    Space,
    CapsLock,
    Tab,
    Enter,
    Escape,
    Backspace,

    // Function row
    F1,
    F2,
    F3,
    F4,
    F5,
    F6,
    F7,
    F8,
    F9,
    F10,
    F11,
    F12,
    F13,
    F14,
    F15,
    F16,
    F17,
    F18,
    F19,
    F20,
    F21,
    F22,
    F23,
    F24,

    // Number row numbers
    Num1,
    Num2,
    Num3,
    Num4,
    Num5,
    Num6,
    Num7,
    Num8,
    Num9,
    Num0,
    Minus,
    Equals,

    // Number row shifted
    ExclamationMark,
    At,
    Pound,
    Hash,
    Dollar,
    Percent,
    Caret,
    Ampersand,
    Asterisk,
    LeftParenthesis,
    RightParenthesis,
    Underscore,
    Plus,

    // Control block
    PrintScreen,
    ScrollLock,
    Pause,
    Insert,
    Home,
    PageUp,
    Delete,
    End,
    PageDown,

    // Arrow keys
    ArrowUp,
    ArrowDown,
    ArrowLeft,
    ArrowRight,

    // Numpad
    NumLock,
    PadForwardSlash,
    PadAsterisk,
    PadMinus,
    PadPlus,
    PadEnter,
    PadPeriod,
    PadNum1,
    PadNum2,
    PadNum3,
    PadNum4,
    PadNum5,
    PadNum6,
    PadNum7,
    PadNum8,
    PadNum9,
    PadNum0,

    // Control keys
    LeftCtrl,
    LeftAlt,
    LeftShift,
    LeftMeta,
    RightCtrl,
    RightAlt,
    RightShift,
    RightMeta,
    Application,
}

impl KeyCode {
    pub fn name(&self) -> &'static str {
        match self {
            KeyCode::A => "A",
            KeyCode::B => "B",
            KeyCode::C => "C",
            KeyCode::D => "D",
            KeyCode::E => "E",
            KeyCode::F => "F",
            KeyCode::G => "G",
            KeyCode::H => "H",
            KeyCode::I => "I",
            KeyCode::J => "J",
            KeyCode::K => "K",
            KeyCode::L => "L",
            KeyCode::M => "M",
            KeyCode::N => "N",
            KeyCode::O => "O",
            KeyCode::P => "P",
            KeyCode::Q => "Q",
            KeyCode::R => "R",
            KeyCode::S => "S",
            KeyCode::T => "T",
            KeyCode::U => "U",
            KeyCode::V => "V",
            KeyCode::W => "W",
            KeyCode::X => "X",
            KeyCode::Y => "Y",
            KeyCode::Z => "Z",
            KeyCode::Tilde => "~",
            KeyCode::BackQuote => "`",
            KeyCode::LeftBrace => "[",
            KeyCode::LeftBracket => "{",
            KeyCode::RightBrace => "]",
            KeyCode::RightBracket => "}",
            KeyCode::BackSlash => "\\",
            KeyCode::VerticalBar => "|",
            KeyCode::Semicolon => ";",
            KeyCode::Colon => ":",
            KeyCode::Apostrophe => "'",
            KeyCode::Quote => "\"",
            KeyCode::Comma => ",",
            KeyCode::LessThan => "<",
            KeyCode::Period => ".",
            KeyCode::GreaterThan => ">",
            KeyCode::ForwardSlash => "/",
            KeyCode::QuestionMark => "?",
            KeyCode::Space => " ",
            KeyCode::CapsLock => "Caps Lock",
            KeyCode::Tab => "Tab",
            KeyCode::Enter => "Enter",
            KeyCode::Escape => "Esc",
            KeyCode::Backspace => "Backspace",
            KeyCode::F1 => "F1",
            KeyCode::F2 => "F2",
            KeyCode::F3 => "F3",
            KeyCode::F4 => "F4",
            KeyCode::F5 => "F5",
            KeyCode::F6 => "F6",
            KeyCode::F7 => "F7",
            KeyCode::F8 => "F8",
            KeyCode::F9 => "F9",
            KeyCode::F10 => "F10",
            KeyCode::F11 => "F11",
            KeyCode::F12 => "F12",
            KeyCode::F13 => "F13",
            KeyCode::F14 => "F14",
            KeyCode::F15 => "F15",
            KeyCode::F16 => "F16",
            KeyCode::F17 => "F17",
            KeyCode::F18 => "F18",
            KeyCode::F19 => "F19",
            KeyCode::F20 => "F20",
            KeyCode::F21 => "F21",
            KeyCode::F22 => "F22",
            KeyCode::F23 => "F23",
            KeyCode::F24 => "F24",
            KeyCode::Num1 => "1",
            KeyCode::Num2 => "2",
            KeyCode::Num3 => "3",
            KeyCode::Num4 => "4",
            KeyCode::Num5 => "5",
            KeyCode::Num6 => "6",
            KeyCode::Num7 => "7",
            KeyCode::Num8 => "8",
            KeyCode::Num9 => "9",
            KeyCode::Num0 => "0",
            KeyCode::Minus => "-",
            KeyCode::Equals => "=",
            KeyCode::ExclamationMark => "!",
            KeyCode::At => "@",
            KeyCode::Pound => "£",
            KeyCode::Hash => "#",
            KeyCode::Dollar => "$",
            KeyCode::Percent => "%",
            KeyCode::Caret => "^",
            KeyCode::Ampersand => "&",
            KeyCode::Asterisk => "*",
            KeyCode::LeftParenthesis => "(",
            KeyCode::RightParenthesis => ")",
            KeyCode::Underscore => "_",
            KeyCode::Plus => "+",
            KeyCode::PrintScreen => "Print Screen",
            KeyCode::ScrollLock => "Scroll Lock",
            KeyCode::Pause => "Pause",
            KeyCode::Insert => "Insert",
            KeyCode::Home => "Home",
            KeyCode::PageUp => "Page Up",
            KeyCode::Delete => "Delete",
            KeyCode::End => "End",
            KeyCode::PageDown => "Page Down",
            KeyCode::ArrowUp => "Up",
            KeyCode::ArrowDown => "Down",
            KeyCode::ArrowLeft => "Left",
            KeyCode::ArrowRight => "Right",
            KeyCode::NumLock => "Num Lock",
            KeyCode::PadForwardSlash => "Pad /",
            KeyCode::PadAsterisk => "Pad *",
            KeyCode::PadMinus => "Pad -",
            KeyCode::PadPlus => "Pad +",
            KeyCode::PadEnter => "Pad Enter",
            KeyCode::PadPeriod => "Pad .",
            KeyCode::PadNum1 => "Pad 1",
            KeyCode::PadNum2 => "Pad 2",
            KeyCode::PadNum3 => "Pad 3",
            KeyCode::PadNum4 => "Pad 4",
            KeyCode::PadNum5 => "Pad 5",
            KeyCode::PadNum6 => "Pad 6",
            KeyCode::PadNum7 => "Pad 7",
            KeyCode::PadNum8 => "Pad 8",
            KeyCode::PadNum9 => "Pad 9",
            KeyCode::PadNum0 => "Pad 0",
            KeyCode::LeftCtrl => "Left Control",
            KeyCode::LeftAlt => "Left Alt",
            KeyCode::LeftShift => "Left Shift",
            KeyCode::LeftMeta => "Left Meta",
            KeyCode::RightCtrl => "Right Control",
            KeyCode::RightAlt => "Right Alt",
            KeyCode::RightShift => "Right Shift",
            KeyCode::RightMeta => "Right Meta",
            KeyCode::Application => "Application",
        }
    }
}

impl Display for KeyCode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.name())
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
#[repr(u32)]
pub enum ScanCode {
    Unknown = 0,
    A = 4,
    B = 5,
    C = 6,
    D = 7,
    E = 8,
    F = 9,
    G = 10,
    H = 11,
    I = 12,
    J = 13,
    K = 14,
    L = 15,
    M = 16,
    N = 17,
    O = 18,
    P = 19,
    Q = 20,
    R = 21,
    S = 22,
    T = 23,
    U = 24,
    V = 25,
    W = 26,
    X = 27,
    Y = 28,
    Z = 29,
    Num1 = 30,
    Num2 = 31,
    Num3 = 32,
    Num4 = 33,
    Num5 = 34,
    Num6 = 35,
    Num7 = 36,
    Num8 = 37,
    Num9 = 38,
    Num0 = 39,
    Return = 40,
    Escape = 41,
    Backspace = 42,
    Tab = 43,
    Space = 44,
    Minus = 45,
    Equals = 46,
    LeftBracket = 47,
    RightBracket = 48,
    Backslash = 49,
    NonUsHash = 50,
    Semicolon = 51,
    Apostrophe = 52,
    Grave = 53,
    Comma = 54,
    Period = 55,
    Slash = 56,
    CapsLock = 57,
    F1 = 58,
    F2 = 59,
    F3 = 60,
    F4 = 61,
    F5 = 62,
    F6 = 63,
    F7 = 64,
    F8 = 65,
    F9 = 66,
    F10 = 67,
    F11 = 68,
    F12 = 69,
    PrintScreen = 70,
    ScrollLock = 71,
    Pause = 72,
    Insert = 73,
    Home = 74,
    PageUp = 75,
    Delete = 76,
    End = 77,
    PageDown = 78,
    Right = 79,
    Left = 80,
    Down = 81,
    Up = 82,
    NumLockClear = 83,
    KpDivide = 84,
    KpMultiply = 85,
    KpMinus = 86,
    KpPlus = 87,
    KpEnter = 88,
    Kp1 = 89,
    Kp2 = 90,
    Kp3 = 91,
    Kp4 = 92,
    Kp5 = 93,
    Kp6 = 94,
    Kp7 = 95,
    Kp8 = 96,
    Kp9 = 97,
    Kp0 = 98,
    KpPeriod = 99,
    NonUsBackslash = 100,
    Application = 101,
    Power = 102,
    KpEquals = 103,
    F13 = 104,
    F14 = 105,
    F15 = 106,
    F16 = 107,
    F17 = 108,
    F18 = 109,
    F19 = 110,
    F20 = 111,
    F21 = 112,
    F22 = 113,
    F23 = 114,
    F24 = 115,
    Execute = 116,
    Help = 117,
    Menu = 118,
    Select = 119,
    Stop = 120,
    Again = 121,
    Undo = 122,
    Cut = 123,
    Copy = 124,
    Paste = 125,
    Find = 126,
    Mute = 127,
    VolumeUp = 128,
    VolumeDown = 129,
    KpComma = 133,
    KpEqualsAs400 = 134,
    International1 = 135,
    International2 = 136,
    International3 = 137,
    International4 = 138,
    International5 = 139,
    International6 = 140,
    International7 = 141,
    International8 = 142,
    International9 = 143,
    Lang1 = 144,
    /**< Hangul/English toggle */
    Lang2 = 145,
    /**< Hanja conversion */
    Lang3 = 146,
    /**< Katakana */
    Lang4 = 147,
    /**< Hiragana */
    Lang5 = 148,
    /**< Zenkaku/Hankaku */
    Lang6 = 149,
    /**< reserved */
    Lang7 = 150,
    /**< reserved */
    Lang8 = 151,
    /**< reserved */
    Lang9 = 152,
    /**< reserved */
    AltErase = 153,
    SysReq = 154,
    Cancel = 155,
    Clear = 156,
    Prior = 157,
    Return2 = 158,
    Separator = 159,
    Out = 160,
    Oper = 161,
    ClearAgain = 162,
    CrSel = 163,
    ExSel = 164,
    Kp00 = 176,
    Kp000 = 177,
    ThousandsSeparator = 178,
    DecimalSeparator = 179,
    CurrencyUnit = 180,
    CurrencySubunit = 181,
    KpLeftParen = 182,
    KpRightParen = 183,
    KpLeftBrace = 184,
    KpRightBrace = 185,
    KpTab = 186,
    KpBackspace = 187,
    KpA = 188,
    KpB = 189,
    KpC = 190,
    KpD = 191,
    KpE = 192,
    KpF = 193,
    KpXor = 194,
    KpPower = 195,
    KpPercent = 196,
    KpLess = 197,
    KpGreater = 198,
    KpAmpersand = 199,
    KpDblAmpersand = 200,
    KpVerticalBar = 201,
    KpDblVerticalBar = 202,
    KpColon = 203,
    KpHash = 204,
    KpSpace = 205,
    KpAt = 206,
    KpExclam = 207,
    KpMemStore = 208,
    KpMemRecall = 209,
    KpMemClear = 210,
    KpMemAdd = 211,
    KpMemSubtract = 212,
    KpMemMultiply = 213,
    KpMemDivide = 214,
    KpPlusMinus = 215,
    KpClear = 216,
    KpClearEntry = 217,
    KpBinary = 218,
    KpOctal = 219,
    KpDecimal = 220,
    KpHexadecimal = 221,
    LCtrl = 224,
    LShift = 225,
    LAlt = 226,
    LGui = 227,
    RCtrl = 228,
    RShift = 229,
    RAlt = 230,
    RGui = 231,
    Mode = 257,
    Sleep = 258,
    Wake = 259,
    ChannelIncrement = 260,
    ChannelDecrement = 261,
    MediaPlay = 262,
    MediaPause = 263,
    MediaRecord = 264,
    MediaFastForward = 265,
    MediaRewind = 266,
    MediaNextTrack = 267,
    MediaPreviousTrack = 268,
    MediaStop = 269,
    MediaEject = 270,
    MediaPlayPause = 271,
    MediaSelect = 272,
    AcNew = 273,
    AcOpen = 274,
    AcClose = 275,
    AcExit = 276,
    AcSave = 277,
    AcPrint = 278,
    AcProperties = 279,
    AcSearch = 280,
    AcHome = 281,
    AcBack = 282,
    AcForward = 283,
    AcStop = 284,
    AcRefresh = 285,
    AcBookmarks = 286,
    SoftLeft = 287,
    SoftRight = 288,
    Call = 289,
    EndCall = 290,
    Reserved = 400,
    Count = 512,
}

impl ScanCode {
    /// The maximum number of supported scancode values
    pub const MAX_VALUES: usize = 512;
}

#[repr(transparent)]
#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Debug)]
pub struct KeyMod(pub u16);

impl KeyMod {
    pub fn contains(&self, other: KeyMod) -> bool {
        (self.0 & other.0) == other.0
    }
}

impl KeyMod {
    pub const NONE: Self = Self(0x0000);
    pub const L_SHIFT: Self = Self(0x0001);
    pub const R_SHIFT: Self = Self(0x0002);
    pub const L_CTRL: Self = Self(0x0040);
    pub const R_CTRL: Self = Self(0x0080);
    pub const L_ALT: Self = Self(0x0100);
    pub const R_ALT: Self = Self(0x0200);
    pub const L_GUI: Self = Self(0x0400);
    pub const R_GUI: Self = Self(0x0800);
    pub const NUM: Self = Self(0x1000);
    pub const CAPS: Self = Self(0x2000);
    pub const MODE: Self = Self(0x4000);
    pub const RESERVED: Self = Self(0x8000);
}

impl std::ops::BitOr for KeyMod {
    type Output = Self;

    fn bitor(self, rhs: Self) -> Self::Output {
        Self(self.0 | rhs.0)
    }
}

impl std::ops::BitOrAssign for KeyMod {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0 |= rhs.0
    }
}

impl std::ops::BitAnd for KeyMod {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self::Output {
        Self(self.0 & rhs.0)
    }
}

impl std::ops::BitAndAssign for KeyMod {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0 &= rhs.0
    }
}

impl std::ops::BitXor for KeyMod {
    type Output = Self;

    fn bitxor(self, rhs: Self) -> Self::Output {
        Self(self.0 ^ rhs.0)
    }
}

impl std::ops::BitXorAssign for KeyMod {
    fn bitxor_assign(&mut self, rhs: Self) {
        self.0 ^= rhs.0
    }
}

#[derive(Clone, Debug)]
pub struct KeyDownEvent {
    pub scan_code: ScanCode,
    pub modifiers: KeyMod,
    pub repeat: bool,
}

#[derive(Clone, Debug)]
pub struct KeyUpEvent {
    pub scan_code: ScanCode,
    pub modifiers: KeyMod,
    pub repeat: bool,
}

#[derive(Clone, Debug)]
pub struct TextInputEvent {
    pub text: String,
}

///
/// A mouse event
///
#[derive(Clone, Debug)]
pub enum KeyboardEvent {
    KeyDown(KeyDownEvent),
    KeyUp(KeyUpEvent),
    TextInput(TextInputEvent),
}
