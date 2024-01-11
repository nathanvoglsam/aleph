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

use crate::frame_graph_builder::BufferImportDesc;
use crate::frame_graph_builder::TextureImportDesc;
use crate::FrameGraphResources;
use crate::ImportBundle;
use crate::{FrameGraph, ResourceMut, ResourceRef, ResourceRegistry};
use aleph_any::{declare_interfaces, AnyArc, AnyWeak};
use aleph_pin_board::PinBoard;
use aleph_rhi_api::*;
use std::any::TypeId;
use std::ptr::NonNull;

pub struct MockDevice {
    pub(crate) this: AnyWeak<Self>,
}

impl MockDevice {
    pub fn new() -> AnyArc<dyn IDevice> {
        let device = AnyArc::new_cyclic(move |v| MockDevice { this: v.clone() });
        AnyArc::map::<dyn IDevice, _>(device, |v| v)
    }
}

declare_interfaces!(MockDevice, [IDevice]);

impl IGetPlatformInterface for MockDevice {
    unsafe fn __query_platform_interface(&self, _target: TypeId, _out: *mut ()) -> Option<()> {
        None
    }
}

impl IDevice for MockDevice {
    fn upgrade(&self) -> AnyArc<dyn IDevice> {
        AnyArc::map::<dyn IDevice, _>(self.this.upgrade().unwrap(), |v| v)
    }

    fn strong_count(&self) -> usize {
        self.this.strong_count()
    }

    fn weak_count(&self) -> usize {
        self.this.weak_count()
    }

    fn garbage_collect(&self) {
        todo!()
    }

    fn wait_idle(&self) {
        todo!()
    }

    fn create_graphics_pipeline(
        &self,
        _desc: &GraphicsPipelineDesc,
    ) -> Result<AnyArc<dyn IGraphicsPipeline>, GraphicsPipelineCreateError> {
        todo!()
    }

    fn create_compute_pipeline(
        &self,
        _desc: &ComputePipelineDesc,
    ) -> Result<AnyArc<dyn IComputePipeline>, ComputePipelineCreateError> {
        todo!()
    }

    fn create_shader(
        &self,
        _options: &ShaderOptions,
    ) -> Result<AnyArc<dyn IShader>, ShaderCreateError> {
        todo!()
    }

    fn create_descriptor_set_layout(
        &self,
        _desc: &DescriptorSetLayoutDesc,
    ) -> Result<AnyArc<dyn IDescriptorSetLayout>, DescriptorSetLayoutCreateError> {
        todo!()
    }

    fn create_descriptor_pool(
        &self,
        _desc: &DescriptorPoolDesc,
    ) -> Result<Box<dyn IDescriptorPool>, DescriptorPoolCreateError> {
        todo!()
    }

    fn create_pipeline_layout(
        &self,
        _desc: &PipelineLayoutDesc,
    ) -> Result<AnyArc<dyn IPipelineLayout>, PipelineLayoutCreateError> {
        todo!()
    }

    fn create_buffer(&self, desc: &BufferDesc) -> Result<AnyArc<dyn IBuffer>, BufferCreateError> {
        Ok(MockBuffer::new(desc))
    }

    fn create_texture(
        &self,
        desc: &TextureDesc,
    ) -> Result<AnyArc<dyn ITexture>, TextureCreateError> {
        Ok(MockTexture::new(desc))
    }

    fn create_sampler(
        &self,
        _desc: &SamplerDesc,
    ) -> Result<AnyArc<dyn ISampler>, SamplerCreateError> {
        todo!()
    }

    fn create_command_list(
        &self,
        _desc: &CommandListDesc,
    ) -> Result<Box<dyn ICommandList>, CommandListCreateError> {
        todo!()
    }

    fn get_queue(&self, _queue_type: QueueType) -> Option<AnyArc<dyn IQueue>> {
        todo!()
    }

    unsafe fn update_descriptor_sets(&self, _writes: &[DescriptorWriteDesc]) {
        todo!()
    }

