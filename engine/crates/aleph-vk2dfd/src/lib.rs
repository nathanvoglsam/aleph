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

use crate::table::VK_TO_DFD;
use thiserror::Error;

mod table;

#[cfg(test)]
mod test;

/// A Vulkan format, one of the `VK_FORMAT_` constants.
/// 
/// This is just a type alias for `u32` for maximum compatibility with the Rust Vulkan ecosystem.
pub type VkFormat = u32;

/// Errors that can occur when translating a Vulkan format into a DFD.
#[derive(Error, Debug)]
pub enum Error {
    /// The Vulkan type is unknown.
    #[error("No DFD is available for that Vulkan type")]
    UnknownType,
}

/// Converts a Vulkan format to the appropriate descriptor.
/// 
/// Returns the descriptor as a static slice of `u32`s, or an [enum@Error] on
/// failure. If written to a KTX or KTX2 file, the descriptor must be written in
/// little endian byte order per the specification. The slice includes the
/// leading size word.
#[inline]
pub fn vk2dfd(vulkan_format: VkFormat) -> Result<&'static [u32], Error> {
    VK_TO_DFD
        .get(&vulkan_format)
        .cloned()
        .ok_or(Error::UnknownType)
}
