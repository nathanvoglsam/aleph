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

use aleph_device_allocators::UploadBumpAllocator;
use aleph_engine::any::AnyArc;
use aleph_engine::interfaces::components::{StaticMesh, Transform};
use aleph_engine::interfaces::ecs::{EntityId, World};
use aleph_engine::interfaces::math::{Mat4, Rotor3, ToDouble, Vec3, Vec4};
use aleph_engine::interfaces::renderer::{
    BufferHandle, BufferUploadSource, Renderer, RequestError, TextureStreamingRequest,
};
use aleph_rhi_api::*;
use gltf::accessor::{DataType, Dimensions};
use gltf::buffer::Data;
use gltf::material::AlphaMode;
use gltf::{Accessor, Primitive};

use crate::game::async_texture_loader::{AsyncTextureLoadRequest, AsyncTextureLoader};
use crate::game::cube_mesh::Vertex;

pub struct BumpThingy {
    device: AnyArc<dyn IDevice>,
    alloc: UploadBumpAllocator,
}

impl BumpThingy {
    pub fn new(device: &dyn IDevice) -> Self {
        Self {
            device: device.upgrade(),
            alloc: Self::new_alloc(device),
        }
    }

    pub unsafe fn alloc_upload_desc(
        &mut self,
        len: usize,
        align: usize,
        usage: ResourceUsageFlags,
    ) -> Result<BufferUploadSource, BufferCreateError> {
        match BufferUploadSource::new_in_bump_arena(&self.alloc, len, align, usage) {
            Ok(out) => Ok(out),
            Err(BufferCreateError::OutOfMemory) => {
                let mut new_block = Self::new_alloc(self.device.as_ref());
                std::mem::swap(&mut new_block, &mut self.alloc);
                BufferUploadSource::new_in_bump_arena(&self.alloc, len, align, usage)
            }
            e @ Err(_) => return e,
        }
    }

    fn new_alloc(device: &dyn IDevice) -> UploadBumpAllocator {
        const ALLOC_SIZE: usize = 64 * 1024 * 1024;
        UploadBumpAllocator::new_upload_buffer(device, ALLOC_SIZE, Some("BumpThingyUploadBlock"))
            .unwrap()
    }
}

pub struct TextureLoadThinker {
    target: EntityId,
    request: Option<TextureStreamingRequest>,
    target_tex: TargetTex,
}

impl TextureLoadThinker {
    pub fn poll_and_resolve(&mut self, world: &mut World) -> PollResult {
        let new_tex = match self.request.as_ref().map(|v| v.poll_complete()) {
            Some(Ok(v)) => v,
            Some(Err(RequestError::Waiting)) => return PollResult::Waiting,
            Some(Err(_)) => return PollResult::Fail,
            None => return PollResult::Fail,
        };

        let target = if let Some(v) = world.query_one_mut::<&mut StaticMesh>(self.target) {
            v
        } else {
            return PollResult::Fail;
        };

        match self.target_tex {
            TargetTex::Colour => target.colour_tex = new_tex,
            TargetTex::MetalRoughness => target.metal_roughness_tex = new_tex,
            TargetTex::Normal => target.normal_map_tex = new_tex,
        }

        self.request = None;

        PollResult::Success
    }
}

pub enum TargetTex {
    Colour,
    MetalRoughness,
    Normal,
}

pub enum PollResult {
    Success,
    Waiting,
    Fail,
}

