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

use half::f16;

pub enum DynamicBuffer {
    U8(Vec<u8>),
    U16(Vec<u16>),
    U32(Vec<u32>),
    F16(Vec<f16>),
    F32(Vec<f32>),
}

impl DynamicBuffer {
    pub fn as_bytes(&self) -> &[u8] {
        match self {
            DynamicBuffer::U8(items) => bytemuck::cast_slice::<u8, u8>(items.as_slice()),
            DynamicBuffer::U16(items) => bytemuck::cast_slice::<u16, u8>(items.as_slice()),
            DynamicBuffer::U32(items) => bytemuck::cast_slice::<u32, u8>(items.as_slice()),
            DynamicBuffer::F16(items) => bytemuck::cast_slice::<f16, u8>(items.as_slice()),
            DynamicBuffer::F32(items) => bytemuck::cast_slice::<f32, u8>(items.as_slice()),
        }
    }
}
