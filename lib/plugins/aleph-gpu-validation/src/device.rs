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

use crate::adapter::Adapter;
use crate::context::Context;
use interfaces::any::{AnyArc, AnyWeak};
use interfaces::gpu::*;
use std::num::NonZeroU32;

pub struct Device {
    pub(crate) _this: AnyWeak<Self>,
    pub(crate) _context: AnyArc<Context>,
    pub(crate) _adapter: AnyArc<Adapter>,
    pub(crate) inner: AnyArc<dyn IDevice>,
}

crate::validation_declare_interfaces!(Device, [IDevice]);

impl IDevice for Device {
    // ========================================================================================== //
    // ========================================================================================== //

    fn upgrade(&self) -> AnyArc<dyn IDevice> {
        AnyArc::map::<dyn IDevice, _>(self._this.upgrade().unwrap(), |v| v)
    }

    // ========================================================================================== //
    // ========================================================================================== //

    fn strong_count(&self) -> usize {
        self._this.strong_count()
    }

    // ========================================================================================== //
    // ========================================================================================== //

    fn weak_count(&self) -> usize {
        self._this.weak_count()
    }

    // ========================================================================================== //
    // ========================================================================================== //

    fn garbage_collect(&self) {
        self.inner.garbage_collect()
    }

    // ========================================================================================== //
    // ========================================================================================== //

    fn wait_idle(&self) {
        self.inner.wait_idle()
    }

    // ========================================================================================== //
    // ========================================================================================== //

    fn create_graphics_pipeline(
        &self,
        desc: &GraphicsPipelineDesc,
    ) -> Result<AnyArc<dyn IGraphicsPipeline>, GraphicsPipelineCreateError> {
        self.inner.create_graphics_pipeline(desc)
    }

    // ========================================================================================== //
    // ========================================================================================== //

    fn create_compute_pipeline(
        &self,
        desc: &ComputePipelineDesc,
    ) -> Result<AnyArc<dyn IComputePipeline>, ComputePipelineCreateError> {
        self.inner.create_compute_pipeline(desc)
    }

    // ========================================================================================== //
    // ========================================================================================== //

    fn create_shader(
        &self,
        options: &ShaderOptions,
    ) -> Result<AnyArc<dyn IShader>, ShaderCreateError> {
        self.inner.create_shader(options)
    }

    // ========================================================================================== //
    // ========================================================================================== //

    fn create_descriptor_set_layout(
        &self,
        desc: &DescriptorSetLayoutDesc,
    ) -> Result<AnyArc<dyn IDescriptorSetLayout>, DescriptorSetLayoutCreateError> {
        Self::validate_descriptor_set_layout(desc);
        self.inner.create_descriptor_set_layout(desc)
    }

    // ========================================================================================== //
    // ========================================================================================== //

    fn create_descriptor_pool(
        &self,
        layout: &dyn IDescriptorSetLayout,
        num_sets: u32,
    ) -> Result<Box<dyn IDescriptorPool>, DescriptorPoolCreateError> {
        self.inner.create_descriptor_pool(layout, num_sets)
    }

    // ========================================================================================== //
    // ========================================================================================== //

    fn create_pipeline_layout(
        &self,
        desc: &PipelineLayoutDesc,
    ) -> Result<AnyArc<dyn IPipelineLayout>, PipelineLayoutCreateError> {
        self.inner.create_pipeline_layout(desc)
    }

    // ========================================================================================== //
    // ========================================================================================== //

    fn create_buffer(&self, desc: &BufferDesc) -> Result<AnyArc<dyn IBuffer>, BufferCreateError> {
        self.inner.create_buffer(desc)
    }

    // ========================================================================================== //
    // ========================================================================================== //

    fn create_texture(
        &self,
        desc: &TextureDesc,
    ) -> Result<AnyArc<dyn ITexture>, TextureCreateError> {
        self.inner.create_texture(desc)
    }

    // ========================================================================================== //
    // ========================================================================================== //

    fn create_sampler(
        &self,
        desc: &SamplerDesc,
    ) -> Result<AnyArc<dyn ISampler>, SamplerCreateError> {
        self.inner.create_sampler(desc)
    }

    // ========================================================================================== //
    // ========================================================================================== //

    fn create_command_pool(&self) -> Result<AnyArc<dyn ICommandPool>, CommandPoolCreateError> {
        self.inner.create_command_pool()
    }

    // ========================================================================================== //
    // ========================================================================================== //

    fn get_queue(&self, queue_type: QueueType) -> Option<AnyArc<dyn IQueue>> {
        self.inner.get_queue()
    }

    // ========================================================================================== //
    // ========================================================================================== //

    unsafe fn update_descriptor_sets(&self, writes: &[DescriptorWriteDesc]) {
        writes.iter().for_each(Self::validate_descriptor_write);
        self.inner.update_descriptor_sets(writes)
    }

    // ========================================================================================== //
    // ========================================================================================== //

    fn get_backend_api(&self) -> BackendAPI {
        self.inner.get_backend_api()
    }
}

impl Device {
    // ========================================================================================== //
    // ========================================================================================== //

