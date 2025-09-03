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

use std::num::NonZeroU64;
use std::ptr::NonNull;

use aleph_any::AnyArc;
use aleph_object_system::unsafe_impl_iobject;
use aleph_rhi_api::*;
use aleph_rhi_impl_utils::owned_desc::OwnedTextureDesc;

use crate::NullDevice;

pub struct NullTexture {
    pub(crate) _device: AnyArc<NullDevice>,
    pub(crate) id: NonZeroU64,
    pub(crate) desc: OwnedTextureDesc,
}

unsafe_impl_iobject!(NullTexture, "01944ecd-4c40-7793-83c0-d01bf99fd58f");

impl NullTexture {
    pub(crate) fn get(v: &TextureHandle) -> &Self {
        v.get()
            .downcast_ref::<Self>()
            .expect("Unknown Texture implementation!")
    }

    pub(crate) const fn get_id(&self) -> NonZeroU64 {
        self.id
    }

    pub(crate) const fn desc(&self) -> &TextureDesc<'_> {
        self.desc.get()
    }

    pub(crate) fn get_view(&self, _desc: &ImageViewDesc) -> Result<ImageView, ()> {
        Ok(unsafe { ImageView::from_raw(NonNull::<()>::dangling()) })
    }

    pub(crate) fn get_rtv(&self, _desc: &ImageViewDesc) -> Result<ImageView, ()> {
        Ok(unsafe { ImageView::from_raw(NonNull::<()>::dangling()) })
    }

    pub(crate) fn get_dsv(&self, _desc: &ImageViewDesc) -> Result<ImageView, ()> {
        Ok(unsafe { ImageView::from_raw(NonNull::<()>::dangling()) })
    }
}
