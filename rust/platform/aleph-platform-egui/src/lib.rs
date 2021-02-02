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

use aleph_platform::{
    Clipboard, Cursor, Events, FrameTimer, Keyboard, KeyboardEvent, Keycode, Mod, Mouse,
    MouseEvent, Window,
};

pub fn get_egui_input() -> egui::RawInput {
    optick::event!();

    let mouse_state = Mouse::get_state();
    let window_size = Window::size();

    let mouse_down = mouse_state.left();
    let mouse_pos = mouse_state.pos();
    let mouse_pos = Some(egui::Pos2::new(mouse_pos.0 as f32, mouse_pos.1 as f32));

    let scroll_delta = get_egui_scroll_delta();

    let screen_rect = egui::Pos2::new(window_size.0 as f32, window_size.1 as f32);
    let screen_rect = Some(egui::Rect::from_min_max(Default::default(), screen_rect));

    // TODO: Integrate with SDL2 hdpi stuff
    let pixels_per_point = Some(1.0);

    let time = Some(FrameTimer::elapsed_time());

    let predicted_dt = 1.0 / Window::refresh_rate() as f32;

    let modifiers = get_egui_modifiers();

    let events = get_egui_events();

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

pub fn process_egui_output(output: egui::Output) {
    optick::event!();

    if !output.copied_text.is_empty() {
        Clipboard::set(&output.copied_text);
    }

    match output.cursor_icon {
        egui::CursorIcon::Default => Mouse::set_cursor(Cursor::Arrow),
        egui::CursorIcon::PointingHand => Mouse::set_cursor(Cursor::Hand),
        egui::CursorIcon::ResizeHorizontal => Mouse::set_cursor(Cursor::SizeWE),
        egui::CursorIcon::ResizeNeSw => Mouse::set_cursor(Cursor::SizeNESW),
        egui::CursorIcon::ResizeNwSe => Mouse::set_cursor(Cursor::SizeNWSE),
        egui::CursorIcon::ResizeVertical => Mouse::set_cursor(Cursor::SizeNS),
        egui::CursorIcon::Text => Mouse::set_cursor(Cursor::IBeam),
        egui::CursorIcon::Grab => Mouse::set_cursor(Cursor::Arrow),
        egui::CursorIcon::Grabbing => Mouse::set_cursor(Cursor::Arrow),
    }
}

fn get_egui_scroll_delta() -> egui::Vec2 {
    let mouse_events = Mouse::events();
    let mut delta = egui::Vec2::new(0.0, 0.0);

    for event in mouse_events.iter() {
        match event {
            MouseEvent::MouseWheel(e) => {
                if matches!(e.direction, aleph_platform::MouseWheelDirection::Normal) {
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

fn get_egui_events() -> Vec<egui::Event> {
    let events = Events::get();

    let mut out = Vec::new();
    for event in events.iter() {
        match event {
            aleph_platform::Event::KeyboardEvent(event) => match event {
                KeyboardEvent::KeyDown(e) => {
                    // Add an event for copy paste
                    if e.keymod.contains(Mod::LCTRLMOD) || e.keymod.contains(Mod::RCTRLMOD) {
                        if e.keycode == Keycode::C {
                            out.push(egui::Event::Copy);
                        }
                        if e.keycode == Keycode::X {
                            out.push(egui::Event::Cut);
                        }
                    }

                    if let Some(key) = translate_key(e.keycode) {
                        let event = egui::Event::Key {
                            key,
                            pressed: true,
                            modifiers: translate_modifiers(e.keymod),
                        };
                        out.push(event);
                    }
                }
                KeyboardEvent::KeyUp(e) => {
                    if let Some(key) = translate_key(e.keycode) {
                        let event = egui::Event::Key {
                            key,
                            pressed: false,
                            modifiers: translate_modifiers(e.keymod),
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

fn get_egui_modifiers() -> egui::Modifiers {
    let keyboard_state = Keyboard::get_state();
    let alt =
        keyboard_state.keycode_down(Keycode::LAlt) || keyboard_state.keycode_down(Keycode::RAlt);
    let ctrl =
        keyboard_state.keycode_down(Keycode::LCtrl) || keyboard_state.keycode_down(Keycode::RCtrl);
    let shift = keyboard_state.keycode_down(Keycode::LShift)
        || keyboard_state.keycode_down(Keycode::RShift);

    egui::Modifiers {
        alt,
        ctrl,
        shift,
        command: ctrl, // This would need tweaking for mac, but I don't care about mac so oh well
        ..Default::default()
    }
}

fn translate_modifiers(m: Mod) -> egui::Modifiers {
    egui::Modifiers {
        alt: m.contains(Mod::LALTMOD) || m.contains(Mod::RALTMOD),
        ctrl: m.contains(Mod::LCTRLMOD) || m.contains(Mod::RCTRLMOD),
        shift: m.contains(Mod::LSHIFTMOD) || m.contains(Mod::RSHIFTMOD),
        command: m.contains(Mod::LCTRLMOD) || m.contains(Mod::RCTRLMOD), // Mac stuff here
        ..Default::default()
    }
}

fn translate_key(key: Keycode) -> Option<egui::Key> {
    let val = match key {
        Keycode::Backspace => egui::Key::Backspace,
        Keycode::Tab => egui::Key::Tab,
        Keycode::Return => egui::Key::Enter,
        Keycode::Escape => egui::Key::Escape,
        Keycode::Space => egui::Key::Space,
        Keycode::Num0 => egui::Key::Num0,
        Keycode::Num1 => egui::Key::Num1,
        Keycode::Num2 => egui::Key::Num2,
        Keycode::Num3 => egui::Key::Num3,
        Keycode::Num4 => egui::Key::Num4,
        Keycode::Num5 => egui::Key::Num5,
        Keycode::Num6 => egui::Key::Num6,
        Keycode::Num7 => egui::Key::Num7,
        Keycode::Num8 => egui::Key::Num8,
        Keycode::Num9 => egui::Key::Num9,
        Keycode::A => egui::Key::A,
        Keycode::B => egui::Key::B,
        Keycode::C => egui::Key::C,
        Keycode::D => egui::Key::D,
        Keycode::E => egui::Key::E,
        Keycode::F => egui::Key::F,
        Keycode::G => egui::Key::G,
        Keycode::H => egui::Key::H,
        Keycode::I => egui::Key::I,
        Keycode::J => egui::Key::J,
        Keycode::K => egui::Key::K,
        Keycode::L => egui::Key::L,
        Keycode::M => egui::Key::M,
        Keycode::N => egui::Key::N,
        Keycode::O => egui::Key::O,
        Keycode::P => egui::Key::P,
        Keycode::Q => egui::Key::Q,
        Keycode::R => egui::Key::R,
        Keycode::S => egui::Key::S,
        Keycode::T => egui::Key::T,
        Keycode::U => egui::Key::U,
        Keycode::V => egui::Key::V,
        Keycode::W => egui::Key::W,
        Keycode::X => egui::Key::X,
        Keycode::Y => egui::Key::Y,
        Keycode::Z => egui::Key::Z,
        Keycode::Delete => egui::Key::Delete,
        Keycode::Insert => egui::Key::Insert,
        Keycode::Home => egui::Key::Home,
        Keycode::PageUp => egui::Key::PageUp,
        Keycode::End => egui::Key::End,
        Keycode::PageDown => egui::Key::PageDown,
        Keycode::Right => egui::Key::ArrowRight,
        Keycode::Left => egui::Key::ArrowLeft,
        Keycode::Down => egui::Key::ArrowDown,
        Keycode::Up => egui::Key::ArrowUp,
        Keycode::Kp1 => egui::Key::Num1,
        Keycode::Kp2 => egui::Key::Num2,
        Keycode::Kp3 => egui::Key::Num3,
        Keycode::Kp4 => egui::Key::Num4,
        Keycode::Kp5 => egui::Key::Num5,
        Keycode::Kp6 => egui::Key::Num6,
        Keycode::Kp7 => egui::Key::Num7,
        Keycode::Kp8 => egui::Key::Num8,
        Keycode::Kp9 => egui::Key::Num9,
        Keycode::Kp0 => egui::Key::Num0,
        _ => return None,
    };
    Some(val)
}
