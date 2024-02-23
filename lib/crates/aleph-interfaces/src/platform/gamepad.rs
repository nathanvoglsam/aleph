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

///
/// This interface represents the API expected of something that gives the engine access to a
/// device's gamepads.
///
pub trait IGamepads: IAny + Send + Sync + 'static {
    /// Gets the current state of the gamepad this frame.
    fn get_state(&self, _todo_identifier: u32) -> GamepadState;

    ///
    /// Get read only access to this frame's list of gamepad events.
    ///
    /// # Warning
    ///
    /// This will probably lock an RwLock so trying to hold on to this between frames will likely
    /// deadlock the engine.
    ///
    fn events<'a>(&'a self) -> Box<dyn IGamepadEventsLock + 'a>;
}

///
/// This interface is used to provide access to the list of mouse events for the current frame.
///
/// Some implementations may need to lock a mutex or read/write lock to provide access to the list
/// safely so this interface is passed to wrap the lock guard
///
pub trait IGamepadEventsLock {    
    fn events(&self) -> &[GamepadEvent];
}

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
    pub which: u32,
    pub axis: GamepadAxis,
    pub value: i16,
}

#[derive(Clone, PartialEq, Debug)]
pub struct GamepadButtonDown {
    pub which: u32,
    pub button: GamepadButton,
}

#[derive(Clone, PartialEq, Debug)]
pub struct GamepadButtonUp {
    pub which: u32,
    pub button: GamepadButton,
}

#[derive(Clone, PartialEq, Debug)]
pub struct GamepadDeviceAdded {
    pub which: u32,
}

#[derive(Clone, PartialEq, Debug)]
pub struct GamepadDeviceRemoved {
    pub which: u32,
}

#[derive(Clone, PartialEq, Debug)]
pub struct GamepadDeviceRemapped {
    pub which: u32,
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
