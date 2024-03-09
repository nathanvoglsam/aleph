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

use egui::{ViewportId, ViewportInfo};
use interfaces::platform::{
    Cursor, Event, IClipboard, IEvents, IFrameTimer, IKeyboard, IMouse, IWindow, KeyCode, KeyMod,
    KeyboardEvent, MouseButton, MouseEvent, MouseWheelDirection, ScanCode,
};

pub fn get_egui_input(
    window: &dyn IWindow,
    keyboard: &dyn IKeyboard,
    frame_timer: &dyn IFrameTimer,
    events: &dyn IEvents,
) -> egui::RawInput {
    let window_size = window.size();

    let screen_rect = egui::Pos2::new(window_size.0 as f32, window_size.1 as f32);
    let screen_rect = Some(egui::Rect::from_min_max(Default::default(), screen_rect));

    let time = Some(frame_timer.elapsed_time());

    let predicted_dt = 1.0 / 60 as f32;

    let modifiers = get_egui_modifiers(keyboard);

    let events = get_egui_events(events, &modifiers);

    let viewport_info = ViewportInfo {
        native_pixels_per_point: Some(window.current_display_scale()),
        ..Default::default()
    };
    egui::RawInput {
        viewport_id: ViewportId::ROOT,
        viewports: std::iter::once((ViewportId::ROOT, viewport_info)).collect(),
        screen_rect,
        time,
        predicted_dt,
        modifiers,
        events,
        ..Default::default()
    }
}

pub fn process_egui_output(
    output: egui::PlatformOutput,
    mouse: &dyn IMouse,
    clipboard: &dyn IClipboard,
) {
    if !output.copied_text.is_empty() {
        clipboard.set(&output.copied_text);
    }

    match output.cursor_icon {
        egui::CursorIcon::Default => mouse.set_cursor(Cursor::Arrow),
        egui::CursorIcon::PointingHand => mouse.set_cursor(Cursor::Hand),
        egui::CursorIcon::ResizeHorizontal => mouse.set_cursor(Cursor::SizeWE),
        egui::CursorIcon::ResizeNeSw => mouse.set_cursor(Cursor::SizeNESW),
        egui::CursorIcon::ResizeNwSe => mouse.set_cursor(Cursor::SizeNWSE),
        egui::CursorIcon::ResizeVertical => mouse.set_cursor(Cursor::SizeNS),
        egui::CursorIcon::Text => mouse.set_cursor(Cursor::IBeam),
        egui::CursorIcon::Grab => mouse.set_cursor(Cursor::Arrow),
        egui::CursorIcon::Grabbing => mouse.set_cursor(Cursor::Arrow),
        _ => mouse.set_cursor(Cursor::Arrow),
    }
}

pub fn get_egui_events(events: &dyn IEvents, modifiers: &egui::Modifiers) -> Vec<egui::Event> {
    let events = events.get();

    let mut out = Vec::new();
    for event in events.events() {
        match event {
            Event::KeyboardEvent(event) => match event {
                KeyboardEvent::KeyDown(e) => {
                    // Add an event for copy paste
                    if e.modifiers.contains(KeyMod::L_CTRL) || e.modifiers.contains(KeyMod::R_CTRL)
                    {
                        if e.scan_code == ScanCode::C {
                            out.push(egui::Event::Copy);
                        }
                        if e.scan_code == ScanCode::X {
                            out.push(egui::Event::Cut);
                        }
                    }

                    if let Some(key) = translate_scan_code(e.scan_code) {
                        let event = egui::Event::Key {
                            key,
                            physical_key: Some(key),
                            pressed: true,
                            repeat: e.repeat,
                            modifiers: translate_modifiers(e.modifiers),
                        };
                        out.push(event);
                    }
                }
                KeyboardEvent::KeyUp(e) => {
                    if let Some(key) = translate_scan_code(e.scan_code) {
                        let event = egui::Event::Key {
                            key,
                            physical_key: Some(key),
                            pressed: false,
                            repeat: e.repeat,
                            modifiers: translate_modifiers(e.modifiers),
                        };

                        out.push(event);
                    }
                }
                KeyboardEvent::TextInput(e) => {
                    let event = egui::Event::Text(e.text.clone());
                    out.push(event);
                }
            },
            Event::MouseEvent(event) => {
                if let Some(event) = translate_mouse_event(event, modifiers) {
                    out.push(event);
                }
            }
            _ => {}
        }
    }
    out
}

