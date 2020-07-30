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
pub extern crate aleph_imgui as imgui;
pub extern crate aleph_log as log;
pub extern crate aleph_platform as platform;
pub extern crate aleph_render as render;
pub extern crate aleph_vulkan as vulkan;
pub extern crate aleph_vulkan_alloc as vulkan_alloc;
pub extern crate aleph_vulkan_core as vulkan_core;
pub extern crate rayon;

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
