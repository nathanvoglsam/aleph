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

pub const API_VERSION_MAJOR: &str = env!("CARGO_PKG_VERSION_MAJOR");
pub const API_VERSION_MINOR: &str = env!("CARGO_PKG_VERSION_MINOR");
pub const API_VERSION_PATCH: &str = env!("CARGO_PKG_VERSION_PATCH");

use any::{AnyArc, IAny};
use bitflags::bitflags;
use raw_window_handle::HasRawWindowHandle;
use std::any::Any;
use std::fmt::{Debug, Display, Formatter};
use thiserror::Error;

//
// =================================================================================================
// UTILITY MACROS
// =================================================================================================
//

macro_rules! any_arc_trait_utils_decl {
    ($x: path) => {
        /// Returns an `AnyArc` that points to `self`. This is similar to upgrading a weak
        /// reference. We take a non-owning reference `&dyn SomeTrait` and upgrade it to an owning
        /// `AnyArc<dyn SomeTrait>` handle.
        fn upgrade(&self) -> AnyArc<dyn $x>;

        /// Returns the number of strong references to the object.
        ///
        /// A strong reference is an owning handle to the object (`AnyArc`). The object will remain
        /// alive as long as this remains > 0. The object will be dropped when this reaches 0.
        ///
        /// It is only possible to observe a 0 value for `strong_count` through an `AnyWeak`.
        fn strong_count(&self) -> usize;

        /// Returns the number of weak references to the object.
        ///
        /// A weak reference is a non-owning handle to the object (`AnyWeak`). Weak references do
        /// not extend the lifetime of the object itself, only the ref-count block and the memory
        /// allocation that backs it.
        ///
        /// If `strong_count` is 0 and `weak_count` is >0 then the object is no longer accessible as
        /// it will have been dropped.
        ///
        /// It is only possible to observe a 0 value for `weak_count` through an `AnyArc`.
        fn weak_count(&self) -> usize;
    };
}

//
// =================================================================================================
// ENUMS
// =================================================================================================
//

//
//
// _________________________________________________________________________________________________
// Context

/// Enumeration of all available backends.
#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Debug)]
pub enum BackendAPI {
    Vulkan,
    D3D12,
}

//
//
// _________________________________________________________________________________________________
// Adapter

/// The set of adapter power classes. Primarily used as part of requesting an adapter from the
/// [IContext].
#[derive(Copy, Clone, Debug)]
pub enum AdapterPowerClass {
    /// A low-power adapter refers to the most power efficient GPU installed in the host system.
    ///
    /// e.g. In a laptop with an integrated and discrete GPU, low-power refers to the integrated
    /// GPU as it will almost certainly use less power than the discrete GPU.
    LowPower,

    /// A high-power adapter refers to the highest performance GPU installed in the host system.
    ///
    /// e.g. In a laptop with an integrated and discrete GPU, high-power refers to the discrete GPU
    /// as it will almost certainly be faster than the integrated GPU (otherwise why would it be
    /// installed in the system?).
    HighPower,
}

impl Default for AdapterPowerClass {
    fn default() -> Self {
        Self::LowPower
    }
}

//
//
// _________________________________________________________________________________________________
// SwapChain

#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Debug)]
pub enum PresentationMode {
    Immediate,
    Mailbox,
    Fifo,
}

impl Default for PresentationMode {
    fn default() -> Self {
        Self::Immediate
    }
}

impl Display for PresentationMode {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            PresentationMode::Immediate => f.write_str("PresentationMode::Immediate"),
            PresentationMode::Mailbox => f.write_str("PresentationMode::Mailbox"),
            PresentationMode::Fifo => f.write_str("PresentationMode::Fifo"),
        }
    }
}

//
//
// _________________________________________________________________________________________________
// Resources

/// Enumeration of all CPU access modes for resources
#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Debug)]
pub enum CpuAccessMode {
    /// Resource can not be accessed by the CPU at all (device local)
    None,

    /// Resource can be read by the CPU (read back)
    Read,

    /// Resource can be written by the CPU (upload)
    Write,
}

impl Default for CpuAccessMode {
    fn default() -> Self {
        Self::None
    }
}

//
//
// _________________________________________________________________________________________________
// Resources - Texture

#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Debug)]
pub enum TextureFormat {
    R8Unorm,
    R8Snorm,
    R8Uint,
    R8Sint,
    R16Uint,
    R16Sint,
    R16Unorm,
    R16Snorm,
    R16Float,
    Rg8Unorm,
    Rg8Snorm,
    Rg8Uint,
    Rg8Sint,
    R32Uint,
    R32Sint,
    R32Float,
    Rg16Uint,
    Rg16Sint,
    Rg16Unorm,
    Rg16Snorm,
    Rg16Float,
    Rgba8Unorm,
    Rgba8UnormSrgb,
    Rgba8Snorm,
    Rgba8Uint,
    Rgba8Sint,
    Bgra8Unorm,
    Bgra8UnormSrgb,
    Rgb10a2Unorm,
    Rg11b10Float,
    Rg32Uint,
    Rg32Sint,
    Rg32Float,
    Rgba16Uint,
    Rgba16Sint,
    Rgba16Unorm,
    Rgba16Snorm,
    Rgba16Float,
    Rgba32Uint,
    Rgba32Sint,
    Rgba32Float,
    Depth32Float,
    Depth24Stencil8,
}

impl Default for TextureFormat {
    fn default() -> Self {
        Self::R8Unorm
    }
}

impl TextureFormat {
    /// Returns whether the format is a depth texture format
    pub fn is_depth(&self) -> bool {
        matches!(self, Self::Depth32Float)
    }

    /// Returns whether the format is a stencil texture format
    pub fn is_stencil(&self) -> bool {
        matches!(self, Self::Depth24Stencil8)
    }

