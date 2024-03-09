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

use std::ffi::{c_char, CStr};

use ash::vk;

use crate::internal::features::CheckMeetsMinimum;
use crate::internal::profile::CreateProfile;

#[derive(Clone, Default)]
#[rustfmt::skip]
pub struct DeviceInfo {
    pub extensions: Vec<vk::ExtensionProperties>,
    pub properties_10: vk::PhysicalDeviceProperties,
    pub id_properties: vk::PhysicalDeviceIDProperties,
    pub subgroup_properties: vk::PhysicalDeviceSubgroupProperties,
    pub point_clipping_properties: vk::PhysicalDevicePointClippingProperties,
    pub multiview_properties: vk::PhysicalDeviceMultiviewProperties,
    pub protected_memory_properties: vk::PhysicalDeviceProtectedMemoryProperties,
    pub maintenance_3_properties: vk::PhysicalDeviceMaintenance3Properties,
    pub descriptor_indexing_properties: vk::PhysicalDeviceDescriptorIndexingProperties,
    pub float_controls_properties: vk::PhysicalDeviceFloatControlsProperties,
    pub depth_stencil_resolve_properties: vk::PhysicalDeviceDepthStencilResolveProperties,
    pub timeline_semaphore_properties: vk::PhysicalDeviceTimelineSemaphoreProperties,
    pub sampler_filter_minmax_properties: vk::PhysicalDeviceSamplerFilterMinmaxProperties,
    pub driver_properties: vk::PhysicalDeviceDriverProperties,
    pub portability_properties: vk::PhysicalDevicePortabilitySubsetPropertiesKHR,
    pub features_10: vk::PhysicalDeviceFeatures,
    pub t_16bit_storage_features: vk::PhysicalDevice16BitStorageFeatures,
    pub multiview_features: vk::PhysicalDeviceMultiviewFeatures,
    pub variable_pointers_features: vk::PhysicalDeviceVariablePointersFeatures,
    pub protected_memory_features: vk::PhysicalDeviceProtectedMemoryFeatures,
    pub sampler_ycbcr_conversion_features: vk::PhysicalDeviceSamplerYcbcrConversionFeatures,
    pub shader_draw_parameters_features: vk::PhysicalDeviceShaderDrawParametersFeatures,
    pub descriptor_indexing_features: vk::PhysicalDeviceDescriptorIndexingFeatures,
    pub imageless_framebuffer_features: vk::PhysicalDeviceImagelessFramebufferFeaturesKHR,
    pub scalar_block_layout_features: vk::PhysicalDeviceScalarBlockLayoutFeatures,
    pub timeline_semaphore_features: vk::PhysicalDeviceTimelineSemaphoreFeatures,
    pub buffer_device_address_features: vk::PhysicalDeviceBufferDeviceAddressFeatures,
    pub uniform_buffer_standard_layout_features: vk::PhysicalDeviceUniformBufferStandardLayoutFeatures,
    pub t_8bit_storage_features: vk::PhysicalDevice8BitStorageFeatures,
    pub shader_float16int8features: vk::PhysicalDeviceShaderFloat16Int8Features,
    pub host_query_reset_features: vk::PhysicalDeviceHostQueryResetFeatures,
    pub shader_atomic_int_64_features: vk::PhysicalDeviceShaderAtomicInt64Features,
    pub vulkan_memory_model_features: vk::PhysicalDeviceVulkanMemoryModelFeatures,
    pub dynamic_rendering_features: vk::PhysicalDeviceDynamicRenderingFeaturesKHR,
    pub portability_features: vk::PhysicalDevicePortabilitySubsetFeaturesKHR,
    pub synchronization_2_features: vk::PhysicalDeviceSynchronization2FeaturesKHR,
}

