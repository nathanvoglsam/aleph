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

use ash::vk;
use std::ffi::c_float;

pub trait CreateProfile {
    fn baseline() -> Self;

    fn minimum() -> Self;
}

impl CreateProfile for vk::PhysicalDeviceLimits {
    fn baseline() -> Self {
        Self {
            max_image_dimension1_d: 0,
            max_image_dimension2_d: 0,
            max_image_dimension3_d: 0,
            max_image_dimension_cube: 0,
            max_image_array_layers: 0,
            max_texel_buffer_elements: 0,
            max_uniform_buffer_range: 0,
            max_storage_buffer_range: 0,
            max_push_constants_size: 0,
            max_memory_allocation_count: 0,
            max_sampler_allocation_count: 0,
            buffer_image_granularity: vk::DeviceSize::MAX,
            sparse_address_space_size: vk::DeviceSize::MIN,
            max_bound_descriptor_sets: u32::MIN,
            max_per_stage_descriptor_samplers: u32::MIN,
            max_per_stage_descriptor_uniform_buffers: u32::MIN,
            max_per_stage_descriptor_storage_buffers: u32::MIN,
            max_per_stage_descriptor_sampled_images: u32::MIN,
            max_per_stage_descriptor_storage_images: u32::MIN,
            max_per_stage_descriptor_input_attachments: u32::MIN,
            max_per_stage_resources: u32::MIN,
            max_descriptor_set_samplers: u32::MIN,
            max_descriptor_set_uniform_buffers: u32::MIN,
            max_descriptor_set_uniform_buffers_dynamic: u32::MIN,
            max_descriptor_set_storage_buffers: u32::MIN,
            max_descriptor_set_storage_buffers_dynamic: u32::MIN,
            max_descriptor_set_sampled_images: u32::MIN,
            max_descriptor_set_storage_images: u32::MIN,
            max_descriptor_set_input_attachments: u32::MIN,
            max_vertex_input_attributes: u32::MIN,
            max_vertex_input_bindings: u32::MIN,
            max_vertex_input_attribute_offset: u32::MIN,
            max_vertex_input_binding_stride: u32::MIN,
            max_vertex_output_components: u32::MIN,
            max_tessellation_generation_level: u32::MIN,
            max_tessellation_patch_size: u32::MIN,
            max_tessellation_control_per_vertex_input_components: u32::MIN,
            max_tessellation_control_per_vertex_output_components: u32::MIN,
            max_tessellation_control_per_patch_output_components: u32::MIN,
            max_tessellation_control_total_output_components: u32::MIN,
            max_tessellation_evaluation_input_components: u32::MIN,
            max_tessellation_evaluation_output_components: u32::MIN,
            max_geometry_shader_invocations: u32::MIN,
            max_geometry_input_components: u32::MIN,
            max_geometry_output_components: u32::MIN,
            max_geometry_output_vertices: u32::MIN,
            max_geometry_total_output_components: u32::MIN,
            max_fragment_input_components: u32::MIN,
            max_fragment_output_attachments: u32::MIN,
            max_fragment_dual_src_attachments: u32::MIN,
            max_fragment_combined_output_resources: u32::MIN,
            max_compute_shared_memory_size: u32::MIN,
            max_compute_work_group_count: [u32::MIN, u32::MIN, u32::MIN],
            max_compute_work_group_invocations: u32::MIN,
            max_compute_work_group_size: [u32::MIN, u32::MIN, u32::MIN],
            sub_pixel_precision_bits: u32::MIN,
            sub_texel_precision_bits: u32::MIN,
            mipmap_precision_bits: u32::MIN,
            max_draw_indexed_index_value: u32::MIN,
            max_draw_indirect_count: u32::MIN,
            max_sampler_lod_bias: c_float::NEG_INFINITY,
            max_sampler_anisotropy: c_float::NEG_INFINITY,
            max_viewports: u32::MIN,
            max_viewport_dimensions: [u32::MIN, u32::MIN],
            viewport_bounds_range: [c_float::INFINITY, c_float::NEG_INFINITY],
            viewport_sub_pixel_bits: u32::MIN,
            min_memory_map_alignment: usize::MAX,
            min_texel_buffer_offset_alignment: vk::DeviceSize::MAX,
            min_uniform_buffer_offset_alignment: vk::DeviceSize::MAX,
            min_storage_buffer_offset_alignment: vk::DeviceSize::MAX,
            min_texel_offset: i32::MAX,
            max_texel_offset: u32::MIN,
            min_texel_gather_offset: i32::MAX,
            max_texel_gather_offset: u32::MIN,
            min_interpolation_offset: c_float::INFINITY,
            max_interpolation_offset: c_float::NEG_INFINITY,
            sub_pixel_interpolation_offset_bits: u32::MIN,
            max_framebuffer_width: u32::MIN,
            max_framebuffer_height: u32::MIN,
            max_framebuffer_layers: u32::MIN,
            framebuffer_color_sample_counts: Default::default(),
            framebuffer_depth_sample_counts: Default::default(),
            framebuffer_stencil_sample_counts: Default::default(),
            framebuffer_no_attachments_sample_counts: Default::default(),
            max_color_attachments: u32::MIN,
            sampled_image_color_sample_counts: Default::default(),
            sampled_image_integer_sample_counts: Default::default(),
            sampled_image_depth_sample_counts: Default::default(),
            sampled_image_stencil_sample_counts: Default::default(),
            storage_image_sample_counts: Default::default(),
            max_sample_mask_words: u32::MIN,
            timestamp_compute_and_graphics: vk::FALSE,
            timestamp_period: c_float::INFINITY,
            max_clip_distances: u32::MIN,
            max_cull_distances: u32::MIN,
            max_combined_clip_and_cull_distances: u32::MIN,
            discrete_queue_priorities: u32::MIN,
            point_size_range: [c_float::INFINITY, c_float::NEG_INFINITY],
            line_width_range: [c_float::INFINITY, c_float::NEG_INFINITY],
            point_size_granularity: c_float::INFINITY,
            line_width_granularity: c_float::INFINITY,
            strict_lines: Default::default(),
            standard_sample_locations: Default::default(),
            optimal_buffer_copy_offset_alignment: vk::DeviceSize::MAX,
            optimal_buffer_copy_row_pitch_alignment: vk::DeviceSize::MAX,
            non_coherent_atom_size: vk::DeviceSize::MAX,
        }
    }

