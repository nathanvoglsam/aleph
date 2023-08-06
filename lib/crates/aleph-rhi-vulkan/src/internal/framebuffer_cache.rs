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

use crate::device::Device;
use crate::internal::framebuffer_cache_key::FramebufferCacheKey;
use ash::prelude::VkResult;
use ash::vk;
use bumpalo::Bump;
use std::collections::HashMap;
use std::mem::ManuallyDrop;

pub struct FramebufferCache {
    /// A memory pool used for allocating data for the keys in the hashmap. Must outlive the cache.
    bump: ManuallyDrop<Bump>,

    /// A hashtable that maps a list of image formats to a matching framebuffer
    ///
    /// We must be _very_ _very_ careful to never send references of the keys out as the lifetime
    /// is a lie. The struct is self referential, but with stable addresses.
    ///
    /// In order to keep this struct safe we must always allocate the lists inside the key using
    /// the bump allocator on this struct, we can _never_ send references to the key or copies of
    /// the key out of the struct (unless it's a deep copy) and we must guarantee that the arena
    /// outlives the map so the allocations remain live the whole time.
    ///
    /// We should justify _why_. We would like the cache hit path to never allocate, for obvious
    /// performance reasons. Using Vec inside RenderingInfoKey would be _safe_, but would require
    /// copying and allocating new arrays for any incoming keys we wanted to test against the map
    /// to work correctly with rust's hashing infrastructure. Instead we use the struct in-place,
    /// viewed through 'RenderingInfoKey' to control hashing semantics. All arrays point into an
    /// arena that outlives them.
    ///
    /// An enterprising rustacean can see this is safe, but rustc can't.
    map: ManuallyDrop<HashMap<FramebufferCacheKey<'static>, vk::Framebuffer>>,
}

impl FramebufferCache {
    pub unsafe fn destroy(&self, device: &ash::Device) {
        for (_, &fb) in self.map.iter() {
            device.destroy_framebuffer(fb, None);
        }
    }
}

impl FramebufferCache {
    pub fn new() -> Self {
        Self {
            bump: Default::default(),
            map: Default::default(),
        }
    }

    /// # Arguments
    ///
    /// - `device`      : The device handle, needed for calling Vulkan for creating framebuffers when
    ///                   we get a cache miss.
    /// - `key`         : The cache key to lookup the framebuffer for.
    /// - `width`       : The width of the attachments in the framebuffer. This will be the same
    ///                   between all the attachments, likely pulled from the first attachment in the
    ///                   framebuffer.
    /// - `height`      : See 'width', same semantics but for the height instead.
    /// - `render_pass` : Any compatible render pass so we can create the framebuffer. This will
    ///                 : probably be the pass pulled for begin_rendering.
    pub unsafe fn get_render_pass_for_key(
        &mut self,
        device: &Device,
        key: &FramebufferCacheKey,
        width: u32,
        height: u32,
        render_pass: vk::RenderPass,
    ) -> VkResult<vk::Framebuffer> {
        use bumpalo::collections::Vec as BVec;

        if let Some(hit) = self.map.get(key) {
            // We got a cache hit! Just return the existing fb and exit. Cleaning up the
            // allocations from the create info translation is the responsibility of the caller
            // (they're all allocated into the bump allocator)
            Ok(*hit)
        } else {
            let mut attachments = BVec::with_capacity_in(key.attachments.len(), &self.bump);
            for v in key.attachments {
                let v = vk::FramebufferAttachmentImageInfo::builder()
                    .flags(v.creation_flags)
                    .usage(v.usage)
                    .width(width)
                    .height(height)
                    .layer_count(key.layer_count)
                    .view_formats(std::slice::from_ref(&v.format));
                attachments.push(v.build());
            }

            // Cache miss :(. We're going to have to create a new framebuffer object.
            let mut attachment_infos = vk::FramebufferAttachmentsCreateInfo::builder()
                .attachment_image_infos(&attachments);
            let mut create_info = vk::FramebufferCreateInfo::builder()
                .flags(vk::FramebufferCreateFlags::IMAGELESS)
                .render_pass(render_pass)
                .width(width)
                .height(height)
                .layers(key.layer_count)
                .push_next(&mut attachment_infos);
            create_info.attachment_count = attachments.len() as _;
            let new = device
                .device
                .create_framebuffer(&create_info, None)
                .unwrap();

            // The lifetime of the arrays in the create info are all tied to the outer function. To
            // correctly store the keys we have to make a copy of the create info with all the
            // arrays allocated out of the cache's internal arena. This guarantees that all the
            // references are valid for the lifetime of the map.
            let key = FramebufferCacheKey {
                layer_count: key.layer_count,
                attachments: self.bump.alloc_slice_clone(&key.attachments),
            };

            // Can't get an old value here so drop it. Transmute for our naughty lifetime hacks.
            // Don't try this at home kids! We're 'trained professionals' (morons)
            let _ = self.map.insert(std::mem::transmute(key), new);

            Ok(new)
        }
    }
}

impl Default for FramebufferCache {
    fn default() -> Self {
        Self::new()
    }
}

impl Drop for FramebufferCache {
    fn drop(&mut self) {
        // Safety: This guarantees that the arena outlives the map. This is needed to uphold an
        //         internal invariant to keep the RenderPassCache interface safe. This operation is
        //         itself safe as we're just manually implementing drop with the required order.
        unsafe {
            ManuallyDrop::drop(&mut self.map);
            ManuallyDrop::drop(&mut self.bump);
        }
    }
}

unsafe impl Send for FramebufferCache {}