impl DeviceInfo {
    pub fn minimum() -> Self {
        fn make_ext_prop(v: &'static str) -> vk::ExtensionProperties {
            assert!(
                v.len() < vk::MAX_EXTENSION_NAME_SIZE - 1,
                "Extension name must be shorter than 255"
            );

            // Zero initialized buffer. Max length of 'v' is 255 to leave one space for a null
            // terminator.
            let mut props = vk::ExtensionProperties {
                extension_name: [0; vk::MAX_EXTENSION_NAME_SIZE],
                spec_version: 0,
            };

            // Memcpy the source string into the destination buffer
            unsafe {
                assert_eq!(std::mem::size_of::<u8>(), std::mem::size_of::<c_char>());
                let src = v.as_ptr() as *const c_char;
                let dst = props.extension_name.as_mut_ptr() as *mut c_char;
                std::ptr::copy(src, dst, v.len());
            }

            props
        }
        Self {
            extensions: vec![
                make_ext_prop("VK_KHR_timeline_semaphore\0"),
                make_ext_prop("VK_EXT_descriptor_indexing\0"),
                make_ext_prop("VK_KHR_buffer_device_address\0"),
                make_ext_prop("VK_KHR_imageless_framebuffer\0"),
                make_ext_prop("VK_EXT_host_query_reset\0"),
                make_ext_prop("VK_KHR_uniform_buffer_standard_layout\0"),
                make_ext_prop("VK_EXT_scalar_block_layout\0"),
                // make_ext_prop("VK_KHR_draw_indirect_count\0"),
                make_ext_prop("VK_EXT_separate_stencil_usage\0"),
                make_ext_prop("VK_KHR_separate_depth_stencil_layouts\0"),
                make_ext_prop("VK_KHR_driver_properties\0"),
                make_ext_prop("VK_KHR_create_renderpass2\0"),
                make_ext_prop("VK_KHR_dynamic_rendering\0"),
                make_ext_prop("VK_KHR_image_format_list\0"),
                make_ext_prop("VK_KHR_sampler_mirror_clamp_to_edge\0"),
                // make_ext_prop("VK_EXT_sampler_filter_minmax\0"),
                make_ext_prop("VK_KHR_shader_float_controls\0"),
                make_ext_prop("VK_KHR_shader_subgroup_extended_types\0"),
                make_ext_prop("VK_KHR_depth_stencil_resolve\0"),
            ],
            properties_10: CreateProfile::minimum(),
            id_properties: CreateProfile::minimum(),
            subgroup_properties: CreateProfile::minimum(),
            point_clipping_properties: CreateProfile::minimum(),
            multiview_properties: CreateProfile::minimum(),
            protected_memory_properties: CreateProfile::minimum(),
            maintenance_3_properties: CreateProfile::minimum(),
            descriptor_indexing_properties: CreateProfile::minimum(),
            float_controls_properties: CreateProfile::minimum(),
            depth_stencil_resolve_properties: CreateProfile::minimum(),
            timeline_semaphore_properties: CreateProfile::minimum(),
            sampler_filter_minmax_properties: CreateProfile::minimum(),
            driver_properties: CreateProfile::minimum(),
            portability_properties: Default::default(),
            features_10: CreateProfile::minimum(),
            t_16bit_storage_features: CreateProfile::minimum(),
            multiview_features: CreateProfile::minimum(),
            variable_pointers_features: CreateProfile::minimum(),
            protected_memory_features: CreateProfile::minimum(),
            sampler_ycbcr_conversion_features: CreateProfile::minimum(),
            shader_draw_parameters_features: CreateProfile::minimum(),
            descriptor_indexing_features: CreateProfile::minimum(),
            imageless_framebuffer_features: CreateProfile::minimum(),
            scalar_block_layout_features: CreateProfile::minimum(),
            timeline_semaphore_features: CreateProfile::minimum(),
            buffer_device_address_features: CreateProfile::minimum(),
            uniform_buffer_standard_layout_features: CreateProfile::minimum(),
            t_8bit_storage_features: CreateProfile::minimum(),
            shader_float16int8features: CreateProfile::minimum(),
            host_query_reset_features: CreateProfile::minimum(),
            shader_atomic_int_64_features: CreateProfile::minimum(),
            vulkan_memory_model_features: CreateProfile::minimum(),
            dynamic_rendering_features: CreateProfile::minimum(),
            portability_features: Default::default(),
            synchronization_2_features: Default::default(),
        }
    }

