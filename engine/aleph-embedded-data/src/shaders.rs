//
//
// This file is a part of Aleph
//
// <ALEPH_REPO_REPLACE>
//
// <ALEPH_LICENSE_REPLACE>
//

///
/// Gets the raw spirv bytes for the imgui fragment shader
///
pub fn imgui_frag_shader() -> (&'static [u8], &'static [u32]) {
    macros::include_spirv_bytes!("../shaders/compiled/imgui/imgui.frag.spv")
}

///
/// Gets the raw spirv bytes for the imgui vertex shader
///
pub fn imgui_vert_shader() -> (&'static [u8], &'static [u32]) {
    macros::include_spirv_bytes!("../shaders/compiled/imgui/imgui.vert.spv")
}

///
/// Gets the raw spirv bytes for the standard fragment shader
///
pub fn standard_frag_shader() -> (&'static [u8], &'static [u32]) {
    macros::include_spirv_bytes!("../shaders/compiled/standard/standard.frag.spv")
}

///
/// Gets the raw spirv bytes for the standard vertex shader
///
pub fn standard_vert_shader() -> (&'static [u8], &'static [u32]) {
    macros::include_spirv_bytes!("../shaders/compiled/standard/standard.vert.spv")
}

///
/// Gets the raw spirv bytes for the standard vertex shader
///
pub fn tonemapping_frag_shader() -> (&'static [u8], &'static [u32]) {
    macros::include_spirv_bytes!("../shaders/compiled/postprocess/tonemapping.frag.spv")
}

///
/// Gets the raw spirv bytes for the standard vertex shader
///
pub fn fullscreen_quad_vert_shader() -> (&'static [u8], &'static [u32]) {
    macros::include_spirv_bytes!("../shaders/compiled/fullscreen_quad/fullscreen_quad.vert.spv")
}
