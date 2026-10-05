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

use std::borrow::Cow;
use std::io;
use std::sync::Arc;

use aleph_gen_arena::HandleType;
use aleph_math::{Mat4, Rotor3, ToDouble, Vec3, Vec4};
use aleph_vfs::IRouter;
use aleph_vfs::path::VPath;
use api::components::{StaticMesh, Transform};
use bytemuck::{Pod, Zeroable};
use mg::material_instance::MaterialInstanceHandle;
use mg::resource::buffer::BufferHandle;

use crate::core::async_io::context::IoContext;
use crate::render::async_loader::internal::buffer_load::issue_load_buffer_from_data;
use crate::render::async_loader::systems::async_load_resolver::AsyncLoadResolverQueue;

pub struct GltfLoadPayload {
    /// The vfs path of the file to open and read from.
    pub path: Box<VPath>,

    /// The destination the loaded GLTF file should be pushed on to.
    pub dest: AsyncLoadResolverQueue,

    /// A dummy material instance to use.
    pub default_material: MaterialInstanceHandle,
}

pub async fn task<'a>(
    vfs: Arc<dyn IRouter>,
    io: IoContext<'a>,
    msg: GltfLoadPayload,
) -> io::Result<()> {
    let result = import_path(vfs.as_ref(), &io, &msg.path).await;
    let result = result.map_err(|e| match e {
        gltf::Error::Io(e) => e,
        _ => {
            log::error!("Failed to import glTF: {e}");
            io::Error::from(io::ErrorKind::Other)
        }
    });
    let imported = result?;
    let imported = Arc::new(imported);

    process_document(vfs.as_ref(), &io, imported, msg).await?;

    Ok(())
}

async fn import_path(
    vfs: &dyn IRouter,
    io: &IoContext<'_>,
    path: &VPath,
) -> gltf::Result<(gltf::Document, Vec<gltf::buffer::Data>)> {
    let base = path.parent().unwrap_or_else(|| VPath::new("./"));

    let file = io.open_file(vfs, path).await?;
    let data = io.load_file(file.as_ref()).await?;

    let gltf::Gltf { document, blob } = gltf::Gltf::from_slice(&data).map_err(|e| {
        log::error!("Failed to load gltf: {:?}", e);
        io::Error::from(io::ErrorKind::InvalidData)
    })?;

    let buffer_data = import_buffers(vfs, io, &document, Some(base), blob).await?;
    let import = (document, buffer_data);
    Ok(import)
}

async fn import_buffers(
    vfs: &dyn IRouter,
    io: &IoContext<'_>,
    document: &gltf::Document,
    base: Option<&VPath>,
    mut blob: Option<Vec<u8>>,
) -> gltf::Result<Vec<gltf::buffer::Data>> {
    let mut buffers = Vec::new();
    for buffer in document.buffers() {
        let data = from_source_and_blob(vfs, io, buffer.source(), base, &mut blob).await?;
        if data.len() < buffer.length() {
            return Err(gltf::Error::BufferLength {
                buffer: buffer.index(),
                expected: buffer.length(),
                actual: data.len(),
            });
        }
        buffers.push(data);
    }
    Ok(buffers)
}

async fn from_source_and_blob(
    vfs: &dyn IRouter,
    io: &IoContext<'_>,
    source: gltf::buffer::Source<'_>,
    base: Option<&VPath>,
    blob: &mut Option<Vec<u8>>,
) -> gltf::Result<gltf::buffer::Data> {
    let mut data = match source {
        gltf::buffer::Source::Uri(uri) => Scheme::read(vfs, io, base, uri).await,
        gltf::buffer::Source::Bin => blob.take().ok_or(gltf::Error::MissingBlob),
    }?;
    while data.len() % 4 != 0 {
        data.push(0);
    }
    Ok(gltf::buffer::Data(data))
}

