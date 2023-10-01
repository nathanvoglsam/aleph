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

use crate::resource::ResourceId;
use crate::{ResourceMut, ResourceRef};
use aleph_rhi_api::*;
use bumpalo::collections::Vec as BVec;
use bumpalo::Bump;
use std::num::NonZeroU64;
use std::ptr::NonNull;

pub struct ResourceRegistry<'bump> {
    pub(crate) arena: &'bump Bump,
    pub(crate) reads: BVec<'bump, ResourceRef>,
    pub(crate) writes: BVec<'bump, ResourceMut>,
    pub(crate) creates: BVec<'bump, CreatedResource>,
}

impl<'bump> ResourceRegistry<'bump> {
    pub fn read_resource<R: Into<ResourceRef>>(&mut self, resource: R) -> ResourceRef {
        let r = resource.into();
        self.reads.push(r);
        r
    }

    pub fn write_resource<R: Into<ResourceMut>>(&mut self, resource: R) -> ResourceMut {
        let r = resource.into();
        self.writes.push(r);
        r
    }

    pub fn create_texture(&mut self, desc: &TextureDesc) -> ResourceMut {
        let name = desc.name.map(|v| self.arena.alloc_str(v));
        let name = name.map(|v| NonNull::from(v));

        let desc = CreatedTexture {
            width: desc.width,
            height: desc.height,
            depth: desc.depth,
            format: desc.format,
            dimension: desc.dimension,
            clear_value: desc.clear_value.clone(),
            array_size: desc.array_size,
            mip_levels: desc.mip_levels,
            sample_count: desc.sample_count,
            sample_quality: desc.sample_quality,
            usage: desc.usage,
            name,
        };
        let created = CreatedResource::Texture(desc);
        self.creates.push(created);

        ResourceMut(ResourceId(NonZeroU64::new(9876).unwrap())) // TODO: This is temporary
    }

    pub fn create_buffer(&mut self, desc: &BufferDesc) -> ResourceMut {
        let name = desc.name.map(|v| self.arena.alloc_str(v));
        let name = name.map(|v| NonNull::from(v));

        let desc = CreatedBuffer {
            size: desc.size,
            cpu_access: desc.cpu_access,
            usage: desc.usage,
            name,
        };
        let created = CreatedResource::Buffer(desc);
        self.creates.push(created);
        ResourceMut(ResourceId(NonZeroU64::new(858484).unwrap())) // TODO: This is temporary
    }
}

pub enum CreatedResource {
    Buffer(CreatedBuffer),
    Texture(CreatedTexture),
}

pub struct CreatedBuffer {
    pub size: u64,
    pub cpu_access: CpuAccessMode,
    pub usage: BufferUsageFlags,
    pub name: Option<NonNull<str>>,
}

pub struct CreatedTexture {
    pub width: u32,
    pub height: u32,
    pub depth: u32,
    pub format: Format,
    pub dimension: TextureDimension,
    pub clear_value: Option<OptimalClearValue>,
    pub array_size: u32,
    pub mip_levels: u32,
    pub sample_count: u32,
    pub sample_quality: u32,
    pub usage: TextureUsageFlags,
    pub name: Option<NonNull<str>>,
}
