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

pub(crate) mod raw;

#[derive(Clone)]
pub enum RootParameterType {
    DescriptorTable(D3D12_ROOT_DESCRIPTOR_TABLE),
    Constants32Bit(D3D12_ROOT_CONSTANTS),
    CBV(D3D12_ROOT_DESCRIPTOR),
    SRV(D3D12_ROOT_DESCRIPTOR),
    UAV(D3D12_ROOT_DESCRIPTOR),
}

#[derive(Clone)]
pub struct RootParameter {
    pub parameter_type: RootParameterType,
    pub visibility: D3D12_SHADER_VISIBILITY,
}

impl Into<raw::D3D12_ROOT_PARAMETER> for RootParameter {
    fn into(self) -> raw::D3D12_ROOT_PARAMETER {
        let (parameter_type, types) = {
            match self.parameter_type {
                RootParameterType::DescriptorTable(v) => (
                    D3D12_ROOT_PARAMETER_TYPE::D3D12_ROOT_PARAMETER_TYPE_DESCRIPTOR_TABLE,
                    v.into(),
                ),
                RootParameterType::Constants32Bit(v) => (
                    D3D12_ROOT_PARAMETER_TYPE::D3D12_ROOT_PARAMETER_TYPE_32BIT_CONSTANTS,
                    v.into(),
                ),
                RootParameterType::CBV(v) => (
                    D3D12_ROOT_PARAMETER_TYPE::D3D12_ROOT_PARAMETER_TYPE_CBV,
                    v.into(),
                ),
                RootParameterType::SRV(v) => (
                    D3D12_ROOT_PARAMETER_TYPE::D3D12_ROOT_PARAMETER_TYPE_SRV,
                    v.into(),
                ),
                RootParameterType::UAV(v) => (
                    D3D12_ROOT_PARAMETER_TYPE::D3D12_ROOT_PARAMETER_TYPE_UAV,
                    v.into(),
                ),
            }
        };
        raw::D3D12_ROOT_PARAMETER {
            parameter_type,
            types,
            shader_visibility: self.visibility,
        }
    }
}

#[derive(Clone)]
pub enum RootParameter1Type {
    DescriptorTable(D3D12_ROOT_DESCRIPTOR_TABLE1),
    Constants32Bit(D3D12_ROOT_CONSTANTS),
    CBV(D3D12_ROOT_DESCRIPTOR1),
    SRV(D3D12_ROOT_DESCRIPTOR1),
    UAV(D3D12_ROOT_DESCRIPTOR1),
}

#[derive(Clone)]
pub struct RootParameter1 {
    pub parameter_type: RootParameter1Type,
    pub visibility: D3D12_SHADER_VISIBILITY,
}

impl Into<raw::D3D12_ROOT_PARAMETER1> for RootParameter1 {
    fn into(self) -> raw::D3D12_ROOT_PARAMETER1 {
        let (parameter_type, types) = {
            match self.parameter_type {
                RootParameter1Type::DescriptorTable(v) => (
                    D3D12_ROOT_PARAMETER_TYPE::D3D12_ROOT_PARAMETER_TYPE_DESCRIPTOR_TABLE,
                    v.into(),
                ),
                RootParameter1Type::Constants32Bit(v) => (
                    D3D12_ROOT_PARAMETER_TYPE::D3D12_ROOT_PARAMETER_TYPE_32BIT_CONSTANTS,
                    v.into(),
                ),
                RootParameter1Type::CBV(v) => (
                    D3D12_ROOT_PARAMETER_TYPE::D3D12_ROOT_PARAMETER_TYPE_CBV,
                    v.into(),
                ),
                RootParameter1Type::SRV(v) => (
                    D3D12_ROOT_PARAMETER_TYPE::D3D12_ROOT_PARAMETER_TYPE_SRV,
                    v.into(),
                ),
                RootParameter1Type::UAV(v) => (
                    D3D12_ROOT_PARAMETER_TYPE::D3D12_ROOT_PARAMETER_TYPE_UAV,
                    v.into(),
                ),
            }
        };
        raw::D3D12_ROOT_PARAMETER1 {
            parameter_type,
            types,
            shader_visibility: self.visibility,
        }
    }
}
