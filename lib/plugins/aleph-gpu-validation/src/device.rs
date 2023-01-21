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

use crate::adapter::ValidationAdapter;
use crate::command_pool::ValidationCommandPool;
use crate::context::ValidationContext;
use crate::descriptor_pool::ValidationDescriptorPool;
use crate::descriptor_set_layout::{DescriptorBindingInfo, ValidationDescriptorSetLayout};
use crate::internal::descriptor_set::DescriptorSet;
use crate::sampler::ValidationSampler;
use crate::{
    ValidationBuffer, ValidationComputePipeline, ValidationGraphicsPipeline,
    ValidationPipelineLayout, ValidationShader, ValidationTexture,
};
use interfaces::any::{AnyArc, AnyWeak, QueryInterface};
use interfaces::gpu::*;
use std::collections::HashMap;
use std::num::NonZeroU32;
use std::ops::Deref;
use std::ptr::NonNull;
use std::sync::atomic::{AtomicU64, Ordering};

pub struct ValidationDevice {
    pub(crate) _this: AnyWeak<Self>,
    pub(crate) _context: AnyArc<ValidationContext>,
    pub(crate) _adapter: AnyArc<ValidationAdapter>,
    pub(crate) inner: AnyArc<dyn IDevice>,
    pub(crate) pool_counter: AtomicU64,
}

crate::validation_declare_interfaces!(ValidationDevice, [IDevice]);

impl IDevice for ValidationDevice {
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
        let shader_stages: Vec<&dyn IShader> = desc
            .shader_stages
            .iter()
            .copied()
            .map(|v| {
                v.query_interface::<ValidationShader>()
                    .expect("Unknown IShader implementation")
                    .inner
                    .as_ref()
            })
            .collect();

        let pipeline_layout = desc
            .pipeline_layout
            .query_interface::<ValidationPipelineLayout>()
            .expect("Unknown IGraphicsPipeline implementation");

        let new_desc = GraphicsPipelineDesc {
            shader_stages: &shader_stages,
            pipeline_layout: pipeline_layout.inner.as_ref(),
            vertex_layout: &desc.vertex_layout,
            input_assembly_state: &desc.input_assembly_state,
            rasterizer_state: &desc.rasterizer_state,
            depth_stencil_state: &desc.depth_stencil_state,
            blend_state: &desc.blend_state,
            render_target_formats: &desc.render_target_formats,
            depth_stencil_format: desc.depth_stencil_format,
        };