    pub fn load(instance: &ash::Instance, physical_device: vk::PhysicalDevice) -> DeviceInfo {
        let extensions = unsafe {
            instance
                .enumerate_device_extension_properties(physical_device)
                .unwrap_or_default()
                .to_vec()
        };

        let mut out = DeviceInfo {
            extensions,
            ..Default::default()
        };
        let DeviceInfo {
            extensions,
            properties_10,
            id_properties,
            subgroup_properties,
            point_clipping_properties,
            multiview_properties,
            protected_memory_properties,
            maintenance_3_properties,
            descriptor_indexing_properties,
            float_controls_properties,
            depth_stencil_resolve_properties,
            timeline_semaphore_properties,
            sampler_filter_minmax_properties,
            driver_properties,
            portability_properties,
            features_10,
            t_16bit_storage_features,
            multiview_features,
            variable_pointers_features,
            protected_memory_features,
            sampler_ycbcr_conversion_features,
            shader_draw_parameters_features,
            descriptor_indexing_features,
            imageless_framebuffer_features,
            scalar_block_layout_features,
            timeline_semaphore_features,
            buffer_device_address_features,
            uniform_buffer_standard_layout_features,
            t_8bit_storage_features,
            shader_float16int8features,
            host_query_reset_features,
            shader_atomic_int_64_features,
            vulkan_memory_model_features,
            dynamic_rendering_features,
            portability_features,
            synchronization_2_features,
        } = &mut out;

        // Unconditionally required properties
        let mut properties = vk::PhysicalDeviceProperties2::builder()
            .push_next(id_properties)
            .push_next(subgroup_properties)
            .push_next(point_clipping_properties)
            .push_next(multiview_properties)
            .push_next(protected_memory_properties)
            .push_next(maintenance_3_properties)
            .push_next(descriptor_indexing_properties)
            .push_next(float_controls_properties)
            .push_next(depth_stencil_resolve_properties)
            .push_next(timeline_semaphore_properties)
            .push_next(sampler_filter_minmax_properties)
            .push_next(driver_properties);

        // Safety: we assume all the strings vulkan gives us are valid
        unsafe {
            // We load the portability subset properties if the extension is present
            if Self::list_contains_extension_cstr(extensions, vk::KhrPortabilitySubsetFn::name()) {
                properties = properties.push_next(portability_properties)
            }
        };

        *properties_10 = unsafe {
            instance.get_physical_device_properties2(physical_device, &mut properties);
            properties.properties
        };

        // Glue all the feature extension structs together into our monster instance
        let mut features = vk::PhysicalDeviceFeatures2::builder()
            .push_next(t_16bit_storage_features)
            .push_next(multiview_features)
            .push_next(variable_pointers_features)
            .push_next(protected_memory_features)
            .push_next(sampler_ycbcr_conversion_features)
            .push_next(shader_draw_parameters_features)
            .push_next(descriptor_indexing_features)
            .push_next(imageless_framebuffer_features)
            .push_next(scalar_block_layout_features)
            .push_next(timeline_semaphore_features)
            .push_next(buffer_device_address_features)
            .push_next(uniform_buffer_standard_layout_features)
            .push_next(host_query_reset_features);

        unsafe {
            if Self::list_contains_extension_cstr(extensions, vk::KhrVulkanMemoryModelFn::name()) {
                features = features.push_next(vulkan_memory_model_features)
            }
        };
        unsafe {
            if Self::list_contains_extension_cstr(extensions, vk::Khr8bitStorageFn::name()) {
                features = features.push_next(t_8bit_storage_features)
            }
        };
        unsafe {
            if Self::list_contains_extension_cstr(extensions, vk::KhrShaderFloat16Int8Fn::name()) {
                features = features.push_next(shader_float16int8features)
            }
        };
        unsafe {
            if Self::list_contains_extension_cstr(extensions, vk::KhrShaderAtomicInt64Fn::name()) {
                features = features.push_next(shader_atomic_int_64_features)
            }
        };
        unsafe {
            if Self::list_contains_extension_cstr(extensions, vk::KhrDynamicRenderingFn::name()) {
                features = features.push_next(dynamic_rendering_features)
            }
        };
        unsafe {
            if Self::list_contains_extension_cstr(extensions, vk::KhrPortabilitySubsetFn::name()) {
                features = features.push_next(portability_features)
            }
        };
        unsafe {
            if Self::list_contains_extension_cstr(extensions, vk::KhrSynchronization2Fn::name()) {
                features = features.push_next(synchronization_2_features)
            }
        };

        *features_10 = unsafe {
            instance.get_physical_device_features2(physical_device, &mut features);
            features.features
        };

        // Null the p_next chain pointers to avoid leaving the dangling references. They can't be
        // *used* without unsafe but better be careful.
        out.null_p_next_ptrs();

        out
    }

