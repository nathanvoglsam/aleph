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

use erupt::vk;

pub trait CheckMeetsProfile {
    fn meets_profile(&self, v: &Self) -> Option<()>;
}

macro_rules! delegate {
    ($base:ident, $compare:ident, $v:ident) => {
        let text = stringify!($v);
        let b = &$base.$v;
        let c = &$compare.$v;
        let result = b.meets_profile(c);
        if result.is_none() {
            log::error!(
                "Device does not meet requirement: '{}'. Want {:?}, got {:?}",
                text,
                b,
                c
            );
            return None;
        }
    };
}

macro_rules! feat {
    ($base:ident, $compare:ident, $v:ident) => {
        let text = stringify!($v);
        if $compare.$v == vk::TRUE && $base.$v != vk::TRUE {
            log::error!("Device does not support feature: '{}'", text);
            return None;
        }
    };
}

macro_rules! feat_mask {
    ($base:ident, $compare:ident, $v:ident) => {
        let b = $base.$v;
        let c = $compare.$v;
        let t = stringify!($v);
        if !b.contains(c) {
            log::error!(
                "Device does not support feature: '{}'. Want {:?}, got {:?}",
                t,
                b,
                c
            );
            return None;
        }
    };
}

macro_rules! range {
    ($base:ident, $compare:ident, $v:ident) => {{
        let limit_name = stringify!($v);
        let b = &$base.$v;
        let c = &$compare.$v;
        if b[0] > c[0] || b[1] < c[1] {
            log::error!(
                "Device limit '{limit_name}' incompatible. Want: {:?}, got {:?}",
                b,
                c
            );
            return None;
        }
    }};
}

macro_rules! lmin {
    ($base:ident, $compare:ident, $v:ident) => {{
        let limit_name = stringify!($v);
        let limit = $base.$v;
        let min = $compare.$v;
        if limit < min {
            log::error!("Device limit '{limit_name}' too low. Want: {min}, got {limit}");
            return None;
        }
    }};
}

macro_rules! lmin_arr {
    ($base:ident, $compare:ident, $v:ident) => {{
        let limit_name = stringify!($v);
        let limit = &$base.$v;
        let min = &$compare.$v;
        for (l, m) in limit.iter().zip(min.iter()) {
            if l < m {
                log::error!(
                    "Device limit '{limit_name}' too low. Want: {:?}, got {:?}",
                    min,
                    limit
                );
                return None;
            }
        }
    }};
}

macro_rules! lmax {
    ($base:ident, $compare:ident, $v:ident) => {{
        let limit_name = stringify!($v);
        let limit = $base.$v;
        let max = $compare.$v;
        if limit > max {
            log::error!("Device limit '{limit_name}' too high. Want: {max}, got {limit}");
            return None;
        }
    }};
}

// macro_rules! lmax_arr {
//     ($base:ident, $compare:ident, $v:ident) => {{
//         let limit_name = stringify!($v);
//         let limit = &$base.$v;
//         let max = &$compare.$v;
//         for (l, m) in limit.iter().zip(max.iter()) {
//             if l >= m {
//                 log::error!(
//                     "Device limit '{limit_name}' too high. Want: {:?}, got {:?}",
//                     max,
//                     limit
//                 );
//                 return None;
//             }
//         }
//     }};
// }