#[aleph_profile::function]
pub fn load_scene(
    world: &mut World,
    renderer: &mut Renderer,
    arena: &mut BumpThingy,
    loader: &AsyncTextureLoader,
    path: &std::path::Path,
) -> Vec<TextureLoadThinker> {
    let (document, buffers) = import_path(path).unwrap();

    let base = path.parent().unwrap();
    let mut tex_table: Vec<Option<TextureStreamingRequest>> = Vec::new();
    for image in document.images() {
        match image.source() {
            gltf::image::Source::Uri { uri, .. } => {
                let path = base.join(uri);
                let request = loader.load(AsyncTextureLoadRequest { path });
                tex_table.push(Some(request));
            }
            _ => tex_table.push(None),
        }
    }

    let mut load_thinkers = Vec::new();

    let mut mesh_table = Vec::with_capacity(document.meshes().len());
    for mesh in document.meshes() {
        let mut prims = Vec::new();
        for prim in mesh.primitives() {
            assert_eq!(prim.mode(), gltf::mesh::Mode::Triangles);

            let indices = prim.indices().unwrap();
            let idx_buffer = load_index_buffer(renderer, arena, &buffers, &indices);

            // Get the upper bound for number of vertices
            let vtx_buffer = load_vertex_buffer(renderer, arena, &buffers, &prim);

            prims.push((idx_buffer, vtx_buffer));
        }
        mesh_table.push(prims);
    }

    fn process_node(
        world: &mut World,
        renderer: &Renderer,
        mesh_table: &[Vec<(BufferHandle, BufferHandle)>],
        tex_table: &[Option<TextureStreamingRequest>],
        thinkers: &mut Vec<TextureLoadThinker>,
        parent_transform: Mat4,
        node: &gltf::Node,
    ) {
        let [col1, col2, col3, col4] = node.transform().matrix();
        let self_transform = Mat4::new(
            Vec4::from(col1),
            Vec4::from(col2),
            Vec4::from(col3),
            Vec4::from(col4),
        );

        let world_transform = parent_transform * self_transform;

        let (t, r, s) = gltf::scene::Transform::Matrix {
            matrix: [
                *world_transform.cols[0].as_array(),
                *world_transform.cols[1].as_array(),
                *world_transform.cols[2].as_array(),
                *world_transform.cols[3].as_array(),
            ],
        }
        .decomposed();

        if let Some(mesh) = node.mesh() {
            for (prim, (idx, vtx)) in mesh.primitives().zip(mesh_table[mesh.index()].iter()) {
                let mat = prim.material();
                let pbr_mat = mat.pbr_metallic_roughness();
                match prim.material().alpha_mode() {
                    AlphaMode::Mask | AlphaMode::Blend | AlphaMode::Opaque => {
                        let transform = Transform {
                            position: Vec3::from(t).to_double(),
                            rotation: Rotor3::from_quaternion_array(r),
                            scale: Vec3::from(s),
                        };

                        let white_tex = renderer.default_resources().white_texture_rgba8();
                        let norm_tex = renderer.default_resources().normal_texture_rgba8();

                        let static_mesh = StaticMesh {
                            vtx: *vtx,
                            idx: *idx,
                            colour_tex: white_tex,
                            colour: pbr_mat.base_color_factor(),
                            metalness: pbr_mat.metallic_factor(),
                            roughness: pbr_mat.roughness_factor(),
                            metal_roughness_tex: white_tex,
                            normal_map_tex: norm_tex,
                        };
                        let entity = world.extend_one((transform, static_mesh));

                        if let Some(tex) = pbr_mat.base_color_texture() {
                            if let Some(req) = &tex_table[tex.texture().source().index()] {
                                thinkers.push(TextureLoadThinker {
                                    target: entity,
                                    request: Some(req.clone()),
                                    target_tex: TargetTex::Colour,
                                });
                            }
                        }

                        if let Some(tex) = pbr_mat.metallic_roughness_texture() {
                            if let Some(req) = &tex_table[tex.texture().source().index()] {
                                thinkers.push(TextureLoadThinker {
                                    target: entity,
                                    request: Some(req.clone()),
                                    target_tex: TargetTex::MetalRoughness,
                                });
                            }
                        }

                        if let Some(tex) = mat.normal_texture() {
                            if let Some(req) = &tex_table[tex.texture().source().index()] {
                                thinkers.push(TextureLoadThinker {
                                    target: entity,
                                    request: Some(req.clone()),
                                    target_tex: TargetTex::Normal,
                                });
                            }
                        }
                    }
                    #[allow(unreachable_patterns)]
                    _ => {}
                }
            }
        }

        for node in node.children() {
            process_node(
                world,
                renderer,
                mesh_table,
                tex_table,
                thinkers,
                world_transform,
                &node,
            );
        }
    }

    let root_transform = Mat4::identity();
    if let Some(scene) = document.default_scene() {
        for node in scene.nodes() {
            process_node(
                world,
                renderer,
                &mesh_table,
                &tex_table,
                &mut load_thinkers,
                root_transform,
                &node,
            );
        }
    }

    load_thinkers
}

#[aleph_profile::function]
fn load_index_buffer(
    renderer: &mut Renderer,
    arena: &mut BumpThingy,
    buffers: &[Data],
    indices: &Accessor,
) -> BufferHandle {
    let data_type = indices.data_type();
    assert!(
        matches!(data_type, DataType::U32 | DataType::U16 | DataType::U8),
        "{data_type:?}"
    );
    assert_eq!(indices.dimensions(), Dimensions::Scalar);

    // We only use 32-bit indices
    let size = indices.count() * size_of::<u32>();

    let mut idx_buffer = unsafe {
        arena
            .alloc_upload_desc(size, align_of::<u32>(), ResourceUsageFlags::INDEX_BUFFER)
            .unwrap()
    };

    let dst = idx_buffer.data_mut();
    load_index_buffer_data(bytemuck::cast_slice_mut(dst), buffers, indices);

    renderer.create_buffer(idx_buffer).unwrap()
}

