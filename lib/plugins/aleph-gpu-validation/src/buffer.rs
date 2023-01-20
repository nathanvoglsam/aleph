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

use crate::device::Device;
use interfaces::any::{AnyArc, AnyWeak};
use interfaces::gpu::{BufferDesc, IBuffer, INamedObject, ResourceMapError};
use std::ptr::NonNull;
use std::sync::atomic::{AtomicBool, Ordering};

pub struct Buffer {
    pub(crate) _this: AnyWeak<Self>,
    pub(crate) _device: AnyArc<Device>,
    pub(crate) inner: AnyArc<dyn IBuffer>,
    pub(crate) debug_mapped_tracker: AtomicBool,
}

crate::validation_declare_interfaces!(Buffer, [IBuffer]);

impl IBuffer for Buffer {
    fn upgrade(&self) -> AnyArc<dyn IBuffer> {
        AnyArc::map::<dyn IBuffer, _>(self._this.upgrade().unwrap(), |v| v)
    }

    fn strong_count(&self) -> usize {
        self._this.strong_count()
    }

    fn weak_count(&self) -> usize {
        self._this.weak_count()
    }

    fn desc(&self) -> &BufferDesc {
        self.inner.desc()
    }

    fn map(&self) -> Result<NonNull<u8>, ResourceMapError> {
        // Debug check for tracking that the resource is unmapped when trying to map it
        debug_assert!(self
            .debug_mapped_tracker
            .compare_exchange(false, true, Ordering::Relaxed, Ordering::Relaxed)
            .is_ok());

        self.inner.map()
    }

    fn unmap(&self) {
        // Debug check for tracking that the resource is mapped when trying to unmap it
        debug_assert!(self
            .debug_mapped_tracker
            .compare_exchange(true, false, Ordering::Relaxed, Ordering::Relaxed)
            .is_ok());

        self.inner.unmap();
    }

    fn flush_range(&self, offset: u64, len: u64) {
        self.inner.flush_range(offset, len);
    }

    fn invalidate_range(&self, offset: u64, len: u64) {
        self.inner.invalidate_range(offset, len);
    }
}

impl INamedObject for Buffer {
    fn set_name(&self, name: &str) {
        self.inner.set_name(name)
    }
}
