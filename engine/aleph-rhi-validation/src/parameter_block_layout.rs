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

use aleph_any::{AnyArc, AnyWeak, declare_interfaces};
use aleph_rhi_api::*;

use crate::device::ValidationDevice;
use crate::internal::unwrap;

pub struct ValidationParameterBlockLayout {
    pub(crate) this: AnyWeak<Self>,
    pub(crate) _device: AnyArc<ValidationDevice>,
    pub(crate) inner: AnyArc<dyn IParameterBlockLayout>,
}

declare_interfaces!(ValidationParameterBlockLayout, [IParameterBlockLayout]);

impl IParameterBlockLayout for ValidationParameterBlockLayout {
    fn upgrade(&self) -> AnyArc<dyn IParameterBlockLayout> {
        AnyArc::map::<dyn IParameterBlockLayout, _>(self.this.upgrade().unwrap(), |v| v)
    }

    fn strong_count(&self) -> usize {
        self.this.strong_count()
    }

    fn weak_count(&self) -> usize {
        self.this.weak_count()
    }

    fn desc(&self) -> &ParameterBlockDesc<'_> {
        self.inner.desc()
    }

    fn get_id(&self) -> NonZeroU64 {
        self.inner.get_id()
    }

    fn is_compatible(&self, other: &dyn IParameterBlockLayout) -> bool {
        let other = unwrap::parameter_block_layout(other);
        self.inner.is_compatible(other)
    }
}
