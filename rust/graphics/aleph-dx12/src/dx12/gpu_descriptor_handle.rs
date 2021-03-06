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

use crate::windows_raw::win32::direct3d12::D3D12_GPU_DESCRIPTOR_HANDLE;
use std::convert::TryFrom;
use std::num::NonZeroU64;

#[repr(transparent)]
#[derive(Copy, Clone, PartialOrd, PartialEq, Ord, Eq, Debug, Hash)]
pub struct GPUDescriptorHandle(pub(crate) NonZeroU64);

impl GPUDescriptorHandle {
    pub fn offset(self, offset: i64) -> Self {
        let ptr = self.0.get() as i64;
        let ptr = ptr + offset;
        Self(NonZeroU64::new(ptr as u64).unwrap())
    }

    pub fn add(self, offset: u64) -> Self {
        Self(NonZeroU64::new(self.0.get() + offset).unwrap())
    }

    pub fn offset_increments(self, offset: i64, increment: u64) -> Self {
        let ptr = self.0.get() as i64;
        let ptr = ptr + (offset * increment as i64);
        Self(NonZeroU64::new(ptr as u64).unwrap())
    }

    pub fn add_increments(self, offset: u64, increment: u64) -> Self {
        Self(NonZeroU64::new(self.0.get() + (offset * increment)).unwrap())
    }

    pub fn get_inner(&self) -> NonZeroU64 {
        self.0
    }
}

impl Into<D3D12_GPU_DESCRIPTOR_HANDLE> for GPUDescriptorHandle {
    fn into(self) -> D3D12_GPU_DESCRIPTOR_HANDLE {
        D3D12_GPU_DESCRIPTOR_HANDLE { ptr: self.0.get() }
    }
}

impl TryFrom<D3D12_GPU_DESCRIPTOR_HANDLE> for GPUDescriptorHandle {
    type Error = ();

    fn try_from(value: D3D12_GPU_DESCRIPTOR_HANDLE) -> Result<Self, Self::Error> {
        let value = NonZeroU64::new(value.ptr).ok_or(())?;
        Ok(Self(value))
    }
}
