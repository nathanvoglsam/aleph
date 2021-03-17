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

// =================================================================================================
// Crate Imports
// =================================================================================================

// Re-export useful crates
pub extern crate aleph_app_info as app_info;
pub extern crate aleph_egui as egui;
pub extern crate aleph_log as log;
pub extern crate aleph_platform as platform;
pub extern crate aleph_plugin_registry as plugin_registry;
pub extern crate rayon;

pub use plugin_registry::any;
pub use plugin_registry::interfaces;

extern crate aleph_dx12 as dx12;
extern crate aleph_dx12_alloc as dx12_alloc;
extern crate aleph_pix as pix;
extern crate aleph_render as render;
extern crate aleph_sdl2 as sdl2;
extern crate aleph_windows_raw as windows_raw;

// =================================================================================================
// Modules
// =================================================================================================

mod app_logic;
mod engine;
mod frame_rate;
mod thread_pools;

pub use self::app_logic::AppLogic;
pub use self::engine::Engine;
pub use self::frame_rate::FrameRate;
