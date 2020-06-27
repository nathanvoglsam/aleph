//
//
// This file is a part of Aleph
//
// <ALEPH_REPO_REPLACE>
//
// <ALEPH_LICENSE_REPLACE>
//

use once_cell::sync::Lazy;
use parking_lot::{Mutex, RwLock};
use sdl2::event::Event;
use sdl2::mouse::SystemCursor;
use std::collections::HashMap;
use std::convert::TryFrom;
use std::ops::Deref;

///
/// The internal global mouse state
///
pub(crate) struct InternalMouseState {
    pos: (i32, i32),
    buttons: u32,
}

///
/// Internal enum for representing a mouse request
///
pub(crate) enum MouseRequest {
    SetPos(i32, i32),
    SetCursor(Cursor),
    ShowCursor,
    HideCursor,
}

///
/// The global mouse state
///
pub(crate) static MOUSE_STATE: Lazy<RwLock<Option<InternalMouseState>>> =
    Lazy::new(|| RwLock::new(None));

///
/// The global mouse request queue
///
pub(crate) static MOUSE_REQUEST_QUEUE: Lazy<Mutex<Option<Vec<MouseRequest>>>> =
    Lazy::new(|| Mutex::new(None));

///
/// The global mouse events
///
pub(crate) static MOUSE_EVENTS: Lazy<RwLock<Option<Vec<MouseEvent>>>> =
    Lazy::new(|| RwLock::new(None));

///
/// Represents the set of cursors available
///
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Debug, Hash)]
pub enum Cursor {
    Arrow,
    IBeam,
    SizeAll,
    SizeNS,
    SizeWE,
    SizeNESW,
    SizeNWSE,
    Hand,
    No,
}

impl Cursor {
    pub(crate) fn load_sdl_cursor(&self) -> sdl2::mouse::Cursor {
        let cursor: SystemCursor = (*self).into();
        sdl2::mouse::Cursor::from_system(cursor).unwrap()
    }
}

impl Into<SystemCursor> for Cursor {
    fn into(self) -> SystemCursor {
        match self {
            Cursor::Arrow => SystemCursor::Arrow,
            Cursor::IBeam => SystemCursor::IBeam,
            Cursor::SizeAll => SystemCursor::SizeAll,
            Cursor::SizeNS => SystemCursor::SizeNS,
            Cursor::SizeWE => SystemCursor::SizeWE,
            Cursor::SizeNESW => SystemCursor::SizeNESW,
            Cursor::SizeNWSE => SystemCursor::SizeNWSE,
            Cursor::Hand => SystemCursor::Hand,
            Cursor::No => SystemCursor::No,
        }
    }
}

impl Into<usize> for Cursor {
    fn into(self) -> usize {
        match self {
            Cursor::Arrow => 0,
            Cursor::IBeam => 1,
            Cursor::SizeAll => 2,
            Cursor::SizeNS => 3,
            Cursor::SizeWE => 4,
            Cursor::SizeNESW => 5,
            Cursor::SizeNWSE => 6,
            Cursor::Hand => 7,
            Cursor::No => 8,
        }
    }
}

impl TryFrom<usize> for Cursor {
    type Error = ();

    fn try_from(value: usize) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Cursor::Arrow),
            1 => Ok(Cursor::IBeam),
            2 => Ok(Cursor::SizeAll),
            3 => Ok(Cursor::SizeNS),
            4 => Ok(Cursor::SizeWE),
            5 => Ok(Cursor::SizeNESW),
            6 => Ok(Cursor::SizeNWSE),
            7 => Ok(Cursor::Hand),
            8 => Ok(Cursor::No),
            _ => Err(()),
        }
    }
}

///
/// The different types of
///
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub enum MouseButton {
    Left,
    Middle,
    Right,
    X1,
    X2,
}

///
/// Mouse wheel direction
///
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub enum MouseWheelDirection {
    Normal,
    Flipped,
}

///
/// A mouse motion event
///
pub struct MouseMotionEvent {
    pub mousestate: MouseState,
    pub x: i32,
    pub y: i32,
    pub xrel: i32,
    pub yrel: i32,
}

///
/// A mouse button down event
///
pub struct MouseButtonDownEvent {
    pub button: MouseButton,
    pub clicks: u8,
    pub x: i32,
    pub y: i32,
}

///
/// A mouse button up event
///
pub struct MouseButtonUpEvent {
    pub button: MouseButton,
    pub clicks: u8,
    pub x: i32,
    pub y: i32,
}

///
/// A mouse wheel event
///
pub struct MouseWheelEvent {
    pub x: i32,
    pub y: i32,
    pub direction: MouseWheelDirection,
}

