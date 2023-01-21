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

use crate::adapter::ValidationAdapter;
use crate::surface::ValidationSurface;
use interfaces::any::{AnyArc, AnyWeak, QueryInterface};
use interfaces::gpu::{
    AdapterRequestOptions, BackendAPI, IAdapter, IContext, ISurface, SurfaceCreateError,
};
use interfaces::platform::HasRawWindowHandle;

pub struct ValidationContext {
    pub(crate) _this: AnyWeak<Self>,
    pub(crate) inner: AnyArc<dyn IContext>,
}

crate::validation_declare_interfaces!(ValidationContext, [IContext]);

impl IContext for ValidationContext {
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
        // Unwrap the ISurface reference to the inner object
        let mut options = options.clone();
        options.surface = options.surface.map(|v| {
            v.query_interface::<ValidationSurface>()
                .expect("Unknown ISurface implementation")
                .inner
                .as_ref()
        });

        let inner = self.inner.request_adapter(&options)?;
        let adapter = AnyArc::new_cyclic(move |v| ValidationAdapter {
            _this: v.clone(),
            _context: self._this.upgrade().unwrap(),
            inner,
        });
        Some(AnyArc::map::<dyn IAdapter, _>(adapter, |v| v))
    }

    fn create_surface(
        &self,
        window: &dyn HasRawWindowHandle,
    ) -> Result<AnyArc<dyn ISurface>, SurfaceCreateError> {
        let inner = self.inner.create_surface(window)?;
        let surface = AnyArc::new_cyclic(move |v| ValidationSurface {
            _this: v.clone(),
            _context: self._this.upgrade().unwrap(),
            inner,
            has_swap_chain: Default::default(),
        });
        Ok(AnyArc::map::<dyn ISurface, _>(surface, |v| v))
    }

    fn get_backend_api(&self) -> BackendAPI {
        self.inner.get_backend_api()
    }
}

impl ValidationContext {
    pub fn wrap_context(inner: AnyArc<dyn IContext>) -> AnyArc<dyn IContext> {
        let adapter = AnyArc::new_cyclic(move |v| ValidationContext {
            _this: v.clone(),
            inner,
        });
        AnyArc::map::<dyn IContext, _>(adapter, |v| v)
    }
}
