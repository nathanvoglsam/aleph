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

use crate::dx12::pipeline_state_stream::PackedPipelineStateStreamObject;
use crate::{
    dxgi, BlendDesc, DepthStencilDesc, IndexBufferStripCutValue, InputElementDesc,
    PrimitiveTopologyType, RasterizerDesc, RootSignature, StreamOutputDeclaration,
    StreamOutputDesc,
};
use std::hash::{Hash, Hasher};
use std::marker::PhantomData;
use std::mem::{align_of, size_of, transmute};
use std::ops::Deref;
use windows_raw::utils::{blob_to_shader, optional_blob_to_cached_pso, optional_blob_to_shader};
use windows_raw::Win32::Direct3D12::{
    D3D12_INPUT_ELEMENT_DESC, D3D12_INPUT_LAYOUT_DESC, D3D12_PIPELINE_STATE_FLAGS,
    D3D12_PIPELINE_STATE_SUBOBJECT_TYPE, D3D12_RT_FORMAT_ARRAY, D3D12_SO_DECLARATION_ENTRY,
    D3D12_STREAM_OUTPUT_DESC,
};
use windows_raw::Win32::Dxgi::DXGI_FORMAT;

pub struct GraphicsPipelineStateStreamBuilder<'a> {
    root_signature: Option<RootSignature>,
    vertex_shader: Option<&'a [u8]>,
    pixel_shader: Option<&'a [u8]>,
    domain_shader: Option<&'a [u8]>,
    hull_shader: Option<&'a [u8]>,
    geometry_shader: Option<&'a [u8]>,
    stream_output: Option<(&'a [StreamOutputDeclaration<'a>], &'a [u32], u32)>,
    blend_state: BlendDesc,
    sample_mask: u32,
    rasterizer_state: RasterizerDesc,
    depth_stencil_state: DepthStencilDesc,
    input_layout: Option<&'a [InputElementDesc<'a>]>,
    strip_cut_value: IndexBufferStripCutValue,
    primitive_topology_type: PrimitiveTopologyType,
    rtv_formats: &'a [dxgi::Format],
    dsv_format: dxgi::Format,
    sample_desc: dxgi::SampleDesc,
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
            stream_output: None,
            blend_state: BlendDesc::default(),
            sample_mask: 0,
            rasterizer_state: RasterizerDesc::default(),
            depth_stencil_state: DepthStencilDesc::default(),
            input_layout: None,
            strip_cut_value: IndexBufferStripCutValue::default(),
            primitive_topology_type: PrimitiveTopologyType::default(),
            rtv_formats: &[],
            dsv_format: dxgi::Format::default(),
            sample_desc: dxgi::SampleDesc {
                count: 1,
                quality: 0,
            },
            cached_pso: None,
            node_mask: 0,
        }
    }

    #[inline]
    pub fn root_signature(mut self, root_signature: crate::RootSignature) -> Self {
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
    pub fn stream_output(mut self, stream_output: StreamOutputDesc<'a>) -> Self {
        assert_eq!(
            size_of::<StreamOutputDeclaration>(),
            size_of::<D3D12_SO_DECLARATION_ENTRY>()
        );
        assert_eq!(
            align_of::<StreamOutputDeclaration>(),
            align_of::<D3D12_SO_DECLARATION_ENTRY>()
        );
        self.stream_output = Some((
            stream_output.so_declarations,
            stream_output.buffer_strides,
            stream_output.rasterized_stream,
        ));
        self
    }

    #[inline]
    pub fn blend_state(mut self, blend_state: BlendDesc) -> Self {
        self.blend_state = blend_state;
        self
    }

    #[inline]
    pub fn sample_mask(mut self, sample_mask: u32) -> Self {
        self.sample_mask = sample_mask;
        self
    }

    #[inline]
    pub fn rasterizer_state(mut self, rasterizer_state: RasterizerDesc) -> Self {
        self.rasterizer_state = rasterizer_state;
        self
    }

    #[inline]
    pub fn depth_stencil_state(mut self, depth_stencil_state: DepthStencilDesc) -> Self {
        self.depth_stencil_state = depth_stencil_state;
        self
    }

    #[inline]
    pub fn input_layout(mut self, input_layout: &'a [InputElementDesc<'a>]) -> Self {
        assert_eq!(
            size_of::<InputElementDesc>(),
            size_of::<D3D12_INPUT_ELEMENT_DESC>()
        );
        assert_eq!(
            align_of::<InputElementDesc>(),
            align_of::<D3D12_INPUT_ELEMENT_DESC>()
        );
        self.input_layout = Some(input_layout);
        self
    }

    #[inline]
    pub fn strip_cut_value(mut self, strip_cut_value: IndexBufferStripCutValue) -> Self {
        self.strip_cut_value = strip_cut_value;
        self
    }

    #[inline]
    pub fn primitive_topology_type(
        mut self,
        primitive_topology_type: PrimitiveTopologyType,
    ) -> Self {
        self.primitive_topology_type = primitive_topology_type;
        self
    }

    #[inline]
    pub fn rtv_formats(mut self, rtv_formats: &'a [dxgi::Format]) -> Self {
        self.rtv_formats = rtv_formats;
        self
    }

    #[inline]
    pub fn dsv_format(mut self, dsv_format: dxgi::Format) -> Self {
        self.dsv_format = dsv_format;
        self
    }

    #[inline]
    pub fn sample_desc(mut self, sample_desc: dxgi::SampleDesc) -> Self {
        self.sample_desc = sample_desc;
        self
    }

    #[inline]
    pub fn cached_pso(mut self, cached_pso: &'a [u8]) -> Self {
        self.cached_pso = Some(cached_pso.into());
        self
    }

    #[inline]
    pub fn node_mask(mut self, node_mask: u32) -> Self {
        self.node_mask = node_mask;
        self
    }

    pub fn build(self) -> GraphicsPipelineStateStream<'a> {
        type T = D3D12_PIPELINE_STATE_SUBOBJECT_TYPE;

        // Build the render target format array
        let mut rt_formats = [DXGI_FORMAT::default(); 8];
        for i in 0..self.rtv_formats.len() {
            rt_formats[i] = self.rtv_formats[i].into();
        }

        // Get the input layout array
        let input_layout = self.input_layout.unwrap();

        let stream_output = if let Some(stream_output) = self.stream_output {
            D3D12_STREAM_OUTPUT_DESC {
                pSODeclaration: stream_output.0.as_ptr() as *mut _,
                NumEntries: stream_output.0.len() as _,
                pBufferStrides: stream_output.1.as_ptr() as *mut _,
                NumStrides: stream_output.1.len() as _,
                RasterizedStream: stream_output.2,
            }
        } else {
            D3D12_STREAM_OUTPUT_DESC {
                pSODeclaration: std::ptr::null_mut(),
                NumEntries: 0,
                pBufferStrides: std::ptr::null_mut(),
                NumStrides: 0,
                RasterizedStream: 0,
            }
        };

        let packed = packed::Packed {
            root_signature: PackedPipelineStateStreamObject::new(
                T::D3D12_PIPELINE_STATE_SUBOBJECT_TYPE_ROOT_SIGNATURE,
                self.root_signature.clone().map(|v| v.0),
            ),
            vertex_shader: PackedPipelineStateStreamObject::new(
                T::D3D12_PIPELINE_STATE_SUBOBJECT_TYPE_VS,
                blob_to_shader(self.vertex_shader.unwrap()),
            ),
            pixel_shader: PackedPipelineStateStreamObject::new(
                T::D3D12_PIPELINE_STATE_SUBOBJECT_TYPE_PS,
                blob_to_shader(self.pixel_shader.unwrap()),
            ),
            domain_shader: PackedPipelineStateStreamObject::new(
                T::D3D12_PIPELINE_STATE_SUBOBJECT_TYPE_DS,
                optional_blob_to_shader(self.domain_shader),
            ),
            hull_shader: PackedPipelineStateStreamObject::new(
                T::D3D12_PIPELINE_STATE_SUBOBJECT_TYPE_HS,
                optional_blob_to_shader(self.hull_shader),
            ),
            geometry_shader: PackedPipelineStateStreamObject::new(
                T::D3D12_PIPELINE_STATE_SUBOBJECT_TYPE_GS,
                optional_blob_to_shader(self.geometry_shader),
            ),
            stream_output: PackedPipelineStateStreamObject::new(
                T::D3D12_PIPELINE_STATE_SUBOBJECT_TYPE_STREAM_OUTPUT,
                stream_output,
            ),
            blend_state: PackedPipelineStateStreamObject::new(
                T::D3D12_PIPELINE_STATE_SUBOBJECT_TYPE_BLEND,
                unsafe { transmute(self.blend_state.clone()) },
            ),
            sample_mask: PackedPipelineStateStreamObject::new(
                T::D3D12_PIPELINE_STATE_SUBOBJECT_TYPE_SAMPLE_MASK,
                self.sample_mask,
            ),
            rasterizer_state: PackedPipelineStateStreamObject::new(
                T::D3D12_PIPELINE_STATE_SUBOBJECT_TYPE_RASTERIZER,
                unsafe { transmute(self.rasterizer_state.clone()) },
            ),
            depth_stencil_state: PackedPipelineStateStreamObject::new(
                T::D3D12_PIPELINE_STATE_SUBOBJECT_TYPE_DEPTH_STENCIL,
                unsafe { transmute(self.depth_stencil_state.clone()) },
            ),
            input_layout: PackedPipelineStateStreamObject::new(
                T::D3D12_PIPELINE_STATE_SUBOBJECT_TYPE_INPUT_LAYOUT,
                D3D12_INPUT_LAYOUT_DESC {
                    pInputElementDescs: input_layout.as_ptr() as *mut _,
                    NumElements: input_layout.len() as _,
                },
            ),
            strip_cut_value: PackedPipelineStateStreamObject::new(
                T::D3D12_PIPELINE_STATE_SUBOBJECT_TYPE_IB_STRIP_CUT_VALUE,
                self.strip_cut_value.into(),
            ),
            primitive_topology_type: PackedPipelineStateStreamObject::new(
                T::D3D12_PIPELINE_STATE_SUBOBJECT_TYPE_PRIMITIVE_TOPOLOGY,
                self.primitive_topology_type.into(),
            ),
            render_targets: PackedPipelineStateStreamObject::new(
                T::D3D12_PIPELINE_STATE_SUBOBJECT_TYPE_RENDER_TARGET_FORMATS,
                D3D12_RT_FORMAT_ARRAY {
                    RTFormats: rt_formats,
                    NumRenderTargets: self.rtv_formats.len() as _,
                },
            ),
            dsv_format: PackedPipelineStateStreamObject::new(
                T::D3D12_PIPELINE_STATE_SUBOBJECT_TYPE_DEPTH_STENCIL_FORMAT,
                self.dsv_format.into(),
            ),
            sample_desc: PackedPipelineStateStreamObject::new(
                T::D3D12_PIPELINE_STATE_SUBOBJECT_TYPE_SAMPLE_DESC,
                unsafe { transmute(self.sample_desc.clone()) },
            ),
            node_mask: PackedPipelineStateStreamObject::new(
                T::D3D12_PIPELINE_STATE_SUBOBJECT_TYPE_NODE_MASK,
                self.node_mask,
            ),
            cached_pso: PackedPipelineStateStreamObject::new(
                T::D3D12_PIPELINE_STATE_SUBOBJECT_TYPE_CACHED_PSO,
                optional_blob_to_cached_pso(self.cached_pso),
            ),
            flags: PackedPipelineStateStreamObject::new(
                T::D3D12_PIPELINE_STATE_SUBOBJECT_TYPE_FLAGS,
                D3D12_PIPELINE_STATE_FLAGS::D3D12_PIPELINE_STATE_FLAG_NONE,
            ),
        };

        GraphicsPipelineStateStream {
            packed,
            phantom: Default::default(),
        }
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

