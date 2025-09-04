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

pub trait IDescriptorPool: IAny + IGetPlatformInterface + Send {
    /// Allocates a new individual parameter block from the pool.
    ///
    /// May fail if the pool's backing memory has been exhausted.
    ///
    /// # Warning
    ///
    /// The parameter blocks returned by a pool will by default contain invalid descriptors. That
    /// is, assume they contain uninitialized memory. It is required to update the set with fresh
    /// descriptors before use.
    ///
    /// Vulkan requires this behavior for valid API usage. Other implementations may re-use
    /// previously freed parameter blocks without zeroing out their contents meaning you may reuse
    /// stale descriptors.
    fn allocate_block(&mut self) -> Result<ParameterBlockHandle, DescriptorAllocateError>;

    /// Allocates `num_blocks` descriptors from the pool. Some implementations may be able to
    /// implement this more efficiently than naively calling [IDescriptorPool::allocate_block] in a
    /// loop.
    ///
    /// # Warning
    ///
    /// See [IDescriptorPool::allocate_block] for some pitfalls and warnings to check for.
    fn allocate_blocks(
        &mut self,
        num_blocks: usize,
    ) -> Result<Box<[ParameterBlockHandle]>, DescriptorAllocateError>;

    /// Will free the given parameter blocks, allowing them and their memory to be reused.
    ///
    /// # Safety
    ///
    /// [ParameterBlockHandle] is semantically a pointer. This function will take ownership of the
    /// set, so it is unsafe to call this function and then use the [ParameterBlockHandle] again.
    /// That would be an immediate use after free.
    ///
    /// This also means double-freeing is unsafe.
    unsafe fn free(&mut self, blocks: &[ParameterBlockHandle]);

    /// Will free all the parameter blocks allocated from the pool, resetting it to an empty state
    /// where it can allocate sets again. Even after an OOM error.
    ///
    /// # Safety
    ///
    /// The safety requirements are similar to [IDescriptorPool::free]. This will implicitly take
    /// ownership of all [ParameterBlockHandle]s allocated from the pool and free them. It is the
    /// responsibility of the caller to ensure that all handles are never re-used after they are
    /// freed.
    ///
    /// This function requires extra care as it will affect every set in the pool instead of only
    /// the individual sets requested like in 'free'.
    unsafe fn reset(&mut self);
}

#[derive(Clone)]
pub struct DescriptorPoolDesc<'a> {
    /// The parameter block layout that the descriptor pool will allocate parameter blocks for. A
    /// pool can only allocate parameter blocks with a single layout.
    pub layout: &'a dyn IParameterBlockLayout,

    /// The number of sets the pool should have capacity for. A pool is only guaranteed to have
    /// enough space for `num_blocks` parameter blocks.
    pub num_blocks: u32,

    /// The name of the object
    pub name: Option<&'a str>,
}

#[derive(Error, Debug)]
pub enum DescriptorAllocateError {
    #[error("The descriptor pool's backing memory has been exhausted due to pool fragmentation")]
    FragmentedPool,

    #[error("The descriptor pool's backing memory has been exhausted")]
    OutOfPoolMemory,

    #[error("The host or device's memory has been exhausted")]
    OutOfMemory,

    #[error("An internal backend error has occurred. Details were logged.")]
    Platform,
}
error_enum_from_unit_type!(DescriptorAllocateError);
