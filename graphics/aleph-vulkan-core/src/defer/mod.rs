//
//
// This file is a part of Aleph
//
// <ALEPH_REPO_REPLACE>
//
// <ALEPH_LICENSE_REPLACE>
//

use crossbeam::queue::SegQueue;

///
/// Type alias for the box used for storing deferral objects
///
pub type DeferBox<T> = Box<T>;

///
/// A generic defer list
///
/// This should not be used as a direct public interface. It should be used behind a wrapper type
/// that provides more concrete rules for the lifetime of the DeferList object.
///
pub struct DeferList<T: ?Sized> {
    list: SegQueue<DeferBox<T>>,
}

impl<T: ?Sized> DeferList<T> {
    ///
    /// Creates a new device defer list
    ///
    pub fn new() -> Self {
        Self {
            list: Default::default(),
        }
    }

    ///
    /// Adds a defer item into
    ///
    pub fn add(&self, item: DeferBox<T>) {
        self.list.push(item);
    }

    ///
    /// Consume all deferred items by iterating over the list and calling the functor for each item.
    ///
    pub fn consume(&self, mut func: impl FnMut(DeferBox<T>)) {
        while let Ok(item) = self.list.pop() {
            func(item);
        }
    }
}
