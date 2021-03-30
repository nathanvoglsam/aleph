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

use crate::{dxgi, DepthStencilValue};
use std::mem::transmute;
use windows_raw::Win32::Direct3D12::{D3D12_CLEAR_VALUE, D3D12_CLEAR_VALUE_0};

#[derive(Clone, Debug)]
pub enum ClearValue {
    Color {
        format: dxgi::Format,
        color: [f32; 4],
    },
    Depth {
        format: dxgi::Format,
        depth_stencil: DepthStencilValue,
    },
}

impl ClearValue {
    pub fn format(&self) -> dxgi::Format {
        match self {
            ClearValue::Color { format, .. } => *format,
            ClearValue::Depth { format, .. } => *format,
        }
    }
}

impl Into<D3D12_CLEAR_VALUE> for ClearValue {
    fn into(self) -> D3D12_CLEAR_VALUE {
        match self {
            ClearValue::Color { format, color } => {
                assert!(!format.is_depth_stencil());
                D3D12_CLEAR_VALUE {
                    Format: format.into(),
                    Anonymous: D3D12_CLEAR_VALUE_0 { Color: color },
                }
            }
            ClearValue::Depth {
                format,
                depth_stencil,
            } => {
                assert!(format.is_depth_stencil());
                D3D12_CLEAR_VALUE {
                    Format: format.into(),
                    Anonymous: D3D12_CLEAR_VALUE_0 {
                        DepthStencil: unsafe { transmute(depth_stencil) },
                    },
                }
            }
        }
    }
}
