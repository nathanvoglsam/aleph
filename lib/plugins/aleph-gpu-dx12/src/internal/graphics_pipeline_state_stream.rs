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

#![allow(unused)]
#![warn(unused_imports)]

use std::ffi::c_void;
use std::hash::{Hash, Hasher};
use std::marker::PhantomData;
use std::mem::{size_of, ManuallyDrop};
use std::ops::Deref;
use windows::utils::{blob_to_shader, optional_blob_to_cached_pso, optional_blob_to_shader};
use windows::Win32::Graphics::Direct3D12::*;
use windows::Win32::Graphics::Dxgi::Common::*;

pub struct GraphicsPipelineStateStreamBuilder<'a> {
    root_signature: Option<ID3D12RootSignature>,
    vertex_shader: Option<&'a [u8]>,
    pixel_shader: Option<&'a [u8]>,
    domain_shader: Option<&'a [u8]>,
    hull_shader: Option<&'a [u8]>,
    geometry_shader: Option<&'a [u8]>,
    stream_output: D3D12_STREAM_OUTPUT_DESC,
    blend_state: D3D12_BLEND_DESC,
    sample_mask: u32,
    rasterizer_state: D3D12_RASTERIZER_DESC,
    depth_stencil_state: D3D12_DEPTH_STENCIL_DESC,
    input_layout: Option<&'a [D3D12_INPUT_ELEMENT_DESC]>,
    strip_cut_value: D3D12_INDEX_BUFFER_STRIP_CUT_VALUE,
    primitive_topology_type: D3D12_PRIMITIVE_TOPOLOGY_TYPE,
    rtv_formats: &'a [DXGI_FORMAT],
    dsv_format: DXGI_FORMAT,
    sample_desc: DXGI_SAMPLE_DESC,
    cached_pso: Option<&'a [u8]>,
    node_mask: u32,
}

impl<'a> GraphicsPipelineStateStreamBuilder<'a> {
    #[inline]
    pub fn new() -> Self {
        Self {
            root_signature: None,
            vertex_shader: None,
            pixel_shader: None,
            domain_shader: None,
            hull_shader: None,
            geometry_shader: None,
            stream_output: D3D12_STREAM_OUTPUT_DESC::default(),
            blend_state: D3D12_BLEND_DESC::default(),
            sample_mask: 0,
            rasterizer_state: D3D12_RASTERIZER_DESC::default(),
            depth_stencil_state: D3D12_DEPTH_STENCIL_DESC::default(),
            input_layout: None,
            strip_cut_value: D3D12_INDEX_BUFFER_STRIP_CUT_VALUE::default(),
            primitive_topology_type: D3D12_PRIMITIVE_TOPOLOGY_TYPE::default(),
            rtv_formats: &[],
            dsv_format: DXGI_FORMAT::default(),
            sample_desc: DXGI_SAMPLE_DESC {
                Count: 1,
                Quality: 0,
            },
            cached_pso: None,
            node_mask: 0,
        }
    }

    #[inline]
    pub fn root_signature(mut self, root_signature: ID3D12RootSignature) -> Self {
        self.root_signature = Some(root_signature);
        self
    }

    #[inline]
    pub fn vertex_shader(mut self, vertex_shader: &'a [u8]) -> Self {
        self.vertex_shader = Some(vertex_shader);
        self
    }

    #[inline]
    pub fn pixel_shader(mut self, pixel_shader: &'a [u8]) -> Self {
        self.pixel_shader = Some(pixel_shader);
        self
    }

    #[inline]
    pub fn domain_shader(mut self, domain_shader: &'a [u8]) -> Self {
        self.domain_shader = Some(domain_shader);
        self
    }

    #[inline]
    pub fn hull_shader(mut self, hull_shader: &'a [u8]) -> Self {
        self.hull_shader = Some(hull_shader);
        self
    }

    #[inline]
    pub fn geometry_shader(mut self, geometry_shader: &'a [u8]) -> Self {
        self.geometry_shader = Some(geometry_shader);
        self
    }

    #[inline]
    pub fn stream_output(mut self, stream_output: D3D12_STREAM_OUTPUT_DESC) -> Self {
        self.stream_output = stream_output;
        self
    }

