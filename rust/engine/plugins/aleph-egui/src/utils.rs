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

use interfaces::platform::{
    Cursor, Event, IClipboard, IEvents, IFrameTimer, IKeyboard, IMouse, IWindow, KeyCode, KeyMod,
    KeyboardEvent, MouseEvent, MouseWheelDirection,
};

pub fn get_egui_input(
    window: &dyn IWindow,
    mouse: &dyn IMouse,
    keyboard: &dyn IKeyboard,
    frame_timer: &dyn IFrameTimer,
    events: &dyn IEvents,
) -> egui::RawInput {
    let mouse_state = mouse.get_state();
    let window_size = window.size();

    let mouse_down = mouse_state.left();
    let mouse_pos = mouse_state.pos();
    let mouse_pos = Some(egui::Pos2::new(mouse_pos.0 as f32, mouse_pos.1 as f32));

    let scroll_delta = get_egui_scroll_delta(mouse);

    let screen_rect = egui::Pos2::new(window_size.0 as f32, window_size.1 as f32);
    let screen_rect = Some(egui::Rect::from_min_max(Default::default(), screen_rect));

    // TODO: Integrate with SDL2 hdpi stuff
    let pixels_per_point = Some(1.0);

    let time = Some(frame_timer.elapsed_time());

    let predicted_dt = 1.0 / window.refresh_rate() as f32;

    let modifiers = get_egui_modifiers(keyboard);

    let events = get_egui_events(events);

    egui::RawInput {
        mouse_down,
        mouse_pos,
        scroll_delta,
        screen_rect,
        pixels_per_point,
        time,
        predicted_dt,
        modifiers,
        events,
        ..Default::default()
    }
}

pub fn process_egui_output(output: egui::Output, mouse: &dyn IMouse, clipboard: &dyn IClipboard) {
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
    }
}

pub fn get_egui_scroll_delta(mouse: &dyn IMouse) -> egui::Vec2 {
    let mouse_events = mouse.events();
    let mut delta = egui::Vec2::new(0.0, 0.0);

    for event in mouse_events.events() {
        match event {
            MouseEvent::MouseWheel(e) => {
                if matches!(e.direction, MouseWheelDirection::Normal) {
                    delta.x += e.x as f32;
                    delta.y += e.y as f32;
                } else {
                    delta.x -= e.x as f32;
                    delta.y -= e.y as f32;
                }
            }
            _ => {}
        }
    }

    delta
}

pub fn get_egui_events(events: &dyn IEvents) -> Vec<egui::Event> {
    let events = events.get();

    let mut out = Vec::new();
    for event in events.events() {
        match event {
            Event::KeyboardEvent(event) => match event {
                KeyboardEvent::KeyDown(e) => {
                    // Add an event for copy paste
                    if e.key_mod.contains(KeyMod::L_CTRL) || e.key_mod.contains(KeyMod::R_CTRL) {
                        if e.key_code == KeyCode::C {
                            out.push(egui::Event::Copy);
                        }
                        if e.key_code == KeyCode::X {
                            out.push(egui::Event::Cut);
                        }
                    }

                    if let Some(key) = translate_key(e.key_code) {
                        let event = egui::Event::Key {
                            key,
                            pressed: true,
                            modifiers: translate_modifiers(e.key_mod),
                        };
                        out.push(event);
                    }
                }
                KeyboardEvent::KeyUp(e) => {
                    if let Some(key) = translate_key(e.key_code) {
                        let event = egui::Event::Key {
                            key,
                            pressed: false,
                            modifiers: translate_modifiers(e.key_mod),
                        };

                        out.push(event);
                    }
                }
                KeyboardEvent::TextInput(e) => {
                    let event = egui::Event::Text(e.text.clone());
                    out.push(event);
                }
            },
            _ => {}
        }
    }
    out
}

