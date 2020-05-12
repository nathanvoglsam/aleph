//
//
// This file is a part of Aleph
//
// <ALEPH_REPO_REPLACE>
//
// <ALEPH_LICENSE_REPLACE>
//

pub struct WindowSettings {
    pub fullscreen: bool,
    pub width: u32,
    pub height: u32,
}

impl Default for WindowSettings {
    fn default() -> Self {
        WindowSettings {
            fullscreen: false,
            width: 1280,
            height: 720,
        }
    }
}

pub struct Settings {
    pub window: WindowSettings,
}
