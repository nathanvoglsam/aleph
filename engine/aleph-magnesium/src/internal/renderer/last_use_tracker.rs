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

use aleph_alloc::HashMap;
use aleph_alloc::instrumentation::system;
use aleph_identity_hasher::BuildIdentityHasher;

use crate::internal::MgSystem;
use crate::internal::buffer::BufferObjectStore;
use crate::internal::texture::TextureObjectStore;
use crate::resource::buffer::BufferHandle;
use crate::resource::texture::TextureHandle;

/// Hash table intended for tracking the if and how of GPU resource's last usages.
///
/// This is intended to be used for GPU work outside a frame graph where the usage is highly
/// dynamic.
pub struct LastUseTracker {
    pub buffers: HashMap<BufferHandle, LastBufferUse, BuildIdentityHasher, MgSystem>,
    pub textures: HashMap<TextureHandle, LastTextureUse, BuildIdentityHasher, MgSystem>,
}

impl LastUseTracker {
    /// Constructs a new, empty instance
    pub fn new() -> Self {
        Self {
            buffers: HashMap::with_hasher_in(Default::default(), system()),
            textures: HashMap::with_hasher_in(Default::default(), system()),
        }
    }

    /// Issues barriers on the given command buffer to flush all outstanding un-synchronized
    /// accesses that are queued in this usage tracker.
    ///
    /// The expected use case here is to "release" the previous accesses to subsequent commands. We
    /// expect these subsequent commands to be a frame graph execution.
    ///
    /// Any non-frame-graph users are expected to track their accesses to different resoruces via
    /// a last-use-tracker.
    pub unsafe fn flush_for_read_in_frame_graph(
        &mut self,
        cmd: &mut rhi::CommandEncoder,
        bpool: &BufferObjectStore,
        tpool: &TextureObjectStore,
    ) {
        // Early exit if there are no previous usages to flush
        if self.buffers.is_empty() && self.textures.is_empty() {
            return;
        }

        let has_buffer_barriers = !self.buffers.is_empty();

        // Flush all buffer usages and accumulate the before sync/access flags into a single
        // global barrier.
        //
        // TODO: much like below, we assume _all_ usage as the after scope. this should
        //       change.
        let usage = rhi::ResourceUsageFlags::BUFFER_USAGE_MASK
            .difference(rhi::ResourceUsageFlags::RAYTRACING_USAGE_MASK);
        let after = LastBufferUse::new_for_usage(usage, UsageType::Read);
        let mut global_barrier = rhi::GlobalBarrier {
            before_sync: rhi::BarrierSync::NONE,
            after_sync: after.sync,
            before_access: rhi::BarrierAccess::NONE,
            after_access: after.access,
        };
        let mut buffer_barriers = Vec::new();
        for (handle, last_use) in self.buffers.drain() {
            if last_use.queue_transition.is_some() {
                // Skip if the handle is invalid
                let object = match bpool.pool.get_ref(handle) {
                    None => continue,
                    Some(v) => v,
                };

                // Skip if the texture has no rhi resource
                let rhi_handle = match object.object.as_ref() {
                    None => continue,
                    Some(v) => v,
                };

                // Queue ownership transitions must be issued as buffer barriers, rather than as a
                // global barrier
                buffer_barriers.push(rhi::BufferBarrier {
                    buffer: Some(rhi_handle),
                    offset: 0,
                    size: u64::MAX,
                    before_sync: rhi::BarrierSync::NONE,
                    after_sync: after.sync,
                    before_access: rhi::BarrierAccess::NONE,
                    after_access: after.access,
                    queue_transition: last_use.queue_transition,
                });
            } else {
                global_barrier.before_sync |= last_use.sync;
                global_barrier.before_access |= last_use.access;
            }
        }

        // Flush all texture usages and accumulate them into a list of texture barriers. We can't
        // collapse them into a single barrier because of layout transitions.
        let mut texture_barriers = Vec::new();
        for (handle, last_use) in self.textures.drain() {
            // Skip if the handle is invalid
            let object = match tpool.pool.get_ref(handle) {
                None => continue,
                Some(v) => v,
            };

            // Skip if the texture has no rhi resource
            let rhi_handle = match object.object.as_ref() {
                None => continue,
                Some(v) => v,
            };

            // TODO: right now we hardcode 'shader resource' as the only valid usage within the
            //       frame graph. we likely need to be smarter.
            //
            // Derive the after scope from an assumed shader resource usage.
            let after = LastTextureUse::new_for_usage(
                rhi::ResourceUsageFlags::SHADER_RESOURCE,
                UsageType::Read,
                object.format,
            );

            texture_barriers.push(rhi::TextureBarrier {
                texture: Some(rhi_handle),
                subresource_range: object.subresource_all.clone(),
                before_sync: last_use.sync,
                after_sync: after.sync,
                before_access: last_use.access,
                after_access: after.access,
                before_layout: last_use.layout,
                after_layout: after.layout,
                queue_transition: last_use.queue_transition,
            });
        }

        // Skip sending the empty global barrier if no buffers were changed
        let memory_barriers = if has_buffer_barriers {
            std::slice::from_ref(&global_barrier)
        } else {
            &[]
        };

        unsafe {
            cmd.resource_barrier(memory_barriers, &buffer_barriers, &texture_barriers);
        }
    }
}

