//
//
// This file is a part of Aleph
//
// <ALEPH_REPO_REPLACE>
//
// <ALEPH_LICENSE_REPLACE>
//

use std::any::Any;
use std::collections::HashMap;

///
/// A pure data struct that is used for describing the initial state of the OS window
///
/// This will usually be de-serialized from a file
///
pub struct WindowSettings {
    ///
    /// Will the window start fullscreen
    ///
    pub fullscreen: bool,

    ///
    /// Initial width of the window
    ///
    pub width: u32,

    ///
    /// Initial height of the window
    ///
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

///
/// A pure data struct that is used for describing initial engine settings, such as initial window
/// state or graphics options,
///
pub struct Settings {
    ///
    /// The initial state of the window
    ///
    pub window: WindowSettings,

    ///
    /// This member is for injecting your own settings into the engine's own preferences storage
    /// system. This would generally be used for custom preferences specific to your game
    ///
    pub other: HashMap<String, Box<dyn Any>>,
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            window: WindowSettings::default(),
            other: HashMap::new(),
        }
    }
}
