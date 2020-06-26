//
//
// This file is a part of Aleph
//
// <ALEPH_REPO_REPLACE>
//
// <ALEPH_LICENSE_REPLACE>
//

mod app_logic;
mod engine;
mod frame_rate;
mod frame_timer;
mod imgui;
mod keyboard;
mod mouse;
mod settings;
mod thread_pools;
mod window;

pub use self::app_logic::AppLogic;
pub use self::engine::Engine;
pub use self::frame_rate::FrameRate;
pub use self::frame_timer::FrameTimer;
pub use self::imgui::ImguiStyleBuilder;
pub use self::keyboard::KeyDownEvent;
pub use self::keyboard::KeyUpEvent;
pub use self::keyboard::Keyboard;
pub use self::keyboard::KeyboardEvent;
pub use self::keyboard::KeyboardEvents;
pub use self::keyboard::KeyboardState;
pub use self::keyboard::KeyboardStateLock;
pub use self::keyboard::Keycode;
pub use self::keyboard::Mod;
pub use self::keyboard::Scancode;
pub use self::mouse::Mouse;
pub use self::mouse::MouseButton;
pub use self::mouse::MouseButtonDownEvent;
pub use self::mouse::MouseButtonUpEvent;
pub use self::mouse::MouseEvent;
pub use self::mouse::MouseMotionEvent;
pub use self::mouse::MouseState;
pub use self::mouse::MouseWheelDirection;
pub use self::mouse::MouseWheelEvent;
pub use self::settings::Settings;
pub use self::settings::WindowSettings;
pub use self::window::Window;

pub(crate) use self::imgui::Imgui;
pub(crate) use self::keyboard::KEYBOARD_EVENTS;
pub(crate) use self::keyboard::KEYBOARD_STATE;
pub(crate) use self::mouse::MOUSE_EVENTS;
pub(crate) use self::mouse::MOUSE_STATE;
pub(crate) use self::thread_pools::init_long_thread_pool;
pub(crate) use self::thread_pools::init_short_thread_pool;
pub(crate) use self::thread_pools::LONG_RUNNING_THREAD_POOL;
pub(crate) use self::thread_pools::SHORT_RUNNING_THREAD_POOL;
pub(crate) use self::window::WindowState;
pub(crate) use self::window::WINDOW_EVENTS;
pub(crate) use self::window::WINDOW_STATE;
