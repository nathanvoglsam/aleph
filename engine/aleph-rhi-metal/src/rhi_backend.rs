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

use std::sync::atomic::{AtomicBool, Ordering};

use aleph_any::AnyArc;
use aleph_rhi_api::{ContextCreateError, IContext};

use crate::context::Context;

pub static RHI_BACKEND_OBJECT: MetalLoader = MetalLoader {
    context_made: AtomicBool::new(false),
};

#[derive(Clone, Hash, PartialEq, Eq, Debug, Default)]
pub struct MetalConfig {}

pub struct MetalLoader {
    /// Flags whether a context has already been created
    context_made: AtomicBool,
}

impl MetalLoader {
    pub fn is_available(&self) -> bool {
        // Always true on Apple platforms, or you'd bloody hope so anyway.
        true
    }

    pub fn make_context(
        &self,
        validation: bool,
        debug: bool,
        config: &MetalConfig,
    ) -> Result<AnyArc<dyn IContext>, ContextCreateError> {
        match self
            .context_made
            .compare_exchange(false, true, Ordering::SeqCst, Ordering::SeqCst)
        {
            Ok(_) => {
                let context = AnyArc::new_cyclic(move |v| Context {
                    _this: v.clone(),
                    _config: config.clone(),
                    validation,
                    debug,
                });
                Ok(AnyArc::map::<dyn IContext, _>(context, |v| v))
            }
            Err(_) => Err(ContextCreateError::ContextAlreadyCreated),
        }
    }
}
