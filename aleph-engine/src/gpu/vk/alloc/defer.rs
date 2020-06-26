//
//
// This file is a part of Aleph
//
// <ALEPH_REPO_REPLACE>
//
// <ALEPH_LICENSE_REPLACE>
//

use crate::gpu::vk::alloc::Allocator;
use crate::gpu::vk::defer::{DeferBox, DeferList};

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
        Box::new(self)
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
