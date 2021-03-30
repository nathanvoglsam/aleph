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
use std::mem::transmute;
use windows_raw::Win32::Direct3D12::{
    D3D12_TEXTURE_COPY_LOCATION, D3D12_TEXTURE_COPY_LOCATION_0, D3D12_TEXTURE_COPY_TYPE,
};

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
                pResource: resource.map(|v| v.0),
                Type: D3D12_TEXTURE_COPY_TYPE::D3D12_TEXTURE_COPY_TYPE_PLACED_FOOTPRINT,
                Anonymous: D3D12_TEXTURE_COPY_LOCATION_0 {
                    PlacedFootprint: unsafe { transmute(placed_footprint) },
                },
            },
            TextureCopyLocation::Subresource {
                resource,
                subresource_index,
            } => D3D12_TEXTURE_COPY_LOCATION {
                pResource: resource.map(|v| v.0),
                Type: D3D12_TEXTURE_COPY_TYPE::D3D12_TEXTURE_COPY_TYPE_SUBRESOURCE_INDEX,
                Anonymous: D3D12_TEXTURE_COPY_LOCATION_0 {
                    SubresourceIndex: subresource_index,
                },
            },
        }
    }
}
