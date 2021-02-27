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

use crate::{PlacedSubresourceFootprint, Resource};
use raw::windows::win32::direct3d12::{
    ID3D12Resource, D3D12_PLACED_SUBRESOURCE_FOOTPRINT, D3D12_TEXTURE_COPY_TYPE,
};
use std::mem::{transmute, ManuallyDrop};

#[derive(Clone)]
pub enum TextureCopyLocation {
    Placed {
        resource: Option<Resource>,
        placed_footprint: PlacedSubresourceFootprint,
    },
    Subresource {
        resource: Option<Resource>,
        subresource_index: u32,
    },
}

impl TextureCopyLocation {
    pub fn resource(&self) -> Option<Resource> {
        match self {
            TextureCopyLocation::Placed { resource, .. } => resource.clone(),
            TextureCopyLocation::Subresource { resource, .. } => resource.clone(),
        }
    }
}

impl Into<D3D12_TEXTURE_COPY_LOCATION> for TextureCopyLocation {
    fn into(self) -> D3D12_TEXTURE_COPY_LOCATION {
        match self {
            TextureCopyLocation::Placed {
                resource,
                placed_footprint,
            } => D3D12_TEXTURE_COPY_LOCATION {
                p_resource: resource.map(|v| v.0),
                r#type: D3D12_TEXTURE_COPY_TYPE::D3D12_TEXTURE_COPY_TYPE_PLACED_FOOTPRINT,
                variant: D3D12_TEXTURE_COPY_LOCATION_VARIANT {
                    placed_footprint: ManuallyDrop::new(unsafe { transmute(placed_footprint) }),
                },
            },
            TextureCopyLocation::Subresource {
                resource,
                subresource_index,
            } => D3D12_TEXTURE_COPY_LOCATION {
                p_resource: resource.map(|v| v.0),
                r#type: D3D12_TEXTURE_COPY_TYPE::D3D12_TEXTURE_COPY_TYPE_SUBRESOURCE_INDEX,
                variant: D3D12_TEXTURE_COPY_LOCATION_VARIANT { subresource_index },
            },
        }
    }
}

#[repr(C)]
#[allow(non_camel_case_types)]
pub struct D3D12_TEXTURE_COPY_LOCATION {
    pub p_resource: Option<ID3D12Resource>,
    pub r#type: D3D12_TEXTURE_COPY_TYPE,
    pub variant: D3D12_TEXTURE_COPY_LOCATION_VARIANT,
}

#[repr(C)]
#[allow(non_camel_case_types)]
pub union D3D12_TEXTURE_COPY_LOCATION_VARIANT {
    placed_footprint: ManuallyDrop<D3D12_PLACED_SUBRESOURCE_FOOTPRINT>,
    subresource_index: u32,
}