///
/// A mouse event
///
pub enum MouseEvent {
    MouseMotion(MouseMotionEvent),
    MouseButtonDown(MouseButtonDownEvent),
    MouseButtonUp(MouseButtonUpEvent),
    MouseWheel(MouseWheelEvent),
}

///
/// Represents the state of the mouse this frame
///
pub struct MouseState {
    pos: (i32, i32),
    buttons: u32,
}

impl MouseState {
    ///
    /// Get the position of the mouse
    ///
    pub fn pos(&self) -> (i32, i32) {
        self.pos
    }

    ///
    /// Is the left mouse button down
    ///
    pub fn left(&self) -> bool {
        (self.buttons & self.button_mask(sdl2::sys::SDL_BUTTON_LEFT)) != 0
    }

    ///
    /// Is the middle mouse button down
    ///
    pub fn middle(&self) -> bool {
        (self.buttons & self.button_mask(sdl2::sys::SDL_BUTTON_MIDDLE)) != 0
    }

    ///
    /// Is the right mouse button down
    ///
    pub fn right(&self) -> bool {
        (self.buttons & self.button_mask(sdl2::sys::SDL_BUTTON_RIGHT)) != 0
    }

    ///
    /// Is mouse button x1 (mouse button 4) down
    ///
    pub fn x1(&self) -> bool {
        (self.buttons & self.button_mask(sdl2::sys::SDL_BUTTON_X1)) != 0
    }

    ///
    /// Is mouse button x2 (mouse button 5) down
    ///
    pub fn x2(&self) -> bool {
        (self.buttons & self.button_mask(sdl2::sys::SDL_BUTTON_X2)) != 0
    }

    fn button_mask(&self, button: u32) -> u32 {
        1 << (button - 1)
    }
}

///
/// A wrapper around a read guard on the underlying RwLock used to make the global mouse events list
/// thread safe.
///
/// # Warning
///
/// Do not try and hold onto this between frames, it will deadlock the engine.
///
pub struct MouseEvents {
    lock: parking_lot::RwLockReadGuard<'static, Option<Vec<MouseEvent>>>,
}

impl Deref for MouseEvents {
    type Target = [MouseEvent];

    fn deref(&self) -> &Self::Target {
        self.lock.as_ref().unwrap().as_slice()
    }
}

///
/// A "namespace" struct similar to the `Engine` struct that is used to encapsulate the global mouse
/// state.
///
/// # Implementation Details
///
/// See the documentation for `Window`. This struct follows the same paradigm of queueing changes
/// for next frame as `Window`
///
pub struct Mouse {}

impl Mouse {
    ///
    /// Internal function for initializing the global mouse state
    ///
    pub(crate) fn init() {
        aleph_log::trace!("Initializing the Mouse system");
        let mouse_state = InternalMouseState {
            pos: (0, 0),
            buttons: 0,
        };

        *MOUSE_STATE.write() = Some(mouse_state);
        *MOUSE_REQUEST_QUEUE.lock() = Some(Vec::new());
        *MOUSE_EVENTS.write() = Some(Vec::new());
        aleph_log::trace!("");
    }

    ///
    /// Internal function for handling requests made in the last frame
    ///
    pub(crate) fn process_mouse_requests(
        window: &sdl2::video::Window,
        mouse_utils: &sdl2::mouse::MouseUtil,
        cursors: &HashMap<Cursor, sdl2::mouse::Cursor>,
    ) {
        let mut mouse_requests_lock = MOUSE_REQUEST_QUEUE.lock();
        let mouse_requests = mouse_requests_lock.as_mut().unwrap();

        for request in mouse_requests.drain(..) {
            match request {
                MouseRequest::SetPos(x, y) => {
                    aleph_log::trace!("Attempting to set new mouse position");
                    mouse_utils.warp_mouse_in_window(window, x as i32, y as i32);
                    aleph_log::trace!("Moved mouse to : {}, {}", x, y);
                }
                MouseRequest::SetCursor(cursor) => {
                    let cursor = cursors.get(&cursor).unwrap();
                    cursor.set();
                }
                MouseRequest::ShowCursor => {
                    mouse_utils.show_cursor(true);
                }
                MouseRequest::HideCursor => {
                    mouse_utils.show_cursor(false);
                }
            }
        }
    }

