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
    D3D12_RESOURCE_ALIASING_BARRIER, D3D12_RESOURCE_BARRIER_FLAGS, D3D12_RESOURCE_BARRIER_TYPE,
    D3D12_RESOURCE_TRANSITION_BARRIER, D3D12_RESOURCE_UAV_BARRIER,
};
use crate::{Resource, ResourceBarrierFlags, ResourceStates};
use std::mem::ManuallyDrop;

#[derive(Clone)]
pub enum ResourceBarrier {
    Transition {
        flags: ResourceBarrierFlags,
        resource: Option<Resource>,
        subresource: u32,
        state_before: ResourceStates,
        state_after: ResourceStates,
    },
    Aliasing {
        flags: ResourceBarrierFlags,
        resource_before: Option<Resource>,
        resource_after: Option<Resource>,
    },
    UAV {
        flags: ResourceBarrierFlags,
        resource: Option<Resource>,
    },
}

impl Into<D3D12_RESOURCE_BARRIER> for ResourceBarrier {
    fn into(self) -> D3D12_RESOURCE_BARRIER {
        match self {
            ResourceBarrier::Transition {
                flags,
                resource,
                subresource,
                state_before,
                state_after,
            } => D3D12_RESOURCE_BARRIER {
                r#type: D3D12_RESOURCE_BARRIER_TYPE::D3D12_RESOURCE_BARRIER_TYPE_TRANSITION,
                flags: flags.into(),
                variant: D3D12_RESOURCE_BARRIER_VARIANT {
                    transition: ManuallyDrop::new(D3D12_RESOURCE_TRANSITION_BARRIER {
                        p_resource: resource.map(|v| v.0),
                        subresource,
                        state_before: state_before.into(),
                        state_after: state_after.into(),
                    }),
                },
            },
            ResourceBarrier::Aliasing {
                flags,
                resource_before,
                resource_after,
            } => D3D12_RESOURCE_BARRIER {
                r#type: D3D12_RESOURCE_BARRIER_TYPE::D3D12_RESOURCE_BARRIER_TYPE_ALIASING,
                flags: flags.into(),
                variant: D3D12_RESOURCE_BARRIER_VARIANT {
                    aliasing: ManuallyDrop::new(D3D12_RESOURCE_ALIASING_BARRIER {
                        p_resource_before: resource_before.map(|v| v.0),
                        p_resource_after: resource_after.map(|v| v.0),
                    }),
                },
            },
            ResourceBarrier::UAV { flags, resource } => D3D12_RESOURCE_BARRIER {
                r#type: D3D12_RESOURCE_BARRIER_TYPE::D3D12_RESOURCE_BARRIER_TYPE_UAV,
                flags: flags.into(),
                variant: D3D12_RESOURCE_BARRIER_VARIANT {
                    uav: ManuallyDrop::new(D3D12_RESOURCE_UAV_BARRIER {
                        p_resource: resource.map(|v| v.0),
                    }),
                },
            },
        }
    }
}

#[repr(C)]
#[allow(non_camel_case_types)]
pub struct D3D12_RESOURCE_BARRIER {
    r#type: D3D12_RESOURCE_BARRIER_TYPE,
    flags: D3D12_RESOURCE_BARRIER_FLAGS,
    variant: D3D12_RESOURCE_BARRIER_VARIANT,
}

impl Drop for D3D12_RESOURCE_BARRIER {
    fn drop(&mut self) {
        unsafe {
            if self.r#type == D3D12_RESOURCE_BARRIER_TYPE::D3D12_RESOURCE_BARRIER_TYPE_TRANSITION {
                ManuallyDrop::drop(&mut self.variant.transition);
            } else if self.r#type
                == D3D12_RESOURCE_BARRIER_TYPE::D3D12_RESOURCE_BARRIER_TYPE_ALIASING
            {
                ManuallyDrop::drop(&mut self.variant.aliasing);
            } else if self.r#type == D3D12_RESOURCE_BARRIER_TYPE::D3D12_RESOURCE_BARRIER_TYPE_UAV {
                ManuallyDrop::drop(&mut self.variant.uav);
            } else {
                unreachable!("All possible types of D3D12_RESOURCE_BARRIER_TYPE enumerated");
            }
        }
    }
}

#[repr(C)]
#[allow(non_camel_case_types)]
pub union D3D12_RESOURCE_BARRIER_VARIANT {
    transition: ManuallyDrop<D3D12_RESOURCE_TRANSITION_BARRIER>,
    aliasing: ManuallyDrop<D3D12_RESOURCE_ALIASING_BARRIER>,
    uav: ManuallyDrop<D3D12_RESOURCE_UAV_BARRIER>,
}
