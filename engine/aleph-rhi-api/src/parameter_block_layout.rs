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

use std::num::NonZeroU64;

use aleph_any::IAny;

use crate::*;

/// Object that represents a compiled parameter block layout.
///
/// A [`IParameterBlockLayout`] represents pre-computed data for a single parameter block layout.
///
/// # What is a parameter block?
///
/// A parameter block layout is similar to a descriptor set layout in Vulkan. A parameter block
/// describes the layout of a parameter block. Parameter block layouts are combined into a binding
/// signature object to fully describe the binding signature of a shader program.
///
/// We have chosen to adopt the 'slang' shader language's parameter block conventions to be _the_
/// way we handle binding signatures in aleph-rhi. The 'slang' language provides a binding layout
/// abstraction called Parameter Blocks. Slang's parameter blocks provide automatic register/binding
/// assignment with a precise, deterministic algorithm. By adopting this algorithm in aleph-rhi we
/// can make shader authoring much simpler while being backend agnostic.
///
/// The layout rules are described in slang's documentation, and are reasonably simple. Each
/// parameter block maps to a descriptor set/register space on Vulkan/D3D12. The binding/register
/// index is selected sequentially based on the order each parameter appears in the block. And most
/// importantly, binding/register indices are selected _before_ dead code elimination so unused
/// bindings will still be asigned a binding (unlike DXC).
///
/// # How to create
///
/// There are two ways to create a [`IParameterBlockLayout`].
///
/// ## 1. Manually
///
/// You can manually construct a parameter block layout by constructing the layout from a
/// [`ParameterBlockDesc`]. It is up to you to ensure the layout is correct, and is used with the
/// correct shader.
///
/// ## 2. Automatically
///
/// Parameter block layouts can also be created from shader objects. Shader reflection is mandatory
/// in aleph-rhi, and is easily acquired from the slang compiler.
///
/// [`IShader`] can use the shader reflection data to automatically create [`IParameterBlockLayout`]
/// objects that match the shader bytecode.
///
/// # Which to use?
///
/// It is almost always best to use the automatic, reflection-generated parameter block layout.
/// There are only a few, specific circumstances to use manual layouts typically when a collection
/// of shaders must use compatible binding sets (i.e. Gbuffer lay down), or if opting in to features
/// like 'dynamic constant buffers' which can't be deduced from the shader.
pub trait IParameterBlockLayout: IAny + Send + Sync {
    any_arc_trait_utils_decl!(IParameterBlockLayout);

    /// Returns a [`ParameterBlockDesc`] that describes the [`IParameterBlockLayout`], and could be
    /// used to create another compatible layout object.  
    fn desc(&self) -> &ParameterBlockDesc<'_>;

    /// Returns a globally unique ID that is guaranteed to not be shared by any other object
    /// allocated from the same [`IDevice`] instance.
    fn get_id(&self) -> NonZeroU64;

    /// Returns if 'self' is compatible with the given 'other' [`IParameterBlockLayout`].
    ///
    /// If two parameter block layouts are compatible then parameter blocks created from either
    /// layout can be used interchangably.
    fn is_compatible(&self, other: &dyn IParameterBlockLayout) -> bool;
}