pub fn get_egui_modifiers(keyboard: &dyn IKeyboard) -> egui::Modifiers {
    // TODO: Use KeyCode api when KeyCode->ScanCode translation works again
    let keyboard_state = keyboard.get_state();
    let alt = keyboard_state.scan_code_down(ScanCode::LeftAlt)
        || keyboard_state.scan_code_down(ScanCode::RightAlt);
    let ctrl = keyboard_state.scan_code_down(ScanCode::LeftCtrl)
        || keyboard_state.scan_code_down(ScanCode::RightCtrl);
    let shift = keyboard_state.scan_code_down(ScanCode::LeftShift)
        || keyboard_state.scan_code_down(ScanCode::RightShift);

    egui::Modifiers {
        alt,
        ctrl,
        shift,
        command: ctrl, // This would need tweaking for mac, but I don't care about mac so oh well
        ..Default::default()
    }
}

pub fn translate_mouse_event(
    event: &MouseEvent,
    modifiers: &egui::Modifiers,
) -> Option<egui::Event> {
    match event {
        MouseEvent::MouseMotion(e) => {
            let pos = egui::Pos2::new(e.x as f32, e.y as f32);
            let event = egui::Event::PointerMoved(pos);
            Some(event)
        }
        MouseEvent::MouseButtonDown(e) => {
            let pos = egui::Pos2::new(e.x as f32, e.y as f32);
            let button = translate_mouse_button(&e.button)?;
            let event = egui::Event::PointerButton {
                pos,
                button,
                pressed: true,
                modifiers: *modifiers,
            };
            Some(event)
        }
        MouseEvent::MouseButtonUp(e) => {
            let pos = egui::Pos2::new(e.x as f32, e.y as f32);
            let button = translate_mouse_button(&e.button)?;
            let event = egui::Event::PointerButton {
                pos,
                button,
                pressed: false,
                modifiers: *modifiers,
            };
            Some(event)
        }
        MouseEvent::MouseWheel(e) => {
            let scroll = if matches!(e.direction, MouseWheelDirection::Normal) {
                egui::Vec2::new(e.x as f32, e.y as f32)
            } else {
                egui::Vec2::new(-e.x as f32, -e.y as f32)
            };
            let event = egui::Event::Scroll(scroll);
            Some(event)
        }
    }
}

pub fn translate_mouse_button(button: &MouseButton) -> Option<egui::PointerButton> {
    match button {
        MouseButton::Left => Some(egui::PointerButton::Primary),
        MouseButton::Middle => Some(egui::PointerButton::Middle),
        MouseButton::Right => Some(egui::PointerButton::Secondary),
        MouseButton::X1 => None, // Skip emitting an event for this button
        MouseButton::X2 => None, // Skip emitting an event for this button
    }
}

pub fn translate_modifiers(m: KeyMod) -> egui::Modifiers {
    egui::Modifiers {
        alt: m.contains(KeyMod::L_ALT) || m.contains(KeyMod::R_ALT),
        ctrl: m.contains(KeyMod::L_CTRL) || m.contains(KeyMod::R_CTRL),
        shift: m.contains(KeyMod::L_SHIFT) || m.contains(KeyMod::R_SHIFT),
        command: m.contains(KeyMod::L_CTRL) || m.contains(KeyMod::R_CTRL), // Mac stuff here
        ..Default::default()
    }
}

