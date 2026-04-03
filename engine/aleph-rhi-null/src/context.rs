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

use crate::{NullAdapter, NullSurface};

pub struct NullContext {
    pub(crate) _this: Weak<Self>,
}

crate::impl_platform_interface_passthrough!(NullContext);

impl NullContext {
    pub fn new_arced() -> Arc<dyn IContext> {
        Arc::new_cyclic(move |v| NullContext { _this: v.clone() })
    }
}

impl IContext for NullContext {
    fn upgrade(&self) -> Arc<dyn IContext> {
        self._this.upgrade().unwrap()
    }

    fn strong_count(&self) -> usize {
        self._this.strong_count()
    }

    fn weak_count(&self) -> usize {
        self._this.weak_count()
    }

    fn request_adapter(&self, _options: &AdapterRequestOptions) -> Option<Arc<dyn IAdapter>> {
        let adapter = Arc::new_cyclic(move |v| NullAdapter {
            _this: v.clone(),
            _context: self._this.upgrade().unwrap(),
        });
        Some(adapter)
    }

    fn create_surface(
        &self,
        _display: &dyn HasDisplayHandle,
        _window: &dyn HasWindowHandle,
    ) -> Result<Arc<dyn ISurface>, SurfaceCreateError> {
        let surface = Arc::new_cyclic(move |v| NullSurface {
            _this: v.clone(),
            _context: self._this.upgrade().unwrap(),
        });
        Ok(surface)
    }

    fn create_surface_for_metal_layer(
        &self,
        _layer: NonNull<c_void>,
    ) -> Result<Arc<dyn ISurface>, SurfaceCreateError> {
        let surface = Arc::new_cyclic(move |v| NullSurface {
            _this: v.clone(),
            _context: self._this.upgrade().unwrap(),
        });
        Ok(surface)
    }

    fn get_backend_api(&self) -> BackendAPI {
        BackendAPI::Null
    }
}