    fn create_fence(&self, _signalled: bool) -> Result<AnyArc<dyn IFence>, FenceCreateError> {
        todo!()
    }

    fn create_semaphore(&self) -> Result<AnyArc<dyn ISemaphore>, SemaphoreCreateError> {
        todo!()
    }

    fn wait_fences(
        &self,
        _fences: &[&dyn IFence],
        _wait_all: bool,
        _timeout: u32,
    ) -> FenceWaitResult {
        todo!()
    }

    fn poll_fence(&self, _fence: &dyn IFence) -> bool {
        todo!()
    }

    fn reset_fences(&self, _fences: &[&dyn IFence]) {
        todo!()
    }

    fn get_backend_api(&self) -> BackendAPI {
        todo!()
    }
}

pub struct MockBuffer {
    pub(crate) this: AnyWeak<Self>,
    pub(crate) desc: BufferDesc<'static>,
    pub(crate) name: Option<String>,
}

impl MockBuffer {
    pub fn new(desc: &BufferDesc) -> AnyArc<dyn IBuffer> {
        let name = desc.name.map(str::to_string);
        let desc = desc.clone().strip_name();

        let buffer = AnyArc::new_cyclic(move |v| MockBuffer {
            this: v.clone(),
            desc,
            name,
        });
        AnyArc::map::<dyn IBuffer, _>(buffer, |v| v)
    }
}

declare_interfaces!(MockBuffer, [IBuffer]);

impl IGetPlatformInterface for MockBuffer {
    unsafe fn __query_platform_interface(&self, _target: TypeId, _out: *mut ()) -> Option<()> {
        None
    }
}

impl IBuffer for MockBuffer {
    fn upgrade(&self) -> AnyArc<dyn IBuffer> {
        AnyArc::map::<dyn IBuffer, _>(self.this.upgrade().unwrap(), |v| v)
    }

    fn strong_count(&self) -> usize {
        self.this.strong_count()
    }

    fn weak_count(&self) -> usize {
        self.this.weak_count()
    }

    fn desc(&self) -> BufferDesc {
        let mut desc = self.desc.clone();
        desc.name = self.name.as_deref();
        desc
    }

    fn map(&self) -> Result<NonNull<u8>, ResourceMapError> {
        unimplemented!()
    }

    fn unmap(&self) {
        unimplemented!()
    }

    fn flush_range(&self, _offset: u64, _len: u64) {
        unimplemented!()
    }

    fn invalidate_range(&self, _offset: u64, _len: u64) {
        unimplemented!()
    }
}

pub struct MockTexture {
    pub(crate) this: AnyWeak<Self>,
    pub(crate) desc: TextureDesc<'static>,
    pub(crate) name: Option<String>,
}

impl MockTexture {
    pub fn new(desc: &TextureDesc) -> AnyArc<dyn ITexture> {
        let name = desc.name.map(str::to_string);
        let desc = desc.clone().strip_name();

        let texture = AnyArc::new_cyclic(move |v| MockTexture {
            this: v.clone(),
            desc,
            name,
        });
        AnyArc::map::<dyn ITexture, _>(texture, |v| v)
    }
}

declare_interfaces!(MockTexture, [ITexture]);

impl IGetPlatformInterface for MockTexture {
    unsafe fn __query_platform_interface(&self, _target: TypeId, _out: *mut ()) -> Option<()> {
        None
    }
}

impl ITexture for MockTexture {
    fn upgrade(&self) -> AnyArc<dyn ITexture> {
        AnyArc::map::<dyn ITexture, _>(self.this.upgrade().unwrap(), |v| v)
    }

    fn strong_count(&self) -> usize {
        self.this.strong_count()
    }

    fn weak_count(&self) -> usize {
        self.this.weak_count()
    }

    fn desc(&self) -> TextureDesc {
        let mut desc = self.desc.clone();
        desc.name = self.name.as_deref();
        desc
    }

    fn get_view(&self, _desc: &ImageViewDesc) -> Result<ImageView, ()> {
        unimplemented!()
    }