    fn minimum() -> Self {
        Self {
            max_bound_descriptor_sets: 8,
            ..CreateProfile::baseline()
        }
    }
}

impl CreateProfile for vk::PhysicalDeviceSparseProperties {
    fn baseline() -> Self {
        Self {
            residency_standard2_d_block_shape: vk::FALSE,
            residency_standard2_d_multisample_block_shape: vk::FALSE,
            residency_standard3_d_block_shape: vk::FALSE,
            residency_aligned_mip_size: vk::FALSE,
            residency_non_resident_strict: vk::FALSE,
        }
    }

    fn minimum() -> Self {
        Self {
            ..CreateProfile::baseline()
        }
    }
}

impl CreateProfile for vk::PhysicalDeviceProperties {
    fn baseline() -> Self {
        Self {
            limits: CreateProfile::baseline(),
            sparse_properties: CreateProfile::baseline(),
            ..Default::default()
        }
    }

    fn minimum() -> Self {
        Self {
            limits: CreateProfile::minimum(),
            sparse_properties: CreateProfile::minimum(),
            ..CreateProfile::baseline()
        }
    }
}

impl CreateProfile for vk::PhysicalDeviceVulkan11Properties {
    fn baseline() -> Self {
        Self {
            subgroup_size: u32::MIN,
            subgroup_supported_stages: Default::default(),
            subgroup_supported_operations: Default::default(),
            subgroup_quad_operations_in_all_stages: vk::FALSE,
            point_clipping_behavior: Default::default(),
            max_multiview_view_count: u32::MIN,
            max_multiview_instance_index: u32::MIN,
            protected_no_fault: vk::FALSE,
            max_per_set_descriptors: u32::MIN,
            max_memory_allocation_size: vk::DeviceSize::MIN,
            ..Default::default()
        }
    }

    fn minimum() -> Self {
        Self {
            ..CreateProfile::baseline()
        }
    }
}