    #[inline]
    pub fn blend_state(mut self, blend_state: D3D12_BLEND_DESC) -> Self {
        self.blend_state = blend_state;
        self
    }

    #[inline]
    pub fn sample_mask(mut self, sample_mask: u32) -> Self {
        self.sample_mask = sample_mask;
        self
    }

    #[inline]
    pub fn rasterizer_state(mut self, rasterizer_state: D3D12_RASTERIZER_DESC) -> Self {
        self.rasterizer_state = rasterizer_state;
        self
    }

    #[inline]
    pub fn depth_stencil_state(mut self, depth_stencil_state: D3D12_DEPTH_STENCIL_DESC) -> Self {
        self.depth_stencil_state = depth_stencil_state;
        self
    }

    #[inline]
    pub fn input_layout(mut self, input_layout: &'a [D3D12_INPUT_ELEMENT_DESC]) -> Self {
        self.input_layout = Some(input_layout);
        self
    }

    #[inline]
    pub fn strip_cut_value(mut self, strip_cut_value: D3D12_INDEX_BUFFER_STRIP_CUT_VALUE) -> Self {
        self.strip_cut_value = strip_cut_value;
        self
    }

    #[inline]
    pub fn primitive_topology_type(
        mut self,
        primitive_topology_type: D3D12_PRIMITIVE_TOPOLOGY_TYPE,
    ) -> Self {
        self.primitive_topology_type = primitive_topology_type;
        self
    }

    #[inline]
    pub fn rtv_formats(mut self, rtv_formats: &'a [DXGI_FORMAT]) -> Self {
        self.rtv_formats = rtv_formats;
        self
    }

    #[inline]
    pub fn dsv_format(mut self, dsv_format: DXGI_FORMAT) -> Self {
        self.dsv_format = dsv_format;
        self
    }

    #[inline]
    pub fn sample_desc(mut self, sample_desc: DXGI_SAMPLE_DESC) -> Self {
        self.sample_desc = sample_desc;
        self
    }

    #[inline]
    pub fn cached_pso(mut self, cached_pso: &'a [u8]) -> Self {
        self.cached_pso = Some(cached_pso);
        self
    }

    #[inline]
    pub fn node_mask(mut self, node_mask: u32) -> Self {
        self.node_mask = node_mask;
        self
    }

    #[inline]
    pub fn build(self) -> GraphicsPipelineStateStream<'a> {
        // Build the render target format array
        let mut rt_formats = [DXGI_FORMAT::default(); 8];
        for (i, item) in self.rtv_formats.iter().enumerate() {
            rt_formats[i] = *item;
        }

        // Get the input layout array
        let input_layout = self.input_layout.unwrap();

        let packed = packed::Packed {
            root_signature: self.root_signature.into(),
            vertex_shader: blob_to_shader(self.vertex_shader.unwrap()).into(),
            pixel_shader: blob_to_shader(self.pixel_shader.unwrap()).into(),
            domain_shader: optional_blob_to_shader(self.domain_shader).into(),
            hull_shader: optional_blob_to_shader(self.hull_shader).into(),
            geometry_shader: optional_blob_to_shader(self.geometry_shader).into(),
            stream_output: self.stream_output.into(),
            blend_state: self.blend_state.into(),
            sample_mask: self.sample_mask.into(),
            rasterizer_state: self.rasterizer_state.into(),
            depth_stencil_state: self.depth_stencil_state.into(),
            input_layout: D3D12_INPUT_LAYOUT_DESC {
                pInputElementDescs: input_layout.as_ptr() as *mut _,
                NumElements: input_layout.len() as _,
            }
            .into(),
            strip_cut_value: self.strip_cut_value.into(),
            primitive_topology_type: self.primitive_topology_type.into(),
            render_targets: D3D12_RT_FORMAT_ARRAY {
                RTFormats: rt_formats,
                NumRenderTargets: self.rtv_formats.len() as _,
            }
            .into(),
            dsv_format: self.dsv_format.into(),
            sample_desc: self.sample_desc.into(),
            node_mask: self.node_mask.into(),
            cached_pso: optional_blob_to_cached_pso(self.cached_pso).into(),
            flags: D3D12_PIPELINE_STATE_FLAG_NONE.into(),
        };