    /// Returns whether the format is a depth/stencil texture format
    pub fn is_depth_stencil(&self) -> bool {
        matches!(self, Self::Depth32Float | Self::Depth24Stencil8)
    }

    /// Returns whether the format is a float format
    pub fn is_float(&self) -> bool {
        matches!(
            self,
            Self::R16Float
                | Self::R32Float
                | Self::Rg16Float
                | Self::Rg11b10Float
                | Self::Rg32Float
                | Self::Rgba16Float
                | Self::Rgba32Float
                | Self::Depth32Float
        )
    }

    /// Returns whether the format is a signed-int format
    pub fn is_sint(&self) -> bool {
        matches!(
            self,
            Self::R8Sint
                | Self::R16Sint
                | Self::Rg8Sint
                | Self::R32Sint
                | Self::Rg16Sint
                | Self::Rgba8Sint
                | Self::Rg32Sint
                | Self::Rgba16Sint
                | Self::Rgba32Sint
        )
    }

    /// Returns whether the format is an unsigned-int format
    pub fn is_uint(&self) -> bool {
        matches!(
            self,
            Self::R8Uint
                | Self::R16Uint
                | Self::Rg8Uint
                | Self::R32Uint
                | Self::Rg16Uint
                | Self::Rgba8Uint
                | Self::Rg32Uint
                | Self::Rgba16Uint
                | Self::Rgba32Uint
        )
    }

    /// Returns whether the format is a signed-normalized-int format
    pub fn is_snorm(&self) -> bool {
        matches!(
            self,
            Self::R8Snorm
                | Self::R16Snorm
                | Self::Rg8Snorm
                | Self::Rg16Snorm
                | Self::Rgba8Snorm
                | Self::Rgba16Snorm
        )
    }

    /// Returns whether the format is an unsigned-normalized-int format
    pub fn is_unorm(&self) -> bool {
        matches!(
            self,
            Self::R8Unorm
                | Self::R16Unorm
                | Self::Rg8Unorm
                | Self::Rg16Unorm
                | Self::Rgba8Unorm
                | Self::Rgba8UnormSrgb
                | Self::Bgra8Unorm
                | Self::Bgra8UnormSrgb
                | Self::Rgb10a2Unorm
                | Self::Rgba16Unorm
        )
    }
}

impl Display for TextureFormat {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            TextureFormat::R8Unorm => f.write_str("TextureFormat::R8Unorm"),
            TextureFormat::R8Snorm => f.write_str("TextureFormat::R8Snorm"),
            TextureFormat::R8Uint => f.write_str("TextureFormat::R8Uint"),
            TextureFormat::R8Sint => f.write_str("TextureFormat::R8Sint"),
            TextureFormat::R16Uint => f.write_str("TextureFormat::R16Uint"),
            TextureFormat::R16Sint => f.write_str("TextureFormat::R16Sint"),
            TextureFormat::R16Unorm => f.write_str("TextureFormat::R16Unorm"),
            TextureFormat::R16Snorm => f.write_str("TextureFormat::R16Snorm"),
            TextureFormat::R16Float => f.write_str("TextureFormat::R16Float"),
            TextureFormat::Rg8Unorm => f.write_str("TextureFormat::Rg8Unorm"),
            TextureFormat::Rg8Snorm => f.write_str("TextureFormat::Rg8Snorm"),
            TextureFormat::Rg8Uint => f.write_str("TextureFormat::Rg8Uint"),
            TextureFormat::Rg8Sint => f.write_str("TextureFormat::Rg8Sint"),
            TextureFormat::R32Uint => f.write_str("TextureFormat::R32Uint"),
            TextureFormat::R32Sint => f.write_str("TextureFormat::R32Sint"),
            TextureFormat::R32Float => f.write_str("TextureFormat::R32Float"),
            TextureFormat::Rg16Uint => f.write_str("TextureFormat::Rg16Uint"),
            TextureFormat::Rg16Sint => f.write_str("TextureFormat::Rg16Sint"),
            TextureFormat::Rg16Unorm => f.write_str("TextureFormat::Rg16Unorm"),
            TextureFormat::Rg16Snorm => f.write_str("TextureFormat::Rg16Snorm"),
            TextureFormat::Rg16Float => f.write_str("TextureFormat::Rg16Float"),
            TextureFormat::Rgba8Unorm => f.write_str("TextureFormat::Rgba8Unorm"),
            TextureFormat::Rgba8UnormSrgb => f.write_str("TextureFormat::Rgba8UnormSrgb"),
            TextureFormat::Rgba8Snorm => f.write_str("TextureFormat::Rgba8Snorm"),
            TextureFormat::Rgba8Uint => f.write_str("TextureFormat::Rgba8Uint"),
            TextureFormat::Rgba8Sint => f.write_str("TextureFormat::Rgba8Sint"),
            TextureFormat::Bgra8Unorm => f.write_str("TextureFormat::Bgra8Unorm"),
            TextureFormat::Bgra8UnormSrgb => f.write_str("TextureFormat::Bgra8UnormSrgb"),
            TextureFormat::Rgb10a2Unorm => f.write_str("TextureFormat::Rgb10a2Unorm"),
            TextureFormat::Rg11b10Float => f.write_str("TextureFormat::Rg11b10Float"),
            TextureFormat::Rg32Uint => f.write_str("TextureFormat::Rg32Uint"),
            TextureFormat::Rg32Sint => f.write_str("TextureFormat::Rg32Sint"),
            TextureFormat::Rg32Float => f.write_str("TextureFormat::Rg32Float"),
            TextureFormat::Rgba16Uint => f.write_str("TextureFormat::Rgba16Uint"),
            TextureFormat::Rgba16Sint => f.write_str("TextureFormat::Rgba16Sint"),
            TextureFormat::Rgba16Unorm => f.write_str("TextureFormat::Rgba16Unorm"),
            TextureFormat::Rgba16Snorm => f.write_str("TextureFormat::Rgba16Snorm"),
            TextureFormat::Rgba16Float => f.write_str("TextureFormat::Rgba16Float"),
            TextureFormat::Rgba32Uint => f.write_str("TextureFormat::Rgba32Uint"),
            TextureFormat::Rgba32Sint => f.write_str("TextureFormat::Rgba32Sint"),
            TextureFormat::Rgba32Float => f.write_str("TextureFormat::Rgba32Float"),
            TextureFormat::Depth32Float => f.write_str("TextureFormat::Depth32Float"),
            TextureFormat::Depth24Stencil8 => f.write_str("TextureFormat::Depth24Stencil8"),
        }
    }
}

