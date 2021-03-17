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

use any::*;

///
/// This interface should be used by plugins that wish to register themselves as the engine's
/// keyboard provider. Anything that implements this should correctly handle creating and destroying
/// whatever is needed to access the system's keyboard, and should be able to give out an
/// `AnyArc<IKeyboard>` to allow others to retrieve information about and manipulate the keyboard.
///
pub trait IKeyboardProvider: ISendSyncAny + 'static {
    ///
    /// Returns an `AnyArc` that holds an `IKeyboard` interface.
    ///
    /// This will always return the same `IKeyboard` instance as `IKeyboardProvider` only supports
    /// handling a single keyboard device.
    ///
    /// A return value of `None` should signal that the functionality is not supported.
    ///
    fn get_keyboard(&self) -> Option<AnyArc<dyn IKeyboard>>;
}

///
/// This interface represents the API expected of something that gives the engine access to a
/// device's keyboard.
///
pub trait IKeyboard: ISendSyncAny + 'static {
    ///
    /// Get the current state of the mouse, last updated at the beginning of the frame
    ///
    /// # Warning
    ///
    /// This will likely lock an RwLock so trying to hold on to this between frames will deadlock
    /// the engine.
    ///
    fn get_state<'a>(&'a self) -> Box<dyn IKeyboardStateLock + 'a>;

    ///
    /// Get read only access to this frame's list of mouse events.
    ///
    /// # Warning
    ///
    /// This will likely lock an RwLock so trying to hold on to this between frames will deadlock
    /// the engine.
    ///
    fn events<'a>(&'a self) -> Box<dyn IKeyboardEventsLock + 'a>;
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
    /// Returns whether the provided key code is currently pressed down
    fn key_code_down(&self, key_code: KeyCode) -> bool;

    /// Returns whether the provided scan code is currently pressed down
    fn scan_code_down(&self, scan_code: ScanCode) -> bool;
}