    fn get_rtv(&self, _desc: &ImageViewDesc) -> Result<ImageView, ()> {
        unimplemented!()
    }

    fn get_dsv(&self, _desc: &ImageViewDesc) -> Result<ImageView, ()> {
        unimplemented!()
    }
}

#[test]
pub fn test_builder() {
    #[derive(Default)]
    struct TestPassData {
        value: u32,
        resource: Option<ResourceMut>,
    }
    #[derive(Default)]
    struct TestPassData2 {
        value: i16,
        resource: Option<ResourceRef>,
    }

    let device = MockDevice::new();

    let mut out_create = None;
    let mut out_write = None;
    let mut out_read = None;

    let mut builder = FrameGraph::builder();

    builder.add_pass(
        "test-pass-0",
        |data: &mut TestPassData, resources: &mut ResourceRegistry| {
            // Payload init
            data.value = 54321;

            // Create a transient resource and send it out of the setup closure
            data.resource = Some(resources.create_buffer(
                &BufferDesc {
                    size: 256,
                    name: Some("test-pass-0-transient-resource"),
                    ..Default::default()
                },
                BarrierSync::COMPUTE_SHADING,
                ResourceUsageFlags::UNORDERED_ACCESS,
            ));
            out_create = data.resource;
        },
        |data: &TestPassData, _resources: &FrameGraphResources| {
            // Verify we got the right payload
            assert_eq!(data.value, 54321);
        },
    );

    builder.add_pass(
        "test-pass-1",
        |data: &mut TestPassData, resources: &mut ResourceRegistry| {
            // Payload init
            data.value = 1234;

            // Write the transient resource created in the previous pass and send it out of the
            // setup closure
            data.resource = Some(resources.write_buffer(
                out_create.unwrap(),
                BarrierSync::COMPUTE_SHADING,
                ResourceUsageFlags::UNORDERED_ACCESS,
            ));
            out_write = data.resource;
        },
        |data: &TestPassData, _resources: &FrameGraphResources| {
            // Verify we got the right payload
            assert_eq!(data.value, 1234);
        },
    );

    builder.add_pass(
        "test-pass-2",
        |data: &mut TestPassData2, resources: &mut ResourceRegistry| {
            // Payload init
            data.value = -432;

            // Read the transient resource based on the write rename created from the previous
            // pass and send it out of the setup closure
            data.resource = Some(resources.read_buffer(
                out_write.unwrap(),
                BarrierSync::PIXEL_SHADING,
                ResourceUsageFlags::CONSTANT_BUFFER,
            ));
            out_read = data.resource;
        },
        |data: &TestPassData2, _resources: &FrameGraphResources| {
            // Verify we got the right payload
            assert_eq!(data.value, -432);
        },
    );

    let mut graph = builder.build();

    let import_bundle = ImportBundle::default();
    let transient_bundle = graph.allocate_transient_resource_bundle(device.as_ref());
    unsafe {
        graph.execute(&transient_bundle, &import_bundle);
    }
}

