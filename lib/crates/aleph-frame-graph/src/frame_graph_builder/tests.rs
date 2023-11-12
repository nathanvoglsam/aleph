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
use crate::ResourceAccessFlags;
use crate::{FrameGraph, ResourceMut, ResourceRef, ResourceRegistry};
use aleph_any::{declare_interfaces, AnyArc, AnyWeak};
use aleph_rhi_api::*;
use std::any::TypeId;
use std::ptr::NonNull;

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
                ResourceAccessFlags::UNORDERED_ACCESS,
            ));
            out_create = data.resource;
        },
        |data: &TestPassData| {
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
                ResourceAccessFlags::UNORDERED_ACCESS,
            ));
            out_write = data.resource;
        },
        |data: &TestPassData| {
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
                ResourceAccessFlags::CONSTANT_BUFFER,
            ));
            out_read = data.resource;
        },
        |data: &TestPassData2| {
            // Verify we got the right payload
            assert_eq!(data.value, -432);
        },
    );

    let mut graph = builder.build();

    unsafe {
        graph.execute();
    }
}

#[test]
pub fn test_handle_equality() {
    let mock_buffer = MockBuffer::new(&BufferDesc {
        size: 512,
        cpu_access: CpuAccessMode::None,
        usage: BufferUsageFlags::UNORDERED_ACCESS | BufferUsageFlags::CONSTANT_BUFFER,
        name: Some("imported-mock-resource"),
    });
    let mut out_create = None;
    let mut out_read_import = None;
    let mut out_write_import = None;
    let mut out_write_transient = None;
    let mut out_read_transient = None;

    let mut builder = FrameGraph::builder();

    let imported_resource = builder.import_buffer(&BufferImportDesc {
        resource: mock_buffer.as_ref(),
        before_sync: BarrierSync::COMPUTE_SHADING,
        before_access: BarrierAccess::SHADER_WRITE,
        after_sync: BarrierSync::COPY,
        after_access: BarrierAccess::COPY_READ,
    });

    builder.add_pass(
        "test-pass-0",
        |_data: &mut (), resources: &mut ResourceRegistry| {
            out_read_import = Some(resources.read_buffer(
                imported_resource,
                BarrierSync::PIXEL_SHADING,
                ResourceAccessFlags::CONSTANT_BUFFER,
            ));
            out_create = Some(resources.create_buffer(
                &BufferDesc {
                    size: 256,
                    name: Some("test-pass-0-transient-resource"),
                    ..Default::default()
                },
                BarrierSync::COMPUTE_SHADING,
                ResourceAccessFlags::UNORDERED_ACCESS,
            ));
        },
        |_data: &()| {},
    );

    builder.add_pass(
        "test-pass-1",
        |_data: &mut (), resources: &mut ResourceRegistry| {
            out_write_import = Some(resources.write_buffer(
                imported_resource,
                BarrierSync::COMPUTE_SHADING,
                ResourceAccessFlags::UNORDERED_ACCESS,
            ));
            out_write_transient = Some(resources.write_buffer(
                out_create.unwrap(),
                BarrierSync::COMPUTE_SHADING,
                ResourceAccessFlags::UNORDERED_ACCESS,
            ));
        },
        |_data: &()| {},
    );

    builder.add_pass(
        "test-pass-2",
        |_data: &mut (), resources: &mut ResourceRegistry| {
            out_read_transient = Some(resources.read_buffer(
                out_write_transient.unwrap(),
                BarrierSync::PIXEL_SHADING,
                ResourceAccessFlags::CONSTANT_BUFFER,
            ));
        },
        |_data: &()| {},
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

    unsafe {
        graph.execute();
    }
}

#[test]
pub fn test_usage_collection() {
    let mock_buffer = MockBuffer::new(&BufferDesc {
        size: 512,
        cpu_access: CpuAccessMode::None,
        usage: BufferUsageFlags::UNORDERED_ACCESS
            | BufferUsageFlags::CONSTANT_BUFFER
            | BufferUsageFlags::VERTEX_BUFFER,
        name: Some("imported-mock-resource"),
    });
    let mut out_create = None;
    let mut out_write_import = None;
    let mut out_write_transient = None;

    let mut builder = FrameGraph::builder();

    let imported_resource = builder.import_buffer(&BufferImportDesc {
        resource: mock_buffer.as_ref(),
        before_sync: BarrierSync::COMPUTE_SHADING,
        before_access: BarrierAccess::SHADER_WRITE,
        after_sync: BarrierSync::COPY,
        after_access: BarrierAccess::COPY_READ,
    });

    builder.add_pass(
        "test-pass-0",
        |_data: &mut (), resources: &mut ResourceRegistry| {
            resources.read_buffer(
                imported_resource,
                BarrierSync::VERTEX_SHADING,
                ResourceAccessFlags::VERTEX_BUFFER,
            );
            out_create = Some(resources.create_buffer(
                &BufferDesc {
                    size: 256,
                    name: Some("test-pass-0-transient-resource"),
                    ..Default::default()
                },
                BarrierSync::VERTEX_SHADING,
                ResourceAccessFlags::INDEX_BUFFER,
            ));
        },
        |_data: &()| {},
    );

    builder.add_pass(
        "test-pass-1",
        |_data: &mut (), resources: &mut ResourceRegistry| {
            out_write_import = Some(resources.write_buffer(
                imported_resource,
                BarrierSync::COMPUTE_SHADING,
                ResourceAccessFlags::UNORDERED_ACCESS,
            ));
            out_write_transient = Some(resources.write_buffer(
                out_create.unwrap(),
                BarrierSync::COMPUTE_SHADING,
                ResourceAccessFlags::UNORDERED_ACCESS,
            ));
        },
        |_data: &()| {},
    );

    builder.add_pass(
        "test-pass-2",
        |_data: &mut (), resources: &mut ResourceRegistry| {
            resources.read_buffer(
                out_write_transient.unwrap(),
                BarrierSync::PIXEL_SHADING,
                ResourceAccessFlags::CONSTANT_BUFFER,
            );
        },
        |_data: &()| {},
    );

    let mut graph = builder.build();

    let out_create = out_create.unwrap();

    let imported_r = imported_resource.0.root_id();
    let imported_usage = graph.root_resources[imported_r as usize]
        .resource_type
        .unwrap_buffer()
        .create_desc
        .usage;
    let imported_access = graph.root_resources[imported_r as usize].access_flags;
    assert_eq!(
        imported_usage,
        BufferUsageFlags::UNORDERED_ACCESS | BufferUsageFlags::VERTEX_BUFFER
    );
    assert_eq!(
        imported_access,
        ResourceAccessFlags::UNORDERED_ACCESS | ResourceAccessFlags::VERTEX_BUFFER
    );

    let out_create_r = out_create.0.root_id();
    let out_create_usage = graph.root_resources[out_create_r as usize]
        .resource_type
        .unwrap_buffer()
        .create_desc
        .usage;
    let out_create_access = graph.root_resources[out_create_r as usize].access_flags;
    assert_eq!(
        out_create_usage,
        BufferUsageFlags::UNORDERED_ACCESS
            | BufferUsageFlags::CONSTANT_BUFFER
            | BufferUsageFlags::INDEX_BUFFER
    );
    assert_eq!(
        out_create_access,
        ResourceAccessFlags::UNORDERED_ACCESS
            | ResourceAccessFlags::CONSTANT_BUFFER
            | ResourceAccessFlags::INDEX_BUFFER
    );

    unsafe {
        graph.execute();
    }
}