impl CheckMeetsProfile for vk::PhysicalDeviceLimits {
    #[rustfmt::skip]
    fn meets_profile(&self, v: &Self) -> Option<()> {
        lmin!(self, v, max_image_dimension1_d);
        lmin!(self, v, max_image_dimension2_d);
        lmin!(self, v, max_image_dimension3_d);
        lmin!(self, v, max_image_dimension_cube);
        lmin!(self, v, max_image_array_layers);
        lmin!(self, v, max_texel_buffer_elements);
        lmin!(self, v, max_uniform_buffer_range);
        lmin!(self, v, max_storage_buffer_range);
        lmin!(self, v, max_push_constants_size);
        lmin!(self, v, max_memory_allocation_count);
        lmin!(self, v, max_sampler_allocation_count);
        lmax!(self, v, buffer_image_granularity);
        lmin!(self, v, sparse_address_space_size);
        lmin!(self, v, max_bound_descriptor_sets);
        lmin!(self, v, max_per_stage_descriptor_samplers);
        lmin!(self, v, max_per_stage_descriptor_uniform_buffers);
        lmin!(self, v, max_per_stage_descriptor_storage_buffers);
        lmin!(self, v, max_per_stage_descriptor_sampled_images);
        lmin!(self, v, max_per_stage_descriptor_storage_images);
        lmin!(self, v, max_per_stage_descriptor_input_attachments);
        lmin!(self, v, max_per_stage_resources);
        lmin!(self, v, max_descriptor_set_samplers);
        lmin!(self, v, max_descriptor_set_uniform_buffers);
        lmin!(self, v, max_descriptor_set_uniform_buffers_dynamic);
        lmin!(self, v, max_descriptor_set_storage_buffers);
        lmin!(self, v, max_descriptor_set_storage_buffers_dynamic);
        lmin!(self, v, max_descriptor_set_sampled_images);
        lmin!(self, v, max_descriptor_set_storage_images);
        lmin!(self, v, max_descriptor_set_input_attachments);
        lmin!(self, v, max_vertex_input_attributes);
        lmin!(self, v, max_vertex_input_bindings);
        lmin!(self, v, max_vertex_input_attribute_offset);
        lmin!(self, v, max_vertex_input_binding_stride);
        lmin!(self, v, max_vertex_output_components);
        lmin!(self, v, max_tessellation_generation_level);
        lmin!(self, v, max_tessellation_patch_size);
        lmin!(self, v, max_tessellation_control_per_vertex_input_components);
        lmin!(self, v, max_tessellation_control_per_vertex_output_components);
        lmin!(self, v, max_tessellation_control_per_patch_output_components);
        lmin!(self, v, max_tessellation_control_total_output_components);
        lmin!(self, v, max_tessellation_evaluation_input_components);
        lmin!(self, v, max_tessellation_evaluation_output_components);
        lmin!(self, v, max_geometry_shader_invocations);
        lmin!(self, v, max_geometry_input_components);
        lmin!(self, v, max_geometry_output_components);
        lmin!(self, v, max_geometry_output_vertices);
        lmin!(self, v, max_geometry_total_output_components);
        lmin!(self, v, max_fragment_input_components);
        lmin!(self, v, max_fragment_output_attachments);
        lmin!(self, v, max_fragment_dual_src_attachments);
        lmin!(self, v, max_fragment_combined_output_resources);
        lmin!(self, v, max_compute_shared_memory_size);
        lmin_arr!(self, v, max_compute_work_group_count);
        lmin!(self, v, max_compute_work_group_invocations);
        lmin_arr!(self, v, max_compute_work_group_size);
        lmin!(self, v, sub_pixel_precision_bits);
        lmin!(self, v, sub_texel_precision_bits);
        lmin!(self, v, mipmap_precision_bits);
        lmin!(self, v, max_draw_indexed_index_value);
        lmin!(self, v, max_draw_indirect_count);
        lmin!(self, v, max_sampler_lod_bias);
        lmin!(self, v, max_sampler_anisotropy);
        lmin!(self, v, max_viewports);
        lmin_arr!(self, v, max_viewport_dimensions);
        range!(self, v, viewport_bounds_range);
        lmin!(self, v, viewport_sub_pixel_bits);
        lmax!(self, v, min_memory_map_alignment);
        lmax!(self, v, min_texel_buffer_offset_alignment);
        lmax!(self, v, min_uniform_buffer_offset_alignment);
        lmax!(self, v, min_storage_buffer_offset_alignment);
        lmax!(self, v, min_texel_offset);
        lmin!(self, v, max_texel_offset);
        lmax!(self, v, min_texel_gather_offset);
        lmin!(self, v, max_texel_gather_offset);
        lmax!(self, v, min_interpolation_offset);
        lmin!(self, v, max_interpolation_offset);
        lmin!(self, v, sub_pixel_interpolation_offset_bits);
        lmin!(self, v, max_framebuffer_width);
        lmin!(self, v, max_framebuffer_height);
        lmin!(self, v, max_framebuffer_layers);
        feat_mask!(self, v, framebuffer_color_sample_counts);
        feat_mask!(self, v, framebuffer_depth_sample_counts);
        feat_mask!(self, v, framebuffer_stencil_sample_counts);
        feat_mask!(self, v, framebuffer_no_attachments_sample_counts);
        lmin!(self, v, max_color_attachments);
        feat_mask!(self, v, sampled_image_color_sample_counts);
        feat_mask!(self, v, sampled_image_integer_sample_counts);
        feat_mask!(self, v, sampled_image_depth_sample_counts);
        feat_mask!(self, v, sampled_image_stencil_sample_counts);
        feat_mask!(self, v, storage_image_sample_counts);
        lmin!(self, v, max_sample_mask_words);
        feat!(self, v, timestamp_compute_and_graphics);
        lmax!(self, v, timestamp_period);
        lmin!(self, v, max_clip_distances);
        lmin!(self, v, max_cull_distances);
        lmin!(self, v, max_combined_clip_and_cull_distances);
        lmin!(self, v, discrete_queue_priorities);
        range!(self, v, point_size_range);
        range!(self, v, line_width_range);
        lmax!(self, v, point_size_granularity);
        lmax!(self, v, line_width_granularity);
        feat!(self, v, strict_lines);
        feat!(self, v, standard_sample_locations);
        lmax!(self, v, optimal_buffer_copy_offset_alignment);
        lmax!(self, v, optimal_buffer_copy_row_pitch_alignment);
        lmax!(self, v, non_coherent_atom_size);
        Some(())
    }
}

