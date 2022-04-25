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

use crate::gpu::INamedObject;
use any::{AnyArc, IAny};
use std::any::Any;
use thiserror::Error;

pub trait IShader: INamedObject + Send + Sync + IAny + Any + 'static {
    any_arc_trait_utils_decl!(IShader);

    fn shader_type(&self) -> ShaderType;
    fn entry_point(&self) -> &str;
}

/// Set of options for creating a new shader module
#[derive(Clone, Hash, PartialEq, Eq, Debug)]
pub struct ShaderOptions<'a> {
    pub shader_type: ShaderType,
    pub data: ShaderBinary<'a>,
    pub entry_point: &'a str,
}

/// An enumeration of the supported set of shader input types.
#[derive(Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Debug)]
pub enum ShaderBinary<'a> {
    /// This variant encloses a SPIR-V binary. Only supported by the `Vulkan` backend.
    Spirv(&'a [u8]),

    /// This variant encloses a DXIL binary. Only supported by the `D3D12` backend.
    Dxil(&'a [u8]),
}

/// An enumeration of all individual shader types
#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Debug)]
pub enum ShaderType {
    Compute,
    Vertex,
    Hull,
    Domain,
    Geometry,
    Fragment,
    Amplification,
    Mesh,
}

#[derive(Error, Debug)]
pub enum ShaderCreateError {
    /// This error occurs when the byte size of the shader blob is of an invalid size.
    ///
    /// Invalid sizes include:
    ///     - 0
    ///     - Non multiples of 4 (on Vulkan)
    ///
    /// # Vulkan
    ///
    /// Vulkan consumes SPIR-V as the shader blob. SPIR-V is encoded as a sequence of `u32` values.
    /// It is impossible for a valid SPIR-V binary to have a size that is not a multiple of 4 (the
    /// size of a u32) for this reason.
    #[error("The shader binary size '{0}' is invalid")]
    InvalidInputSize(usize),

    /// This error occurs when a shader binary is provided in a format not supported by the active
    /// backend.
    ///
    /// The `Vulkan` backend can only accept SPIR-V shaders, while the `D3D12` backend can only
    /// accept DXIL shaders.
    #[error("The shader binary is of unsupported format")]
    UnsupportedShaderFormat,

    #[error("An internal backend error has occurred '{0}'")]
    Platform(#[from] anyhow::Error),
}
