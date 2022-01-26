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

use crate::{Resource, ResourceBarrierFlags, ResourceStates};
use std::mem::{transmute_copy, ManuallyDrop};
use windows::Win32::Graphics::Direct3D12::{
    D3D12_RESOURCE_ALIASING_BARRIER, D3D12_RESOURCE_BARRIER, D3D12_RESOURCE_BARRIER_0,
    D3D12_RESOURCE_BARRIER_TYPE_ALIASING, D3D12_RESOURCE_BARRIER_TYPE_TRANSITION,
    D3D12_RESOURCE_BARRIER_TYPE_UAV, D3D12_RESOURCE_TRANSITION_BARRIER, D3D12_RESOURCE_UAV_BARRIER,
};

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

impl ResourceBarrier {
    pub(crate) fn get_raw(&self) -> D3D12_RESOURCE_BARRIER {
        match self {
            ResourceBarrier::Transition {
                flags,
                resource,
                subresource,
                state_before,
                state_after,
            } => D3D12_RESOURCE_BARRIER {
                Type: D3D12_RESOURCE_BARRIER_TYPE_TRANSITION,
                Flags: (*flags).into(),
                Anonymous: D3D12_RESOURCE_BARRIER_0 {
                    Transition: ManuallyDrop::new(D3D12_RESOURCE_TRANSITION_BARRIER {
                        pResource: unsafe { transmute_copy(resource) },
                        Subresource: *subresource,
                        StateBefore: (*state_before).into(),
                        StateAfter: (*state_after).into(),
                    }),
                },
            },
            ResourceBarrier::Aliasing {
                flags,
                resource_before,
                resource_after,
            } => D3D12_RESOURCE_BARRIER {
                Type: D3D12_RESOURCE_BARRIER_TYPE_ALIASING,
                Flags: (*flags).into(),
                Anonymous: D3D12_RESOURCE_BARRIER_0 {
                    Aliasing: ManuallyDrop::new(D3D12_RESOURCE_ALIASING_BARRIER {
                        pResourceBefore: unsafe { transmute_copy(resource_before) },
                        pResourceAfter: unsafe { transmute_copy(resource_after) },
                    }),
                },
            },
            ResourceBarrier::UAV { flags, resource } => D3D12_RESOURCE_BARRIER {
                Type: D3D12_RESOURCE_BARRIER_TYPE_UAV,
                Flags: (*flags).into(),
                Anonymous: D3D12_RESOURCE_BARRIER_0 {
                    UAV: ManuallyDrop::new(D3D12_RESOURCE_UAV_BARRIER {
                        pResource: unsafe { transmute_copy(resource) },
                    }),
                },
            },
        }
    }
}