/// A full description of a parameter block.
///
/// A parameter block is simply an ordered list of parameters described by a [`ParameterDesc`]
/// struct. Parameters are addressed by index in the parameter block, and the index of each
/// parameter is based on their index in [`ParameterBlockDesc::params`].
///
/// # Push Descriptors
///
/// Setting the [`ParameterBlockFlags::PUSH_DESCRIPTOR`] flag will declare that the resulting
/// parameter block layout will be bound using the push descriptor binding method, and not using
/// a normal parameter block.
///
/// A normal parameter block requires creating a pool or arena, allocating a block, writing
/// descriptors into that block, and finally binding the set at a block index in a command buffer.
/// Two aspects of these can be expensive, allocating the block and binding the block, which may
/// cause problems for high-frequency rebinding. For example, per-object blocks in a raster pass
/// will lead to creation of excessive parameter block objects and frequent, expensive binds.
/// For small parameter blocks that will be read from very few commands it may be more efficient
/// to use push descriptors.
///
/// 'Push descriptors' is based on the namesake extension from `VK_KHR_push_descriptor`. The idea
/// is that for small, frequently changing parameter blocks it may be more efficient to not use
/// a pool/arena + allocated parameter block but to instead directly write the descriptors into a
/// small piece of memory controlled by the command buffer. Instead of 'binding' a block to a slot
/// like normal, you instead perform a direct 'update_parameter_block'-like operation directly into
/// the command buffer. The descriptors written into that internal memory are what become bound in
/// the block.
///
/// The benefit is you get a nice, driver managed bump-allocation like mechanism for these
/// parameters with minimal application side book keeping. This is transparent to the authored
/// shaders and requires no code changes.
///
/// All major APIs have a mechanism to do this efficiently:
/// - Vulkan does this via the 'VK_KHR_push_descriptor' extension directly.
/// - D3D12 can implement this using root descriptors in the root signature instead of descriptor
///   tables. The command encoder simply needs to write root descriptors into the root signature.
/// - Metal can implement this by using 'setBuffer' overloads that take a host-memory slice that
///   is copied into internal memory. The RHI command list will need to maintain a shadow buffer
///   that will be written to and flushed via the 'setBuffer' call.
///
/// ## Restrictions
///
/// - You can not use a [`ParameterBlockDesc`] created with the
///   [`ParameterBlockFlags::PUSH_DESCRIPTOR`] flag to create pools or arenas, nor can you create a
///   [`ParameterBlockHandle`] object from them.
/// - The parameter block must be populated at command recording time using
///   [`IComputeEncoder::push_parameters`].
#[derive(Clone, PartialEq, Hash, Debug)]
pub struct ParameterBlockDesc<'a> {
    /// Ordered list of parameters that make up the parameter block.
    pub params: &'a [ParameterDesc],

    /// Specifies which shader stages this parameter block will be visible to
    pub visibility: DescriptorShaderVisibility,

    /// Set of flags for enabling/disabling features on a whole parameter block.
    pub flags: ParameterBlockFlags,

    /// Debug name to apply to the parameter block, visible in debug tools.
    pub name: Option<&'a str>,
}

impl<'a> ParameterBlockDesc<'a> {
    /// Returns 'true' if 'self' is compatible with 'other'.
    ///
    /// Parameter blocks can be bound to any compatible parameter block layout. The compatibility
    /// rules are _not_ associative. That is, `self.is_compaitlbe(other) == true`` does _not_ imply
    /// `other.is_compatible(self) == true`.
    ///
    /// # Compatibility Rules
    ///
    /// A parameter block is considered compatible with another if:
    /// - It has at least the same number of parameters as the other. It may have more, but never
    ///   less.
    /// - All parameter indices that are common to both parameter blocks must be themselves
    ///   compatible following the rules of [`ParameterDesc::is_compatible`].
    pub fn is_compatible(&self, other: &ParameterBlockDesc) -> bool {
        // If 'self' is smaller than 'other' it can't be used in its place because 'other' is
        // expecting more params than 'self' provides. This could lead to reading uninitialized
        // descriptors (or worse, descriptors from other sets!).
        if self.params.len() < other.params.len() {
            return false;
        }

        // Only the parameter slot indices that are common to both 'self' and 'other' are relevant.
        // Any extra parameters in 'self' are ignored because shaders using the 'other' layout will
        // just ignore the parameters they don't know about.
        //
        // TODO: unbounded arrays will change this.
        let shortest_len = usize::min(self.params.len(), other.params.len());
        let self_params = &self.params[0..shortest_len];
        let other_params = &other.params[0..shortest_len];

        // The two layouts are considered compatible if all relevant bindings are compatible.
        self_params
            .iter()
            .zip(other_params.iter())
            .all(|(a, b)| a.is_compatible(b))
    }
}

