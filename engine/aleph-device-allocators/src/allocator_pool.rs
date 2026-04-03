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

use std::mem::ManuallyDrop;
use std::ops::{Deref, DerefMut};
use std::sync::{Arc, Weak};

use aleph_rhi_api::IDevice;
use crossbeam::queue::ArrayQueue;

use crate::LinearDescriptorPool;

/// Interface expected of a type that can build allocators for an [`AllocatorPool`].
///
/// Expected to close over some state and a function that yields fresh objects when the pool's
/// internal queue is exhausted. Must be Send+Sync like [`AllocatorPool`].
pub trait IAllocatorFactory: Send + Sync {
    /// The type we yield from the factory. Doesn't require Sync but must at least be Send.
    type Out: Send;

    /// The builder function itself. Takes it's internal state and yields exactly one
    /// [`IAllocatorFactory::Out`] object.
    fn build(&self) -> Self::Out;
}

/// A thread safe, [`Send`] + [`Sync`] object pool that can safely hand out allocators which are
/// only [`Send`] and don't implement [`Sync`].
///
/// This structure is generic over a [`IAllocatorFactory`] type, which is a state object which
/// allows constructing new pool objects when the queue is exhausted.
pub struct AllocatorPool<T: IAllocatorFactory> {
    this: Weak<AllocatorPool<T>>,
    factory: T,
    queue: ArrayQueue<T::Out>,
}

impl<T: IAllocatorFactory> AllocatorPool<T> {
    /// Constructs a new [`AllocatorPool`] with space for `capacity` items and the given factory
    /// object which is used for building fresh pool items when the queue is exhausted.
    ///
    /// [`AllocatorPool`] is always boxed in an [`Arc`] so that the items allocated from the pool
    /// can be sent back to the pool when they fall out of scope.
    ///
    /// It may be useful to tweak `capacity` so that you never reach that number of items allocated
    /// from one of these pools as this will create lots of object churn which can be quite
    /// expensive.
    pub fn new(factory: T, capacity: usize) -> Arc<Self> {
        Arc::new_cyclic(|v| Self {
            this: v.clone(),
            factory,
            queue: ArrayQueue::new(capacity),
        })
    }

    /// Will return one ready to use [`IAllocatorFactory::Out`] item. This may be an existing item
    /// being reused or it could be a freshly allocated item if the queue was exhausted.
    pub fn get(&self) -> AllocatorPoolItem<T> {
        let item = match self.queue.pop() {
            Some(item) => item,
            _ => self.factory.build(),
        };
        AllocatorPoolItem {
            pool: self.this.upgrade().unwrap(),
            item: ManuallyDrop::new(item),
        }
    }
}

/// Smart wrapper over an item returned from an [`AllocatorPool`]. Wraps an
/// [`IAllocatorFactory::Out`] object as well as a handle back to the pool so that the item can be
/// returned to the pool when it goes out of scope.
pub struct AllocatorPoolItem<T: IAllocatorFactory> {
    pool: Arc<AllocatorPool<T>>,
    item: ManuallyDrop<T::Out>,
}

impl<T: IAllocatorFactory> AsRef<T::Out> for AllocatorPoolItem<T> {
    #[inline(always)]
    fn as_ref(&self) -> &T::Out {
        self.item.deref()
    }
}

impl<T: IAllocatorFactory> AsMut<T::Out> for AllocatorPoolItem<T> {
    #[inline(always)]
    fn as_mut(&mut self) -> &mut T::Out {
        self.item.deref_mut()
    }
}

impl<T: IAllocatorFactory> Deref for AllocatorPoolItem<T> {
    type Target = T::Out;

    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        self.item.deref()
    }
}

impl<T: IAllocatorFactory> DerefMut for AllocatorPoolItem<T> {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.item.deref_mut()
    }
}

impl<T: IAllocatorFactory> Drop for AllocatorPoolItem<T> {
    fn drop(&mut self) {
        // Safety: This is the only place we ever take from the item's place. Because this is the
        //         Drop function the item can't be observed elswhere so it's safe to take as nobody
        //         else can take from it after us.
        let item = unsafe { ManuallyDrop::take(&mut self.item) };
        let _ = self.pool.queue.push(item);
    }
}

/// An [`IAllocatorFactory`] implementation that builds [`LinearDescriptorPool`] objects.
pub struct LinearDescriptorPoolFactory {
    device: Arc<dyn IDevice>,
    default_block_num: u32,
}

impl LinearDescriptorPoolFactory {
    /// Constructs a new [`LinearDescriptorPoolFactory`] over the given 'device' and
    /// 'default_block_num'.
    ///
    /// 'default_block_num' is the initial size of a [`LinearDescriptorPool`] object when it is
    /// created by a factory. Be sure to tweak this to your workload if relevant.
    pub const fn new(device: Arc<dyn IDevice>, default_block_num: u32) -> Self {
        Self {
            device,
            default_block_num,
        }
    }
}

impl IAllocatorFactory for LinearDescriptorPoolFactory {
    type Out = LinearDescriptorPool;

    fn build(&self) -> Self::Out {
        LinearDescriptorPool::new(self.device.as_ref(), self.default_block_num).unwrap()
    }
}
