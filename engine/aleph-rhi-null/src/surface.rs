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

use std::sync::{Arc, Weak};

use aleph_rhi_api::*;

use crate::internal::unwrap;
use crate::{NullContext, NullSwapChain};

pub struct NullSurface {
    pub(crate) _this: Weak<Self>,
    pub(crate) _context: Arc<NullContext>,
}

crate::impl_platform_interface_passthrough!(NullSurface);

impl ISurface for NullSurface {
    fn upgrade(&self) -> Arc<dyn ISurface> {
        self._this.upgrade().unwrap()
    }

    fn strong_count(&self) -> usize {
        self._this.strong_count()
    }

    fn weak_count(&self) -> usize {
        self._this.weak_count()
    }

    fn create_swap_chain(
        &self,
        device: &dyn IDevice,
        config: &SwapChainConfiguration,
    ) -> Result<Arc<dyn ISwapChain>, SwapChainCreateError> {
        let device = unwrap::device(device);

        let swap_chain = Arc::new_cyclic(move |v| NullSwapChain {
            _this: v.clone(),
            _device: device._this.upgrade().unwrap(),
            _surface: self._this.upgrade().unwrap(),
            config: config.clone(),
        });
        Ok(swap_chain)
    }
}