/// Enumeration of all types of resource descriptors that can be used in a parameter block.
///
/// All descriptor types as they would be authored in a shader are exposed here.
///
/// # Constant Buffer
///
/// A constant buffer is a special kind of buffer intended for providing uniform constant data to
/// shader programs. They have more restrictive semantics than a structured buffer, but in return
/// they can be more efficient on some hardware.
///
/// The main limitations are:
/// - Size.
///     - Generally they have smaller size limits.
/// - Can't be changed once bound.
///     - Once you bind a constant buffer via a parameter block it must not be changed. Some
///       hardware will take advantage of this to hoist values out of the constant buffer into
///       faster memory or registers before the shader actually executes.
///
/// Some hardware (AMD, mostly) don't treat constant buffers specially, but older Nvidia cards very
/// much do.
///
/// # Structured Buffer
///
/// A structured buffer (including its RW variant) provides a typed, structured view over a buffer's
/// memory. Given a `StructuredBuffer<T>`, the parameter will expose an array of `T` structs.
///
/// # Byte Address Buffer
///
/// Quite possibly one of the worst names for anything ever. A byte address buffer (and its RW
/// variant) provides a raw, untyped view over the bytes in a buffer resource.
///
/// However, despite Microsoft's fantastic name, you can't actually address individual bytes in a
/// byte address buffer. No, instead a byte address buffer provides access to the buffer as raw
/// 4-byte words. There is also code for loading arbitrary structs at different offsets in the
/// buffer however. This does still work well for treating buffers as a bag of bytes, even if the
/// read/write granularity is actualy uint instead of char.
///
/// We chose to adopt the name for consistency.
///
/// # Typed Buffer / Texel Buffer
///
/// These are a somewhat special type of descriptor. They're like a buffer when used in a shader
/// but they borrow features from texture descriptors. They can perform implicit format casts
/// on read/write to the resource, unlike a plain buffer. i.e. a `Buffer<float>` could be backed
/// by bytes and a UNORM conversion will take place on load/store.
///
/// On many devices these are actually backed by texture descriptors. They're quite similar to 1D
/// textures, and on some hardware they actually are 1D texture views. But most importantly the
/// backing resource is a _buffer_ and not a _texture_, allowing much more flexible addressing and
/// re-interpreting of the data with none of the downsides of a texture resource like the opaque
/// memory layout.
///
/// # Texture / RW-Texture
///
/// These are your plain-and-simple texture descriptors. They're either 1D, 2D or 3D. They come as
/// read-only SRVs, or read-write UAVs. They need a 'sampler' handle to be sampled from.
///
/// # Texture Arrays
///
/// These are an extension of Texture/RWTexture descriptors. They are _not_ an array of texture
/// descriptors, rather they are a handle to an array texture. These are only a single descriptor.
///
/// # MS Textures
///
/// These are a unique type of texture, and only come in 2D form. MS is short for 'multi-sample'.
/// MS texture handles provide special access to the samples in an MSAA texture.
///
/// # Sampler State
///
/// The simplest parameter type here. Samplers are used in conjunction with textures to enable
/// filtered sampling of textures. SamplerState parameters encapsulate the filter configuration into
/// a addressable resource.
#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Debug)]
pub enum ParameterType {
    /// A constant buffer/uniform buffer parameter.
    ConstantBuffer,

    /// A structured buffer SRV.
    StructuredBuffer,

    /// RW-capable structured buffer UAV.
    RWStructuredBuffer,

    /// An SRV buffer of raw 4-byte words.
    ByteAddressBuffer,

    /// An RW-capable UAV buffer of raw 4-byte words.
    RWByteAddressBuffer,

    /// A typed buffer/texel buffer SRV.
    Buffer,

    /// A RW-capable typed buffer/texel buffer UAV.
    RWBuffer,

    /// 1D texture SRV.
    Texture1D,

    /// 1D, writable texture UAV.
    RWTexture1D,

    /// 2D texture SRV.
    Texture2D,

    /// 2D, writable texture UAV.
    RWTexture2D,

    /// 3D texture SRV.
    Texture3D,

    /// 3D, writable texture UAV.
    RWTexture3D,

    /// 1D texture array SRV.
    Texture1DArray,

    /// 1D, writable texture array UAV.
    RWTexture1DArray,

    /// 2D texture array SRV.
    Texture2DArray,

    /// 2D, writable texture array UAV.
    RWTexture2DArray,

    /// 3D texture array SRV.
    Texture3DArray,

    /// 3D, writable texture array UAV.
    RWTexture3DArray,

    /// 2D, multisampled texture SRV.
    Texture2DMS,

    /// 2D, writable, multisampled texture UAV.
    RWTexture2DMS,

    /// 2D, multisampled texture array SRV.
    Texture2DMSArray,

    /// 2D, writable, multisampled texture array UAV.
    RWTexture2DMSArray,

    /// A texture cube SRV.
    TextureCube,

    /// A texture cube array SRV.
    TextureCubeArray,

    /// A sampler state handle.
    SamplerState,

