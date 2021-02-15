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

use crate::RootDescriptorFlags;
use raw::windows::win32::direct3d12::D3D12_ROOT_DESCRIPTOR;
use raw::windows::win32::direct3d12::D3D12_ROOT_DESCRIPTOR1;

#[derive(Clone, Debug)]
pub struct RootDescriptor {
    pub shader_register: u32,
    pub register_space: u32,
}

impl Into<D3D12_ROOT_DESCRIPTOR> for RootDescriptor {
    fn into(self) -> D3D12_ROOT_DESCRIPTOR {
        D3D12_ROOT_DESCRIPTOR {
            shader_register: self.shader_register,
            register_space: self.register_space,
        }
    }
}

#[derive(Clone, Debug)]
pub struct RootDescriptor1 {
    pub shader_register: u32,
    pub register_space: u32,
    pub flags: RootDescriptorFlags,
}

impl Into<D3D12_ROOT_DESCRIPTOR1> for RootDescriptor1 {
    fn into(self) -> D3D12_ROOT_DESCRIPTOR1 {
        D3D12_ROOT_DESCRIPTOR1 {
            shader_register: self.shader_register,
            register_space: self.register_space,
            flags: self.flags.into(),
        }
    }
}
