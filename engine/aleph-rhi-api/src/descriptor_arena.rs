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

use aleph_any::IAny;
use thiserror::Error;

use crate::*;

pub trait IDescriptorArena: IAny + IGetPlatformInterface + Send {
    /// Allocates a new individual descriptor set from the pool.
    ///
    /// May fail if the pool's backing memory has been exhausted.
    ///
    /// # Warning
    ///
    /// The descriptor sets returned by a pool will by default contain invalid descriptors. That is,
    /// assume they contain uninitialized memory. It is required to update the set with fresh
    /// descriptors before use.
    ///
    /// Vulkan requires this behavior for valid API usage. Other implementations may re-use
    /// previously freed descriptor sets without zeroing out their contents meaning you may reuse
    /// stale descriptors.
    fn allocate_set(
        &self,
        layout: &DescriptorSetLayoutHandle,
    ) -> Result<DescriptorSetHandle, DescriptorArenaAllocateError>;

    /// Allocates `num_sets` descriptors from the pool. Some implementations may be able to
    /// implement this more efficiently than naively calling [IDescriptorArena::allocate_set] in a
    /// loop.
    ///
    /// # Warning
    ///
    /// See [IDescriptorArena::allocate_set] for some pitfalls and warnings to check for.
    fn allocate_sets(
        &self,
        layout: &DescriptorSetLayoutHandle,
        num_sets: usize,
    ) -> Result<Box<[DescriptorSetHandle]>, DescriptorArenaAllocateError> {
        let mut sets = Vec::with_capacity(num_sets);
        for _ in 0..num_sets {
            sets.push(self.allocate_set(layout)?);
        }
        debug_assert_eq!(sets.len(), sets.capacity());
        debug_assert_eq!(sets.len(), num_sets);
        Ok(sets.into_boxed_slice())
    }

    /// Will free the given descriptor sets, allowing them and their memory to be reused.
    ///
    /// # Warning
    ///
    /// Depending on the [DescriptorArenaType] this arena was created with, this may not free any
    /// memory back to the arena. For those arena types it is required to call
    /// [IDescriptorArena::reset] to reset all allocations at once for memory to be freed.
    ///
    /// # Safety
    ///
    /// [DescriptorSetHandle] is semantically a pointer. This function will take ownership of the
    /// set, so it is unsafe to call this function and then use the [DescriptorSetHandle] again.
    /// That would be an immediate use after free.
    ///
    /// This also means double-freeing is unsafe.
    unsafe fn free(&self, sets: &[DescriptorSetHandle]);

    /// Will free all the descriptor sets allocated from the pool, resetting it to an empty state
    /// where it can allocate sets again. Even after an OOM error.
    ///
    /// # Safety
    ///
    /// The safety requirements are similar to [IDescriptorArena::free]. This will implicitly take
    /// ownership of all [DescriptorSetHandle]s allocated from the pool and free them. It is the
    /// responsibility of the caller to ensure that all handles are never re-used after they are
    /// freed.
    ///
    /// This function requires extra care as it will affect every set in the pool instead of only
    /// the individual sets requested like in 'free'.
    unsafe fn reset(&self);
}

#[derive(Clone)]
pub struct DescriptorArenaDesc<'a> {
    /// The type of arena, which controls which allocation algorithm is used and which features
    /// are supported.
    pub arena_type: DescriptorArenaType,

    /// The number of sets the pool should have capacity for. A pool is only guaranteed to have
    /// enough space for `num_sets` descriptor sets.
    pub num_sets: u32,

    /// The name of the object
    pub name: Option<&'a str>,
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub enum DescriptorArenaType {
    /// A linear (bump) allocation based arena. Allocating sets should be cheap(er) but freeing
    /// individual sets is not possible.
    Linear,

    /// A general purpose heap allocation based arena. Slower to allocate from but sets can be freed
    /// individually without resetting the whole arena.
    Heap,
}

impl Default for DescriptorArenaType {
    fn default() -> Self {
        Self::Linear
    }
}

#[derive(Error, Debug)]
pub enum DescriptorArenaAllocateError {
    #[error("The descriptor pool's backing memory has been exhausted due to pool fragmentation")]
    FragmentedPool,

    #[error("The descriptor pool's backing memory has been exhausted")]
    OutOfPoolMemory,

    #[error("The host or device's memory has been exhausted")]
    OutOfMemory,

    #[error("An internal backend error has occurred. Details were logged.")]
    Platform,
}
error_enum_from_unit_type!(DescriptorArenaAllocateError);
