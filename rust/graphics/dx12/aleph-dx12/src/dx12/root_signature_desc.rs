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
    D3D12_DESCRIPTOR_RANGE, D3D12_DESCRIPTOR_RANGE1, D3D12_ROOT_SIGNATURE_FLAGS,
    D3D12_STATIC_SAMPLER_DESC,
};
use crate::{RootParameter, RootParameter1};
use std::marker::PhantomData;

type F = D3D12_ROOT_SIGNATURE_FLAGS;

pub struct RootSignatureDescBuilder {
    ranges: Vec<Vec<D3D12_DESCRIPTOR_RANGE>>,
    parameters: Vec<D3D12_ROOT_PARAMETER>,
    static_samplers: Vec<D3D12_STATIC_SAMPLER_DESC>,
    flags: D3D12_ROOT_SIGNATURE_FLAGS,
}

impl RootSignatureDescBuilder {
    pub fn new() -> Self {
        Self {
            ranges: vec![],
            parameters: vec![],
            static_samplers: vec![],
            flags: Default::default(),
        }
    }

    pub fn parameters(mut self, parameters: &[RootParameter]) -> Self {
        self.parameters = parameters
            .iter()
            .map(|v| {
                let (ranges, types) = v.parameter.get_raw();
                self.ranges.push(ranges);
                D3D12_ROOT_PARAMETER {
                    parameter_type: v.parameter.get_type(),
                    types,
                    shader_visibility: v.shader_visibility.into(),
                }
            })
            .collect();
        self
    }

    pub fn static_samplers(mut self, static_samplers: &[()]) -> Self {
        self
    }

    pub fn allow_input_assembler_input_layout(mut self, flag: bool) -> Self {
        if flag {
            self.flags.0 |= F::D3D12_ROOT_SIGNATURE_FLAG_ALLOW_INPUT_ASSEMBLER_INPUT_LAYOUT.0;
        }
        self
    }
    pub fn deny_vertex_shader_root_access(mut self, flag: bool) -> Self {
        if flag {
            self.flags.0 |= F::D3D12_ROOT_SIGNATURE_FLAG_DENY_VERTEX_SHADER_ROOT_ACCESS.0;
        }
        self
    }
    pub fn deny_hull_shader_root_access(mut self, flag: bool) -> Self {
        if flag {
            self.flags.0 |= F::D3D12_ROOT_SIGNATURE_FLAG_DENY_HULL_SHADER_ROOT_ACCESS.0;
        }
        self
    }
    pub fn deny_domain_shader_root_access(mut self, flag: bool) -> Self {
        if flag {
            self.flags.0 |= F::D3D12_ROOT_SIGNATURE_FLAG_DENY_DOMAIN_SHADER_ROOT_ACCESS.0;
        }
        self
    }
    pub fn deny_geometry_shader_root_access(mut self, flag: bool) -> Self {
        if flag {
            self.flags.0 |= F::D3D12_ROOT_SIGNATURE_FLAG_DENY_GEOMETRY_SHADER_ROOT_ACCESS.0;
        }
        self
    }
    pub fn deny_pixel_shader_root_access(mut self, flag: bool) -> Self {
        if flag {
            self.flags.0 |= F::D3D12_ROOT_SIGNATURE_FLAG_DENY_PIXEL_SHADER_ROOT_ACCESS.0;
        }
        self
    }
    pub fn allow_stream_output(mut self, flag: bool) -> Self {
        if flag {
            self.flags.0 |= F::D3D12_ROOT_SIGNATURE_FLAG_ALLOW_STREAM_OUTPUT.0;
        }
        self
    }
    pub fn local_root_signature(mut self, flag: bool) -> Self {
        if flag {
            self.flags.0 |= F::D3D12_ROOT_SIGNATURE_FLAG_LOCAL_ROOT_SIGNATURE.0;
        }
        self
    }
    pub fn deny_amplification_shader_root_access(mut self, flag: bool) -> Self {
        if flag {
            self.flags.0 |= F::D3D12_ROOT_SIGNATURE_FLAG_DENY_AMPLIFICATION_SHADER_ROOT_ACCESS.0;
        }
        self
    }
    pub fn deny_mesh_shader_root_access(mut self, flag: bool) -> Self {
        if flag {
            self.flags.0 |= F::D3D12_ROOT_SIGNATURE_FLAG_DENY_MESH_SHADER_ROOT_ACCESS.0;
        }
        self
    }

