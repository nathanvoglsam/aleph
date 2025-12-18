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

use aleph_alloc::BVec;
use aleph_alloc::instrumentation::system;
use aleph_any::AnyArc;
use smallbox::SmallBox;
use smallbox::space::S8;

use crate::internal::MgSystem;
use crate::internal::buffer::BufferObjectStore;
use crate::internal::renderer::deferred_deletion_manager::DeletionBundle;
use crate::internal::renderer::last_use_tracker::{
    LastBufferUse, LastTextureUse, LastUseTracker, UsageType,
};
use crate::internal::texture::TextureObjectStore;
use crate::resource::buffer::BufferHandle;
use crate::resource::texture::TextureHandle;
use crate::resource::texture::simple::SimpleTextureLayout;
use crate::resource_loader::mip_upload::MipUploadDesc;
use crate::resource_loader::upload_buffer::IUploadBuffer;

pub struct ImmediateUploadQueue {
    pub device: AnyArc<dyn rhi::IDevice>,
    pub textures: BVec<ImmediateTextureUpload, MgSystem>,
    pub buffers: BVec<ImmediateBufferUpload, MgSystem>,
}

impl ImmediateUploadQueue {
    pub const fn new(device: AnyArc<dyn rhi::IDevice>) -> Self {
        Self {
            device,
            textures: BVec::new_in(system()),
            buffers: BVec::new_in(system()),
        }
    }

    pub fn record_upload_commands(
        &mut self,
        last_uses: &mut LastUseTracker,
        deletion_bundle: &mut DeletionBundle,
        cmd: &mut dyn rhi::IGeneralEncoder,
        bpool: &BufferObjectStore,
        tpool: &TextureObjectStore,
    ) {
        // Do nothing if there are no queued upload commands
        if self.buffers.is_empty() && self.textures.is_empty() {
            return;
        }

        // This loop handles a few things:
        //
        // First we cache a list of optional rhi::TextureHandles looked up from the object pool. We
        // need these when emitting the copy commands. We cache them now so we don't have to hit
        // the pool again in the second loop.
        //
        // At the same time we also emit the texture barriers we need to issue in order to correctly
        // initialize the textures that are being copied into. This struct is only responsible for
        // uploading into _new_ resources so we don't need a before scope.
        let mut fetched_rhi_textures = Vec::new();
        let mut before_texture_barriers = Vec::new();
        for texture_upload in self.textures.iter() {
            // Get the texture handle (if there is one) from the texture object pool
            let handle = tpool
                .pool
                .get_ref(texture_upload.target)
                .map(|tgt| tgt.object.as_ref())
                .flatten();
            fetched_rhi_textures.push(handle);

            // If we had a handle (we always should, but we're fault-tolerant for now) then we
            // should add barriers for it.
            if let Some(handle) = handle {
                let subresource_range =
                    rhi::TextureSubResourceSet::all(self.device.get_texture_desc(handle));

                // NONE -> COPY barrier
                before_texture_barriers.push(rhi::TextureBarrier {
                    texture: Some(handle),
                    subresource_range: subresource_range.clone(),
                    before_sync: rhi::BarrierSync::NONE,
                    after_sync: rhi::BarrierSync::COPY,
                    before_access: rhi::BarrierAccess::NONE,
                    after_access: rhi::BarrierAccess::COPY_WRITE,
                    before_layout: rhi::ImageLayout::Undefined,
                    after_layout: rhi::ImageLayout::CopyDst,
                    queue_transition: None,
                });
            }
        }

        // Issue our 'acquire' barrier that will prepare all our new resources for being written
        // into. No before sync is needed as we're guaranteed to be the first use of the resources.
        unsafe {
            // NONE -> COPY barrier
            let memory_barriers = [rhi::GlobalBarrier {
                before_sync: rhi::BarrierSync::NONE,
                after_sync: rhi::BarrierSync::COPY,
                before_access: rhi::BarrierAccess::NONE,
                after_access: rhi::BarrierAccess::COPY_WRITE,
            }];

            // Only issue global barriers if buffer uploads were queued
            let actual_memory_barriers: &[rhi::GlobalBarrier] = if !self.buffers.is_empty() {
                &memory_barriers
            } else {
                &[]
            };

            cmd.resource_barrier(actual_memory_barriers, &[], &before_texture_barriers);
        }

        // This is where we perform our actual upload copies. We will copy as much data from the
        // upload source as will fit into the destination buffer, then push the src buffer into
        // the deletion bundle. As a result it remains alive long enough for the command to execute
        // on the GPU timeline.
        for buffer_upload in self.buffers.drain(..) {
            let src = buffer_upload.data.handle();
            let dst = match bpool.pool.get_ref(buffer_upload.target) {
                None => continue,
                Some(v) => v,
            };
            match dst.object.as_ref() {
                None => continue,
                Some(dst) => unsafe {
                    let upload_size = buffer_upload.data.bytes().len() as u64;

                    // Get the size as a smaller of the buffer's actual size and the number of
                    // bytes provided in the upload buffer. This prevents out of bounds writes.
                    let size = self.device.get_buffer_desc(dst).size;
                    let size = u64::min(size, upload_size);

                    if size < upload_size {
                        log::warn!(
                            "Provided {} bytes to upload for buffer of size {} bytes.",
                            upload_size,
                            size
                        );
                    }

                    let region = rhi::BufferCopyRegion {
                        src_offset: buffer_upload.data.device_offset(),
                        dst_offset: 0,
                        size,
                    };
                    cmd.copy_buffer_regions(src, dst, &[region]);

                    last_uses.buffers.insert(
                        buffer_upload.target,
                        LastBufferUse::new_for_usage(
                            rhi::ResourceUsageFlags::COPY_DEST,
                            UsageType::Write,
                        ),
                    );
                },
            }

            // Extend the lifetime of the source buffer by placing it into the deletion bundle.
            deletion_bundle.buffers.push(src.clone());
        }

        // This is where we perform our actual upload copies... for textures.
        let mut regions = Vec::new();
        for (i, texture_upload) in self.textures.drain(..).enumerate() {
            let src = texture_upload.mips.buffer.handle();
            let dst = fetched_rhi_textures[i];
            match dst.as_ref() {
                None => continue,
                Some(dst) => unsafe {
                    for level in texture_upload.mips.data.level_range() {
                        regions.push(texture_upload.mips.get_copy_region(
                            &texture_upload.desc,
                            level,
                            rhi::TextureCopyAspect::Color,
                        ));
                    }

                    cmd.copy_buffer_to_texture(src, dst, &regions);
                    regions.clear();

                    last_uses.textures.insert(
                        texture_upload.target,
                        LastTextureUse::new_for_usage(
                            rhi::ResourceUsageFlags::COPY_DEST,
                            UsageType::Write,
                            texture_upload.desc.format,
                        ),
                    );
                },
            }

            // Extend the lifetime of the source buffer by placing it into the deletion bundle.
            deletion_bundle.buffers.push(src.clone());
        }
    }

    pub fn clean_up(&mut self) {
        self.buffers.clear();
        self.textures.clear();
    }
}

pub struct ImmediateTextureUpload {
    pub desc: SimpleTextureLayout,
    pub target: TextureHandle,
    pub mips: MipUploadDesc,
}

pub struct ImmediateBufferUpload {
    pub target: BufferHandle,
    pub data: SmallBox<dyn IUploadBuffer, S8>,
}