impl CheckMeetsProfile for vk::PhysicalDeviceSparseProperties {
    #[rustfmt::skip]
    fn meets_profile(&self, v: &Self) -> Option<()> {
        feat!(self, v, residency_standard2_d_block_shape);
        feat!(self, v, residency_standard2_d_multisample_block_shape);
        feat!(self, v, residency_standard3_d_block_shape);
        feat!(self, v, residency_aligned_mip_size);
        feat!(self, v, residency_non_resident_strict);
        Some(())
    }
}

impl CheckMeetsProfile for vk::PhysicalDeviceProperties {
    fn meets_profile(&self, v: &Self) -> Option<()> {
        let major_version = vk::api_version_major(self.api_version);
        let wanted_major_version = vk::api_version_major(v.api_version);
        if major_version < wanted_major_version {
            log::error!(
                "Incompatible API major version. \
                Wanted {wanted_major_version}.x. \
                Got {major_version}.x"
            );
            return None;
        }

        let minor_version = vk::api_version_minor(self.api_version);
        let wanted_minor_version = vk::api_version_minor(v.api_version);
        if minor_version < wanted_minor_version {
            log::error!(
                "Incompatible API major version. \
                Wanted {wanted_major_version}.{wanted_minor_version}. \
                Got {major_version}.{minor_version}"
            );
            return None;
        }

        self.limits.meets_profile(&v.limits)?;
        self.sparse_properties.meets_profile(&v.sparse_properties)?;
        Some(())
    }
}

impl CheckMeetsProfile for vk::PhysicalDeviceVulkan11Properties {
    #[rustfmt::skip]
    fn meets_profile(&self, v: &Self) -> Option<()> {
        lmin!(self, v, subgroup_size);
        feat_mask!(self, v, subgroup_supported_stages);
        feat_mask!(self, v, subgroup_supported_operations);
        feat!(self, v, subgroup_quad_operations_in_all_stages);
        delegate!(self, v, point_clipping_behavior);
        lmin!(self, v, max_multiview_view_count);
        lmin!(self, v, max_multiview_instance_index);
        feat!(self, v, protected_no_fault);
        lmin!(self, v, max_per_set_descriptors);
        lmin!(self, v, max_memory_allocation_size);
        Some(())
    }
}