/// Represents the set of URI schemes the importer supports.
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
enum Scheme<'a> {
    /// `data:[<media type>];base64,<data>`.
    Data(Option<&'a str>, &'a str),

    /// `file:[//]<absolute file path>`.
    ///
    /// Note: The file scheme does not implement authority.
    File(&'a str),

    /// `../foo`, etc.
    Relative(Cow<'a, str>),

    /// Placeholder for an unsupported URI scheme identifier.
    Unsupported,
}

impl<'a> Scheme<'a> {
    fn parse(uri: &str) -> io::Result<Scheme<'_>> {
        let out = if uri.contains(':') {
            if let Some(rest) = uri.strip_prefix("data:") {
                let mut it = rest.split(";base64,");

                match (it.next(), it.next()) {
                    (match0_opt, Some(match1)) => Scheme::Data(match0_opt, match1),
                    (Some(match0), _) => Scheme::Data(None, match0),
                    _ => Scheme::Unsupported,
                }
            } else if let Some(rest) = uri.strip_prefix("file://") {
                Scheme::File(rest)
            } else if let Some(rest) = uri.strip_prefix("file:") {
                Scheme::File(rest)
            } else {
                Scheme::Unsupported
            }
        } else {
            Scheme::Relative(
                urlencoding::decode(uri)
                    .map_err(|_| io::Error::from(io::ErrorKind::InvalidData))?,
            )
        };
        Ok(out)
    }

    async fn read(
        vfs: &dyn IRouter,
        io: &IoContext<'_>,
        base: Option<&VPath>,
        uri: &str,
    ) -> gltf::Result<Vec<u8>> {
        match Scheme::parse(uri)? {
            // The path may be unused in the Scheme::Data case
            // Example: "uri" : "data:application/octet-stream;base64,wsVHPgA...."
            Scheme::Data(_, base64) => base64::decode(base64).map_err(gltf::Error::Base64),
            Scheme::File(path) if base.is_some() => read_to_end(vfs, io, path).await,
            Scheme::Relative(path) if base.is_some() => {
                read_to_end(vfs, io, base.unwrap().join(&*path)).await
            }
            Scheme::Unsupported => Err(gltf::Error::UnsupportedScheme),
            _ => Err(gltf::Error::ExternalReferenceInSliceImport),
        }
    }
}