    fn null_p_next_ptrs(&mut self) {
        self.id_properties.p_next = std::ptr::null_mut();
        self.subgroup_properties.p_next = std::ptr::null_mut();
        self.point_clipping_properties.p_next = std::ptr::null_mut();
        self.multiview_properties.p_next = std::ptr::null_mut();
        self.protected_memory_properties.p_next = std::ptr::null_mut();
        self.maintenance_3_properties.p_next = std::ptr::null_mut();
        self.descriptor_indexing_properties.p_next = std::ptr::null_mut();
        self.float_controls_properties.p_next = std::ptr::null_mut();
        self.depth_stencil_resolve_properties.p_next = std::ptr::null_mut();
        self.timeline_semaphore_properties.p_next = std::ptr::null_mut();
        self.sampler_filter_minmax_properties.p_next = std::ptr::null_mut();
        self.driver_properties.p_next = std::ptr::null_mut();
        self.portability_properties.p_next = std::ptr::null_mut();
        self.t_16bit_storage_features.p_next = std::ptr::null_mut();
        self.multiview_features.p_next = std::ptr::null_mut();
        self.variable_pointers_features.p_next = std::ptr::null_mut();
        self.protected_memory_features.p_next = std::ptr::null_mut();
        self.sampler_ycbcr_conversion_features.p_next = std::ptr::null_mut();
        self.shader_draw_parameters_features.p_next = std::ptr::null_mut();
        self.descriptor_indexing_features.p_next = std::ptr::null_mut();
        self.imageless_framebuffer_features.p_next = std::ptr::null_mut();
        self.scalar_block_layout_features.p_next = std::ptr::null_mut();
        self.timeline_semaphore_features.p_next = std::ptr::null_mut();
        self.buffer_device_address_features.p_next = std::ptr::null_mut();
        self.uniform_buffer_standard_layout_features.p_next = std::ptr::null_mut();
        self.t_8bit_storage_features.p_next = std::ptr::null_mut();
        self.shader_float16int8features.p_next = std::ptr::null_mut();
        self.host_query_reset_features.p_next = std::ptr::null_mut();
        self.shader_atomic_int_64_features.p_next = std::ptr::null_mut();
        self.vulkan_memory_model_features.p_next = std::ptr::null_mut();
        self.dynamic_rendering_features.p_next = std::ptr::null_mut();
        self.portability_features.p_next = std::ptr::null_mut();
        self.synchronization_2_features.p_next = std::ptr::null_mut();
    }
}

impl DeviceInfo {
    #[allow(unused)]
    pub fn supports_extension_ptr(&self, wanted: *const c_char) -> bool {
        unsafe { Self::list_contains_extension_ptr(&self.extensions, wanted) }
    }

