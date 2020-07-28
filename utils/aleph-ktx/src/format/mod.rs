//
//
// This file is a part of Aleph
//
// <ALEPH_REPO_REPLACE>
//
// <ALEPH_LICENSE_REPLACE>
//

#[cfg(test)]
mod tests;

mod bit_count;
mod component_order;
mod layout;
mod samples;
mod support;

pub use bit_count::format_alpha_bits;
pub use bit_count::format_blue_bits;
pub use bit_count::format_bytes_per_block;
pub use bit_count::format_depth_bits;
pub use bit_count::format_exponent_bits;
pub use bit_count::format_green_bits;
pub use bit_count::format_red_bits;
pub use bit_count::format_stencil_bits;
pub use component_order::is_format_alpha_first_ordered;
pub use component_order::is_format_rgbds_ordered;
pub use layout::format_pack_bits;
pub use layout::format_type_size;
pub use samples::format_sample_info_count;
pub use support::is_format_prohibited;
pub use support::is_format_unsupported;
pub use support::ALLOWED_FORMATS;
