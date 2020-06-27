//
//
// This file is a part of Aleph
//
// <ALEPH_REPO_REPLACE>
//
// <ALEPH_LICENSE_REPLACE>
//

pub mod clipboard;
pub mod frame_timer;
pub mod keyboard;
pub mod mouse;
pub mod window;

mod platform;

pub use platform::Platform;
pub use platform::PlatformBuildError;
pub use platform::PlatformBuilder;
