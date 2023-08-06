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

use crate::internal::slice_from_ptr_len_vk;
use ash::vk;
use std::hash::{Hash, Hasher};

/// A new-type wrapper over [BeginRenderingInfo] with a manual Hash and PartialEq function that only
/// hashes and compares state that is used for constructing `VkRenderPass` objects.
///
/// `VkRenderPass` is agnostic to the specific images and clear colours, so we key from and compare
/// only the relevant pieces. This improves cache hit rates, but is also necessary to implement Hash
/// on [BeginRenderingInfo] correctly. Hash/Eq is not implemented on `f32` because of `inf/NaN`
/// breaking expected invariants of comparison and hashing equality. The only f32 fields are in the
/// clear colours, which we ignore, so we can safely implement Hash and Eq on the new-type and this
/// allows using the type in rust's HashMap.
#[repr(transparent)]
pub struct RenderingInfoKey(vk::RenderPassCreateInfo2);

impl Into<vk::RenderPassCreateInfo2> for RenderingInfoKey {
    fn into(self) -> vk::RenderPassCreateInfo2 {
        self.0
    }
}

impl Hash for RenderingInfoKey {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.0.flags.hash(state);
        self.hash_attachments(state);
        self.hash_subpasses(state);
        self.hash_subpass_dependencies(state);
        self.hash_correlated_view_masks(state);
    }
}

impl PartialEq for RenderingInfoKey {
    fn eq(&self, other: &Self) -> bool {
        if self.0.flags != other.0.flags {
            return false;
        }
        if self.eq_attachments(other).is_none() {
            return false;
        }
        if self.eq_subpasses(other).is_none() {
            return false;
        }
        if self.eq_subpass_dependencies(other).is_none() {
            return false;
        }
        if self.eq_correlated_view_masks(other).is_none() {
            return false;
        }
        true
    }
}

impl Eq for RenderingInfoKey {}

impl RenderingInfoKey {
    pub unsafe fn new(info: vk::RenderPassCreateInfo2) -> RenderingInfoKey {
        Self(info)
    }

    pub unsafe fn from_info(info: &vk::RenderPassCreateInfo2) -> &RenderingInfoKey {
        let info = info as *const vk::RenderPassCreateInfo2 as *const RenderingInfoKey;
        // Safety: This is safe because both types have the same layout (repr transparent) and the
        //         lifetime is correctly passed across. This is just a slightly different view of
        //         the same type
        &*info
    }

    fn hash_attachments(&self, state: &mut impl Hasher) {
        let attachments = unsafe {
            std::slice::from_raw_parts(self.0.p_attachments, self.0.attachment_count as usize)
        };

        self.0.attachment_count.hash(state);
        for attachment in attachments {
            attachment.flags.hash(state);
            attachment.format.hash(state);
            attachment.samples.hash(state);
            attachment.load_op.hash(state);
            attachment.store_op.hash(state);
            attachment.stencil_load_op.hash(state);
            attachment.stencil_store_op.hash(state);
            attachment.initial_layout.hash(state);
            attachment.final_layout.hash(state);
        }
    }

    fn eq_attachments(&self, other: &Self) -> Option<()> {
        let iter = unsafe {
            let l = slice_from_ptr_len_vk(self.0.p_attachments, self.0.attachment_count);
            let r = slice_from_ptr_len_vk(other.0.p_attachments, other.0.attachment_count);
            l.iter().zip(r.iter())
        };

        if self.0.attachment_count != other.0.attachment_count {
            return None;
        }
        for (l, r) in iter {
            cmp(&l.flags, &r.flags)?;
            cmp(&l.format, &r.format)?;
            cmp(&l.samples, &r.samples)?;
            cmp(&l.load_op, &r.load_op)?;
            cmp(&l.store_op, &r.store_op)?;
            cmp(&l.stencil_load_op, &r.stencil_load_op)?;
            cmp(&l.stencil_store_op, &r.stencil_store_op)?;
            cmp(&l.initial_layout, &r.initial_layout)?;
            cmp(&l.final_layout, &r.final_layout)?;
        }

        Some(())
    }

