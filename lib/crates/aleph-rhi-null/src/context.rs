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

use aleph_any::{declare_interfaces, AnyArc, AnyWeak};
use aleph_rhi_api::*;
use raw_window_handle::{HasRawDisplayHandle, HasRawWindowHandle};

use crate::{NullAdapter, NullSurface};

pub struct NullContext {
    pub(crate) _this: AnyWeak<Self>,
}

declare_interfaces!(NullContext, [IContext]);

crate::impl_platform_interface_passthrough!(NullContext);

impl NullContext {
    pub fn new_arced() -> AnyArc<dyn IContext> {
        let context = AnyArc::new_cyclic(move |v| NullContext { _this: v.clone() });
        AnyArc::map::<dyn IContext, _>(context, |v| v)
    }
}

impl IContext for NullContext {
    fn upgrade(&self) -> AnyArc<dyn IContext> {
        AnyArc::map::<dyn IContext, _>(self._this.upgrade().unwrap(), |v| v)
    }

    fn strong_count(&self) -> usize {
        self._this.strong_count()
    }

    fn weak_count(&self) -> usize {
        self._this.weak_count()
    }

    fn request_adapter(&self, _options: &AdapterRequestOptions) -> Option<AnyArc<dyn IAdapter>> {
        let adapter = AnyArc::new_cyclic(move |v| NullAdapter {
            _this: v.clone(),
            _context: self._this.upgrade().unwrap(),
        });
        Some(AnyArc::map::<dyn IAdapter, _>(adapter, |v| v))
    }

    fn create_surface(
        &self,
        _display: &dyn HasRawDisplayHandle,
        _window: &dyn HasRawWindowHandle,
    ) -> Result<AnyArc<dyn ISurface>, SurfaceCreateError> {
        let surface = AnyArc::new_cyclic(move |v| NullSurface {
            _this: v.clone(),
            _context: self._this.upgrade().unwrap(),
        });
        Ok(AnyArc::map::<dyn ISurface, _>(surface, |v| v))
    }

    fn get_backend_api(&self) -> BackendAPI {
        BackendAPI::Null
    }
}