/// Layout used for tracking the last usage of a buffer resource.
///
/// Contains a sync/access mask. Clients of a [`LastUseTracker`] are expected to accumulate their
/// usages and flush barriers when they need to into an instance of this type.
pub struct LastBufferUse {
    pub sync: rhi::BarrierSync,
    pub access: rhi::BarrierAccess,
    pub queue_transition: Option<rhi::QueueTransition>,
}

impl LastBufferUse {
    /// Constructs a new instance by deducing from a usage flag ([`rhi::ResourceUsageFlags`]) the
    /// full sync scope needed for accessing some buffer as described.
    ///
    /// The 'write' flag declares if the access includes writes, and is needed to correctly resolve
    /// the access mask for some usages.
    ///
    /// The resulting masks are only as accurate as the information you give this function. Take
    /// care to ensure the access
    pub const fn new_for_usage(usage: rhi::ResourceUsageFlags, usage_type: UsageType) -> Self {
        let usage = usage.intersection(rhi::ResourceUsageFlags::BUFFER_USAGE_MASK);
        let sync = usage.default_barrier_sync(usage_type.is_read_only(), rhi::Format::R8Unorm);
        let access = if usage_type.is_read_only() {
            usage.barrier_access_for_read(rhi::Format::R8Unorm)
        } else {
            usage.barrier_access_for_write(rhi::Format::R8Unorm)
        };
        Self {
            sync,
            access,
            queue_transition: None,
        }
    }
}

/// Layout used for tracking the last usage of a texture resource.
///
/// Contains a sync/access mask. Clients of a [`LastUseTracker`] are expected to accumlate their
/// usages and flush barriers when they need to into an instance of this type. Textures must take
/// special care because we need to manage layout transitions too.
pub struct LastTextureUse {
    pub sync: rhi::BarrierSync,
    pub access: rhi::BarrierAccess,
    pub layout: rhi::ImageLayout,
    pub queue_transition: Option<rhi::QueueTransition>,
}

impl LastTextureUse {
    /// Constructs a new instance by deducing from a usage flag ([`rhi::ResourceUsageFlags`]) the
    /// full sync scope needed for accessing some texture as described.
    ///
    /// The 'write' flag declares if the access includes writes, and is needed to correctly resolve
    /// the access mask for some usages.
    ///
    /// The 'format' is also required to deduce the correct access and image layout for some usages
    /// such as depth/stencil textures.
    pub const fn new_for_usage(
        usage: rhi::ResourceUsageFlags,
        usage_type: UsageType,
        format: rhi::Format,
    ) -> Self {
        let usage = usage.intersection(rhi::ResourceUsageFlags::TEXTURE_USAGE_MASK);
        let sync = usage.default_barrier_sync(usage_type.is_read_only(), format);
        let access = if usage_type.is_read_only() {
            usage.barrier_access_for_read(format)
        } else {
            usage.barrier_access_for_write(format)
        };
        let layout = usage.image_layout(usage_type.is_read_only(), format);
        Self {
            sync,
            access,
            layout,
            queue_transition: None,
        }
    }
}

/// Types of access to some resource.
///
/// Basically a fancy, self documenting boolean.
#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Debug)]
pub enum UsageType {
    Read,
    Write,
}

impl UsageType {
    /// Returns true if this [`UsageType`] is a [`UsageType::Read`] variant.
    pub const fn is_read_only(&self) -> bool {
        matches!(self, UsageType::Read)
    }
}