    fn hash_subpasses(&self, state: &mut impl Hasher) {
        let subpasses = unsafe { slice_from_ptr_len_vk(self.0.p_subpasses, self.0.subpass_count) };

        self.0.subpass_count.hash(state);
        for subpass in subpasses {
            let input_attachments = unsafe {
                slice_from_ptr_len_vk(subpass.p_input_attachments, subpass.input_attachment_count)
            };
            let color_attachments = unsafe {
                slice_from_ptr_len_vk(subpass.p_color_attachments, subpass.color_attachment_count)
            };
            let resolve_attachments = unsafe {
                slice_from_ptr_len_vk(
                    subpass.p_resolve_attachments,
                    subpass.color_attachment_count,
                )
            };
            let depth_stencil_attachment = unsafe { subpass.p_depth_stencil_attachment.as_ref() };
            let preserve_attachments = unsafe {
                slice_from_ptr_len_vk(
                    subpass.p_preserve_attachments,
                    subpass.preserve_attachment_count,
                )
            };

            subpass.flags.hash(state);
            subpass.pipeline_bind_point.hash(state);
            subpass.view_mask.hash(state);

            subpass.input_attachment_count.hash(state);
            for v in input_attachments {
                hash_attachment_reference(v, state);
            }

            subpass.color_attachment_count.hash(state);
            for v in color_attachments {
                hash_attachment_reference(v, state);
            }

            subpass.color_attachment_count.hash(state);
            for v in resolve_attachments {
                hash_attachment_reference(v, state);
            }

            if let Some(v) = depth_stencil_attachment {
                hash_attachment_reference(v, state);
            }

            subpass.preserve_attachment_count.hash(state);
            for preserve_attachment in preserve_attachments {
                preserve_attachment.hash(state);
            }
        }
    }

    fn eq_subpasses(&self, other: &Self) -> Option<()> {
        let iter = unsafe {
            let l = slice_from_ptr_len_vk(self.0.p_subpasses, self.0.subpass_count);
            let r = slice_from_ptr_len_vk(other.0.p_subpasses, other.0.subpass_count);
            l.iter().zip(r.iter())
        };

        if self.0.subpass_count != other.0.subpass_count {
            return None;
        }
        for (l, r) in iter {
            let input_attachments = unsafe {
                let l = slice_from_ptr_len_vk(l.p_input_attachments, l.input_attachment_count);
                let r = slice_from_ptr_len_vk(r.p_input_attachments, r.input_attachment_count);
                l.iter().zip(r.iter())
            };
            let color_attachments = unsafe {
                let l = slice_from_ptr_len_vk(l.p_color_attachments, l.color_attachment_count);
                let r = slice_from_ptr_len_vk(r.p_color_attachments, r.color_attachment_count);
                l.iter().zip(r.iter())
            };
            let resolve_attachments = unsafe {
                let l = slice_from_ptr_len_vk(l.p_resolve_attachments, l.color_attachment_count);
                let r = slice_from_ptr_len_vk(r.p_resolve_attachments, r.color_attachment_count);
                l.iter().zip(r.iter())
            };
            let depth_stencil_attachments = unsafe {
                let l = l.p_depth_stencil_attachment.as_ref();
                let r = r.p_depth_stencil_attachment.as_ref();
                match (l, r) {
                    (Some(l), Some(r)) => Some((l, r)),
                    (None, None) => None,
                    _ => return None,
                }
            };
            let preserve_attachments = unsafe {
                let l =
                    slice_from_ptr_len_vk(l.p_preserve_attachments, l.preserve_attachment_count);
                let r =
                    slice_from_ptr_len_vk(r.p_preserve_attachments, r.preserve_attachment_count);
                l.iter().zip(r.iter())
            };

            cmp(&l.flags, &r.flags)?;
            cmp(&l.pipeline_bind_point, &r.pipeline_bind_point)?;
            cmp(&l.view_mask, &r.view_mask)?;

            if l.input_attachment_count != r.input_attachment_count {
                return None;
            }
            for (l, r) in input_attachments {
                eq_attachment_reference(l, r)?;
            }

            if l.color_attachment_count != r.color_attachment_count {
                return None;
            }
            for (l, r) in color_attachments {
                eq_attachment_reference(l, r)?;
            }

            if l.color_attachment_count != r.color_attachment_count {
                return None;
            }
            for (l, r) in resolve_attachments {
                eq_attachment_reference(l, r)?;
            }

            if let Some((l, r)) = depth_stencil_attachments {
                eq_attachment_reference(l, r)?;
            }

            if l.preserve_attachment_count != r.preserve_attachment_count {
                return None;
            }
            for (l, r) in preserve_attachments {
                cmp(l, r)?;
            }
        }
        Some(())
    }

