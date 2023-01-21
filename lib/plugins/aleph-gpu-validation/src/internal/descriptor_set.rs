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

use crate::descriptor_set_layout::ValidationDescriptorSetLayout;
use interfaces::any::AnyArc;
use interfaces::gpu::DescriptorSetHandle;
use std::ptr::NonNull;

pub struct DescriptorSet {
    /// The descriptor set layout of this set
    pub _layout: AnyArc<ValidationDescriptorSetLayout>,

    /// The inner handle
    pub inner: DescriptorSetHandle,
}

impl DescriptorSet {
    /// Grabs the pointer inside a [DescriptorSetHandle] as a non-null [DescriptorSet] ptr
    pub fn ptr_from_handle(handle: DescriptorSetHandle) -> NonNull<DescriptorSet> {
        let inner: NonNull<()> = handle.into();
        inner.cast()
    }

    /// Constructs a reference to a [DescriptorSet] from the given handle, with the lifetime
    /// attached to the lifetime of the handle.
    ///
    /// # Safety
    ///
    /// This function has all the same soundness requirements as creating a reference from a raw
    /// pointer. At the very least a [DescriptorSetHandle] is guaranteed to be non-null, but many
    /// things are not known at this call-site without the caller tracking these things themselves.
    ///
    /// - It is the caller's responsibility to ensure that no mutable reference can exist at the
    ///   same time as the reference this function creates. If it can't be proven statically then
    ///   locks must be used. This in general means:
    ///     - DescriptorSet objects themselves are immutable, so access to the set object itself
    ///       requires no synchronization. But the descriptor memory that the sets point to
    ///       *is not* immutable. The caller must synchronize access to the descriptor memory.
    /// - It is the caller's responsibility to ensure the handle still points to a live set object.
    ///   This means:
    ///     - The caller must ensure that they do not use any sets after the pool they were
    ///       allocated from are destroyed.
    /// - It is the caller's responsibility to ensure the handle points to a value of the correct
    ///   type. This is more subtle, but:
    ///     - Handles allocated from a different device will point to a different type. They will
    ///       also point to descriptor memory on another device. Thus it becomes a rust soundness
    ///       issue *as well as* an API soundness issue as using a set from one device on another
    ///       device is invalid as the set allocation is local to the device it was created from.
    ///     - The caller must ensure that they only use set handles with the device they were
    ///       created from. If the two devices use different implementations then the handles
    ///       *will* be interpreted as the incorrect type.
    ///
    pub unsafe fn ref_from_handle(handle: &DescriptorSetHandle) -> &DescriptorSet {
        let ptr = Self::ptr_from_handle(handle.clone());

        // Safety: all left to the caller lmao
        ptr.as_ref()
    }
}
