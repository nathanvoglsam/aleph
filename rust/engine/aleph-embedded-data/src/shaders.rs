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

///
/// Gets the raw spirv bytes for the imgui fragment shader
///
pub fn imgui_frag_shader() -> (&'static [u8], &'static [u32]) {
    aleph_macros::include_spirv_bytes!("../shaders/compiled/imgui/imgui.frag.spv")
}

///
/// Gets the raw spirv bytes for the imgui vertex shader
///
pub fn imgui_vert_shader() -> (&'static [u8], &'static [u32]) {
    aleph_macros::include_spirv_bytes!("../shaders/compiled/imgui/imgui.vert.spv")
}

///
/// Gets the raw spirv bytes for the egui fragment shader
///
pub fn egui_frag_shader() -> (&'static [u8], &'static [u32]) {
    aleph_macros::include_spirv_bytes!("../shaders/compiled/egui/egui.frag.spv")
}

///
/// Gets the raw spirv bytes for the egui vertex shader
///
pub fn egui_vert_shader() -> (&'static [u8], &'static [u32]) {
    aleph_macros::include_spirv_bytes!("../shaders/compiled/egui/egui.vert.spv")
}

///
/// Gets the raw spirv bytes for the standard fragment shader
///
pub fn standard_frag_shader() -> (&'static [u8], &'static [u32]) {
    aleph_macros::include_spirv_bytes!("../shaders/compiled/standard/standard.frag.spv")
}

///
/// Gets the raw spirv bytes for the standard vertex shader
///
pub fn standard_vert_shader() -> (&'static [u8], &'static [u32]) {
    aleph_macros::include_spirv_bytes!("../shaders/compiled/standard/standard.vert.spv")
}

///
/// Gets the raw spirv bytes for the standard vertex shader
///
pub fn tonemapping_frag_shader() -> (&'static [u8], &'static [u32]) {
    aleph_macros::include_spirv_bytes!("../shaders/compiled/postprocess/tonemapping.frag.spv")
}

///
/// Gets the raw spirv bytes for the standard vertex shader
///
pub fn fullscreen_quad_vert_shader() -> (&'static [u8], &'static [u32]) {
    aleph_macros::include_spirv_bytes!(
        "../shaders/compiled/fullscreen_quad/fullscreen_quad.vert.spv"
    )
}