    ///
    /// Internal function for handling the events produced by the OS
    ///
    pub(crate) fn process_mouse_event(
        mouse_events: &mut Vec<MouseEvent>,
        event: sdl2::event::Event,
    ) {
        match event {
            Event::MouseMotion {
                mousestate,
                x,
                y,
                xrel,
                yrel,
                ..
            } => {
                let mousestate = MouseState {
                    pos: (mousestate.x(), mousestate.y()),
                    buttons: mousestate.to_sdl_state(),
                };
                let event = MouseMotionEvent {
                    mousestate,
                    x,
                    y,
                    xrel,
                    yrel,
                };
                mouse_events.push(MouseEvent::MouseMotion(event))
            }
            Event::MouseButtonDown {
                mouse_btn,
                clicks,
                x,
                y,
                ..
            } => {
                let button = match mouse_btn {
                    sdl2::mouse::MouseButton::Unknown => return,
                    sdl2::mouse::MouseButton::Left => MouseButton::Left,
                    sdl2::mouse::MouseButton::Middle => MouseButton::Middle,
                    sdl2::mouse::MouseButton::Right => MouseButton::Right,
                    sdl2::mouse::MouseButton::X1 => MouseButton::X1,
                    sdl2::mouse::MouseButton::X2 => MouseButton::X2,
                };
                let event = MouseButtonDownEvent {
                    button,
                    clicks,
                    x,
                    y,
                };
                mouse_events.push(MouseEvent::MouseButtonDown(event))
            }
            Event::MouseButtonUp {
                mouse_btn,
                clicks,
                x,
                y,
                ..
            } => {
                let button = match mouse_btn {
                    sdl2::mouse::MouseButton::Unknown => return,
                    sdl2::mouse::MouseButton::Left => MouseButton::Left,
                    sdl2::mouse::MouseButton::Middle => MouseButton::Middle,
                    sdl2::mouse::MouseButton::Right => MouseButton::Right,
                    sdl2::mouse::MouseButton::X1 => MouseButton::X1,
                    sdl2::mouse::MouseButton::X2 => MouseButton::X2,
                };
                let event = MouseButtonUpEvent {
                    button,
                    clicks,
                    x,
                    y,
                };
                mouse_events.push(MouseEvent::MouseButtonUp(event))
            }
            Event::MouseWheel {
                x, y, direction, ..
            } => {
                let direction = match direction {
                    sdl2::mouse::MouseWheelDirection::Normal => MouseWheelDirection::Normal,
                    sdl2::mouse::MouseWheelDirection::Flipped => MouseWheelDirection::Flipped,
                    sdl2::mouse::MouseWheelDirection::Unknown(_) => return,
                };
                let event = MouseWheelEvent { x, y, direction };
                mouse_events.push(MouseEvent::MouseWheel(event))
            }
            _ => {}
        }
    }

    ///
    /// Internal function for getting new mouse state from SDL2
    ///
    pub(crate) fn update_state(event_pump: &sdl2::EventPump) {
        let state = sdl2::mouse::MouseState::new(event_pump);

        let mut mouse_state_lock = MOUSE_STATE.write();
        let mouse_state = mouse_state_lock.as_mut();
        let mouse_state = mouse_state.expect("Mouse system not initialized");

        mouse_state.pos = (state.x(), state.y());
        mouse_state.buttons = state.to_sdl_state();
    }

    ///
    /// Get the current state of the mouse, last updated at the beginning of the frame
    ///
    pub fn get_state() -> MouseState {
        let mouse_lock = MOUSE_STATE.read();
        let mouse = mouse_lock.as_ref().expect("Mouse system not initialized");
        MouseState {
            pos: mouse.pos,
            buttons: mouse.buttons,
        }
    }

    ///
    /// Set the position of the mouse
    ///
    pub fn set_pos(x: i32, y: i32) {
        MOUSE_REQUEST_QUEUE
            .lock()
            .as_mut()
            .expect("Mouse system not initialized")
            .push(MouseRequest::SetPos(x, y));
    }

    ///
    /// Sets the mouse cursor
    ///
    pub fn set_cursor(cursor: Cursor) {
        MOUSE_REQUEST_QUEUE
            .lock()
            .as_mut()
            .expect("Mouse system not initialized")
            .push(MouseRequest::SetCursor(cursor));
    }

    ///
    /// Makes the cursor visible
    ///
    pub fn show_cursor() {
        MOUSE_REQUEST_QUEUE
            .lock()
            .as_mut()
            .expect("Mouse system not initialized")
            .push(MouseRequest::ShowCursor);
    }

    ///
    /// Makes the cursor invisible
    ///
    pub fn hide_cursor() {
        MOUSE_REQUEST_QUEUE
            .lock()
            .as_mut()
            .expect("Mouse system not initialized")
            .push(MouseRequest::HideCursor);
    }

    ///
    /// Get read only access to this frame's list of mouse events.
    ///
    /// # Warning
    ///
    /// This will lock a global RwLock so trying to hold on to this between frames will deadlock the
    /// engine.
    ///
    pub fn events() -> MouseEvents {
        let lock = MOUSE_EVENTS.read();
        MouseEvents { lock }
    }
}