impl CreateProfile for vk::PhysicalDeviceVulkan12Properties {
    fn baseline() -> Self {
        Self {
            conformance_version: Default::default(),
            denorm_behavior_independence: vk::ShaderFloatControlsIndependence::NONE,
            rounding_mode_independence: vk::ShaderFloatControlsIndependence::NONE,
            shader_signed_zero_inf_nan_preserve_float16: vk::FALSE,
            shader_signed_zero_inf_nan_preserve_float32: vk::FALSE,
            shader_signed_zero_inf_nan_preserve_float64: vk::FALSE,
            shader_denorm_preserve_float16: vk::FALSE,
            shader_denorm_preserve_float32: vk::FALSE,
            shader_denorm_preserve_float64: vk::FALSE,
            shader_denorm_flush_to_zero_float16: vk::FALSE,
            shader_denorm_flush_to_zero_float32: vk::FALSE,
            shader_denorm_flush_to_zero_float64: vk::FALSE,
            shader_rounding_mode_rte_float16: vk::FALSE,
            shader_rounding_mode_rte_float32: vk::FALSE,
            shader_rounding_mode_rte_float64: vk::FALSE,
            shader_rounding_mode_rtz_float16: vk::FALSE,
            shader_rounding_mode_rtz_float32: vk::FALSE,
            shader_rounding_mode_rtz_float64: vk::FALSE,
            max_update_after_bind_descriptors_in_all_pools: u32::MIN,
            shader_uniform_buffer_array_non_uniform_indexing_native: vk::FALSE,
            shader_sampled_image_array_non_uniform_indexing_native: vk::FALSE,
            shader_storage_buffer_array_non_uniform_indexing_native: vk::FALSE,
            shader_storage_image_array_non_uniform_indexing_native: vk::FALSE,
            shader_input_attachment_array_non_uniform_indexing_native: vk::FALSE,
            robust_buffer_access_update_after_bind: vk::FALSE,
            quad_divergent_implicit_lod: vk::FALSE,
            max_per_stage_descriptor_update_after_bind_samplers: u32::MIN,
            max_per_stage_descriptor_update_after_bind_uniform_buffers: u32::MIN,
            max_per_stage_descriptor_update_after_bind_storage_buffers: u32::MIN,
            max_per_stage_descriptor_update_after_bind_sampled_images: u32::MIN,
            max_per_stage_descriptor_update_after_bind_storage_images: u32::MIN,
            max_per_stage_descriptor_update_after_bind_input_attachments: u32::MIN,
            max_per_stage_update_after_bind_resources: u32::MIN,
            max_descriptor_set_update_after_bind_samplers: u32::MIN,
            max_descriptor_set_update_after_bind_uniform_buffers: u32::MIN,
            max_descriptor_set_update_after_bind_uniform_buffers_dynamic: u32::MIN,
            max_descriptor_set_update_after_bind_storage_buffers: u32::MIN,
            max_descriptor_set_update_after_bind_storage_buffers_dynamic: u32::MIN,
            max_descriptor_set_update_after_bind_sampled_images: u32::MIN,
            max_descriptor_set_update_after_bind_storage_images: u32::MIN,
            max_descriptor_set_update_after_bind_input_attachments: u32::MIN,
            supported_depth_resolve_modes: Default::default(),
            supported_stencil_resolve_modes: Default::default(),
            independent_resolve_none: vk::FALSE,
            independent_resolve: vk::FALSE,
            filter_minmax_single_component_formats: vk::FALSE,
            filter_minmax_image_component_mapping: vk::FALSE,
            max_timeline_semaphore_value_difference: u64::MIN,
            framebuffer_integer_color_sample_counts: Default::default(),
            ..Default::default()
        }
    }

    fn minimum() -> Self {
        Self {
            supported_depth_resolve_modes: vk::ResolveModeFlags::SAMPLE_ZERO,
            supported_stencil_resolve_modes: vk::ResolveModeFlags::SAMPLE_ZERO,
            independent_resolve_none: vk::TRUE,
            independent_resolve: vk::TRUE,
            max_per_stage_descriptor_update_after_bind_samplers: 1024,
            max_per_stage_descriptor_update_after_bind_uniform_buffers: 12,
            max_per_stage_descriptor_update_after_bind_storage_buffers: 500_000,
            max_per_stage_descriptor_update_after_bind_sampled_images: 500_000,
            max_per_stage_descriptor_update_after_bind_storage_images: 500_000,
            max_per_stage_descriptor_update_after_bind_input_attachments: 7,
            max_per_stage_update_after_bind_resources: 500_000,
            max_descriptor_set_update_after_bind_samplers: 1024,
            max_descriptor_set_update_after_bind_uniform_buffers: 72,
            max_descriptor_set_update_after_bind_uniform_buffers_dynamic: 8,
            max_descriptor_set_update_after_bind_storage_buffers: 500_000,
            max_descriptor_set_update_after_bind_storage_buffers_dynamic: 4,
            max_descriptor_set_update_after_bind_sampled_images: 500_000,
            max_descriptor_set_update_after_bind_storage_images: 500_000,
            max_descriptor_set_update_after_bind_input_attachments: 7,
            ..CreateProfile::baseline()
        }
    }
}

impl CreateProfile for vk::PhysicalDeviceFeatures {
    fn baseline() -> Self {
        Self::default()
    }

