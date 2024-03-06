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

//!
//! The gamepad interface provides a high-level API for accessing gamepad input across all
//! platforms. This API is designed assuming only single-player gamepad usage. That is, we only
//! expose a single logical 'gamepad' that is internally managed by the implementation, even in the
//! presence of multiple gamepads connected to the host.
//!
//! This simplification is a pragmatic choice based on aleph-engine's intended usecases which do not
//! include local multiplayer gameplay.
//!

use any::*;

///
/// This interface should be used by plugins that wish to register themselves as the engine's mouse
/// provider. Anything that implements this should correctly handle creating and destroying whatever
/// is needed to access the system's mouse, and should be able to give out an `AnyArc<IMouse>` to
/// allow others to retrieve information about and manipulate the mouse.
///
pub trait IGamepadsProvider: IAny + 'static {
    ///
    /// Returns an [AnyArc] that holds an [IGamepads] interface.
    ///
    /// This will always return the same [IGamepads] instance as [IGamepadsProvider] only supports
    /// handling a single mouse device.
    ///
    /// A return value of `None` should signal that the functionality is not supported.
    ///
    fn get_gamepads(&self) -> Option<AnyArc<dyn IGamepads>>;
}

/// This interface represents the API expected of something that gives the engine access to a
/// device's gamepads.
pub trait IGamepads: IAny + 'static {
    /// Returns a thread-safe accessor to the gamepad state as recorded from the most recently
    /// executed input polling cycle (i.e most recent frame).
    fn get_accessor(&self) -> AnyArc<dyn IGamepadsAccessor>;
}

/// A thread-safe, sharable accessor to the gamepad state.
pub trait IGamepadsAccessor: IAny + Send + Sync + 'static {
    /// Returns the current state of the active gamepad as recorded for this frame. May return
    /// [None] if no gamepad is connected.
    fn get_active_controller_state(&self) -> Option<GamepadState>;

    /// Returns a list of events as recorded this frame for the active game controller. May return
    /// [None] if no gamepad is connected.
    ///
    /// This will contain only the events for the active controller.
    fn get_active_controller_events(&self) -> Option<Vec<GamepadEvent>>;
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
#[repr(transparent)]
pub struct GamepadId(pub u32);

#[derive(Clone, PartialEq, Debug)]
pub enum GamepadEvent {
    AxisMotion(GamepadAxisMotion),
    ButtonDown(GamepadButtonDown),
    ButtonUp(GamepadButtonUp),
    DeviceAdded(GamepadDeviceAdded),
    DeviceRemoved(GamepadDeviceRemoved),
    DeviceRemapped(GamepadDeviceRemapped),
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub enum GamepadAxis {
    LeftX,
    LeftY,
    RightX,
    RightY,
    TriggerLeft,
    TriggerRight,
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub enum GamepadButton {
    A,
    B,
    X,
    Y,
    Back,
    Guide,
    Start,
    LeftStick,
    RightStick,
    LeftShoulder,
    RightShoulder,
    DPadUp,
    DPadDown,
    DPadLeft,
    DPadRight,
    /// Additional button (e.g. Xbox Series X share button, PS5 microphone button, Nintendo Switch
    /// Pro capture button, Amazon Luna microphone button, Google Stadia capture button)
    Misc1,
    /// Upper or primary paddle, under your right hand (e.g. Xbox Elite paddle P1)
    RightPaddle1,
    /// Upper or primary paddle, under your left hand (e.g. Xbox Elite paddle P3)
    LeftPaddle1,
    /// Lower or secondary paddle, under your right hand (e.g. Xbox Elite paddle P2)
    RightPaddle2,
    /// Lower or secondary paddle, under your left hand (e.g. Xbox Elite paddle P4)
    LeftPaddle2,
    /// PS4/PS5 touchpad button
    Touchpad,
    /// Additional button
    Misc2,
    /// Additional button
    Misc3,
    /// Additional button
    Misc4,
    /// Additional button
    Misc5,
    /// Additional button
    Misc6,
}

#[derive(Clone, PartialEq, Debug)]
pub struct GamepadAxisMotion {
    /// The ID of the gamepad this event was emitted from
    pub which: GamepadId,

    /// The Axis this event is emitted from
    pub axis: GamepadAxis,

    /// The axis value, at the time of this event being emitted
    pub value: i16,
}

#[derive(Clone, PartialEq, Debug)]
pub struct GamepadButtonDown {
    /// The ID of the gamepad this event was emitted from
    pub which: GamepadId,

    /// The button that has been pressed
    pub button: GamepadButton,
}

#[derive(Clone, PartialEq, Debug)]
pub struct GamepadButtonUp {
    /// The ID of the gamepad this event was emitted from
    pub which: GamepadId,

    /// The button that has been released
    pub button: GamepadButton,
}

#[derive(Clone, PartialEq, Debug)]
pub struct GamepadDeviceAdded {
    /// The ID of the gamepad this event was emitted from
    pub which: GamepadId,
}

#[derive(Clone, PartialEq, Debug)]
pub struct GamepadDeviceRemoved {
    /// The ID of the gamepad this event was emitted from
    pub which: GamepadId,
}

#[derive(Clone, PartialEq, Debug)]
pub struct GamepadDeviceRemapped {
    /// The ID of the gamepad this event was emitted from
    pub which: GamepadId,
}

///
/// Represents the state of a gamepad this frame
///
#[derive(Clone, Default, Debug)]
pub struct GamepadState {
    /// The current state of the six different axis, indexed by [GamepadAxis].
    pub axis: [i16; 6],

    /// The current down/up state of the gamepad buttons as a bit mask, indexed by [GamepadButton].
    pub buttons: u32,
}

impl GamepadState {
    /// The current state of the given axis.
    pub const fn axis(&self, axis: GamepadAxis) -> i16 {
        self.axis[axis as usize]
    }

    /// Lookup the state of the given button. True = 'pressed', false = 'released'
    pub const fn button(&self, button: GamepadButton) -> bool {
        // Convert button to index in bitmap
        let button = 0b1u32 << (button as u32);

        (self.buttons & button) != 0
    }
}
