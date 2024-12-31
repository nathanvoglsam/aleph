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

use std::mem::ManuallyDrop;

use aleph_any::AnyArc;
use aleph_device_allocators::{AllocatorPoolItem, Grave, LinearDescriptorPoolFactory};
use aleph_rhi_api::*;
use smallbox::space::S8;
use smallbox::SmallBox;

use crate::IUploadBuffer;

/// Deletion pool that will hold textures, buffers and upload data handles that must have their
/// lifetime extended to meet GPU timeline requirements.
///
/// # Leaking
///
/// If an instance of [`DeletionPool`] is dropped while still containing items
/// inside any of its internal pools, those items will be leaked.
#[derive(Default)]
pub struct DeletionPool {
    textures: Vec<ManuallyDrop<AnyArc<dyn ITexture>>>,
    buffers: Vec<ManuallyDrop<AnyArc<dyn IBuffer>>>,
    upload: Vec<ManuallyDrop<SmallBox<dyn IUploadBuffer, S8>>>,
    descriptor_pools: Vec<Grave<AllocatorPoolItem<LinearDescriptorPoolFactory>>>,
}

impl DeletionPool {
    pub const fn new() -> Self {
        Self {
            textures: Vec::new(),
            buffers: Vec::new(),
            upload: Vec::new(),
            descriptor_pools: Vec::new(),
        }
    }

    #[inline]
    pub fn push_texture(&mut self, texture: AnyArc<dyn ITexture>) {
        self.textures.push(ManuallyDrop::new(texture))
    }

    #[inline]
    pub fn push_buffer(&mut self, buffer: AnyArc<dyn IBuffer>) {
        self.buffers.push(ManuallyDrop::new(buffer))
    }

    #[inline]
    pub fn push_upload(&mut self, upload: SmallBox<dyn IUploadBuffer, S8>) {
        self.upload.push(ManuallyDrop::new(upload))
    }

    #[inline]
    pub fn push_descriptor_pool(
        &mut self,
        pool: Grave<AllocatorPoolItem<LinearDescriptorPoolFactory>>,
    ) {
        self.descriptor_pools.push(pool)
    }

    /// This function will purge the internal pools and _will_ drop the resources being held inside.
    ///
    /// # Safety
    ///
    /// This is unsafe as the entire purpose of this tool is to extend the lifetime of resources
    /// to satisfy the GPU timeline. It is not possible to do this safely in the general case. This
    /// function forms part of the safe abstraction you use to allow someone _else_ to do this
    /// safely at a higher level.
    ///
    /// To that end. This is unsafe as we have no way of guaranteeing that it is truly safe to drop
    /// any of the resources in this pool without system level coordination that Rust can't prove
    /// is happening.
    ///
    /// It is the caller's responsibility to ensure that all resources in this pool are no longer
    /// being used on the GPU timeline.
    #[inline]
    pub unsafe fn purge(&mut self, mode: DeletionMode) {
        // Drain the pools and explicitly drop the contained elements.
        self.purge_textures(mode);
        self.purge_buffers(mode);
        self.purge_uploads(mode);
        self.purge_descriptor_pools(mode);
    }

    /// Alternate version of [`DeletionPool::purge`] that only purges the
    /// texture objects.
    ///
    /// # Safety
    ///
    /// See `purge` for more info, the constraints are the same.
    #[inline]
    pub unsafe fn purge_textures(&mut self, mode: DeletionMode) {
        // Drain the pools and explicitly drop the contained elements.
        self.textures.drain(..).for_each(|mut v| {
            if v.strong_count() == 1 && mode == DeletionMode::Deferred {
                rayon::spawn(move || drop_texture_on_pool(v));
            } else {
                ManuallyDrop::drop(&mut v);
            }
        });
    }

    /// Alternate version of [`DeletionPool::purge`] that only purges the
    /// buffer objects.
    ///
    /// # Safety
    ///
    /// See `purge` for more info, the constraints are the same.
    #[inline]
    pub unsafe fn purge_buffers(&mut self, mode: DeletionMode) {
        // Drain the pools and explicitly drop the contained elements.
        self.buffers.drain(..).for_each(|mut v| {
            if v.strong_count() == 1 && mode == DeletionMode::Deferred {
                rayon::spawn(move || drop_buffer_on_pool(v));
            } else {
                ManuallyDrop::drop(&mut v);
            }
        });
    }

    /// Alternate version of [`DeletionPool::purge`] that only purges the
    /// upload objects.
    ///
    /// # Safety
    ///
    /// See `purge` for more info, the constraints are the same.
    #[inline]
    pub unsafe fn purge_uploads(&mut self, mode: DeletionMode) {
        // Drain the pools and explicitly drop the contained elements.
        self.upload.drain(..).for_each(|mut v| {
            if v.buffer().strong_count() == 1 && mode == DeletionMode::Deferred {
                rayon::spawn(move || drop_upload_on_pool(v));
            } else {
                ManuallyDrop::drop(&mut v);
            }
        });
    }

    /// Alternate version of [`DeletionPool::purge`] that only purges the
    /// descriptor pools.
    ///
    /// # Safety
    ///
    /// See `purge` for more info, the constraints are the same.
    #[inline]
    pub unsafe fn purge_descriptor_pools(&mut self, _mode: DeletionMode) {
        // These simply get sent back to the pool they came from so we don't need to do any threaded
        // destruction here to avoid tanking the main thread.
        self.descriptor_pools.clear()
    }
}

impl Drop for DeletionPool {
    fn drop(&mut self) {
        if !self.textures.is_empty() {
            let len = self.textures.len();
            log::warn!("Deletion Pool dropped with {len} textures! This is leaking memory!");
        }
        if !self.buffers.is_empty() {
            let len = self.buffers.len();
            log::warn!("Deletion Pool dropped with {len} buffers! This is leaking memory!");
        }
    }
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub enum DeletionMode {
    /// All resources should be destroyed immediately in the purge functions.
    Inline,

    /// All resources will be sent into the threadpool to be dropped and deallocated.
    Deferred,
}

impl Default for DeletionMode {
    #[inline]
    fn default() -> Self {
        Self::Deferred
    }
}

#[aleph_profile::function]
unsafe fn drop_texture_on_pool(mut v: ManuallyDrop<AnyArc<dyn ITexture>>) {
    ManuallyDrop::drop(&mut v)
}

#[aleph_profile::function]
unsafe fn drop_buffer_on_pool(mut v: ManuallyDrop<AnyArc<dyn IBuffer>>) {
    ManuallyDrop::drop(&mut v)
}

#[aleph_profile::function]
unsafe fn drop_upload_on_pool(mut v: ManuallyDrop<SmallBox<dyn IUploadBuffer, S8>>) {
    ManuallyDrop::drop(&mut v)
}
