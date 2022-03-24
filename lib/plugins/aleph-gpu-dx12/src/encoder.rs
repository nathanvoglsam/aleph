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

impl<'a> Encoder<'a> {
    #[inline]
    fn clear_swap_texture(
        &mut self,
        texture: WeakRefPtr<dyn ITexture>,
        concrete: WeakRefPtr<SwapTexture>,
        value: &ColorClearValue,
    ) {
        let buffer = match value {
            ColorClearValue::Float { r, g, b, a } => [*r, *g, *b, *a],
            ColorClearValue::Int(v) => decode_u32_color_to_float(*v),
        };

        unsafe {
            self.list
                .clear_render_target_view(concrete.view, &buffer, None);
        }
        self.tracker.images.push(texture.to_strong());
    }

    #[inline]
    fn clear_plain_texture(
        &mut self,
        texture: WeakRefPtr<dyn ITexture>,
        concrete: WeakRefPtr<Texture>,
        sub_resources: &TextureSubResourceSet,
        value: &ColorClearValue,
    ) {
        if concrete.desc.format.is_depth_stencil() {
            aleph_log::debug!("Tried to perform clear_color on a depth/stencil texture");
            return;
        }

        let buffer = match value {
            ColorClearValue::Float { r, g, b, a } => [*r, *g, *b, *a],
            ColorClearValue::Int(v) => decode_u32_color_to_float(*v),
        };

        let sub_resources = self.clamp_sub_resource_set_to_texture(&concrete.desc, sub_resources);

        // DX12 handles clearing textures differently between render targets and non render target
        // textures.
        if concrete.desc.is_render_target {
            // DX12 can only clear a single mip level per call to ClearRenderTargetView, to clear
            // all the requested layers we need to emit multiple calls to ClearRenderTargetView.
            let begin = sub_resources.base_mip_level;
            let end = begin + sub_resources.num_mip_levels;
            for level in begin..end {
                let level_sub_resources = TextureSubResourceSet {
                    base_mip_level: level,
                    num_mip_levels: 1,
                    base_array_slice: sub_resources.base_array_slice,
                    num_array_slices: sub_resources.base_mip_level,
                };

                let view = concrete.get_or_create_rtv_for_usage(None, &level_sub_resources);

                if let Some(view) = view {
                    unsafe {
                        self.list.clear_render_target_view(view, &buffer, None);
                    }
                } else {
                    aleph_log::debug!(
                        "Called IEncoder::clear_texture with TextureSubResourceSet::num_mip_levels = 0."
                    );
                    return;
                }
            }
        } else {
            todo!()
        }

        self.tracker.images.push(texture.to_strong());
    }

    #[inline]
    fn clear_depth_image(
        &mut self,
        texture: WeakRefPtr<dyn ITexture>,
        concrete: WeakRefPtr<Texture>,
        sub_resources: &TextureSubResourceSet,
        value: &DepthStencilClearValue,
    ) {
        if !concrete.desc.format.is_depth_stencil() {
            aleph_log::debug!("Tried to perform clear_depth_stencil_texture on a color texture");
            return;
        }

        let (depth, stencil, clear_flags) = match value {
            DepthStencilClearValue::DepthStencil(d, s) => (*d, *s, dx12::ClearFlags::all()),
            DepthStencilClearValue::Depth(d) => (*d, 0, dx12::ClearFlags::DEPTH),
            DepthStencilClearValue::Stencil(s) => (0.0, *s, dx12::ClearFlags::STENCIL),
        };

        let sub_resources = self.clamp_sub_resource_set_to_texture(&concrete.desc, sub_resources);

        // DX12 handles clearing textures differently between render targets and non render target
        // textures.
        if concrete.desc.is_render_target {
            // DX12 can only clear a single mip level per call to ClearRenderTargetView, to clear
            // all the requested layers we need to emit multiple calls to ClearRenderTargetView.
            let begin = sub_resources.base_mip_level;
            let end = begin + sub_resources.num_mip_levels;
            for level in begin..end {
                let level_sub_resources = TextureSubResourceSet {
                    base_mip_level: level,
                    num_mip_levels: 1,
                    base_array_slice: sub_resources.base_array_slice,
                    num_array_slices: sub_resources.base_mip_level,
                };

                let view = concrete.get_or_create_dsv_for_usage(None, &level_sub_resources);
                if let Some(view) = view {
                    unsafe {
                        self.list
                            .clear_depth_stencil_view(view, clear_flags, depth, stencil, None);
                    }
                } else {
                    aleph_log::debug!(
                    "Called IEncoder::clear_depth_stencil_texture with TextureSubResourceSet::num_mip_levels = 0."
                );
                    return;
                }
            }
        }

        self.tracker.images.push(texture.to_strong());
    }