impl CheckMeetsProfile for vk::PhysicalDeviceVulkan12Properties {
    #[rustfmt::skip]
    fn meets_profile(&self, v: &Self) -> Option<()> {
        delegate!(self, v, conformance_version);
        delegate!(self, v, denorm_behavior_independence);
        delegate!(self, v, rounding_mode_independence);
        feat!(self, v, shader_signed_zero_inf_nan_preserve_float16);
        feat!(self, v, shader_signed_zero_inf_nan_preserve_float32);
        feat!(self, v, shader_signed_zero_inf_nan_preserve_float64);
        feat!(self, v, shader_denorm_preserve_float16);
        feat!(self, v, shader_denorm_preserve_float32);
        feat!(self, v, shader_denorm_preserve_float64);
        feat!(self, v, shader_denorm_flush_to_zero_float16);
        feat!(self, v, shader_denorm_flush_to_zero_float32);
        feat!(self, v, shader_denorm_flush_to_zero_float64);
        feat!(self, v, shader_rounding_mode_rte_float16);
        feat!(self, v, shader_rounding_mode_rte_float32);
        feat!(self, v, shader_rounding_mode_rte_float64);
        feat!(self, v, shader_rounding_mode_rtz_float16);
        feat!(self, v, shader_rounding_mode_rtz_float32);
        feat!(self, v, shader_rounding_mode_rtz_float64);
        lmin!(self, v, max_update_after_bind_descriptors_in_all_pools);
        feat!(self, v, shader_uniform_buffer_array_non_uniform_indexing_native);
        feat!(self, v, shader_sampled_image_array_non_uniform_indexing_native);
        feat!(self, v, shader_storage_buffer_array_non_uniform_indexing_native);
        feat!(self, v, shader_storage_image_array_non_uniform_indexing_native);
        feat!(self, v, shader_input_attachment_array_non_uniform_indexing_native);
        feat!(self, v, robust_buffer_access_update_after_bind);
        feat!(self, v, quad_divergent_implicit_lod);
        lmin!(self, v, max_per_stage_descriptor_update_after_bind_samplers);
        lmin!(self, v, max_per_stage_descriptor_update_after_bind_uniform_buffers);
        lmin!(self, v, max_per_stage_descriptor_update_after_bind_storage_buffers);
        lmin!(self, v, max_per_stage_descriptor_update_after_bind_sampled_images);
        lmin!(self, v, max_per_stage_descriptor_update_after_bind_storage_images);
        lmin!(self, v, max_per_stage_descriptor_update_after_bind_input_attachments);
        lmin!(self, v, max_per_stage_update_after_bind_resources);
        lmin!(self, v, max_descriptor_set_update_after_bind_samplers);
        lmin!(self, v, max_descriptor_set_update_after_bind_uniform_buffers);
        lmin!(self, v, max_descriptor_set_update_after_bind_uniform_buffers_dynamic);
        lmin!(self, v, max_descriptor_set_update_after_bind_storage_buffers);
        lmin!(self, v, max_descriptor_set_update_after_bind_storage_buffers_dynamic);
        lmin!(self, v, max_descriptor_set_update_after_bind_sampled_images);
        lmin!(self, v, max_descriptor_set_update_after_bind_storage_images);
        lmin!(self, v, max_descriptor_set_update_after_bind_input_attachments);
        feat_mask!(self, v, supported_depth_resolve_modes);
        feat_mask!(self, v, supported_stencil_resolve_modes);
        feat!(self, v, independent_resolve_none);
        feat!(self, v, independent_resolve);
        feat!(self, v, filter_minmax_single_component_formats);
        feat!(self, v, filter_minmax_image_component_mapping);
        lmin!(self, v, max_timeline_semaphore_value_difference);
        feat_mask!(self, v, framebuffer_integer_color_sample_counts);
        Some(())
    }
}

