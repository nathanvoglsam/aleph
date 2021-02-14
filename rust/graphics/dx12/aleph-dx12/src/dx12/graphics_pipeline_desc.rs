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
    ID3D12RootSignature, D3D12_BLEND_DESC, D3D12_CACHED_PIPELINE_STATE, D3D12_DEPTH_STENCIL_DESC,
    D3D12_INDEX_BUFFER_STRIP_CUT_VALUE, D3D12_INPUT_LAYOUT_DESC, D3D12_PIPELINE_STATE_FLAGS,
    D3D12_PIPELINE_STATE_SUBOBJECT_TYPE, D3D12_PRIMITIVE_TOPOLOGY_TYPE, D3D12_RASTERIZER_DESC,
    D3D12_RT_FORMAT_ARRAY, D3D12_SHADER_BYTECODE, D3D12_SIMULTANEOUS_RENDER_TARGET_COUNT,
    D3D12_STREAM_OUTPUT_DESC,
};
use crate::raw::windows::win32::dxgi::{DXGI_FORMAT, DXGI_SAMPLE_DESC};
use crate::ToPipelineStateStream;
use std::ffi::c_void;
use std::mem::transmute;

#[derive(Clone, Debug, Default)]
pub struct GraphicsPipelineStateDesc {
    pub root_signature: Option<ID3D12RootSignature>,
    pub vertex_shader: D3D12_SHADER_BYTECODE,
    pub pixel_shader: D3D12_SHADER_BYTECODE,
    pub domain_shader: D3D12_SHADER_BYTECODE,
    pub hull_shader: D3D12_SHADER_BYTECODE,
    pub geometry_shader: D3D12_SHADER_BYTECODE,
    pub stream_output: D3D12_STREAM_OUTPUT_DESC,
    pub blend_state: D3D12_BLEND_DESC,
    pub sample_mask: u32,
    pub rasterizer_state: D3D12_RASTERIZER_DESC,
    pub depth_stencil_state: D3D12_DEPTH_STENCIL_DESC,
    pub input_layout: D3D12_INPUT_LAYOUT_DESC,
    pub strip_cut_value: D3D12_INDEX_BUFFER_STRIP_CUT_VALUE,
    pub primitive_topology_type: D3D12_PRIMITIVE_TOPOLOGY_TYPE,
    pub num_render_targets: u32,
    pub rtv_formats: [DXGI_FORMAT; D3D12_SIMULTANEOUS_RENDER_TARGET_COUNT as _],
    pub dsv_format: DXGI_FORMAT,
    pub sample_desc: DXGI_SAMPLE_DESC,
    pub node_mask: u32,
    pub cached_pso: D3D12_CACHED_PIPELINE_STATE,
    pub flags: D3D12_PIPELINE_STATE_FLAGS,
}

impl ToPipelineStateStream for GraphicsPipelineStateDesc {
    type Buffer = [u8; std::mem::size_of::<Packed>()];