// TODO: Once scan_code->key_code translation is done switch to this to handle regional keyboard
//       layouts
#[allow(unused)]
pub fn translate_key_code(key: KeyCode) -> Option<egui::Key> {
    match key {
        KeyCode::Backspace => Some(egui::Key::Backspace),
        KeyCode::Tab => Some(egui::Key::Tab),
        KeyCode::Enter => Some(egui::Key::Enter),
        KeyCode::Escape => Some(egui::Key::Escape),
        KeyCode::Space => Some(egui::Key::Space),
        KeyCode::Num0 => Some(egui::Key::Num0),
        KeyCode::Num1 => Some(egui::Key::Num1),
        KeyCode::Num2 => Some(egui::Key::Num2),
        KeyCode::Num3 => Some(egui::Key::Num3),
        KeyCode::Num4 => Some(egui::Key::Num4),
        KeyCode::Num5 => Some(egui::Key::Num5),
        KeyCode::Num6 => Some(egui::Key::Num6),
        KeyCode::Num7 => Some(egui::Key::Num7),
        KeyCode::Num8 => Some(egui::Key::Num8),
        KeyCode::Num9 => Some(egui::Key::Num9),
        KeyCode::A => Some(egui::Key::A),
        KeyCode::B => Some(egui::Key::B),
        KeyCode::C => Some(egui::Key::C),
        KeyCode::D => Some(egui::Key::D),
        KeyCode::E => Some(egui::Key::E),
        KeyCode::F => Some(egui::Key::F),
        KeyCode::G => Some(egui::Key::G),
        KeyCode::H => Some(egui::Key::H),
        KeyCode::I => Some(egui::Key::I),
        KeyCode::J => Some(egui::Key::J),
        KeyCode::K => Some(egui::Key::K),
        KeyCode::L => Some(egui::Key::L),
        KeyCode::M => Some(egui::Key::M),
        KeyCode::N => Some(egui::Key::N),
        KeyCode::O => Some(egui::Key::O),
        KeyCode::P => Some(egui::Key::P),
        KeyCode::Q => Some(egui::Key::Q),
        KeyCode::R => Some(egui::Key::R),
        KeyCode::S => Some(egui::Key::S),
        KeyCode::T => Some(egui::Key::T),
        KeyCode::U => Some(egui::Key::U),
        KeyCode::V => Some(egui::Key::V),
        KeyCode::W => Some(egui::Key::W),
        KeyCode::X => Some(egui::Key::X),
        KeyCode::Y => Some(egui::Key::Y),
        KeyCode::Z => Some(egui::Key::Z),
        KeyCode::Delete => Some(egui::Key::Delete),
        KeyCode::Insert => Some(egui::Key::Insert),
        KeyCode::Home => Some(egui::Key::Home),
        KeyCode::PageUp => Some(egui::Key::PageUp),
        KeyCode::End => Some(egui::Key::End),
        KeyCode::PageDown => Some(egui::Key::PageDown),
        KeyCode::ArrowRight => Some(egui::Key::ArrowRight),
        KeyCode::ArrowLeft => Some(egui::Key::ArrowLeft),
        KeyCode::ArrowDown => Some(egui::Key::ArrowDown),
        KeyCode::ArrowUp => Some(egui::Key::ArrowUp),
        KeyCode::PadNum1 => Some(egui::Key::Num1),
        KeyCode::PadNum2 => Some(egui::Key::Num2),
        KeyCode::PadNum3 => Some(egui::Key::Num3),
        KeyCode::PadNum4 => Some(egui::Key::Num4),
        KeyCode::PadNum5 => Some(egui::Key::Num5),
        KeyCode::PadNum6 => Some(egui::Key::Num6),
        KeyCode::PadNum7 => Some(egui::Key::Num7),
        KeyCode::PadNum8 => Some(egui::Key::Num8),
        KeyCode::PadNum9 => Some(egui::Key::Num9),
        KeyCode::PadNum0 => Some(egui::Key::Num0),
        _ => None,
    }
}

