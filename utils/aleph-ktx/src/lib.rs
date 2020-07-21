//
//
// This file is a part of Aleph
//
// <ALEPH_REPO_REPLACE>
//
// <ALEPH_LICENSE_REPLACE>
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
pub use format::format_sample_info_count;
pub use format::is_format_prohibited;
pub use format::is_format_unsupported;
pub use format::ALLOWED_FORMATS;
pub use format::SUPPORTED_FORMATS;