async fn read_to_end<P>(vfs: &dyn IRouter, io: &IoContext<'_>, path: P) -> gltf::Result<Vec<u8>>
where
    P: AsRef<VPath>,
{
    let file = io.open_file(vfs, path).await?;
    let data = io.load_file(file.as_ref()).await?;
    Ok(data)
}

async fn process_document(
    _vfs: &dyn IRouter,
    io: &IoContext<'_>,
    imported: Arc<(gltf::Document, Vec<gltf::buffer::Data>)>,
    msg: GltfLoadPayload,
) -> io::Result<()> {
    let document = &imported.0;

    let (sender, receiver) = kanal::bounded_async(document.meshes().len());

    spawn_mesh_uploader(&io, imported.clone(), sender);

    let mut mesh_table = Vec::with_capacity(document.meshes().len());
    for _ in document.meshes() {
        mesh_table.push(Vec::new())
    }

    for _ in 0..mesh_table.len() {
        let result = receiver.recv().await;
        let (mesh_i, prims) =
            result.map_err(|_| io::Error::from(io::ErrorKind::ConnectionAborted))?;
        mesh_table[mesh_i] = prims;
    }

    // let mut mat_table = Vec::new();
    // for (i, mat) in document.materials().enumerate() {
    //     let _i = mat.index().unwrap();
    //     debug_assert_eq!(i, _i);
    //
    //     let pbr_mat = mat.pbr_metallic_roughness();
    //
    //     let white_tex: TextureHandle = renderer.default_resources().white_texture_rgba8();
    //     let norm_tex: TextureHandle = renderer.default_resources().normal_texture_rgba8();
    //     let layout = StandardMaterialLayout {
    //         colour: pbr_mat.base_color_factor(),
    //         metal_roughness: [
    //             pbr_mat.metallic_factor(),
    //             pbr_mat.roughness_factor(),
    //             0.0,
    //             0.0,
    //         ],
    //         _padding1: [0; 128],
    //         _padding2: [0; 96],
    //     };
    //
    //     let mut desc = BufferObjectDesc::new();
    //     desc.size(256);
    //     desc.usage(ResourceUsageFlags::CONSTANT_BUFFER);
    //     let mut upload = BufferUploadDesc::new_owned(renderer.device(), &desc).unwrap();
    //     upload
    //         .buffer
    //         .bytes_mut()
    //         .copy_from_slice(bytemuck::bytes_of(&layout));
    //     let buffer = renderer.create_buffer(object).unwrap();
    //
    //     let bindings = [
    //         MaterialBinding::Buffer(Some(buffer)),
    //         MaterialBinding::Texture(Some(white_tex)),
    //         MaterialBinding::Texture(Some(white_tex)),
    //         MaterialBinding::Texture(Some(norm_tex)),
    //     ];
    //     let desc = MaterialInstanceDesc {
    //         double_sided: mat.double_sided(),
    //         bindings: &bindings,
    //     };
    //     let material_instance = renderer.create_material_instance(desc).unwrap();
    //
    //     mat_table.push(material_instance);
    // }

    let send_imported = imported.clone();
    rayon::spawn(move || {
        let imported = send_imported;
        let document = &imported.0;

        let mut transforms = Vec::new();
        let mut static_meshes = Vec::new();

        let root_transform = Mat4::identity();
        if let Some(scene) = document.default_scene() {
            for node in scene.nodes() {
                process_node(
                    &mut transforms,
                    &mut static_meshes,
                    root_transform,
                    &node,
                    &mesh_table,
                    &msg,
                );
            }
        }

        msg.dest.push(move |world, _renderer| {
            let _ = world.bulk_insert((transforms, static_meshes));
        });
    });

    Ok(())
}

fn spawn_mesh_uploader(
    io: &IoContext<'_>,
    imported: Arc<(gltf::Document, Vec<gltf::buffer::Data>)>,
    sender: kanal::AsyncSender<(usize, Vec<(BufferHandle, BufferHandle)>)>,
) {
    let async_queue = io.async_queue();
    rayon::spawn(move || {
        aleph_profile::scope_named!("gltf::scene_mesh_uploader");
        use rayon::prelude::*;

        let document = &imported.0;
        let buffers = imported.1.as_slice();

        let meshes: Vec<_> = document.meshes().collect();
        meshes
            .into_par_iter()
            .enumerate()
            .for_each(|(mesh_index, mesh)| {
                aleph_profile::scope_named!("gltf::mesh_upload");
                let mut prims = Vec::with_capacity(mesh.primitives().len());
                for prim in mesh.primitives() {
                    let indices = prim.indices().unwrap();

                    let index_count = index_upload_count(&indices);
                    let mut i_data = Vec::with_capacity(index_count);
                    i_data.resize(index_count, 0);

                    let stride = 60;
                    let vertex_count = vertex_upload_count(&prim);
                    let vertex_size = vertex_count * stride;
                    let mut v_data = Vec::with_capacity(vertex_size);
                    v_data.resize(vertex_size, 0);

                    {
                        aleph_profile::scope_named!("gltf::load_index_buffer_data");
                        load_index_buffer_data(&mut i_data, buffers, &indices);
                    }
                    {
                        aleph_profile::scope_named!("gltf::load_vertex_buffer_data");
                        load_vertex_buffer_data(
                            &mut v_data,
                            buffers,
                            &prim,
                            &i_data,
                            vertex_count,
                            stride,
                        );
                    }

                    prims.push((i_data, v_data));
                }

                let sender = sender.clone();
                let _ = async_queue.spawn(async move |io| {
                    copy_mesh_data_to_gpu(io, mesh_index, prims, sender).await
                });
            });
    });
}

async fn copy_mesh_data_to_gpu(
    io: IoContext<'_>,
    mesh_index: usize,
    prims: Vec<(Vec<u32>, Vec<u8>)>,
    sender: kanal::AsyncSender<(usize, Vec<(BufferHandle, BufferHandle)>)>,
) -> io::Result<()> {
    let mut prim_buffers = Vec::with_capacity(prims.len());
    prim_buffers.resize(
        prims.len(),
        (BufferHandle::dangling(), BufferHandle::dangling()),
    );

    let (i_sender, i_receiver) = kanal::bounded_async(prims.len());
    let (v_sender, v_receiver) = kanal::bounded_async(prims.len());

    for (i, (i_data, _)) in prims.iter().enumerate() {
        let i_data = bytemuck::cast_slice::<_, u8>(i_data.as_slice());
        issue_load_buffer_from_data(&io, i_sender.clone_sync(), i as u64, i_data)?;
    }

    for (i, (_, v_data)) in prims.iter().enumerate() {
        issue_load_buffer_from_data(&io, v_sender.clone_sync(), i as u64, v_data)?;
    }

    drop(i_sender);
    drop(v_sender);

    for _ in 0..i_receiver.capacity() {
        let v = match i_receiver.recv().await {
            Ok(v) => v,
            Err(_) => {
                return Err(io::Error::from(io::ErrorKind::ConnectionAborted));
            }
        };
        let handle = match v.0 {
            Ok(v) => Ok(v),
            Err(_) => {
                Err(io::Error::from(io::ErrorKind::Other))
            }
        };
        prim_buffers[v.1 as usize].0 = handle?;
    }

    for _ in 0..v_receiver.capacity() {
        let v = match v_receiver.recv().await {
            Ok(v) => v,
            Err(_) => {
                return Err(io::Error::from(io::ErrorKind::ConnectionAborted));
            }
        };
        let handle = match v.0 {
            Ok(v) => Ok(v),
            Err(_) => {
                Err(io::Error::from(io::ErrorKind::Other))
            }
        };
        prim_buffers[v.1 as usize].1 = handle?;
    }

    let _ = sender.send((mesh_index, prim_buffers)).await;
    Ok(())
}

fn process_node(
    transforms: &mut Vec<Transform>,
    static_meshes: &mut Vec<StaticMesh>,
    parent_transform: Mat4,
    node: &gltf::Node,
    mesh_table: &[Vec<(BufferHandle, BufferHandle)>],
    msg: &GltfLoadPayload,
) {
    let [col1, col2, col3, col4] = node.transform().matrix();
    let self_transform = Mat4::new(
        Vec4::from(col1),
        Vec4::from(col2),
        Vec4::from(col3),
        Vec4::from(col4),
    );

    let world_transform = parent_transform * self_transform;

    let decompose = gltf::scene::Transform::Matrix {
        matrix: [
            *world_transform.cols[0].as_array(),
            *world_transform.cols[1].as_array(),
            *world_transform.cols[2].as_array(),
            *world_transform.cols[3].as_array(),
        ],
    };
    let (t, r, s) = decompose.decomposed();

    if let Some(mesh) = node.mesh() {
        for (prim, (idx, vtx)) in mesh.primitives().zip(mesh_table[mesh.index()].iter()) {
            // let mat = prim.material().index().unwrap();
            match prim.material().alpha_mode() {
                gltf::material::AlphaMode::Opaque => {
                    transforms.push(Transform {
                        position: Vec3::from(t).to_double(),
                        rotation: Rotor3::from_quaternion_array(r),
                        scale: Vec3::from(s),
                    });

                    static_meshes.push(StaticMesh {
                        vtx: *vtx,
                        idx: *idx,
                        material_instance: msg.default_material,
                    });
                }
                #[allow(unreachable_patterns)]
                _ => {}
            }
        }
    }

    for node in node.children() {
        process_node(
            transforms,
            static_meshes,
            world_transform,
            &node,
            mesh_table,
            &msg,
        );
    }
}

fn index_upload_count(indices: &gltf::Accessor) -> usize {
    use gltf::accessor::{DataType, Dimensions};
    let data_type = indices.data_type();
    assert!(
        matches!(data_type, DataType::U32 | DataType::U16 | DataType::U8),
        "{data_type:?}"
    );
    assert_eq!(indices.dimensions(), Dimensions::Scalar);

    // We only use 32-bit indices
    indices.count()
}

fn index_upload_size(indices: &gltf::Accessor) -> usize {
    // We only use 32-bit indices
    index_upload_count(indices) * size_of::<u32>()
}

fn load_index_buffer_data(
    dst: &mut [u32],
    buffers: &[gltf::buffer::Data],
    indices: &gltf::Accessor,
) {
    use gltf::accessor::DataType;

    // We only use 32bit indices
    let size = index_upload_size(indices);

    let view = indices.view().unwrap();
    let src = &buffers[view.buffer().index()];
    let offset = view.offset() + indices.offset();
    assert_eq!(view.stride(), None);
    if indices.data_type() == DataType::U8 {
        assert!(view.length() >= (size / 4));
        let size = size / 4;
        let src = &src.0[offset..offset + size];
        for (d, s) in dst.iter_mut().zip(src) {
            *d = *s as u32;
        }
    } else if indices.data_type() == DataType::U16 {
        assert!(view.length() >= (size / 2));
        let size = size / 2;
        let src = &src.0[offset..offset + size];
        let src = bytemuck::cast_slice::<_, u16>(src);
        for (d, s) in dst.iter_mut().zip(src) {
            *d = *s as u32;
        }
    } else if indices.data_type() == DataType::U32 {
        assert!(
            view.length() >= size,
            "view.length < size ({} < {})",
            view.length(),
            size
        );

        let src = &src.0[offset..offset + size];
        let dst = bytemuck::cast_slice_mut::<_, u8>(dst);
        dst.copy_from_slice(src);
    } else {
        unreachable!();
    }
}

fn vertex_upload_count(prim: &gltf::Primitive) -> usize {
    let a = prim
        .attributes()
        .reduce(|l, r| {
            assert_eq!(l.1.count(), r.1.count());
            l
        })
        .unwrap();
    a.1.count()
}

fn load_vertex_buffer_data(
    dst: &mut [u8],
    buffers: &[gltf::buffer::Data],
    prim: &gltf::Primitive,
    indices: &[u32],
    vertex_count: usize,
    stride: usize,
) {
    {
        let dst = &mut dst[48..];
        for i in 0..vertex_count {
            let dst_i = stride * i;
            let dst = &mut dst[dst_i..dst_i + 12];
            let dst = bytemuck::cast_slice_mut::<_, f32>(dst);
            dst[0] = 1.0;
            dst[1] = 1.0;
            dst[2] = 1.0;
        }
    }

    let has_texcoord = prim
        .attributes()
        .any(|(s, _)| s == gltf::Semantic::TexCoords(0));
    let has_normals = prim.attributes().any(|(s, _)| s == gltf::Semantic::Normals);
    let has_tangents = prim
        .attributes()
        .any(|(s, _)| s == gltf::Semantic::Tangents);
    for (semantic, accessor) in prim.attributes() {
        match semantic {
            gltf::Semantic::Positions => {
                copy_vec3_f32_semantic(dst, &accessor, buffers, stride, 0);
            }
            gltf::Semantic::Normals => {
                copy_vec3_f32_semantic(dst, &accessor, buffers, stride, 20);
            }
            gltf::Semantic::Tangents => {
                copy_vec4_f32_semantic(dst, &accessor, buffers, stride, 32);
            }
            gltf::Semantic::Colors(0) => {
                // copy_vec3_f32_semantic(dst, &accessor, buffers, upload.stride, 48);
            }
            gltf::Semantic::Colors(_) => {}
            gltf::Semantic::TexCoords(0) => {
                copy_vec2_f32_semantic(dst, &accessor, buffers, stride, 12);
            }
            gltf::Semantic::TexCoords(_) => {
                // copy_vec2_f32_semantic(&mut upload.data, &accessor, buffers, upload.stride, ??);
            }
            gltf::Semantic::Joints(_) => unimplemented!(),
            gltf::Semantic::Weights(_) => unimplemented!(),
        }
    }

    if has_texcoord && has_normals && !has_tangents {
        aleph_profile::scope_named!("gltf::mikktspace");
        fn get_attr<'a, 'b>(
            buffers: &'b [gltf::buffer::Data],
            prim: &'a gltf::Primitive<'a>,
            semantic: gltf::Semantic,
        ) -> (gltf::Accessor<'a>, gltf::buffer::View<'a>, &'b [u8]) {
            let (_, attr) = prim.attributes().find(|(s, _)| *s == semantic).unwrap();
            let view = attr.view().unwrap();
            let buffer = &buffers[view.buffer().index()];
            let offset = view.offset() + attr.offset();
            let size = attr.size() * attr.count();
            let buffer = &buffer.0[offset..offset + size];
            (attr, view, buffer)
        }
        let (positions, positions_view, positions_buffer) =
            get_attr(buffers, prim, gltf::Semantic::Positions);
        let (normals, normals_view, normals_buffer) =
            get_attr(buffers, prim, gltf::Semantic::Normals);
        let (uvs, uvs_view, uvs_buffer) = get_attr(buffers, prim, gltf::Semantic::TexCoords(0));

        assert!(!indices.is_empty());
        assert_eq!(indices.len().next_multiple_of(3), indices.len());

        let mut geom = Geom {
            positions,
            positions_view,
            positions_buffer,
            normals,
            normals_view,
            normals_buffer,
            uvs,
            uvs_view,
            uvs_buffer,
            indices,
            dst: bytemuck::cast_slice_mut(dst),
        };
        let success = aleph_mikktspace::generate_tangents(&mut geom);
        assert!(success);
    }
}

fn copy_vec4_f32_semantic(
    dst: &mut [u8],
    accessor: &gltf::Accessor,
    buffers: &[gltf::buffer::Data],
    dst_stride: usize,
    dst_offset: usize,
) {
    use gltf::accessor::{DataType, Dimensions};
    assert_eq!(accessor.data_type(), DataType::F32);
    assert_eq!(accessor.dimensions(), Dimensions::Vec4);

    copy_vec_f32_semantic(dst, accessor, buffers, dst_stride, dst_offset);
}

fn copy_vec3_f32_semantic(
    dst: &mut [u8],
    accessor: &gltf::Accessor,
    buffers: &[gltf::buffer::Data],
    dst_stride: usize,
    dst_offset: usize,
) {
    use gltf::accessor::{DataType, Dimensions};
    assert_eq!(accessor.data_type(), DataType::F32);
    assert_eq!(accessor.dimensions(), Dimensions::Vec3);

    copy_vec_f32_semantic(dst, accessor, buffers, dst_stride, dst_offset);
}

fn copy_vec2_f32_semantic(
    dst: &mut [u8],
    accessor: &gltf::Accessor,
    buffers: &[gltf::buffer::Data],
    dst_stride: usize,
    dst_offset: usize,
) {
    use gltf::accessor::{DataType, Dimensions};
    assert_eq!(accessor.data_type(), DataType::F32);
    assert_eq!(accessor.dimensions(), Dimensions::Vec2);

    copy_vec_f32_semantic(dst, accessor, buffers, dst_stride, dst_offset);
}

fn copy_vec_f32_semantic(
    dst: &mut [u8],
    accessor: &gltf::Accessor,
    buffers: &[gltf::buffer::Data],
    dst_stride: usize,
    dst_offset: usize,
) {
    let view = accessor.view().unwrap();
    let e_size = accessor.size();
    let stride = view.stride().unwrap_or(e_size);

    let src = &buffers[view.buffer().index()];

    let offset = view.offset() + accessor.offset();
    let size = accessor.size() * accessor.count();
    let src = &src[offset..offset + size];
    let dst = &mut dst[dst_offset..];
    for i in 0..accessor.count() {
        let src_i = stride * i;
        let dst_i = dst_stride * i;
        // Copy one element from the source to the dest
        let src = &src[src_i..src_i + e_size];
        dst[dst_i..dst_i + e_size].copy_from_slice(src);
    }
}

struct Geom<'a, 'b> {
    positions: gltf::Accessor<'a>,
    positions_view: gltf::buffer::View<'a>,
    positions_buffer: &'a [u8],
    normals: gltf::Accessor<'a>,
    normals_view: gltf::buffer::View<'a>,
    normals_buffer: &'a [u8],
    uvs: gltf::Accessor<'a>,
    uvs_view: gltf::buffer::View<'a>,
    uvs_buffer: &'a [u8],
    indices: &'a [u32],
    dst: &'b mut [Vertex],
}

impl<'a, 'b> Geom<'a, 'b> {
    fn get_attribute<T: bytemuck::AnyBitPattern>(
        &self,
        buffer: &[u8],
        accessor: &gltf::Accessor,
        view: &gltf::buffer::View,
        face: usize,
        vert: usize,
    ) -> T {
        let e_size = accessor.size();
        let stride = view.stride().unwrap_or(e_size);

        let index = self.face_and_vert_to_index(face, vert);

        let start = index * stride;
        let end = start + e_size;
        let pos = &buffer[start..end];

        *bytemuck::from_bytes::<T>(pos)
    }

    fn face_and_vert_to_index(&self, face: usize, vert: usize) -> usize {
        let index = (face * 3) + vert;
        self.indices[index] as usize
    }
}

impl<'a, 'b> aleph_mikktspace::Geometry for Geom<'a, 'b> {
    fn num_faces(&self) -> usize {
        self.indices.len() / 3
    }

    fn num_vertices_of_face(&self, _face: usize) -> usize {
        3
    }

    fn position(&self, face: usize, vert: usize) -> [f32; 3] {
        use gltf::accessor::{DataType, Dimensions};
        debug_assert_eq!(self.positions.data_type(), DataType::F32);
        debug_assert_eq!(self.positions.dimensions(), Dimensions::Vec3);

        let buffer = &self.positions_buffer;
        let accessor = &self.positions;
        let view = &self.positions_view;

        self.get_attribute(buffer, accessor, view, face, vert)
    }

    fn normal(&self, face: usize, vert: usize) -> [f32; 3] {
        use gltf::accessor::{DataType, Dimensions};
        debug_assert_eq!(self.normals.data_type(), DataType::F32);
        debug_assert_eq!(self.normals.dimensions(), Dimensions::Vec3);

        let buffer = &self.normals_buffer;
        let accessor = &self.normals;
        let view = &self.normals_view;

        self.get_attribute(buffer, accessor, view, face, vert)
    }

    fn tex_coord(&self, face: usize, vert: usize) -> [f32; 2] {
        use gltf::accessor::{DataType, Dimensions};
        debug_assert_eq!(self.uvs.data_type(), DataType::F32);
        debug_assert_eq!(self.uvs.dimensions(), Dimensions::Vec2);

        let buffer = &self.uvs_buffer;
        let accessor = &self.uvs;
        let view = &self.uvs_view;

        self.get_attribute(buffer, accessor, view, face, vert)
    }

    fn set_tangent_encoded(&mut self, tangent: [f32; 4], face: usize, vert: usize) {
        let index = self.face_and_vert_to_index(face, vert);
        self.dst[index].tangent = tangent;
    }
}

#[repr(C)]
#[derive(Copy, Clone, Pod, Zeroable)]
struct Vertex {
    pub position: [f32; 3],
    pub uv: [f32; 2],
    pub normal: [f32; 3],
    pub tangent: [f32; 4],
    pub colour: [f32; 3],
}