/// Enumeration about all major texture types.
#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Debug)]
pub enum TextureDimension {
    /// One dimensional texture. Logically similar to a 2D image with a height of 1
    Texture1D,

    /// A standard 2D texture.
    Texture2D,

    /// A 3D volume texture.
    Texture3D,
}

impl Default for TextureDimension {
    fn default() -> Self {
        Self::Texture1D
    }
}

/// An enumeration of all possible input types for initializing a texture's optimal clear color
/// value
#[derive(Clone, Debug, PartialEq)]
pub enum OptimalClearValue {
    /// A full 4-channel f32 colour
    ColorF32 { r: f32, g: f32, b: f32, a: f32 },

    /// A 4-channel color packed into a single u32
    ColorInt(u32),

    /// A floating point + u8 pair for clearing a depth stencil texture
    DepthStencil(f32, u8),
}

impl From<u32> for OptimalClearValue {
    fn from(v: u32) -> Self {
        Self::ColorInt(v)
    }
}

impl Display for OptimalClearValue {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            OptimalClearValue::ColorF32 { r, g, b, a } => {
                write!(f, "OptimalClearValue::ColorF32({}, {}, {}, {})", r, g, b, a)
            }
            OptimalClearValue::ColorInt(v) => {
                write!(f, "OptimalClearValue::ColorInt({:X})", *v)
            }
            OptimalClearValue::DepthStencil(depth, stencil) => {
                write!(f, "OptimalClearValue::DepthStencil({}, {})", depth, stencil)
            }
        }
    }
}

//
//
// _________________________________________________________________________________________________
// Resources - Sampler

#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Debug)]
pub enum SamplerAddressMode {
    Wrap,
    Mirror,
    Clamp,
    Border,
    MirrorOnce,
}

impl Default for SamplerAddressMode {
    fn default() -> Self {
        Self::Wrap
    }
}

#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Debug)]
pub enum SamplerFilter {
    Nearest,
    Linear,
}

impl Default for SamplerFilter {
    fn default() -> Self {
        Self::Nearest
    }
}

#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Debug)]
pub enum SamplerMipFilter {
    Nearest,
    Linear,
}

#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Debug)]
pub enum SamplerComparisonOp {
    Never,
    Less,
    Equal,
    LessEqual,
    Greater,
    NotEqual,
    GreaterOrEqual,
    Always,
}

//
//
// _________________________________________________________________________________________________
// Resources - Shader

/// An enumeration of the supported set of shader input types.
#[derive(Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Debug)]
pub enum ShaderBinary<'a> {
    /// This variant encloses a SPIR-V binary. Only supported by the `Vulkan` backend.
    Spirv(&'a [u8]),

    /// This variant encloses a DXIL binary. Only supported by the `D3D12` backend.
    Dxil(&'a [u8]),
}

/// An enumeration of all individual shader types
#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Debug)]
pub enum ShaderType {
    Compute,
    Vertex,
    Hull,
    Domain,
    Geometry,
    Fragment,
    Amplification,
    Mesh,
}

impl Default for ShaderType {
    fn default() -> Self {
        Self::Compute
    }
}

//
//
// _________________________________________________________________________________________________
// Descriptors

#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Debug)]
pub enum DescriptorType {
    Texture,
    TypedBuffer,
    StructuredBuffer,
    RawBuffer,
    ConstantBuffer,
    Sampler,
    PushConstants { size: u16 },
}

impl Default for DescriptorType {
    fn default() -> Self {
        Self::Texture
    }
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

impl Default for DescriptorShaderVisibility {
    fn default() -> Self {
        Self::All
    }
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

//
//
// _________________________________________________________________________________________________
// Queue

#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Debug)]
pub enum QueueType {
    General,
    Compute,
    Transfer,
}

impl Default for QueueType {
    fn default() -> Self {
        Self::General
    }
}

impl Display for QueueType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            QueueType::General => f.write_str("QueueType::General"),
            QueueType::Compute => f.write_str("QueueType::Compute"),
            QueueType::Transfer => f.write_str("QueueType::Transfer"),
        }
    }
}

//
//
// _________________________________________________________________________________________________
// Command Options

/// An enumeration of all possible input types to a color texture clear operation
#[derive(Clone, Debug, PartialEq)]
pub enum ColorClearValue {
    /// A full 4-channel f32 colour
    Float { r: f32, g: f32, b: f32, a: f32 },

    /// A 4-channel color packed into a single u32
    Int(u32),
}

impl From<u32> for ColorClearValue {
    fn from(v: u32) -> Self {
        Self::Int(v)
    }
}

impl Display for ColorClearValue {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            ColorClearValue::Float { r, g, b, a } => {
                write!(f, "ColorClearValue::Float({}, {}, {}, {})", r, g, b, a)
            }
            ColorClearValue::Int(v) => {
                write!(f, "ColorClearValue::Int({:X})", *v)
            }
        }
    }
}

/// An enumeration of all possible input types to a depth/stencil texture clear operation
#[derive(Clone, Debug, PartialEq)]
pub enum DepthStencilClearValue {
    /// A floating point + u8 pair for clearing a depth stencil texture
    DepthStencil(f32, u8),

    /// A floating point value for clearing only depth
    Depth(f32),

    /// A u8 value for clearing only stencil
    Stencil(u8),
}