impl CheckMeetsProfile for vk::PhysicalDeviceFeatures {
    #[rustfmt::skip]
    fn meets_profile(&self, v: &Self) -> Option<()> {
        feat!(self, v, robust_buffer_access);
        feat!(self, v, full_draw_index_uint32);
        feat!(self, v, image_cube_array);
        feat!(self, v, independent_blend);
        feat!(self, v, geometry_shader);
        feat!(self, v, tessellation_shader);
        feat!(self, v, sample_rate_shading);
        feat!(self, v, dual_src_blend);
        feat!(self, v, logic_op);
        feat!(self, v, multi_draw_indirect);
        feat!(self, v, draw_indirect_first_instance);
        feat!(self, v, depth_clamp);
        feat!(self, v, depth_bias_clamp);
        feat!(self, v, fill_mode_non_solid);
        feat!(self, v, depth_bounds);
        feat!(self, v, wide_lines);
        feat!(self, v, large_points);
        feat!(self, v, alpha_to_one);
        feat!(self, v, multi_viewport);
        feat!(self, v, sampler_anisotropy);
        feat!(self, v, texture_compression_etc2);
        feat!(self, v, texture_compression_astc_ldr);
        feat!(self, v, texture_compression_bc);
        feat!(self, v, occlusion_query_precise);
        feat!(self, v, pipeline_statistics_query);
        feat!(self, v, vertex_pipeline_stores_and_atomics);
        feat!(self, v, fragment_stores_and_atomics);
        feat!(self, v, shader_tessellation_and_geometry_point_size);
        feat!(self, v, shader_image_gather_extended);
        feat!(self, v, shader_storage_image_extended_formats);
        feat!(self, v, shader_storage_image_multisample);
        feat!(self, v, shader_storage_image_read_without_format);
        feat!(self, v, shader_storage_image_write_without_format);
        feat!(self, v, shader_uniform_buffer_array_dynamic_indexing);
        feat!(self, v, shader_sampled_image_array_dynamic_indexing);
        feat!(self, v, shader_storage_buffer_array_dynamic_indexing);
        feat!(self, v, shader_storage_image_array_dynamic_indexing);
        feat!(self, v, shader_clip_distance);
        feat!(self, v, shader_cull_distance);
        feat!(self, v, shader_float64);
        feat!(self, v, shader_int64);
        feat!(self, v, shader_int16);
        feat!(self, v, shader_resource_residency);
        feat!(self, v, shader_resource_min_lod);
        feat!(self, v, sparse_binding);
        feat!(self, v, sparse_residency_buffer);
        feat!(self, v, sparse_residency_image2_d);
        feat!(self, v, sparse_residency_image3_d);
        feat!(self, v, sparse_residency2_samples);
        feat!(self, v, sparse_residency4_samples);
        feat!(self, v, sparse_residency8_samples);
        feat!(self, v, sparse_residency16_samples);
        feat!(self, v, sparse_residency_aliased);
        feat!(self, v, variable_multisample_rate);
        feat!(self, v, inherited_queries);
        Some(())
    }
}

impl CheckMeetsProfile for vk::PhysicalDeviceVulkan11Features {
    #[rustfmt::skip]
    fn meets_profile(&self, v: &Self) -> Option<()> {
        feat!(self, v, storage_buffer16_bit_access);
        feat!(self, v, uniform_and_storage_buffer16_bit_access);
        feat!(self, v, storage_push_constant16);
        feat!(self, v, storage_input_output16);
        feat!(self, v, multiview);
        feat!(self, v, multiview_geometry_shader);
        feat!(self, v, multiview_tessellation_shader);
        feat!(self, v, variable_pointers_storage_buffer);
        feat!(self, v, variable_pointers);
        feat!(self, v, protected_memory);
        feat!(self, v, sampler_ycbcr_conversion);
        feat!(self, v, shader_draw_parameters);
        Some(())
    }
}

