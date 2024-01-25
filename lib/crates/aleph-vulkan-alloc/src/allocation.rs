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

use std::fmt::{Debug, Formatter};
use std::marker::PhantomData;
use std::ops::Deref;

use ash::vk;

use crate::raw::{AllocationCreateFlags, AllocationCreateInfo, AllocationH, MemoryUsage};
use crate::vma;

#[derive(Copy, Clone)]
#[repr(transparent)]
pub struct Allocation {
    pub(crate) allocation: AllocationH,
}

unsafe impl Send for Allocation {}
unsafe impl Sync for Allocation {}

#[derive(Default)]
pub struct AllocationCreateInfoBuilder<'a> {
    info: AllocationCreateInfo,
    phantom: PhantomData<&'a vma::Pool>,
}

impl<'a> AllocationCreateInfoBuilder<'a> {
    pub const fn new() -> Self {
        let info = AllocationCreateInfo {
            flags: AllocationCreateFlags::empty(),
            usage: MemoryUsage::Unknown,
            required_flags: vk::MemoryPropertyFlags::empty(),
            preferred_flags: vk::MemoryPropertyFlags::empty(),
            memory_type_bits: 0,
            pool: None,
            p_user_data: None,
            priority: 0.0,
        };
        AllocationCreateInfoBuilder {
            info,
            phantom: PhantomData,
        }
    }

    pub const fn flags(mut self, flags: AllocationCreateFlags) -> Self {
        self.info.flags = flags;
        self
    }

    pub const fn required_flags(mut self, flags: vk::MemoryPropertyFlags) -> Self {
        self.info.required_flags = flags;
        self
    }

    pub const fn preferred_flags(mut self, flags: vk::MemoryPropertyFlags) -> Self {
        self.info.preferred_flags = flags;
        self
    }

    pub const fn usage(mut self, usage: MemoryUsage) -> Self {
        self.info.usage = usage;
        self
    }

    pub const fn pool(mut self, pool: &'a vma::Pool) -> Self {
        self.info.pool = Some(pool.get_raw());
        self
    }

    pub const fn build(self) -> AllocationCreateInfo {
        self.info
    }
}

impl<'a> Deref for AllocationCreateInfoBuilder<'a> {
    type Target = AllocationCreateInfo;

    fn deref(&self) -> &Self::Target {
        &self.info
    }
}

impl<'a> Debug for AllocationCreateInfoBuilder<'a> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        self.info.fmt(f)
    }
}
