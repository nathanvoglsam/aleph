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

use std::collections::HashMap;

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

    /// Table used for translating scan codes into key codes
    scan_code_translation_table: [Option<KeyCode>; ScanCode::MAX_VALUES],

    /// Table used for translating key codes into scan codes
    key_code_translation_table: HashMap<KeyCode, ScanCode>,
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
        log::info!("Initializing the Keyboard system");
        let keyboard_state = KeyboardState {
            keys: [false; ScanCode::MAX_VALUES],
            scan_code_translation_table: [None; ScanCode::MAX_VALUES], // TODO: Fill out
            key_code_translation_table: Default::default(),            // TODO: Fill out
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
                scancode,
                keymod,
                repeat,
                ..
            } => {
                let event = KeyDownEvent {
                    scan_code: translate_scan_code(scancode.unwrap()),
                    modifiers: translate_key_mod(keymod),
                    repeat,
                };
                keyboard_state.keys[event.scan_code as usize] = true;
                let event = KeyboardEvent::KeyDown(event);
                keyboard_events.push(event.clone());
                all_events.push(Event::KeyboardEvent(event));
            }
            sdl2::event::Event::KeyUp {
                scancode,
                keymod,
                repeat,
                ..
            } => {
                let event = KeyUpEvent {
                    scan_code: translate_scan_code(scancode.unwrap()),
                    modifiers: translate_key_mod(keymod),
                    repeat,
                };
                keyboard_state.keys[event.scan_code as usize] = false;
                let event = KeyboardEvent::KeyUp(event);
                keyboard_events.push(event.clone());
                all_events.push(Event::KeyboardEvent(event));
            }
            sdl2::event::Event::TextInput { text, .. } => {
                let event = TextInputEvent { text };
                let event = KeyboardEvent::TextInput(event);
                keyboard_events.push(event.clone());
                all_events.push(Event::KeyboardEvent(event));
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
    fn translate_scan_code(&self, scan_code: ScanCode) -> Option<KeyCode> {
        self.0.scan_code_translation_table[scan_code as usize]
    }

    fn translate_key_code(&self, key_code: KeyCode) -> Option<ScanCode> {
        self.0.key_code_translation_table.get(&key_code).copied()
    }

    fn key_code_down(&self, key_code: KeyCode) -> bool {
        self.translate_key_code(key_code)
            .map(|v| self.scan_code_down(v))
            .unwrap_or(false)
    }

    fn scan_code_down(&self, scan_code: ScanCode) -> bool {
        self.0.keys[scan_code as usize]
    }
}

