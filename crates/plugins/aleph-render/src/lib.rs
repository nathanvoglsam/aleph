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

#[cfg(target_os = "windows")]
extern crate aleph_dx12 as dx12;

#[cfg(target_os = "windows")]
extern crate aleph_dx12_alloc as dx12_alloc;

#[cfg(target_os = "windows")]
extern crate aleph_egui as egui;

#[cfg(target_os = "windows")]
extern crate aleph_interfaces as interfaces;

#[cfg(target_os = "windows")]
extern crate aleph_log as log;

#[cfg(target_os = "windows")]
extern crate aleph_pix as pix;

#[cfg(target_os = "windows")]
extern crate cstr;

#[cfg(target_os = "windows")]
mod plugin;

#[cfg(target_os = "windows")]
mod renderer;

#[cfg(target_os = "windows")]
mod shaders;

#[cfg(target_os = "windows")]
pub use plugin::PluginRenderDX12;
