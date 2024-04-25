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

use aleph_any::AnyArc;
use aleph_pin_board::PinBoard;
use aleph_rhi_api::*;
use aleph_rhi_null::NullContext;

use crate::frame_graph_builder::GraphBuildError;
use crate::{
    BufferImportDesc, FrameGraph, ImportBundle, ResourceMut, ResourceRef, TextureImportDesc,
};

fn make_null_device() -> AnyArc<dyn IDevice> {
    let context = NullContext::new_arced();
    let adapter = context.request_adapter(&Default::default()).unwrap();
    adapter.request_device().unwrap()
}

struct Import(ResourceMut);
struct Write(ResourceMut);
struct Read(ResourceRef);

#[test]
pub fn test_builder() {
    struct TestPassData {
        value: u32,
        resource: ResourceMut,
    }
    struct TestPassData2 {
        value: i16,
        resource: ResourceRef,
    }

    let device = make_null_device();
    let mut command_list = device
        .create_command_list(&CommandListDesc {
            queue_type: QueueType::General,
            name: None,
        })
        .unwrap();
    let mut encoder = command_list.begin_general().unwrap();

    let mut out_create = None;
    let mut out_write = None;
    let mut out_read = None;

    let mut builder = FrameGraph::builder();

    builder.add_pass("test-pass-0", |resources| {
        let payload = TestPassData {
            value: 54321,
            resource: resources.create_buffer_with_sync(
                &BufferDesc {
                    size: 256,
                    name: Some("test-pass-0-transient-resource"),
                    ..Default::default()
                },
                BarrierSync::COMPUTE_SHADING,
                ResourceUsageFlags::UNORDERED_ACCESS,
            ),
        };
        out_create = Some(payload.resource);

        move |_encoder, resources| {
            // Verify we got the right payload
            assert_eq!(payload.value, 54321);
            let context = resources.context().get::<usize>().copied().unwrap();
            assert_eq!(context, 512);
        }
    });

    builder.add_pass("test-pass-1", |resources| {
        let payload = TestPassData {
            value: 1234,
            resource: resources.write_buffer_with_sync(
                out_create.unwrap(),
                BarrierSync::COMPUTE_SHADING,
                ResourceUsageFlags::UNORDERED_ACCESS,
            ),
        };
        out_write = Some(payload.resource);

        move |_encoder, resources| {
            // Verify we got the right payload
            assert_eq!(payload.value, 1234);
            let context = resources.context().get::<usize>().copied().unwrap();
            assert_eq!(context, 512);
        }
    });

    builder.add_pass("test-pass-2", |resources| {
        let payload = TestPassData2 {
            value: -432,
            resource: resources.read_buffer_with_sync(
                out_write.unwrap(),
                BarrierSync::PIXEL_SHADING,
                ResourceUsageFlags::CONSTANT_BUFFER,
            ),
        };

        out_read = Some(payload.resource);

        move |_encoder, resources| {
            // Verify we got the right payload
            assert_eq!(payload.value, -432);
            let context = resources.context().get::<usize>().copied().unwrap();
            assert_eq!(context, 512);
        }
    });

    let mut graph = builder.build(device.as_ref());
    unsafe {
        graph.allocate_transients(1);
    }

    let import_bundle = ImportBundle::default();
    let context = PinBoard::new();
    context.publish(512usize);
    unsafe {
        graph.execute(0, &import_bundle, encoder.as_mut(), &context);
    }
}