fn translate_scan_code(scan_code: sdl2::keyboard::Scancode) -> ScanCode {
    use sdl2::keyboard::Scancode;
    match scan_code {
        Scancode::A => ScanCode::A,
        Scancode::B => ScanCode::B,
        Scancode::C => ScanCode::C,
        Scancode::D => ScanCode::D,
        Scancode::E => ScanCode::E,
        Scancode::F => ScanCode::F,
        Scancode::G => ScanCode::G,
        Scancode::H => ScanCode::H,
        Scancode::I => ScanCode::I,
        Scancode::J => ScanCode::J,
        Scancode::K => ScanCode::K,
        Scancode::L => ScanCode::L,
        Scancode::M => ScanCode::M,
        Scancode::N => ScanCode::N,
        Scancode::O => ScanCode::O,
        Scancode::P => ScanCode::P,
        Scancode::Q => ScanCode::Q,
        Scancode::R => ScanCode::R,
        Scancode::S => ScanCode::S,
        Scancode::T => ScanCode::T,
        Scancode::U => ScanCode::U,
        Scancode::V => ScanCode::V,
        Scancode::W => ScanCode::W,
        Scancode::X => ScanCode::X,
        Scancode::Y => ScanCode::Y,
        Scancode::Z => ScanCode::Z,
        Scancode::Num1 => ScanCode::Num1,
        Scancode::Num2 => ScanCode::Num2,
        Scancode::Num3 => ScanCode::Num3,
        Scancode::Num4 => ScanCode::Num4,
        Scancode::Num5 => ScanCode::Num5,
        Scancode::Num6 => ScanCode::Num6,
        Scancode::Num7 => ScanCode::Num7,
        Scancode::Num8 => ScanCode::Num8,
        Scancode::Num9 => ScanCode::Num9,
        Scancode::Num0 => ScanCode::Num0,
        Scancode::Return => ScanCode::Return,
        Scancode::Escape => ScanCode::Escape,
        Scancode::Backspace => ScanCode::Backspace,
        Scancode::Tab => ScanCode::Tab,
        Scancode::Space => ScanCode::Space,
        Scancode::Minus => ScanCode::Minus,
        Scancode::Equals => ScanCode::Equals,
        Scancode::LeftBracket => ScanCode::LeftBracket,
        Scancode::RightBracket => ScanCode::RightBracket,
        Scancode::Backslash => ScanCode::BackSlash,
        Scancode::NonUsHash => ScanCode::NonUsSlash,
        Scancode::Semicolon => ScanCode::Semicolon,
        Scancode::Apostrophe => ScanCode::Apostrophe,
        Scancode::Grave => ScanCode::Grave,
        Scancode::Comma => ScanCode::Comma,
        Scancode::Period => ScanCode::Period,
        Scancode::Slash => ScanCode::Slash,
        Scancode::CapsLock => ScanCode::CapsLock,
        Scancode::F1 => ScanCode::F1,
        Scancode::F2 => ScanCode::F2,
        Scancode::F3 => ScanCode::F3,
        Scancode::F4 => ScanCode::F4,
        Scancode::F5 => ScanCode::F5,
        Scancode::F6 => ScanCode::F6,
        Scancode::F7 => ScanCode::F7,
        Scancode::F8 => ScanCode::F8,
        Scancode::F9 => ScanCode::F9,
        Scancode::F10 => ScanCode::F10,
        Scancode::F11 => ScanCode::F11,
        Scancode::F12 => ScanCode::F12,
        Scancode::PrintScreen => ScanCode::PrintScreen,
        Scancode::ScrollLock => ScanCode::ScrollLock,
        Scancode::Pause => ScanCode::Pause,
        Scancode::Insert => ScanCode::Insert,
        Scancode::Home => ScanCode::Home,
        Scancode::PageUp => ScanCode::PageUp,
        Scancode::Delete => ScanCode::Delete,
        Scancode::End => ScanCode::End,
        Scancode::PageDown => ScanCode::PageDown,
        Scancode::Right => ScanCode::ArrowRight,
        Scancode::Left => ScanCode::ArrowLeft,
        Scancode::Down => ScanCode::ArrowDown,
        Scancode::Up => ScanCode::ArrowUp,
        Scancode::NumLockClear => ScanCode::NumLockClear,
        Scancode::KpDivide => ScanCode::PadDivide,
        Scancode::KpMultiply => ScanCode::PadMultiply,
        Scancode::KpMinus => ScanCode::PadMinus,
        Scancode::KpPlus => ScanCode::PadPlus,
        Scancode::KpEnter => ScanCode::PadEnter,
        Scancode::Kp1 => ScanCode::PadNum1,
        Scancode::Kp2 => ScanCode::PadNum2,
        Scancode::Kp3 => ScanCode::PadNum3,
        Scancode::Kp4 => ScanCode::PadNum4,
        Scancode::Kp5 => ScanCode::PadNum5,
        Scancode::Kp6 => ScanCode::PadNum6,
        Scancode::Kp7 => ScanCode::PadNum7,
        Scancode::Kp8 => ScanCode::PadNum8,
        Scancode::Kp9 => ScanCode::PadNum9,
        Scancode::Kp0 => ScanCode::PadNum0,
        Scancode::KpPeriod => ScanCode::PadPeriod,
        Scancode::NonUsBackslash => ScanCode::NonUsBackslash,
        Scancode::Application => ScanCode::Application,
        Scancode::Power => ScanCode::Power,
        Scancode::KpEquals => ScanCode::PadEquals,
        Scancode::F13 => ScanCode::F13,
        Scancode::F14 => ScanCode::F14,
        Scancode::F15 => ScanCode::F15,
        Scancode::F16 => ScanCode::F16,
        Scancode::F17 => ScanCode::F17,
        Scancode::F18 => ScanCode::F18,
        Scancode::F19 => ScanCode::F19,
        Scancode::F20 => ScanCode::F20,
        Scancode::F21 => ScanCode::F21,
        Scancode::F22 => ScanCode::F22,
        Scancode::F23 => ScanCode::F23,
        Scancode::F24 => ScanCode::F24,
        Scancode::Execute => ScanCode::Execute,
        Scancode::Help => ScanCode::Help,
        Scancode::Menu => ScanCode::Menu,
        Scancode::Select => ScanCode::Select,
        Scancode::Stop => ScanCode::Stop,
        Scancode::Again => ScanCode::Again,
        Scancode::Undo => ScanCode::Undo,
        Scancode::Cut => ScanCode::Cut,
        Scancode::Copy => ScanCode::Copy,
        Scancode::Paste => ScanCode::Paste,
        Scancode::Find => ScanCode::Find,
        Scancode::Mute => ScanCode::Mute,
        Scancode::VolumeUp => ScanCode::VolumeUp,
        Scancode::VolumeDown => ScanCode::VolumeDown,
        Scancode::KpComma => ScanCode::PadComma,
        Scancode::KpEqualsAS400 => ScanCode::PadEqualsAS400,
        Scancode::International1 => ScanCode::International1,
        Scancode::International2 => ScanCode::International2,
        Scancode::International3 => ScanCode::International3,
        Scancode::International4 => ScanCode::International4,
        Scancode::International5 => ScanCode::International5,
        Scancode::International6 => ScanCode::International6,
        Scancode::International7 => ScanCode::International7,
        Scancode::International8 => ScanCode::International8,
        Scancode::International9 => ScanCode::International9,
        Scancode::Lang1 => ScanCode::Lang1,
        Scancode::Lang2 => ScanCode::Lang2,
        Scancode::Lang3 => ScanCode::Lang3,
        Scancode::Lang4 => ScanCode::Lang4,
        Scancode::Lang5 => ScanCode::Lang5,
        Scancode::Lang6 => ScanCode::Lang6,
        Scancode::Lang7 => ScanCode::Lang7,
        Scancode::Lang8 => ScanCode::Lang8,
        Scancode::Lang9 => ScanCode::Lang9,
        Scancode::AltErase => ScanCode::AltErase,
        Scancode::SysReq => ScanCode::SysReq,
        Scancode::Cancel => ScanCode::Cancel,
        Scancode::Clear => ScanCode::Clear,
        Scancode::Prior => ScanCode::Prior,
        Scancode::Return2 => ScanCode::Return2,
        Scancode::Separator => ScanCode::Separator,
        Scancode::Out => ScanCode::Out,
        Scancode::Oper => ScanCode::Oper,
        Scancode::ClearAgain => ScanCode::ClearAgain,
        Scancode::CrSel => ScanCode::CrSel,
        Scancode::ExSel => ScanCode::ExSel,
        Scancode::Kp00 => ScanCode::Pad00,
        Scancode::Kp000 => ScanCode::Pad000,
        Scancode::ThousandsSeparator => ScanCode::ThousandsSeparator,
        Scancode::DecimalSeparator => ScanCode::DecimalSeparator,
        Scancode::CurrencyUnit => ScanCode::CurrencyUnit,
        Scancode::CurrencySubUnit => ScanCode::CurrencySubUnit,
        Scancode::KpLeftParen => ScanCode::PadLeftParen,
        Scancode::KpRightParen => ScanCode::PadRightParen,
        Scancode::KpLeftBrace => ScanCode::PadLeftBrace,
        Scancode::KpRightBrace => ScanCode::PadRightBrace,
        Scancode::KpTab => ScanCode::PadTab,
        Scancode::KpBackspace => ScanCode::PadBackspace,
        Scancode::KpA => ScanCode::PadA,
        Scancode::KpB => ScanCode::PadB,
        Scancode::KpC => ScanCode::PadC,
        Scancode::KpD => ScanCode::PadD,
        Scancode::KpE => ScanCode::PadE,
        Scancode::KpF => ScanCode::PadF,
        Scancode::KpXor => ScanCode::PadXor,
        Scancode::KpPower => ScanCode::PadPower,
        Scancode::KpPercent => ScanCode::PadPercent,
        Scancode::KpLess => ScanCode::PadLess,
        Scancode::KpGreater => ScanCode::PadGreater,
        Scancode::KpAmpersand => ScanCode::PadAmpersand,
        Scancode::KpDblAmpersand => ScanCode::PadDoubleAmpersand,
        Scancode::KpVerticalBar => ScanCode::PadVerticalBar,
        Scancode::KpDblVerticalBar => ScanCode::PadDoubleVerticalBar,
        Scancode::KpColon => ScanCode::PadColon,
        Scancode::KpHash => ScanCode::PadHash,
        Scancode::KpSpace => ScanCode::PadSpace,
        Scancode::KpAt => ScanCode::PadAt,
        Scancode::KpExclam => ScanCode::PadExclamation,
        Scancode::KpMemStore => ScanCode::PadMemStore,
        Scancode::KpMemRecall => ScanCode::PadMemRecall,
        Scancode::KpMemClear => ScanCode::PadMemClear,
        Scancode::KpMemAdd => ScanCode::PadMemAdd,
        Scancode::KpMemSubtract => ScanCode::PadMemSubtract,
        Scancode::KpMemMultiply => ScanCode::PadMemMultiply,
        Scancode::KpMemDivide => ScanCode::PadMemDivide,
        Scancode::KpPlusMinus => ScanCode::PadPlusMinus,
        Scancode::KpClear => ScanCode::PadClear,
        Scancode::KpClearEntry => ScanCode::PadClearEntry,
        Scancode::KpBinary => ScanCode::PadBinary,
        Scancode::KpOctal => ScanCode::PadOctal,
        Scancode::KpDecimal => ScanCode::PadDecimal,
        Scancode::KpHexadecimal => ScanCode::PadHexadecimal,
        Scancode::LCtrl => ScanCode::LeftCtrl,
        Scancode::LShift => ScanCode::LeftShift,
        Scancode::LAlt => ScanCode::LeftAlt,
        Scancode::LGui => ScanCode::LeftMeta,
        Scancode::RCtrl => ScanCode::RightCtrl,
        Scancode::RShift => ScanCode::RightShift,
        Scancode::RAlt => ScanCode::RightAlt,
        Scancode::RGui => ScanCode::RightMeta,
        Scancode::Mode => ScanCode::Mode,
        Scancode::AudioNext => ScanCode::AudioNext,
        Scancode::AudioPrev => ScanCode::AudioPrev,
        Scancode::AudioStop => ScanCode::AudioStop,
        Scancode::AudioPlay => ScanCode::AudioPlay,
        Scancode::AudioMute => ScanCode::AudioMute,
        Scancode::MediaSelect => ScanCode::MediaSelect,
        Scancode::Www => ScanCode::WWW,
        Scancode::Mail => ScanCode::Mail,
        Scancode::Calculator => ScanCode::Calculator,
        Scancode::Computer => ScanCode::Computer,
        Scancode::AcSearch => ScanCode::AcSearch,
        Scancode::AcHome => ScanCode::AcHome,
        Scancode::AcBack => ScanCode::AcBack,
        Scancode::AcForward => ScanCode::AcForward,
        Scancode::AcStop => ScanCode::AcStop,
        Scancode::AcRefresh => ScanCode::AcRefresh,
        Scancode::AcBookmarks => ScanCode::AcBookmarks,
        Scancode::BrightnessDown => ScanCode::BrightnessDown,
        Scancode::BrightnessUp => ScanCode::BrightnessUp,
        Scancode::DisplaySwitch => ScanCode::DisplaySwitch,
        Scancode::KbdIllumToggle => ScanCode::KeyboardIlluminationToggle,
        Scancode::KbdIllumDown => ScanCode::KeyboardIlluminationDown,
        Scancode::KbdIllumUp => ScanCode::KeyboardIlluminationUp,
        Scancode::Eject => ScanCode::Eject,
        Scancode::Sleep => ScanCode::Sleep,
        Scancode::App1 => ScanCode::App1,
        Scancode::App2 => ScanCode::App2,
        Scancode::Num => ScanCode::Unknown,
    }
}

fn translate_key_mod(keymod: sdl2::keyboard::Mod) -> KeyMod {
    KeyMod(keymod.bits())
}
