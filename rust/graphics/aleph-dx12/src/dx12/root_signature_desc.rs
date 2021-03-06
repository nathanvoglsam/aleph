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
use crate::windows_raw::win32::direct3d12::{
    D3D12_DESCRIPTOR_RANGE, D3D12_DESCRIPTOR_RANGE1, D3D12_ROOT_SIGNATURE_FLAGS,
    D3D12_STATIC_SAMPLER_DESC,
};
use crate::{
    RootParameter, RootParameter1, RootSignatureFlags, StaticSamplerDesc,
    VersionedRootSignatureDesc,
};
use std::marker::PhantomData;
use std::mem::{align_of, size_of, transmute};

pub struct RootSignatureDescBuilder<'a> {
    parameters: Vec<D3D12_ROOT_PARAMETER>,
    static_samplers: &'a [D3D12_STATIC_SAMPLER_DESC],
    flags: RootSignatureFlags,
    phantom: PhantomData<&'a [D3D12_DESCRIPTOR_RANGE]>,
}

static SS_EMPTY: [D3D12_STATIC_SAMPLER_DESC; 0] = [];

impl<'a> RootSignatureDescBuilder<'a> {
    pub fn new() -> Self {
        Self {
            parameters: Vec::new(),
            static_samplers: &SS_EMPTY,
            flags: RootSignatureFlags::NONE,
            phantom: Default::default(),
        }
    }

    pub fn parameters(mut self, parameters: &[RootParameter<'a>]) -> Self {
        self.parameters = parameters
            .iter()
            .map(|v| D3D12_ROOT_PARAMETER {
                parameter_type: v.get_parameter_type(),
                variant: v.get_variant(),
                shader_visibility: v.get_shader_visibility(),
            })
            .collect();
        self
    }

    pub fn static_samplers(mut self, static_samplers: &'a [StaticSamplerDesc]) -> Self {
        assert_eq!(
            size_of::<StaticSamplerDesc>(),
            size_of::<D3D12_STATIC_SAMPLER_DESC>()
        );
        assert_eq!(
            align_of::<StaticSamplerDesc>(),
            align_of::<D3D12_STATIC_SAMPLER_DESC>()
        );
        self.static_samplers = unsafe { transmute(static_samplers) };
        self
    }

    pub fn flags(mut self, flags: RootSignatureFlags) -> Self {
        self.flags |= flags;
        self
    }

    pub fn build(&'a self) -> RootSignatureDesc<'a> {
        let (num_parameters, p_parameters) = if self.parameters.is_empty() {
            (0, std::ptr::null_mut())
        } else {
            (
                self.parameters.len() as _,
                self.parameters.as_ptr() as *mut _,
            )
        };
        let (num_static_samplers, p_static_samplers) = if self.static_samplers.is_empty() {
            (0, std::ptr::null_mut())
        } else {
            (
                self.static_samplers.len() as _,
                self.static_samplers.as_ptr() as *mut _,
            )
        };
        RootSignatureDesc {
            inner: D3D12_ROOT_SIGNATURE_DESC {
                num_parameters,
                p_parameters,
                num_static_samplers,
                p_static_samplers,
                flags: self.flags.into(),
            },
            phantom: Default::default(),
        }
    }
}

pub struct RootSignatureDesc1Builder<'a> {
    parameters: Vec<D3D12_ROOT_PARAMETER1>,
    static_samplers: &'a [D3D12_STATIC_SAMPLER_DESC],
    flags: RootSignatureFlags,
    phantom: PhantomData<&'a [D3D12_DESCRIPTOR_RANGE1]>,
}

impl<'a> RootSignatureDesc1Builder<'a> {
    pub fn new() -> Self {
        Self {
            parameters: vec![],
            static_samplers: &SS_EMPTY,
            flags: RootSignatureFlags::NONE,
            phantom: Default::default(),
        }
    }

