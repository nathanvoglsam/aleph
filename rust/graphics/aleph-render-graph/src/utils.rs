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

use aleph_vulkan_core::erupt::vk1_0::AccessFlagBits;

///
/// Internal function for asserting that a given access type is valid for a read only image
///
#[inline]
pub fn debug_check_image_read_access_type(access: AccessFlagBits) {
    // Debug check if passing write accesses to a read only description
    debug_assert!(access != AccessFlagBits::MEMORY_WRITE);
    debug_assert!(access != AccessFlagBits::COLOR_ATTACHMENT_WRITE);
    debug_assert!(access != AccessFlagBits::TRANSFER_WRITE);
    debug_assert!(access != AccessFlagBits::DEPTH_STENCIL_ATTACHMENT_WRITE);
    debug_assert!(access != AccessFlagBits::HOST_WRITE);
    debug_assert!(access != AccessFlagBits::SHADER_WRITE);
    debug_assert!(access != AccessFlagBits::TRANSFORM_FEEDBACK_COUNTER_WRITE_EXT);
    debug_assert!(access != AccessFlagBits::TRANSFORM_FEEDBACK_WRITE_EXT);
    debug_assert!(access != AccessFlagBits::ACCELERATION_STRUCTURE_WRITE_KHR);
    debug_assert!(access != AccessFlagBits::ACCELERATION_STRUCTURE_WRITE_NV);
    debug_assert!(access != AccessFlagBits::COMMAND_PREPROCESS_WRITE_NV);
}

///
/// Internal function for asserting that a given access type is valid for an image to be used as
///
#[inline]
pub fn debug_check_image_access_type(access: AccessFlagBits) {
    // Debug check if passing invalid access type in
    debug_assert!(access != AccessFlagBits::INDEX_READ);
    debug_assert!(access != AccessFlagBits::UNIFORM_READ);
    debug_assert!(access != AccessFlagBits::VERTEX_ATTRIBUTE_READ);
    debug_assert!(access != AccessFlagBits::INDIRECT_COMMAND_READ);
    debug_assert!(access != AccessFlagBits::ACCELERATION_STRUCTURE_READ_KHR);
    debug_assert!(access != AccessFlagBits::ACCELERATION_STRUCTURE_READ_NV);
    debug_assert!(access != AccessFlagBits::COMMAND_PREPROCESS_READ_NV);
    debug_assert!(access != AccessFlagBits::TRANSFORM_FEEDBACK_COUNTER_READ_EXT);

    // Debug check if passing unsupported access type in
    debug_assert!(access != AccessFlagBits::FRAGMENT_DENSITY_MAP_READ_EXT);
    debug_assert!(access != AccessFlagBits::SHADING_RATE_IMAGE_READ_NV);
    debug_assert!(access != AccessFlagBits::COLOR_ATTACHMENT_READ_NONCOHERENT_EXT);
}

///
/// Internal function for asserting that a given access type is valid for a read only buffer
///
#[inline]
pub fn debug_check_buffer_read_access_type(access: AccessFlagBits) {
    // Debug check if passing write accesses to a read only description
    debug_assert!(access != AccessFlagBits::MEMORY_WRITE);
    debug_assert!(access != AccessFlagBits::COLOR_ATTACHMENT_WRITE);
    debug_assert!(access != AccessFlagBits::TRANSFER_WRITE);
    debug_assert!(access != AccessFlagBits::DEPTH_STENCIL_ATTACHMENT_WRITE);
    debug_assert!(access != AccessFlagBits::HOST_WRITE);
    debug_assert!(access != AccessFlagBits::SHADER_WRITE);
    debug_assert!(access != AccessFlagBits::TRANSFORM_FEEDBACK_COUNTER_WRITE_EXT);
    debug_assert!(access != AccessFlagBits::TRANSFORM_FEEDBACK_WRITE_EXT);
    debug_assert!(access != AccessFlagBits::ACCELERATION_STRUCTURE_WRITE_KHR);
    debug_assert!(access != AccessFlagBits::ACCELERATION_STRUCTURE_WRITE_NV);
    debug_assert!(access != AccessFlagBits::COMMAND_PREPROCESS_WRITE_NV);
}
