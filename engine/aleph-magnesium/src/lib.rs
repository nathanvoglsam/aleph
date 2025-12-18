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

//! # Magnesium
//!
//! Magnesium is the core of Aleph's renderer! Calling it the whole renderer would be misleading
//! as it only a piece of the puzzle, and is used cooperatively with higher layers to provide the
//! entire suite of functionality needed for rendering frames of a game. This library is primarily
//! concerned with abstracting rendering resources, and to turn a scene description into rendering
//! commands sent to the GPU.
//!
//! Magnesium is designed to live on its own thread, existing logically as the (mostly) consumer
//! side of a channel where the producer is some world simulation sends scene snapshots that are
//! then processed into GPU commands.

extern crate aleph_rhi_api as rhi;
extern crate aleph_shader_db as sdb;

pub mod async_resource_loader;
pub mod common;
pub(crate) mod internal;
pub mod material;
pub mod material_instance;
pub mod renderer;
pub mod resource;
pub mod resource_loader;
pub mod scene;
