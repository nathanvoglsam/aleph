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

use crate::commands::img2ktx::{unorm_u16_to_f32, unorm_u8_to_f32};

pub enum DynamicBuffer {
    U8(Vec<u8>),
    U16(Vec<u16>),
    F16(Vec<f16>),
    F32(Vec<f32>),
}

impl DynamicBuffer {
    pub fn to_half(&mut self) {
        let new = match self {
            DynamicBuffer::U8(v) => {
                let old = std::mem::take(v);
                let new = Vec::from_iter(old.into_iter().map(|v| {
                    let v = unorm_u8_to_f32(v);
                    let v = f16::from_f32(v);
                    v
                }));
                new
            }
            DynamicBuffer::U16(v) => {
                let old = std::mem::take(v);
                let new = Vec::from_iter(old.into_iter().map(|v| {
                    let v = unorm_u16_to_f32(v);
                    let v = f16::from_f32(v);
                    v
                }));
                new
            }
            DynamicBuffer::F16(_) => {
                // We're already f16, so do nothing
                return;
            }
            DynamicBuffer::F32(v) => {
                let old = std::mem::take(v);
                let new = Vec::from_iter(old.into_iter().map(|v| {
                    let v = f16::from_f32(v);
                    v
                }));
                new
            }
        };
        *self = Self::F16(new);
    }

    pub fn to_little_endian(&mut self) {
        if !cfg!(target_endian = "big") {
            return;
        }

        match self {
            DynamicBuffer::U8(_) => {
                // Nothing to do here, u8 is the same in LE or BE
                return;
            }
            DynamicBuffer::U16(v) => {
                for p in v.iter_mut() {
                    *p = bytemuck::cast::<_, u16>(p.to_le_bytes());
                }
            }
            DynamicBuffer::F16(v) => {
                for p in v.iter_mut() {
                    *p = bytemuck::cast::<_, f16>(p.to_le_bytes());
                }
            }
            DynamicBuffer::F32(v) => {
                for p in v.iter_mut() {
                    *p = bytemuck::cast::<_, f32>(p.to_le_bytes());
                }
            }
        }
    }

    pub fn as_bytes(&self) -> &[u8] {
        match self {
            DynamicBuffer::U8(items) => bytemuck::cast_slice::<u8, u8>(items.as_slice()),
            DynamicBuffer::U16(items) => bytemuck::cast_slice::<u16, u8>(items.as_slice()),
            DynamicBuffer::F16(items) => bytemuck::cast_slice::<f16, u8>(items.as_slice()),
            DynamicBuffer::F32(items) => bytemuck::cast_slice::<f32, u8>(items.as_slice()),
        }
    }
}