    fn minimum() -> Self {
        Self {
            // robust_buffer_access: vk::TRUE,
            full_draw_index_uint32: vk::TRUE,
            image_cube_array: vk::TRUE,
            independent_blend: vk::TRUE,
            sample_rate_shading: vk::TRUE,
            dual_src_blend: vk::TRUE,
            multi_draw_indirect: vk::TRUE,
            draw_indirect_first_instance: vk::TRUE,
            depth_clamp: vk::TRUE,
            depth_bias_clamp: vk::TRUE,
            fill_mode_non_solid: vk::TRUE,
            large_points: vk::TRUE,
            multi_viewport: vk::TRUE,
            sampler_anisotropy: vk::TRUE,
            texture_compression_bc: vk::TRUE,
            occlusion_query_precise: vk::TRUE,
            vertex_pipeline_stores_and_atomics: vk::TRUE,
            fragment_stores_and_atomics: vk::TRUE,
            // shader_tessellation_and_geometry_point_size: vk::TRUE,
            shader_image_gather_extended: vk::TRUE,
            shader_storage_image_extended_formats: vk::TRUE,
            // shader_storage_image_read_without_format: vk::TRUE,
            shader_storage_image_write_without_format: vk::TRUE,
            shader_uniform_buffer_array_dynamic_indexing: vk::TRUE,
            shader_sampled_image_array_dynamic_indexing: vk::TRUE,
            shader_storage_buffer_array_dynamic_indexing: vk::TRUE,
            shader_storage_image_array_dynamic_indexing: vk::TRUE,
            shader_clip_distance: vk::TRUE,
            inherited_queries: vk::TRUE,
            ..CreateProfile::baseline()
        }
    }
}

impl CreateProfile for vk::PhysicalDeviceVulkan11Features {
    fn baseline() -> Self {
        Self::default()
    }

    fn minimum() -> Self {
        Self {
            storage_buffer16_bit_access: vk::TRUE,
            uniform_and_storage_buffer16_bit_access: vk::TRUE,
            multiview: vk::TRUE,
            variable_pointers_storage_buffer: vk::TRUE,
            variable_pointers: vk::TRUE,
            // shader_draw_parameters: vk::TRUE,
            ..CreateProfile::baseline()
        }
    }
}

impl CreateProfile for vk::PhysicalDeviceVulkan12Features {
    fn baseline() -> Self {
        Self::default()
    }

    fn minimum() -> Self {
        Self {
            sampler_mirror_clamp_to_edge: vk::TRUE,
            buffer_device_address: vk::TRUE,
            // buffer_device_address_capture_replay: vk::TRUE,
            timeline_semaphore: vk::TRUE,
            descriptor_indexing: vk::TRUE,
            descriptor_binding_partially_bound: vk::TRUE,
            descriptor_binding_update_unused_while_pending: vk::TRUE,
            descriptor_binding_variable_descriptor_count: vk::TRUE,
            runtime_descriptor_array: vk::TRUE,
            scalar_block_layout: vk::TRUE,
            descriptor_binding_sampled_image_update_after_bind: vk::TRUE,
            descriptor_binding_storage_image_update_after_bind: vk::TRUE,
            descriptor_binding_storage_buffer_update_after_bind: vk::TRUE,
            descriptor_binding_uniform_texel_buffer_update_after_bind: vk::TRUE,
            descriptor_binding_storage_texel_buffer_update_after_bind: vk::TRUE,
            shader_sampled_image_array_non_uniform_indexing: vk::TRUE,
            // shader_storage_buffer_array_non_uniform_indexing: vk::TRUE,
            shader_storage_image_array_non_uniform_indexing: vk::TRUE,
            shader_storage_texel_buffer_array_dynamic_indexing: vk::TRUE,
            shader_uniform_texel_buffer_array_dynamic_indexing: vk::TRUE,
            shader_uniform_texel_buffer_array_non_uniform_indexing: vk::TRUE,
            shader_storage_texel_buffer_array_non_uniform_indexing: vk::TRUE,
            shader_int8: vk::TRUE,
            storage_buffer8_bit_access: vk::TRUE,
            uniform_and_storage_buffer8_bit_access: vk::TRUE,
            // shader_buffer_int64_atomics: vk::TRUE,
            // shader_shared_int64_atomics: vk::TRUE,
            uniform_buffer_standard_layout: vk::TRUE,
            // draw_indirect_count: vk::TRUE,
            // shader_float16: vk::TRUE,
            imageless_framebuffer: vk::TRUE,
            host_query_reset: vk::TRUE,
            ..CreateProfile::baseline()
        }
    }
}

impl CreateProfile for vk::PhysicalDeviceDynamicRenderingFeatures {
    fn baseline() -> Self {
        Self::default()
    }

    fn minimum() -> Self {
        Self {
            dynamic_rendering: vk::TRUE,
            ..CreateProfile::baseline()
        }
    }
}