impl Display for DepthStencilClearValue {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            DepthStencilClearValue::DepthStencil(depth, stencil) => {
                write!(f, "ColorClearValue::Float({}, {})", *depth, *stencil)
            }
            DepthStencilClearValue::Depth(v) => {
                write!(f, "DepthStencilClearValue::Depth({})", *v)
            }
            DepthStencilClearValue::Stencil(v) => {
                write!(f, "DepthStencilClearValue::Stencil({})", *v)
            }
        }
    }
}

/// Enum flags for barrier commands for specifying the split barrier behavior.
#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Debug)]
pub enum SplitBarrierMode {
    /// A regular, non split barrier
    None,

    /// Flags the barrier as the begin half of a split barrier
    Begin,

    /// Flags the barrier as the end half of a split barrier
    End,
}

impl Default for SplitBarrierMode {
    fn default() -> Self {
        Self::None
    }
}

/// Enum flags for barrier commands for specifying queue ownership transition behavior.
#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Debug)]
pub enum QueueTransitionMode {
    /// No queue ownership transition will be performed
    None,

    /// Flag the barrier to acquire the resource from the queue provided
    Acquire(QueueType),

    /// Flag the barrier to release the flag to the queue provided
    Release(QueueType),
}

impl Default for QueueTransitionMode {
    fn default() -> Self {
        Self::None
    }
}

//
// =================================================================================================
// STRUCTURES
// =================================================================================================
//

//
//
// _________________________________________________________________________________________________
// Context

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
    /// validation layers. Vulkan requires `VK_EXT_debug_utils` for object naming as that is the
    /// extension that provides the naming functionality.
    pub debug: bool,
}

//
//
// _________________________________________________________________________________________________
// Adapter

#[derive(Clone)]
pub struct AdapterRequestOptions<'a> {
    /// A handle to an [ISurface] which the device adapter must be able to render and present to.
    ///
    /// Can be set to `None` to indicate we aren't going to present. Useful for compute-only
    /// workloads.
    pub surface: Option<&'a dyn ISurface>,

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

            // 99.9999% users will ask for the HighPower adapter so we default to that.
            power_class: AdapterPowerClass::HighPower,
        }
    }
}

impl<'a> Debug for AdapterRequestOptions<'a> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("AdapterRequestOptions")
            .field("surface", &self.surface.as_ref().map(|_| "<ptr>"))
            .field("power_class", &self.power_class)
            .finish()
    }
}

#[derive(Clone, Hash, PartialEq, Eq, Debug)]
pub struct AdapterDescription<'a> {
    /// The name of the adapter
    pub name: &'a str,
}

//
//
// _________________________________________________________________________________________________
// SwapChain

#[derive(Clone, Hash, PartialEq, Eq, Debug, Default)]
pub struct SwapChainConfiguration {
    pub format: TextureFormat,
    pub width: u32,
    pub height: u32,
    pub present_mode: PresentationMode,
    pub preferred_queue: QueueType,
}

//
//
// _________________________________________________________________________________________________
// Resource States

bitflags! {
    pub struct ResourceStates: u32 {
        const UNDEFINED = 0;
        const VERTEX_AND_CONSTANT_BUFFER = 0x1;
        const INDEX_BUFFER = 0x2;
        const RENDER_TARGET = 0x4;
        const UNORDERED_ACCESS = 0x8;
        const DEPTH_WRITE = 0x10;
        const DEPTH_READ = 0x20;
        const NON_PIXEL_SHADER_RESOURCE = 0x40;
        const PIXEL_SHADER_RESOURCE = 0x80;
        const SHADER_RESOURCE = 0x40 | 0x80;
        const STREAM_OUT = 0x100;
        const INDIRECT_ARGUMENT = 0x200;
        const COPY_DEST = 0x400;
        const COPY_SOURCE = 0x800;
        const GENERIC_READ = 0x1 | 0x2 | 0x40 | 0x80 | 0x200 | 0x800;
        const PRESENT = 0x1000;
        const COMMON = 0x2000;
        // const RAYTRACING_ACCELERATION_STRUCTURE = 0x4000;
        // const SHADING_RATE_SOURCE = 0x8000;
    }
}

impl Default for ResourceStates {
    #[inline]
    fn default() -> Self {
        ResourceStates::UNDEFINED
    }
}

//
//
// _________________________________________________________________________________________________
// Resource Descriptions - Buffer

/// Description object used for creating a new buffer.
#[derive(Clone, Hash, PartialEq, Eq, Debug, Default)]
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

//
//
// _________________________________________________________________________________________________
// Resource Descriptions - Texture

/// Description object used for creating a new texture.
#[derive(Clone, PartialEq, Debug, Default)]
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
    pub clear_value: Option<OptimalClearValue>,

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

//
//
// _________________________________________________________________________________________________
// Resource Descriptions - Sampler

#[derive(Clone, Debug, PartialEq)]
pub struct SamplerDesc {
    pub min_filter: SamplerFilter,
    pub mag_filter: SamplerFilter,
    pub mip_filter: SamplerMipFilter,
    pub address_mode_u: SamplerAddressMode,
    pub address_mode_v: SamplerAddressMode,
    pub address_mode_w: SamplerAddressMode,
    pub lod_bias: f32,
    pub min_lod: f32,
    pub max_lod: f32,
    pub enable_anisotropy: bool,
    pub max_anisotropy: f32,
    pub compare_op: SamplerComparisonOp,
    // TODO: Border colour
}

//
//
// _________________________________________________________________________________________________
// Resource Descriptions - Shader

/// Set of options for creating a new shader module
#[derive(Clone, Hash, PartialEq, Eq, Debug)]
pub struct ShaderOptions<'a> {
    pub shader_type: ShaderType,
    pub data: ShaderBinary<'a>,
    pub entry_point: &'a str,
}

//
//
// _________________________________________________________________________________________________
// Descriptor Set Layout

