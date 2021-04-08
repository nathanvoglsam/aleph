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
use std::mem::transmute_copy;
use windows_raw::Win32::Direct3D12::{
    D3D12_RESOURCE_ALIASING_BARRIER_abi, D3D12_RESOURCE_TRANSITION_BARRIER_abi,
    D3D12_RESOURCE_UAV_BARRIER_abi, D3D12_RESOURCE_BARRIER, D3D12_RESOURCE_BARRIER_0,
    D3D12_RESOURCE_BARRIER_TYPE,
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
                Type: D3D12_RESOURCE_BARRIER_TYPE::D3D12_RESOURCE_BARRIER_TYPE_TRANSITION,
                Flags: flags.clone().into(),
                Anonymous: D3D12_RESOURCE_BARRIER_0 {
                    Transition: D3D12_RESOURCE_TRANSITION_BARRIER_abi {
                        pResource: unsafe { transmute_copy(resource) },
                        Subresource: *subresource,
                        StateBefore: state_before.clone().into(),
                        StateAfter: state_after.clone().into(),
                    },
                },
            },
            ResourceBarrier::Aliasing {
                flags,
                resource_before,
                resource_after,
            } => D3D12_RESOURCE_BARRIER {
                Type: D3D12_RESOURCE_BARRIER_TYPE::D3D12_RESOURCE_BARRIER_TYPE_ALIASING,
                Flags: flags.clone().into(),
                Anonymous: D3D12_RESOURCE_BARRIER_0 {
                    Aliasing: D3D12_RESOURCE_ALIASING_BARRIER_abi {
                        pResourceBefore: unsafe { transmute_copy(resource_before) },
                        pResourceAfter: unsafe { transmute_copy(resource_after) },
                    },
                },
            },
            ResourceBarrier::UAV { flags, resource } => D3D12_RESOURCE_BARRIER {
                Type: D3D12_RESOURCE_BARRIER_TYPE::D3D12_RESOURCE_BARRIER_TYPE_UAV,
                Flags: flags.clone().into(),
                Anonymous: D3D12_RESOURCE_BARRIER_0 {
                    UAV: D3D12_RESOURCE_UAV_BARRIER_abi {
                        pResource: unsafe { transmute_copy(resource) },
                    },
                },
            },
        }
    }
}
