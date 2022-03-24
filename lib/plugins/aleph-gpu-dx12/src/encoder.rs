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

use crate::general_command_list::GeneralCommandList;
use crate::internal::command_list_tracker::CommandListTracker;
use crate::internal::conv::decode_u32_color_to_float;
use crate::swap_texture::SwapTexture;
use crate::texture::Texture;
use interfaces::gpu::{
    ColorClearValue, DepthStencilClearValue, DrawIndexedOptions, DrawOptions, IComputeEncoder,
    IGeneralEncoder, ITexture, TextureDesc, TextureSubResourceSet,
};
use interfaces::ref_ptr::WeakRefPtr;
use std::marker::PhantomData;

pub struct Encoder<'a> {
    pub(crate) list: dx12::GraphicsCommandList,
    pub(crate) tracker: CommandListTracker,
    pub(crate) _phantom: PhantomData<&'a mut GeneralCommandList>,
}

impl<'a> Drop for Encoder<'a> {
    fn drop(&mut self) {
        // TODO: Consider an API that forces manually closing so we can avoid the unwrap here
        unsafe { self.list.close().unwrap() }
    }
}

impl<'a> IGeneralEncoder for Encoder<'a> {
    fn clear_texture(
        &mut self,
        texture: WeakRefPtr<dyn ITexture>,
        _subresources: &TextureSubresourceSet,
        value: &ClearValue,
    ) {
        let _texture = texture.query_interface::<Texture>().unwrap();
        match value {
            ClearValue::ColorF32(v) => unsafe {
                let buffer = [v.r, v.g, v.b, v.a];
                self.list.clear_render_target_view(todo!(), &buffer, None);
            },
            ClearValue::ColorInt(v) => unsafe {
                let buffer = decode_u32_color_to_float(*v);
                self.list.clear_render_target_view(todo!(), &buffer, None);
            },
            ClearValue::DepthStencil(depth, stencil) => unsafe {
                self.list.clear_depth_stencil_view(
                    todo!(),
                    dx12::ClearFlags::all(),
                    *depth,
                    *stencil,
                    None,
                );
            },
        }
    }

    fn draw(&mut self, options: &DrawOptions) {
        // TODO: State check
        unsafe {
            self.list.draw_instanced(
                options.vertex_count,
                options.instance_count,
                options.first_vertex,
                options.first_instance,
            )
        }
    }

    fn draw_indexed(&mut self, options: &DrawIndexedOptions) {
        // TODO: State check
        unsafe {
            self.list.draw_indexed_instanced(
                options.index_count,
                options.instance_count,
                options.first_index,
                options.vertex_offset,
                options.first_instance,
            )
        }
    }
}

impl<'a> IComputeEncoder for Encoder<'a> {
    fn dispatch(&mut self, group_count_x: u32, group_count_y: u32, group_count_z: u32) {
        // TODO: State check
        unsafe {
            self.list
                .dispatch(group_count_x, group_count_y, group_count_z);
        }
    }
}