#[test]
pub fn test_handle_equality() {
    let device = MockDevice::new();
    let mock_buffer = device
        .create_buffer(&BufferDesc {
            size: 512,
            cpu_access: CpuAccessMode::None,
            usage: ResourceUsageFlags::UNORDERED_ACCESS | ResourceUsageFlags::CONSTANT_BUFFER,
            name: Some("imported-mock-resource"),
        })
        .unwrap();
    let mock_desc = mock_buffer.desc();

    let mut out_create = None;
    let mut out_read_import = None;
    let mut out_write_import = None;
    let mut out_write_transient = None;
    let mut out_read_transient = None;

    let mut builder = FrameGraph::builder();

    let mut imported_resource = None;
    builder.add_pass(
        "test-pass-0",
        |_data: &mut (), resources: &mut ResourceRegistry| {
            let r = resources.import_buffer(
                &BufferImportDesc {
                    desc: &mock_desc,
                    before_sync: BarrierSync::COMPUTE_SHADING,
                    before_access: BarrierAccess::SHADER_WRITE,
                    after_sync: BarrierSync::COPY,
                    after_access: BarrierAccess::COPY_READ,
                },
                BarrierSync::NONE,
                ResourceUsageFlags::NONE,
            );
            imported_resource = Some(r);
        },
        |_data: &(), _resources: &FrameGraphResources| {},
    );
    let imported_resource = imported_resource.unwrap();

    builder.add_pass(
        "test-pass-1",
        |_data: &mut (), resources: &mut ResourceRegistry| {
            out_read_import = Some(resources.read_buffer(
                imported_resource,
                BarrierSync::PIXEL_SHADING,
                ResourceUsageFlags::CONSTANT_BUFFER,
            ));
            out_create = Some(resources.create_buffer(
                &BufferDesc {
                    size: 256,
                    name: Some("test-pass-1-transient-resource"),
                    ..Default::default()
                },
                BarrierSync::COMPUTE_SHADING,
                ResourceUsageFlags::UNORDERED_ACCESS,
            ));
        },
        |_data: &(), _resources: &FrameGraphResources| {},
    );

    builder.add_pass(
        "test-pass-2",
        |_data: &mut (), resources: &mut ResourceRegistry| {
            out_write_import = Some(resources.write_buffer(
                imported_resource,
                BarrierSync::COMPUTE_SHADING,
                ResourceUsageFlags::UNORDERED_ACCESS,
            ));
            out_write_transient = Some(resources.write_buffer(
                out_create.unwrap(),
                BarrierSync::COMPUTE_SHADING,
                ResourceUsageFlags::UNORDERED_ACCESS,
            ));
        },
        |_data: &(), _resources: &FrameGraphResources| {},
    );

    builder.add_pass(
        "test-pass-3",
        |_data: &mut (), resources: &mut ResourceRegistry| {
            out_read_transient = Some(resources.read_buffer(
                out_write_transient.unwrap(),
                BarrierSync::PIXEL_SHADING,
                ResourceUsageFlags::CONSTANT_BUFFER,
            ));
        },
        |_data: &(), _resources: &FrameGraphResources| {},
    );

    let out_create = out_create.unwrap();
    let out_read_import = out_read_import.unwrap();
    let out_write_import = out_write_import.unwrap();
    let out_write_transient = out_write_transient.unwrap();
    let out_read_transient = out_read_transient.unwrap();

    assert_ne!(imported_resource, out_write_import); // Write is a rename
    assert_ne!(imported_resource, out_write_transient); // Completely different resource
    assert_ne!(imported_resource, out_read_transient); // Completely different resource
    assert_eq!(imported_resource, out_read_import); // Read is _not_ a rename

    assert_ne!(out_create, out_write_transient); // Write is a rename
    assert_ne!(out_create, imported_resource); // Completely different resource
    assert_ne!(out_create, out_write_import); // Completely different resource

    assert_eq!(out_write_transient, out_read_transient); // Read of the transient write, not renamed

    let mut graph = builder.build();

    let mut import_bundle = ImportBundle::default();
    import_bundle.add_resource(imported_resource, &mock_buffer);
    let transient_bundle = graph.allocate_transient_resource_bundle(device.as_ref());
    unsafe {
        graph.execute(&transient_bundle, &import_bundle);
    }
}

