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

pub use data_format_descriptor::ASTCChannelType;
pub use data_format_descriptor::BC1ChannelType;
pub use data_format_descriptor::BC2ChannelType;
pub use data_format_descriptor::BC3ChannelType;
pub use data_format_descriptor::BC4ChannelType;
pub use data_format_descriptor::BC5ChannelType;
pub use data_format_descriptor::BC6ChannelType;
pub use data_format_descriptor::BC7ChannelType;
pub use data_format_descriptor::ColorModel;
pub use data_format_descriptor::ColorPrimaries;
pub use data_format_descriptor::DFDError;
pub use data_format_descriptor::DFDFlags;
pub use data_format_descriptor::DataFormatDescriptor;
pub use data_format_descriptor::ETC1ChannelType;
pub use data_format_descriptor::ETC1SChannelType;
pub use data_format_descriptor::ETC2ChannelType;
pub use data_format_descriptor::PVRTC2ChannelType;
pub use data_format_descriptor::PVRTCChannelType;
pub use data_format_descriptor::RGBSDAChannelType;
pub use data_format_descriptor::SampleFlags;
pub use data_format_descriptor::SampleInfo;
pub use data_format_descriptor::SampleInfoIterator;
pub use data_format_descriptor::TransferFunction;
pub use document::KTXDocument;
pub use document::KTXReadError;
pub use document::SuperCompressionScheme;
pub use format::format_alpha_bits;
pub use format::format_blue_bits;
pub use format::format_bytes_for_image;
pub use format::format_bytes_per_block;
pub use format::format_depth_bits;
pub use format::format_exponent_bits;
pub use format::format_green_bits;
pub use format::format_pack_bits;
pub use format::format_red_bits;
pub use format::format_sample_info_count;
pub use format::format_stencil_bits;
pub use format::format_type_size;
pub use format::is_format_alpha_first_ordered;
pub use format::is_format_prohibited;
pub use format::is_format_rgbds_ordered;
pub use format::is_format_unsupported;
pub use format::ALLOWED_FORMATS;
