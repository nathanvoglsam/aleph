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

use std::cell::Cell;

use aleph_any::{declare_interfaces, AnyArc, AnyWeak, QueryInterface};
use aleph_rhi_api::*;

use crate::NullDescriptorArena;
use crate::{
    NullAdapter, NullBuffer, NullCommandList, NullComputePipeline, NullContext, NullDescriptorPool,
    NullDescriptorSetLayout, NullFence, NullGraphicsPipeline, NullPipelineLayout, NullQueue,
    NullSampler, NullSemaphore, NullTexture,
};

pub struct NullDevice {
    pub(crate) _this: AnyWeak<Self>,
    pub(crate) _context: AnyArc<NullContext>,
    pub(crate) _adapter: AnyArc<NullAdapter>,
    pub(crate) general_queue: Option<AnyArc<NullQueue>>,
    pub(crate) compute_queue: Option<AnyArc<NullQueue>>,
    pub(crate) transfer_queue: Option<AnyArc<NullQueue>>,
}

declare_interfaces!(NullDevice, [IDevice]);

crate::impl_platform_interface_passthrough!(NullDevice);

impl IDevice for NullDevice {
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

    fn garbage_collect(&self) {}

    // ========================================================================================== //
    // ========================================================================================== //

    fn wait_idle(&self) {}

    // ========================================================================================== //
    // ========================================================================================== //

    fn create_graphics_pipeline(
        &self,
        desc: &GraphicsPipelineDesc,
    ) -> Result<AnyArc<dyn IGraphicsPipeline>, PipelineCreateError> {
        let pipeline_layout = desc
            .pipeline_layout
            .query_interface::<NullPipelineLayout>()
            .expect("Unknown IGraphicsPipeline implementation");

        let pipeline = AnyArc::new_cyclic(move |v| NullGraphicsPipeline {
            _this: v.clone(),
            _device: self._this.upgrade().unwrap(),
            _pipeline_layout: pipeline_layout._this.upgrade().unwrap(),
        });
        Ok(AnyArc::map::<dyn IGraphicsPipeline, _>(pipeline, |v| v))
    }

    // ========================================================================================== //
    // ========================================================================================== //

    fn create_compute_pipeline(
        &self,
        desc: &ComputePipelineDesc,
    ) -> Result<AnyArc<dyn IComputePipeline>, PipelineCreateError> {
        let pipeline_layout = desc
            .pipeline_layout
            .query_interface::<NullPipelineLayout>()
            .expect("Unknown IGraphicsPipeline implementation");

        let pipeline = AnyArc::new_cyclic(move |v| NullComputePipeline {
            _this: v.clone(),
            _device: self._this.upgrade().unwrap(),
            _pipeline_layout: pipeline_layout._this.upgrade().unwrap(),
        });
        Ok(AnyArc::map::<dyn IComputePipeline, _>(pipeline, |v| v))
    }

    // ========================================================================================== //
    // ========================================================================================== //

    fn create_descriptor_set_layout(
        &self,
        _desc: &DescriptorSetLayoutDesc,
    ) -> Result<AnyArc<dyn IDescriptorSetLayout>, DescriptorSetLayoutCreateError> {
        let layout = AnyArc::new_cyclic(move |v| NullDescriptorSetLayout {
            _this: v.clone(),
            _device: self._this.upgrade().unwrap(),
        });
        Ok(AnyArc::map::<dyn IDescriptorSetLayout, _>(layout, |v| v))
    }

    // ========================================================================================== //
    // ========================================================================================== //

    fn create_descriptor_pool(
        &self,
        desc: &DescriptorPoolDesc,
    ) -> Result<Box<dyn IDescriptorPool>, DescriptorPoolCreateError> {
        let inner_layout = desc
            .layout
            .query_interface::<NullDescriptorSetLayout>()
            .expect("Unknown IDescriptorSetLayout implementation")
            ._this
            .upgrade()
            .unwrap();

        let pool = Box::new(NullDescriptorPool {
            _device: self._this.upgrade().unwrap(),
            _layout: inner_layout,
            counter: 1,
        });

        Ok(pool)
    }

    // ========================================================================================== //
    // ========================================================================================== //

    fn create_descriptor_arena(
        &self,
        _desc: &DescriptorArenaDesc,
    ) -> Result<Box<dyn IDescriptorArena>, DescriptorPoolCreateError> {
        let pool = Box::new(NullDescriptorArena {
            _device: self._this.upgrade().unwrap(),
            counter: Cell::new(1),
        });

        Ok(pool)
    }

    // ========================================================================================== //
    // ========================================================================================== //

    fn create_pipeline_layout(
        &self,
        _desc: &PipelineLayoutDesc,
    ) -> Result<AnyArc<dyn IPipelineLayout>, PipelineLayoutCreateError> {
        let layout = AnyArc::new_cyclic(move |v| NullPipelineLayout {
            _this: v.clone(),
            _device: self._this.upgrade().unwrap(),
        });
        Ok(AnyArc::map::<dyn IPipelineLayout, _>(layout, |v| v))
    }