#[test]
pub fn test_usage_collection() {
    let device = MockDevice::new();
    let mock_buffer = device
        .create_buffer(&BufferDesc {
            size: 512,
            cpu_access: CpuAccessMode::None,
            usage: ResourceUsageFlags::UNORDERED_ACCESS
                | ResourceUsageFlags::CONSTANT_BUFFER
                | ResourceUsageFlags::VERTEX_BUFFER,
            name: Some("imported-mock-resource"),
        })
        .unwrap();
    let mock_desc = mock_buffer.desc();

    let mut out_create = None;
    let mut out_write_import = None;
    let mut out_write_transient = None;

    let mut builder = FrameGraph::builder();

    let mut imported_resource = None;
    builder.add_pass(
        "test-pass-0",
        |_data: &mut (), resources: &mut ResourceRegistry| {
            let r = resources.import_buffer(
                &BufferImportDesc {
                    desc: &mock_desc,
                    before_sync: BarrierSync::COMPUTE_SHADING,
                    before_access: BarrierAccess::SHADER_WRITE,
                    after_sync: BarrierSync::COPY,
                    after_access: BarrierAccess::COPY_READ,
                },
                BarrierSync::NONE,
                ResourceUsageFlags::NONE,
            );
            imported_resource = Some(r);
        },
        |_data: &(), _resources: &FrameGraphResources| {},
    );
    let imported_resource = imported_resource.unwrap();

    builder.add_pass(
        "test-pass-1",
        |_data: &mut (), resources: &mut ResourceRegistry| {
            resources.read_buffer(
                imported_resource,
                BarrierSync::VERTEX_SHADING,
                ResourceUsageFlags::VERTEX_BUFFER,
            );
            out_create = Some(resources.create_buffer(
                &BufferDesc {
                    size: 256,
                    name: Some("test-pass-1-transient-resource"),
                    ..Default::default()
                },
                BarrierSync::VERTEX_SHADING,
                ResourceUsageFlags::INDEX_BUFFER,
            ));
        },
        |_data: &(), _resources: &FrameGraphResources| {},
    );

    builder.add_pass(
        "test-pass-2",
        |_data: &mut (), resources: &mut ResourceRegistry| {
            out_write_import = Some(resources.write_buffer(
                imported_resource,
                BarrierSync::COMPUTE_SHADING,
                ResourceUsageFlags::UNORDERED_ACCESS,
            ));
            out_write_transient = Some(resources.write_buffer(
                out_create.unwrap(),
                BarrierSync::COMPUTE_SHADING,
                ResourceUsageFlags::UNORDERED_ACCESS,
            ));
        },
        |_data: &(), _resources: &FrameGraphResources| {},
    );

    builder.add_pass(
        "test-pass-3",
        |_data: &mut (), resources: &mut ResourceRegistry| {
            resources.read_buffer(
                out_write_transient.unwrap(),
                BarrierSync::PIXEL_SHADING,
                ResourceUsageFlags::CONSTANT_BUFFER,
            );
        },
        |_data: &(), _resources: &FrameGraphResources| {},
    );

    let mut graph = builder.build();

    let out_create = out_create.unwrap();

    let imported_r = imported_resource.0.root_id();
    let imported_usage = graph.root_resources[imported_r as usize].total_access_flags;
    assert_eq!(
        imported_usage,
        ResourceUsageFlags::UNORDERED_ACCESS | ResourceUsageFlags::VERTEX_BUFFER
    );

    let out_create_r = out_create.0.root_id();
    let out_create_usage = graph.root_resources[out_create_r as usize].total_access_flags;
    assert_eq!(
        out_create_usage,
        ResourceUsageFlags::UNORDERED_ACCESS
            | ResourceUsageFlags::CONSTANT_BUFFER
            | ResourceUsageFlags::INDEX_BUFFER
    );

    let mut import_bundle = ImportBundle::default();
    import_bundle.add_resource(imported_resource, &mock_buffer);
    let transient_bundle = graph.allocate_transient_resource_bundle(device.as_ref());
    unsafe {
        graph.execute(&transient_bundle, &import_bundle);
    }
}

