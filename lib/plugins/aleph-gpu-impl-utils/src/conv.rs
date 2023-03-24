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

use aleph_interfaces::gpu::*;

#[inline(always)]
#[allow(clippy::erasing_op, clippy::identity_op)]
pub fn decode_u32_color_to_float(v: u32) -> [f32; 4] {
    let a = ((v >> (8 * 0)) & 0xFF) as f32 / 255.0;
    let b = ((v >> (8 * 1)) & 0xFF) as f32 / 255.0;
    let g = ((v >> (8 * 2)) & 0xFF) as f32 / 255.0;
    let r = ((v >> (8 * 3)) & 0xFF) as f32 / 255.0;
    [r, g, b, a]
}

pub const fn pci_id_to_vendor(v: u32) -> AdapterVendor {
    match v {
        0x1002 => AdapterVendor::AMD,
        0x1010 => AdapterVendor::ImaginationTechnology,
        0x10DE => AdapterVendor::NVIDIA,
        0x13B5 => AdapterVendor::ARM,
        0x5143 => AdapterVendor::Qualcomm,
        0x8086 => AdapterVendor::Intel,
        _ => AdapterVendor::Unknown,
    }
}
