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

use aleph_rhi_api::*;
use interfaces::any::AnyArc;

use crate::render::BufferUploadSource;

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
}

impl DeletionPool {
    pub const fn new() -> Self {
        Self {
            textures: Vec::new(),
            buffers: Vec::new(),
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
    pub fn push_upload(&mut self, upload: impl Into<BufferUploadSource>) {
        let upload = upload.into().into_buffer();
        self.buffers.push(ManuallyDrop::new(upload))
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
    pub unsafe fn purge(&mut self) {
        // Drain the pools and explicitly drop the contained elements.
        self.purge_textures();
        self.purge_buffers();
    }

    /// Alternate version of [`DeletionPool::purge`] that only purges the
    /// texture objects.
    ///
    /// # Safety
    ///
    /// See `purge` for more info, the constraints are the same.
    #[inline]
    pub unsafe fn purge_textures(&mut self) {
        // Drain the pools and explicitly drop the contained elements.
        self.textures
            .drain(..)
            .for_each(|mut v| ManuallyDrop::drop(&mut v));
    }

    /// Alternate version of [`DeletionPool::purge`] that only purges the
    /// buffer objects.
    ///
    /// # Safety
    ///
    /// See `purge` for more info, the constraints are the same.
    #[inline]
    pub unsafe fn purge_buffers(&mut self) {
        // Drain the pools and explicitly drop the contained elements.
        self.buffers
            .drain(..)
            .for_each(|mut v| ManuallyDrop::drop(&mut v));
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