#[derive(Clone, Debug, PartialEq, Eq, Hash, Default)]
pub struct DescriptorSetLayoutDescItem {
    pub binding_num: u32,
    pub binding_type: DescriptorType,
    pub read_only: bool,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash, Default)]
pub struct DescriptorSetLayoutDesc<'a> {
    pub visibility: DescriptorShaderVisibility,
    pub items: &'a [DescriptorSetLayoutDescItem],
}

//
//
// _________________________________________________________________________________________________
// Command Payloads

#[derive(Clone, Debug, PartialEq, Eq, Hash, Default)]
pub struct TextureSubResourceSet {
    pub base_mip_level: u32,
    pub num_mip_levels: u32,
    pub base_array_slice: u32,
    pub num_array_slices: u32,
}

/// Describes a resource barrier that will apply to an [IBuffer] resource on a command queue
#[derive(Clone)]
pub struct BufferBarrier<'a> {
    /// The buffer that the barrier will describe a state transition for
    pub buffer: &'a dyn IBuffer,

    /// The state the buffer is in before the barrier is executed
    pub before_state: ResourceStates,

    /// The state the buffer will transition to after the barrier is executed
    pub after_state: ResourceStates,

    /// Enables describing split barriers, where one barrier begins a transition and another ends
    /// the transition. This allows interleaving other rendering commands with state transitions.
    pub split_barrier_mode: SplitBarrierMode,

    /// Enables describing a queue ownership transition. Ownership of resources must be explicitly
    /// passed from one queue to another to be used across multiple queues.
    pub queue_transition_mode: QueueTransitionMode,
}

impl<'a> Debug for BufferBarrier<'a> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("BufferBarrier")
            .field("buffer", &"<ptr>")
            .field("before_state", &self.before_state)
            .field("after_state", &self.after_state)
            .field("split_barrier_mode", &self.split_barrier_mode)
            .field("queue_transition_mode", &self.queue_transition_mode)
            .finish()
    }
}

/// Structure used by [TextureBarrier] for describing which image sub resource to make the subject
/// of a resource barrier.
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash, Default)]
pub struct BarrierSubresourceOptions {
    pub mip_level: u8,
    pub array_layer: u16,
}

/// Describes a resource barrier that will apply to an [ITexture] resource on a command queue
#[derive(Clone)]
pub struct TextureBarrier<'a> {
    /// The texture that the barrier will describe a state transition for
    pub texture: &'a dyn ITexture,

    /// The state the texture is in before the barrier is executed
    pub before_state: ResourceStates,

    /// The state the texture will transition to after the barrier is executed
    pub after_state: ResourceStates,

    /// Enables describing split barriers, where one barrier begins a transition and another ends
    /// the transition. This allows interleaving other rendering commands with state transitions.
    pub split_barrier_mode: SplitBarrierMode,

    /// Enables describing a queue ownership transition. Ownership of resources must be explicitly
    /// passed from one queue to another to be used across multiple queues.
    pub queue_transition_mode: QueueTransitionMode,

    /// Enables specifying the buffer affect only a specific sub-resource of the texture. When left
    /// as `None` the entire texture will be affected by the barrier.
    pub subresource: Option<BarrierSubresourceOptions>,
}

impl<'a> Debug for TextureBarrier<'a> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("TextureBarrier")
            .field("texture", &"<ptr>")
            .field("before_state", &self.before_state)
            .field("after_state", &self.after_state)
            .field("split_barrier_mode", &self.split_barrier_mode)
            .field("queue_transition_mode", &self.queue_transition_mode)
            .field("subresource", &self.subresource)
            .finish()
    }
}

//
// =================================================================================================
// INTERFACES
// =================================================================================================
//

//
//
// _________________________________________________________________________________________________
// ContextProvider

/// Entry point of the RHI. This interface is intended to be installed into a plugin registry where
/// some other use can request a handle to the [IContextProvider] instance and create the context.
pub trait IContextProvider: IAny + 'static {
    /// Creates the RHI [IContext] object. This can only succeed once. Calling this more than once
    /// will always return Err.
    fn make_context(
        &self,
        options: &ContextOptions,
    ) -> Result<AnyArc<dyn IContext>, ContextCreateError>;
}

//
//
// _________________________________________________________________________________________________
// Context

/// Represents the underlying API context. Handles creating surfaces from window handles, and
/// retrieving.
pub trait IContext: IAny + 'static {
    any_arc_trait_utils_decl!(IContext);

    /// Create an adapter that suitably meets the requested requirements and preferences specified
    /// by `options`. Will return `None` if no adapter meeting the requirements could be found.
    fn request_adapter(&self, options: &AdapterRequestOptions) -> Option<AnyArc<dyn IAdapter>>;

    /// Create a surface from the provided window handle.
    fn create_surface(
        &self,
        window: &dyn HasRawWindowHandle,
    ) -> Result<AnyArc<dyn ISurface>, SurfaceCreateError>;

    /// Returns the API used by the underlying backend implementation.
    fn get_backend_api(&self) -> BackendAPI;
}

//
//
// _________________________________________________________________________________________________
// Surface

/// Represents the graphics API's handle to the window or monitor surface. SwapChains are created
/// from surfaces.
///
/// A surface is not tied to a specific [IDevice], it represents an API level handle to a rendering
/// surface. As such [ISurface] is not created by an [IDevice], rather it is created by the
/// [IContext]. An [IDevice] will be selected and created based on its compatibility with an
/// [ISurface].
pub trait ISurface: IAny + 'static {
    any_arc_trait_utils_decl!(ISurface);

    fn create_swap_chain(
        &self,
        device: &dyn IDevice,
        config: &SwapChainConfiguration,
    ) -> Result<AnyArc<dyn ISwapChain>, SwapChainCreateError>;
}

//
//
// _________________________________________________________________________________________________
// Adapter

