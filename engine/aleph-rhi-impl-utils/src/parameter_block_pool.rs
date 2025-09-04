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

use std::cell::{Cell, RefCell};
use std::mem::{MaybeUninit, needs_drop};
use std::ptr::NonNull;

use aleph_alloc::boxed::Box as BBox;
use aleph_alloc::vec::Vec as BVec;
use aleph_rhi_api::{DescriptorAllocateError, ParameterBlockHandle};

use crate::RhiGlobal;

/// A generic object pool intended to be used as part of [`aleph_rhi_api::IDescriptorPool`] and
/// [`aleph_rhi_api::IDescriptorArena`].
///
/// This is used as a backing object pool for parameter block objects. These are the objects that
/// are behind the [`ParameterBlockHandle`] handle/pointers.
///
/// This data structure is a fixed-size object pool. It must be paired with a [`IBlockFactory`]
/// factory type and instance in order to construct the parameter block objects themselves, as well
/// as any internal dynamically sized allocations.
///
/// Several different backends need a data structure like this.
pub struct ParameterBlockPool<I: IBlockFactory> {
    /// Pool we allocate block objects from
    pool: PoolStorage<I>,

    /// The number of objects allocated into 'pool'. This is the number of block objects that have
    /// been initialized into 'pool', not the number of block objects that are live and in use
    /// outside the pool. This must never exceed 'capacity'.
    num_allocated: Cell<usize>,

    /// The number of live blocks. This is strictly <= 'num_allocated' and should be equal to
    /// 'num_allocated - free_list.len()'. This represents the total number of live blocks that are
    /// in use outside the pool.
    num_blocks: Cell<usize>,

    /// Free list of descriptors
    free_list: RefCell<BVec<ParameterBlockHandle, RhiGlobal>>,

    /// Generic initializer object that handles creating the underlying parameter block objects
    /// once they have been allocated
    pub factory: RefCell<I>,
}

impl<I: IBlockFactory> ParameterBlockPool<I> {
    pub fn new(factory: I, capacity: usize) -> Self {
        let pool = BBox::new_uninit_slice_in(capacity, RhiGlobal::default());
        let num_allocated = Cell::new(0);
        let num_blocks = Cell::new(0);
        let free_list = RefCell::new(BVec::with_capacity_in(64, RhiGlobal::default()));
        let factory = RefCell::new(factory);
        Self {
            pool: PoolStorage {
                buf: NonNull::from(BBox::leak(pool)),
            },
            num_allocated,
            num_blocks,
            free_list,
            factory,
        }
    }

