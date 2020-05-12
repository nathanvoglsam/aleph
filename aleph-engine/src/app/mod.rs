//
//
// This file is a part of Aleph
//
// <ALEPH_REPO_REPLACE>
//
// <ALEPH_LICENSE_REPLACE>
//

mod app_info;
mod app_logic;
mod engine;
mod settings;
mod thread_pools;
mod window;

pub use app_info::AppInfo;
pub use app_logic::AppLogic;
pub use engine::Engine;
pub use engine::ENGINE_NAME;
pub use engine::ENGINE_VERSION_MAJOR;
pub use engine::ENGINE_VERSION_MINOR;
pub use engine::ENGINE_VERSION_PATCH;
pub use engine::ENGINE_VERSION_STRING;
pub use engine::ENGINE_VERSION_VK;
pub use settings::Settings;
pub use settings::WindowSettings;
pub use window::Window;

pub(crate) use thread_pools::init_long_thread_pool;
pub(crate) use thread_pools::init_short_thread_pool;
pub(crate) use thread_pools::LONG_RUNNING_THREAD_POOL;
pub(crate) use thread_pools::SHORT_RUNNING_THREAD_POOL;
pub(crate) use window::WindowState;
pub(crate) use window::WINDOW_EVENTS;
pub(crate) use window::WINDOW_STATE;
