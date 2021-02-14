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

use crate::{Blend, BlendOp, LogicOp};
use raw::windows::win32::direct3d12::D3D12_RENDER_TARGET_BLEND_DESC;

#[derive(Copy, Clone, Debug, Hash)]
pub struct RenderTargetBlendDesc {
    pub blend_enable: bool,
    pub logic_op_enable: bool,
    pub src_blend: Blend,
    pub dest_blend: Blend,
    pub blend_op: BlendOp,
    pub src_blend_alpha: Blend,
    pub dest_blend_alpha: Blend,
    pub blend_op_alpha: BlendOp,
    pub logic_op: LogicOp,
    pub render_target_write_mask: u8,
}

impl Into<D3D12_RENDER_TARGET_BLEND_DESC> for RenderTargetBlendDesc {
    fn into(self) -> D3D12_RENDER_TARGET_BLEND_DESC {
        D3D12_RENDER_TARGET_BLEND_DESC {
            blend_enable: self.blend_enable.into(),
            logic_op_enable: self.logic_op_enable.into(),
            src_blend: self.src_blend.into(),
            dest_blend: self.dest_blend.into(),
            blend_op: self.blend_op.into(),
            src_blend_alpha: self.src_blend_alpha.into(),
            dest_blend_alpha: self.dest_blend_alpha.into(),
            blend_op_alpha: self.blend_op_alpha.into(),
            logic_op: self.logic_op.into(),
            render_target_write_mask: self.render_target_write_mask.into(),
        }
    }
}