    /// A ray tracing acceleration structure.
    AccelerationStructure,
}

impl Default for ParameterType {
    #[inline(always)]
    fn default() -> Self {
        Self::ConstantBuffer
    }
}

impl ParameterType {
    /// Returns 'true' if self is of [`ParameterType::SamplerState`] parameter type.
    pub const fn is_sampler(&self) -> bool {
        matches!(self, Self::SamplerState)
    }

    /// Returns 'true' if self is one of the 'texture' parameter types. This does _not_ include
    /// [`ParameterType::Buffer`] or [`ParameterType::RWBuffer`].
    pub const fn is_texture(&self) -> bool {
        match self {
            ParameterType::Texture1D
            | ParameterType::RWTexture1D
            | ParameterType::Texture2D
            | ParameterType::RWTexture2D
            | ParameterType::Texture3D
            | ParameterType::RWTexture3D
            | ParameterType::Texture1DArray
            | ParameterType::RWTexture1DArray
            | ParameterType::Texture2DArray
            | ParameterType::RWTexture2DArray
            | ParameterType::Texture3DArray
            | ParameterType::RWTexture3DArray
            | ParameterType::Texture2DMS
            | ParameterType::RWTexture2DMS
            | ParameterType::Texture2DMSArray
            | ParameterType::RWTexture2DMSArray
            | ParameterType::TextureCube
            | ParameterType::TextureCubeArray => true,
            _ => false,
        }
    }

    /// Returns 'true' if self is one of the 'buffer' parameter types. This does _not_ include
    /// [`ParameterType::Buffer`] or [`ParameterType::RWBuffer`].
    pub const fn is_buffer(&self) -> bool {
        match self {
            ParameterType::ConstantBuffer
            | ParameterType::StructuredBuffer
            | ParameterType::RWStructuredBuffer
            | ParameterType::ByteAddressBuffer
            | ParameterType::RWByteAddressBuffer => true,
            _ => false,
        }
    }

    /// Returns 'true' if self is one of the texture buffer/texel buffer parameter types. This
    /// includes [`ParameterType::Buffer`] or [`ParameterType::RWBuffer`].
    pub const fn is_texture_buffer(&self) -> bool {
        match self {
            ParameterType::Buffer => true,
            ParameterType::RWBuffer => true,
            _ => false,
        }
    }

    /// Returns 'true' if self is a constant buffer.
    pub const fn is_constant_buffer(&self) -> bool {
        matches!(self, ParameterType::ConstantBuffer)
    }

    /// Returns 'true' if self is a shader-resource-view parameter.
    pub const fn is_srv(&self) -> bool {
        match self {
            ParameterType::StructuredBuffer
            | ParameterType::ByteAddressBuffer
            | ParameterType::Buffer
            | ParameterType::Texture1D
            | ParameterType::Texture2D
            | ParameterType::Texture3D
            | ParameterType::Texture1DArray
            | ParameterType::Texture2DArray
            | ParameterType::Texture3DArray
            | ParameterType::Texture2DMS
            | ParameterType::Texture2DMSArray
            | ParameterType::TextureCube
            | ParameterType::TextureCubeArray => true,
            _ => false,
        }
    }

    /// Returns 'true' if self is a unordered-access-view parameter.
    pub const fn is_uav(&self) -> bool {
        match self {
            ParameterType::RWStructuredBuffer
            | ParameterType::RWByteAddressBuffer
            | ParameterType::RWBuffer
            | ParameterType::RWTexture1D
            | ParameterType::RWTexture2D
            | ParameterType::RWTexture3D
            | ParameterType::RWTexture1DArray
            | ParameterType::RWTexture2DArray
            | ParameterType::RWTexture3DArray
            | ParameterType::RWTexture2DMS
            | ParameterType::RWTexture2DMSArray => true,
            _ => false,
        }
    }

    /// Returns 'true' if self is a texture array parameter.
    pub const fn is_texture_array(&self) -> bool {
        match self {
            ParameterType::Texture1DArray
            | ParameterType::RWTexture1DArray
            | ParameterType::Texture2DArray
            | ParameterType::RWTexture2DArray
            | ParameterType::Texture3DArray
            | ParameterType::RWTexture3DArray
            | ParameterType::Texture2DMSArray
            | ParameterType::RWTexture2DMSArray
            | ParameterType::TextureCubeArray => true,
            _ => false,
        }
    }

