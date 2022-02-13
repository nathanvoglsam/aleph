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

use crate::resource::{Any, Resource};
use crossbeam::atomic::AtomicCell;
use std::sync::Arc;

///
/// A slot returned to a render pass during the setup phase for declaring a resource access. The
/// slot represents the method for delivering the concrete resource to the render pass during the
/// recording stage.
///
pub struct ResourceSlot {
    pub(crate) inner: Arc<AtomicCell<Option<Resource<Any>>>>,
}

impl ResourceSlot {
    /// Take the resource from the slot, leaving the slot empty
    ///
    /// # Panics
    ///
    /// Will panic if the slot is empty, only call this once per render graph recording.
    pub fn take(&self) -> Resource<Any> {
        // Ensure we get a lock free implementation of atomic cell. Option<dx12::Resource> should be
        // a single pointer in size so this should be lock free
        assert!(AtomicCell::<Option<Resource<Any>>>::is_lock_free());

        // Take from the slot, panicking if the resource has already been taken///
        self.inner.take().unwrap()
    }
}