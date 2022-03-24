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

use crate::descriptor_allocator_cpu::DescriptorAllocatorCPU;
use crate::device::Device;
use crate::internal::conv::texture_format_to_dxgi;
use dx12::{dxgi, D3D12Object};
use interfaces::gpu::{
    INamedObject, ITexture, TextureDesc, TextureDimension, TextureFormat, TextureSubResourceSet,
};
use interfaces::ref_ptr::{ref_ptr_object, RefPtr};
use parking_lot::RwLock;
use std::collections::HashMap;

type CacheViewCPU = HashMap<(TextureFormat, TextureSubResourceSet), dx12::CPUDescriptorHandle>;

ref_ptr_object! {
    pub struct Texture: ITexture, ITextureExt {
        pub(crate) device: RefPtr<Device>,
        pub(crate) resource: dx12::Resource,
        pub(crate) desc: TextureDesc,
        pub(crate) dxgi_format: dxgi::Format,
        pub(crate) rtv_cache: RwLock<CacheViewCPU>,
        pub(crate) dsv_cache: RwLock<CacheViewCPU>,
    }
}

impl Texture {}

impl ITexture for Texture {
    fn desc(&self) -> &TextureDesc {
        &self.desc
    }
}

pub trait ITextureExt: ITexture {
    fn get_raw_handle(&self) -> dx12::Resource;

    fn get_raw_format(&self) -> dxgi::Format;
}

impl ITextureExt for Texture {
    fn get_raw_handle(&self) -> dx12::Resource {
        self.resource.clone()
    }

    fn get_raw_format(&self) -> dxgi::Format {
        self.dxgi_format
    }
}

impl INamedObject for Texture {
    fn set_name(&self, name: &str) {
        self.resource.set_name(name).unwrap()
    }
}
