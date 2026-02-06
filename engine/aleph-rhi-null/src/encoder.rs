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

use std::any::TypeId;

use aleph_rhi_api::*;

pub struct NullEncoder {}

impl IGetPlatformInterface for NullEncoder {
    unsafe fn __query_platform_interface(&self, _target: TypeId, _out: *mut ()) -> Option<()> {
        None
    }
}

impl ICommandEncoderAbi for NullEncoder {
    unsafe fn __bind_graphics_pipeline(&mut self, _pipeline: &GraphicsPipelineHandle) {}

    unsafe fn __bind_vertex_buffers(
        &mut self,
        _first_binding: u32,
        _bindings: &[InputAssemblyBufferBinding],
    ) {
    }

    unsafe fn __bind_index_buffer(
        &mut self,
        _index_type: IndexType,
        _binding: &InputAssemblyBufferBinding,
    ) {
    }

    unsafe fn __set_viewports(&mut self, _viewports: &[Viewport]) {}

    unsafe fn __set_scissor_rects(&mut self, _rects: &[Rect]) {}

    unsafe fn __set_push_constant_block(&mut self, _data: &[u8]) {}

    unsafe fn __begin_rendering(&mut self, _info: &BeginRenderingInfo) {}

    unsafe fn __end_rendering(&mut self) {}

    unsafe fn __draw(
        &mut self,
        _vertex_count: u32,
        _instance_count: u32,
        _first_vertex: u32,
        _first_instance: u32,
    ) {
    }

    unsafe fn __draw_indexed(
        &mut self,
        _index_count: u32,
        _instance_count: u32,
        _first_index: u32,
        _first_instance: u32,
        _vertex_offset: i32,
    ) {
    }

    unsafe fn __bind_compute_pipeline(&mut self, _pipeline: &ComputePipelineHandle) {}

    unsafe fn __bind_parameter_blocks(
        &mut self,
        _binding_signature: &dyn IBindingSignature,
        _bind_point: PipelineBindPoint,
        _first_block: u32,
        _blocks: &[ParameterBlockHandle],
    ) {
    }

    unsafe fn __push_parameters(
        &mut self,
        _binding_signature: &dyn IBindingSignature,
        _bind_point: PipelineBindPoint,
        _block: u32,
        _base: u32,
        _writes: &[ParameterWrite],
    ) {
        todo!()
    }

    unsafe fn __dispatch(&mut self, _group_count_x: u32, _group_count_y: u32, _group_count_z: u32) {
    }

    unsafe fn __resource_barrier(
        &mut self,
        _global_barriers: &[GlobalBarrier],
        _buffer_barriers: &[BufferBarrier],
        _texture_barriers: &[TextureBarrier],
    ) {
    }

    unsafe fn __copy_buffer_regions(
        &mut self,
        _src: &BufferHandle,
        _dst: &BufferHandle,
        _regions: &[BufferCopyRegion],
    ) {
    }

    unsafe fn __copy_buffer_to_texture(
        &mut self,
        _src: &BufferHandle,
        _dst: &TextureHandle,
        _regions: &[BufferToTextureCopyRegion],
    ) {
    }

    unsafe fn __copy_texture_regions(
        &mut self,
        _src: &TextureHandle,
        _dst: &TextureHandle,
        _regions: &[TextureToTextureCopyInfo],
    ) {
    }

    unsafe fn __close(&mut self) -> Result<(), CommandListCloseError> {
        Ok(())
    }

    unsafe fn __set_marker(&mut self, _color: Color, _message: &aleph_nstr::NStr) {}

    unsafe fn __begin_event(&mut self, _color: Color, _message: &aleph_nstr::NStr) {}

    unsafe fn __end_event(&mut self) {}
}
