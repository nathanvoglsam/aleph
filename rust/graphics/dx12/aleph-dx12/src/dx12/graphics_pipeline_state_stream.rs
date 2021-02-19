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
use crate::raw::windows::win32::direct3d12::{
    D3D12_INPUT_ELEMENT_DESC, D3D12_INPUT_LAYOUT_DESC, D3D12_PIPELINE_STATE_FLAGS,
    D3D12_PIPELINE_STATE_SUBOBJECT_TYPE, D3D12_RT_FORMAT_ARRAY, D3D12_SO_DECLARATION_ENTRY,
    D3D12_STREAM_OUTPUT_DESC,
};
use crate::raw::windows::win32::dxgi::DXGI_FORMAT;
use crate::utils::{blob_to_shader, optional_blob_to_cached_pso, optional_blob_to_shader};
use crate::{
    dxgi, BlendDesc, DepthStencilDesc, IndexBufferStripCutValue, InputElementDesc,
    PrimitiveTopologyType, RasterizerDesc, RootSignature, StreamOutputDesc,
};
use std::hash::{Hash, Hasher};
use std::marker::PhantomData;
use std::mem::{size_of, transmute, transmute_copy};
use std::ops::Deref;

pub struct GraphicsPipelineStateStreamBuilder<'a> {
    root_signature: Option<&'a RootSignature>,
    vertex_shader: Option<&'a [u8]>,
    pixel_shader: Option<&'a [u8]>,
    domain_shader: Option<&'a [u8]>,
    hull_shader: Option<&'a [u8]>,
    geometry_shader: Option<&'a [u8]>,
    stream_output: Option<(Vec<D3D12_SO_DECLARATION_ENTRY>, &'a [u32], u32)>,
    blend_state: BlendDesc,
    sample_mask: u32,
    rasterizer_state: RasterizerDesc,
    depth_stencil_state: DepthStencilDesc,
    input_layout: Option<Vec<D3D12_INPUT_ELEMENT_DESC>>,
    strip_cut_value: IndexBufferStripCutValue,
    primitive_topology_type: PrimitiveTopologyType,
    rtv_formats: &'a [dxgi::Format],
    dsv_format: dxgi::Format,
    sample_desc: dxgi::SampleDesc,
    cached_pso: Option<&'a [u8]>,
}