fn load_index_buffer_into_vec(buffers: &[Data], indices: &Accessor) -> Vec<u32> {
    let data_type = indices.data_type();
    assert!(
        matches!(data_type, DataType::U32 | DataType::U16 | DataType::U8),
        "{data_type:?}"
    );
    assert_eq!(indices.dimensions(), Dimensions::Scalar);

    let mut dst = Vec::new();
    dst.resize(indices.count(), 0u32);

    load_index_buffer_data(&mut dst, buffers, indices);

    dst
}

fn load_index_buffer_data(dst: &mut [u32], buffers: &[Data], indices: &Accessor) {
    let data_type = indices.data_type();
    assert!(
        matches!(data_type, DataType::U32 | DataType::U16 | DataType::U8),
        "{data_type:?}"
    );
    assert_eq!(indices.dimensions(), Dimensions::Scalar);

    // We only use 32bit indices
    let size = indices.count() * size_of::<u32>();

    let view = indices.view().unwrap();
    let src = &buffers[view.buffer().index()];
    let offset = view.offset();
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

#[aleph_profile::function]
fn load_vertex_buffer(
    renderer: &mut Renderer,
    arena: &mut BumpThingy,
    buffers: &[Data],
    prim: &Primitive,
) -> BufferHandle {
    // Get the upper bound for number of vertices
    let vertex_count = prim.attributes().map(|v| v.1.count()).max().unwrap();

    // We have an implicit vtx layout for now
    let dst_stride = 60;
    let size = vertex_count * dst_stride;

    let mut vtx_buffer = unsafe {
        arena
            .alloc_upload_desc(size, 256, ResourceUsageFlags::VERTEX_BUFFER)
            .unwrap()
    };

    let dst = &mut vtx_buffer.data_mut()[48..];
    for i in 0..vertex_count {
        let dst_i = dst_stride * i;
        let dst = &mut dst[dst_i..dst_i + 12];
        let dst = bytemuck::cast_slice_mut::<_, f32>(dst);
        dst[0] = 1.0;
        dst[1] = 1.0;
        dst[2] = 1.0;
    }

    let has_texcoord = prim.attributes().any(|(s, _)| s == gltf::Semantic::TexCoords(0));
    let has_normals = prim.attributes().any(|(s, _)| s == gltf::Semantic::Normals);
    let has_tangents = prim
        .attributes()
        .any(|(s, _)| s == gltf::Semantic::Tangents);
    for (semantic, accessor) in prim.attributes() {
        match semantic {
            gltf::Semantic::Positions => {
                copy_vec3_f32_semantic(&mut vtx_buffer, &accessor, buffers, dst_stride, 0);
            }
            gltf::Semantic::Normals => {
                copy_vec3_f32_semantic(&mut vtx_buffer, &accessor, buffers, dst_stride, 20);
            }
            gltf::Semantic::Tangents => {
                copy_vec4_f32_semantic(&mut vtx_buffer, &accessor, buffers, dst_stride, 32);
            }
            gltf::Semantic::Colors(0) => {
                // copy_vec3_f32_semantic(&mut vtx_buffer, &accessor, buffers, dst_stride, 48);
            }
            gltf::Semantic::Colors(_) => {}
            gltf::Semantic::TexCoords(0) => {
                copy_vec2_f32_semantic(&mut vtx_buffer, &accessor, buffers, dst_stride, 12);
            }
            gltf::Semantic::TexCoords(_) => {
                // copy_vec2_f32_semantic(&mut vtx_buffer, &accessor, buffers, dst_stride, ??);
            }
            gltf::Semantic::Joints(_) => unimplemented!(),
            gltf::Semantic::Weights(_) => unimplemented!(),
        }
    }

    if has_texcoord && has_normals && !has_tangents {
        fn get_attr<'a, 'b>(
            buffers: &'b [Data],
            prim: &'a gltf::Primitive<'a>,
            semantic: gltf::Semantic,
        ) -> (gltf::Accessor<'a>, gltf::buffer::View<'a>, &'b [u8]) {
            let (_, attr) = prim.attributes().find(|(s, _)| *s == semantic).unwrap();
            let view = attr.view().unwrap();
            let buffer = &buffers[view.buffer().index()];
            let buffer = &buffer.0[view.offset()..view.offset() + view.length()];
            (attr, view, buffer)
        }
        let (positions, positions_view, positions_buffer) =
            get_attr(buffers, prim, gltf::Semantic::Positions);
        let (normals, normals_view, normals_buffer) =
            get_attr(buffers, prim, gltf::Semantic::Normals);
        let (uvs, uvs_view, uvs_buffer) = get_attr(buffers, prim, gltf::Semantic::TexCoords(0));

        let indices = load_index_buffer_into_vec(buffers, &prim.indices().unwrap());
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
            dst: bytemuck::cast_slice_mut(vtx_buffer.data_mut()),
        };
        let success = mikktspace::generate_tangents(&mut geom);
        assert!(success);
    }

    renderer.create_buffer(vtx_buffer).unwrap()
}