    // ========================================================================================== //
    // ========================================================================================== //

    fn create_buffer(&self, desc: &BufferDesc) -> Result<AnyArc<dyn IBuffer>, BufferCreateError> {
        let name = desc.name.map(String::from);
        let desc = desc.clone().strip_name();
        let layout = AnyArc::new_cyclic(move |v| NullBuffer {
            _this: v.clone(),
            _device: self._this.upgrade().unwrap(),
            desc,
            name,
        });
        Ok(AnyArc::map::<dyn IBuffer, _>(layout, |v| v))
    }

    // ========================================================================================== //
    // ========================================================================================== //

    fn create_texture(
        &self,
        desc: &TextureDesc,
    ) -> Result<AnyArc<dyn ITexture>, TextureCreateError> {
        assert!(
            ResourceUsageFlags::TEXTURE_USAGE_MASK.contains(desc.usage),
            "Attempted to create a texture with usage flags meant only for buffers!"
        );
        let name = desc.name.map(String::from);
        let desc = desc.clone().strip_name();
        let texture = AnyArc::new_cyclic(move |v| NullTexture {
            _this: v.clone(),
            _device: self._this.upgrade().unwrap(),
            desc,
            name,
        });
        Ok(AnyArc::map::<dyn ITexture, _>(texture, |v| v))
    }

    // ========================================================================================== //
    // ========================================================================================== //

    fn create_sampler(
        &self,
        desc: &SamplerDesc,
    ) -> Result<AnyArc<dyn ISampler>, SamplerCreateError> {
        let name = desc.name.map(String::from);
        let desc = desc.clone().strip_name();
        let sampler = AnyArc::new_cyclic(move |v| NullSampler {
            _this: v.clone(),
            _device: self._this.upgrade().unwrap(),
            desc,
            name,
        });
        Ok(AnyArc::map::<dyn ISampler, _>(sampler, |v| v))
    }

    // ========================================================================================== //
    // ========================================================================================== //

    fn create_command_list(
        &self,
        _desc: &CommandListDesc,
    ) -> Result<Box<dyn ICommandList>, CommandListCreateError> {
        let list = Box::new(NullCommandList {
            _device: self._this.upgrade().unwrap(),
        });
        Ok(list)
    }

    // ========================================================================================== //
    // ========================================================================================== //

    fn get_queue(&self, queue_type: QueueType) -> Option<AnyArc<dyn IQueue>> {
        match queue_type {
            QueueType::General => self
                .general_queue
                .clone()
                .map(|v| AnyArc::map::<dyn IQueue, _>(v, |v| v)),
            QueueType::Compute => self
                .compute_queue
                .clone()
                .map(|v| AnyArc::map::<dyn IQueue, _>(v, |v| v)),
            QueueType::Transfer => self
                .transfer_queue
                .clone()
                .map(|v| AnyArc::map::<dyn IQueue, _>(v, |v| v)),
        }
    }

    // ========================================================================================== //
    // ========================================================================================== //

    unsafe fn update_descriptor_sets(&self, _writes: &[DescriptorWriteDesc]) {}

    // ========================================================================================== //
    // ========================================================================================== //

    fn create_fence(&self, _signalled: bool) -> Result<AnyArc<dyn IFence>, FenceCreateError> {
        let fence = AnyArc::new_cyclic(move |v| NullFence {
            _this: v.clone(),
            _device: self._this.upgrade().unwrap(),
        });
        Ok(AnyArc::map::<dyn IFence, _>(fence, |v| v))
    }

    // ========================================================================================== //
    // ========================================================================================== //

    fn create_semaphore(&self) -> Result<AnyArc<dyn ISemaphore>, SemaphoreCreateError> {
        let fence = AnyArc::new_cyclic(move |v| NullSemaphore {
            _this: v.clone(),
            _device: self._this.upgrade().unwrap(),
        });
        Ok(AnyArc::map::<dyn ISemaphore, _>(fence, |v| v))
    }

    // ========================================================================================== //
    // ========================================================================================== //

    fn wait_fences(
        &self,
        _fences: &[&dyn IFence],
        _wait_all: bool,
        _timeout: u32,
    ) -> FenceWaitResult {
        FenceWaitResult::Complete
    }

    // ========================================================================================== //
    // ========================================================================================== //

    fn poll_fence(&self, _fence: &dyn IFence) -> bool {
        true
    }

    fn reset_fences(&self, _fences: &[&dyn IFence]) {}

    // ========================================================================================== //
    // ========================================================================================== //

    fn get_backend_api(&self) -> BackendAPI {
        BackendAPI::Null
    }
}