pub fn translate_scan_code(key: ScanCode) -> Option<egui::Key> {
    match key {
        ScanCode::Backspace => Some(egui::Key::Backspace),
        ScanCode::Tab => Some(egui::Key::Tab),
        ScanCode::Return => Some(egui::Key::Enter),
        ScanCode::Escape => Some(egui::Key::Escape),
        ScanCode::Space => Some(egui::Key::Space),
        ScanCode::Num0 => Some(egui::Key::Num0),
        ScanCode::Num1 => Some(egui::Key::Num1),
        ScanCode::Num2 => Some(egui::Key::Num2),
        ScanCode::Num3 => Some(egui::Key::Num3),
        ScanCode::Num4 => Some(egui::Key::Num4),
        ScanCode::Num5 => Some(egui::Key::Num5),
        ScanCode::Num6 => Some(egui::Key::Num6),
        ScanCode::Num7 => Some(egui::Key::Num7),
        ScanCode::Num8 => Some(egui::Key::Num8),
        ScanCode::Num9 => Some(egui::Key::Num9),
        ScanCode::A => Some(egui::Key::A),
        ScanCode::B => Some(egui::Key::B),
        ScanCode::C => Some(egui::Key::C),
        ScanCode::D => Some(egui::Key::D),
        ScanCode::E => Some(egui::Key::E),
        ScanCode::F => Some(egui::Key::F),
        ScanCode::G => Some(egui::Key::G),
        ScanCode::H => Some(egui::Key::H),
        ScanCode::I => Some(egui::Key::I),
        ScanCode::J => Some(egui::Key::J),
        ScanCode::K => Some(egui::Key::K),
        ScanCode::L => Some(egui::Key::L),
        ScanCode::M => Some(egui::Key::M),
        ScanCode::N => Some(egui::Key::N),
        ScanCode::O => Some(egui::Key::O),
        ScanCode::P => Some(egui::Key::P),
        ScanCode::Q => Some(egui::Key::Q),
        ScanCode::R => Some(egui::Key::R),
        ScanCode::S => Some(egui::Key::S),
        ScanCode::T => Some(egui::Key::T),
        ScanCode::U => Some(egui::Key::U),
        ScanCode::V => Some(egui::Key::V),
        ScanCode::W => Some(egui::Key::W),
        ScanCode::X => Some(egui::Key::X),
        ScanCode::Y => Some(egui::Key::Y),
        ScanCode::Z => Some(egui::Key::Z),
        ScanCode::Delete => Some(egui::Key::Delete),
        ScanCode::Insert => Some(egui::Key::Insert),
        ScanCode::Home => Some(egui::Key::Home),
        ScanCode::PageUp => Some(egui::Key::PageUp),
        ScanCode::End => Some(egui::Key::End),
        ScanCode::PageDown => Some(egui::Key::PageDown),
        ScanCode::ArrowRight => Some(egui::Key::ArrowRight),
        ScanCode::ArrowLeft => Some(egui::Key::ArrowLeft),
        ScanCode::ArrowDown => Some(egui::Key::ArrowDown),
        ScanCode::ArrowUp => Some(egui::Key::ArrowUp),
        ScanCode::PadNum1 => Some(egui::Key::Num1),
        ScanCode::PadNum2 => Some(egui::Key::Num2),
        ScanCode::PadNum3 => Some(egui::Key::Num3),
        ScanCode::PadNum4 => Some(egui::Key::Num4),
        ScanCode::PadNum5 => Some(egui::Key::Num5),
        ScanCode::PadNum6 => Some(egui::Key::Num6),
        ScanCode::PadNum7 => Some(egui::Key::Num7),
        ScanCode::PadNum8 => Some(egui::Key::Num8),
        ScanCode::PadNum9 => Some(egui::Key::Num9),
        ScanCode::PadNum0 => Some(egui::Key::Num0),
        _ => None,
    }
}
