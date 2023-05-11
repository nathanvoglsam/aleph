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

use ash::extensions::ext::DebugUtils;
use ash::vk;
use bumpalo::Bump;
use std::ffi::CStr;

pub fn set_name<T: vk::Handle>(
    loader: Option<&DebugUtils>,
    device: vk::Device,
    bump: &Bump,
    handle: T,
    name: Option<&str>,
) {
    // Do nothing if needed extension isn't loaded
    if let Some(loader) = loader {
        // Can only set a name if one is provided
        if let Some(name) = name {
            let iter = name.bytes().chain([0u8].into_iter());
            let name = bump.alloc_slice_fill_default(name.len() + 1);
            name.iter_mut().zip(iter).for_each(|(n, v)| {
                *n = v;
            });
            let name = unsafe { CStr::from_bytes_with_nul_unchecked(name) };

            let info = vk::DebugUtilsObjectNameInfoEXT::builder()
                .object_type(T::TYPE)
                .object_handle(handle.as_raw())
                .object_name(&name);
            unsafe {
                let _ = loader.set_debug_utils_object_name(device, &info).unwrap();
            }
        }
    }
}