    fn hash_subpass_dependencies(&self, state: &mut impl Hasher) {
        let dependencies =
            unsafe { slice_from_ptr_len_vk(self.0.p_dependencies, self.0.dependency_count) };

        self.0.dependency_count.hash(state);
        for dependency in dependencies {
            dependency.src_subpass.hash(state);
            dependency.dst_subpass.hash(state);
            dependency.src_stage_mask.hash(state);
            dependency.dst_stage_mask.hash(state);
            dependency.src_access_mask.hash(state);
            dependency.dst_access_mask.hash(state);
            dependency.dependency_flags.hash(state);
            dependency.view_offset.hash(state);
        }
    }

    fn eq_subpass_dependencies(&self, other: &Self) -> Option<()> {
        let iter = unsafe {
            let l = slice_from_ptr_len_vk(self.0.p_dependencies, self.0.dependency_count);
            let r = slice_from_ptr_len_vk(other.0.p_dependencies, other.0.dependency_count);
            l.iter().zip(r.iter())
        };

        if self.0.dependency_count != other.0.dependency_count {
            return None;
        }
        for (l, r) in iter {
            cmp(&l.src_subpass, &r.src_subpass)?;
            cmp(&l.dst_subpass, &r.dst_subpass)?;
            cmp(&l.src_stage_mask, &r.src_stage_mask)?;
            cmp(&l.dst_stage_mask, &r.dst_stage_mask)?;
            cmp(&l.src_access_mask, &r.src_access_mask)?;
            cmp(&l.dst_access_mask, &r.dst_access_mask)?;
            cmp(&l.dependency_flags, &r.dependency_flags)?;
            cmp(&l.view_offset, &r.view_offset)?;
        }

        Some(())
    }

    fn hash_correlated_view_masks(&self, state: &mut impl Hasher) {
        let correlated_view_masks = unsafe {
            slice_from_ptr_len_vk(
                self.0.p_correlated_view_masks,
                self.0.correlated_view_mask_count,
            )
        };

        self.0.correlated_view_mask_count.hash(state);
        for mask in correlated_view_masks {
            mask.hash(state);
        }
    }

    fn eq_correlated_view_masks(&self, other: &Self) -> Option<()> {
        let iter = unsafe {
            let l = slice_from_ptr_len_vk(
                self.0.p_correlated_view_masks,
                self.0.correlated_view_mask_count,
            );
            let r = slice_from_ptr_len_vk(
                other.0.p_correlated_view_masks,
                other.0.correlated_view_mask_count,
            );
            l.iter().zip(r.iter())
        };

        if self.0.correlated_view_mask_count != other.0.correlated_view_mask_count {
            return None;
        }
        for (l, r) in iter {
            cmp(l, r)?;
        }

        Some(())
    }
}

fn hash_attachment_reference(v: &vk::AttachmentReference2, state: &mut impl Hasher) {
    v.attachment.hash(state);
    v.layout.hash(state);
    v.aspect_mask.hash(state);
}

fn eq_attachment_reference(
    l: &vk::AttachmentReference2,
    r: &vk::AttachmentReference2,
) -> Option<()> {
    cmp(&l.attachment, &r.attachment)?;
    cmp(&l.layout, &r.layout)?;
    cmp(&l.aspect_mask, &r.aspect_mask)?;
    Some(())
}

fn cmp<T: PartialEq + Eq>(l: &T, r: &T) -> Option<()> {
    if l.eq(r) {
        Some(())
    } else {
        None
    }
}

