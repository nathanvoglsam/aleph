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

mod data_format_descriptor;
mod document;
mod format;

#[cfg(test)]
mod tests;

pub use data_format_descriptor::{
    ASTCChannelType, BC1ChannelType, BC2ChannelType, BC3ChannelType, BC4ChannelType,
    BC5ChannelType, BC6ChannelType, BC7ChannelType, ColorModel, ColorPrimaries, DFDError, DFDFlags,
    DataFormatDescriptor, ETC1ChannelType, ETC1SChannelType, ETC2ChannelType, PVRTC2ChannelType,
    PVRTCChannelType, RGBSDAChannelType, SampleFlags, SampleInfo, SampleInfoIterator,
    TransferFunction,
};
pub use document::{KTXDocument, KTXReadError, SuperCompressionScheme};
pub use format::{
    format_alpha_bits, format_blue_bits, format_bytes_for_image, format_bytes_per_block,
    format_depth_bits, format_exponent_bits, format_green_bits, format_pack_bits, format_red_bits,
    format_sample_info_count, format_stencil_bits, format_type_size, is_format_alpha_first_ordered,
    is_format_prohibited, is_format_rgbds_ordered, is_format_unsupported, ALLOWED_FORMATS,
};