fn copy_vec4_f32_semantic(
    vtx_buffer: &mut BufferUploadSource,
    accessor: &Accessor,
    buffers: &[Data],
    dst_stride: usize,
    dst_offset: usize,
) {
    assert_eq!(accessor.data_type(), DataType::F32);
    assert_eq!(accessor.dimensions(), Dimensions::Vec4);

    copy_vec_f32_semantic(vtx_buffer, accessor, buffers, dst_stride, dst_offset);
}

fn copy_vec3_f32_semantic(
    vtx_buffer: &mut BufferUploadSource,
    accessor: &Accessor,
    buffers: &[Data],
    dst_stride: usize,
    dst_offset: usize,
) {
    assert_eq!(accessor.data_type(), DataType::F32);
    assert_eq!(accessor.dimensions(), Dimensions::Vec3);

    copy_vec_f32_semantic(vtx_buffer, accessor, buffers, dst_stride, dst_offset);
}

fn copy_vec2_f32_semantic(
    vtx_buffer: &mut BufferUploadSource,
    accessor: &Accessor,
    buffers: &[Data],
    dst_stride: usize,
    dst_offset: usize,
) {
    assert_eq!(accessor.data_type(), DataType::F32);
    assert_eq!(accessor.dimensions(), Dimensions::Vec2);

    copy_vec_f32_semantic(vtx_buffer, accessor, buffers, dst_stride, dst_offset);
}

fn copy_vec_f32_semantic(
    vtx_buffer: &mut BufferUploadSource,
    accessor: &Accessor,
    buffers: &[Data],
    dst_stride: usize,
    dst_offset: usize,
) {
    let view = accessor.view().unwrap();
    let e_size = accessor.size();
    let stride = view.stride().unwrap_or(e_size);

    let src = &buffers[view.buffer().index()];

    let src = &src[view.offset()..view.offset() + view.length()];
    let dst = &mut vtx_buffer.data_mut()[dst_offset..];
    for i in 0..accessor.count() {
        let src_i = stride * i;
        let dst_i = dst_stride * i;
        // Copy one element from the source to the dest
        let src = &src[src_i..src_i + e_size];
        dst[dst_i..dst_i + e_size].copy_from_slice(src);
    }
}

fn import_impl(
    gltf::Gltf { document, blob }: gltf::Gltf,
    base: Option<&std::path::Path>,
) -> Option<(gltf::Document, Vec<gltf::buffer::Data>)> {
    let buffer_data = gltf::import_buffers(&document, base, blob).ok()?;
    let import = (document, buffer_data);
    Some(import)
}

fn import_path(path: &std::path::Path) -> Option<(gltf::Document, Vec<gltf::buffer::Data>)> {
    let base = path.parent().unwrap_or_else(|| std::path::Path::new("./"));
    let file = std::fs::File::open(path).ok()?;
    let reader = std::io::BufReader::new(file);
    import_impl(gltf::Gltf::from_reader(reader).ok()?, Some(base))
}

struct Geom<'a> {
    positions: gltf::Accessor<'a>,
    positions_view: gltf::buffer::View<'a>,
    positions_buffer: &'a [u8],
    normals: gltf::Accessor<'a>,
    normals_view: gltf::buffer::View<'a>,
    normals_buffer: &'a [u8],
    uvs: gltf::Accessor<'a>,
    uvs_view: gltf::buffer::View<'a>,
    uvs_buffer: &'a [u8],
    indices: Vec<u32>,
    dst: &'a mut [Vertex],
}

impl<'a> Geom<'a> {
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

impl<'a> mikktspace::Geometry for Geom<'a> {
    fn num_faces(&self) -> usize {
        self.indices.len() / 3
    }

    fn num_vertices_of_face(&self, _face: usize) -> usize {
        3
    }

    fn position(&self, face: usize, vert: usize) -> [f32; 3] {
        debug_assert_eq!(self.positions.data_type(), DataType::F32);
        debug_assert_eq!(self.positions.dimensions(), Dimensions::Vec3);

        let buffer = &self.positions_buffer;
        let accessor = &self.positions;
        let view = &self.positions_view;

        self.get_attribute(buffer, accessor, view, face, vert)
    }

    fn normal(&self, face: usize, vert: usize) -> [f32; 3] {
        debug_assert_eq!(self.normals.data_type(), DataType::F32);
        debug_assert_eq!(self.normals.dimensions(), Dimensions::Vec3);

        let buffer = &self.normals_buffer;
        let accessor = &self.normals;
        let view = &self.normals_view;

        self.get_attribute(buffer, accessor, view, face, vert)
    }

    fn tex_coord(&self, face: usize, vert: usize) -> [f32; 2] {
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
