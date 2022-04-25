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

use crate::gpu::{ITexture, TextureSubResourceSet};
use std::fmt::{Display, Formatter};

pub trait IGeneralEncoder: IComputeEncoder + Send {
    fn clear_texture(
        &mut self,
        texture: &dyn ITexture,
        sub_resources: &TextureSubResourceSet,
        value: &ColorClearValue,
    );
    fn clear_depth_stencil_texture(
        &mut self,
        texture: &dyn ITexture,
        sub_resources: &TextureSubResourceSet,
        value: &DepthStencilClearValue,
    );
    fn draw(&mut self, options: &DrawOptions);
    fn draw_indexed(&mut self, options: &DrawIndexedOptions);
}

pub trait IComputeEncoder: ITransferEncoder + Send {
    fn dispatch(&mut self, group_count_x: u32, group_count_y: u32, group_count_z: u32);
}

pub trait ITransferEncoder: Send {}

/// Set of options for a draw call command
#[derive(Clone, Default, Hash, PartialEq, Eq, Debug)]
pub struct DrawOptions {
    pub vertex_count: u32,
    pub instance_count: u32,
    pub first_vertex: u32,
    pub first_instance: u32,
}

/// Set of options for a draw call command
#[derive(Clone, Default, Hash, PartialEq, Eq, Debug)]
pub struct DrawIndexedOptions {
    pub index_count: u32,
    pub instance_count: u32,
    pub first_index: u32,
    pub first_instance: u32,
    pub vertex_offset: i32,
}

/// An enumeration of all possible input types to a color texture clear operation
#[derive(Clone, Debug, PartialEq)]
pub enum ColorClearValue {
    /// A full 4-channel f32 colour
    Float { r: f32, g: f32, b: f32, a: f32 },

    /// A 4-channel color packed into a single u32
    Int(u32),
}

impl From<u32> for ColorClearValue {
    fn from(v: u32) -> Self {
        Self::Int(v)
    }
}

impl Display for ColorClearValue {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            ColorClearValue::Float { r, g, b, a } => {
                write!(f, "ColorClearValue::Float({}, {}, {}, {})", r, g, b, a)
            }
            ColorClearValue::Int(v) => {
                write!(f, "ColorClearValue::Int({:X})", *v)
            }
        }
    }
}

/// An enumeration of all possible input types to a depth/stencil texture clear operation
#[derive(Clone, Debug, PartialEq)]
pub enum DepthStencilClearValue {
    /// A floating point + u8 pair for clearing a depth stencil texture
    DepthStencil(f32, u8),

    /// A floating point value for clearing only depth
    Depth(f32),

    /// A u8 value for clearing only stencil
    Stencil(u8),
}

impl Display for DepthStencilClearValue {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            DepthStencilClearValue::DepthStencil(depth, stencil) => {
                write!(f, "ColorClearValue::Float({}, {})", *depth, *stencil)
            }
            DepthStencilClearValue::Depth(v) => {
                write!(f, "DepthStencilClearValue::Depth({})", *v)
            }
            DepthStencilClearValue::Stencil(v) => {
                write!(f, "DepthStencilClearValue::Stencil({})", *v)
            }
        }
    }
}
