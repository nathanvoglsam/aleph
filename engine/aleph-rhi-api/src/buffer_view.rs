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

use std::ptr::NonNull;

use crate::*;

#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Debug)]
pub struct BufferView(NonNull<()>);

impl BufferView {
    /// Gets a 'default' buffer view for the given buffer and format.
    ///
    /// This is a convenience function that constructs a [`BufferViewDesc`] for the buffer and
    /// format, and then retrieves the corresponding buffer view from the device.
    #[inline]
    pub fn get_for(
        device: &dyn IDevice,
        buffer: &BufferHandle,
        format: Format,
    ) -> Result<Self, ()> {
        device.get_buffer_view(buffer, &BufferViewDesc::for_buffer(device, buffer, format))
    }

    /// Unsafe utility function for constructing a new [`BufferView`] from a raw pointer.
    ///
    /// # Safety
    ///
    /// This technically doesn't cause immediate UB, the implementation is safe, but doing this
    /// outside of an RHI implementation is almost certainly incorrect. This function is marked as
    /// unsafe to discourage using it. There should be zero need to call this unless you're
    /// constructing handles from internal RHI types.
    ///
    /// This function exists to avoid using *actual* unsafe via [core::mem::transmute] to allow
    /// RHI implementations to construct this otherwise opaque type safely.
    pub unsafe fn from_raw(v: NonNull<()>) -> Self {
        Self(v)
    }

    /// The inverse of [`BufferView::from_raw`].
    pub fn into_raw<U>(self) -> NonNull<U> {
        self.0.cast()
    }

    /// # Safety
    ///
    /// See [`BufferView::from_raw`]
    pub unsafe fn from_raw_int(v: u64) -> Option<Self> {
        NonNull::new(v as *mut ()).map(Self)
    }

    /// The inverse of [`BufferView::from_raw_int`].
    pub fn into_raw_int(self) -> u64 {
        self.0.as_ptr() as usize as u64
    }
}

unsafe impl Send for BufferView {}
unsafe impl Sync for BufferView {}

#[derive(Clone, Eq, PartialEq, Hash, Debug)]
pub struct BufferViewDesc {
    /// The format that the buffer will be viewed as through this descriptor
    pub format: Format,

    /// The offset into the buffer, in 'elements' of the view's format, where the view starts.
    pub offset: u64,

    /// The number of buffer elements in this view, in terms of the view's format.
    ///
    /// If set to u64::MAX the view will cover the entire buffer starting from the specified offset.
    pub num_elements: u64,

    /// Whether the buffer can be written to through this descriptor.
    pub writable: bool,
}

impl BufferViewDesc {
    /// Returns the [`BufferViewDesc`] as-is, except with [`BufferViewDesc::format`] set to the given
    /// format.
    pub const fn with_format(mut self, format: Format) -> BufferViewDesc {
        self.format = format;
        self
    }

    /// Returns the [`BufferViewDesc`] as-is, except with [`BufferViewDesc::writable`] set to the
    /// given value.
    pub const fn with_writable(mut self, writable: bool) -> BufferViewDesc {
        self.writable = writable;
        self
    }

    /// Constructs a new [`BufferViewDesc`] for the given buffer and format.
    ///
    /// The desc will create a view covering the entire buffer with the specified format.
    #[inline]
    pub fn for_buffer(
        device: &dyn IDevice,
        buffer: &BufferHandle,
        format: Format,
    ) -> BufferViewDesc {
        let desc = device.get_buffer_desc(buffer);
        Self::for_desc(desc, format)
    }

    /// Constructs a new [`BufferViewDesc`] for the given buffer desc and format.
    ///
    /// The desc will create a view covering the entire buffer with the specified format.
    #[inline]
    pub fn for_desc(desc: &BufferDesc, format: Format) -> BufferViewDesc {
        debug_assert!(desc.usage.contains(ResourceUsageFlags::SHADER_RESOURCE));
        BufferViewDesc {
            format: format,
            offset: 0,
            num_elements: u64::MAX,
            writable: false,
        }
    }
}