pub fn get_egui_modifiers(keyboard: &dyn IKeyboard) -> egui::Modifiers {
    let keyboard_state = keyboard.get_state();
    let alt =
        keyboard_state.key_code_down(KeyCode::LAlt) || keyboard_state.key_code_down(KeyCode::RAlt);
    let ctrl = keyboard_state.key_code_down(KeyCode::LCtrl)
        || keyboard_state.key_code_down(KeyCode::RCtrl);
    let shift = keyboard_state.key_code_down(KeyCode::LShift)
        || keyboard_state.key_code_down(KeyCode::RShift);

    egui::Modifiers {
        alt,
        ctrl,
        shift,
        command: ctrl, // This would need tweaking for mac, but I don't care about mac so oh well
        ..Default::default()
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

pub fn translate_key(key: KeyCode) -> Option<egui::Key> {
    let val = match key {
        KeyCode::Backspace => egui::Key::Backspace,
        KeyCode::Tab => egui::Key::Tab,
        KeyCode::Return => egui::Key::Enter,
        KeyCode::Escape => egui::Key::Escape,
        KeyCode::Space => egui::Key::Space,
        KeyCode::Num0 => egui::Key::Num0,
        KeyCode::Num1 => egui::Key::Num1,
        KeyCode::Num2 => egui::Key::Num2,
        KeyCode::Num3 => egui::Key::Num3,
        KeyCode::Num4 => egui::Key::Num4,
        KeyCode::Num5 => egui::Key::Num5,
        KeyCode::Num6 => egui::Key::Num6,
        KeyCode::Num7 => egui::Key::Num7,
        KeyCode::Num8 => egui::Key::Num8,
        KeyCode::Num9 => egui::Key::Num9,
        KeyCode::A => egui::Key::A,
        KeyCode::B => egui::Key::B,
        KeyCode::C => egui::Key::C,
        KeyCode::D => egui::Key::D,
        KeyCode::E => egui::Key::E,
        KeyCode::F => egui::Key::F,
        KeyCode::G => egui::Key::G,
        KeyCode::H => egui::Key::H,
        KeyCode::I => egui::Key::I,
        KeyCode::J => egui::Key::J,
        KeyCode::K => egui::Key::K,
        KeyCode::L => egui::Key::L,
        KeyCode::M => egui::Key::M,
        KeyCode::N => egui::Key::N,
        KeyCode::O => egui::Key::O,
        KeyCode::P => egui::Key::P,
        KeyCode::Q => egui::Key::Q,
        KeyCode::R => egui::Key::R,
        KeyCode::S => egui::Key::S,
        KeyCode::T => egui::Key::T,
        KeyCode::U => egui::Key::U,
        KeyCode::V => egui::Key::V,
        KeyCode::W => egui::Key::W,
        KeyCode::X => egui::Key::X,
        KeyCode::Y => egui::Key::Y,
        KeyCode::Z => egui::Key::Z,
        KeyCode::Delete => egui::Key::Delete,
        KeyCode::Insert => egui::Key::Insert,
        KeyCode::Home => egui::Key::Home,
        KeyCode::PageUp => egui::Key::PageUp,
        KeyCode::End => egui::Key::End,
        KeyCode::PageDown => egui::Key::PageDown,
        KeyCode::Right => egui::Key::ArrowRight,
        KeyCode::Left => egui::Key::ArrowLeft,
        KeyCode::Down => egui::Key::ArrowDown,
        KeyCode::Up => egui::Key::ArrowUp,
        KeyCode::Kp1 => egui::Key::Num1,
        KeyCode::Kp2 => egui::Key::Num2,
        KeyCode::Kp3 => egui::Key::Num3,
        KeyCode::Kp4 => egui::Key::Num4,
        KeyCode::Kp5 => egui::Key::Num5,
        KeyCode::Kp6 => egui::Key::Num6,
        KeyCode::Kp7 => egui::Key::Num7,
        KeyCode::Kp8 => egui::Key::Num8,
        KeyCode::Kp9 => egui::Key::Num9,
        KeyCode::Kp0 => egui::Key::Num0,
        _ => return None,
    };
    Some(val)
}