impl CheckMeetsProfile for vk::PhysicalDeviceVulkan12Features {
    #[rustfmt::skip]
    fn meets_profile(&self, v: &Self) -> Option<()> {
        feat!(self, v, sampler_mirror_clamp_to_edge);
        feat!(self, v, draw_indirect_count);
        feat!(self, v, storage_buffer8_bit_access);
        feat!(self, v, uniform_and_storage_buffer8_bit_access);
        feat!(self, v, storage_push_constant8);
        feat!(self, v, shader_buffer_int64_atomics);
        feat!(self, v, shader_shared_int64_atomics);
        feat!(self, v, shader_float16);
        feat!(self, v, shader_int8);
        feat!(self, v, descriptor_indexing);
        feat!(self, v, shader_input_attachment_array_dynamic_indexing);
        feat!(self, v, shader_uniform_texel_buffer_array_dynamic_indexing);
        feat!(self, v, shader_storage_texel_buffer_array_dynamic_indexing);
        feat!(self, v, shader_uniform_buffer_array_non_uniform_indexing);
        feat!(self, v, shader_sampled_image_array_non_uniform_indexing);
        feat!(self, v, shader_storage_buffer_array_non_uniform_indexing);
        feat!(self, v, shader_storage_image_array_non_uniform_indexing);
        feat!(self, v, shader_input_attachment_array_non_uniform_indexing);
        feat!(self, v, shader_uniform_texel_buffer_array_non_uniform_indexing);
        feat!(self, v, shader_storage_texel_buffer_array_non_uniform_indexing);
        feat!(self, v, descriptor_binding_uniform_buffer_update_after_bind);
        feat!(self, v, descriptor_binding_sampled_image_update_after_bind);
        feat!(self, v, descriptor_binding_storage_image_update_after_bind);
        feat!(self, v, descriptor_binding_storage_buffer_update_after_bind);
        feat!(self, v, descriptor_binding_uniform_texel_buffer_update_after_bind);
        feat!(self, v, descriptor_binding_storage_texel_buffer_update_after_bind);
        feat!(self, v, descriptor_binding_update_unused_while_pending);
        feat!(self, v, descriptor_binding_partially_bound);
        feat!(self, v, descriptor_binding_variable_descriptor_count);
        feat!(self, v, runtime_descriptor_array);
        feat!(self, v, sampler_filter_minmax);
        feat!(self, v, scalar_block_layout);
        feat!(self, v, imageless_framebuffer);
        feat!(self, v, uniform_buffer_standard_layout);
        feat!(self, v, shader_subgroup_extended_types);
        feat!(self, v, separate_depth_stencil_layouts);
        feat!(self, v, host_query_reset);
        feat!(self, v, timeline_semaphore);
        feat!(self, v, buffer_device_address);
        feat!(self, v, buffer_device_address_capture_replay);
        feat!(self, v, buffer_device_address_multi_device);
        feat!(self, v, vulkan_memory_model);
        feat!(self, v, vulkan_memory_model_device_scope);
        feat!(self, v, vulkan_memory_model_availability_visibility_chains);
        feat!(self, v, shader_output_viewport_index);
        feat!(self, v, shader_output_layer);
        feat!(self, v, subgroup_broadcast_dynamic_id);
        Some(())
    }
}

impl CheckMeetsProfile for vk::PhysicalDeviceDynamicRenderingFeatures {
    #[rustfmt::skip]
    fn meets_profile(&self, v: &Self) -> Option<()> {
        feat!(self, v, dynamic_rendering);
        Some(())
    }
}

impl CheckMeetsProfile for vk::PhysicalDeviceSynchronization2Features {
    #[rustfmt::skip]
    fn meets_profile(&self, v: &Self) -> Option<()> {
        feat!(self, v, synchronization2);
        Some(())
    }
}

impl CheckMeetsProfile for vk::PointClippingBehavior {
    fn meets_profile(&self, v: &Self) -> Option<()> {
        // USER_CLIP_PLANES_ONLY is the default

        // ALL_CLIP_PLANES is the most capable and must match
        if *self == vk::PointClippingBehavior::ALL_CLIP_PLANES {
            if *v != vk::PointClippingBehavior::ALL_CLIP_PLANES {
                return None;
            }
        }
        Some(())
    }
}

impl CheckMeetsProfile for vk::ConformanceVersion {
    fn meets_profile(&self, v: &Self) -> Option<()> {
        Some(())
    }
}

impl CheckMeetsProfile for vk::ShaderFloatControlsIndependence {
    fn meets_profile(&self, v: &Self) -> Option<()> {
        // NONE is the default

        // only NONE is less capable than 32_ONLY
        if *self == vk::ShaderFloatControlsIndependence::_32_ONLY {
            if *v == vk::ShaderFloatControlsIndependence::NONE {
                return None;
            }
        }

        // ALL is the most capable
        if *self == vk::ShaderFloatControlsIndependence::ALL {
            if *v != vk::ShaderFloatControlsIndependence::ALL {
                return None;
            }
        }
        Some(())
    }
}
