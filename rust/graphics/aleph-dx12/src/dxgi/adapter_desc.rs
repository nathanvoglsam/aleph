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

use windows_raw::Win32::Kernel::LUID;

#[repr(C)]
#[derive(Clone)]
pub struct AdapterDesc {
    pub description: [u16; 128usize],
    pub vendor_id: u32,
    pub device_id: u32,
    pub sub_sys_id: u32,
    pub revision: u32,
    pub dedicated_video_memory: usize,
    pub dedicated_system_memory: usize,
    pub shared_system_memory: usize,
    adapter_luid: LUID,
    flags: u32,
}

impl AdapterDesc {
    pub fn description_string(&self) -> Option<String> {
        let sub_slice = self.description.split(|v| *v == 0).next()?;
        String::from_utf16(sub_slice).ok()
    }

    pub fn vendor_id_string(&self) -> Option<&'static str> {
        if self.vendor_id == 0x10DE {
            Some("NVIDIA")
        } else if self.vendor_id == 0x1002 {
            Some("AMD")
        } else if self.vendor_id == 0x8086 {
            Some("INTEL")
        } else {
            None
        }
    }
}