    /// Returns 'true' if self is a multi-sample texture parameter.
    pub const fn is_ms_texture(&self) -> bool {
        match self {
            ParameterType::Texture2DMS
            | ParameterType::RWTexture2DMS
            | ParameterType::Texture2DMSArray
            | ParameterType::RWTexture2DMSArray => true,
            _ => false,
        }
    }

    /// Returns a [`TextureDimension`] that matches the dimension of 'self'. Will return [`None`] if
    /// self is not a texture parameter.
    ///
    /// # Note
    ///
    /// For [`ParameterType::TextureCube`] and [`ParameterType::TextureCubeArray`] this will return
    /// [`TextureDimension::Texture2D`]. This doesn't seem like the obvious choice, but is the most
    /// logical. A texture cube is simply a special kind of view over a 2D texture array.
    pub const fn texture_dimension(&self) -> Option<TextureDimension> {
        let out = match self {
            ParameterType::Texture1D => TextureDimension::Texture1D,
            ParameterType::RWTexture1D => TextureDimension::Texture1D,
            ParameterType::Texture2D => TextureDimension::Texture2D,
            ParameterType::RWTexture2D => TextureDimension::Texture2D,
            ParameterType::Texture3D => TextureDimension::Texture3D,
            ParameterType::RWTexture3D => TextureDimension::Texture3D,
            ParameterType::Texture1DArray => TextureDimension::Texture1D,
            ParameterType::RWTexture1DArray => TextureDimension::Texture1D,
            ParameterType::Texture2DArray => TextureDimension::Texture2D,
            ParameterType::RWTexture2DArray => TextureDimension::Texture2D,
            ParameterType::Texture3DArray => TextureDimension::Texture3D,
            ParameterType::RWTexture3DArray => TextureDimension::Texture3D,
            ParameterType::Texture2DMS => TextureDimension::Texture2D,
            ParameterType::RWTexture2DMS => TextureDimension::Texture2D,
            ParameterType::Texture2DMSArray => TextureDimension::Texture2D,
            ParameterType::RWTexture2DMSArray => TextureDimension::Texture2D,
            ParameterType::TextureCube => TextureDimension::Texture2D,
            ParameterType::TextureCubeArray => TextureDimension::Texture2D,
            _ => return None,
        };
        Some(out)
    }

    /// Shorthand for constructing a [`ParameterDesc`] with 'ty = self'.
    pub const fn param(self) -> ParameterDesc {
        ParameterDesc {
            ty: self,
            array_size: ParameterArraySize::NONE,
            structured_buffer_stride: 0,
        }
    }

    /// Shorthand for constructing a [`ParameterDesc`] with 'ty = self' with the given array size.
    pub const fn param_array(self, count: u64) -> ParameterDesc {
        ParameterDesc {
            ty: self,
            array_size: ParameterArraySize::new(count),
            structured_buffer_stride: 0,
        }
    }
}

impl std::fmt::Display for ParameterType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let name = match self {
            Self::ConstantBuffer => "ConstantBuffer",
            Self::StructuredBuffer => "StructuredBuffer",
            Self::RWStructuredBuffer => "RWStructuredBuffer",
            Self::ByteAddressBuffer => "ByteAddressBuffer",
            Self::RWByteAddressBuffer => "RWByteAddressBuffer",
            Self::Buffer => "Buffer",
            Self::RWBuffer => "RWBuffer",
            Self::Texture1D => "Texture1D",
            Self::RWTexture1D => "RWTexture1D",
            Self::Texture2D => "Texture2D",
            Self::RWTexture2D => "RWTexture2D",
            Self::Texture3D => "Texture3D",
            Self::RWTexture3D => "RWTexture3D",
            Self::Texture1DArray => "Texture1DArray",
            Self::RWTexture1DArray => "RWTexture1DArray",
            Self::Texture2DArray => "Texture2DArray",
            Self::RWTexture2DArray => "RWTexture2DArray",
            Self::Texture3DArray => "Texture3DArray",
            Self::RWTexture3DArray => "RWTexture3DArray",
            Self::Texture2DMS => "Texture2DMS",
            Self::RWTexture2DMS => "RWTexture2DMS",
            Self::Texture2DMSArray => "Texture2DMSArray",
            Self::RWTexture2DMSArray => "RWTexture2DMSArray",
            Self::TextureCube => "TextureCube",
            Self::TextureCubeArray => "TextureCubeArray",
            Self::SamplerState => "SamplerState",
            Self::AccelerationStructure => "AccelerationStructure",
        };
        f.write_str(name)
    }
}

