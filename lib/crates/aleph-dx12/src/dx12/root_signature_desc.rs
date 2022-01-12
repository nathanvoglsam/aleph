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

use crate::{
    RootParameter, RootParameter1, RootSignatureFlags, StaticSamplerDesc,
    VersionedRootSignatureDesc,
};
use std::marker::PhantomData;
use std::mem::{align_of, size_of, transmute};
use windows::Win32::Graphics::Direct3D12::{
    D3D12_DESCRIPTOR_RANGE, D3D12_DESCRIPTOR_RANGE1, D3D12_ROOT_PARAMETER, D3D12_ROOT_PARAMETER1,
    D3D12_ROOT_SIGNATURE_DESC, D3D12_ROOT_SIGNATURE_DESC1, D3D12_STATIC_SAMPLER_DESC,
};

pub struct RootSignatureDescBuilder<'a> {
    parameters: Vec<D3D12_ROOT_PARAMETER>,
    static_samplers: &'a [D3D12_STATIC_SAMPLER_DESC],
    flags: RootSignatureFlags,
    phantom: PhantomData<&'a [D3D12_DESCRIPTOR_RANGE]>,
}

static SS_EMPTY: [D3D12_STATIC_SAMPLER_DESC; 0] = [];

impl<'a> RootSignatureDescBuilder<'a> {
    #[inline]
    pub fn new() -> Self {
        Self {
            parameters: Vec::new(),
            static_samplers: &SS_EMPTY,
            flags: RootSignatureFlags::NONE,
            phantom: Default::default(),
        }
    }

    #[inline]
    pub fn parameters(mut self, parameters: &[RootParameter<'a>]) -> Self {
        self.parameters = parameters
            .iter()
            .map(|v| D3D12_ROOT_PARAMETER {
                ParameterType: v.get_parameter_type(),
                Anonymous: v.get_variant(),
                ShaderVisibility: v.get_shader_visibility(),
            })
            .collect();
        self
    }

    #[inline]
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

    #[inline]
    pub fn flags(mut self, flags: RootSignatureFlags) -> Self {
        self.flags |= flags;
        self
    }

    #[inline]
    pub fn build(&'a self) -> RootSignatureDesc<'a> {
        let (num_parameters, p_parameters) =
            windows::utils::optional_slice_to_num_ptr_pair(Some(&self.parameters));
        let (num_static_samplers, p_static_samplers) =
            windows::utils::optional_slice_to_num_ptr_pair(Some(self.static_samplers));
        RootSignatureDesc {
            inner: D3D12_ROOT_SIGNATURE_DESC {
                NumParameters: num_parameters,
                pParameters: p_parameters as *mut _,
                NumStaticSamplers: num_static_samplers,
                pStaticSamplers: p_static_samplers as *mut _,
                Flags: self.flags.into(),
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
    #[inline]
    pub fn new() -> Self {
        Self {
            parameters: vec![],
            static_samplers: &SS_EMPTY,
            flags: RootSignatureFlags::NONE,
            phantom: Default::default(),
        }
    }

    #[inline]
    pub fn parameters(mut self, parameters: &[RootParameter1<'a>]) -> Self {
        self.parameters = parameters
            .iter()
            .map(|v| D3D12_ROOT_PARAMETER1 {
                ParameterType: v.get_parameter_type(),
                Anonymous: v.get_variant(),
                ShaderVisibility: v.get_shader_visibility(),
            })
            .collect();
        self
    }

    #[inline]
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

    #[inline]
    pub fn flags(mut self, flags: RootSignatureFlags) -> Self {
        self.flags |= flags;
        self
    }

    #[inline]
    pub fn build(&self) -> RootSignatureDesc1<'a> {
        let (num_parameters, p_parameters) =
            windows::utils::optional_slice_to_num_ptr_pair(Some(&self.parameters));
        let (num_static_samplers, p_static_samplers) =
            windows::utils::optional_slice_to_num_ptr_pair(Some(self.static_samplers));
        RootSignatureDesc1 {
            inner: D3D12_ROOT_SIGNATURE_DESC1 {
                NumParameters: num_parameters,
                pParameters: p_parameters as *mut _,
                NumStaticSamplers: num_static_samplers,
                pStaticSamplers: p_static_samplers as *mut _,
                Flags: self.flags.into(),
            },
            phantom: Default::default(),
        }
    }
}

#[derive(Clone)]
pub struct RootSignatureDesc<'a> {
    pub(crate) inner: D3D12_ROOT_SIGNATURE_DESC,
    phantom: PhantomData<&'a ()>,
}

impl<'a> RootSignatureDesc<'a> {
    #[inline]
    pub fn builder() -> RootSignatureDescBuilder<'a> {
        RootSignatureDescBuilder::new()
    }
}

impl<'a> Into<VersionedRootSignatureDesc<'a>> for RootSignatureDesc<'a> {
    #[inline]
    fn into(self) -> VersionedRootSignatureDesc<'a> {
        VersionedRootSignatureDesc::Desc(self)
    }
}

#[derive(Clone)]
pub struct RootSignatureDesc1<'a> {
    pub(crate) inner: D3D12_ROOT_SIGNATURE_DESC1,
    phantom: PhantomData<&'a ()>,
}

impl<'a> RootSignatureDesc1<'a> {
    #[inline]
    pub fn builder() -> RootSignatureDesc1Builder<'a> {
        RootSignatureDesc1Builder::new()
    }
}

impl<'a> Into<VersionedRootSignatureDesc<'a>> for RootSignatureDesc1<'a> {
    #[inline]
    fn into(self) -> VersionedRootSignatureDesc<'a> {
        VersionedRootSignatureDesc::Desc1(self)
    }
}
