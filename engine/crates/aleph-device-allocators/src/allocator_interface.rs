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

use crate::{DeviceAllocationResult, RawDeviceAllocationResult};

/// Generic interface to an arena allocator that sub-allocates from some chunk of memory that is
/// conventionally upload memory accessible by the GPU device.
///
/// The objects allocated into one of these arenas will _not_ be dropped when the arena is reset.
///
/// # Performance
///
/// Typically the memory that backs any objects allocated into the arena will be from write-combine
/// upload memory. Reading from this will typically be exceptionally slow. The optimal way to use
/// this memory is linear writes with absolutely no reads.
pub trait IUploadAllocator {
    /// Allocate the given number of bytes from the ring buffer.
    #[must_use = "Do not ignore allocation failure"]
    fn allocate(&self, size: usize) -> Option<RawDeviceAllocationResult>;

    /// Allocate the number of bytes from the allocator, accounting for the requested alignment.
    #[must_use = "Do not ignore allocation failure"]
    fn allocate_aligned(&self, size: usize, align: usize) -> Option<RawDeviceAllocationResult>;

    /// Wrapper over [IUploadAllocator::allocate_object] that default-initializes the object.
    #[inline]
    #[must_use = "Do not ignore allocation failure"]
    fn allocate_object_default<T: Sized + Default>(
        &self,
    ) -> Option<DeviceAllocationResult<&mut T>> {
        self.allocate_object(T::default())
    }

    /// Wrapper over [IUploadAllocator::allocate_object] that clones the given resource using
    /// [Copy].
    #[inline]
    #[must_use = "Do not ignore allocation failure"]
    fn allocate_object_copy<T: Sized + Copy>(
        &self,
        src: &T,
    ) -> Option<DeviceAllocationResult<&mut T>> {
        self.allocate_object(*src)
    }

    /// Wrapper over [IUploadAllocator::allocate_object] that clones the given resource using
    /// [Clone].
    #[inline]
    #[must_use = "Do not ignore allocation failure"]
    fn allocate_object_clone<T: Sized + Clone>(
        &self,
        src: &T,
    ) -> Option<DeviceAllocationResult<&mut T>> {
        self.allocate_object(src.clone())
    }

    /// A utility function that will allocate a sufficiently large and aligned block to store a
    /// single `T` object. This will return the object completely uninitialized.
    ///
    /// It is the caller's responsibility to handle correctly initializing the objects.
    /// Alternatively utility methods are available for common cases.
    #[inline]
    #[must_use = "Do not ignore allocation failure"]
    fn allocate_object_uninit<T: Sized>(
        &self,
    ) -> Option<DeviceAllocationResult<&mut std::mem::MaybeUninit<T>>> {
        let v = self.allocate_objects_uninit(1)?;
        Some(DeviceAllocationResult {
            device_offset: v.device_offset,
            result: &mut v.result[0],
            allocated: v.allocated,
        })
    }

    /// Wrapper over [IUploadAllocator::allocate_object_uninit] that initializes an object of
    /// type `T` by placement of the given object.
    #[inline]
    #[must_use = "Do not ignore allocation failure"]
    fn allocate_object<T: Sized>(&self, object: T) -> Option<DeviceAllocationResult<&mut T>> {
        let v = self.allocate_object_uninit()?;
        Some(DeviceAllocationResult {
            device_offset: v.device_offset,
            result: v.result.write(object),
            allocated: v.allocated,
        })
    }

    /// Wrapper over [IUploadAllocator::allocate_objects_iter] that default-initializes `count`
    /// objects.
    #[inline]
    #[must_use = "Do not ignore allocation failure"]
    fn allocate_objects_default<T: Sized + Default>(
        &self,
        count: usize,
    ) -> Option<DeviceAllocationResult<&mut [T]>> {
        self.allocate_objects_iter((0..count).map(|_| T::default()))
    }

    /// Wrapper over [IUploadAllocator::allocate_objects_iter] that copies the objects from the
    /// provided array using [Copy].
    #[inline]
    #[must_use = "Do not ignore allocation failure"]
    fn allocate_objects_copy<T: Sized + Copy>(
        &self,
        src: &[T],
    ) -> Option<DeviceAllocationResult<&mut [T]>> {
        self.allocate_objects_iter(src.iter().copied())
    }

    /// Wrapper over [IUploadAllocator::allocate_objects_iter] that copies the objects from the
    /// provided array using [Clone].
    #[inline]
    #[must_use]
    fn allocate_objects_clone<T: Sized + Clone>(
        &self,
        src: &[T],
    ) -> Option<DeviceAllocationResult<&mut [T]>> {
        self.allocate_objects_iter(src.iter().cloned())
    }

    /// A utility function that will allocate a sufficiently large and aligned block to store a
    /// `count` sized array of `T` objects. This will return the objects completely uninitialized.
    ///
    /// It is the caller's responsibility to handle correctly initializing the objects.
    /// Alternatively utility methods are available for common cases.
    #[inline]
    #[must_use = "Do not ignore allocation failure"]
    fn allocate_objects_uninit<T: Sized>(
        &self,
        count: usize,
    ) -> Option<DeviceAllocationResult<&mut [std::mem::MaybeUninit<T>]>> {
        let size = count * std::mem::size_of::<T>();
        let allocation = self.allocate_aligned(size, std::mem::align_of::<T>())?;

        // Safety: This is safe as the allocator already satisfies all the preconditions.
        let result = unsafe {
            let data = allocation.result.cast::<std::mem::MaybeUninit<T>>();
            std::slice::from_raw_parts_mut(data.as_ptr(), count)
        };

        Some(DeviceAllocationResult {
            device_offset: allocation.device_offset,
            result,
            allocated: allocation.allocated,
        })
    }

    /// Wrapper over [UploadRingBuffer::allocate_objects_uninit] that initializes an array of
    /// objects from the provided [ExactSizeIterator].
    #[inline]
    #[must_use = "Do not ignore allocation failure"]
    fn allocate_objects_iter<T: Sized>(
        &self,
        src: impl ExactSizeIterator<Item = T>,
    ) -> Option<DeviceAllocationResult<&mut [T]>> {
        let DeviceAllocationResult {
            device_offset,
            result,
            allocated,
        } = self.allocate_objects_uninit(src.len())?;

        result.iter_mut().zip(src).for_each(|(v, src)| {
            v.write(src);
        });

        // Convert the array to an initialized array
        let ptr = result.as_mut_ptr();
        let len = result.len();
        let result = unsafe { std::slice::from_raw_parts_mut(ptr.cast::<T>(), len) };

        Some(DeviceAllocationResult {
            device_offset,
            result,
            allocated,
        })
    }
}
