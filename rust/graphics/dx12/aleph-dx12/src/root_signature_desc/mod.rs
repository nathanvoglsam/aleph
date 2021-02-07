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
    D3D12_ROOT_SIGNATURE_FLAGS, D3D12_STATIC_SAMPLER_DESC,
};
use crate::versioned_root_signature_desc::raw::D3D12_VERSIONED_ROOT_SIGNATURE_DESC_VERSIONS;
use crate::{RootParameter, RootParameter1};
use std::mem::ManuallyDrop;

pub(crate) mod raw;

#[derive(Clone)]
pub struct RootSignatureDesc<'a> {
    pub parameters: &'a [RootParameter],
    pub static_samplers: &'a [D3D12_STATIC_SAMPLER_DESC],
    pub flags: D3D12_ROOT_SIGNATURE_FLAGS,
}

impl<'a> Into<raw::D3D12_ROOT_SIGNATURE_DESC> for RootSignatureDesc<'a> {
    fn into(self) -> raw::D3D12_ROOT_SIGNATURE_DESC {
        let num_parameters = self.parameters.len() as _;
        let p_parameters = self.parameters.as_ptr() as *mut RootParameter as *mut _;
        let num_static_samplers = self.static_samplers.len() as _;
        let p_static_samplers = self.static_samplers.as_ptr() as *mut _;
        let flags = self.flags;
        raw::D3D12_ROOT_SIGNATURE_DESC {
            num_parameters,
            p_parameters,
            num_static_samplers,
            p_static_samplers,
            flags,
        }
    }
}

impl<'a> Into<D3D12_VERSIONED_ROOT_SIGNATURE_DESC_VERSIONS> for RootSignatureDesc<'a> {
    fn into(self) -> D3D12_VERSIONED_ROOT_SIGNATURE_DESC_VERSIONS {
        D3D12_VERSIONED_ROOT_SIGNATURE_DESC_VERSIONS {
            version_1_0: ManuallyDrop::new(self.into()),
        }
    }
}

#[derive(Clone)]
pub struct RootSignatureDesc1<'a> {
    pub parameters: &'a [RootParameter1],
    pub static_samplers: &'a [D3D12_STATIC_SAMPLER_DESC],
    pub flags: D3D12_ROOT_SIGNATURE_FLAGS,
}

impl<'a> Into<raw::D3D12_ROOT_SIGNATURE_DESC1> for RootSignatureDesc1<'a> {
    fn into(self) -> raw::D3D12_ROOT_SIGNATURE_DESC1 {
        let num_parameters = self.parameters.len() as _;
        let p_parameters = self.parameters.as_ptr() as *mut RootParameter1 as *mut _;
        let num_static_samplers = self.static_samplers.len() as _;
        let p_static_samplers = self.static_samplers.as_ptr() as *mut _;
        let flags = self.flags;
        raw::D3D12_ROOT_SIGNATURE_DESC1 {
            num_parameters,
            p_parameters,
            num_static_samplers,
            p_static_samplers,
            flags,
        }
    }
}

impl<'a> Into<D3D12_VERSIONED_ROOT_SIGNATURE_DESC_VERSIONS> for RootSignatureDesc1<'a> {
    fn into(self) -> D3D12_VERSIONED_ROOT_SIGNATURE_DESC_VERSIONS {
        D3D12_VERSIONED_ROOT_SIGNATURE_DESC_VERSIONS {
            version_1_1: ManuallyDrop::new(self.into()),
        }
    }
}