    fn into_pipeline_state_stream(self) -> Self::Buffer {
        type T = D3D12_PIPELINE_STATE_SUBOBJECT_TYPE;

        let packed = Packed {
            root_signature: PackedPipelineStateStreamObject::new(
                T::D3D12_PIPELINE_STATE_SUBOBJECT_TYPE_ROOT_SIGNATURE,
                unsafe { transmute(self.root_signature) },
            ),
            vertex_shader: PackedPipelineStateStreamObject::new(
                T::D3D12_PIPELINE_STATE_SUBOBJECT_TYPE_VS,
                self.vertex_shader,
            ),
            pixel_shader: PackedPipelineStateStreamObject::new(
                T::D3D12_PIPELINE_STATE_SUBOBJECT_TYPE_PS,
                self.pixel_shader,
            ),
            domain_shader: PackedPipelineStateStreamObject::new(
                T::D3D12_PIPELINE_STATE_SUBOBJECT_TYPE_DS,
                self.domain_shader,
            ),
            hull_shader: PackedPipelineStateStreamObject::new(
                T::D3D12_PIPELINE_STATE_SUBOBJECT_TYPE_HS,
                self.hull_shader,
            ),
            geometry_shader: PackedPipelineStateStreamObject::new(
                T::D3D12_PIPELINE_STATE_SUBOBJECT_TYPE_GS,
                self.geometry_shader,
            ),
            stream_output: PackedPipelineStateStreamObject::new(
                T::D3D12_PIPELINE_STATE_SUBOBJECT_TYPE_STREAM_OUTPUT,
                self.stream_output,
            ),
            blend_state: PackedPipelineStateStreamObject::new(
                T::D3D12_PIPELINE_STATE_SUBOBJECT_TYPE_BLEND,
                self.blend_state,
            ),
            sample_mask: PackedPipelineStateStreamObject::new(
                T::D3D12_PIPELINE_STATE_SUBOBJECT_TYPE_SAMPLE_MASK,
                self.sample_mask,
            ),
            rasterizer_state: PackedPipelineStateStreamObject::new(
                T::D3D12_PIPELINE_STATE_SUBOBJECT_TYPE_RASTERIZER,
                self.rasterizer_state,
            ),
            depth_stencil_state: PackedPipelineStateStreamObject::new(
                T::D3D12_PIPELINE_STATE_SUBOBJECT_TYPE_DEPTH_STENCIL,
                self.depth_stencil_state,
            ),
            input_layout: PackedPipelineStateStreamObject::new(
                T::D3D12_PIPELINE_STATE_SUBOBJECT_TYPE_INPUT_LAYOUT,
                self.input_layout,
            ),
            strip_cut_value: PackedPipelineStateStreamObject::new(
                T::D3D12_PIPELINE_STATE_SUBOBJECT_TYPE_IB_STRIP_CUT_VALUE,
                self.strip_cut_value,
            ),
            primitive_topology_type: PackedPipelineStateStreamObject::new(
                T::D3D12_PIPELINE_STATE_SUBOBJECT_TYPE_PRIMITIVE_TOPOLOGY,
                self.primitive_topology_type,
            ),
            render_targets: PackedPipelineStateStreamObject::new(
                T::D3D12_PIPELINE_STATE_SUBOBJECT_TYPE_RENDER_TARGET_FORMATS,
                D3D12_RT_FORMAT_ARRAY {
                    rt_formats: self.rtv_formats,
                    num_render_targets: self.num_render_targets,
                },
            ),
            dsv_format: PackedPipelineStateStreamObject::new(
                T::D3D12_PIPELINE_STATE_SUBOBJECT_TYPE_DEPTH_STENCIL_FORMAT,
                self.dsv_format,
            ),
            sample_desc: PackedPipelineStateStreamObject::new(
                T::D3D12_PIPELINE_STATE_SUBOBJECT_TYPE_SAMPLE_DESC,
                self.sample_desc,
            ),
            node_mask: PackedPipelineStateStreamObject::new(
                T::D3D12_PIPELINE_STATE_SUBOBJECT_TYPE_NODE_MASK,
                self.node_mask,
            ),
            cached_pso: PackedPipelineStateStreamObject::new(
                T::D3D12_PIPELINE_STATE_SUBOBJECT_TYPE_CACHED_PSO,
                self.cached_pso,
            ),
            flags: PackedPipelineStateStreamObject::new(
                T::D3D12_PIPELINE_STATE_SUBOBJECT_TYPE_FLAGS,
                self.flags,
            ),
        };

        unsafe { transmute(packed) }
    }
}

type RootSignature = PackedPipelineStateStreamObject<*mut c_void>;
type VertexShader = PackedPipelineStateStreamObject<D3D12_SHADER_BYTECODE>;
type PixelShader = PackedPipelineStateStreamObject<D3D12_SHADER_BYTECODE>;
type DomainShader = PackedPipelineStateStreamObject<D3D12_SHADER_BYTECODE>;
type HullShader = PackedPipelineStateStreamObject<D3D12_SHADER_BYTECODE>;
type GeometryShader = PackedPipelineStateStreamObject<D3D12_SHADER_BYTECODE>;
type BlendState = PackedPipelineStateStreamObject<D3D12_BLEND_DESC>;
type SampleMask = PackedPipelineStateStreamObject<u32>;
type RasterizerState = PackedPipelineStateStreamObject<D3D12_RASTERIZER_DESC>;
type DepthStencilState = PackedPipelineStateStreamObject<D3D12_DEPTH_STENCIL_DESC>;
type PrimitiveTopologyType = PackedPipelineStateStreamObject<D3D12_PRIMITIVE_TOPOLOGY_TYPE>;
type RenderTargets = PackedPipelineStateStreamObject<D3D12_RT_FORMAT_ARRAY>;
type DsvFormat = PackedPipelineStateStreamObject<DXGI_FORMAT>;
type SampleDesc = PackedPipelineStateStreamObject<DXGI_SAMPLE_DESC>;
type NodeMask = PackedPipelineStateStreamObject<u32>;
type CachedPso = PackedPipelineStateStreamObject<D3D12_CACHED_PIPELINE_STATE>;
type Flags = PackedPipelineStateStreamObject<D3D12_PIPELINE_STATE_FLAGS>;
type StreamOutput = PackedPipelineStateStreamObject<D3D12_STREAM_OUTPUT_DESC>;
type InputLayout = PackedPipelineStateStreamObject<D3D12_INPUT_LAYOUT_DESC>;
type StripCutValue = PackedPipelineStateStreamObject<D3D12_INDEX_BUFFER_STRIP_CUT_VALUE>;
#[repr(C)]
struct Packed {
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