/// Represents some GPU device installed in the system. An adapter is used to create an [IDevice].
pub trait IAdapter: IAny + 'static {
    any_arc_trait_utils_decl!(IAdapter);

    /// Returns the [AdapterDescription] that provides information about this specific adapter.
    fn description(&self) -> AdapterDescription;

    /// Requests an IDevice
    fn request_device(&self) -> Result<AnyArc<dyn IDevice>, RequestDeviceError>;
}

//
//
// _________________________________________________________________________________________________
// Device

pub trait IDevice: INamedObject + Send + Sync + IAny + Any + 'static {
    any_arc_trait_utils_decl!(IDevice);

    /// Triggers a non blocking garbage collection cycle. This must be called for resources used in
    /// command lists to be freed. It is recommended to call this at least once per frame.
    fn garbage_collect(&self);

    /// Block the calling thread until all GPU queues are flushed of work. This is similar to
    /// vkDeviceWaitIdle.
    ///
    /// This will also trigger a GC cycle, freeing the releases from the now completed command
    /// lists.
    fn wait_idle(&self);

    fn create_shader(
        &self,
        options: &ShaderOptions,
    ) -> Result<AnyArc<dyn IShader>, ShaderCreateError>;

    fn create_descriptor_set_layout(&self, desc: &DescriptorSetLayoutDesc);

    fn create_buffer(&self, desc: &BufferDesc) -> Result<AnyArc<dyn IBuffer>, BufferCreateError>;

    fn create_texture(
        &self,
        desc: &TextureDesc,
    ) -> Result<AnyArc<dyn ITexture>, TextureCreateError>;

    fn create_sampler(
        &self,
        desc: &SamplerDesc,
    ) -> Result<AnyArc<dyn ISampler>, SamplerCreateError>;

    fn create_command_pool(&self) -> Result<AnyArc<dyn ICommandPool>, CommandPoolCreateError>;

    ///
    /// # Safety
    ///
    /// It is the caller's responsibility to ensure that the command lists submitted to the GPU
    /// contain a valid command stream.
    ///
    /// The GPU interfaces will uphold resource lifetime requirements and CPU synchronization
    /// requirements, but makes a very limited effort to handle GPU synchronization. It is up to the
    /// caller to record correct barriers.
    ///
    unsafe fn general_queue_submit_list(
        &self,
        command_list: Box<dyn IGeneralCommandList>,
    ) -> Result<(), QueueSubmitError>;

    ///
    /// # Safety
    ///
    /// It is the caller's responsibility to ensure that the command lists submitted to the GPU
    /// contain a valid command stream.
    ///
    /// The GPU interfaces will uphold resource lifetime requirements and CPU synchronization
    /// requirements, but makes a very limited effort to handle GPU synchronization. It is up to the
    /// caller to record correct barriers.
    ///
    unsafe fn general_queue_submit_lists(
        &self,
        command_lists: &mut dyn Iterator<Item = Box<dyn IGeneralCommandList>>,
    ) -> Result<(), QueueSubmitError>;

    ///
    /// # Safety
    ///
    /// It is the caller's responsibility to ensure that the image that is being presented will be
    /// in the required resource state for presentation by the time this operation will be executed
    /// on the GPU timeline.
    ///
    unsafe fn general_queue_present(
        &self,
        image: Box<dyn IAcquiredTexture>,
    ) -> Result<(), QueuePresentError>;

    /// Returns the API used by the underlying backend implementation.
    fn get_backend_api(&self) -> BackendAPI;
}

//
//
// _________________________________________________________________________________________________
// SwapChain

pub trait ISwapChain: INamedObject + IAny + 'static {
    any_arc_trait_utils_decl!(ISwapChain);

    /// Returns whether support operations are supported on the given queue.
    fn present_supported_on_queue(&self, queue: QueueType) -> bool;

    /// Force a resize of the swap chain. Will block until the swap chain is no longer in use before
    /// performing the resize operation.
    fn queue_resize(&self, width: u32, height: u32);

    /// Returns a [SwapChainConfiguration] that describes the state of the swap chain at the time
    /// of the function being called.
    ///
    /// The state may change after this function is called. If a rebuild was needed internally in
    /// [ISwapChain::acquire_image] then the size may be different once the
    /// [ISwapChain::acquire_image] call returns.
    fn get_config(&self) -> SwapChainConfiguration;

    /// Acquire an image from the swap chain for use with rendering
    fn acquire_image(&self) -> Result<Box<dyn IAcquiredTexture>, AcquireImageError>;
}

//
//
// _________________________________________________________________________________________________
// Resources

pub trait IBuffer: INamedObject + Send + Sync + IAny + Any + 'static {
    any_arc_trait_utils_decl!(IBuffer);

    fn desc(&self) -> &BufferDesc;
}

pub trait ITexture: INamedObject + Send + Sync + IAny + Any + 'static {
    any_arc_trait_utils_decl!(ITexture);

    fn desc(&self) -> &TextureDesc;
}

pub trait IAcquiredTexture: IAny + Send + 'static {
    fn image(&self) -> &dyn ITexture;
}

pub trait ISampler: INamedObject + Send + Sync + IAny + Any + 'static {
    any_arc_trait_utils_decl!(ISampler);
}

//
//
// _________________________________________________________________________________________________
// Command Encoders

pub trait IGeneralEncoder: IComputeEncoder + Send {
    unsafe fn clear_texture(
        &mut self,
        texture: &dyn ITexture,
        sub_resources: &TextureSubResourceSet,
        value: &ColorClearValue,
    );
    unsafe fn clear_depth_stencil_texture(
        &mut self,
        texture: &dyn ITexture,
        sub_resources: &TextureSubResourceSet,
        value: &DepthStencilClearValue,
    );
    unsafe fn draw(
        &mut self,
        vertex_count: u32,
        instance_count: u32,
        first_vertex: u32,
        first_instance: u32,
    );
    unsafe fn draw_indexed(
        &mut self,
        index_count: u32,
        instance_count: u32,
        first_index: u32,
        first_instance: u32,
        vertex_offset: i32,
    );
}