        let inner = self.inner.create_graphics_pipeline(&new_desc)?;
        let pipeline = AnyArc::new_cyclic(move |v| ValidationGraphicsPipeline {
            _this: v.clone(),
            _device: self._this.upgrade().unwrap(),
            _pipeline_layout: pipeline_layout._this.upgrade().unwrap(),
            inner,
        });
        Ok(AnyArc::map::<dyn IGraphicsPipeline, _>(pipeline, |v| v))
    }

    // ========================================================================================== //
    // ========================================================================================== //

    fn create_compute_pipeline(
        &self,
        desc: &ComputePipelineDesc,
    ) -> Result<AnyArc<dyn IComputePipeline>, ComputePipelineCreateError> {
        let shader_module = desc
            .shader_module
            .query_interface::<ValidationShader>()
            .expect("Unknown IShader implementation");

        let pipeline_layout = desc
            .pipeline_layout
            .query_interface::<ValidationPipelineLayout>()
            .expect("Unknown IGraphicsPipeline implementation");

        let new_desc = ComputePipelineDesc {
            shader_module: shader_module.inner.as_ref(),
            pipeline_layout: pipeline_layout.inner.as_ref(),
        };

        let inner = self.inner.create_compute_pipeline(&new_desc)?;
        let pipeline = AnyArc::new_cyclic(move |v| ValidationComputePipeline {
            _this: v.clone(),
            _device: self._this.upgrade().unwrap(),
            _pipeline_layout: pipeline_layout._this.upgrade().unwrap(),
            inner,
        });
        Ok(AnyArc::map::<dyn IComputePipeline, _>(pipeline, |v| v))
    }

    // ========================================================================================== //
    // ========================================================================================== //

    fn create_shader(
        &self,
        options: &ShaderOptions,
    ) -> Result<AnyArc<dyn IShader>, ShaderCreateError> {
        let inner = self.inner.create_shader(options)?;
        let shader = AnyArc::new_cyclic(move |v| ValidationShader {
            _this: v.clone(),
            _device: self._this.upgrade().unwrap(),
            inner,
            shader_type: options.shader_type,
        });
        Ok(AnyArc::map::<dyn IShader, _>(shader, |v| v))
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
        let inner_layout = layout
            .query_interface::<ValidationDescriptorSetLayout>()
            .expect("Unknown IDescriptorSetLayout implementation")
            ._this
            .upgrade()
            .unwrap();

        let inner = self
            .inner
            .create_descriptor_pool(inner_layout.inner.as_ref(), num_sets)?;
        let pool = Box::new(ValidationDescriptorPool {
            _device: self._this.upgrade().unwrap(),
            _layout: inner_layout,
            inner,
            pool_id: self.pool_counter.fetch_add(1, Ordering::Relaxed),
            set_objects: Vec::with_capacity(num_sets as usize),
            free_list: Vec::with_capacity(128),
        });

        Ok(pool)
    }

    // ========================================================================================== //
    // ========================================================================================== //

    fn create_pipeline_layout(
        &self,
        desc: &PipelineLayoutDesc,
    ) -> Result<AnyArc<dyn IPipelineLayout>, PipelineLayoutCreateError> {
        // Unwrap the objects in 'set_layouts' into a new list so the layer below gets the correct
        // object implementations
        let set_layouts: Vec<&dyn IDescriptorSetLayout> = desc.set_layouts.iter().copied().map(|v| {
            let v = v.query_interface::<ValidationDescriptorSetLayout>().expect("Unknown IDescriptorSetLayout implementation");
            v.inner.as_ref()
        }).collect();
        let new_desc = PipelineLayoutDesc {
            set_layouts: &set_layouts,
            push_constant_blocks: &desc.push_constant_blocks,
        };

        let inner = self.inner.create_pipeline_layout(&new_desc)?;
        let layout = AnyArc::new_cyclic(move |v| ValidationPipelineLayout {
            _this: v.clone(),
            _device: self._this.upgrade().unwrap(),
            inner,
            push_constant_blocks: vec![],
        });
        Ok(AnyArc::map::<dyn IPipelineLayout, _>(layout, |v| v))
    }

    // ========================================================================================== //
    // ========================================================================================== //

    fn create_buffer(&self, desc: &BufferDesc) -> Result<AnyArc<dyn IBuffer>, BufferCreateError> {
        let inner = self.inner.create_buffer(desc)?;
        let layout = AnyArc::new_cyclic(move |v| ValidationBuffer {
            _this: v.clone(),
            _device: self._this.upgrade().unwrap(),
            inner,
            debug_mapped_tracker: Default::default(),
        });
        Ok(AnyArc::map::<dyn IBuffer, _>(layout, |v| v))
    }

    // ========================================================================================== //
    // ========================================================================================== //

    fn create_texture(
        &self,
        desc: &TextureDesc,
    ) -> Result<AnyArc<dyn ITexture>, TextureCreateError> {
        let inner = self.inner.create_texture(desc)?;
        let layout = AnyArc::new_cyclic(move |v| ValidationTexture {
            _this: v.clone(),
            _device: self._this.upgrade().unwrap(),
            inner,
        });
        Ok(AnyArc::map::<dyn ITexture, _>(layout, |v| v))
    }

    // ========================================================================================== //
    // ========================================================================================== //

    fn create_sampler(
        &self,
        desc: &SamplerDesc,
    ) -> Result<AnyArc<dyn ISampler>, SamplerCreateError> {
        let inner = self.inner.create_sampler(desc)?;
        let layout = AnyArc::new_cyclic(move |v| ValidationSampler {
            _this: v.clone(),
            _device: self._this.upgrade().unwrap(),
            inner,
        });
        Ok(AnyArc::map::<dyn ISampler, _>(layout, |v| v))
    }

    // ========================================================================================== //
    // ========================================================================================== //

    fn create_command_pool(&self) -> Result<AnyArc<dyn ICommandPool>, CommandPoolCreateError> {
        let inner = self.inner.create_command_pool()?;
        let layout = AnyArc::new_cyclic(move |v| ValidationCommandPool {
            _this: v.clone(),
            _device: self._this.upgrade().unwrap(),
            inner,
        });
        Ok(AnyArc::map::<dyn ICommandPool, _>(layout, |v| v))
    }

    // ========================================================================================== //
    // ========================================================================================== //

    fn get_queue(&self, queue_type: QueueType) -> Option<AnyArc<dyn IQueue>> {
        self.inner.get_queue(queue_type)
    }

    // ========================================================================================== //
    // ========================================================================================== //

    unsafe fn update_descriptor_sets(&self, writes: &[DescriptorWriteDesc]) {
        writes
            .iter()
            .for_each(|v| Self::validate_descriptor_write(v));
        self.inner.update_descriptor_sets(writes)
    }

    // ========================================================================================== //
    // ========================================================================================== //

    fn get_backend_api(&self) -> BackendAPI {
        self.inner.get_backend_api()
    }
}

impl ValidationDevice {
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
        let is_array_binding = info.descriptor_count.is_some();
        let num_writes = write.writes.len();
        debug_assert!(
            !is_array_binding && num_writes <= 1,
            "It is invalid to write more than 1 descriptor into a non-array binding."
        );

        // Check if the user is trying to write outside of an array binding's range.
        let write_start = write.array_element as usize;
        let write_end = write_start + write.writes.len();
        let binding_start = 0;
        let binding_end = info.descriptor_count.map(NonZeroU32::get).unwrap_or(1) as usize;
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

impl INamedObject for ValidationDevice {
    fn set_name(&self, name: &str) {
        self.inner.set_name(name)
    }
}
