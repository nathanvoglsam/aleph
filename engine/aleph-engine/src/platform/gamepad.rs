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

use std::cell::{Cell, RefCell};
use std::collections::HashMap;
use std::hash::BuildHasherDefault;
use std::ops::Deref;

use aleph_identity_hasher::IdentityHasher;
use interfaces::any::{AnyArc, declare_interfaces};
use interfaces::platform::{
    GamepadAxis, GamepadAxisMotion, GamepadButton, GamepadButtonDown, GamepadButtonUp,
    GamepadEvent, GamepadId, GamepadState, IGamepads, IGamepadsAccessor,
};
use parking_lot::RwLock;
use sdl3::gamepad::Gamepad;

pub(crate) type GamepadsMap = HashMap<u32, GamepadEntry, BuildHasherDefault<IdentityHasher>>;

pub struct Gamepads {
    /// Set of all gamepads currently connected and opened by the gamepads system. Only one of these
    /// will be considered active at any one point.
    pub(crate) gamepads: RefCell<GamepadsMap>,

    /// The highest gamepad ID we've opened. Used to determine if we get handed a bad ID or an ID
    /// for a controller that has since been disconnected and not in the gamepads set anymore.
    pub(crate) highest_id: Cell<u32>,

    /// The index of the currently active controller, if one is active
    pub(crate) active_controller: Cell<Option<u32>>,

    /// The thread-safe object we can share to enable safe access to the current gamepad state
    /// outside of the main thread. New state and events will be published here every frame.
    pub(crate) accessor: AnyArc<GamepadsAccessor>,
}

declare_interfaces!(Gamepads, [IGamepads]);

impl Gamepads {
    pub(crate) fn new() -> AnyArc<Self> {
        AnyArc::new(Self {
            gamepads: Default::default(),
            highest_id: Default::default(),
            active_controller: Default::default(),
            accessor: AnyArc::new(Default::default()),
        })
    }

    ///
    /// Internal function for handling the events produced by the OS
    ///
    pub(crate) fn process_gamepad_event(
        &self,
        gamepads: &mut GamepadsMap,
        gamepad: &sdl3::GamepadSubsystem,
        event: &sdl3::event::Event,
    ) {
        use sdl3::event::Event as SdlEvent;
        match event {
            SdlEvent::ControllerDeviceAdded { which, .. } => {
                let which = *which;

                let is_controller = gamepad.is_gamepad(which);
                let name = gamepad.name_for_id(which).unwrap();

                log::info!("Controller Added: {which}");
                log::info!("name = {name}; is_controller = {is_controller};");

                if is_controller {
                    let pad = gamepad.open(which).unwrap();
                    let instance_id = pad.id().unwrap();
                    let current_highest = self.highest_id.take();
                    self.highest_id.set(instance_id.max(current_highest));

                    log::info!("Controller Opened: instance_id = {instance_id}");

                    let entry = GamepadEntry {
                        _device_index: which,
                        pad,
                        state: Default::default(),
                    };
                    gamepads.insert(instance_id, entry);
                }
            }
            SdlEvent::ControllerDeviceRemoved { which, .. } => {
                let which = *which;

                log::info!("Controller Removed: {which}");

                if let Some(gamepad) = gamepads.get(&which) {
                    let name = gamepad.pad.name();
                    let name = name.as_deref().unwrap_or("Unknown");
                    log::info!("Removing Controler: name = {name};");
                    gamepads.remove(&which);

                    // Remove the active controller if we happened to have it disconnect
                    if let Some(active) = self.active_controller.get() {
                        if active == which {
                            self.active_controller.set(None);
                        }
                    }
                }
            }
            SdlEvent::ControllerDeviceRemapped { which, .. } => {
                let which = *which;
                log::info!("Controller Remapped: {which}");
            }
            SdlEvent::ControllerAxisMotion {
                which, axis, value, ..
            } => {
                let which = *which;
                let axis = *axis;
                let value = *value;

                self.update_active_controller(which);

                let entry = gamepads.get_mut(&which).unwrap();
                set_axis(&mut entry.state, axis, value);
            }
            SdlEvent::ControllerButtonDown { which, button, .. } => {
                let which = *which;
                let button = *button;

                self.update_active_controller(which);

                let entry = gamepads.get_mut(&which).unwrap();
                set_button(&mut entry.state, button, true);
            }
            SdlEvent::ControllerButtonUp { which, button, .. } => {
                let which = *which;
                let button = *button;

                self.update_active_controller(which);

                let entry = gamepads.get_mut(&which).unwrap();
                set_button(&mut entry.state, button, false);
            }
            _ => {}
        }
    }

    fn update_active_controller(&self, new_id: u32) {
        match self.active_controller.get() {
            Some(active) => {
                if active != new_id {
                    log::info!("Controller {new_id} became the active controller!");
                }
            }
            None => {
                log::info!("Controller {new_id} became the active controller!");
            }
        }
        self.active_controller.set(Some(new_id));
    }