#[cfg(test)]
mod test {
    use ash::vk;
    use std::collections::hash_map::DefaultHasher;
    use std::hash::{Hash, Hasher};
    use crate::internal::render_pass_cache::RenderPassCache;
    use crate::internal::rendering_info_key::RenderingInfoKey;

    #[test]
    pub fn hash_test() {
        unsafe {
            hash_test_unsafe()
        }

    }

    unsafe fn hash_test_unsafe() {
        let attachments = [
            vk::AttachmentDescription2::builder()
                .flags(vk::AttachmentDescriptionFlags::MAY_ALIAS)
                .format(vk::Format::R8G8B8A8_UNORM)
                .samples(vk::SampleCountFlags::TYPE_8)
                .load_op(vk::AttachmentLoadOp::LOAD)
                .store_op(vk::AttachmentStoreOp::STORE)
                .stencil_load_op(vk::AttachmentLoadOp::LOAD)
                .stencil_store_op(vk::AttachmentStoreOp::STORE)
                .initial_layout(vk::ImageLayout::ATTACHMENT_OPTIMAL)
                .final_layout(vk::ImageLayout::GENERAL)
                .build(),
            vk::AttachmentDescription2::builder()
                .flags(vk::AttachmentDescriptionFlags::MAY_ALIAS)
                .format(vk::Format::R8_UNORM)
                .samples(vk::SampleCountFlags::TYPE_4)
                .load_op(vk::AttachmentLoadOp::DONT_CARE)
                .store_op(vk::AttachmentStoreOp::DONT_CARE)
                .stencil_load_op(vk::AttachmentLoadOp::DONT_CARE)
                .stencil_store_op(vk::AttachmentStoreOp::DONT_CARE)
                .initial_layout(vk::ImageLayout::GENERAL)
                .final_layout(vk::ImageLayout::UNDEFINED)
                .build(),
        ];

        let reference_1 = vk::AttachmentReference2::builder()
            .attachment(56)
            .layout(vk::ImageLayout::FRAGMENT_DENSITY_MAP_OPTIMAL_EXT)
            .aspect_mask(vk::ImageAspectFlags::DEPTH)
            .build();
        let reference_2 = vk::AttachmentReference2::builder()
            .attachment(21)
            .layout(vk::ImageLayout::SHARED_PRESENT_KHR)
            .aspect_mask(vk::ImageAspectFlags::PLANE_0)
            .build();
        let reference_3 = vk::AttachmentReference2::builder()
            .attachment(420)
            .layout(vk::ImageLayout::SHADER_READ_ONLY_OPTIMAL)
            .aspect_mask(vk::ImageAspectFlags::MEMORY_PLANE_0_EXT)
            .build();

        let input_attachments = [reference_1, reference_2, reference_3];
        let color_attachments = [reference_2, reference_1];
        let resolve_attachments = [reference_3, reference_1];
        let preserve_attachments = [1, 4];
        let subpasses = [vk::SubpassDescription2::builder()
            .flags(vk::SubpassDescriptionFlags::FRAGMENT_REGION_QCOM)
            .view_mask(0b110)
            .input_attachments(&input_attachments)
            .color_attachments(&color_attachments)
            // .resolve_attachments(&resolve_attachments)
            .depth_stencil_attachment(&reference_3)
            .preserve_attachments(&preserve_attachments)
            .build()];

        let info = vk::RenderPassCreateInfo2::builder()
            .flags(vk::RenderPassCreateFlags::TRANSFORM_QCOM)
            .attachments(&attachments)
            .subpasses(&subpasses)
            // .dependencies()
            // .correlated_view_masks()
            .build();

        let info_key = RenderingInfoKey::from_info(&info);

        let bump = bumpalo::Bump::new();
        let cloned_info = RenderPassCache::clone_create_info_in(&info, &bump);
        let cloned_info_key = RenderingInfoKey::from_info(&cloned_info);

        let mut hasher = DefaultHasher::new();
        info_key.hash(&mut hasher);
        let hash = hasher.finish();

        let mut hasher = DefaultHasher::new();
        cloned_info_key.hash(&mut hasher);
        let cloned_hash = hasher.finish();

        assert_eq!(hash, cloned_hash);
        assert!(info_key == cloned_info_key);
    }
}
