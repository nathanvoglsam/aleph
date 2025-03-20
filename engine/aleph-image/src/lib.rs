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

mod image_buffer;
mod image_ops;
mod image_view;
mod pixel_format;
mod spherical_mapping;
mod texture;
mod texture_ops;
mod utils;

pub use image_buffer::{ColorType, DowncastImageBuffer, DynamicImageBuffer, ImageBuffer};
pub use image_ops::*;
pub use image_view::{ImageView, ImageViewMut};
pub use pixel_format::{
    FromImagePixel, PixR, PixRG, PixRGB, PixRGBA, PixelChannelType, PixelFormat,
};
pub use spherical_mapping::{EnvironmentMapProjection, SphericalMapping};
pub use texture::{
    layer_and_level_from_set_index, set_index_for_layer_and_level, DynamicTextureBuffer,
    TextureBuffer, TextureType,
};
pub use texture_ops::*;
