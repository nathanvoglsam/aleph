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

use std::cell::Cell;

use aleph_interfaces::any::AnyArc;
use aleph_rhi_api::*;

/// A descriptor set allocator that grabs arenas as pages and bump allocates descriptor sets from
/// those pages. Intended for cheaply allocating one-time-use descriptor sets.
pub struct LinearDescriptorPool {
    /// The device we're working with
    device: AnyArc<dyn IDevice>,

    /// The active descriptor arena
    active: Cell<Option<Box<dyn IDescriptorArena>>>,

    /// The num_sets value used to allocate the current active arena.
    last_num_sets: Cell<u32>,

    /// The list of descriptor arenas that have been exhausted. We have to keep them around so that
    /// the arena stays live
    exhausted: Cell<Vec<Box<dyn IDescriptorArena>>>,
}

impl LinearDescriptorPool {
    pub fn new(device: &dyn IDevice, num_sets: u32) -> Result<Self, DescriptorPoolCreateError> {
        let active = device.create_descriptor_arena(&DescriptorArenaDesc {
            arena_type: DescriptorArenaType::Linear,
            num_sets,
            name: Some("LinearDescriptorPoolArena"),
        })?;
        Ok(Self {
            device: device.upgrade(),
            active: Cell::new(Some(active)),
            last_num_sets: Cell::new(num_sets),
            exhausted: Cell::new(Vec::new()),
        })
    }

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
    pub fn allocate_set(
        &self,
        layout: &dyn IDescriptorSetLayout,
    ) -> Result<DescriptorSetHandle, DescriptorPoolAllocateError> {
        use DescriptorPoolAllocateError::*;

        let active = self.active.take().unwrap();
        match active.allocate_set(layout) {
            Ok(v) => {
                assert!(self.active.replace(Some(active)).is_none());
                Ok(v)
            }
            Err(FragmentedPool) | Err(OutOfMemory) | Err(OutOfPoolMemory) => {
                let active = self.grow(active)?;
                let result = active.allocate_set(layout);
                assert!(self.active.replace(Some(active)).is_none());
                result
            }
            v @ _ => v,
        }
    }

    /// Allocates `num_sets` descriptors from the pool. Some implementations may be able to
    /// implement this more efficiently than naively calling [IDescriptorArena::allocate_set] in a
    /// loop.
    ///
    /// # Warning
    ///
    /// See [IDescriptorArena::allocate_set] for some pitfalls and warnings to check for.
    pub fn allocate_sets(
        &self,
        layout: &dyn IDescriptorSetLayout,
        num_sets: usize,
    ) -> Result<Vec<DescriptorSetHandle>, DescriptorPoolAllocateError> {
        use DescriptorPoolAllocateError::*;

        let active = self.active.take().unwrap();
        match active.allocate_sets(layout, num_sets) {
            Ok(v) => {
                assert!(self.active.replace(Some(active)).is_none());
                Ok(v)
            }
            Err(FragmentedPool) | Err(OutOfMemory) | Err(OutOfPoolMemory) => {
                let active = self.grow(active)?;
                let result = active.allocate_sets(layout, num_sets);
                assert!(self.active.replace(Some(active)).is_none());
                result
            }
            v @ _ => v,
        }
    }

    fn grow(
        &self,
        active: Box<dyn IDescriptorArena>,
    ) -> Result<Box<dyn IDescriptorArena>, DescriptorPoolAllocateError> {
        use DescriptorPoolAllocateError::*;

        // Immediately move the active arena into the exhausted set before trying to allocate a new
        // pool
        let mut exhausted = self.exhausted.take();
        exhausted.push(active);
        self.exhausted.set(exhausted);

        let last_num_sets = self.last_num_sets.get();
        let new_arena = self.device.create_descriptor_arena(&DescriptorArenaDesc {
            arena_type: DescriptorArenaType::Linear,
            num_sets: last_num_sets * 2,
            name: Some("LinearDescriptorPoolArena"),
        });
        self.last_num_sets.set(last_num_sets * 2);

        match new_arena {
            Ok(v) => Ok(v),
            Err(DescriptorPoolCreateError::OutOfMemory) => Err(OutOfMemory),
            Err(DescriptorPoolCreateError::Platform) => Err(Platform),
        }
    }

    /// This will reset the pool, freeing all descriptors allocated from it and releasing all the
    /// arenas except for the current active arena.
    ///
    /// # Safety
    ///
    /// It is the caller's responsibility to ensure that none of the descriptors that will be freed
    /// by this operation are in use on the host or device.
    pub unsafe fn reset(&self) {
        let active = self.active.take().unwrap();
        active.reset();
        self.active.set(Some(active));

        let mut exhausted = self.exhausted.take();
        exhausted.clear();
        self.exhausted.set(exhausted);
    }
}
