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

use crate::gpu::{
    AdapterPowerClass, ClearValue, CpuAccessMode, ISurface, PresentationMode, QueueType,
    ShaderBinary, ShaderType, TextureDimension, TextureFormat,
};
use bitflags::bitflags;
use ref_ptr::WeakRefPtr;

/// Options provided when a context is created
#[derive(Clone, Default, Hash, PartialEq, Eq, Debug)]
pub struct ContextOptions {
    /// Whether backend API validation should be enabled.
    ///
    /// Will implicitly force the `debug` option to true if `validation` is also true as on some
    /// backends the `validation` option requires loading the same `debug` utilities to function.
    ///
    /// This flag requests that the backend should enable their backend specific API validation.
    ///
    /// This will add massive amounts of overhead and should never be enabled unless debugging the
    /// backends themselves.
    ///
    /// # Detail
    ///
    /// This is will enable w/e API validation and debug tools that are available to the backend.
    ///
    /// For Vulkan this will enable the validation layers and install a debug messenger the uses
    /// the rust `log` framework.
    ///
    /// For Direct3D 12 this will enable API validation.
    pub validation: bool,

    /// Whether backend debug utilities should be enabled. This enables debug integrations for
    /// naming objects and marking code sections to the backend's API for markup in debug tools.
    ///
    /// # Detail
    ///
    /// Basically just a request to enable `VK_EXT_debug_utils` for Vulkan without enabling
    /// validation layers.
    pub debug: bool,
}

#[derive(Clone)]
pub struct AdapterRequestOptions<'a> {
    /// A handle to an [ISurface] which the device adapter must be able to render and present to.
    ///
    /// Can be set to `None` to indicate we aren't going to present. Useful for compute-only
    /// workloads.
    pub surface: Option<WeakRefPtr<'a, dyn ISurface>>,

    /// Specifies the preferred power class of the adapter the context should return. See
    /// [AdapterPowerClass] for the meaning of each power class.
    ///
    /// This only specifies a preference. There is no guarantee that the returned adapter will be
    /// of any particular power class, only that the context will chose the best available match
    /// out of the set of compatible adapters.
    ///
    /// e.g. If a system only has a single dedicated GPU and the preferred power class is low-power
    /// then the context will still yield the dedicated GPU.
    pub power_class: AdapterPowerClass,
}

impl<'a> Default for AdapterRequestOptions<'a> {
    #[inline]
    fn default() -> Self {
        Self {
            // We can't make a "default" surface so just default to no surface.
            surface: None,

            // 99.9999% of the time this will be HighPower so we default to that.
            power_class: AdapterPowerClass::HighPower,
        }
    }
}

#[derive(Clone, Hash, PartialEq, Eq, Debug)]
pub struct AdapterDescription<'a> {
    /// The name of the adapter
    pub name: &'a str,
}

#[derive(Clone, Hash, PartialEq, Eq, Debug)]
pub struct SwapChainConfiguration {
    pub format: TextureFormat,
    pub width: u32,
    pub height: u32,
    pub present_mode: PresentationMode,
    pub preferred_queue: QueueType,
}

impl Default for SwapChainConfiguration {
    #[inline]
    fn default() -> Self {
        Self {
            format: TextureFormat::Bgra8UnormSrgb,
            width: 0,
            height: 0,
            present_mode: PresentationMode::Fifo,
            preferred_queue: QueueType::General,
        }
    }
}

/// A floating point colour value
#[derive(Clone, PartialEq, Debug)]
pub struct ColorRGBA {
    pub r: f32,
    pub g: f32,
    pub b: f32,
    pub a: f32,
}

/// Set of options for a draw call command
#[derive(Clone, Default, Hash, PartialEq, Eq, Debug)]
pub struct DrawOptions {
    pub vertex_count: u32,
    pub instance_count: u32,
    pub first_vertex: u32,
    pub first_instance: u32,
}

/// Set of options for a draw call command
#[derive(Clone, Default, Hash, PartialEq, Eq, Debug)]
pub struct DrawIndexedOptions {
    pub index_count: u32,
    pub instance_count: u32,
    pub first_index: u32,
    pub first_instance: u32,
    pub vertex_offset: i32,
}

/// Set of options for creating a new shader module
#[derive(Clone, Hash, PartialEq, Eq, Debug)]
pub struct ShaderOptions<'a> {
    pub shader_type: ShaderType,
    pub data: ShaderBinary<'a>,
    pub entry_point: &'a str,
}

/// Description object used for creating a new buffer.
#[derive(Clone, Hash, PartialEq, Eq, Debug)]
pub struct BufferDesc {
    /// The size of the buffer in bytes
    pub size: u64,

    /// What kind of CPU access is allowed.
    /// - None -> device local
    /// - Read -> read back
    /// - Write -> upload
    pub cpu_access: CpuAccessMode,

    /// Enables the buffer to be used with unordered access (unordered access view, storage buffer)
    pub allow_unordered_access: bool,

    /// Enables the buffer to be used as a texel buffer
    pub allow_texel_buffer: bool,

    /// Enables the buffer to be used as a vertex buffer
    pub is_vertex_buffer: bool,

    /// Enables the buffer to be used as an index buffer
    pub is_index_buffer: bool,

    /// Enables the buffer to be used as a constant buffer
    pub is_constant_buffer: bool,

