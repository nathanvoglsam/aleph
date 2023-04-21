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

use crate::ValidationDevice;
use interfaces::any::{AnyArc, AnyWeak};
use interfaces::gpu::*;
use parking_lot::Mutex;
use std::collections::HashMap;

pub struct ValidationTexture {
    pub(crate) _this: AnyWeak<Self>,
    pub(crate) _device: AnyArc<ValidationDevice>,
    pub(crate) inner: AnyArc<dyn ITexture>,
    pub(crate) views: Mutex<HashMap<ImageViewDesc, Box<ValidationImageView>>>,
    pub(crate) rtvs: Mutex<HashMap<ImageViewDesc, Box<ValidationImageView>>>,
    pub(crate) dsvs: Mutex<HashMap<ImageViewDesc, Box<ValidationImageView>>>,
}

interfaces::any::declare_interfaces!(ValidationTexture, [ITexture]);

crate::impl_platform_interface_passthrough!(ValidationTexture);

impl ITexture for ValidationTexture {
    fn upgrade(&self) -> AnyArc<dyn ITexture> {
        AnyArc::map::<dyn ITexture, _>(self._this.upgrade().unwrap(), |v| v)
    }

    fn strong_count(&self) -> usize {
        self._this.strong_count()
    }

    fn weak_count(&self) -> usize {
        self._this.weak_count()
    }

    fn desc(&self) -> TextureDesc {
        self.inner.desc()
    }

    fn get_view(&self, desc: &ImageViewDesc) -> Result<ImageView, ()> {
        let mut views = self.views.lock();
        let view = if let Some(v) = views.get(desc) {
            unsafe { std::mem::transmute_copy::<_, ImageView>(v) }
        } else {
            let image_view = Box::new(ValidationImageView {
                _image: self._this.clone(),
                image_view: self.inner.get_view(desc)?,
                desc: desc.clone(),
            });

            let v = unsafe { std::mem::transmute_copy::<_, ImageView>(&image_view) };
            views.insert(desc.clone(), image_view);

            v
        };
        Ok(view)
    }

    fn get_rtv(&self, desc: &ImageViewDesc) -> Result<ImageView, ()> {
        let mut views = self.rtvs.lock();
        let view = if let Some(v) = views.get(desc) {
            let v = unsafe { std::mem::transmute_copy::<_, ImageView>(v) };
            v
        } else {
            let image_view = Box::new(ValidationImageView {
                _image: self._this.clone(),
                image_view: self.inner.get_rtv(desc)?,
                desc: desc.clone(),
            });

            let v = unsafe { std::mem::transmute_copy::<_, ImageView>(&image_view) };
            views.insert(desc.clone(), image_view);

            v
        };
        Ok(view)
    }

    fn get_dsv(&self, desc: &ImageViewDesc) -> Result<ImageView, ()> {
        let mut views = self.dsvs.lock();
        let view = if let Some(v) = views.get(desc) {
            let v = unsafe { std::mem::transmute_copy::<_, ImageView>(v) };
            v
        } else {
            let image_view = Box::new(ValidationImageView {
                _image: self._this.clone(),
                image_view: self.inner.get_dsv(desc)?,
                desc: desc.clone(),
            });

            let v = unsafe { std::mem::transmute_copy::<_, ImageView>(&image_view) };
            views.insert(desc.clone(), image_view);

            v
        };
        Ok(view)
    }
}

pub struct ValidationImageView {
    pub _image: AnyWeak<ValidationTexture>,
    pub image_view: ImageView,
    pub desc: ImageViewDesc,
}
