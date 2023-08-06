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
use crate::internal::conv::{image_layout_to_vk, load_op_to_vk, store_op_to_vk};
use crate::internal::rendering_info_key::RenderingInfoKey;
use crate::internal::slice_from_ptr_len_vk;
use crate::texture::RenderTargetView;
use aleph_rhi_api::BeginRenderingInfo;
use ash::prelude::VkResult;
use ash::vk;
use bumpalo::Bump;
use std::collections::HashMap;
use std::mem::ManuallyDrop;
use std::slice::from_raw_parts;

pub struct RenderPassCache {
    /// A memory pool used for allocating data for the keys in the hashmap. Must outlive the cache.
    bump: ManuallyDrop<Bump>,

    /// A hashtable that maps a BeginRenderingInfo to a matching VkRenderPass
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
    map: ManuallyDrop<HashMap<RenderingInfoKey, vk::RenderPass>>,
}

impl RenderPassCache {
    pub unsafe fn destroy(&self, device: &ash::Device) {
        for (_, &pass) in self.map.iter() {
            device.destroy_render_pass(pass, None);
        }
    }
}

impl RenderPassCache {
    pub fn new() -> Self {
        Self {
            bump: Default::default(),
            map: Default::default(),
        }
    }

    pub unsafe fn get_render_pass_for_create_info(
        &mut self,
        device: &Device,
        create_info: &vk::RenderPassCreateInfo2,
    ) -> VkResult<vk::RenderPass> {
        let key = RenderingInfoKey::from_info(&create_info);

        if let Some(hit) = self.map.get(key) {
            // We got a cache hit! Just return the existing pass and exit. Cleaning up the
            // allocations from the create info translation is the responsibility of the caller
            // (they're all allocated into the bump allocator)
            Ok(*hit)
        } else {
            // Cache miss :(. We're going to have to create a new pass object.
            let new = device
                .create_renderpass_2
                .create_render_pass2(&create_info, None)?;

            // The lifetime of the arrays in the create info are all tied to the outer function. To
            // correctly store the keys we have to make a copy of the create info with all the
            // arrays allocated out of the cache's internal arena. This guarantees that all the
            // references are valid for the lifetime of the map.
            let create_info = Self::clone_create_info_in(&create_info, &self.bump);
            let key = RenderingInfoKey::new(create_info);
            let old = self.map.insert(key, new);
            debug_assert!(old.is_none());

            Ok(new)
        }
    }

    pub unsafe fn get_render_pass_for_begin_rendering(
        &mut self,
        device: &Device,
        info: &BeginRenderingInfo,
        temp_bump: &Bump,
    ) -> VkResult<vk::RenderPass> {
        // We create our initial translation for creating a render pass in the outer temp allocator
        // and get our key wrapper so we can look up an existing pass in the cache.
        let create_info = Self::translate_begin_rendering_info(info, temp_bump);
        self.get_render_pass_for_create_info(device, &create_info)
    }

    unsafe fn translate_begin_rendering_info<'a, 'b>(
        info: &'b BeginRenderingInfo,
        bump: &'a Bump,
    ) -> vk::RenderPassCreateInfo2Builder<'a> {
        // Number of attachments is number of color attachments + 1 if we have a depth attachment
        let num_attachments =
            info.color_attachments.len() + info.depth_stencil_attachment.map(|_| 1).unwrap_or(0);

        let attachments: &mut [vk::AttachmentDescription2] =
            bump.alloc_slice_fill_default(num_attachments);
        let references: &mut [vk::AttachmentReference2] =
            bump.alloc_slice_fill_default(num_attachments);

        let mut attachment_index = 0;
        for v in info.color_attachments.iter() {
            let view = &*RenderTargetView::from_view(v.image_view);

            attachments[attachment_index] = vk::AttachmentDescription2::builder()
                .format(view.format)
                .samples(vk::SampleCountFlags::TYPE_1)
                .load_op(load_op_to_vk(&v.load_op))
                .store_op(store_op_to_vk(&v.store_op))
                .initial_layout(image_layout_to_vk(v.image_layout))
                .final_layout(image_layout_to_vk(v.image_layout))
                .build();

            references[attachment_index] = vk::AttachmentReference2::builder()
                .attachment(attachment_index as _)
                .layout(image_layout_to_vk(v.image_layout))
                .aspect_mask(vk::ImageAspectFlags::COLOR)
                .build();

            attachment_index += 1;
        }

