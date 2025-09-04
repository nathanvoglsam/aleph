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

use aleph_alloc::BBox;
use aleph_any::AnyArc;
use aleph_object_system::unsafe_impl_iobject;
use aleph_rhi_api::*;
use aleph_rhi_impl_utils::RhiSystem;

use crate::ValidationDevice;

pub struct ValidationBuffer {
    pub(crate) _device: AnyArc<ValidationDevice>,
    pub(crate) size: u64,
    pub(crate) usage: ResourceUsageFlags,
    pub(crate) name: Option<BBox<str, RhiSystem>>,
    pub(crate) inner: BufferHandle,
}

unsafe_impl_iobject!(ValidationBuffer, "01944e4f-dacb-7122-bdec-ae9da89e1a0e");

impl ValidationBuffer {
    pub(crate) fn get(v: &BufferHandle) -> &Self {
        v.get()
            .downcast_ref::<Self>()
            .expect("Unknown Buffer implementation!")
    }

    pub fn validate_range(&self, offset: u64, len: u64) {
        let size = self.size;
        assert!(
            offset < size,
            "Invalidation range (offset: {offset}, len: {len}) outside buffer size ({size})",
        );
        assert!(
            offset + len < size,
            "Invalidation range (offset: {offset}, len: {len}) outside buffer size ({size})",
        );
    }
}