        GraphicsPipelineStateStream {
            packed,
            phantom: Default::default(),
        }
    }
}

impl<'a> Default for GraphicsPipelineStateStreamBuilder<'a> {
    #[inline]
    fn default() -> Self {
        Self::new()
    }
}

pub struct GraphicsPipelineStateStream<'a> {
    packed: packed::Packed,
    phantom: PhantomData<&'a ()>,
}

impl<'a> GraphicsPipelineStateStream<'a> {
    #[inline]
    pub fn builder() -> GraphicsPipelineStateStreamBuilder<'a> {
        GraphicsPipelineStateStreamBuilder::new()
    }
}

impl<'a> Hash for GraphicsPipelineStateStream<'a> {
    #[inline]
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.deref().hash(state)
    }
}

impl<'a> Deref for GraphicsPipelineStateStream<'a> {
    type Target = [u8];

    #[inline]
    fn deref(&self) -> &Self::Target {
        unsafe {
            std::slice::from_raw_parts(
                &self.packed as *const packed::Packed as *const u8,
                size_of::<packed::Packed>(),
            )
        }
    }
}

#[repr(transparent)]
pub(crate) struct PackedPipelineStateStreamObject<T, const I: i32>(AlignmentWrapper<Packed<T, I>>);

impl<T, const I: i32> From<T> for PackedPipelineStateStreamObject<T, I> {
    #[inline]
    fn from(object: T) -> Self {
        let out = AlignmentWrapper {
            wrapped: ManuallyDrop::new(Packed {
                object_type: D3D12_PIPELINE_STATE_SUBOBJECT_TYPE(I),
                object,
            }),
        };
        Self(out)
    }
}

#[repr(C)]
struct Packed<T, const I: i32> {
    pub object_type: D3D12_PIPELINE_STATE_SUBOBJECT_TYPE,
    pub object: T,
}

#[repr(C)]
union AlignmentWrapper<T> {
    wrapped: ManuallyDrop<T>,
    alignment: *const c_void,
}

impl<T> Drop for AlignmentWrapper<T> {
    #[inline]
    fn drop(&mut self) {
        unsafe { ManuallyDrop::drop(&mut self.wrapped) };
    }
}

mod packed {
    use super::PackedPipelineStateStreamObject;
    use windows::Win32::Graphics::Direct3D12::*;
    use windows::Win32::Graphics::Dxgi::Common::*;