#[test]
pub fn test_usage_schedule() {
    let device = MockDevice::new();

    let mock_buffer = device
        .create_buffer(&BufferDesc {
            size: 512,
            cpu_access: CpuAccessMode::None,
            usage: ResourceUsageFlags::UNORDERED_ACCESS
                | ResourceUsageFlags::CONSTANT_BUFFER
                | ResourceUsageFlags::VERTEX_BUFFER,
            name: Some("imported-mock-buffer"),
        })
        .unwrap();
    let mock_buffer_desc = mock_buffer.desc();

    let mock_texture = device
        .create_texture(&TextureDesc {
            width: 1024,
            height: 1024,
            depth: 1,
            format: Format::Depth24Stencil8,
            dimension: TextureDimension::Texture2D,
            clear_value: None,
            array_size: 1,
            mip_levels: 1,
            sample_count: 1,
            sample_quality: 0,
            usage: ResourceUsageFlags::RENDER_TARGET
                | ResourceUsageFlags::UNORDERED_ACCESS
                | ResourceUsageFlags::SHADER_RESOURCE,
            name: Some("imported-mock-texture"),
        })
        .unwrap();
    let mock_texture_desc = mock_texture.desc();

    let pin_board = PinBoard::new();
    let mut builder = FrameGraph::builder();

    struct Pass0 {
        import: ResourceMut,
    }
    builder.add_pass(
        "test-pass-0",
        |_data: &mut (), resources: &mut ResourceRegistry| {
            let import = resources.import_buffer(
                &BufferImportDesc {
                    desc: &mock_buffer_desc,
                    before_sync: BarrierSync::COMPUTE_SHADING,
                    before_access: BarrierAccess::SHADER_WRITE,
                    after_sync: BarrierSync::COPY,
                    after_access: BarrierAccess::COPY_READ,
                },
                BarrierSync::NONE,
                ResourceUsageFlags::NONE,
            );
            pin_board.publish(Pass0 { import })
        },
        |_data: &(), _resources: &FrameGraphResources| {},
    );

    struct Pass1 {
        create: ResourceMut,
        import: ResourceMut,
    }
    builder.add_pass(
        "test-pass-1",
        |_data: &mut (), resources: &mut ResourceRegistry| {
            let import = pin_board.get::<Pass0>().unwrap().import;
            resources.read_buffer(
                import,
                BarrierSync::VERTEX_SHADING,
                ResourceUsageFlags::VERTEX_BUFFER,
            );
            let create = resources.create_buffer(
                &BufferDesc {
                    size: 256,
                    name: Some("test-pass-1-transient-resource"),
                    ..Default::default()
                },
                BarrierSync::VERTEX_SHADING,
                ResourceUsageFlags::INDEX_BUFFER,
            );
            let import = resources.import_texture(
                &TextureImportDesc {
                    desc: &mock_texture_desc,
                    before_sync: BarrierSync::ALL,
                    before_access: BarrierAccess::NONE,
                    before_layout: ImageLayout::Undefined,
                    after_sync: BarrierSync::DEPTH_STENCIL,
                    after_access: BarrierAccess::DEPTH_STENCIL_READ,
                    after_layout: ImageLayout::DepthStencilReadOnly,
                },
                BarrierSync::empty(),
                ResourceUsageFlags::RENDER_TARGET,
            );

            pin_board.publish(Pass1 { create, import });
        },
        |_data: &(), _resources: &FrameGraphResources| {},
    );

    struct Pass2 {
        // import_buffer_write: ResourceMut,
        import_texture_write: ResourceMut,
        transient_write: ResourceMut,
    }
    builder.add_pass(
        "test-pass-2",
        |_data: &mut (), resources: &mut ResourceRegistry| {
            let import_buffer = pin_board.get::<Pass0>().unwrap().import;
            let pass1 = pin_board.get::<Pass1>().unwrap();
            let create = pass1.create;
            let import_texture = pass1.import;

            let _import_buffer_write = resources.write_buffer(
                import_buffer,
                BarrierSync::COMPUTE_SHADING,
                ResourceUsageFlags::UNORDERED_ACCESS,
            );

            let import_texture_write = resources.write_texture(
                import_texture,
                BarrierSync::COMPUTE_SHADING,
                ResourceUsageFlags::UNORDERED_ACCESS,
            );

            let transient_write = resources.write_buffer(
                create,
                BarrierSync::COMPUTE_SHADING,
                ResourceUsageFlags::UNORDERED_ACCESS,
            );

            pin_board.publish(Pass2 {
                // import_buffer_write,
                import_texture_write,
                transient_write,
            });
        },
        |_data: &(), _resources: &FrameGraphResources| {},
    );

    builder.add_pass(
        "test-pass-3",
        |_data: &mut (), resources: &mut ResourceRegistry| {
            let transient = pin_board.get::<Pass2>().unwrap().transient_write;
            resources.read_buffer(
                transient,
                BarrierSync::PIXEL_SHADING,
                ResourceUsageFlags::CONSTANT_BUFFER,
            );
        },
        |_data: &(), _resources: &FrameGraphResources| {},
    );

    builder.add_pass(
        "test-pass-4",
        |_data: &mut (), resources: &mut ResourceRegistry| {
            let resource = pin_board.get::<Pass2>().unwrap().import_texture_write;
            resources.read_texture(
                resource,
                BarrierSync::PIXEL_SHADING,
                ResourceUsageFlags::SHADER_RESOURCE,
            );
        },
        |_data: &(), _resources: &FrameGraphResources| {},
    );

    builder.add_pass(
        "test-pass-5",
        |_data: &mut (), resources: &mut ResourceRegistry| {
            let resource = pin_board.get::<Pass2>().unwrap().import_texture_write;
            resources.read_texture(
                resource,
                BarrierSync::PIXEL_SHADING,
                ResourceUsageFlags::SHADER_RESOURCE,
            );
        },
        |_data: &(), _resources: &FrameGraphResources| {},
    );

    builder.add_pass(
        "test-pass-6",
        |_data: &mut (), resources: &mut ResourceRegistry| {
            let resource = pin_board.get::<Pass2>().unwrap().import_texture_write;
            resources.read_texture(
                resource,
                BarrierSync::PIXEL_SHADING,
                ResourceUsageFlags::SHADER_RESOURCE,
            );
        },
        |_data: &(), _resources: &FrameGraphResources| {},
    );

    builder.add_pass(
        "test-pass-7",
        |_data: &mut (), resources: &mut ResourceRegistry| {
            let resource = pin_board.get::<Pass2>().unwrap().import_texture_write;
            resources.read_texture(
                resource,
                BarrierSync::DEPTH_STENCIL,
                ResourceUsageFlags::RENDER_TARGET,
            );
        },
        |_data: &(), _resources: &FrameGraphResources| {},
    );

    builder.add_pass(
        "test-pass-8",
        |_data: &mut (), resources: &mut ResourceRegistry| {
            let resource = pin_board.get::<Pass2>().unwrap().import_texture_write;
            resources.read_texture(
                resource,
                BarrierSync::DEPTH_STENCIL,
                ResourceUsageFlags::RENDER_TARGET,
            );
        },
        |_data: &(), _resources: &FrameGraphResources| {},
    );

    // struct Pass8 {
    //     pub import_texture_write: ResourceMut,
    // }
    builder.add_pass(
        "test-pass-9",
        |_data: &mut (), resources: &mut ResourceRegistry| {
            let resource = pin_board.get::<Pass2>().unwrap().import_texture_write;
            let _import_texture_write = resources.write_texture(
                resource,
                BarrierSync::DEPTH_STENCIL,
                ResourceUsageFlags::RENDER_TARGET,
            );
            // pin_board.publish(Pass8 {
            //     import_texture_write,
            // });
        },
        |_data: &(), _resources: &FrameGraphResources| {},
    );

    let mut dot_text = Vec::<u8>::new();
    let mut graph = builder
        .build_with_graph_viz("TestGraph", &mut dot_text, &Default::default())
        .unwrap();
    graph
        .graph_viz_for_pass_order("PassOrder", &mut dot_text)
        .unwrap();

    std::fs::write("./graphviz.dot", dot_text).unwrap();

    let import_buffer = pin_board.get::<Pass0>().unwrap().import;
    let import_texture = pin_board.get::<Pass1>().unwrap().import;
    let mut import_bundle = ImportBundle::default();
    import_bundle.add_resource(import_buffer, &mock_buffer);
    import_bundle.add_resource(import_texture, &mock_texture);
    let transient_bundle = graph.allocate_transient_resource_bundle(device.as_ref());
    unsafe {
        graph.execute(&transient_bundle, &import_bundle);
    }
}