///
/// The set of supported keycodes
///
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
#[repr(u32)]
pub enum KeyCode {
    Return = 13,
    Escape = 27,
    Backspace = 8,
    Tab = 9,
    Space = 32,
    Exclaim = 33,
    QuoteDouble = 34,
    Hash = 35,
    Percent = 37,
    Dollar = 36,
    Ampersand = 38,
    Quote = 39,
    LeftParen = 40,
    RightParen = 41,
    Asterisk = 42,
    Plus = 43,
    Comma = 44,
    Minus = 45,
    Period = 46,
    Slash = 47,
    Num0 = 48,
    Num1 = 49,
    Num2 = 50,
    Num3 = 51,
    Num4 = 52,
    Num5 = 53,
    Num6 = 54,
    Num7 = 55,
    Num8 = 56,
    Num9 = 57,
    Colon = 58,
    SemiColon = 59,
    Less = 60,
    Equals = 61,
    Greater = 62,
    Question = 63,
    At = 64,
    LeftBracket = 91,
    BackSlash = 92,
    RightBracket = 93,
    Caret = 94,
    Underscore = 95,
    BackQuote = 96,
    A = 97,
    B = 98,
    C = 99,
    D = 100,
    E = 101,
    F = 102,
    G = 103,
    H = 104,
    I = 105,
    J = 106,
    K = 107,
    L = 108,
    M = 109,
    N = 110,
    O = 111,
    P = 112,
    Q = 113,
    R = 114,
    S = 115,
    T = 116,
    U = 117,
    V = 118,
    W = 119,
    X = 120,
    Y = 121,
    Z = 122,
    CapsLock = 1073741881,
    F1 = 1073741882,
    F2 = 1073741883,
    F3 = 1073741884,
    F4 = 1073741885,
    F5 = 1073741886,
    F6 = 1073741887,
    F7 = 1073741888,
    F8 = 1073741889,
    F9 = 1073741890,
    F10 = 1073741891,
    F11 = 1073741892,
    F12 = 1073741893,
    PrintScreen = 1073741894,
    ScrollLock = 1073741895,
    Pause = 1073741896,
    Insert = 1073741897,
    Home = 1073741898,
    PageUp = 1073741899,
    Delete = 127,
    End = 1073741901,
    PageDown = 1073741902,
    Right = 1073741903,
    Left = 1073741904,
    Down = 1073741905,
    Up = 1073741906,
    NumLockClear = 1073741907,
    KpDivide = 1073741908,
    KpMultiply = 1073741909,
    KpMinus = 1073741910,
    KpPlus = 1073741911,
    KpEnter = 1073741912,
    Kp1 = 1073741913,
    Kp2 = 1073741914,
    Kp3 = 1073741915,
    Kp4 = 1073741916,
    Kp5 = 1073741917,
    Kp6 = 1073741918,
    Kp7 = 1073741919,
    Kp8 = 1073741920,
    Kp9 = 1073741921,
    Kp0 = 1073741922,
    KpPeriod = 1073741923,
    Application = 1073741925,
    Power = 1073741926,
    KpEquals = 1073741927,
    F13 = 1073741928,
    F14 = 1073741929,
    F15 = 1073741930,
    F16 = 1073741931,
    F17 = 1073741932,
    F18 = 1073741933,
    F19 = 1073741934,
    F20 = 1073741935,
    F21 = 1073741936,
    F22 = 1073741937,
    F23 = 1073741938,
    F24 = 1073741939,
    Execute = 1073741940,
    Help = 1073741941,
    Menu = 1073741942,
    Select = 1073741943,
    Stop = 1073741944,
    Again = 1073741945,
    Undo = 1073741946,
    Cut = 1073741947,
    Copy = 1073741948,
    Paste = 1073741949,
    Find = 1073741950,
    Mute = 1073741951,
    VolumeUp = 1073741952,
    VolumeDown = 1073741953,
    KpComma = 1073741957,
    KpEqualsAS400 = 1073741958,
    AltErase = 1073741977,
    SysReq = 1073741978,
    Cancel = 1073741979,
    Clear = 1073741980,
    Prior = 1073741981,
    Return2 = 1073741982,
    Separator = 1073741983,
    Out = 1073741984,
    Oper = 1073741985,
    ClearAgain = 1073741986,
    CrSel = 1073741987,
    ExSel = 1073741988,
    Kp00 = 1073742000,
    Kp000 = 1073742001,
    ThousandsSeparator = 1073742002,
    DecimalSeparator = 1073742003,
    CurrencyUnit = 1073742004,
    CurrencySubUnit = 1073742005,
    KpLeftParen = 1073742006,
    KpRightParen = 1073742007,
    KpLeftBrace = 1073742008,
    KpRightBrace = 1073742009,
    KpTab = 1073742010,
    KpBackspace = 1073742011,
    KpA = 1073742012,
    KpB = 1073742013,
    KpC = 1073742014,
    KpD = 1073742015,
    KpE = 1073742016,
    KpF = 1073742017,
    KpXor = 1073742018,
    KpPower = 1073742019,
    KpPercent = 1073742020,
    KpLess = 1073742021,
    KpGreater = 1073742022,
    KpAmpersand = 1073742023,
    KpDoubleAmpersand = 1073742024,
    KpVerticalBar = 1073742025,
    KpDoubleVerticalBar = 1073742026,
    KpColon = 1073742027,
    KpHash = 1073742028,
    KpSpace = 1073742029,
    KpAt = 1073742030,
    KpExclamation = 1073742031,
    KpMemStore = 1073742032,
    KpMemRecall = 1073742033,
    KpMemClear = 1073742034,
    KpMemAdd = 1073742035,
    KpMemSubtract = 1073742036,
    KpMemMultiply = 1073742037,
    KpMemDivide = 1073742038,
    KpPlusMinus = 1073742039,
    KpClear = 1073742040,
    KpClearEntry = 1073742041,
    KpBinary = 1073742042,
    KpOctal = 1073742043,
    KpDecimal = 1073742044,
    KpHexadecimal = 1073742045,
    LCtrl = 1073742048,
    LShift = 1073742049,
    LAlt = 1073742050,
    LGui = 1073742051,
    RCtrl = 1073742052,
    RShift = 1073742053,
    RAlt = 1073742054,
    RGui = 1073742055,
    Mode = 1073742081,
    AudioNext = 1073742082,
    AudioPrev = 1073742083,
    AudioStop = 1073742084,
    AudioPlay = 1073742085,
    AudioMute = 1073742086,
    MediaSelect = 1073742087,
    WWW = 1073742088,
    Mail = 1073742089,
    Calculator = 1073742090,
    Computer = 1073742091,
    AcSearch = 1073742092,
    AcHome = 1073742093,
    AcBack = 1073742094,
    AcForward = 1073742095,
    AcStop = 1073742096,
    AcRefresh = 1073742097,
    AcBookmarks = 1073742098,
    BrightnessDown = 1073742099,
    BrightnessUp = 1073742100,
    DisplaySwitch = 1073742101,
    KeyboardIlluminationToggle = 1073742102,
    KeyboardIlluminationDown = 1073742103,
    KeyboardIlluminationUp = 1073742104,
    Eject = 1073742105,
    Sleep = 1073742106,
    App1 = 1073742107,
    App2 = 1073742108,
    AudioRewind = 1073742109,
    AudioFastForward = 1073742110,
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
    BackSlash = 49,
    NonUsSlash = 50,
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
    KpEqualsAS400 = 134,
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
    Lang2 = 145,
    Lang3 = 146,
    Lang4 = 147,
    Lang5 = 148,
    Lang6 = 149,
    Lang7 = 150,
    Lang8 = 151,
    Lang9 = 152,
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
    CurrencySubUnit = 181,
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
    KpDoubleAmpersand = 200,
    KpVerticalBar = 201,
    KpDoubleVerticalBar = 202,
    KpColon = 203,
    KpHash = 204,
    KpSpace = 205,
    KpAt = 206,
    KpExclamation = 207,
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
    AudioNext = 258,
    AudioPrev = 259,
    AudioStop = 260,
    AudioPlay = 261,
    AudioMute = 262,
    MediaSelect = 263,
    WWW = 264,
    Mail = 265,
    Calculator = 266,
    Computer = 267,
    AcSearch = 268,
    AcHome = 269,
    AcBack = 270,
    AcForward = 271,
    AcStop = 272,
    AcRefresh = 273,
    AcBookmarks = 274,
    BrightnessDown = 275,
    BrightnessUp = 276,
    DisplaySwitch = 277,
    KeyboardIlluminationToggle = 278,
    KeyboardIlluminationDown = 279,
    KeyboardIlluminationUp = 280,
    Eject = 281,
    Sleep = 282,
    App1 = 283,
    App2 = 284,
    AudioRewind = 285,
    AudioFastForward = 286,
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
    pub key_code: KeyCode,
    pub scan_code: ScanCode,
    pub key_mod: KeyMod,
    pub repeat: bool,
}

#[derive(Clone, Debug)]
pub struct KeyUpEvent {
    pub key_code: KeyCode,
    pub scan_code: ScanCode,
    pub key_mod: KeyMod,
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