/// Description of an individual parameter inside a [`ParameterBlockDesc`].
///
/// A parameter can either be explicitly a single parameter (`array_size = 0`), or it can be an
/// array of parameters (`array_size != 0`). `array_size = 1` is superficially identical to the
/// non-array case, but is distinct as a descriptor array isn't always exactly equal to a single
/// descriptor on some backends.
#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Debug, Default)]
pub struct ParameterDesc {
    /// The type of the parameter
    pub ty: ParameterType,

    /// If this is a descriptor array, this is the size of the descriptor array. The
    /// [`ParameterArraySize`] type encodes two niche values for the different type of parameters.
    pub array_size: ParameterArraySize,

    /// If this is a structured buffer, this is the stride of the structured buffer element. This
    /// _must_ match the stride of the `StructuredBuffer<T>` in the shader.
    ///
    /// It is expected this information will come from the generated reflection data.
    pub structured_buffer_stride: u32,
}

impl ParameterDesc {
    /// Constructs a new [`ParameterDesc`] with the given type and array size.
    pub const fn new(
        ty: ParameterType,
        array_size: ParameterArraySize,
        structured_buffer_stride: u32,
    ) -> Self {
        Self {
            ty,
            array_size,
            structured_buffer_stride,
        }
    }

    /// Takes 'self' and returns a new version set to have the given 'structured_buffer_stride'.
    pub const fn with_stride(mut self, structured_buffer_stride: u32) -> Self {
        self.structured_buffer_stride = structured_buffer_stride;
        self
    }

    /// Returns 'true' if self describes a descriptor array (of any size, including 1).
    pub const fn is_array(&self) -> bool {
        self.array_size.is_array()
    }

    /// Returns 'true' if self is compatible with 'other'.
    ///
    /// # Compatibility
    ///
    /// Compatibility is a straight-forward concept, but has some specifics that mean it doesn't
    /// just boil down to a direct equality comparison.
    ///
    /// Two parameters are compatible if they can occupy the same spot in a parameter block and be
    /// used interchangably on the same shader bytecode. Some options like
    /// [`ParameterFlags::DYNAMIC_CONSTANT_BUFFER`] change how the parameter is bound but doesn't
    /// change interface compatibility with the shader. This function checks for interface
    /// compatibility.
    ///
    /// The rules are:
    /// - For [`ParameterType::ConstantBuffer`]
    ///     - `self.array_size` must exactly equal `other.array_size`
    ///     - `flags` is ignored as the only relevant flag doesn't change compatibility
    /// - For all other parameter types
    ///     - `self.array_size` must exactly equal `other.array_size`
    ///     - `flags` is ignored as no flags apply to these parameter types.
    pub const fn is_compatible(&self, other: &ParameterDesc) -> bool {
        match self.ty {
            // Constant buffers ignore the flag option because the only applicable flag
            // doesn't change interface compatibility
            ParameterType::ConstantBuffer => {
                self.ty as u32 == other.ty as u32 && self.array_size.is_equal(&other.array_size)
            }
            // The other types also ignore the flag option because there are no flags that affect
            // them. You could collapse this code but it has been split out to document that the
            // reason 'flags' is ignore differs.
            _ => self.ty as u32 == other.ty as u32 && self.array_size.is_equal(&other.array_size),
        }
    }
}

/// When associated with a [`ParameterDesc`] this type encodes whether that parameter is an array
/// and the size of array.
///
/// This is a wrapper over a [`Option<NonZeroU64>`].
///
/// - `None` -> not-an-array.
/// - `Some(v)` -> an-array of length 'v'.
/// - `Some(u64::MAX)` -> an unbounded array.
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug, Default)]
pub struct ParameterArraySize {
    pub array_size: Option<NonZeroU64>,
}

impl ParameterArraySize {
    /// Special value that declares a parameter array as unbounded
    pub const UNBOUNDED: ParameterArraySize = Self::new(u64::MAX);

    /// Special value for when a parameter is _not_ an array
    pub const NONE: ParameterArraySize = Self::new(0);

