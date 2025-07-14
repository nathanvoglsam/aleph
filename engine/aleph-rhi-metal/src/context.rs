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

use std::any::TypeId;
use std::ffi::c_void;
use std::ptr::NonNull;

use aleph_any::{AnyArc, AnyWeak, declare_interfaces};
use aleph_rhi_api::*;

use raw_window_handle::{HasDisplayHandle, HasWindowHandle};

use crate::MetalConfig;

pub struct Context {
    pub _this: AnyWeak<Self>,
    pub config: MetalConfig,
    pub validation: bool,
    pub debug: bool,
}

declare_interfaces!(Context, [IContext]);

impl IGetPlatformInterface for Context {
    unsafe fn __query_platform_interface(&self, _target: TypeId, _out: *mut ()) -> Option<()> {
        // TODO: expose the instance loader via an arc or something
        None
    }
}

impl IContext for Context {
    fn upgrade(&self) -> AnyArc<dyn IContext> {
        AnyArc::map::<dyn IContext, _>(self._this.upgrade().unwrap(), |v| v)
    }

    fn strong_count(&self) -> usize {
        self._this.strong_count()
    }

    fn weak_count(&self) -> usize {
        self._this.weak_count()
    }

    fn request_adapter(&self, options: &AdapterRequestOptions) -> Option<AnyArc<dyn IAdapter>> {
        todo!()
    }

    fn create_surface(
        &self,
        _display: &dyn HasDisplayHandle,
        _window: &dyn HasWindowHandle,
    ) -> Result<AnyArc<dyn ISurface>, SurfaceCreateError> {
        unimplemented!("Use IContext::create_surface_for_metal_layer")
    }

    fn create_surface_for_metal_layer(
        &self,
        layer: NonNull<c_void>,
    ) -> Result<AnyArc<dyn ISurface>, SurfaceCreateError> {
        todo!()
    }

    fn get_backend_api(&self) -> BackendAPI {
        BackendAPI::Metal
    }
}

impl Drop for Context {
    fn drop(&mut self) {
        todo!()
    }
}