pub trait IComputeEncoder: ITransferEncoder + Send {
    unsafe fn resource_barrier(
        &mut self,
        buffer_barriers: &[BufferBarrier],
        texture_barriers: &[TextureBarrier],
    );

    unsafe fn dispatch(&mut self, group_count_x: u32, group_count_y: u32, group_count_z: u32);
}

pub trait ITransferEncoder: Send {}

//
//
// _________________________________________________________________________________________________
// Command Lists

pub trait IGeneralCommandList: INamedObject + Send + IAny + Any + 'static {
    fn begin<'a>(&'a mut self) -> Result<Box<dyn IGeneralEncoder + 'a>, CommandListBeginError>;
}

pub trait IComputeCommandList: INamedObject + Send + IAny + Any + 'static {
    fn begin<'a>(&'a mut self) -> Result<Box<dyn IComputeEncoder + 'a>, CommandListBeginError>;
}

pub trait ITransferCommandList: INamedObject + Send + IAny + Any + 'static {
    fn begin<'a>(&'a mut self) -> Result<Box<dyn ITransferEncoder + 'a>, CommandListBeginError>;
}

//
//
// _________________________________________________________________________________________________
// CommandPool

pub trait ICommandPool: INamedObject + Send + Sync + IAny + Any + 'static {
    any_arc_trait_utils_decl!(ICommandPool);

    fn create_general_command_list(
        &self,
    ) -> Result<Box<dyn IGeneralCommandList>, CommandListCreateError>;
}

//
//
// _________________________________________________________________________________________________
// Descriptors

pub trait IDescriptorSetLayout: INamedObject + Send + Sync + IAny + Any + 'static {
    any_arc_trait_utils_decl!(IDescriptorSetLayout);
}

pub trait IDescriptorSet: INamedObject + Send + Sync + IAny + Any + 'static {
    any_arc_trait_utils_decl!(IDescriptorSet);
}

//
//
// _________________________________________________________________________________________________
// Pipeline Objects

pub trait IGraphicsPipeline: INamedObject + Send + Sync + IAny + Any + 'static {
    any_arc_trait_utils_decl!(IGraphicsPipeline);
}

pub trait IComputePipeline: INamedObject + Send + Sync + IAny + Any + 'static {
    any_arc_trait_utils_decl!(IComputePipeline);
}

//
//
// _________________________________________________________________________________________________
// Shader

pub trait IShader: INamedObject + Send + Sync + IAny + Any + 'static {
    any_arc_trait_utils_decl!(IShader);

    fn shader_type(&self) -> ShaderType;
    fn entry_point(&self) -> &str;
}

//
//
// _________________________________________________________________________________________________
// NamedObject

/// A common trait definition shared by any API object that can be given a name for debug purposes.
///
/// Vulkan and D3D12 have debug functionality that allow the user to attach a string name to API
/// objects for debug purposes. This exposes that functionality.
pub trait INamedObject {
    /// Attach a name to the API object for debug purposes. This will show up associated with the
    /// underlying backend API objects in graphics debuggers
    fn set_name(&self, name: &str);
}

//
// =================================================================================================
// ERROR TYPES
// =================================================================================================
//

//
//
// _________________________________________________________________________________________________
// Context

/// Set of errors that can occur when creating an [IContext]
#[derive(Error, Debug)]
#[non_exhaustive]
pub enum ContextCreateError {
    #[error("A context has already been created by this provider")]
    ContextAlreadyCreated,

    #[error("An internal backend error has occurred '{0}'")]
    Platform(#[from] anyhow::Error),
}

//
//
// _________________________________________________________________________________________________
// Surface

/// Set of errors that can occur when creating an [ISurface]
#[derive(Error, Debug)]
#[non_exhaustive]
pub enum SurfaceCreateError {
    #[error("An internal backend error has occurred '{0}'")]
    Platform(#[from] anyhow::Error),
}

//
//
// _________________________________________________________________________________________________
// SwapChain

#[derive(Error, Debug)]
#[non_exhaustive]
pub enum SwapChainCreateError {
    #[error("The requested image format '{0}' is not supported by the swap chain")]
    UnsupportedFormat(TextureFormat),

    #[error("The requested image usage is not supported by the swap chain")]
    UnsupportedUsage(()),

    #[error("The requested width '{0}' is not supported by the swap chain")]
    UnsupportedWidth(u32),

    #[error("The requested height '{0}' is not supported by the swap chain")]
    UnsupportedHeight(u32),

    #[error("The requested presentation mode '{0}' is not supported by the swap chain")]
    UnsupportedPresentMode(PresentationMode),

    #[error("There is no queue available for the swap chain to be attached to")]
    NoQueueAvailable,

    #[error("The surface is already owned by another existing swap chain")]
    SurfaceAlreadyOwned,

    /// For a detailed explanation see [AcquireImageError::SurfaceNotAvailable]
    #[error("The surface is currently in a state where it can not be used")]
    SurfaceNotAvailable,

    #[error("An internal backend error has occurred '{0}'")]
    Platform(#[from] anyhow::Error),
}

//
//
// _________________________________________________________________________________________________
// Device

/// Set of errors that can occur when creating an [IDevice]
#[derive(Error, Debug)]
#[non_exhaustive]
pub enum RequestDeviceError {
    #[error("An internal backend error has occurred '{0}'")]
    Platform(#[from] anyhow::Error),
}

//
//
// _________________________________________________________________________________________________
// Resource Construction - Buffer

#[derive(Error, Debug)]
pub enum BufferCreateError {
    #[error("An internal backend error has occurred '{0}'")]
    Platform(#[from] anyhow::Error),
}

//
//
// _________________________________________________________________________________________________
// Resource Construction - Texture

#[derive(Error, Debug)]
pub enum TextureCreateError {
    #[error("Requested texture width '{0}' is invalid")]
    InvalidWidth(u32),

