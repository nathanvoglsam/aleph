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

use std::any::Any;
use std::collections::HashMap;

///
/// A pure data struct that is used for describing the initial state of the OS window
///
/// This will usually be de-serialized from a file
///
#[derive(Debug)]
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
#[derive(Debug)]
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