    #[inline]
    fn clamp_sub_resource_set_to_texture(
        &self,
        texture: &TextureDesc,
        sub_resources: &TextureSubResourceSet,
    ) -> TextureSubResourceSet {
        #[inline(always)]
        fn clamp_range(base: u32, len: u32, min: u32, max: u32) -> (u32, u32) {
            let base_level = base.clamp(min, max);
            let end_level = base_level + len;
            let end_level_clamped = end_level.clamp(min, max);
            let num_levels = (end_level_clamped + 1) - base_level;
            (base_level, num_levels)
        }

        let min_mip_level = 0;
        let max_mip_level = texture.mip_levels - 1;
        let (base_mip_level, num_mip_levels) = clamp_range(
            sub_resources.base_mip_level,
            sub_resources.num_mip_levels,
            min_mip_level,
            max_mip_level,
        );

        let min_array_slice = 0;
        let max_array_slice = texture.array_size - 1;
        let (base_array_slice, num_array_slices) = clamp_range(
            sub_resources.base_array_slice,
            sub_resources.num_array_slices,
            min_array_slice,
            max_array_slice,
        );

        // Warn if the base mip level is out of bounds
        if sub_resources.base_mip_level >= max_mip_level {
            aleph_log::debug!("TextureSubResourceSet out of bounds: base_mip_level");
        }

        // Warn if the sub resource set is addressing the mip levels out of bounds unintentionally
        //
        // u32::MAX flags "use all mip levels"
        let use_all_mips = texture.mip_levels == u32::MAX;
        if (base_mip_level + num_mip_levels) > max_mip_level && !use_all_mips {
            aleph_log::debug!("TextureSubResourceSet out of bounds: num_mip_levels")
        }

        // Warn if the base array slice is out of bounds
        if sub_resources.base_array_slice >= max_array_slice {
            aleph_log::debug!("TextureSubResourceSet out of bounds: base_array_slice");
        }

        // Warn if the sub resource set is addressing the array slices out of bounds unintentionally
        //
        // u32::MAX flags "use all array slices"
        let use_all_slices = texture.array_size == u32::MAX;
        if (base_array_slice + num_array_slices) > max_array_slice && !use_all_slices {
            aleph_log::debug!("TextureSubResourceSet out of bounds: num_array_slices")
        }

        TextureSubResourceSet {
            base_mip_level,
            num_mip_levels,
            base_array_slice,
            num_array_slices,
        }
    }
}

impl<'a> IGeneralEncoder for Encoder<'a> {
    fn clear_texture(
        &mut self,
        texture: WeakRefPtr<dyn ITexture>,
        sub_resources: &TextureSubResourceSet,
        value: &ColorClearValue,
    ) {
        if let Some(concrete) = texture.query_interface::<Texture>() {
            self.clear_plain_texture(texture.clone(), concrete, sub_resources, value);
        } else if let Some(concrete) = texture.query_interface::<SwapTexture>() {
            self.clear_swap_texture(texture.clone(), concrete, value);
        } else {
            panic!("Unknown ITexture implementation");
        }
    }

    fn clear_depth_stencil_texture(
        &mut self,
        texture: WeakRefPtr<dyn ITexture>,
        sub_resources: &TextureSubResourceSet,
        value: &DepthStencilClearValue,
    ) {
        if let Some(concrete) = texture.query_interface::<Texture>() {
            self.clear_depth_image(texture.clone(), concrete, sub_resources, value);
        } else if let Some(_) = texture.query_interface::<SwapTexture>() {
            aleph_log::debug!("Tried to clear swap chain image as a depth stencil texture");
        } else {
            panic!("Unknown ITexture implementation");
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
