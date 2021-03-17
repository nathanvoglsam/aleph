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

use interfaces::any::{declare_interfaces, AnyArc};
use interfaces::platform::{
    Event, IKeyboard, IKeyboardEventsLock, IKeyboardStateLock, KeyCode, KeyDownEvent, KeyMod,
    KeyUpEvent, KeyboardEvent, ScanCode, TextInputEvent,
};
use parking_lot::{RwLock, RwLockReadGuard};

///
/// Represents the state of the keyboard this frame
///
pub struct KeyboardState {
    /// Array of boolean values that should be indexed with a `ScanCode` to see if that `ScanCode`
    /// is pressed.
    pub keys: [bool; ScanCode::MAX_VALUES],
}

///
/// The struct that provides the context for, and implements, `IKeyboard`
///
pub struct KeyboardImpl {
    /// The current state of the keyboard, as recorded at the beginning of a frame
    pub state: RwLock<KeyboardState>,

    /// The event list for the current frame
    pub events: RwLock<Vec<KeyboardEvent>>,
}

declare_interfaces!(KeyboardImpl, [IKeyboard]);

impl IKeyboard for KeyboardImpl {
    fn get_state<'a>(&'a self) -> Box<dyn IKeyboardStateLock + 'a> {
        let lock = KeyboardStateLockImpl(self.state.read());
        Box::new(lock)
    }

    fn events<'a>(&'a self) -> Box<dyn IKeyboardEventsLock + 'a> {
        let lock = KeyboardEventsLockImpl(self.events.read());
        Box::new(lock)
    }
}

impl KeyboardImpl {
    ///
    /// Internal function for initializing the keyboard state
    ///
    pub fn new() -> AnyArc<Self> {
        aleph_log::trace!("Initializing the Keyboard system");
        let keyboard_state = KeyboardState {
            keys: [false; ScanCode::MAX_VALUES],
        };

        let out = Self {
            state: RwLock::new(keyboard_state),
            events: RwLock::new(Vec::new()),
        };
        AnyArc::new(out)
    }

    ///
    /// Internal function for handling the events produced by the OS
    ///
    pub fn process_keyboard_event(
        &self,
        keyboard_events: &mut Vec<KeyboardEvent>,
        keyboard_state: &mut KeyboardState,
        all_events: &mut Vec<Event>,
        event: sdl2::event::Event,
    ) {
        match event {
            sdl2::event::Event::KeyDown {
                keycode,
                scancode,
                keymod,
                repeat,
                ..
            } => {
                let event = KeyDownEvent {
                    key_code: translate_key_code(keycode.unwrap()),
                    scan_code: translate_scan_code(scancode.unwrap()),
                    key_mod: translate_key_mod(keymod),
                    repeat,
                };
                keyboard_state.keys[event.scan_code as usize] = true;
                let event = KeyboardEvent::KeyDown(event);
                keyboard_events.push(event.clone());
                all_events.push(Event::KeyboardEvent(event.clone()));
            }
            sdl2::event::Event::KeyUp {
                keycode,
                scancode,
                keymod,
                repeat,
                ..
            } => {
                let event = KeyUpEvent {
                    key_code: translate_key_code(keycode.unwrap()),
                    scan_code: translate_scan_code(scancode.unwrap()),
                    key_mod: translate_key_mod(keymod),
                    repeat,
                };
                keyboard_state.keys[event.scan_code as usize] = false;
                let event = KeyboardEvent::KeyUp(event);
                keyboard_events.push(event.clone());
                all_events.push(Event::KeyboardEvent(event.clone()));
            }
            sdl2::event::Event::TextInput { text, .. } => {
                let event = TextInputEvent { text };
                let event = KeyboardEvent::TextInput(event);
                keyboard_events.push(event.clone());
                all_events.push(Event::KeyboardEvent(event.clone()));
            }
            _ => {}
        }
    }
}

///
/// Wrapper around RwLockReadGuard and implementation of `IKeyboardEventsLock`
///
pub struct KeyboardEventsLockImpl<'a>(pub RwLockReadGuard<'a, Vec<KeyboardEvent>>);

impl<'a> IKeyboardEventsLock for KeyboardEventsLockImpl<'a> {
    fn events(&self) -> &[KeyboardEvent] {
        self.0.as_slice()
    }
}

///
/// Wrapper around RwLockReadGuard and implementation of `IKeyboardStateLock`
///
pub struct KeyboardStateLockImpl<'a>(pub RwLockReadGuard<'a, KeyboardState>);

impl<'a> IKeyboardStateLock for KeyboardStateLockImpl<'a> {
    fn key_code_down(&self, _key_code: KeyCode) -> bool {
        false // TODO: implement me
    }

    fn scan_code_down(&self, scan_code: ScanCode) -> bool {
        self.0.keys[scan_code as usize]
    }
}

