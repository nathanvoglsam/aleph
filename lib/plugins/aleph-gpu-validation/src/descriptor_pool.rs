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

use crate::descriptor_set_layout::ValidationDescriptorSetLayout;
use crate::device::ValidationDevice;
use crate::internal::descriptor_set::DescriptorSet;
use interfaces::any::AnyArc;
use interfaces::gpu::*;
use std::ptr::NonNull;

pub struct ValidationDescriptorPool {
    pub(crate) _device: AnyArc<ValidationDevice>,
    pub(crate) _layout: AnyArc<ValidationDescriptorSetLayout>,
    pub(crate) inner: Box<dyn IDescriptorPool>,
    pub(crate) set_objects: Vec<DescriptorSet>,
    pub(crate) free_list: Vec<DescriptorSetHandle>,
}

crate::validation_declare_interfaces!(ValidationDescriptorPool, [IDescriptorPool]);

impl ValidationDescriptorPool {
    fn validate_set_handle(&self, set: &DescriptorSetHandle) {
        // Validate that a DescriptorSetHandle contains a correctly aligned pointer. This may help
        // catch when someone is passing in bad handles
        let align = core::mem::align_of::<DescriptorSet>();
        let set = set.clone();
        let set = unsafe { core::mem::transmute::<_, NonNull<DescriptorSet>>(set) };

        // This should also never happen in practice, but can help flag when people are doing
        // naughty bit casts and passing bad pointers in.
        if !align.is_power_of_two() {
            panic!("is_aligned_to: align is not a power-of-two");
        }
        debug_assert!(
            (set.as_ptr() as usize) & align - 1 == 0,
            "DescriptorSetHandle contains badly-aligned pointer"
        );

        // If the pool is empty it's impossible for any handles to be from this particular pool.
        // This should never happen as we never allow empty descriptor pools.
        debug_assert!(
            self.set_objects.is_empty(),
            "The DescriptorSet pool is empty, no handle can be valid"
        );

        let sets_base = self.set_objects.as_ptr();
        let sets_end = self
            .set_objects
            .as_ptr()
            .wrapping_add(self.set_objects.len());

        // This should never happen, but we check for completeness sake.
        debug_assert!(
            sets_base < sets_end,
            "The DescriptorSet pool has overflowed the address space"
        );

        // Checks if the given descriptor set was allocated by this pool by checking if the pointer
        // comes from inside the set_objects array bounds.
        let set_ptr = set.as_ptr() as *const DescriptorSet;
        let set_oob = set_ptr < sets_base || set_ptr > sets_end;
        debug_assert!(
            !set_oob,
            "The DescriptorSetHandle points outside of the pool, this handle is from another pool"
        );
    }
}

impl IDescriptorPool for ValidationDescriptorPool {
    fn allocate_set(&mut self) -> Result<DescriptorSetHandle, DescriptorPoolAllocateError> {
        self.inner.allocate_set()
    }

    fn allocate_sets(
        &mut self,
        num_sets: usize,
    ) -> Result<Vec<DescriptorSetHandle>, DescriptorPoolAllocateError> {
        self.inner.allocate_sets(num_sets)
    }

    unsafe fn free(&mut self, sets: &[DescriptorSetHandle]) {
        for set in sets {
            self.validate_set_handle(set);
        }
        self.inner.free(sets)
    }

    unsafe fn reset(&mut self) {
        self.inner.reset()
    }
}

impl INamedObject for ValidationDescriptorPool {
    fn set_name(&self, name: &str) {
        self.inner.set_name(name)
    }
}