        if let Some(v) = info.depth_stencil_attachment {
            let view = &*RenderTargetView::from_view(v.image_view);

            attachments[attachment_index] = vk::AttachmentDescription2::builder()
                .format(view.format)
                .samples(vk::SampleCountFlags::TYPE_1)
                .load_op(load_op_to_vk(&v.depth_load_op))
                .store_op(store_op_to_vk(&v.depth_store_op))
                .stencil_load_op(load_op_to_vk(&v.stencil_load_op))
                .stencil_store_op(store_op_to_vk(&v.stencil_store_op))
                .initial_layout(image_layout_to_vk(v.image_layout))
                .final_layout(image_layout_to_vk(v.image_layout))
                .build();

            references[attachment_index] = vk::AttachmentReference2::builder()
                .attachment(attachment_index as _)
                .layout(image_layout_to_vk(v.image_layout))
                .aspect_mask(vk::ImageAspectFlags::DEPTH | vk::ImageAspectFlags::STENCIL)
                .build();
        }

        let mut subpass = vk::SubpassDescription2::builder()
            .pipeline_bind_point(vk::PipelineBindPoint::GRAPHICS)
            .view_mask(0)
            .color_attachments(&references[0..info.color_attachments.len()]);
        if info.depth_stencil_attachment.is_some() {
            subpass = subpass.depth_stencil_attachment(references.last().unwrap());
        }
        let subpass = &*bump.alloc(subpass);

        vk::RenderPassCreateInfo2::builder()
            .attachments(attachments)
            .subpasses(std::slice::from_ref(&subpass))
        // .dependencies();
    }

    unsafe fn clone_create_info_in(
        info: &vk::RenderPassCreateInfo2,
        bump: &Bump,
    ) -> vk::RenderPassCreateInfo2 {
        let attachments = from_raw_parts(info.p_attachments, info.attachment_count as usize);
        let attachments = bump.alloc_slice_copy(attachments);

        let subpasses = from_raw_parts(info.p_subpasses, info.subpass_count as usize);
        let subpasses = bump.alloc_slice_fill_iter(subpasses.iter().map(|v| {
            let input_attachments = unsafe {
                let v = slice_from_ptr_len_vk(v.p_input_attachments, v.input_attachment_count);
                bump.alloc_slice_copy(v)
            };
            let color_attachments = unsafe {
                let v = slice_from_ptr_len_vk(v.p_color_attachments, v.color_attachment_count);
                bump.alloc_slice_copy(v)
            };
            let resolve_attachments = unsafe {
                let v = slice_from_ptr_len_vk(v.p_resolve_attachments, v.color_attachment_count);
                bump.alloc_slice_copy(v)
            };
            let depth_stencil_attachment = unsafe {
                if !v.p_depth_stencil_attachment.is_null() {
                    bump.alloc(*v.p_depth_stencil_attachment) as *mut vk::AttachmentReference2
                        as *const vk::AttachmentReference2
                } else {
                    std::ptr::null()
                }
            };
            let preserve_attachments = unsafe {
                let v =
                    slice_from_ptr_len_vk(v.p_preserve_attachments, v.preserve_attachment_count);
                bump.alloc_slice_copy(v)
            };

            let mut b = vk::SubpassDescription2::builder()
                .flags(v.flags)
                .pipeline_bind_point(v.pipeline_bind_point)
                .view_mask(v.view_mask)
                .input_attachments(input_attachments)
                .color_attachments(color_attachments)
                .resolve_attachments(resolve_attachments)
                .preserve_attachments(preserve_attachments);
            b.p_depth_stencil_attachment = depth_stencil_attachment;
            b.build()
        }));

        let dependencies = from_raw_parts(info.p_dependencies, info.dependency_count as usize);
        let dependencies = bump.alloc_slice_copy(dependencies);

        let view_masks = from_raw_parts(
            info.p_correlated_view_masks,
            info.correlated_view_mask_count as usize,
        );
        let view_masks = bump.alloc_slice_copy(view_masks);

        vk::RenderPassCreateInfo2::builder()
            .flags(info.flags)
            .attachments(&attachments)
            .subpasses(&subpasses)
            .dependencies(&dependencies)
            .correlated_view_masks(&view_masks)
            .build()
    }
}

impl Default for RenderPassCache {
    fn default() -> Self {
        Self::new()
    }
}

impl Drop for RenderPassCache {
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

unsafe impl Send for RenderPassCache {}