    /// Constructs a new [`ParameterArraySize`] from a raw `u64`. See type level docs for how the
    /// raw value maps to different parameter types.
    pub const fn new(size: u64) -> Self {
        Self {
            array_size: NonZeroU64::new(size),
        }
    }

    /// Returns the number of parameters the associated [`ParameterDesc`] encodes.
    ///
    /// Not-an-array parameters still consume one parameter, this will coerce 'None' -> 1.
    ///
    /// # Warning
    ///
    /// This doesn't handle the 'unbounded' case, this may return 'u64::MAX'. It's recommended to
    /// handle unbounded arrays separately.
    pub const fn count(&self) -> u64 {
        match self.array_size {
            Some(v) => v.get(),
            None => 1,
        }
    }

    /// Returns 'true' if 'self' encode an array
    pub const fn is_array(&self) -> bool {
        matches!(self.array_size, Some(_))
    }

    /// Returns 'true' if 'self' encodes an unbounded array
    pub const fn is_unbounded(&self) -> bool {
        match self.array_size {
            Some(v) => v.get() == u64::MAX,
            None => false,
        }
    }

    /// Returns 'true' if 'self' is exactly equal to 'other' (const).
    pub const fn is_equal(&self, other: &Self) -> bool {
        match (self.array_size, other.array_size) {
            (Some(a), Some(b)) => a.get() == b.get(),
            (None, None) => true,
            _ => false,
        }
    }
}

impl From<u64> for ParameterArraySize {
    #[inline]
    fn from(value: u64) -> Self {
        Self::new(value)
    }
}

bitflags::bitflags! {
    /// Flags for enabling/disabling options for a parameter block layout.
    #[derive(PartialEq, Eq, PartialOrd, Ord, Hash, Debug, Clone, Copy)]
    pub struct ParameterBlockFlags: u8 {
        /// Flags this a parameter block as a 'push descriptor'. This alters how a parameter block
        /// is allocated and bound to a pipeline significantly.
        ///
        /// See [`ParameterBlockDesc`] for more info.
        const PUSH_DESCRIPTOR = 0b1;
    }
}

impl Default for ParameterBlockFlags {
    #[inline(always)]
    fn default() -> Self {
        ParameterBlockFlags::empty()
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

impl DescriptorShaderVisibility {
    /// Returns if the visibility scope of 'self' is smaller than 'other'.
    pub const fn is_compatible(&self, other: &Self) -> bool {
        match (self, other) {
            // Both 'all' then obviously compatible
            (Self::All, Self::All) => true,

            // If self is 'all', but other is not (case caught above), not compatible
            (Self::All, _) => false,

            // If self is not 'all', but other is 'all', then we are compatible
            (_, Self::All) => true,

            // If self is not 'all', and other is not 'all', we are only compatible if both equal.
            (a, b) => *a as usize == *b as usize, // contorted to be const
        }
    }

    /// Returns a minimum visibility that is safe to cover both 'a' and 'b'.
    pub const fn merge(a: Self, b: Self) -> Self {
        match (a, b) {
            // If either 'a' or 'b' are 'all' then we must select 'all'
            (Self::All, Self::All) | (_, Self::All) | (Self::All, _) => Self::All,

            // otherwise...
            (a, b) => {
                // if a == b and != 'all' we can return a/b as the minimal scope. If they aren't
                // equal then we must broaden to 'all'.
                if a as usize == b as usize {
                    a
                } else {
                    Self::All
                }
            }
        }
    }
}

impl std::fmt::Display for DescriptorShaderVisibility {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DescriptorShaderVisibility::All => f.write_str("All"),
            DescriptorShaderVisibility::Compute => f.write_str("Compute"),
            DescriptorShaderVisibility::Vertex => f.write_str("Vertex"),
            DescriptorShaderVisibility::Hull => f.write_str("Hull"),
            DescriptorShaderVisibility::Domain => f.write_str("Domain"),
            DescriptorShaderVisibility::Geometry => f.write_str("Geometry"),
            DescriptorShaderVisibility::Fragment => f.write_str("Fragment"),
            DescriptorShaderVisibility::Amplification => f.write_str("Amplification"),
            DescriptorShaderVisibility::Mesh => f.write_str("Mesh"),
        }
    }
}

impl Default for DescriptorShaderVisibility {
    #[inline(always)]
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
