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

use std::ffi::c_void;
use std::ptr::NonNull;
use std::sync::{Arc, Weak};

use aleph_rhi_api::*;
use raw_window_handle::{HasDisplayHandle, HasWindowHandle};

use crate::internal::unwrap;
use crate::{ValidationAdapter, ValidationSurface};

pub struct ValidationContext {
    pub(crate) _this: Weak<Self>,
    pub(crate) inner: Arc<dyn IContext>,
}

crate::impl_platform_interface_passthrough!(ValidationContext);

impl IContext for ValidationContext {
    fn upgrade(&self) -> Arc<dyn IContext> {
        self._this.upgrade().unwrap()
    }

    fn strong_count(&self) -> usize {
        self._this.strong_count()
    }

    fn weak_count(&self) -> usize {
        self._this.weak_count()
    }

    fn request_adapter(&self, options: &AdapterRequestOptions) -> Option<Arc<dyn IAdapter>> {
        // Unwrap the ISurface reference to the inner object
        let mut options = options.clone();
        options.surface = options.surface.map(|v| unwrap::surface(v).inner.as_ref());

        let inner = self.inner.request_adapter(&options)?;
        let adapter = Arc::new_cyclic(move |v| ValidationAdapter {
            _this: v.clone(),
            _context: self._this.upgrade().unwrap(),
            inner,
        });
        Some(adapter)
    }

    fn create_surface(
        &self,
        display: &dyn HasDisplayHandle,
        window: &dyn HasWindowHandle,
    ) -> Result<Arc<dyn ISurface>, SurfaceCreateError> {
        let inner = self.inner.create_surface(display, window)?;
        let surface = Arc::new_cyclic(move |v| ValidationSurface {
            _this: v.clone(),
            _context: self._this.upgrade().unwrap(),
            inner,
            has_swap_chain: Default::default(),
        });
        Ok(surface)
    }

    fn create_surface_for_metal_layer(
        &self,
        layer: NonNull<c_void>,
    ) -> Result<Arc<dyn ISurface>, SurfaceCreateError> {
        let inner = self.inner.create_surface_for_metal_layer(layer)?;
        let surface = Arc::new_cyclic(move |v| ValidationSurface {
            _this: v.clone(),
            _context: self._this.upgrade().unwrap(),
            inner,
            has_swap_chain: Default::default(),
        });
        Ok(surface)
    }

    fn get_backend_api(&self) -> BackendAPI {
        self.inner.get_backend_api()
    }
}

impl ValidationContext {
    pub fn wrap_context(inner: Arc<dyn IContext>) -> Arc<dyn IContext> {
        Arc::new_cyclic(move |v| ValidationContext {
            _this: v.clone(),
            inner,
        })
    }
}
