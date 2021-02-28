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
use crate::{
    DescriptorRange, DescriptorRange1, RootConstants, RootDescriptor, RootDescriptor1,
    ShaderVisibility,
};
use std::mem::{transmute, ManuallyDrop};

#[derive(Clone, Debug, Hash)]
pub enum RootParameter<'a> {
    DescriptorTable {
        visibility: ShaderVisibility,
        ranges: &'a [DescriptorRange],
    },
    Constants {
        visibility: ShaderVisibility,
        constants: RootConstants,
    },
    CBV {
        visibility: ShaderVisibility,
        cbv: RootDescriptor,
    },
    SRV {
        visibility: ShaderVisibility,
        srv: RootDescriptor,
    },
    UAV {
        visibility: ShaderVisibility,
        uav: RootDescriptor,
    },
}

impl<'a> RootParameter<'a> {
    pub(crate) fn get_parameter_type(&self) -> D3D12_ROOT_PARAMETER_TYPE {
        match self {
            RootParameter::DescriptorTable { .. } => {
                D3D12_ROOT_PARAMETER_TYPE::D3D12_ROOT_PARAMETER_TYPE_DESCRIPTOR_TABLE
            }
            RootParameter::Constants { .. } => {
                D3D12_ROOT_PARAMETER_TYPE::D3D12_ROOT_PARAMETER_TYPE_32BIT_CONSTANTS
            }
            RootParameter::CBV { .. } => D3D12_ROOT_PARAMETER_TYPE::D3D12_ROOT_PARAMETER_TYPE_CBV,
            RootParameter::SRV { .. } => D3D12_ROOT_PARAMETER_TYPE::D3D12_ROOT_PARAMETER_TYPE_SRV,
            RootParameter::UAV { .. } => D3D12_ROOT_PARAMETER_TYPE::D3D12_ROOT_PARAMETER_TYPE_UAV,
        }
    }

    pub(crate) fn get_variant(&self) -> D3D12_ROOT_PARAMETER_VARIANT {
        match self {
            RootParameter::DescriptorTable { ranges, .. } => {
                if ranges.is_empty() {
                    D3D12_ROOT_PARAMETER_VARIANT {
                        descriptor_table: ManuallyDrop::new(D3D12_ROOT_DESCRIPTOR_TABLE {
                            num_descriptor_ranges: 0,
                            p_descriptor_ranges: std::ptr::null_mut(),
                        }),
                    }
                } else {
                    D3D12_ROOT_PARAMETER_VARIANT {
                        descriptor_table: ManuallyDrop::new(D3D12_ROOT_DESCRIPTOR_TABLE {
                            num_descriptor_ranges: ranges.len() as _,
                            p_descriptor_ranges: ranges.as_ptr() as *mut DescriptorRange as *mut _,
                        }),
                    }
                }
            }
            RootParameter::Constants { constants, .. } => D3D12_ROOT_PARAMETER_VARIANT {
                constants: ManuallyDrop::new(unsafe { transmute(constants.clone()) }),
            },
            RootParameter::CBV { cbv, .. } => D3D12_ROOT_PARAMETER_VARIANT {
                descriptor: ManuallyDrop::new(unsafe { transmute(cbv.clone()) }),
            },
            RootParameter::SRV { srv, .. } => D3D12_ROOT_PARAMETER_VARIANT {
                descriptor: ManuallyDrop::new(unsafe { transmute(srv.clone()) }),
            },
            RootParameter::UAV { uav, .. } => D3D12_ROOT_PARAMETER_VARIANT {
                descriptor: ManuallyDrop::new(unsafe { transmute(uav.clone()) }),
            },
        }
    }

    pub(crate) fn get_shader_visibility(&self) -> D3D12_SHADER_VISIBILITY {
        match self {
            RootParameter::DescriptorTable { visibility, .. } => visibility.clone().into(),
            RootParameter::Constants { visibility, .. } => visibility.clone().into(),
            RootParameter::CBV { visibility, .. } => visibility.clone().into(),
            RootParameter::SRV { visibility, .. } => visibility.clone().into(),
            RootParameter::UAV { visibility, .. } => visibility.clone().into(),
        }
    }
}

#[derive(Clone, Debug, Hash)]
pub enum RootParameter1<'a> {
    DescriptorTable {
        visibility: ShaderVisibility,
        ranges: &'a [DescriptorRange1],
    },
    Constants {
        visibility: ShaderVisibility,
        constants: RootConstants,
    },
    CBV {
        visibility: ShaderVisibility,
        cbv: RootDescriptor1,
    },
    SRV {
        visibility: ShaderVisibility,
        srv: RootDescriptor1,
    },
    UAV {
        visibility: ShaderVisibility,
        uav: RootDescriptor1,
    },
}

