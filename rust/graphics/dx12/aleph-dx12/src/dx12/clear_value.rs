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
use windows_raw::win32::direct3d12::D3D12_DEPTH_STENCIL_VALUE;
use windows_raw::win32::dxgi::DXGI_FORMAT;
use std::mem::{transmute, ManuallyDrop};

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
                    format: format.into(),
                    variant: D3D12_CLEAR_VALUE_VARIANT { color },
                }
            }
            ClearValue::Depth {
                format,
                depth_stencil,
            } => {
                assert!(format.is_depth_stencil());
                D3D12_CLEAR_VALUE {
                    format: format.into(),
                    variant: D3D12_CLEAR_VALUE_VARIANT {
                        depth_stencil: ManuallyDrop::new(unsafe { transmute(depth_stencil) }),
                    },
                }
            }
        }
    }
}

#[repr(C)]
#[allow(non_camel_case_types)]
pub struct D3D12_CLEAR_VALUE {
    format: DXGI_FORMAT,
    variant: D3D12_CLEAR_VALUE_VARIANT,
}

#[repr(C)]
#[allow(non_camel_case_types)]
pub union D3D12_CLEAR_VALUE_VARIANT {
    color: [f32; 4],
    depth_stencil: ManuallyDrop<D3D12_DEPTH_STENCIL_VALUE>,
}
