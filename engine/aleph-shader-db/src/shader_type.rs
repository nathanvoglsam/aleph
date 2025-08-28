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

#[derive(
    rkyv::Archive,
    rkyv::Serialize,
    rkyv::Deserialize,
    Clone,
    Copy,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Hash,
    Debug,
)]
#[rkyv(compare(PartialEq), derive(Copy, Clone))]
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

impl Default for ShaderType {
    #[inline(always)]
    fn default() -> Self {
        Self::Compute
    }
}

macro_rules! convert_shader_type_match {
    ($sself: ident, $dst_t: path) => {{
        type T = $dst_t;
        match $sself {
            Self::Compute => T::Compute,
            Self::Vertex => T::Vertex,
            Self::Hull => T::Hull,
            Self::Domain => T::Domain,
            Self::Geometry => T::Geometry,
            Self::Fragment => T::Fragment,
            Self::Amplification => T::Amplification,
            Self::Mesh => T::Mesh,
        }
    }};
}

impl Into<ShaderType> for ArchivedShaderType {
    #[inline]
    fn into(self) -> ShaderType {
        convert_shader_type_match!(self, ShaderType)
    }
}

impl Into<aleph_rhi_api::ShaderType> for ShaderType {
    #[inline]
    fn into(self) -> aleph_rhi_api::ShaderType {
        convert_shader_type_match!(self, aleph_rhi_api::ShaderType)
    }
}

impl Into<aleph_rhi_api::ShaderType> for ArchivedShaderType {
    #[inline]
    fn into(self) -> aleph_rhi_api::ShaderType {
        convert_shader_type_match!(self, aleph_rhi_api::ShaderType)
    }
}

pub trait ShaderStage: Copy + Clone {
    const API_SHADER_TYPE: aleph_rhi_api::ShaderType;
}

macro_rules! shader_stage_variant {
    ($v_name: ident) => {
        #[derive(Copy, Clone)]
        pub struct $v_name {}

        impl ShaderStage for $v_name {
            const API_SHADER_TYPE: aleph_rhi_api::ShaderType = aleph_rhi_api::ShaderType::$v_name;
        }
    };
}

shader_stage_variant!(Compute);
shader_stage_variant!(Vertex);
shader_stage_variant!(Hull);
shader_stage_variant!(Domain);
shader_stage_variant!(Geometry);
shader_stage_variant!(Fragment);
shader_stage_variant!(Amplification);
shader_stage_variant!(Mesh);