    pub(crate) fn publish_active_state(
        &self,
        gamepads: &GamepadsMap,
        events: &[sdl3::event::Event],
    ) {
        let mut shared_state = self.accessor.shared_state.write();
        if let Some(active) = self.active_controller.get() {
            let entry = gamepads.get(&active).unwrap();
            *shared_state = Some(entry.state.clone());
        } else {
            *shared_state = None;
        }

        let mut shared_events = self.accessor.shared_events.write();
        if let Some(active) = self.active_controller.get() {
            let filtered_events = events.iter().filter_map(|v| {
                use sdl3::event::Event as SdlEvent;
                match v {
                    SdlEvent::ControllerAxisMotion {
                        which, axis, value, ..
                    } => {
                        if *which == active {
                            Some(GamepadEvent::AxisMotion(GamepadAxisMotion {
                                which: GamepadId(active),
                                axis: map_axis(*axis),
                                value: *value,
                            }))
                        } else {
                            None
                        }
                    }
                    SdlEvent::ControllerButtonDown { which, button, .. } => {
                        if *which == active {
                            Some(GamepadEvent::ButtonDown(GamepadButtonDown {
                                which: GamepadId(active),
                                button: map_button(*button),
                            }))
                        } else {
                            None
                        }
                    }
                    SdlEvent::ControllerButtonUp { which, button, .. } => {
                        if *which == active {
                            Some(GamepadEvent::ButtonUp(GamepadButtonUp {
                                which: GamepadId(active),
                                button: map_button(*button),
                            }))
                        } else {
                            None
                        }
                    }
                    _ => None,
                }
            });
            *shared_events = Some(filtered_events.collect());
        } else {
            *shared_events = None;
        }
    }
}

impl IGamepads for Gamepads {
    fn get_accessor(&self) -> AnyArc<dyn IGamepadsAccessor> {
        AnyArc::map::<dyn IGamepadsAccessor, _>(self.accessor.clone(), |v| v)
    }
}

pub(crate) struct GamepadEntry {
    pub(crate) _device_index: u32,
    pub(crate) pad: Gamepad,
    pub(crate) state: GamepadState,
}

#[derive(Default)]
pub struct GamepadsAccessor {
    pub(crate) shared_state: RwLock<Option<GamepadState>>,
    pub(crate) shared_events: RwLock<Option<Vec<GamepadEvent>>>,
}

declare_interfaces!(GamepadsAccessor, [IGamepadsAccessor]);

impl IGamepadsAccessor for GamepadsAccessor {
    fn get_active_controller_state(&self) -> Option<GamepadState> {
        let reader = self.shared_state.read();
        reader.deref().clone()
    }

    fn get_active_controller_events(&self) -> Option<Vec<GamepadEvent>> {
        let reader = self.shared_events.read();
        reader.deref().clone()
    }
}

#[inline(always)]
/// The current state of the given axis.
fn set_axis(state: &mut GamepadState, axis: sdl3::gamepad::Axis, value: i16) {
    state.axis[axis as usize] = value;
}

#[inline(always)]
/// Lookup the state of the given button. True = 'pressed', false = 'released'
fn set_button(state: &mut GamepadState, button: sdl3::gamepad::Button, value: bool) {
    // Convert button to index in bitmap
    let button = 0b1u32 << (button as u32);

    if value {
        state.buttons |= button;
    } else {
        state.buttons &= !button;
    }
}

const fn map_button(button: sdl3::gamepad::Button) -> GamepadButton {
    match button {
        sdl3::gamepad::Button::South => GamepadButton::A,
        sdl3::gamepad::Button::East => GamepadButton::B,
        sdl3::gamepad::Button::West => GamepadButton::X,
        sdl3::gamepad::Button::North => GamepadButton::Y,
        sdl3::gamepad::Button::Back => GamepadButton::Back,
        sdl3::gamepad::Button::Guide => GamepadButton::Guide,
        sdl3::gamepad::Button::Start => GamepadButton::Start,
        sdl3::gamepad::Button::LeftStick => GamepadButton::LeftStick,
        sdl3::gamepad::Button::RightStick => GamepadButton::RightStick,
        sdl3::gamepad::Button::LeftShoulder => GamepadButton::LeftShoulder,
        sdl3::gamepad::Button::RightShoulder => GamepadButton::RightShoulder,
        sdl3::gamepad::Button::DPadUp => GamepadButton::DPadUp,
        sdl3::gamepad::Button::DPadDown => GamepadButton::DPadDown,
        sdl3::gamepad::Button::DPadLeft => GamepadButton::DPadLeft,
        sdl3::gamepad::Button::DPadRight => GamepadButton::DPadRight,
        sdl3::gamepad::Button::Misc1 => GamepadButton::Misc1,
        sdl3::gamepad::Button::RightPaddle1 => GamepadButton::RightPaddle1,
        sdl3::gamepad::Button::LeftPaddle1 => GamepadButton::LeftPaddle1,
        sdl3::gamepad::Button::RightPaddle2 => GamepadButton::RightPaddle2,
        sdl3::gamepad::Button::LeftPaddle2 => GamepadButton::LeftPaddle2,
        sdl3::gamepad::Button::Touchpad => GamepadButton::Touchpad,
        sdl3::gamepad::Button::Misc2 => GamepadButton::Misc2,
        sdl3::gamepad::Button::Misc3 => GamepadButton::Misc3,
        sdl3::gamepad::Button::Misc4 => GamepadButton::Misc4,
        sdl3::gamepad::Button::Misc5 => GamepadButton::Misc5,
    }
}

const fn map_axis(axis: sdl3::gamepad::Axis) -> GamepadAxis {
    match axis {
        sdl3::gamepad::Axis::LeftX => GamepadAxis::LeftX,
        sdl3::gamepad::Axis::LeftY => GamepadAxis::LeftY,
        sdl3::gamepad::Axis::RightX => GamepadAxis::RightX,
        sdl3::gamepad::Axis::RightY => GamepadAxis::RightY,
        sdl3::gamepad::Axis::TriggerLeft => GamepadAxis::TriggerLeft,
        sdl3::gamepad::Axis::TriggerRight => GamepadAxis::TriggerRight,
    }
}