mod packed {
    use crate::dx12::pipeline_state_stream::PackedPipelineStateStreamObject;
    use windows_raw::Win32::Direct3D12::{
        ID3D12RootSignature, D3D12_BLEND_DESC, D3D12_CACHED_PIPELINE_STATE,
        D3D12_DEPTH_STENCIL_DESC, D3D12_INDEX_BUFFER_STRIP_CUT_VALUE, D3D12_INPUT_LAYOUT_DESC,
        D3D12_PIPELINE_STATE_FLAGS, D3D12_PRIMITIVE_TOPOLOGY_TYPE, D3D12_RASTERIZER_DESC,
        D3D12_RT_FORMAT_ARRAY, D3D12_SHADER_BYTECODE, D3D12_STREAM_OUTPUT_DESC,
    };
    use windows_raw::Win32::Dxgi::{DXGI_FORMAT, DXGI_SAMPLE_DESC};

    pub(crate) type RootSignature = PackedPipelineStateStreamObject<Option<ID3D12RootSignature>>;
    pub(crate) type VertexShader = PackedPipelineStateStreamObject<D3D12_SHADER_BYTECODE>;
    pub(crate) type PixelShader = PackedPipelineStateStreamObject<D3D12_SHADER_BYTECODE>;
    pub(crate) type DomainShader = PackedPipelineStateStreamObject<D3D12_SHADER_BYTECODE>;
    pub(crate) type HullShader = PackedPipelineStateStreamObject<D3D12_SHADER_BYTECODE>;
    pub(crate) type GeometryShader = PackedPipelineStateStreamObject<D3D12_SHADER_BYTECODE>;
    pub(crate) type BlendState = PackedPipelineStateStreamObject<D3D12_BLEND_DESC>;
    pub(crate) type SampleMask = PackedPipelineStateStreamObject<u32>;
    pub(crate) type RasterizerState = PackedPipelineStateStreamObject<D3D12_RASTERIZER_DESC>;
    pub(crate) type DepthStencilState = PackedPipelineStateStreamObject<D3D12_DEPTH_STENCIL_DESC>;
    pub(crate) type PrimitiveTopologyType =
        PackedPipelineStateStreamObject<D3D12_PRIMITIVE_TOPOLOGY_TYPE>;
    pub(crate) type RenderTargets = PackedPipelineStateStreamObject<D3D12_RT_FORMAT_ARRAY>;
    pub(crate) type DsvFormat = PackedPipelineStateStreamObject<DXGI_FORMAT>;
    pub(crate) type SampleDesc = PackedPipelineStateStreamObject<DXGI_SAMPLE_DESC>;
    pub(crate) type NodeMask = PackedPipelineStateStreamObject<u32>;
    pub(crate) type CachedPso = PackedPipelineStateStreamObject<D3D12_CACHED_PIPELINE_STATE>;
    pub(crate) type Flags = PackedPipelineStateStreamObject<D3D12_PIPELINE_STATE_FLAGS>;
    pub(crate) type StreamOutput = PackedPipelineStateStreamObject<D3D12_STREAM_OUTPUT_DESC>;
    pub(crate) type InputLayout = PackedPipelineStateStreamObject<D3D12_INPUT_LAYOUT_DESC>;
    pub(crate) type StripCutValue =
        PackedPipelineStateStreamObject<D3D12_INDEX_BUFFER_STRIP_CUT_VALUE>;
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