    pub fn parameters(mut self, parameters: &[RootParameter1<'a>]) -> Self {
        self.parameters = parameters
            .iter()
            .map(|v| D3D12_ROOT_PARAMETER1 {
                parameter_type: v.get_parameter_type(),
                variant: v.get_variant(),
                shader_visibility: v.get_shader_visibility(),
            })
            .collect();
        self
    }

    pub fn static_samplers(mut self, static_samplers: &[StaticSamplerDesc]) -> Self {
        assert_eq!(
            size_of::<StaticSamplerDesc>(),
            size_of::<D3D12_STATIC_SAMPLER_DESC>()
        );
        assert_eq!(
            align_of::<StaticSamplerDesc>(),
            align_of::<D3D12_STATIC_SAMPLER_DESC>()
        );
        self.static_samplers = unsafe { transmute(static_samplers) };
        self
    }

    pub fn flags(mut self, flags: RootSignatureFlags) -> Self {
        self.flags |= flags;
        self
    }

    pub fn build(&self) -> RootSignatureDesc1<'a> {
        let (num_parameters, p_parameters) = if self.parameters.is_empty() {
            (0, std::ptr::null_mut())
        } else {
            (
                self.parameters.len() as _,
                self.parameters.as_ptr() as *mut _,
            )
        };
        let (num_static_samplers, p_static_samplers) = if self.static_samplers.is_empty() {
            (0, std::ptr::null_mut())
        } else {
            (
                self.static_samplers.len() as _,
                self.static_samplers.as_ptr() as *mut _,
            )
        };
        RootSignatureDesc1 {
            inner: D3D12_ROOT_SIGNATURE_DESC1 {
                num_parameters,
                p_parameters,
                num_static_samplers,
                p_static_samplers,
                flags: self.flags.into(),
            },
            phantom: Default::default(),
        }
    }
}

#[derive(Clone, Debug)]
pub struct RootSignatureDesc<'a> {
    pub(crate) inner: D3D12_ROOT_SIGNATURE_DESC,
    phantom: PhantomData<&'a ()>,
}

impl<'a> RootSignatureDesc<'a> {
    pub fn builder() -> RootSignatureDescBuilder<'a> {
        RootSignatureDescBuilder::new()
    }
}

impl<'a> Into<VersionedRootSignatureDesc<'a>> for RootSignatureDesc<'a> {
    fn into(self) -> VersionedRootSignatureDesc<'a> {
        VersionedRootSignatureDesc::Desc(self)
    }
}

#[derive(Clone, Debug)]
pub struct RootSignatureDesc1<'a> {
    pub(crate) inner: D3D12_ROOT_SIGNATURE_DESC1,
    phantom: PhantomData<&'a ()>,
}

impl<'a> RootSignatureDesc1<'a> {
    pub fn builder() -> RootSignatureDesc1Builder<'a> {
        RootSignatureDesc1Builder::new()
    }
}

impl<'a> Into<VersionedRootSignatureDesc<'a>> for RootSignatureDesc1<'a> {
    fn into(self) -> VersionedRootSignatureDesc<'a> {
        VersionedRootSignatureDesc::Desc1(self)
    }
}

#[derive(Clone, Debug)]
#[repr(C)]
#[allow(non_camel_case_types)]
pub struct D3D12_ROOT_SIGNATURE_DESC {
    pub num_parameters: u32,
    pub p_parameters: *mut D3D12_ROOT_PARAMETER,
    pub num_static_samplers: u32,
    pub p_static_samplers: *mut D3D12_STATIC_SAMPLER_DESC,
    pub flags: D3D12_ROOT_SIGNATURE_FLAGS,
}

#[derive(Clone, Debug)]
#[repr(C)]
#[allow(non_camel_case_types)]
pub struct D3D12_ROOT_SIGNATURE_DESC1 {
    pub num_parameters: u32,
    pub p_parameters: *mut D3D12_ROOT_PARAMETER1,
    pub num_static_samplers: u32,
    pub p_static_samplers: *mut D3D12_STATIC_SAMPLER_DESC,
    pub flags: D3D12_ROOT_SIGNATURE_FLAGS,
}
