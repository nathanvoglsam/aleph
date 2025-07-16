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

use aleph_rhi_api::*;
use objc2_metal::*;

pub const fn format_to_pixel_mtl(format: Format) -> MTLPixelFormat {
    match format {
        Format::R8Unorm => MTLPixelFormat::R8Unorm,
        Format::R8Snorm => MTLPixelFormat::R8Snorm,
        Format::R8Uint => MTLPixelFormat::R8Uint,
        Format::R8Sint => MTLPixelFormat::R8Sint,
        Format::R16Uint => MTLPixelFormat::R16Uint,
        Format::R16Sint => MTLPixelFormat::R16Sint,
        Format::R16Unorm => MTLPixelFormat::R16Unorm,
        Format::R16Snorm => MTLPixelFormat::R16Snorm,
        Format::R16Float => MTLPixelFormat::R16Float,
        Format::R32Uint => MTLPixelFormat::R32Uint,
        Format::R32Sint => MTLPixelFormat::R32Sint,
        Format::R32Float => MTLPixelFormat::R32Float,
        Format::Rg8Unorm => MTLPixelFormat::RG8Unorm,
        Format::Rg8Snorm => MTLPixelFormat::RG8Snorm,
        Format::Rg8Uint => MTLPixelFormat::RG8Uint,
        Format::Rg8Sint => MTLPixelFormat::RG8Sint,
        Format::Rg16Uint => MTLPixelFormat::RG16Uint,
        Format::Rg16Sint => MTLPixelFormat::RG16Sint,
        Format::Rg16Unorm => MTLPixelFormat::RG16Unorm,
        Format::Rg16Snorm => MTLPixelFormat::RG16Snorm,
        Format::Rg16Float => MTLPixelFormat::RG16Float,
        Format::Rg32Uint => MTLPixelFormat::RG32Uint,
        Format::Rg32Sint => MTLPixelFormat::RG32Sint,
        Format::Rg32Float => MTLPixelFormat::RG32Float,
        Format::Rgb32Uint => unimplemented!(),
        Format::Rgb32Sint => unimplemented!(),
        Format::Rgb32Float => unimplemented!(),
        Format::Rgba8Unorm => MTLPixelFormat::RGBA8Unorm,
        Format::Rgba8UnormSrgb => MTLPixelFormat::RGBA8Unorm_sRGB,
        Format::Rgba8Snorm => MTLPixelFormat::RGBA8Snorm,
        Format::Rgba8Uint => MTLPixelFormat::RGBA8Uint,
        Format::Rgba8Sint => MTLPixelFormat::RGBA8Sint,
        Format::Bgra8Unorm => MTLPixelFormat::BGRA8Unorm,
        Format::Bgra8UnormSrgb => MTLPixelFormat::BGRA8Unorm_sRGB,
        Format::Rgb10a2Unorm => MTLPixelFormat::RGB10A2Unorm,
        Format::Rg11b10Float => MTLPixelFormat::RG11B10Float,
        Format::Rgb9e5Float => MTLPixelFormat::RGB9E5Float,
        Format::Rgba16Uint => MTLPixelFormat::RGBA16Uint,
        Format::Rgba16Sint => MTLPixelFormat::RGBA16Sint,
        Format::Rgba16Unorm => MTLPixelFormat::RGBA16Unorm,
        Format::Rgba16Snorm => MTLPixelFormat::RGBA16Snorm,
        Format::Rgba16Float => MTLPixelFormat::RGBA16Float,
        Format::Rgba32Uint => MTLPixelFormat::RGBA32Uint,
        Format::Rgba32Sint => MTLPixelFormat::RGBA32Sint,
        Format::Rgba32Float => MTLPixelFormat::RGBA32Float,
        Format::Depth32Float => MTLPixelFormat::Depth32Float,
        Format::Depth32FloatStencil8 => MTLPixelFormat::Depth32Float_Stencil8,
        Format::Depth24Stencil8 => MTLPixelFormat::Depth24Unorm_Stencil8,
    }
}