    pub fn validate_descriptor_set_layout(desc: &DescriptorSetLayoutDesc) {
        for binding in desc.items.iter() {
            if binding.binding_count.is_some() {
                unimplemented!("Currently descriptor arrays are unimplemented");
            }

            if matches!(
                binding.binding_type,
                DescriptorType::UniformBuffer
                    | DescriptorType::UniformTexelBuffer
                    | DescriptorType::Sampler
                    | DescriptorType::InputAttachment
            ) {
                debug_assert!(
                    !binding.allow_writes,
                    "DescriptorType ConstantBuffer or Sampler can't allow writes"
                )
            }
        }

        fn calculate_binding_range(v: &DescriptorSetLayoutBinding) -> (u32, u32) {
            let start = v.binding_num;
            let num = v.binding_count.map(NonZeroU32::get).unwrap_or(1);
            let end = start + num;
            (start, end)
        }

        desc.items.iter().enumerate().for_each(|(outer_i, outer)| {
            let (outer_start, outer_end) = calculate_binding_range(outer);

            desc.items.iter().enumerate().for_each(|(inner_i, inner)| {
                // Skip over outer_i so we don't check if the outer range intersects with itself
                if outer_i == inner_i {
                    return;
                }

                let (inner_start, inner_end) = calculate_binding_range(inner);

                let starts_inside_outer = inner_start >= outer_start && inner_start <= outer_end;
                let ends_inside_outer = inner_end >= outer_start && inner_end <= outer_end;

                debug_assert!(
                    !starts_inside_outer || !ends_inside_outer,
                    "It is invalid for two descriptor binding ranges to intersect"
                );
            })
        });
    }

    // ========================================================================================== //
    // ========================================================================================== //

    /// # Safety
    ///
    /// This function does not check if the pointer inside [DescriptorWriteDesc].set is valid, as
    /// it is unknowable in isolation. It is the caller's responsibility to ensure the descriptor
    /// set being written to is still live and valid, and the caller's responsibility to synchronize
    /// access.
    pub unsafe fn validate_descriptor_write(write: &DescriptorWriteDesc) {
        let set = DescriptorSet::ref_from_handle(&write.set);
        let layout = set._layout.deref();

        // Checks if the binding is actually present in the descriptor set.
        let info = if let Some(info) = layout.get_binding_info(write.binding) {
            info
        } else {
            panic!(
                "Trying to write to a descriptor binding '{}' not present in the set",
                write.binding
            )
        };

        // Check if the caller is trying to write into a static sampler binding, which is
        // categorically invalid.
        debug_assert_eq!(
            info.is_static_sampler, false,
            "Writing a descriptor into a static sampler binding is invalid."
        );

        // Check if the user is requesting to write the correct descriptor type. That is, the
        // 'descriptor_type' in the write description must match the type declared in the descriptor
        // set layout.
        let expected_binding_type = info.r#type;
        let actual_binding_type = write.descriptor_type;
        debug_assert_eq!(
            expected_binding_type, actual_binding_type,
            "It is invalid to write the incorrect descriptor type into a binding."
        );

        // Check if the user is trying to write more than 1 descriptor into a non-array binding.
        let binding_layout = info.layout;
        let is_array_binding = binding_layout.num_descriptors > 1; // TODO: this might not be correct
        let num_writes = write.writes.len();
        debug_assert!(
            !is_array_binding && num_writes <= 1,
            "It is invalid to write more than 1 descriptor into a non-array binding."
        );

        // Check if the user is trying to write outside of an array binding's range.
        let write_start = write.array_element as usize;
        let write_end = write_start + write.writes.len();
        let binding_start = 0;
        let binding_end = binding_layout.num_descriptors as usize;
        debug_assert!(
            write_start >= binding_start && write_start < binding_end,
            "It is invalid to write outside of an array binding's bounds."
        );
        debug_assert!(
            write_end > binding_start && write_end <= binding_end,
            "It is invalid to write outside of an array binding's bounds."
        );

        // Check that the declared descriptor type matches the DescriptorWrites variant provided.
        match write.descriptor_type {
            DescriptorType::Sampler => debug_assert!(
                matches!(write.writes, DescriptorWrites::Sampler(_)),
                "Invalid DescriptorWrites type for descriptor type '{:#?}'",
                write.descriptor_type
            ),
            DescriptorType::SampledImage => debug_assert!(
                matches!(write.writes, DescriptorWrites::Image(_)),
                "Invalid DescriptorWrites type for descriptor type '{:#?}'",
                write.descriptor_type
            ),
            DescriptorType::StorageImage => debug_assert!(
                matches!(write.writes, DescriptorWrites::Image(_)),
                "Invalid DescriptorWrites type for descriptor type '{:#?}'",
                write.descriptor_type
            ),
            DescriptorType::UniformTexelBuffer => debug_assert!(
                matches!(write.writes, DescriptorWrites::TexelBuffer(_)),
                "Invalid DescriptorWrites type for descriptor type '{:#?}'",
                write.descriptor_type
            ),
            DescriptorType::StorageTexelBuffer => debug_assert!(
                matches!(write.writes, DescriptorWrites::TexelBuffer(_)),
                "Invalid DescriptorWrites type for descriptor type '{:#?}'",
                write.descriptor_type
            ),
            DescriptorType::UniformBuffer => debug_assert!(
                matches!(write.writes, DescriptorWrites::Buffer(_)),
                "Invalid DescriptorWrites type' for descriptor type '{:#?}'",
                write.descriptor_type
            ),
            DescriptorType::StorageBuffer => debug_assert!(
                matches!(write.writes, DescriptorWrites::Buffer(_)),
                "Invalid DescriptorWrites type' for descriptor type '{:#?}'",
                write.descriptor_type
            ),
            DescriptorType::StructuredBuffer => debug_assert!(
                matches!(write.writes, DescriptorWrites::StructuredBuffer(_)),
                "Invalid DescriptorWrites type for descriptor type '{:#?}'",
                write.descriptor_type
            ),
            DescriptorType::InputAttachment => debug_assert!(
                matches!(write.writes, DescriptorWrites::InputAttachment(_)),
                "Invalid DescriptorWrites type for descriptor type '{:#?}'",
                write.descriptor_type
            ),
        }
    }
}

impl INamedObject for Device {
    fn set_name(&self, name: &str) {
        self.inner.set_name(name)
    }
}
