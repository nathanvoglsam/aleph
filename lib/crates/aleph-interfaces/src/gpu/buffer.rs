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

use crate::gpu::{CpuAccessMode, INamedObject};
use any::{AnyArc, IAny};
use std::any::Any;
use thiserror::Error;

pub trait IBuffer: INamedObject + Send + Sync + IAny + Any + 'static {
    any_arc_trait_utils_decl!(IBuffer);

    fn desc(&self) -> &BufferDesc;
}

/// Description object used for creating a new buffer.
#[derive(Clone, Hash, PartialEq, Eq, Debug)]
pub struct BufferDesc {
    /// The size of the buffer in bytes
    pub size: u64,

    /// What kind of CPU access is allowed.
    /// - None -> device local
    /// - Read -> read back
    /// - Write -> upload
    pub cpu_access: CpuAccessMode,

    /// Enables the buffer to be used with unordered access (unordered access view, storage buffer)
    pub allow_unordered_access: bool,

    /// Enables the buffer to be used as a texel buffer
    pub allow_texel_buffer: bool,

    /// Enables the buffer to be used as a vertex buffer
    pub is_vertex_buffer: bool,

    /// Enables the buffer to be used as an index buffer
    pub is_index_buffer: bool,

    /// Enables the buffer to be used as a constant buffer
    pub is_constant_buffer: bool,

    /// Enables the buffer to be used as an argument buffer for indirect draw calls
    pub is_indirect_draw_args: bool,

    /// Enables the buffer to be used as input for ray tracing acceleration structure builds
    pub is_accel_struct_build_input: bool,

    /// Enables the buffer to store a constructed and ready to use rt acceleration structure
    pub is_accel_struct_storage: bool,
}

impl Default for BufferDesc {
    #[inline]
    fn default() -> Self {
        Self {
            size: 0,
            cpu_access: CpuAccessMode::None,
            allow_unordered_access: false,
            allow_texel_buffer: false,
            is_vertex_buffer: false,
            is_index_buffer: false,
            is_constant_buffer: false,
            is_indirect_draw_args: false,
            is_accel_struct_build_input: false,
            is_accel_struct_storage: false,
        }
    }
}

#[derive(Error, Debug)]
pub enum BufferCreateError {
    #[error("An internal backend error has occurred '{0}'")]
    Platform(#[from] anyhow::Error),
}
