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

#[cfg(target_os = "windows")]
extern crate aleph_dx12 as dx12;

#[cfg(target_os = "windows")]
extern crate aleph_dx12_alloc_raw as alloc_raw;

#[cfg(target_os = "windows")]
extern crate aleph_windows_raw as windows_raw;

#[cfg(target_os = "windows")]
mod allocation;

#[cfg(target_os = "windows")]
mod allocation_desc;

#[cfg(target_os = "windows")]
mod allocator;

#[cfg(target_os = "windows")]
mod allocator_desc;

#[cfg(target_os = "windows")]
mod pool;

#[cfg(target_os = "windows")]
mod pool_desc;

#[cfg(target_os = "windows")]
mod exports {
    pub use super::allocation::Allocation;
    pub use super::allocation_desc::AllocationDesc;
    pub use super::allocation_desc::AllocationDescBuilder;
    pub use super::allocator::Allocator;
    pub use super::allocator_desc::AllocatorDesc;
    pub use super::allocator_desc::AllocatorDescBuilder;
    pub use super::pool::Pool;
    pub use super::pool_desc::PoolDesc;
    pub use super::pool_desc::PoolDescBuilder;

    pub type AllocationFlags = alloc_raw::D3D12MA_ALLOCATION_FLAGS;
    pub type AllocatorFlags = alloc_raw::D3D12MA_ALLOCATOR_FLAGS;
}

#[cfg(not(target_os = "windows"))]
mod exports {}

pub use exports::*;