    /// Allocate the requested number of blocks into the given array.
    pub fn allocate_blocks<'a>(
        &self,
        p: I::Param<'a>,
        blocks: &mut [MaybeUninit<ParameterBlockHandle>],
    ) -> Result<(), DescriptorAllocateError> {
        // We can't ever allocate more than 'capacity' blocks so just immediately exit
        if blocks.len() > self.pool.buf.len() {
            return Err(DescriptorAllocateError::OutOfMemory);
        }

        // Same if we're asking for more block objects than this pool can provide
        let old_num_blocks = self.num_blocks.get();
        let new_num_blocks = old_num_blocks + blocks.len();
        if new_num_blocks > self.pool.buf.len() {
            return Err(DescriptorAllocateError::OutOfMemory);
        }

        // First try and take from the block object free list
        let mut free_list = self.free_list.borrow_mut();
        let num_from_list = usize::min(blocks.len(), free_list.len());
        {
            // Find the location in the free_list where if you split the list in two the tail will
            // be of length 'num_from_list', and split that tail off
            let start = free_list.len() - num_from_list;
            let reused_blocks = &free_list[start..];
            debug_assert_eq!(reused_blocks.len(), num_from_list);

            // Revive the blocks from the free list using the factory object.
            self.factory.borrow_mut().reuse_blocks(
                p,
                reused_blocks
                    .iter()
                    .copied()
                    .map(ParameterBlockHandle::into_raw::<I::T>),
            )?;

            // Now take that tail of handles from the free list and copy it into the output 'blocks'
            // array as the objects are now ready to use.
            let mut src = reused_blocks.iter();
            let mut dst = blocks[..num_from_list].iter_mut();
            while let (Some(src), Some(dst)) = (src.next(), dst.next()) {
                dst.write(*src);
            }

            // And finally remove those now live handles from the free list. This should always be
            // a shrinking operation so the init closure should never be called.
            free_list.resize_with(start, || unreachable!());

            // Return the free list to its cell, and update the internal 'num_blocks' tracker to
            // respect the number of blocks we have allocated from only the free list.
            //
            // If we've satisfied the whole request this should = new_num_blocks.
            self.num_blocks.set(old_num_blocks + num_from_list);
        }

        // Make sure that num_blocks doesn't exceed num_allocated at this point. Pulling from only
        // the free list should never break this assert.
        debug_assert!(self.num_blocks.get() <= self.num_allocated.get());

        // If we couldn't satisfy the whole request from the free list we must allocate new block
        // objects.
        let num_from_new = blocks.len() - num_from_list;
        if num_from_new > 0 {
            // Make sure that 'num_allocated and num_blocks' exactly match as that is the only case
            // it is correct to allocate new block objects.
            let num_allocated = self.num_allocated.get();
            debug_assert_eq!(num_allocated, self.num_blocks.get());
            debug_assert_eq!(num_allocated + num_from_new, new_num_blocks);

            // Grab a slice of the tail of the 'blocks' array that we're filling out with handles.
            // This should contain only the handles that weren't given when pulling from the free
            // list.
            let remaining = &mut blocks[num_from_list..];
            debug_assert_eq!(remaining.len(), num_from_new);

            // Allocate a fresh batch of block objects from our internal pool, then pass that list
            // into our initializer object so it can fill those objects out.
            let base_addr = unsafe {
                let addr: NonNull<MaybeUninit<I::T>> = self.pool.buf.cast();
                let addr = addr.add(num_allocated);
                let new_blocks = NonNull::slice_from_raw_parts(addr, num_from_new).as_mut();
                self.factory.borrow_mut().init_blocks(p, new_blocks)?;
                addr
            };

            // Fill out the remaining handles from our newly allocated parameter block objects.
            let mut addr = base_addr;
            for dst in remaining {
                unsafe {
                    dst.write(ParameterBlockHandle::from_raw(addr.cast()));
                    addr = addr.add(1);
                }
            }

            // 'num_blocks' is the total number of live block objects, and should now be
            // 'new_num_blocks'.
            //
            // 'num_allocated' is the total number of allocated block objects, and in this case
            // should now also be 'new_num_blocks'.
            self.num_blocks.set(new_num_blocks);
            self.num_allocated.set(new_num_blocks);
        }

        Ok(())
    }

    /// Free the requested number of blocks
    pub fn free_blocks(&self, blocks: &[ParameterBlockHandle]) {
        // First we give the list of blocks to free to the factory so it can free any internal
        // allocations in the blocks.
        self.factory.borrow_mut().free_blocks(
            blocks
                .iter()
                .copied()
                .map(ParameterBlockHandle::into_raw::<I::T>),
        );

        // Add the freed blocks to the free list
        let mut free_list = self.free_list.borrow_mut();
        free_list.extend_from_slice(blocks);

        // And update 'num_blocks' to respect the total number of allocated blocks now that we've
        // returned some to the pool.
        self.num_blocks.set(self.num_blocks.get() - blocks.len());
    }

    pub unsafe fn reset_pool(&self) {
        unsafe {
            // Reset all the blocks in the pool back to the empty state. This is done by calling
            // into the factory. Factory implementations beware: All objects will be initialized,
            // but some may not be live (i.e. have been freed).
            let data: NonNull<I::T> = self.pool.buf.cast();
            let mut all_blocks: NonNull<[I::T]> =
                NonNull::slice_from_raw_parts(data, self.num_allocated.get());
            self.factory.borrow_mut().reset_blocks(all_blocks.as_mut());

            // Clear the free list
            let mut free_list = self.free_list.borrow_mut();
            free_list.clear();

            // And reset our allocation counters to 0. We're now ready to use this pool anew.
            self.num_allocated.set(0);
            self.num_blocks.set(0);
        }
    }
}

