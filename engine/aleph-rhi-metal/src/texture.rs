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

use std::collections::HashMap;
use std::num::NonZeroU64;

use aleph_any::AnyArc;
use aleph_object_system::unsafe_impl_iobject;
use aleph_rhi_api::*;
use aleph_rhi_impl_utils::owned_desc::OwnedTextureDesc;
use objc2::rc::Retained;
use objc2::runtime::ProtocolObject;
use objc2_metal::*;
use parking_lot::Mutex;

use crate::device::Device;

pub struct Texture {
    pub(crate) _device: AnyArc<Device>,
    pub(crate) id: NonZeroU64,
    pub(crate) objects: TextureObjects,
    pub(crate) views: Mutex<HashMap<ImageViewDesc, ()>>,
    pub(crate) rtvs: Mutex<HashMap<ImageViewDesc, ()>>,
    pub(crate) dsvs: Mutex<HashMap<ImageViewDesc, ()>>,
    pub(crate) desc: OwnedTextureDesc,
}

unsafe_impl_iobject!(Texture, "01980753-5c4f-7ae3-be3b-974d0f86022a");

impl Texture {
    pub(crate) fn get(v: &TextureHandle) -> &Self {
        v.get()
            .downcast_ref::<Self>()
            .expect("Unknown Texture implementation!")
    }

    pub(crate) fn get_id(&self) -> NonZeroU64 {
        self.id
    }

    pub(crate) const fn desc(&self) -> &TextureDesc {
        self.desc.get()
    }

    pub(crate) fn get_view(&self, device: &Device, desc: &ImageViewDesc) -> Result<ImageView, ()> {
        todo!()
    }

    pub(crate) fn get_rtv(&self, device: &Device, desc: &ImageViewDesc) -> Result<ImageView, ()> {
        todo!()
    }

    pub(crate) fn get_dsv(&self, device: &Device, desc: &ImageViewDesc) -> Result<ImageView, ()> {
        todo!()
    }
}

/// Wrapper to scope our 'unsafe impl Send+Sync'
pub struct TextureObjects {
    pub texture: Retained<ProtocolObject<dyn MTLTexture>>,
}

// Safety: Needed because of 'MTLTexture'
unsafe impl Send for TextureObjects {}
unsafe impl Sync for TextureObjects {}
