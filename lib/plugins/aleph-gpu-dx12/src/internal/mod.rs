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

use interfaces::gpu::{Format, TextureCopyAspect};

pub mod adapter_description_decoder;
pub mod command_list_tracker;
pub mod component_mapping;
pub mod conv;
pub mod create_device;
pub mod create_dxgi_factory;
pub mod debug_interface;
pub mod descriptor_allocator_cpu;
pub mod descriptor_handles;
pub mod descriptor_heap;
pub mod descriptor_heap_info;
pub mod descriptor_heaps;
pub mod feature_support;
pub mod graphics_pipeline_state_stream;
pub mod in_flight_command_list;
pub mod register_message_callback;
pub mod root_signature_blob;
pub mod swap_chain_creation;

pub const fn calc_subresource_index(
    mip_level: u32,
    array_layer: u32,
    plane_slice: u32,
    mip_levels: u32,
    array_size: u32,
) -> u32 {
    mip_level + (array_layer * mip_levels) + (plane_slice * mip_levels * array_size)
}

pub const fn plane_layer_for_aspect(format: Format, aspect: TextureCopyAspect) -> Option<u32> {
    match format {
        Format::Depth32Float => match aspect {
            TextureCopyAspect::Depth => Some(1),
            _ => None,
        },
        Format::Depth24Stencil8 => match aspect {
            TextureCopyAspect::Color => None,
            TextureCopyAspect::Depth => Some(0),
            TextureCopyAspect::Stencil => Some(1),
        },
        _ => match aspect {
            TextureCopyAspect::Color => Some(0),
            TextureCopyAspect::Depth => None,
            TextureCopyAspect::Stencil => None,
        },
    }
}
