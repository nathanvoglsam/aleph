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

use aleph_frame_graph::PassArgs;
use aleph_pin_board::{BoardParamId, BoardScope};
use parking_lot::Mutex;

use crate::material_instance::MaterialInstancePoolAccessor;
use crate::renderer::state_cache::StateCache;
use crate::resource::buffer::BufferPoolAccessor;
use crate::resource::texture::TexturePoolAccessor;

#[derive(Clone)]
pub struct GraphSwapImageInfo {
    /// [`rhi::TextureDesc`] that describes the swap image we're rendering to.
    pub desc: rhi::TextureDesc<'static>,

    /// Pre-calculated aspect ratio of the swap image we're rendering to.
    pub aspect: f32,
}

impl BoardParamId for GraphSwapImageInfo {
    type Output<'a> = Self;
}

pub struct GraphArgsLayout<'a> {
    pub board: &'a BoardScope<'a>,
    pub texture_pool: TexturePoolAccessor<'a>,
    pub buffer_pool: BufferPoolAccessor<'a>,
    pub material_instance_pool: MaterialInstancePoolAccessor<'a>,
    pub state_cache: &'a Mutex<StateCache>,
}

pub struct GraphArgs();

impl PassArgs for GraphArgs {
    type Args<'a> = GraphArgsLayout<'a>;
}