fn translate_key_code(key_code: sdl2::keyboard::Keycode) -> KeyCode {
    match key_code {
        sdl2::keyboard::Keycode::Return => KeyCode::Return,
        sdl2::keyboard::Keycode::Escape => KeyCode::Escape,
        sdl2::keyboard::Keycode::Backspace => KeyCode::Backspace,
        sdl2::keyboard::Keycode::Tab => KeyCode::Tab,
        sdl2::keyboard::Keycode::Space => KeyCode::Space,
        sdl2::keyboard::Keycode::Exclaim => KeyCode::Exclaim,
        sdl2::keyboard::Keycode::Quotedbl => KeyCode::QuoteDouble,
        sdl2::keyboard::Keycode::Hash => KeyCode::Hash,
        sdl2::keyboard::Keycode::Percent => KeyCode::Percent,
        sdl2::keyboard::Keycode::Dollar => KeyCode::Dollar,
        sdl2::keyboard::Keycode::Ampersand => KeyCode::Ampersand,
        sdl2::keyboard::Keycode::Quote => KeyCode::Quote,
        sdl2::keyboard::Keycode::LeftParen => KeyCode::LeftParen,
        sdl2::keyboard::Keycode::RightParen => KeyCode::RightParen,
        sdl2::keyboard::Keycode::Asterisk => KeyCode::Asterisk,
        sdl2::keyboard::Keycode::Plus => KeyCode::Plus,
        sdl2::keyboard::Keycode::Comma => KeyCode::Comma,
        sdl2::keyboard::Keycode::Minus => KeyCode::Minus,
        sdl2::keyboard::Keycode::Period => KeyCode::Period,
        sdl2::keyboard::Keycode::Slash => KeyCode::Slash,
        sdl2::keyboard::Keycode::Num0 => KeyCode::Num0,
        sdl2::keyboard::Keycode::Num1 => KeyCode::Num1,
        sdl2::keyboard::Keycode::Num2 => KeyCode::Num2,
        sdl2::keyboard::Keycode::Num3 => KeyCode::Num3,
        sdl2::keyboard::Keycode::Num4 => KeyCode::Num4,
        sdl2::keyboard::Keycode::Num5 => KeyCode::Num5,
        sdl2::keyboard::Keycode::Num6 => KeyCode::Num6,
        sdl2::keyboard::Keycode::Num7 => KeyCode::Num7,
        sdl2::keyboard::Keycode::Num8 => KeyCode::Num8,
        sdl2::keyboard::Keycode::Num9 => KeyCode::Num9,
        sdl2::keyboard::Keycode::Colon => KeyCode::Colon,
        sdl2::keyboard::Keycode::Semicolon => KeyCode::SemiColon,
        sdl2::keyboard::Keycode::Less => KeyCode::Less,
        sdl2::keyboard::Keycode::Equals => KeyCode::Equals,
        sdl2::keyboard::Keycode::Greater => KeyCode::Greater,
        sdl2::keyboard::Keycode::Question => KeyCode::Question,
        sdl2::keyboard::Keycode::At => KeyCode::At,
        sdl2::keyboard::Keycode::LeftBracket => KeyCode::LeftBracket,
        sdl2::keyboard::Keycode::Backslash => KeyCode::BackSlash,
        sdl2::keyboard::Keycode::RightBracket => KeyCode::RightBracket,
        sdl2::keyboard::Keycode::Caret => KeyCode::Caret,
        sdl2::keyboard::Keycode::Underscore => KeyCode::Underscore,
        sdl2::keyboard::Keycode::Backquote => KeyCode::BackQuote,
        sdl2::keyboard::Keycode::A => KeyCode::A,
        sdl2::keyboard::Keycode::B => KeyCode::B,
        sdl2::keyboard::Keycode::C => KeyCode::C,
        sdl2::keyboard::Keycode::D => KeyCode::D,
        sdl2::keyboard::Keycode::E => KeyCode::E,
        sdl2::keyboard::Keycode::F => KeyCode::F,
        sdl2::keyboard::Keycode::G => KeyCode::G,
        sdl2::keyboard::Keycode::H => KeyCode::H,
        sdl2::keyboard::Keycode::I => KeyCode::I,
        sdl2::keyboard::Keycode::J => KeyCode::J,
        sdl2::keyboard::Keycode::K => KeyCode::K,
        sdl2::keyboard::Keycode::L => KeyCode::L,
        sdl2::keyboard::Keycode::M => KeyCode::M,
        sdl2::keyboard::Keycode::N => KeyCode::N,
        sdl2::keyboard::Keycode::O => KeyCode::O,
        sdl2::keyboard::Keycode::P => KeyCode::P,
        sdl2::keyboard::Keycode::Q => KeyCode::Q,
        sdl2::keyboard::Keycode::R => KeyCode::R,
        sdl2::keyboard::Keycode::S => KeyCode::S,
        sdl2::keyboard::Keycode::T => KeyCode::T,
        sdl2::keyboard::Keycode::U => KeyCode::U,
        sdl2::keyboard::Keycode::V => KeyCode::V,
        sdl2::keyboard::Keycode::W => KeyCode::W,
        sdl2::keyboard::Keycode::X => KeyCode::X,
        sdl2::keyboard::Keycode::Y => KeyCode::Y,
        sdl2::keyboard::Keycode::Z => KeyCode::Z,
        sdl2::keyboard::Keycode::CapsLock => KeyCode::CapsLock,
        sdl2::keyboard::Keycode::F1 => KeyCode::F1,
        sdl2::keyboard::Keycode::F2 => KeyCode::F2,
        sdl2::keyboard::Keycode::F3 => KeyCode::F3,
        sdl2::keyboard::Keycode::F4 => KeyCode::F4,
        sdl2::keyboard::Keycode::F5 => KeyCode::F5,
        sdl2::keyboard::Keycode::F6 => KeyCode::F6,
        sdl2::keyboard::Keycode::F7 => KeyCode::F7,
        sdl2::keyboard::Keycode::F8 => KeyCode::F8,
        sdl2::keyboard::Keycode::F9 => KeyCode::F9,
        sdl2::keyboard::Keycode::F10 => KeyCode::F10,
        sdl2::keyboard::Keycode::F11 => KeyCode::F11,
        sdl2::keyboard::Keycode::F12 => KeyCode::F12,
        sdl2::keyboard::Keycode::PrintScreen => KeyCode::PrintScreen,
        sdl2::keyboard::Keycode::ScrollLock => KeyCode::ScrollLock,
        sdl2::keyboard::Keycode::Pause => KeyCode::Pause,
        sdl2::keyboard::Keycode::Insert => KeyCode::Insert,
        sdl2::keyboard::Keycode::Home => KeyCode::Home,
        sdl2::keyboard::Keycode::PageUp => KeyCode::PageUp,
        sdl2::keyboard::Keycode::Delete => KeyCode::Delete,
        sdl2::keyboard::Keycode::End => KeyCode::End,
        sdl2::keyboard::Keycode::PageDown => KeyCode::PageDown,
        sdl2::keyboard::Keycode::Right => KeyCode::Right,
        sdl2::keyboard::Keycode::Left => KeyCode::Left,
        sdl2::keyboard::Keycode::Down => KeyCode::Down,
        sdl2::keyboard::Keycode::Up => KeyCode::Up,
        sdl2::keyboard::Keycode::NumLockClear => KeyCode::NumLockClear,
        sdl2::keyboard::Keycode::KpDivide => KeyCode::KpDivide,
        sdl2::keyboard::Keycode::KpMultiply => KeyCode::KpMultiply,
        sdl2::keyboard::Keycode::KpMinus => KeyCode::KpMinus,
        sdl2::keyboard::Keycode::KpPlus => KeyCode::KpPlus,
        sdl2::keyboard::Keycode::KpEnter => KeyCode::KpEnter,
        sdl2::keyboard::Keycode::Kp1 => KeyCode::Kp1,
        sdl2::keyboard::Keycode::Kp2 => KeyCode::Kp2,
        sdl2::keyboard::Keycode::Kp3 => KeyCode::Kp3,
        sdl2::keyboard::Keycode::Kp4 => KeyCode::Kp4,
        sdl2::keyboard::Keycode::Kp5 => KeyCode::Kp5,
        sdl2::keyboard::Keycode::Kp6 => KeyCode::Kp6,
        sdl2::keyboard::Keycode::Kp7 => KeyCode::Kp7,
        sdl2::keyboard::Keycode::Kp8 => KeyCode::Kp8,
        sdl2::keyboard::Keycode::Kp9 => KeyCode::Kp9,
        sdl2::keyboard::Keycode::Kp0 => KeyCode::Kp0,
        sdl2::keyboard::Keycode::KpPeriod => KeyCode::KpPeriod,
        sdl2::keyboard::Keycode::Application => KeyCode::Application,
        sdl2::keyboard::Keycode::Power => KeyCode::Power,
        sdl2::keyboard::Keycode::KpEquals => KeyCode::KpEquals,
        sdl2::keyboard::Keycode::F13 => KeyCode::F13,
        sdl2::keyboard::Keycode::F14 => KeyCode::F14,
        sdl2::keyboard::Keycode::F15 => KeyCode::F15,
        sdl2::keyboard::Keycode::F16 => KeyCode::F16,
        sdl2::keyboard::Keycode::F17 => KeyCode::F17,
        sdl2::keyboard::Keycode::F18 => KeyCode::F18,
        sdl2::keyboard::Keycode::F19 => KeyCode::F19,
        sdl2::keyboard::Keycode::F20 => KeyCode::F20,
        sdl2::keyboard::Keycode::F21 => KeyCode::F21,
        sdl2::keyboard::Keycode::F22 => KeyCode::F22,
        sdl2::keyboard::Keycode::F23 => KeyCode::F23,
        sdl2::keyboard::Keycode::F24 => KeyCode::F24,
        sdl2::keyboard::Keycode::Execute => KeyCode::Execute,
        sdl2::keyboard::Keycode::Help => KeyCode::Help,
        sdl2::keyboard::Keycode::Menu => KeyCode::Menu,
        sdl2::keyboard::Keycode::Select => KeyCode::Select,
        sdl2::keyboard::Keycode::Stop => KeyCode::Stop,
        sdl2::keyboard::Keycode::Again => KeyCode::Again,
        sdl2::keyboard::Keycode::Undo => KeyCode::Undo,
        sdl2::keyboard::Keycode::Cut => KeyCode::Cut,
        sdl2::keyboard::Keycode::Copy => KeyCode::Copy,
        sdl2::keyboard::Keycode::Paste => KeyCode::Paste,
        sdl2::keyboard::Keycode::Find => KeyCode::Find,
        sdl2::keyboard::Keycode::Mute => KeyCode::Mute,
        sdl2::keyboard::Keycode::VolumeUp => KeyCode::VolumeUp,
        sdl2::keyboard::Keycode::VolumeDown => KeyCode::VolumeDown,
        sdl2::keyboard::Keycode::KpComma => KeyCode::KpComma,
        sdl2::keyboard::Keycode::KpEqualsAS400 => KeyCode::KpEqualsAS400,
        sdl2::keyboard::Keycode::AltErase => KeyCode::AltErase,
        sdl2::keyboard::Keycode::Sysreq => KeyCode::SysReq,
        sdl2::keyboard::Keycode::Cancel => KeyCode::Cancel,
        sdl2::keyboard::Keycode::Clear => KeyCode::Clear,
        sdl2::keyboard::Keycode::Prior => KeyCode::Prior,
        sdl2::keyboard::Keycode::Return2 => KeyCode::Return2,
        sdl2::keyboard::Keycode::Separator => KeyCode::Separator,
        sdl2::keyboard::Keycode::Out => KeyCode::Out,
        sdl2::keyboard::Keycode::Oper => KeyCode::Oper,
        sdl2::keyboard::Keycode::ClearAgain => KeyCode::ClearAgain,
        sdl2::keyboard::Keycode::CrSel => KeyCode::CrSel,
        sdl2::keyboard::Keycode::ExSel => KeyCode::ExSel,
        sdl2::keyboard::Keycode::Kp00 => KeyCode::Kp00,
        sdl2::keyboard::Keycode::Kp000 => KeyCode::Kp000,
        sdl2::keyboard::Keycode::ThousandsSeparator => KeyCode::ThousandsSeparator,
        sdl2::keyboard::Keycode::DecimalSeparator => KeyCode::DecimalSeparator,
        sdl2::keyboard::Keycode::CurrencyUnit => KeyCode::CurrencyUnit,
        sdl2::keyboard::Keycode::CurrencySubUnit => KeyCode::CurrencySubUnit,
        sdl2::keyboard::Keycode::KpLeftParen => KeyCode::KpLeftParen,
        sdl2::keyboard::Keycode::KpRightParen => KeyCode::KpRightParen,
        sdl2::keyboard::Keycode::KpLeftBrace => KeyCode::KpLeftBrace,
        sdl2::keyboard::Keycode::KpRightBrace => KeyCode::KpRightBrace,
        sdl2::keyboard::Keycode::KpTab => KeyCode::KpTab,
        sdl2::keyboard::Keycode::KpBackspace => KeyCode::KpBackspace,
        sdl2::keyboard::Keycode::KpA => KeyCode::KpA,
        sdl2::keyboard::Keycode::KpB => KeyCode::KpB,
        sdl2::keyboard::Keycode::KpC => KeyCode::KpC,
        sdl2::keyboard::Keycode::KpD => KeyCode::KpD,
        sdl2::keyboard::Keycode::KpE => KeyCode::KpE,
        sdl2::keyboard::Keycode::KpF => KeyCode::KpF,
        sdl2::keyboard::Keycode::KpXor => KeyCode::KpXor,
        sdl2::keyboard::Keycode::KpPower => KeyCode::KpPower,
        sdl2::keyboard::Keycode::KpPercent => KeyCode::KpPercent,
        sdl2::keyboard::Keycode::KpLess => KeyCode::KpLess,
        sdl2::keyboard::Keycode::KpGreater => KeyCode::KpGreater,
        sdl2::keyboard::Keycode::KpAmpersand => KeyCode::KpAmpersand,
        sdl2::keyboard::Keycode::KpDblAmpersand => KeyCode::KpDoubleAmpersand,
        sdl2::keyboard::Keycode::KpVerticalBar => KeyCode::KpVerticalBar,
        sdl2::keyboard::Keycode::KpDblVerticalBar => KeyCode::KpDoubleVerticalBar,
        sdl2::keyboard::Keycode::KpColon => KeyCode::KpColon,
        sdl2::keyboard::Keycode::KpHash => KeyCode::KpHash,
        sdl2::keyboard::Keycode::KpSpace => KeyCode::KpSpace,
        sdl2::keyboard::Keycode::KpAt => KeyCode::KpAt,
        sdl2::keyboard::Keycode::KpExclam => KeyCode::KpExclamation,
        sdl2::keyboard::Keycode::KpMemStore => KeyCode::KpMemStore,
        sdl2::keyboard::Keycode::KpMemRecall => KeyCode::KpMemRecall,
        sdl2::keyboard::Keycode::KpMemClear => KeyCode::KpMemClear,
        sdl2::keyboard::Keycode::KpMemAdd => KeyCode::KpMemAdd,
        sdl2::keyboard::Keycode::KpMemSubtract => KeyCode::KpMemSubtract,
        sdl2::keyboard::Keycode::KpMemMultiply => KeyCode::KpMemMultiply,
        sdl2::keyboard::Keycode::KpMemDivide => KeyCode::KpMemDivide,
        sdl2::keyboard::Keycode::KpPlusMinus => KeyCode::KpPlusMinus,
        sdl2::keyboard::Keycode::KpClear => KeyCode::KpClear,
        sdl2::keyboard::Keycode::KpClearEntry => KeyCode::KpClearEntry,
        sdl2::keyboard::Keycode::KpBinary => KeyCode::KpBinary,
        sdl2::keyboard::Keycode::KpOctal => KeyCode::KpOctal,
        sdl2::keyboard::Keycode::KpDecimal => KeyCode::KpDecimal,
        sdl2::keyboard::Keycode::KpHexadecimal => KeyCode::KpHexadecimal,
        sdl2::keyboard::Keycode::LCtrl => KeyCode::LCtrl,
        sdl2::keyboard::Keycode::LShift => KeyCode::LShift,
        sdl2::keyboard::Keycode::LAlt => KeyCode::LAlt,
        sdl2::keyboard::Keycode::LGui => KeyCode::LGui,
        sdl2::keyboard::Keycode::RCtrl => KeyCode::RCtrl,
        sdl2::keyboard::Keycode::RShift => KeyCode::RShift,
        sdl2::keyboard::Keycode::RAlt => KeyCode::RAlt,
        sdl2::keyboard::Keycode::RGui => KeyCode::RGui,
        sdl2::keyboard::Keycode::Mode => KeyCode::Mode,
        sdl2::keyboard::Keycode::AudioNext => KeyCode::AudioNext,
        sdl2::keyboard::Keycode::AudioPrev => KeyCode::AudioPrev,
        sdl2::keyboard::Keycode::AudioStop => KeyCode::AudioStop,
        sdl2::keyboard::Keycode::AudioPlay => KeyCode::AudioPlay,
        sdl2::keyboard::Keycode::AudioMute => KeyCode::AudioMute,
        sdl2::keyboard::Keycode::MediaSelect => KeyCode::MediaSelect,
        sdl2::keyboard::Keycode::Www => KeyCode::WWW,
        sdl2::keyboard::Keycode::Mail => KeyCode::Mail,
        sdl2::keyboard::Keycode::Calculator => KeyCode::Calculator,
        sdl2::keyboard::Keycode::Computer => KeyCode::Computer,
        sdl2::keyboard::Keycode::AcSearch => KeyCode::AcSearch,
        sdl2::keyboard::Keycode::AcHome => KeyCode::AcHome,
        sdl2::keyboard::Keycode::AcBack => KeyCode::AcBack,
        sdl2::keyboard::Keycode::AcForward => KeyCode::AcForward,
        sdl2::keyboard::Keycode::AcStop => KeyCode::AcStop,
        sdl2::keyboard::Keycode::AcRefresh => KeyCode::AcRefresh,
        sdl2::keyboard::Keycode::AcBookmarks => KeyCode::AcBookmarks,
        sdl2::keyboard::Keycode::BrightnessDown => KeyCode::BrightnessDown,
        sdl2::keyboard::Keycode::BrightnessUp => KeyCode::BrightnessUp,
        sdl2::keyboard::Keycode::DisplaySwitch => KeyCode::DisplaySwitch,
        sdl2::keyboard::Keycode::KbdIllumToggle => KeyCode::KeyboardIlluminationToggle,
        sdl2::keyboard::Keycode::KbdIllumDown => KeyCode::KeyboardIlluminationDown,
        sdl2::keyboard::Keycode::KbdIllumUp => KeyCode::KeyboardIlluminationUp,
        sdl2::keyboard::Keycode::Eject => KeyCode::Eject,
        sdl2::keyboard::Keycode::Sleep => KeyCode::Sleep,
    }
}