    #[error("Requested texture height '{0}' is invalid")]
    InvalidHeight(u32),

    #[error("Requested texture depth '{0}' is invalid")]
    InvalidDepth(u32),

    #[error("Requested texture array size '{0}' is invalid")]
    InvalidArraySize(u32),

    #[error("Requested texture mip level count '{0}' is invalid")]
    InvalidMipLevelCount(u32),

    #[error("Requested sample count '{0}' is invalid")]
    InvalidSampleCount(u32),

    #[error("Requested optimal clear value '{0}' is invalid")]
    InvalidOptimalClearValue(OptimalClearValue),

    #[error("An internal backend error has occurred '{0}'")]
    Platform(#[from] anyhow::Error),
}

//
//
// _________________________________________________________________________________________________
// Resource Construction - Sampler

#[derive(Error, Debug)]
pub enum SamplerCreateError {
    #[error("An internal backend error has occurred '{0}'")]
    Platform(#[from] anyhow::Error),
}

//
//
// _________________________________________________________________________________________________
// Resource Construction - Shader

#[derive(Error, Debug)]
pub enum ShaderCreateError {
    /// This error occurs when the byte size of the shader blob is of an invalid size.
    ///
    /// Invalid sizes include:
    ///     - 0
    ///     - Non multiples of 4 (on Vulkan)
    ///
    /// # Vulkan
    ///
    /// Vulkan consumes SPIR-V as the shader blob. SPIR-V is encoded as a sequence of `u32` values.
    /// It is impossible for a valid SPIR-V binary to have a size that is not a multiple of 4 (the
    /// size of a u32) for this reason.
    #[error("The shader binary size '{0}' is invalid")]
    InvalidInputSize(usize),

    /// This error occurs when a shader binary is provided in a format not supported by the active
    /// backend.
    ///
    /// The `Vulkan` backend can only accept SPIR-V shaders, while the `D3D12` backend can only
    /// accept DXIL shaders.
    #[error("The shader binary is of unsupported format")]
    UnsupportedShaderFormat,

    #[error("An internal backend error has occurred '{0}'")]
    Platform(#[from] anyhow::Error),
}

//
//
// _________________________________________________________________________________________________
// CommandPool

#[derive(Error, Debug)]
pub enum CommandPoolCreateError {
    #[error("An internal backend error has occurred '{0}'")]
    Platform(#[from] anyhow::Error),
}

//
//
// _________________________________________________________________________________________________
// SwapChain

#[derive(Error, Debug)]
pub enum AcquireImageError {
    ///
    /// This error occurs when a queued resize operation was attempted to be resolved before
    /// acquiring and returning an image handle, but the resize operation could not complete.
    ///
    /// This does not flag when the actual GAPI calls for resizing or recreating the swap chain
    /// fails, rather this failure occurs when the wrapper API requirements for resize operations
    /// are not met and the resize could not be completed.
    ///
    /// A resize operation can only occur if there are no swap textures in use on the GPU and there
    /// are no images acquired by the API consumer. When resizing the GPU queues will be flushed so
    /// it is easy to ensure the first condition by managing your image acquires.
    ///
    /// It is the caller's job to ensure it is possible for the resize operation to complete.
    ///
    #[error("A resize operation that was queued failed to complete")]
    QueuedResizeFailed,

    ///
    /// This error occurs when the swap image has already been acquired and an API consumer attempts
    /// to acquire the image again.
    ///
    /// It is the caller's job to manage image acquisitions to avoid triggering this.
    ///
    #[error("No swap chain images are available to acquire")]
    ImageNotAvailable,

    ///
    /// This error is subtle and requires explanation.
    ///
    /// SurfaceNotAvailable will be returned when it is not possible for the backend to create the
    /// underlying swap chain object for the surface at the present time. This is not a failure, the
    /// surface can return to a valid state.
    ///
    /// This is primarily an issue on Vulkan under Windows. On Windows, when a window is minimized
    /// the vkGetPhysicalDeviceSurfaceCapabilitiesKHR call will return a current_extent of (0, 0).
    /// As per the Vulkan spec if current_extent is specified as anything other than
    /// (U32_MAX, U32_MAX) then you must use exactly current_extent when creating the swap chain.
    /// (0, 0) is an invalid value to pass so a minimized window can't have a swap chain attached
    /// to it.
    ///
    /// If the window is minimized then it is impossible to create a swap chain, making it
    /// impossible to hand out images.
    ///
    #[error("The surface is currently in a state where it can not be used")]
    SurfaceNotAvailable,

    #[error("An internal backend error has occurred '{0}'")]
    Platform(#[from] anyhow::Error),
}

//
//
// _________________________________________________________________________________________________
// Command List

#[derive(Error, Debug)]
pub enum CommandListCreateError {
    #[error("An internal backend error has occurred '{0}'")]
    Platform(#[from] anyhow::Error),
}

#[derive(Error, Debug)]
pub enum CommandListBeginError {
    #[error("An internal backend error has occurred '{0}'")]
    Platform(#[from] anyhow::Error),
}

//
//
// _________________________________________________________________________________________________
// Queue

#[derive(Error, Debug)]
pub enum QueueSubmitError {
    #[error("The queue '{0}' is not available")]
    QueueNotAvailable(QueueType),

    #[error("An internal backend error has occurred '{0}'")]
    Platform(#[from] anyhow::Error),
}

#[derive(Error, Debug)]
pub enum QueuePresentError {
    #[error("The queue '{0}' does not support presentation to the requested swap chain")]
    QueuePresentationNotSupported(QueueType),

    #[error("The queue '{0}' is not available")]
    QueueNotAvailable(QueueType),

    #[error("An internal backend error has occurred '{0}'")]
    Platform(#[from] anyhow::Error),
}
