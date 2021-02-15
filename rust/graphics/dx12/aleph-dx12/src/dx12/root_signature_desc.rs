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

use crate::dx12::root_parameter::{D3D12_ROOT_PARAMETER, D3D12_ROOT_PARAMETER1};
use crate::raw::windows::win32::direct3d12::{
    D3D12_ROOT_SIGNATURE_FLAGS, D3D12_STATIC_SAMPLER_DESC,
};
use std::marker::PhantomData;

pub struct RootSignatureDescBuilder {
    parameters: Vec<D3D12_ROOT_PARAMETER>,
    static_samplers: Vec<D3D12_STATIC_SAMPLER_DESC>,
    flags: D3D12_ROOT_SIGNATURE_FLAGS,
}

impl RootSignatureDescBuilder {
    pub fn new() -> Self {
        Self {
            parameters: vec![],
            static_samplers: vec![],
            flags: Default::default()
        }
    }

    pub fn build(&self) -> RootSignatureDesc {
        RootSignatureDesc {
            inner: D3D12_ROOT_SIGNATURE_DESC {
                num_parameters: self.parameters.len() as _,
                p_parameters: self.parameters.as_ptr() as *mut _,
                num_static_samplers: self.static_samplers.len() as _,
                p_static_samplers: self.static_samplers.as_ptr() as *mut _,
                flags: self.flags
            },
            phantom: Default::default()
        }
    }
}

pub struct RootSignatureDesc1Builder {
    parameters: Vec<D3D12_ROOT_PARAMETER1>,
    static_samplers: Vec<D3D12_STATIC_SAMPLER_DESC>,
    flags: D3D12_ROOT_SIGNATURE_FLAGS,
}

impl RootSignatureDesc1Builder {
    pub fn new() -> Self {
        Self {
            parameters: vec![],
            static_samplers: vec![],
            flags: Default::default()
        }
    }
    
    pub fn build(&self) -> RootSignatureDesc1 {
        RootSignatureDesc1 {
            inner: D3D12_ROOT_SIGNATURE_DESC1 {
                num_parameters: self.parameters.len() as _,
                p_parameters: self.parameters.as_ptr() as *mut _,
                num_static_samplers: self.static_samplers.len() as _,
                p_static_samplers: self.static_samplers.as_ptr() as *mut _,
                flags: self.flags
            },
            phantom: Default::default()
        }
    }
}

pub struct RootSignatureDesc<'a> {
    inner: D3D12_ROOT_SIGNATURE_DESC,
    phantom: PhantomData<&'a ()>,
}

pub struct RootSignatureDesc1<'a> {
    inner: D3D12_ROOT_SIGNATURE_DESC1,
    phantom: PhantomData<&'a ()>,
}

#[repr(C)]
#[allow(non_camel_case_types)]
pub struct D3D12_ROOT_SIGNATURE_DESC {
    pub num_parameters: u32,
    pub p_parameters: *mut D3D12_ROOT_PARAMETER,
    pub num_static_samplers: u32,
    pub p_static_samplers: *mut D3D12_STATIC_SAMPLER_DESC,
    pub flags: D3D12_ROOT_SIGNATURE_FLAGS,
}

#[repr(C)]
#[allow(non_camel_case_types)]
pub struct D3D12_ROOT_SIGNATURE_DESC1 {
    pub num_parameters: u32,
    pub p_parameters: *mut D3D12_ROOT_PARAMETER1,
    pub num_static_samplers: u32,
    pub p_static_samplers: *mut D3D12_STATIC_SAMPLER_DESC,
    pub flags: D3D12_ROOT_SIGNATURE_FLAGS,
}