pub const fn pixel_mtl_to_format(format: MTLPixelFormat) -> Format {
    match format {
        MTLPixelFormat::R8Unorm => Format::R8Unorm,
        MTLPixelFormat::R8Snorm => Format::R8Snorm,
        MTLPixelFormat::R8Uint => Format::R8Uint,
        MTLPixelFormat::R8Sint => Format::R8Sint,
        MTLPixelFormat::R16Uint => Format::R16Uint,
        MTLPixelFormat::R16Sint => Format::R16Sint,
        MTLPixelFormat::R16Unorm => Format::R16Unorm,
        MTLPixelFormat::R16Snorm => Format::R16Snorm,
        MTLPixelFormat::R16Float => Format::R16Float,
        MTLPixelFormat::R32Uint => Format::R32Uint,
        MTLPixelFormat::R32Sint => Format::R32Sint,
        MTLPixelFormat::R32Float => Format::R32Float,
        MTLPixelFormat::RG8Unorm => Format::Rg8Unorm,
        MTLPixelFormat::RG8Snorm => Format::Rg8Snorm,
        MTLPixelFormat::RG8Uint => Format::Rg8Uint,
        MTLPixelFormat::RG8Sint => Format::Rg8Sint,
        MTLPixelFormat::RG16Uint => Format::Rg16Uint,
        MTLPixelFormat::RG16Sint => Format::Rg16Sint,
        MTLPixelFormat::RG16Unorm => Format::Rg16Unorm,
        MTLPixelFormat::RG16Snorm => Format::Rg16Snorm,
        MTLPixelFormat::RG16Float => Format::Rg16Float,
        MTLPixelFormat::RG32Uint => Format::Rg32Uint,
        MTLPixelFormat::RG32Sint => Format::Rg32Sint,
        MTLPixelFormat::RG32Float => Format::Rg32Float,
        MTLPixelFormat::RGBA8Unorm => Format::Rgba8Unorm,
        MTLPixelFormat::RGBA8Unorm_sRGB => Format::Rgba8UnormSrgb,
        MTLPixelFormat::RGBA8Snorm => Format::Rgba8Snorm,
        MTLPixelFormat::RGBA8Uint => Format::Rgba8Uint,
        MTLPixelFormat::RGBA8Sint => Format::Rgba8Sint,
        MTLPixelFormat::BGRA8Unorm => Format::Bgra8Unorm,
        MTLPixelFormat::BGRA8Unorm_sRGB => Format::Bgra8UnormSrgb,
        MTLPixelFormat::RGB10A2Unorm => Format::Rgb10a2Unorm,
        MTLPixelFormat::RG11B10Float => Format::Rg11b10Float,
        MTLPixelFormat::RGB9E5Float => Format::Rgb9e5Float,
        MTLPixelFormat::RGBA16Uint => Format::Rgba16Uint,
        MTLPixelFormat::RGBA16Sint => Format::Rgba16Sint,
        MTLPixelFormat::RGBA16Unorm => Format::Rgba16Unorm,
        MTLPixelFormat::RGBA16Snorm => Format::Rgba16Snorm,
        MTLPixelFormat::RGBA16Float => Format::Rgba16Float,
        MTLPixelFormat::RGBA32Uint => Format::Rgba32Uint,
        MTLPixelFormat::RGBA32Sint => Format::Rgba32Sint,
        MTLPixelFormat::RGBA32Float => Format::Rgba32Float,
        MTLPixelFormat::Depth32Float => Format::Depth32Float,
        MTLPixelFormat::Depth32Float_Stencil8 => Format::Depth32FloatStencil8,
        MTLPixelFormat::Depth24Unorm_Stencil8 => Format::Depth24Stencil8,
        _ => unimplemented!(),
    }
}

pub const fn rect_to_mtl_scissor_rect(rect: &Rect) -> MTLScissorRect {
    MTLScissorRect {
        x: rect.x as usize,
        y: rect.y as usize,
        width: rect.w as usize,
        height: rect.h as usize,
    }
}

pub const fn viewport_to_mtl(viewport: &Viewport) -> MTLViewport {
    MTLViewport {
        originX: viewport.x as f64,
        originY: viewport.y as f64,
        width: viewport.width as f64,
        height: viewport.height as f64,
        znear: viewport.min_depth as f64,
        zfar: viewport.max_depth as f64,
    }
}

pub const fn index_type_to_mtl(index_type: IndexType) -> MTLIndexType {
    match index_type {
        IndexType::U16 => MTLIndexType::UInt16,
        IndexType::U32 => MTLIndexType::UInt32,
    }
}

pub const fn index_type_to_size(index_type: IndexType) -> usize {
    match index_type {
        IndexType::U16 => 2,
        IndexType::U32 => 4,
    }
}