fn translate_scan_code(scan_code: sdl2::keyboard::Scancode) -> ScanCode {
    match scan_code {
        sdl2::keyboard::Scancode::A => ScanCode::A,
        sdl2::keyboard::Scancode::B => ScanCode::B,
        sdl2::keyboard::Scancode::C => ScanCode::C,
        sdl2::keyboard::Scancode::D => ScanCode::D,
        sdl2::keyboard::Scancode::E => ScanCode::E,
        sdl2::keyboard::Scancode::F => ScanCode::F,
        sdl2::keyboard::Scancode::G => ScanCode::G,
        sdl2::keyboard::Scancode::H => ScanCode::H,
        sdl2::keyboard::Scancode::I => ScanCode::I,
        sdl2::keyboard::Scancode::J => ScanCode::J,
        sdl2::keyboard::Scancode::K => ScanCode::K,
        sdl2::keyboard::Scancode::L => ScanCode::L,
        sdl2::keyboard::Scancode::M => ScanCode::M,
        sdl2::keyboard::Scancode::N => ScanCode::N,
        sdl2::keyboard::Scancode::O => ScanCode::O,
        sdl2::keyboard::Scancode::P => ScanCode::P,
        sdl2::keyboard::Scancode::Q => ScanCode::Q,
        sdl2::keyboard::Scancode::R => ScanCode::R,
        sdl2::keyboard::Scancode::S => ScanCode::S,
        sdl2::keyboard::Scancode::T => ScanCode::T,
        sdl2::keyboard::Scancode::U => ScanCode::U,
        sdl2::keyboard::Scancode::V => ScanCode::V,
        sdl2::keyboard::Scancode::W => ScanCode::W,
        sdl2::keyboard::Scancode::X => ScanCode::X,
        sdl2::keyboard::Scancode::Y => ScanCode::Y,
        sdl2::keyboard::Scancode::Z => ScanCode::Z,
        sdl2::keyboard::Scancode::Num1 => ScanCode::Num1,
        sdl2::keyboard::Scancode::Num2 => ScanCode::Num2,
        sdl2::keyboard::Scancode::Num3 => ScanCode::Num3,
        sdl2::keyboard::Scancode::Num4 => ScanCode::Num4,
        sdl2::keyboard::Scancode::Num5 => ScanCode::Num5,
        sdl2::keyboard::Scancode::Num6 => ScanCode::Num6,
        sdl2::keyboard::Scancode::Num7 => ScanCode::Num7,
        sdl2::keyboard::Scancode::Num8 => ScanCode::Num8,
        sdl2::keyboard::Scancode::Num9 => ScanCode::Num9,
        sdl2::keyboard::Scancode::Num0 => ScanCode::Num0,
        sdl2::keyboard::Scancode::Return => ScanCode::Return,
        sdl2::keyboard::Scancode::Escape => ScanCode::Escape,
        sdl2::keyboard::Scancode::Backspace => ScanCode::Backspace,
        sdl2::keyboard::Scancode::Tab => ScanCode::Tab,
        sdl2::keyboard::Scancode::Space => ScanCode::Space,
        sdl2::keyboard::Scancode::Minus => ScanCode::Minus,
        sdl2::keyboard::Scancode::Equals => ScanCode::Equals,
        sdl2::keyboard::Scancode::LeftBracket => ScanCode::LeftBracket,
        sdl2::keyboard::Scancode::RightBracket => ScanCode::RightBracket,
        sdl2::keyboard::Scancode::Backslash => ScanCode::BackSlash,
        sdl2::keyboard::Scancode::NonUsHash => ScanCode::NonUsSlash,
        sdl2::keyboard::Scancode::Semicolon => ScanCode::Semicolon,
        sdl2::keyboard::Scancode::Apostrophe => ScanCode::Apostrophe,
        sdl2::keyboard::Scancode::Grave => ScanCode::Grave,
        sdl2::keyboard::Scancode::Comma => ScanCode::Comma,
        sdl2::keyboard::Scancode::Period => ScanCode::Period,
        sdl2::keyboard::Scancode::Slash => ScanCode::Slash,
        sdl2::keyboard::Scancode::CapsLock => ScanCode::CapsLock,
        sdl2::keyboard::Scancode::F1 => ScanCode::F1,
        sdl2::keyboard::Scancode::F2 => ScanCode::F2,
        sdl2::keyboard::Scancode::F3 => ScanCode::F3,
        sdl2::keyboard::Scancode::F4 => ScanCode::F4,
        sdl2::keyboard::Scancode::F5 => ScanCode::F5,
        sdl2::keyboard::Scancode::F6 => ScanCode::F6,
        sdl2::keyboard::Scancode::F7 => ScanCode::F7,
        sdl2::keyboard::Scancode::F8 => ScanCode::F8,
        sdl2::keyboard::Scancode::F9 => ScanCode::F9,
        sdl2::keyboard::Scancode::F10 => ScanCode::F10,
        sdl2::keyboard::Scancode::F11 => ScanCode::F11,
        sdl2::keyboard::Scancode::F12 => ScanCode::F12,
        sdl2::keyboard::Scancode::PrintScreen => ScanCode::PrintScreen,
        sdl2::keyboard::Scancode::ScrollLock => ScanCode::ScrollLock,
        sdl2::keyboard::Scancode::Pause => ScanCode::Pause,
        sdl2::keyboard::Scancode::Insert => ScanCode::Insert,
        sdl2::keyboard::Scancode::Home => ScanCode::Home,
        sdl2::keyboard::Scancode::PageUp => ScanCode::PageUp,
        sdl2::keyboard::Scancode::Delete => ScanCode::Delete,
        sdl2::keyboard::Scancode::End => ScanCode::End,
        sdl2::keyboard::Scancode::PageDown => ScanCode::PageDown,
        sdl2::keyboard::Scancode::Right => ScanCode::Right,
        sdl2::keyboard::Scancode::Left => ScanCode::Left,
        sdl2::keyboard::Scancode::Down => ScanCode::Down,
        sdl2::keyboard::Scancode::Up => ScanCode::Up,
        sdl2::keyboard::Scancode::NumLockClear => ScanCode::NumLockClear,
        sdl2::keyboard::Scancode::KpDivide => ScanCode::KpDivide,
        sdl2::keyboard::Scancode::KpMultiply => ScanCode::KpMultiply,
        sdl2::keyboard::Scancode::KpMinus => ScanCode::KpMinus,
        sdl2::keyboard::Scancode::KpPlus => ScanCode::KpPlus,
        sdl2::keyboard::Scancode::KpEnter => ScanCode::KpEnter,
        sdl2::keyboard::Scancode::Kp1 => ScanCode::Kp1,
        sdl2::keyboard::Scancode::Kp2 => ScanCode::Kp2,
        sdl2::keyboard::Scancode::Kp3 => ScanCode::Kp3,
        sdl2::keyboard::Scancode::Kp4 => ScanCode::Kp4,
        sdl2::keyboard::Scancode::Kp5 => ScanCode::Kp5,
        sdl2::keyboard::Scancode::Kp6 => ScanCode::Kp6,
        sdl2::keyboard::Scancode::Kp7 => ScanCode::Kp7,
        sdl2::keyboard::Scancode::Kp8 => ScanCode::Kp8,
        sdl2::keyboard::Scancode::Kp9 => ScanCode::Kp9,
        sdl2::keyboard::Scancode::Kp0 => ScanCode::Kp0,
        sdl2::keyboard::Scancode::KpPeriod => ScanCode::KpPeriod,
        sdl2::keyboard::Scancode::NonUsBackslash => ScanCode::NonUsBackslash,
        sdl2::keyboard::Scancode::Application => ScanCode::Application,
        sdl2::keyboard::Scancode::Power => ScanCode::Power,
        sdl2::keyboard::Scancode::KpEquals => ScanCode::KpEquals,
        sdl2::keyboard::Scancode::F13 => ScanCode::F13,
        sdl2::keyboard::Scancode::F14 => ScanCode::F14,
        sdl2::keyboard::Scancode::F15 => ScanCode::F15,
        sdl2::keyboard::Scancode::F16 => ScanCode::F16,
        sdl2::keyboard::Scancode::F17 => ScanCode::F17,
        sdl2::keyboard::Scancode::F18 => ScanCode::F18,
        sdl2::keyboard::Scancode::F19 => ScanCode::F19,
        sdl2::keyboard::Scancode::F20 => ScanCode::F20,
        sdl2::keyboard::Scancode::F21 => ScanCode::F21,
        sdl2::keyboard::Scancode::F22 => ScanCode::F22,
        sdl2::keyboard::Scancode::F23 => ScanCode::F23,
        sdl2::keyboard::Scancode::F24 => ScanCode::F24,
        sdl2::keyboard::Scancode::Execute => ScanCode::Execute,
        sdl2::keyboard::Scancode::Help => ScanCode::Help,
        sdl2::keyboard::Scancode::Menu => ScanCode::Menu,
        sdl2::keyboard::Scancode::Select => ScanCode::Select,
        sdl2::keyboard::Scancode::Stop => ScanCode::Stop,
        sdl2::keyboard::Scancode::Again => ScanCode::Again,
        sdl2::keyboard::Scancode::Undo => ScanCode::Undo,
        sdl2::keyboard::Scancode::Cut => ScanCode::Cut,
        sdl2::keyboard::Scancode::Copy => ScanCode::Copy,
        sdl2::keyboard::Scancode::Paste => ScanCode::Paste,
        sdl2::keyboard::Scancode::Find => ScanCode::Find,
        sdl2::keyboard::Scancode::Mute => ScanCode::Mute,
        sdl2::keyboard::Scancode::VolumeUp => ScanCode::VolumeUp,
        sdl2::keyboard::Scancode::VolumeDown => ScanCode::VolumeDown,
        sdl2::keyboard::Scancode::KpComma => ScanCode::KpComma,
        sdl2::keyboard::Scancode::KpEqualsAS400 => ScanCode::KpEqualsAS400,
        sdl2::keyboard::Scancode::International1 => ScanCode::International1,
        sdl2::keyboard::Scancode::International2 => ScanCode::International2,
        sdl2::keyboard::Scancode::International3 => ScanCode::International3,
        sdl2::keyboard::Scancode::International4 => ScanCode::International4,
        sdl2::keyboard::Scancode::International5 => ScanCode::International5,
        sdl2::keyboard::Scancode::International6 => ScanCode::International6,
        sdl2::keyboard::Scancode::International7 => ScanCode::International7,
        sdl2::keyboard::Scancode::International8 => ScanCode::International8,
        sdl2::keyboard::Scancode::International9 => ScanCode::International9,
        sdl2::keyboard::Scancode::Lang1 => ScanCode::Lang1,
        sdl2::keyboard::Scancode::Lang2 => ScanCode::Lang2,
        sdl2::keyboard::Scancode::Lang3 => ScanCode::Lang3,
        sdl2::keyboard::Scancode::Lang4 => ScanCode::Lang4,
        sdl2::keyboard::Scancode::Lang5 => ScanCode::Lang5,
        sdl2::keyboard::Scancode::Lang6 => ScanCode::Lang6,
        sdl2::keyboard::Scancode::Lang7 => ScanCode::Lang7,
        sdl2::keyboard::Scancode::Lang8 => ScanCode::Lang8,
        sdl2::keyboard::Scancode::Lang9 => ScanCode::Lang9,
        sdl2::keyboard::Scancode::AltErase => ScanCode::AltErase,
        sdl2::keyboard::Scancode::SysReq => ScanCode::SysReq,
        sdl2::keyboard::Scancode::Cancel => ScanCode::Cancel,
        sdl2::keyboard::Scancode::Clear => ScanCode::Clear,
        sdl2::keyboard::Scancode::Prior => ScanCode::Prior,
        sdl2::keyboard::Scancode::Return2 => ScanCode::Return2,
        sdl2::keyboard::Scancode::Separator => ScanCode::Separator,
        sdl2::keyboard::Scancode::Out => ScanCode::Out,
        sdl2::keyboard::Scancode::Oper => ScanCode::Oper,
        sdl2::keyboard::Scancode::ClearAgain => ScanCode::ClearAgain,
        sdl2::keyboard::Scancode::CrSel => ScanCode::CrSel,
        sdl2::keyboard::Scancode::ExSel => ScanCode::ExSel,
        sdl2::keyboard::Scancode::Kp00 => ScanCode::Kp00,
        sdl2::keyboard::Scancode::Kp000 => ScanCode::Kp000,
        sdl2::keyboard::Scancode::ThousandsSeparator => ScanCode::ThousandsSeparator,
        sdl2::keyboard::Scancode::DecimalSeparator => ScanCode::DecimalSeparator,
        sdl2::keyboard::Scancode::CurrencyUnit => ScanCode::CurrencyUnit,
        sdl2::keyboard::Scancode::CurrencySubUnit => ScanCode::CurrencySubUnit,
        sdl2::keyboard::Scancode::KpLeftParen => ScanCode::KpLeftParen,
        sdl2::keyboard::Scancode::KpRightParen => ScanCode::KpRightParen,
        sdl2::keyboard::Scancode::KpLeftBrace => ScanCode::KpLeftBrace,
        sdl2::keyboard::Scancode::KpRightBrace => ScanCode::KpRightBrace,
        sdl2::keyboard::Scancode::KpTab => ScanCode::KpTab,
        sdl2::keyboard::Scancode::KpBackspace => ScanCode::KpBackspace,
        sdl2::keyboard::Scancode::KpA => ScanCode::KpA,
        sdl2::keyboard::Scancode::KpB => ScanCode::KpB,
        sdl2::keyboard::Scancode::KpC => ScanCode::KpC,
        sdl2::keyboard::Scancode::KpD => ScanCode::KpD,
        sdl2::keyboard::Scancode::KpE => ScanCode::KpE,
        sdl2::keyboard::Scancode::KpF => ScanCode::KpF,
        sdl2::keyboard::Scancode::KpXor => ScanCode::KpXor,
        sdl2::keyboard::Scancode::KpPower => ScanCode::KpPower,
        sdl2::keyboard::Scancode::KpPercent => ScanCode::KpPercent,
        sdl2::keyboard::Scancode::KpLess => ScanCode::KpLess,
        sdl2::keyboard::Scancode::KpGreater => ScanCode::KpGreater,
        sdl2::keyboard::Scancode::KpAmpersand => ScanCode::KpAmpersand,
        sdl2::keyboard::Scancode::KpDblAmpersand => ScanCode::KpDoubleAmpersand,
        sdl2::keyboard::Scancode::KpVerticalBar => ScanCode::KpVerticalBar,
        sdl2::keyboard::Scancode::KpDblVerticalBar => ScanCode::KpDoubleVerticalBar,
        sdl2::keyboard::Scancode::KpColon => ScanCode::KpColon,
        sdl2::keyboard::Scancode::KpHash => ScanCode::KpHash,
        sdl2::keyboard::Scancode::KpSpace => ScanCode::KpSpace,
        sdl2::keyboard::Scancode::KpAt => ScanCode::KpAt,
        sdl2::keyboard::Scancode::KpExclam => ScanCode::KpExclamation,
        sdl2::keyboard::Scancode::KpMemStore => ScanCode::KpMemStore,
        sdl2::keyboard::Scancode::KpMemRecall => ScanCode::KpMemRecall,
        sdl2::keyboard::Scancode::KpMemClear => ScanCode::KpMemClear,
        sdl2::keyboard::Scancode::KpMemAdd => ScanCode::KpMemAdd,
        sdl2::keyboard::Scancode::KpMemSubtract => ScanCode::KpMemSubtract,
        sdl2::keyboard::Scancode::KpMemMultiply => ScanCode::KpMemMultiply,
        sdl2::keyboard::Scancode::KpMemDivide => ScanCode::KpMemDivide,
        sdl2::keyboard::Scancode::KpPlusMinus => ScanCode::KpPlusMinus,
        sdl2::keyboard::Scancode::KpClear => ScanCode::KpClear,
        sdl2::keyboard::Scancode::KpClearEntry => ScanCode::KpClearEntry,
        sdl2::keyboard::Scancode::KpBinary => ScanCode::KpBinary,
        sdl2::keyboard::Scancode::KpOctal => ScanCode::KpOctal,
        sdl2::keyboard::Scancode::KpDecimal => ScanCode::KpDecimal,
        sdl2::keyboard::Scancode::KpHexadecimal => ScanCode::KpHexadecimal,
        sdl2::keyboard::Scancode::LCtrl => ScanCode::LCtrl,
        sdl2::keyboard::Scancode::LShift => ScanCode::LShift,
        sdl2::keyboard::Scancode::LAlt => ScanCode::LAlt,
        sdl2::keyboard::Scancode::LGui => ScanCode::LGui,
        sdl2::keyboard::Scancode::RCtrl => ScanCode::RCtrl,
        sdl2::keyboard::Scancode::RShift => ScanCode::RShift,
        sdl2::keyboard::Scancode::RAlt => ScanCode::RAlt,
        sdl2::keyboard::Scancode::RGui => ScanCode::RGui,
        sdl2::keyboard::Scancode::Mode => ScanCode::Mode,
        sdl2::keyboard::Scancode::AudioNext => ScanCode::AudioNext,
        sdl2::keyboard::Scancode::AudioPrev => ScanCode::AudioPrev,
        sdl2::keyboard::Scancode::AudioStop => ScanCode::AudioStop,
        sdl2::keyboard::Scancode::AudioPlay => ScanCode::AudioPlay,
        sdl2::keyboard::Scancode::AudioMute => ScanCode::AudioMute,
        sdl2::keyboard::Scancode::MediaSelect => ScanCode::MediaSelect,
        sdl2::keyboard::Scancode::Www => ScanCode::WWW,
        sdl2::keyboard::Scancode::Mail => ScanCode::Mail,
        sdl2::keyboard::Scancode::Calculator => ScanCode::Calculator,
        sdl2::keyboard::Scancode::Computer => ScanCode::Computer,
        sdl2::keyboard::Scancode::AcSearch => ScanCode::AcSearch,
        sdl2::keyboard::Scancode::AcHome => ScanCode::AcHome,
        sdl2::keyboard::Scancode::AcBack => ScanCode::AcBack,
        sdl2::keyboard::Scancode::AcForward => ScanCode::AcForward,
        sdl2::keyboard::Scancode::AcStop => ScanCode::AcStop,
        sdl2::keyboard::Scancode::AcRefresh => ScanCode::AcRefresh,
        sdl2::keyboard::Scancode::AcBookmarks => ScanCode::AcBookmarks,
        sdl2::keyboard::Scancode::BrightnessDown => ScanCode::BrightnessDown,
        sdl2::keyboard::Scancode::BrightnessUp => ScanCode::BrightnessUp,
        sdl2::keyboard::Scancode::DisplaySwitch => ScanCode::DisplaySwitch,
        sdl2::keyboard::Scancode::KbdIllumToggle => ScanCode::KeyboardIlluminationToggle,
        sdl2::keyboard::Scancode::KbdIllumDown => ScanCode::KeyboardIlluminationDown,
        sdl2::keyboard::Scancode::KbdIllumUp => ScanCode::KeyboardIlluminationUp,
        sdl2::keyboard::Scancode::Eject => ScanCode::Eject,
        sdl2::keyboard::Scancode::Sleep => ScanCode::Sleep,
        sdl2::keyboard::Scancode::App1 => ScanCode::App1,
        sdl2::keyboard::Scancode::App2 => ScanCode::App2,
        sdl2::keyboard::Scancode::Num => ScanCode::AudioRewind,
    }
}

fn translate_key_mod(keymod: sdl2::keyboard::Mod) -> KeyMod {
    KeyMod(keymod.bits())
}