    pub(crate) type RootSignature = PackedPipelineStateStreamObject<
        Option<ID3D12RootSignature>,
        { D3D12_PIPELINE_STATE_SUBOBJECT_TYPE_ROOT_SIGNATURE.0 },
    >;
    pub(crate) type VertexShader = PackedPipelineStateStreamObject<
        D3D12_SHADER_BYTECODE,
        { D3D12_PIPELINE_STATE_SUBOBJECT_TYPE_VS.0 },
    >;
    pub(crate) type PixelShader = PackedPipelineStateStreamObject<
        D3D12_SHADER_BYTECODE,
        { D3D12_PIPELINE_STATE_SUBOBJECT_TYPE_PS.0 },
    >;
    pub(crate) type DomainShader = PackedPipelineStateStreamObject<
        D3D12_SHADER_BYTECODE,
        { D3D12_PIPELINE_STATE_SUBOBJECT_TYPE_DS.0 },
    >;
    pub(crate) type HullShader = PackedPipelineStateStreamObject<
        D3D12_SHADER_BYTECODE,
        { D3D12_PIPELINE_STATE_SUBOBJECT_TYPE_HS.0 },
    >;
    pub(crate) type GeometryShader = PackedPipelineStateStreamObject<
        D3D12_SHADER_BYTECODE,
        { D3D12_PIPELINE_STATE_SUBOBJECT_TYPE_GS.0 },
    >;
    pub(crate) type BlendState = PackedPipelineStateStreamObject<
        D3D12_BLEND_DESC,
        { D3D12_PIPELINE_STATE_SUBOBJECT_TYPE_BLEND.0 },
    >;
    pub(crate) type SampleMask =
        PackedPipelineStateStreamObject<u32, { D3D12_PIPELINE_STATE_SUBOBJECT_TYPE_SAMPLE_MASK.0 }>;
    pub(crate) type RasterizerState = PackedPipelineStateStreamObject<
        D3D12_RASTERIZER_DESC,
        { D3D12_PIPELINE_STATE_SUBOBJECT_TYPE_RASTERIZER.0 },
    >;
    pub(crate) type DepthStencilState = PackedPipelineStateStreamObject<
        D3D12_DEPTH_STENCIL_DESC,
        { D3D12_PIPELINE_STATE_SUBOBJECT_TYPE_DEPTH_STENCIL.0 },
    >;
    pub(crate) type PrimitiveTopologyType = PackedPipelineStateStreamObject<
        D3D12_PRIMITIVE_TOPOLOGY_TYPE,
        { D3D12_PIPELINE_STATE_SUBOBJECT_TYPE_PRIMITIVE_TOPOLOGY.0 },
    >;
    pub(crate) type RenderTargets = PackedPipelineStateStreamObject<
        D3D12_RT_FORMAT_ARRAY,
        { D3D12_PIPELINE_STATE_SUBOBJECT_TYPE_RENDER_TARGET_FORMATS.0 },
    >;
    pub(crate) type DsvFormat = PackedPipelineStateStreamObject<
        DXGI_FORMAT,
        { D3D12_PIPELINE_STATE_SUBOBJECT_TYPE_DEPTH_STENCIL_FORMAT.0 },
    >;
    pub(crate) type SampleDesc = PackedPipelineStateStreamObject<
        DXGI_SAMPLE_DESC,
        { D3D12_PIPELINE_STATE_SUBOBJECT_TYPE_SAMPLE_DESC.0 },
    >;
    pub(crate) type NodeMask =
        PackedPipelineStateStreamObject<u32, { D3D12_PIPELINE_STATE_SUBOBJECT_TYPE_NODE_MASK.0 }>;
    pub(crate) type CachedPso = PackedPipelineStateStreamObject<
        D3D12_CACHED_PIPELINE_STATE,
        { D3D12_PIPELINE_STATE_SUBOBJECT_TYPE_CACHED_PSO.0 },
    >;
    pub(crate) type Flags = PackedPipelineStateStreamObject<
        D3D12_PIPELINE_STATE_FLAGS,
        { D3D12_PIPELINE_STATE_SUBOBJECT_TYPE_FLAGS.0 },
    >;
    pub(crate) type StreamOutput = PackedPipelineStateStreamObject<
        D3D12_STREAM_OUTPUT_DESC,
        { D3D12_PIPELINE_STATE_SUBOBJECT_TYPE_STREAM_OUTPUT.0 },
    >;
    pub(crate) type InputLayout = PackedPipelineStateStreamObject<
        D3D12_INPUT_LAYOUT_DESC,
        { D3D12_PIPELINE_STATE_SUBOBJECT_TYPE_INPUT_LAYOUT.0 },
    >;
    pub(crate) type StripCutValue = PackedPipelineStateStreamObject<
        D3D12_INDEX_BUFFER_STRIP_CUT_VALUE,
        { D3D12_PIPELINE_STATE_SUBOBJECT_TYPE_IB_STRIP_CUT_VALUE.0 },
    >;
    #[repr(C)]
    pub(crate) struct Packed {
        pub root_signature: RootSignature,
        pub vertex_shader: VertexShader,
        pub pixel_shader: PixelShader,
        pub domain_shader: DomainShader,
        pub hull_shader: HullShader,
        pub geometry_shader: GeometryShader,
        pub stream_output: StreamOutput,
        pub blend_state: BlendState,
        pub sample_mask: SampleMask,
        pub rasterizer_state: RasterizerState,
        pub depth_stencil_state: DepthStencilState,
        pub input_layout: InputLayout,
        pub strip_cut_value: StripCutValue,
        pub primitive_topology_type: PrimitiveTopologyType,
        pub render_targets: RenderTargets,
        pub dsv_format: DsvFormat,
        pub sample_desc: SampleDesc,
        pub node_mask: NodeMask,
        pub cached_pso: CachedPso,
        pub flags: Flags,
    }
}
