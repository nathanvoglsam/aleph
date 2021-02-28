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

use crate::{ComparisonFunc, Filter, ShaderVisibility, StaticBorderColor, TextureAddressMode};

pub struct StaticSamplerDescBuilder {
    inner: StaticSamplerDesc,
}

impl StaticSamplerDescBuilder {
    pub fn new() -> Self {
        Self {
            inner: StaticSamplerDesc::default(),
        }
    }

    pub fn filter(mut self, filter: Filter) -> Self {
        self.inner.filter = filter;
        self
    }

    pub fn address_u(mut self, address_u: TextureAddressMode) -> Self {
        self.inner.address_u = address_u;
        self
    }

    pub fn address_v(mut self, address_v: TextureAddressMode) -> Self {
        self.inner.address_v = address_v;
        self
    }

    pub fn address_w(mut self, address_w: TextureAddressMode) -> Self {
        self.inner.address_w = address_w;
        self
    }

    pub fn mip_lod_bias(mut self, mip_lod_bias: f32) -> Self {
        self.inner.mip_lod_bias = mip_lod_bias;
        self
    }

    pub fn max_anisotropy(mut self, max_anisotropy: u32) -> Self {
        self.inner.max_anisotropy = max_anisotropy;
        self
    }

    pub fn comparison_func(mut self, comparison_func: ComparisonFunc) -> Self {
        self.inner.comparison_func = comparison_func;
        self
    }

    pub fn border_color(mut self, border_color: StaticBorderColor) -> Self {
        self.inner.border_color = border_color;
        self
    }

    pub fn min_lod(mut self, min_lod: f32) -> Self {
        self.inner.min_lod = min_lod;
        self
    }

    pub fn max_lod(mut self, max_lod: f32) -> Self {
        self.inner.max_lod = max_lod;
        self
    }

    pub fn shader_register(mut self, shader_register: u32) -> Self {
        self.inner.shader_register = shader_register;
        self
    }

    pub fn register_space(mut self, register_space: u32) -> Self {
        self.inner.register_space = register_space;
        self
    }

    pub fn shader_visibility(mut self, shader_visibility: ShaderVisibility) -> Self {
        self.inner.shader_visibility = shader_visibility;
        self
    }

    pub fn build(self) -> StaticSamplerDesc {
        self.inner
    }
}

#[repr(C)]
#[derive(Clone, Debug)]
pub struct StaticSamplerDesc {
    pub filter: Filter,
    pub address_u: TextureAddressMode,
    pub address_v: TextureAddressMode,
    pub address_w: TextureAddressMode,
    pub mip_lod_bias: f32,
    pub max_anisotropy: u32,
    pub comparison_func: ComparisonFunc,
    pub border_color: StaticBorderColor,
    pub min_lod: f32,
    pub max_lod: f32,
    pub shader_register: u32,
    pub register_space: u32,
    pub shader_visibility: ShaderVisibility,
}

impl StaticSamplerDesc {
    pub fn builder() -> StaticSamplerDescBuilder {
        StaticSamplerDescBuilder::new()
    }
}

impl Default for StaticSamplerDesc {
    fn default() -> Self {
        Self {
            filter: Filter::MinMagMipLinear,
            address_u: TextureAddressMode::Wrap,
            address_v: TextureAddressMode::Wrap,
            address_w: TextureAddressMode::Wrap,
            mip_lod_bias: 0.0,
            max_anisotropy: 0,
            comparison_func: ComparisonFunc::Never,
            border_color: StaticBorderColor::TransparentBlack,
            min_lod: 0.0,
            max_lod: 4096.0,
            shader_register: 0,
            register_space: 0,
            shader_visibility: ShaderVisibility::All,
        }
    }
}
