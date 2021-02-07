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

use crate::raw::windows::win32::direct3d12::{
    D3D12_ROOT_CONSTANTS, D3D12_ROOT_DESCRIPTOR, D3D12_ROOT_DESCRIPTOR1,
    D3D12_ROOT_DESCRIPTOR_TABLE, D3D12_ROOT_DESCRIPTOR_TABLE1, D3D12_ROOT_PARAMETER_TYPE,
    D3D12_SHADER_VISIBILITY,
};
use std::mem::ManuallyDrop;

#[repr(C)]
#[allow(non_camel_case_types)]
pub struct D3D12_ROOT_PARAMETER {
    pub parameter_type: D3D12_ROOT_PARAMETER_TYPE,
    pub types: D3D12_ROOT_PARAMETER_TYPES,
    pub shader_visibility: D3D12_SHADER_VISIBILITY,
}

#[repr(C)]
#[allow(non_camel_case_types)]
pub union D3D12_ROOT_PARAMETER_TYPES {
    pub descriptor_table: ManuallyDrop<D3D12_ROOT_DESCRIPTOR_TABLE>,
    pub constants: ManuallyDrop<D3D12_ROOT_CONSTANTS>,
    pub descriptor: ManuallyDrop<D3D12_ROOT_DESCRIPTOR>,
}

impl From<D3D12_ROOT_DESCRIPTOR_TABLE> for D3D12_ROOT_PARAMETER_TYPES {
    fn from(descriptor_table: D3D12_ROOT_DESCRIPTOR_TABLE) -> Self {
        Self {
            descriptor_table: ManuallyDrop::new(descriptor_table),
        }
    }
}

impl From<D3D12_ROOT_CONSTANTS> for D3D12_ROOT_PARAMETER_TYPES {
    fn from(constants: D3D12_ROOT_CONSTANTS) -> Self {
        Self {
            constants: ManuallyDrop::new(constants),
        }
    }
}

impl From<D3D12_ROOT_DESCRIPTOR> for D3D12_ROOT_PARAMETER_TYPES {
    fn from(descriptor: D3D12_ROOT_DESCRIPTOR) -> Self {
        Self {
            descriptor: ManuallyDrop::new(descriptor),
        }
    }
}

#[repr(C)]
#[allow(non_camel_case_types)]
pub struct D3D12_ROOT_PARAMETER1 {
    pub parameter_type: D3D12_ROOT_PARAMETER_TYPE,
    pub types: D3D12_ROOT_PARAMETER1_TYPES,
    pub shader_visibility: D3D12_SHADER_VISIBILITY,
}

#[repr(C)]
#[allow(non_camel_case_types)]
pub union D3D12_ROOT_PARAMETER1_TYPES {
    pub descriptor_table: ManuallyDrop<D3D12_ROOT_DESCRIPTOR_TABLE1>,
    pub constants: ManuallyDrop<D3D12_ROOT_CONSTANTS>,
    pub descriptor: ManuallyDrop<D3D12_ROOT_DESCRIPTOR1>,
}

impl From<D3D12_ROOT_DESCRIPTOR_TABLE1> for D3D12_ROOT_PARAMETER1_TYPES {
    fn from(descriptor_table: D3D12_ROOT_DESCRIPTOR_TABLE1) -> Self {
        Self {
            descriptor_table: ManuallyDrop::new(descriptor_table),
        }
    }
}

impl From<D3D12_ROOT_CONSTANTS> for D3D12_ROOT_PARAMETER1_TYPES {
    fn from(constants: D3D12_ROOT_CONSTANTS) -> Self {
        Self {
            constants: ManuallyDrop::new(constants),
        }
    }
}

impl From<D3D12_ROOT_DESCRIPTOR1> for D3D12_ROOT_PARAMETER1_TYPES {
    fn from(descriptor: D3D12_ROOT_DESCRIPTOR1) -> Self {
        Self {
            descriptor: ManuallyDrop::new(descriptor),
        }
    }
}
