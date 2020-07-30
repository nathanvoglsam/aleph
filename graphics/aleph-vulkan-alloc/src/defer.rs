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

use crate::{Allocation, Allocator};
use aleph_vulkan_core::erupt::vk1_0::{Buffer, Image};
use aleph_vulkan_core::{DeferBox, DeferList};

///
/// Trait bound for a function/closure that can be consumed by the device defer list
///
pub trait AllocatorDeferFn: FnOnce(&Allocator) + 'static {}

///
/// Blanket implementation for all functions/closures that match the interface of `DeviceDeferFn`
///
impl<T> AllocatorDeferFn for T where T: FnOnce(&Allocator) + 'static {}

///
/// Trait for allowing to generically convert objects into boxed defer functions
///
pub trait IntoAllocatorDeferBox {
    fn into_allocator_defer_box(self) -> DeferBox<dyn AllocatorDeferFn>;
}

///
/// Blanket implementation for all functions/closures that match the interface of `DeviceDeferFn`
///
impl<T: AllocatorDeferFn> IntoAllocatorDeferBox for T {
    fn into_allocator_defer_box(self) -> DeferBox<dyn AllocatorDeferFn> {
        DeferBox::new(self)
    }
}

pub struct AllocatorDeferList {
    list: DeferList<dyn AllocatorDeferFn>,
}

impl AllocatorDeferList {
    ///
    /// Constructs a new `DeviceDeferList`
    ///
    pub fn new() -> Self {
        Self {
            list: DeferList::new(),
        }
    }

    ///
    /// Adds a new item to the defer list
    ///
    pub fn add<T: IntoAllocatorDeferBox>(&self, item: T) {
        self.list.add(item.into_allocator_defer_box());
    }

    ///
    /// Consumes all the items currently in the list while running the given functor on each item
    ///
    pub fn consume(&self, allocator: &Allocator) {
        self.list.consume(|func| {
            func(allocator);
        });
    }
}

// =================================================================================================
// Trait implementations for IntoAllocatorDeferBox for various resources
// =================================================================================================

impl IntoAllocatorDeferBox for (Buffer, Allocation) {
    fn into_allocator_defer_box(self) -> DeferBox<dyn AllocatorDeferFn> {
        DeferBox::new(move |allocator: &Allocator| unsafe {
            allocator.destroy_buffer(self.0, self.1);
        })
    }
}

impl IntoAllocatorDeferBox for (Image, Allocation) {
    fn into_allocator_defer_box(self) -> DeferBox<dyn AllocatorDeferFn> {
        DeferBox::new(move |allocator: &Allocator| unsafe {
            allocator.destroy_image(self.0, self.1);
        })
    }
}