    #[allow(unused)]
    pub fn supports_extension_cstr(&self, wanted: &CStr) -> bool {
        unsafe { Self::list_contains_extension_cstr(&self.extensions, wanted) }
    }

    #[allow(unused)]
    pub fn supports_extension(&self, wanted: &str) -> bool {
        unsafe { Self::list_contains_extension(&self.extensions, wanted) }
    }

    #[rustfmt::skip]
    pub fn meets_minimum_requirements(&self) -> Option<()> {
        let DeviceInfo { extensions: minimum_extensions, .. } = Self::minimum();
        let mut is_missing_required_extensions = false;
        for required in minimum_extensions {
            if !self.supports_extension_ptr(required.extension_name.as_ptr()) {
                let v = unsafe {
                    CStr::from_ptr(required.extension_name.as_ptr()).to_str().unwrap()
                };
                log::error!("Device missing required extension: \"{v}\"");
                is_missing_required_extensions = true;
            }
        }
        if is_missing_required_extensions {
            return None;
        }

        self.properties_10.meets_minimum()?;
        self.id_properties.meets_minimum()?;
        self.subgroup_properties.meets_minimum()?;
        self.point_clipping_properties.meets_minimum()?;
        self.multiview_properties.meets_minimum()?;
        self.protected_memory_properties.meets_minimum()?;
        self.maintenance_3_properties.meets_minimum()?;
        self.descriptor_indexing_properties.meets_minimum()?;
        self.float_controls_properties.meets_minimum()?;
        self.depth_stencil_resolve_properties.meets_minimum()?;
        self.timeline_semaphore_properties.meets_minimum()?;
        self.sampler_filter_minmax_properties.meets_minimum()?;
        self.driver_properties.meets_minimum()?;
        self.portability_properties.meets_minimum()?;
        self.features_10.meets_minimum()?;
        self.t_16bit_storage_features.meets_minimum()?;
        self.multiview_features.meets_minimum()?;
        self.variable_pointers_features.meets_minimum()?;
        self.protected_memory_features.meets_minimum()?;
        self.sampler_ycbcr_conversion_features.meets_minimum()?;
        self.shader_draw_parameters_features.meets_minimum()?;
        self.descriptor_indexing_features.meets_minimum()?;
        self.imageless_framebuffer_features.meets_minimum()?;
        self.scalar_block_layout_features.meets_minimum()?;
        self.timeline_semaphore_features.meets_minimum()?;
        self.buffer_device_address_features.meets_minimum()?;
        self.uniform_buffer_standard_layout_features.meets_minimum()?;
        self.t_8bit_storage_features.meets_minimum()?;
        self.shader_float16int8features.meets_minimum()?;
        self.host_query_reset_features.meets_minimum()?;
        self.shader_atomic_int_64_features.meets_minimum()?;
        self.vulkan_memory_model_features.meets_minimum()?;
        self.dynamic_rendering_features.meets_minimum()?;
        self.portability_features.meets_minimum()?;
        self.synchronization_2_features.meets_minimum()?;
        Some(())
    }

    unsafe fn list_contains_extension_ptr(
        extensions: &[vk::ExtensionProperties],
        wanted: *const c_char,
    ) -> bool {
        Self::list_contains_extension_cstr(extensions, CStr::from_ptr(wanted))
    }

    unsafe fn list_contains_extension_cstr(
        extensions: &[vk::ExtensionProperties],
        wanted: &CStr,
    ) -> bool {
        Self::list_contains_extension(extensions, wanted.to_str().unwrap_unchecked())
    }

    unsafe fn list_contains_extension(
        extensions: &[vk::ExtensionProperties],
        wanted: &str,
    ) -> bool {
        extensions
            .iter()
            .map(|v| {
                CStr::from_ptr(v.extension_name.as_ptr())
                    .to_str()
                    .unwrap_unchecked()
            })
            .any(|v| v == wanted)
    }
}

unsafe impl Send for DeviceInfo {}
unsafe impl Sync for DeviceInfo {}