    /// Enables the buffer to be used as an argument buffer for indirect draw calls
    pub is_indirect_draw_args: bool,

    /// Enables the buffer to be used as input for ray tracing acceleration structure builds
    pub is_accel_struct_build_input: bool,

    /// Enables the buffer to store a constructed and ready to use rt acceleration structure
    pub is_accel_struct_storage: bool,
}

impl Default for BufferDesc {
    #[inline]
    fn default() -> Self {
        Self {
            size: 0,
            cpu_access: CpuAccessMode::None,
            allow_unordered_access: false,
            allow_texel_buffer: false,
            is_vertex_buffer: false,
            is_index_buffer: false,
            is_constant_buffer: false,
            is_indirect_draw_args: false,
            is_accel_struct_build_input: false,
            is_accel_struct_storage: false,
        }
    }
}

/// Description object used for creating a new texture.
#[derive(Clone, PartialEq, Debug)]
pub struct TextureDesc {
    /// The width of the texture
    pub width: u32,

    /// The height of the texture
    pub height: u32,

    /// The depth of the texture
    pub depth: u32,

    /// The pixel format of the texture
    pub format: TextureFormat,

    /// The dimensionality of the texture.
    ///
    /// Declares whether the texture should be a 1D, 2D, 3D or cube texture.
    pub dimension: TextureDimension,

    /// The initial resource state the texture will take
    pub initial_state: ResourceStates,

    /// An optional clear value that will be 'optimal' for the underlying implementation.
    pub clear_value: Option<ClearValue>,

    /// Number of image array elements.
    ///
    /// A value of '1' means to create a regular, non-array texture. Setting this to a value >1
    /// declares the texture as a texture array.
    pub array_size: u32,

    /// Number of mip levels.
    pub mip_levels: u32,

    /// Sample count, for MSAA texture.
    ///
    /// A value of '1' means a regular, non MSAA texture. This value must always be a power of two.
    /// Setting this to a value >1 declares the texture as an MSAA texture.
    pub sample_count: u32,

    /// Sample quality, for MSAA texture
    pub sample_quality: u32,

    /// Enables the texture to be used with unordered access (unordered access view, storage
    /// texture)
    pub allow_unordered_access: bool,

    /// Enables the texture to be used as a face for a cube map
    pub allow_cube_face: bool,

    /// Enables the texture to be used as a render target
    pub is_render_target: bool,
}

impl Default for TextureDesc {
    #[inline]
    fn default() -> Self {
        Self {
            width: 1,
            height: 1,
            depth: 1,
            format: TextureFormat::R8Unorm,
            dimension: TextureDimension::Texture2D,
            initial_state: ResourceStates::UNKNOWN,
            clear_value: None,
            array_size: 1,
            mip_levels: 1,
            sample_count: 1,
            sample_quality: 0,
            allow_unordered_access: false,
            allow_cube_face: false,
            is_render_target: false,
        }
    }
}

bitflags! {
    pub struct ResourceStates: u32 {
        /// The state of the resource is unknown or undefined.
        const UNKNOWN                   = 0;

        /// General purpose state. Resources in this state can be used as anything.
        ///
        /// # Warning
        ///
        /// This has significant performance implications, especially for textures. Only use this
        /// when absolutely necessary.
        const COMMON                    = 0x00000001;

        /// State allowing use as a constant/uniform buffer.
        const CONSTANT_BUFFER           = 0x00000002;

        /// State allowing use as a vertex buffer.
        const VERTEX_BUFFER             = 0x00000004;

        /// State allowing use as an index buffer.
        const INDEX_BUFFER              = 0x00000008;

        const INDIRECT_ARGUMENT         = 0x00000010;

        const SHADER_RESOURCE           = 0x00000020;

        const UNORDERED_ACCESS          = 0x00000040;

        /// State allowing use as a render target.
        const RENDER_TARGET             = 0x00000080;

        const DEPTH_WRITE               = 0x00000100;

        const DEPTH_READ                = 0x00000200;

        const STREAM_OUT                = 0x00000400;

        /// State allowing use as a copy destination.
        const COPY_DEST                 = 0x00000800;

        /// State allowing use as a copy source.
        const COPY_SOURCE               = 0x00001000;

        const RESOLVE_DEST              = 0x00002000;

        const RESOLVE_SOURCE            = 0x00004000;

        /// State allowing use as a presentation surface. A resource must be in this state on the
        /// GPU timeline for a resource to be used to present from.
        const PRESENT                   = 0x00008000;

        const ACCEL_STRUCT_READ         = 0x00010000;

        const ACCEL_STRUCT_WRITE        = 0x00020000;

        const ACCEL_STRUCT_BUILD_INPUT  = 0x00040000;

        const ACCEL_STRUCT_BUILD_BLAS   = 0x00080000;

        const SHADING_RATE_SURFACE      = 0x00100000;
    }
}

impl Default for ResourceStates {
    #[inline]
    fn default() -> Self {
        ResourceStates::UNKNOWN
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct TextureSubresourceSet {
    pub base_mip_level: u32,
    pub num_mip_levels: u32,
    pub base_array_slice: u32,
    pub num_array_slices: u32,
}

impl Default for TextureSubresourceSet {
    fn default() -> Self {
        Self {
            base_mip_level: 0,
            num_mip_levels: 1,
            base_array_slice: 0,
            num_array_slices: 1,
        }
    }
}