    pub fn build(&self) -> RootSignatureDesc {
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
                flags: self.flags,
            },
            phantom: Default::default(),
        }
    }
}

pub struct RootSignatureDesc1Builder {
    ranges: Vec<Vec<D3D12_DESCRIPTOR_RANGE1>>,
    parameters: Vec<D3D12_ROOT_PARAMETER1>,
    static_samplers: Vec<D3D12_STATIC_SAMPLER_DESC>,
    flags: D3D12_ROOT_SIGNATURE_FLAGS,
}

impl RootSignatureDesc1Builder {
    pub fn new() -> Self {
        Self {
            ranges: vec![],
            parameters: vec![],
            static_samplers: vec![],
            flags: Default::default(),
        }
    }

    pub fn parameters(mut self, parameters: &[RootParameter1]) -> Self {
        self.parameters = parameters
            .iter()
            .map(|v| {
                let (ranges, types) = v.parameter.get_raw();
                self.ranges.push(ranges);
                D3D12_ROOT_PARAMETER1 {
                    parameter_type: v.parameter.get_type(),
                    types,
                    shader_visibility: v.shader_visibility.into(),
                }
            })
            .collect();
        self
    }

    pub fn static_samplers(mut self, static_samplers: &[()]) -> Self {
        self
    }

    pub fn allow_input_assembler_input_layout(mut self, flag: bool) -> Self {
        if flag {
            self.flags.0 |= F::D3D12_ROOT_SIGNATURE_FLAG_ALLOW_INPUT_ASSEMBLER_INPUT_LAYOUT.0;
        }
        self
    }

    pub fn deny_vertex_shader_root_access(mut self, flag: bool) -> Self {
        if flag {
            self.flags.0 |= F::D3D12_ROOT_SIGNATURE_FLAG_DENY_VERTEX_SHADER_ROOT_ACCESS.0;
        }
        self
    }

    pub fn deny_hull_shader_root_access(mut self, flag: bool) -> Self {
        if flag {
            self.flags.0 |= F::D3D12_ROOT_SIGNATURE_FLAG_DENY_HULL_SHADER_ROOT_ACCESS.0;
        }
        self
    }

    pub fn deny_domain_shader_root_access(mut self, flag: bool) -> Self {
        if flag {
            self.flags.0 |= F::D3D12_ROOT_SIGNATURE_FLAG_DENY_DOMAIN_SHADER_ROOT_ACCESS.0;
        }
        self
    }

    pub fn deny_geometry_shader_root_access(mut self, flag: bool) -> Self {
        if flag {
            self.flags.0 |= F::D3D12_ROOT_SIGNATURE_FLAG_DENY_GEOMETRY_SHADER_ROOT_ACCESS.0;
        }
        self
    }

    pub fn deny_pixel_shader_root_access(mut self, flag: bool) -> Self {
        if flag {
            self.flags.0 |= F::D3D12_ROOT_SIGNATURE_FLAG_DENY_PIXEL_SHADER_ROOT_ACCESS.0;
        }
        self
    }

    pub fn allow_stream_output(mut self, flag: bool) -> Self {
        if flag {
            self.flags.0 |= F::D3D12_ROOT_SIGNATURE_FLAG_ALLOW_STREAM_OUTPUT.0;
        }
        self
    }

    pub fn local_root_signature(mut self, flag: bool) -> Self {
        if flag {
            self.flags.0 |= F::D3D12_ROOT_SIGNATURE_FLAG_LOCAL_ROOT_SIGNATURE.0;
        }
        self
    }

    pub fn deny_amplification_shader_root_access(mut self, flag: bool) -> Self {
        if flag {
            self.flags.0 |= F::D3D12_ROOT_SIGNATURE_FLAG_DENY_AMPLIFICATION_SHADER_ROOT_ACCESS.0;
        }
        self
    }

    pub fn deny_mesh_shader_root_access(mut self, flag: bool) -> Self {
        if flag {
            self.flags.0 |= F::D3D12_ROOT_SIGNATURE_FLAG_DENY_MESH_SHADER_ROOT_ACCESS.0;
        }
        self
    }

    pub fn build(&self) -> RootSignatureDesc1 {
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
                flags: self.flags,
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

#[derive(Clone, Debug)]
pub struct RootSignatureDesc1<'a> {
    pub(crate) inner: D3D12_ROOT_SIGNATURE_DESC1,
    phantom: PhantomData<&'a ()>,
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