impl<I: IBlockFactory> Drop for ParameterBlockPool<I> {
    fn drop(&mut self) {
        unsafe {
            // Walk through all live block objects and pass them to the initializer to free their
            // internal allocations.
            let data: NonNull<I::T> = self.pool.buf.cast();
            let mut all_blocks: NonNull<[I::T]> =
                NonNull::slice_from_raw_parts(data, self.num_allocated.get());
            self.factory.get_mut().drop_blocks(all_blocks.as_mut());

            // Once the initializer has been able to release the internal allocations we can
            // drop the objects if needed.
            if needs_drop::<I::T>() {
                let mut addr: NonNull<I::T> = self.pool.buf.cast();
                for _ in 0..self.num_allocated.get() {
                    addr.drop_in_place();
                    addr = addr.add(1);
                }
            }

            // Now we can free the backing pool allocation.
            drop(BBox::from_raw_in(
                self.pool.buf.as_ptr(),
                RhiGlobal::default(),
            ));
        }
    }
}

struct PoolStorage<I: IBlockFactory> {
    buf: NonNull<[MaybeUninit<I::T>]>,
}

unsafe impl<I: IBlockFactory> Send for PoolStorage<I> {}

/// Factory object for creating parameter block objects within a [`ParameterBlockPool`].
///
/// This handles constructing new block objects, as well as releasing any internal allocations.
/// There is also a hook for re-using blocks from the free-list, as some regimes can re-use the
/// allocations inside a parameter block.
pub unsafe trait IBlockFactory {
    type Param<'a>: Copy;

    /// The actual type of the parameter block object. Must be sized.
    type T: Sized + Send + Sync + 'static;

    /// Initializes each block in the given list in-place to be fully ready for use.
    fn init_blocks<'a>(
        &mut self,
        p: Self::Param<'a>,
        blocks: &mut [MaybeUninit<Self::T>],
    ) -> Result<(), DescriptorAllocateError>;

    /// Given a list of parameter block objects (pointers), this hook enables reviving them so they
    /// can be reused. It's possible in some cases to re-use the allocations, this hook allows
    /// checking and replacing them if they can't be.
    fn reuse_blocks<'a>(
        &mut self,
        p: Self::Param<'a>,
        blocks: impl Iterator<Item = NonNull<Self::T>>,
    ) -> Result<(), DescriptorAllocateError>;

    /// Release all internal allocations in the given block while leaving the blocks as live
    /// objects. It is invalid to use a parameter block after calling this hook on it. Only after
    /// calling [`IBlockFactory::reuse_blocks`] on it is it correct to reuse the block again.
    ///
    /// All blocks given to this function are assumed to be live. Giving this function a block
    /// object that is not in the live state is considered a user-after-free bug.
    fn free_blocks(&mut self, blocks: impl Iterator<Item = NonNull<Self::T>>);

    /// Release all internal allocations in the given block while leaving the blocks as live
    /// objects. It is invalid to use a parameter block after calling this hook on it. Only after
    /// calling [`IBlockFactory::reuse_blocks`] on it is it correct to reuse the block again.
    ///
    /// This is called in [`ParameterBlockPool::reset_pool`], and is conceptually an alternate form
    /// of [`IBlockFactory::free_blocks`]. The blocks in the given array may or may not be live.
    /// You must be able ot handle block objects that have already had
    /// [`IBlockFactory::free_blocks`] called on them.
    fn reset_blocks(&mut self, blocks: &mut [Self::T]);

    /// An alternate form of [`IBlockFactory::reset_blocks`] that will be called only when the
    /// parameter block pool itself is being dropped. This should free all internal allocations in
    /// the parameter block, similar to 'reset_blocks'.
    ///
    /// This function allows for implementations to skip freeing certain internal allocations when
    /// the pool is being dropped. For example, if the allocation is from some internal pool where
    /// the work to free the allocation is pointless because the whole pool will be dropped too.
    ///
    /// The default implementation makes this an alias for [`IBlockFactory::reset_blocks`]. It is
    /// up to the implementation to provide an optimized version, and to ensure both are correct.
    fn drop_blocks(&mut self, blocks: &mut [Self::T]) {
        self.reset_blocks(blocks)
    }
}