impl<'a> RootParameter1<'a> {
    pub(crate) fn get_parameter_type(&self) -> D3D12_ROOT_PARAMETER_TYPE {
        match self {
            RootParameter1::DescriptorTable { .. } => {
                D3D12_ROOT_PARAMETER_TYPE::D3D12_ROOT_PARAMETER_TYPE_DESCRIPTOR_TABLE
            }
            RootParameter1::Constants { .. } => {
                D3D12_ROOT_PARAMETER_TYPE::D3D12_ROOT_PARAMETER_TYPE_32BIT_CONSTANTS
            }
            RootParameter1::CBV { .. } => D3D12_ROOT_PARAMETER_TYPE::D3D12_ROOT_PARAMETER_TYPE_CBV,
            RootParameter1::SRV { .. } => D3D12_ROOT_PARAMETER_TYPE::D3D12_ROOT_PARAMETER_TYPE_SRV,
            RootParameter1::UAV { .. } => D3D12_ROOT_PARAMETER_TYPE::D3D12_ROOT_PARAMETER_TYPE_UAV,
        }
    }

    pub(crate) fn get_variant(&self) -> D3D12_ROOT_PARAMETER1_VARIANT {
        match self {
            RootParameter1::DescriptorTable { ranges, .. } => {
                if ranges.is_empty() {
                    D3D12_ROOT_PARAMETER1_VARIANT {
                        descriptor_table: ManuallyDrop::new(D3D12_ROOT_DESCRIPTOR_TABLE1 {
                            num_descriptor_ranges: 0,
                            p_descriptor_ranges: std::ptr::null_mut(),
                        }),
                    }
                } else {
                    D3D12_ROOT_PARAMETER1_VARIANT {
                        descriptor_table: ManuallyDrop::new(D3D12_ROOT_DESCRIPTOR_TABLE1 {
                            num_descriptor_ranges: ranges.len() as _,
                            p_descriptor_ranges: ranges.as_ptr() as *mut DescriptorRange1 as *mut _,
                        }),
                    }
                }
            }
            RootParameter1::Constants { constants, .. } => D3D12_ROOT_PARAMETER1_VARIANT {
                constants: ManuallyDrop::new(unsafe { transmute(constants.clone()) }),
            },
            RootParameter1::CBV { cbv, .. } => D3D12_ROOT_PARAMETER1_VARIANT {
                descriptor: ManuallyDrop::new(unsafe { transmute(cbv.clone()) }),
            },
            RootParameter1::SRV { srv, .. } => D3D12_ROOT_PARAMETER1_VARIANT {
                descriptor: ManuallyDrop::new(unsafe { transmute(srv.clone()) }),
            },
            RootParameter1::UAV { uav, .. } => D3D12_ROOT_PARAMETER1_VARIANT {
                descriptor: ManuallyDrop::new(unsafe { transmute(uav.clone()) }),
            },
        }
    }

    pub(crate) fn get_shader_visibility(&self) -> D3D12_SHADER_VISIBILITY {
        match self {
            RootParameter1::DescriptorTable { visibility, .. } => visibility.clone().into(),
            RootParameter1::Constants { visibility, .. } => visibility.clone().into(),
            RootParameter1::CBV { visibility, .. } => visibility.clone().into(),
            RootParameter1::SRV { visibility, .. } => visibility.clone().into(),
            RootParameter1::UAV { visibility, .. } => visibility.clone().into(),
        }
    }
}

#[repr(C)]
#[allow(non_camel_case_types)]
pub struct D3D12_ROOT_PARAMETER {
    pub parameter_type: D3D12_ROOT_PARAMETER_TYPE,
    pub variant: D3D12_ROOT_PARAMETER_VARIANT,
    pub shader_visibility: D3D12_SHADER_VISIBILITY,
}

#[repr(C)]
#[allow(non_camel_case_types)]
pub union D3D12_ROOT_PARAMETER_VARIANT {
    pub descriptor_table: ManuallyDrop<D3D12_ROOT_DESCRIPTOR_TABLE>,
    pub constants: ManuallyDrop<D3D12_ROOT_CONSTANTS>,
    pub descriptor: ManuallyDrop<D3D12_ROOT_DESCRIPTOR>,
}

#[repr(C)]
#[allow(non_camel_case_types)]
pub struct D3D12_ROOT_PARAMETER1 {
    pub parameter_type: D3D12_ROOT_PARAMETER_TYPE,
    pub variant: D3D12_ROOT_PARAMETER1_VARIANT,
    pub shader_visibility: D3D12_SHADER_VISIBILITY,
}

#[repr(C)]
#[allow(non_camel_case_types)]
pub union D3D12_ROOT_PARAMETER1_VARIANT {
    pub descriptor_table: ManuallyDrop<D3D12_ROOT_DESCRIPTOR_TABLE1>,
    pub constants: ManuallyDrop<D3D12_ROOT_CONSTANTS>,
    pub descriptor: ManuallyDrop<D3D12_ROOT_DESCRIPTOR1>,
}
