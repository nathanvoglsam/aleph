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

use crate::gpu::{DescriptorType, INamedObject, ShaderType};
use any::{AnyArc, IAny};
use std::any::Any;

pub trait IDescriptorSetLayout: INamedObject + Send + Sync + IAny + Any + 'static {
    any_arc_trait_utils_decl!(IDescriptorSetLayout);
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct DescriptorSetLayoutDescItem {
    pub binding_num: u32,
    pub binding_type: DescriptorType,
    pub read_only: bool,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct DescriptorSetLayoutDesc<'a> {
    pub visibility: DescriptorShaderVisibility,
    pub items: &'a [DescriptorSetLayoutDescItem],
}

#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Debug)]
pub enum DescriptorShaderVisibility {
    All,
    Compute,
    Vertex,
    Hull,
    Domain,
    Geometry,
    Fragment,
    Amplification,
    Mesh,
}

impl From<ShaderType> for DescriptorShaderVisibility {
    #[inline]
    fn from(v: ShaderType) -> Self {
        match v {
            ShaderType::Compute => DescriptorShaderVisibility::Compute,
            ShaderType::Vertex => DescriptorShaderVisibility::Vertex,
            ShaderType::Hull => DescriptorShaderVisibility::Hull,
            ShaderType::Domain => DescriptorShaderVisibility::Domain,
            ShaderType::Geometry => DescriptorShaderVisibility::Geometry,
            ShaderType::Fragment => DescriptorShaderVisibility::Fragment,
            ShaderType::Amplification => DescriptorShaderVisibility::Amplification,
            ShaderType::Mesh => DescriptorShaderVisibility::Mesh,
        }
    }
}
