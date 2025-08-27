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

use aleph_object_system::ArcObject;

use crate::*;

#[derive(Clone)]
pub struct BufferHandle {
    inner: ArcObject,
}

impl BufferHandle {
    pub const unsafe fn new(inner: ArcObject) -> Self {
        Self { inner }
    }

    ///
    /// Gets the number of strong ([`BufferHandle`]) pointers to this allocation.
    ///
    /// # Safety
    ///
    /// This method by itself is safe, but using it correctly requires extra care.
    /// Another thread can change the strong count at any time,
    /// including potentially between calling this method and acting on the result.
    ///
    /// # Info
    ///
    /// This is just a wrapper around [`std::sync::Arc::strong_count`]
    ///
    #[inline]
    #[must_use]
    pub fn strong_count(&self) -> usize {
        self.inner.strong_count()
    }

    /// Unwrap the [`BufferHandle`] and get the inner [`ArcObject`]
    pub fn into_inner(self) -> ArcObject {
        self.inner
    }

    /// Get the inner [`ArcObject`]
    pub const fn get(&self) -> &ArcObject {
        &self.inner
    }
}

/// Description object used for creating a new buffer.
#[derive(Clone, Hash, PartialEq, Eq, Debug, Default)]
pub struct BufferDesc<'a> {
    /// The size of the buffer in bytes
    pub size: u64,

    /// What kind of CPU access is allowed.
    /// - None -> device local
    /// - Read -> read back
    /// - Write -> upload
    pub cpu_access: CpuAccessMode,

    /// Specifies in what ways the buffer can be used
    pub usage: ResourceUsageFlags,

    /// The name of the object
    pub name: Option<&'a str>,
}

impl<'a> BufferDesc<'a> {
    /// A utility function that strips the debug name from the description so we can get a static
    /// lifetime on the desc
    pub const fn strip_name(self) -> BufferDesc<'static> {
        BufferDesc::<'static> {
            size: self.size,
            cpu_access: self.cpu_access,
            usage: self.usage,
            name: None,
        }
    }

    /// A utility function that replaces any existing name with the given name, yielding a new desc
    /// identical to the source desc differeing only in name.
    pub const fn with_name<'b>(self, name: &'b str) -> BufferDesc<'b> {
        BufferDesc::<'b> {
            size: self.size,
            cpu_access: self.cpu_access,
            usage: self.usage,
            name: Some(name),
        }
    }

    pub const fn new(size: u64) -> Self {
        Self {
            size,
            cpu_access: CpuAccessMode::None,
            usage: ResourceUsageFlags::NONE,
            name: None,
        }
    }

    pub const fn with_cpu_access(mut self, cpu_access: CpuAccessMode) -> Self {
        self.cpu_access = cpu_access;
        self
    }

    pub const fn cpu_read(self) -> Self {
        self.with_cpu_access(CpuAccessMode::Read)
    }

    pub const fn cpu_write(self) -> Self {
        self.with_cpu_access(CpuAccessMode::Write)
    }

    pub const fn with_usage(mut self, usage: ResourceUsageFlags) -> Self {
        self.usage = usage;
        self
    }
}

/// Enumeration of all CPU access modes for resources
#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Debug)]
pub enum CpuAccessMode {
    /// Resource can not be accessed by the CPU at all (device local)
    None,

    /// Resource can be read by the CPU (read back)
    Read,

    /// Resource can be written by the CPU (upload)
    Write,
}

impl Default for CpuAccessMode {
    #[inline(always)]
    fn default() -> Self {
        Self::None
    }
}
