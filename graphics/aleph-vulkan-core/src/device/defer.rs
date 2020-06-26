//
//
// This file is a part of Aleph
//
// <ALEPH_REPO_REPLACE>
//
// <ALEPH_LICENSE_REPLACE>
//

use crate::defer::{DeferBox, DeferList};
use crate::Device;
use erupt::vk1_0::{CommandPool, Vk10DeviceLoaderExt};

///
/// Trait bound for a function/closure that can be consumed by the device defer list
///
pub trait DeviceDeferFn: FnOnce(&Device) + 'static {}

///
/// Blanket implementation for all functions/closures that match the interface of `DeviceDeferFn`
///
impl<T> DeviceDeferFn for T where T: FnOnce(&Device) + 'static {}

///
/// Trait for allowing to generically convert objects into boxed defer functions
///
pub trait IntoDeviceDeferBox {
    fn into_device_defer_box(self) -> DeferBox<dyn DeviceDeferFn>;
}

///
/// Blanket implementation for all functions/closures that match the interface of `DeviceDeferFn`
///
impl<T: DeviceDeferFn> IntoDeviceDeferBox for T {
    fn into_device_defer_box(self) -> DeferBox<dyn DeviceDeferFn> {
        DeferBox::new(self)
    }
}

pub struct DeviceDeferList {
    list: DeferList<dyn DeviceDeferFn>,
}

impl DeviceDeferList {
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
    pub fn add<T: IntoDeviceDeferBox>(&self, item: T) {
        self.list.add(item.into_device_defer_box());
    }

    ///
    /// Consumes all the items currently in the list while running the given functor on each item
    ///
    pub fn consume(&self, device: &Device) {
        self.list.consume(|func| {
            func(device);
        });
    }
}

// =================================================================================================
// Trait implementations for IntoAllocatorDeferBox for various resources
// =================================================================================================

impl IntoDeviceDeferBox for CommandPool {
    fn into_device_defer_box(self) -> DeferBox<dyn DeviceDeferFn> {
        DeferBox::new(move |device: &Device| unsafe {
            device.loader().destroy_command_pool(self, None);
        })
    }
}