#[test]
pub fn test_handle_equality() {
    let device = make_null_device();
    let mut command_list = device
        .create_command_list(&CommandListDesc {
            queue_type: QueueType::General,
            name: None,
        })
        .unwrap();
    let mut encoder = command_list.begin_general().unwrap();

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
    builder.add_pass("test-pass-0", |resources| {
        let r = resources.import_buffer_with_sync(
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

        move |_encoder, _resources| {}
    });
    let imported_resource = imported_resource.unwrap();

    builder.add_pass("test-pass-1", |resources| {
        out_read_import = Some(resources.read_buffer_with_sync(
            imported_resource,
            BarrierSync::PIXEL_SHADING,
            ResourceUsageFlags::CONSTANT_BUFFER,
        ));
        out_create = Some(resources.create_buffer_with_sync(
            &BufferDesc {
                size: 256,
                name: Some("test-pass-1-transient-resource"),
                ..Default::default()
            },
            BarrierSync::COMPUTE_SHADING,
            ResourceUsageFlags::UNORDERED_ACCESS,
        ));

        move |_encoder, _resources| {}
    });

    builder.add_pass("test-pass-2", |resources| {
        out_write_import = Some(resources.write_buffer_with_sync(
            imported_resource,
            BarrierSync::COMPUTE_SHADING,
            ResourceUsageFlags::UNORDERED_ACCESS,
        ));
        out_write_transient = Some(resources.write_buffer_with_sync(
            out_create.unwrap(),
            BarrierSync::COMPUTE_SHADING,
            ResourceUsageFlags::UNORDERED_ACCESS,
        ));

        move |_encoder, _resources| {}
    });

    builder.add_pass("test-pass-3", |resources| {
        out_read_transient = Some(resources.read_buffer_with_sync(
            out_write_transient.unwrap(),
            BarrierSync::PIXEL_SHADING,
            ResourceUsageFlags::CONSTANT_BUFFER,
        ));
        move |_encoder, _resources| {}
    });

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

    let mut graph = builder.build(device.as_ref());
    unsafe {
        graph.allocate_transients(1);
    }

    let mut import_bundle = ImportBundle::default();
    import_bundle.add_resource(imported_resource, &mock_buffer);
    unsafe {
        graph.execute(0, &import_bundle, encoder.as_mut(), &PinBoard::new());
    }
}

#[test]
pub fn test_usage_collection() {
    let device = make_null_device();
    let mut command_list = device
        .create_command_list(&CommandListDesc {
            queue_type: QueueType::General,
            name: None,
        })
        .unwrap();
    let mut encoder = command_list.begin_general().unwrap();

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
    builder.add_pass("test-pass-0", |resources| {
        let r = resources.import_buffer(
            &BufferImportDesc {
                desc: &mock_desc,
                before_sync: BarrierSync::COMPUTE_SHADING,
                before_access: BarrierAccess::SHADER_WRITE,
                after_sync: BarrierSync::COPY,
                after_access: BarrierAccess::COPY_READ,
            },
            ResourceUsageFlags::NONE,
        );
        imported_resource = Some(r);

        move |_encoder, _resources| {}
    });
    let imported_resource = imported_resource.unwrap();

    builder.add_pass("test-pass-1", |resources| {
        resources.read_buffer_with_sync(
            imported_resource,
            BarrierSync::VERTEX_SHADING,
            ResourceUsageFlags::VERTEX_BUFFER,
        );
        out_create = Some(resources.create_buffer_with_sync(
            &BufferDesc {
                size: 256,
                name: Some("test-pass-1-transient-resource"),
                ..Default::default()
            },
            BarrierSync::VERTEX_SHADING,
            ResourceUsageFlags::INDEX_BUFFER,
        ));
        move |_encoder, _resources| {}
    });

    builder.add_pass("test-pass-2", |resources| {
        out_write_import = Some(resources.write_buffer_with_sync(
            imported_resource,
            BarrierSync::COMPUTE_SHADING,
            ResourceUsageFlags::UNORDERED_ACCESS,
        ));
        out_write_transient = Some(resources.write_buffer_with_sync(
            out_create.unwrap(),
            BarrierSync::COMPUTE_SHADING,
            ResourceUsageFlags::UNORDERED_ACCESS,
        ));
        move |_encoder, _resources| {}
    });

    builder.add_pass("test-pass-3", |resources| {
        resources.read_buffer_with_sync(
            out_write_transient.unwrap(),
            BarrierSync::PIXEL_SHADING,
            ResourceUsageFlags::CONSTANT_BUFFER,
        );
        move |_encoder, _resources| {}
    });

    let mut graph = builder.build(device.as_ref());
    unsafe {
        graph.allocate_transients(1);
    }

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
    unsafe {
        graph.execute(0, &import_bundle, encoder.as_mut(), &PinBoard::new());
    }
}

#[test]
pub fn test_usage_schedule() {
    let pin_board = PinBoard::new();
    let device = make_null_device();
    let mut command_list = device
        .create_command_list(&CommandListDesc {
            queue_type: QueueType::General,
            name: None,
        })
        .unwrap();
    let mut encoder = command_list.begin_general().unwrap();

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

    let mut builder = FrameGraph::builder();

    struct Pass0 {
        import: ResourceMut,
    }
    builder.add_pass("test-pass-0", |resources| {
        let import = resources.import_buffer(
            &BufferImportDesc {
                desc: &mock_buffer_desc,
                before_sync: BarrierSync::COMPUTE_SHADING,
                before_access: BarrierAccess::SHADER_WRITE,
                after_sync: BarrierSync::COPY,
                after_access: BarrierAccess::COPY_READ,
            },
            ResourceUsageFlags::NONE,
        );
        pin_board.publish(Pass0 { import });
        move |_encoder, _resources| {}
    });

    struct Pass1 {
        create: ResourceMut,
        import: ResourceMut,
    }
    builder.add_pass("test-pass-1", |resources| {
        let import = pin_board.get::<Pass0>().unwrap().import;
        resources.read_buffer_with_sync(
            import,
            BarrierSync::VERTEX_SHADING,
            ResourceUsageFlags::VERTEX_BUFFER,
        );
        let create = resources.create_buffer_with_sync(
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
            ResourceUsageFlags::RENDER_TARGET,
        );

        pin_board.publish(Pass1 { create, import });
        move |_encoder, _resources| {}
    });

    #[derive(Clone)]
    struct Pass2 {
        import_buffer_write: ResourceMut,
        import_texture_write: ResourceMut,
        transient_write: ResourceMut,
    }
    builder.add_pass("test-pass-2", |resources| {
        let import_buffer = pin_board.get::<Pass0>().unwrap().import;
        let pass1 = pin_board.get::<Pass1>().unwrap();
        let create = pass1.create;
        let import_texture = pass1.import;

        let import_buffer_write = resources.write_buffer_with_sync(
            import_buffer,
            BarrierSync::COMPUTE_SHADING,
            ResourceUsageFlags::UNORDERED_ACCESS,
        );

        let import_texture_write = resources.write_texture_with_sync(
            import_texture,
            BarrierSync::COMPUTE_SHADING,
            ResourceUsageFlags::UNORDERED_ACCESS,
        );

        let transient_write = resources.write_buffer_with_sync(
            create,
            BarrierSync::COMPUTE_SHADING,
            ResourceUsageFlags::UNORDERED_ACCESS,
        );

        let payload = Pass2 {
            import_buffer_write,
            import_texture_write,
            transient_write,
        };
        pin_board.publish(payload.clone());

        move |_encoder, resources| {
            let _resource = resources.get_buffer(payload.import_buffer_write).unwrap();
            let _resource = resources.get_texture(payload.import_texture_write).unwrap();
        }
    });

    builder.add_pass("test-pass-3", |resources| {
        let transient = pin_board.get::<Pass2>().unwrap().transient_write;
        let read = resources.read_buffer_with_sync(
            transient,
            BarrierSync::PIXEL_SHADING,
            ResourceUsageFlags::CONSTANT_BUFFER,
        );

        move |_encoder, resources| {
            let _resource = resources.get_buffer(read).unwrap();
        }
    });

    builder.add_pass("test-pass-4", |resources| {
        let resource = pin_board.get::<Pass2>().unwrap().import_texture_write;
        let read = resources.read_texture_with_sync(
            resource,
            BarrierSync::PIXEL_SHADING,
            ResourceUsageFlags::SHADER_RESOURCE,
        );

        move |_encoder, resources| {
            let _resource = resources.get_texture(read).unwrap();
        }
    });

    builder.add_pass("test-pass-5", |resources| {
        let resource = pin_board.get::<Pass2>().unwrap().import_texture_write;
        let read = resources.read_texture_with_sync(
            resource,
            BarrierSync::PIXEL_SHADING,
            ResourceUsageFlags::SHADER_RESOURCE,
        );

        move |_encoder, resources| {
            let _resource = resources.get_texture(read).unwrap();
        }
    });

    builder.add_pass("test-pass-6", |resources| {
        let resource = pin_board.get::<Pass2>().unwrap().import_texture_write;
        let read = resources.read_texture_with_sync(
            resource,
            BarrierSync::PIXEL_SHADING,
            ResourceUsageFlags::SHADER_RESOURCE,
        );

        move |_encoder, resources| {
            let _resource = resources.get_texture(read).unwrap();
        }
    });

    builder.add_pass("test-pass-7", |resources| {
        let resource = pin_board.get::<Pass2>().unwrap().import_texture_write;
        let read = resources.read_texture_with_sync(
            resource,
            BarrierSync::DEPTH_STENCIL,
            ResourceUsageFlags::RENDER_TARGET,
        );

        move |_encoder, resources| {
            let _resource = resources.get_texture(read).unwrap();
        }
    });

    builder.add_pass("test-pass-8", |resources| {
        let resource = pin_board.get::<Pass2>().unwrap().import_texture_write;
        let read = resources.read_texture_with_sync(
            resource,
            BarrierSync::DEPTH_STENCIL,
            ResourceUsageFlags::RENDER_TARGET,
        );

        move |_encoder, resources| {
            let _resource = resources.get_texture(read).unwrap();
        }
    });

    #[derive(Clone)]
    struct Pass9 {
        pub import_texture_write: ResourceMut,
    }
    builder.add_pass("test-pass-9", |resources| {
        let resource = pin_board.get::<Pass2>().unwrap().import_texture_write;
        let import_texture_write = resources.write_texture_with_sync(
            resource,
            BarrierSync::DEPTH_STENCIL,
            ResourceUsageFlags::RENDER_TARGET,
        );

        let payload = Pass9 {
            import_texture_write,
        };
        pin_board.publish(payload.clone());

        move |_encoder, resources| {
            let _resource = resources.get_texture(payload.import_texture_write).unwrap();
        }
    });

    let mut dot_text = Vec::<u8>::new();
    let mut graph = builder
        .build_with_graph_viz(
            device.as_ref(),
            "TestGraph",
            &mut dot_text,
            &Default::default(),
        )
        .unwrap();
    graph
        .graph_viz_for_pass_order("PassOrder", &mut dot_text)
        .unwrap();
    unsafe {
        graph.allocate_transients(1);
    }

    // std::fs::write("./graphviz.dot", dot_text).unwrap();

    let import_buffer = pin_board.get::<Pass0>().unwrap().import;
    let import_texture = pin_board.get::<Pass1>().unwrap().import;
    let mut import_bundle = ImportBundle::default();
    import_bundle.add_resource(import_buffer, &mock_buffer);
    import_bundle.add_resource(import_texture, &mock_texture);
    unsafe {
        graph.execute(0, &import_bundle, encoder.as_mut(), &PinBoard::new());
    }
}

#[test]
pub fn test_usage_illegal_dependency() {
    let pin_board = PinBoard::new();
    let device = make_null_device();

    let mock_buffer = device
        .create_buffer(&BufferDesc {
            size: 512,
            cpu_access: CpuAccessMode::None,
            usage: ResourceUsageFlags::UNORDERED_ACCESS | ResourceUsageFlags::CONSTANT_BUFFER,
            name: Some("imported-mock-buffer"),
        })
        .unwrap();
    let mock_buffer_desc = mock_buffer.desc();

    let mut builder = FrameGraph::builder();

    builder.add_pass("import-pass", |resources| {
        let import = resources.import_buffer(
            &BufferImportDesc {
                desc: &mock_buffer_desc,
                before_sync: BarrierSync::COMPUTE_SHADING,
                before_access: BarrierAccess::SHADER_WRITE,
                after_sync: BarrierSync::COPY,
                after_access: BarrierAccess::COPY_READ,
            },
            ResourceUsageFlags::UNORDERED_ACCESS,
        );
        pin_board.publish(Import(import));

        move |_encoder, _resources| {}
    });

    builder.add_pass("writer-pass", |resources| {
        let import: &Import = pin_board.get().unwrap();
        let write = resources.write_buffer(import.0, ResourceUsageFlags::UNORDERED_ACCESS);
        pin_board.publish(Write(write));

        move |_, _| {}
    });

    builder.add_pass("reader-pass", |resources| {
        let import: &Import = pin_board.get().unwrap();
        let read = resources.read_buffer(import.0, ResourceUsageFlags::UNORDERED_ACCESS);
        pin_board.publish(Read(read));

        move |_, _| {}
    });

    builder.add_pass("deadly-pass", |resources| {
        let import: &Import = pin_board.get().unwrap();
        let write: &Write = pin_board.get().unwrap();

        // This creates a cyclic dependency. We depend on resource version N and N-1. This
        // dependency can't be satisfied as version N destroys version N-1 when it is written.
        // The graph builder turns this into a cyclic dependency (in what's meant to be a DAG)
        // which then gets picked up by our cycle detection code.
        //
        // This dependency will lead to the build call panicking. If we didn't panic then the
        // pass scheduler would deadlock.
        //
        // A user should work-around this by producing a copy of a resource if it's needed after
        // it gets written over.
        let _bad_read = resources.read_buffer(import.0, ResourceUsageFlags::CONSTANT_BUFFER);
        let _bad_write = resources.write_buffer(write.0, ResourceUsageFlags::UNORDERED_ACCESS);

        move |_, _| {}
    });

    let mut dot_text = Vec::<u8>::new();
    let result = builder.build_with_graph_viz(
        device.as_ref(),
        "TestGraph",
        &mut dot_text,
        &Default::default(),
    );

    // std::fs::write("./graphviz.dot", dot_text).unwrap();

    assert!(matches!(
        result,
        Err(GraphBuildError::CyclicDependencyDetected)
    ));
}

#[test]
pub fn test_usage_illegal_dependency_2() {
    let pin_board = PinBoard::new();
    let device = make_null_device();

    let mock_buffer = device
        .create_buffer(&BufferDesc {
            size: 512,
            cpu_access: CpuAccessMode::None,
            usage: ResourceUsageFlags::UNORDERED_ACCESS | ResourceUsageFlags::CONSTANT_BUFFER,
            name: Some("imported-mock-buffer"),
        })
        .unwrap();
    let mock_buffer_desc = mock_buffer.desc();

    let mut builder = FrameGraph::builder();

    builder.add_pass("import-pass", |resources| {
        let import = resources.import_buffer(
            &BufferImportDesc {
                desc: &mock_buffer_desc,
                before_sync: BarrierSync::COMPUTE_SHADING,
                before_access: BarrierAccess::SHADER_WRITE,
                after_sync: BarrierSync::COPY,
                after_access: BarrierAccess::COPY_READ,
            },
            ResourceUsageFlags::UNORDERED_ACCESS,
        );
        pin_board.publish(Import(import));

        move |_encoder, _resources| {}
    });

    builder.add_pass("writer-pass-1", |resources| {
        let import: &Import = pin_board.get().unwrap();
        let write = resources.write_buffer(import.0, ResourceUsageFlags::UNORDERED_ACCESS);
        pin_board.publish(Write(write));

        move |_encoder, _resources| {}
    });

    builder.add_pass("writer-pass-2", |resources| {
        let write: &Write = pin_board.get().unwrap();
        let write = resources.write_buffer(write.0, ResourceUsageFlags::UNORDERED_ACCESS);
        pin_board.publish(Write(write));

        move |_encoder, _resources| {}
    });

    builder.add_pass("reader-pass", |resources| {
        let import: &Import = pin_board.get().unwrap();
        let read = resources.read_buffer(import.0, ResourceUsageFlags::UNORDERED_ACCESS);
        pin_board.publish(Read(read));

        move |_encoder, _resources| {}
    });

    builder.add_pass("deadly-pass", |resources| {
        let import: &Import = pin_board.get().unwrap();
        let write: &Write = pin_board.get().unwrap();

        // This creates a cyclic dependency. We depend on resource version N and N-1. This
        // dependency can't be satisfied as version N destroys version N-1 when it is written.
        // The graph builder turns this into a cyclic dependency (in what's meant to be a DAG)
        // which then gets picked up by our cycle detection code.
        //
        // This dependency will lead to the build call panicking. If we didn't panic then the
        // pass scheduler would deadlock.
        //
        // A user should work-around this by producing a copy of a resource if it's needed after
        // it gets written over.
        let _bad_read = resources.read_buffer(import.0, ResourceUsageFlags::CONSTANT_BUFFER);
        let _bad_write = resources.write_buffer(write.0, ResourceUsageFlags::UNORDERED_ACCESS);

        move |_encoder, _resources| {}
    });

    let mut dot_text = Vec::<u8>::new();
    let result = builder.build_with_graph_viz(
        device.as_ref(),
        "TestGraph",
        &mut dot_text,
        &Default::default(),
    );

    // std::fs::write("./graphviz.dot", dot_text).unwrap();

    assert!(matches!(
        result,
        Err(GraphBuildError::CyclicDependencyDetected)
    ));
}
