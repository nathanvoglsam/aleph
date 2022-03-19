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

use std::convert::TryFrom;
use std::num::NonZeroUsize;
use windows::Win32::Graphics::Direct3D12::D3D12_CPU_DESCRIPTOR_HANDLE;

#[repr(transparent)]
#[derive(Copy, Clone, PartialOrd, PartialEq, Ord, Eq, Debug, Hash)]
pub struct CPUDescriptorHandle(NonZeroUsize);

#[allow(clippy::should_implement_trait)]
impl CPUDescriptorHandle {
    #[inline]
    pub fn offset(self, offset: isize) -> Self {
        let ptr = self.0.get() as isize;
        let ptr = ptr + offset;
        Self(NonZeroUsize::new(ptr as usize).unwrap())
    }

    #[inline]
    pub fn add(self, offset: usize) -> Self {
        Self(NonZeroUsize::new(self.0.get() + offset).unwrap())
    }

    #[inline]
    pub fn offset_increments(self, offset: isize, increment: usize) -> Self {
        let ptr = self.0.get() as isize;
        let ptr = ptr + (offset * increment as isize);
        Self(NonZeroUsize::new(ptr as usize).unwrap())
    }

    #[inline]
    pub fn add_increments(self, offset: usize, increment: usize) -> Self {
        Self(NonZeroUsize::new(self.0.get() + (offset * increment)).unwrap())
    }

    #[inline]
    pub fn get_inner(&self) -> NonZeroUsize {
        self.0
    }
}

impl From<CPUDescriptorHandle> for D3D12_CPU_DESCRIPTOR_HANDLE {
    #[inline]
    fn from(v: CPUDescriptorHandle) -> Self {
        D3D12_CPU_DESCRIPTOR_HANDLE { ptr: v.0.get() }
    }
}

impl TryFrom<D3D12_CPU_DESCRIPTOR_HANDLE> for CPUDescriptorHandle {
    type Error = ();

    #[inline]
    fn try_from(value: D3D12_CPU_DESCRIPTOR_HANDLE) -> Result<Self, Self::Error> {
        let value = NonZeroUsize::new(value.ptr).ok_or(())?;
        Ok(Self(value))
    }
}

impl From<NonZeroUsize> for CPUDescriptorHandle {
    #[inline]
    fn from(v: NonZeroUsize) -> Self {
        Self(v)
    }
}

impl TryFrom<usize> for CPUDescriptorHandle {
    type Error = ();

    #[inline]
    fn try_from(value: usize) -> Result<Self, Self::Error> {
        let value = NonZeroUsize::new(value).ok_or(())?;
        Ok(Self(value))
    }
}