impl<'a> GraphicsPipelineStateStreamBuilder<'a> {
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
            sample_desc: dxgi::SampleDesc::default(),
            cached_pso: None,
        }
    }

    pub fn root_signature(mut self, root_signature: &'a crate::RootSignature) -> Self {
        self.root_signature = Some(root_signature);
        self
    }

    pub fn vertex_shader(mut self, vertex_shader: &'a [u8]) -> Self {
        self.vertex_shader = Some(vertex_shader);
        self
    }

    pub fn pixel_shader(mut self, pixel_shader: &'a [u8]) -> Self {
        self.pixel_shader = Some(pixel_shader);
        self
    }

    pub fn domain_shader(mut self, domain_shader: &'a [u8]) -> Self {
        self.domain_shader = Some(domain_shader);
        self
    }

    pub fn hull_shader(mut self, hull_shader: &'a [u8]) -> Self {
        self.hull_shader = Some(hull_shader);
        self
    }

    pub fn geometry_shader(mut self, geometry_shader: &'a [u8]) -> Self {
        self.geometry_shader = Some(geometry_shader);
        self
    }

    pub fn stream_output(mut self, stream_output: StreamOutputDesc<'a>) -> Self {
        let vec = stream_output
            .so_declarations
            .iter()
            .map(|v| v.clone().into())
            .collect();
        self.stream_output = Some((
            vec,
            stream_output.buffer_strides,
            stream_output.rasterized_stream,
        ));
        self
    }

    pub fn blend_state(mut self, blend_state: BlendDesc) -> Self {
        self.blend_state = blend_state;
        self
    }

    pub fn sample_mask(mut self, sample_mask: u32) -> Self {
        self.sample_mask = sample_mask;
        self
    }

    pub fn rasterizer_state(mut self, rasterizer_state: RasterizerDesc) -> Self {
        self.rasterizer_state = rasterizer_state;
        self
    }

    pub fn depth_stencil_state(mut self, depth_stencil_state: DepthStencilDesc) -> Self {
        self.depth_stencil_state = depth_stencil_state;
        self
    }

    pub fn input_layout(mut self, input_layout: &[InputElementDesc<'a>]) -> Self {
        let desc = input_layout.into_iter().map(|v| v.clone().into()).collect();
        self.input_layout = Some(desc);
        self
    }

    pub fn strip_cut_value(mut self, strip_cut_value: IndexBufferStripCutValue) -> Self {
        self.strip_cut_value = strip_cut_value;
        self
    }

    pub fn primitive_topology_type(
        mut self,
        primitive_topology_type: PrimitiveTopologyType,
    ) -> Self {
        self.primitive_topology_type = primitive_topology_type;
        self
    }

    pub fn rtv_formats(mut self, rtv_formats: &'a [dxgi::Format]) -> Self {
        self.rtv_formats = rtv_formats;
        self
    }

    pub fn dsv_format(mut self, dsv_format: dxgi::Format) -> Self {
        self.dsv_format = dsv_format;
        self
    }

    pub fn sample_desc(mut self, sample_desc: dxgi::SampleDesc) -> Self {
        self.sample_desc = sample_desc;
        self
    }

    pub fn cached_pso(mut self, cached_pso: &'a [u8]) -> Self {
        self.cached_pso = Some(cached_pso.into());
        self
    }

    pub fn build(&self) -> GraphicsPipelineStateStream<'a> {
        type T = D3D12_PIPELINE_STATE_SUBOBJECT_TYPE;

        // Build the render target format array
        let mut rt_formats = [DXGI_FORMAT::default(); 8];
        for i in 0..self.rtv_formats.len() {
            rt_formats[i] = self.rtv_formats[i].into();
        }

        // Get the input layout array
        let input_layout = self.input_layout.as_ref().unwrap();

        let stream_output = self.stream_output.as_ref().unwrap();

        let packed = packed::Packed {
            root_signature: PackedPipelineStateStreamObject::new(
                T::D3D12_PIPELINE_STATE_SUBOBJECT_TYPE_ROOT_SIGNATURE,
                unsafe { transmute_copy(&self.root_signature.unwrap().0) },
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
                D3D12_STREAM_OUTPUT_DESC {
                    p_so_declaration: stream_output.0.as_ptr() as *mut _,
                    num_entries: stream_output.0.len() as _,
                    p_buffer_strides: stream_output.1.as_ptr() as *mut _,
                    num_strides: stream_output.1.len() as _,
                    rasterized_stream: stream_output.2,
                },
            ),
            blend_state: PackedPipelineStateStreamObject::new(
                T::D3D12_PIPELINE_STATE_SUBOBJECT_TYPE_BLEND,
                self.blend_state.clone().into(),
            ),
            sample_mask: PackedPipelineStateStreamObject::new(
                T::D3D12_PIPELINE_STATE_SUBOBJECT_TYPE_SAMPLE_MASK,
                self.sample_mask,
            ),
            rasterizer_state: PackedPipelineStateStreamObject::new(
                T::D3D12_PIPELINE_STATE_SUBOBJECT_TYPE_RASTERIZER,
                self.rasterizer_state.clone().into(),
            ),
            depth_stencil_state: PackedPipelineStateStreamObject::new(
                T::D3D12_PIPELINE_STATE_SUBOBJECT_TYPE_DEPTH_STENCIL,
                self.depth_stencil_state.clone().into(),
            ),
            input_layout: PackedPipelineStateStreamObject::new(
                T::D3D12_PIPELINE_STATE_SUBOBJECT_TYPE_INPUT_LAYOUT,
                D3D12_INPUT_LAYOUT_DESC {
                    p_input_element_descs: input_layout.as_ptr() as *mut _,
                    num_elements: input_layout.len() as _,
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
                    rt_formats,
                    num_render_targets: self.rtv_formats.len() as _,
                },
            ),
            dsv_format: PackedPipelineStateStreamObject::new(
                T::D3D12_PIPELINE_STATE_SUBOBJECT_TYPE_DEPTH_STENCIL_FORMAT,
                self.dsv_format.into(),
            ),
            sample_desc: PackedPipelineStateStreamObject::new(
                T::D3D12_PIPELINE_STATE_SUBOBJECT_TYPE_SAMPLE_DESC,
                self.sample_desc.clone().into(),
            ),
            node_mask: PackedPipelineStateStreamObject::new(
                T::D3D12_PIPELINE_STATE_SUBOBJECT_TYPE_NODE_MASK,
                0,
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

        unsafe {
            GraphicsPipelineStateStream {
                buffer: transmute(packed),
                phantom: Default::default(),
            }
        }
    }
}

#[derive(Clone)]
pub struct GraphicsPipelineStateStream<'a> {
    buffer: [u8; size_of::<packed::Packed>()],
    phantom: PhantomData<&'a ()>,
}

impl<'a> Hash for GraphicsPipelineStateStream<'a> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.buffer.hash(state)
    }
}

impl<'a> GraphicsPipelineStateStream<'a> {
    pub fn builder() -> GraphicsPipelineStateStreamBuilder<'a> {
        GraphicsPipelineStateStreamBuilder::new()
    }
}

impl<'a> Deref for GraphicsPipelineStateStream<'a> {
    type Target = [u8; size_of::<packed::Packed>()];

    fn deref(&self) -> &Self::Target {
        &self.buffer
    }
}

mod packed {
    use crate::dx12::pipeline_state_stream::PackedPipelineStateStreamObject;
    use crate::raw::windows::win32::direct3d12::{
        D3D12_BLEND_DESC, D3D12_CACHED_PIPELINE_STATE, D3D12_DEPTH_STENCIL_DESC,
        D3D12_INDEX_BUFFER_STRIP_CUT_VALUE, D3D12_INPUT_LAYOUT_DESC, D3D12_PIPELINE_STATE_FLAGS,
        D3D12_PRIMITIVE_TOPOLOGY_TYPE, D3D12_RASTERIZER_DESC, D3D12_RT_FORMAT_ARRAY,
        D3D12_SHADER_BYTECODE, D3D12_STREAM_OUTPUT_DESC,
    };
    use crate::raw::windows::win32::dxgi::{DXGI_FORMAT, DXGI_SAMPLE_DESC};
    use std::ffi::c_void;

    pub(crate) type RootSignature = PackedPipelineStateStreamObject<*mut c_void>;
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
