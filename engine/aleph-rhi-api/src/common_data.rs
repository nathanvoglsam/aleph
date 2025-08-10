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

use crate::*;

#[derive(Clone, Eq, PartialEq, Hash, Default, Debug)]
pub struct Rect {
    /// Origin of the rectangle on the `x` axis
    pub x: u32,

    /// Origin of the rectangle on the `y` axis
    pub y: u32,

    /// Width of the rectangle
    pub w: u32,

    /// Height of the rectangle
    pub h: u32,
}

impl Rect {
    /// Returns the origin of the rectangle as `(x, y)`
    pub const fn origin(&self) -> (u32, u32) {
        (self.x, self.y)
    }

    /// Returns the dimensions of the rectangle as `(w, h)`
    pub const fn dimensions(&self) -> (u32, u32) {
        (self.w, self.h)
    }

    /// Returns the maximum point of the rectangle as `(x, y)` (origin + dimensions)
    pub const fn maximum(&self) -> (u32, u32) {
        (self.x + self.w, self.y + self.h)
    }
}

/// A three-component vector of [i32], conventionally used for specifying offsets.
#[derive(Clone, Eq, PartialEq, Hash, Default, Debug)]
pub struct Offset3D {
    /// Offset along the `x` axis
    pub x: i32,

    /// Offset along the `y` axis
    pub y: i32,

    /// Offset along the `z` axis
    pub z: i32,
}

impl Offset3D {
    /// Construct a new [Offset3D] from the 3 provided coordinates
    pub const fn new(x: i32, y: i32, z: i32) -> Self {
        Self { x, y, z }
    }

    /// Returns an offset equal to the maximum point of a box with origin `self` and the provided
    /// extents.
    ///
    /// Produces a new [Offset3D] where each component is equal to the sum of the corresponding
    /// components in `self` and `extent`.
    pub const fn maximum_with_extent(&self, extent: &Extent3D) -> Self {
        Self {
            x: self.x + (extent.width as i32),
            y: self.y + (extent.height as i32),
            z: self.z + (extent.depth as i32),
        }
    }
}

/// An unsigned version of [Offset3D].
#[derive(Clone, Eq, PartialEq, Hash, Default, Debug)]
pub struct UOffset3D {
    /// Extent along the `x` axis
    pub x: u32,

    /// Extent along the `y` axis
    pub y: u32,

    /// Extent along the `z` axis
    pub z: u32,
}

impl UOffset3D {
    /// Construct a new [UOffset3D] from the 3 provided coordinates
    pub const fn new(x: u32, y: u32, z: u32) -> Self {
        Self { x, y, z }
    }

    /// Returns an offset equal to the maximum point of a box with origin `self` and the provided
    /// extents.
    ///
    /// Produces a new [UOffset3D] where each component is equal to the sum of the corresponding
    /// components in `self` and `extent`.
    pub const fn maximum_with_extent(&self, extent: &Extent3D) -> Self {
        Self {
            x: self.x + extent.width,
            y: self.y + extent.height,
            z: self.z + extent.depth,
        }
    }
}

/// A two-component vector of [u32], canonically used for specifying extents.
#[derive(Copy, Clone, Eq, PartialEq, Hash, Default, Debug)]
pub struct Extent2D {
    /// Extent along the `x` axis
    pub width: u32,

    /// Extent along the `y` axis
    pub height: u32,
}

impl Extent2D {
    /// Construct a new [Extent2D] from the 3 provided coordinates
    pub const fn new(width: u32, height: u32) -> Self {
        Self { width, height }
    }
}

/// A three-component vector of [u32], canonically used for specifying extents.
#[derive(Copy, Clone, Eq, PartialEq, Hash, Default, Debug)]
pub struct Extent3D {
    /// Extent along the `x` axis
    pub width: u32,

    /// Extent along the `y` axis
    pub height: u32,

    /// Extent along the `z` axis
    pub depth: u32,
}

impl Extent3D {
    /// Construct a new [Extent3D] from the 3 provided coordinates
    pub const fn new(width: u32, height: u32, depth: u32) -> Self {
        Self {
            width,
            height,
            depth,
        }
    }
}

/// An `ARGB` color value packed into a single u64. Bit layout: 0xAARRGGBB
#[repr(transparent)]
#[derive(Copy, Clone, PartialOrd, PartialEq, Ord, Eq, Debug, Hash)]
pub struct Color(pub u64);

impl Color {
    pub const RED: Self = Self(0xFFFF0000);
    pub const GREEN: Self = Self(0xFF00FF00);
    pub const BLUE: Self = Self(0xFF0000FF);
    pub const YELLOW: Self = Self(0xFFFFFF00);
    pub const MAGENTA: Self = Self(0xFFFF00FF);
    pub const CYAN: Self = Self(0xFF00FFFF);
    pub const WHITE: Self = Self(0xFFFFFFFF);
    pub const BLACK: Self = Self(0xFF000000);
}

impl From<u64> for Color {
    #[inline(always)]
    fn from(v: u64) -> Self {
        Self(v)
    }
}

impl From<Color> for u64 {
    #[inline(always)]
    fn from(v: Color) -> Self {
        v.0
    }
}

impl From<Color> for (f32, f32, f32, f32) {
    #[inline]
    fn from(val: Color) -> Self {
        #[inline(always)]
        fn convert_channel(c: u64) -> f32 {
            ((c & 0xFF) as f32) / 255.0
        }
        let a = convert_channel(val.0 >> 48);
        let r = convert_channel(val.0 >> 32);
        let g = convert_channel(val.0 >> 16);
        let b = convert_channel(val.0);
        (a, r, g, b)
    }
}

impl From<Color> for [f32; 4] {
    #[inline]
    fn from(value: Color) -> Self {
        type V = (f32, f32, f32, f32);
        let (a, r, g, b) = V::from(value);
        [r, g, b, a]
    }
}

#[repr(C)]
#[derive(Clone, Debug)]
pub struct Viewport {
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
    pub min_depth: f32,
    pub max_depth: f32,
}

/// An enumeration of all possible input types to a color texture clear operation
#[derive(Clone, Debug, PartialEq)]
pub enum ColorClearValue {
    /// A full 4-channel f32 colour
    Float { r: f32, g: f32, b: f32, a: f32 },

    /// A 4-channel color packed into a single u32
    Int(u32),
}

impl ColorClearValue {
    pub fn to_float(&self) -> [f32; 4] {
        match self {
            ColorClearValue::Float { r, g, b, a } => [*r, *g, *b, *a],
            ColorClearValue::Int(v) => {
                let a = ((v >> (8 * 0)) & 0xFF) as f32 / 255.0;
                let b = ((v >> (8 * 1)) & 0xFF) as f32 / 255.0;
                let g = ((v >> (8 * 2)) & 0xFF) as f32 / 255.0;
                let r = ((v >> (8 * 3)) & 0xFF) as f32 / 255.0;
                [r, g, b, a]
            }
        }
    }
}

impl From<u32> for ColorClearValue {
    #[inline(always)]
    fn from(v: u32) -> Self {
        Self::Int(v)
    }
}

impl std::fmt::Display for ColorClearValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ColorClearValue::Float { r, g, b, a } => {
                write!(f, "ColorClearValue::Float({r}, {g}, {b}, {a})")
            }
            ColorClearValue::Int(v) => {
                write!(f, "ColorClearValue::Int({:X})", *v)
            }
        }
    }
}

/// An enumeration of all possible input types to a depth/stencil texture clear operation
#[derive(Copy, Clone, Debug, Default, PartialEq)]
pub struct DepthStencilClearValue {
    pub depth: f32,
    pub stencil: u8,
}

impl DepthStencilClearValue {
    pub const fn new(depth: f32, stencil: u8) -> Self {
        Self { depth, stencil }
    }

    pub const fn depth(depth: f32) -> Self {
        Self::new(depth, 0)
    }
}

#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Debug)]
pub enum IndexType {
    U16,
    U32,
}

impl std::fmt::Display for IndexType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            IndexType::U16 => f.write_str("U16"),
            IndexType::U32 => f.write_str("U32"),
        }
    }
}

/// Enumeration of the available logical operations that can be applied as part of attachment blend
/// operations.
///
/// To describe the behavior of each operation we define the following:
///
/// * ¬ is bitwise invert
/// * ∧ is bitwise and
/// * ∨ is bitwise or
/// * ⊕ is bitwise exclusive or
/// * s is the fragment’s Rs0, Gs0, Bs0 or As0 component value for the fragment output corresponding
///   to the color attachment being updated
/// * d is the color attachment’s R, G, B or A component value
///
#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Debug)]
pub enum LogicOp {
    /// 0
    Clear,

    /// s ∧ d
    And,

    /// s ∧ ¬ d
    AndReverse,

    /// s
    Copy,

    /// ¬ s ∧ d
    AndInverted,

    /// d
    Noop,

    /// s ⊕ d
    Xor,

    /// s ∨ d
    Or,

    /// ¬ (s ∨ d)
    Nor,

    /// ¬ (s ⊕ d)
    Equivalent,

    /// ¬ d
    Invert,

    /// s ∨ ¬ d
    OrReverse,

    /// ¬ s
    CopyInverted,

    /// ¬ s ∨ d
    OrInverted,

    /// ¬ (s ∧ d)
    Nand,

    /// all 1s
    Set,
}

impl std::fmt::Display for LogicOp {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LogicOp::Clear => f.write_str("Clear"),
            LogicOp::And => f.write_str("And"),
            LogicOp::AndReverse => f.write_str("AndReverse"),
            LogicOp::Copy => f.write_str("Copy"),
            LogicOp::AndInverted => f.write_str("AndInverted"),
            LogicOp::Noop => f.write_str("Noop"),
            LogicOp::Xor => f.write_str("Xor"),
            LogicOp::Or => f.write_str("Or"),
            LogicOp::Nor => f.write_str("Nor"),
            LogicOp::Equivalent => f.write_str("Equivalent"),
            LogicOp::Invert => f.write_str("Invert"),
            LogicOp::OrReverse => f.write_str("OrReverse"),
            LogicOp::CopyInverted => f.write_str("CopyInverted"),
            LogicOp::OrInverted => f.write_str("OrInverted"),
            LogicOp::Nand => f.write_str("Nand"),
            LogicOp::Set => f.write_str("Set"),
        }
    }
}

impl Default for LogicOp {
    #[inline(always)]
    fn default() -> Self {
        Self::Clear
    }
}

bitflags::bitflags! {
    /// Bit flags used for identifying and/or masking the color components in operations regarding
    /// texels.
    #[derive(PartialEq, Eq, PartialOrd, Ord, Hash, Debug, Clone, Copy)]
    pub struct ColorComponentFlags: u8 {
        /// Specifies the 'red' channel
        const R = 0b0001;

        /// Specifies the 'green' channel
        const G = 0b0010;

        /// Specifies the 'blue' channel
        const B = 0b0100;

        /// Specifies the 'alpha' channel
        const A = 0b1000;
    }
}

impl Default for ColorComponentFlags {
    #[inline(always)]
    fn default() -> Self {
        ColorComponentFlags::empty()
    }
}

/// Enumeration of available comparison operators. Comparison operators compare a 'reference' and a
/// 'test' value, and return a true (“passed”) or false (“failed”) value depending on the comparison
/// operator chosen.
#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Debug)]
pub enum CompareOp {
    /// Specifies that the comparison always evaluates false
    Never,

    /// Specifies that the comparison always evaluates true
    Always,

    /// Specifies that the comparison evaluates reference = test
    Equal,

    /// Specifies that the comparison evaluates reference ≠ test
    NotEqual,

    /// Specifies that the comparison evaluates reference < test
    Less,

    /// Specifies that the comparison evaluates reference ≤ test
    LessEqual,

    /// Specifies that the comparison evaluates reference > test
    Greater,

    /// Specifies that the comparison evaluates reference ≥ test
    GreaterOrEqual,
}

impl std::fmt::Display for CompareOp {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CompareOp::Never => f.write_str("Never"),
            CompareOp::Always => f.write_str("Always"),
            CompareOp::Equal => f.write_str("Equal"),
            CompareOp::NotEqual => f.write_str("NotEqual"),
            CompareOp::Less => f.write_str("Less"),
            CompareOp::LessEqual => f.write_str("LessEqual"),
            CompareOp::Greater => f.write_str("Greater"),
            CompareOp::GreaterOrEqual => f.write_str("GreaterOrEqual"),
        }
    }
}

impl Default for CompareOp {
    #[inline(always)]
    fn default() -> Self {
        Self::Never
    }
}

bitflags::bitflags! {
    #[derive(PartialEq, Eq, PartialOrd, Ord, Hash, Debug, Clone, Copy)]
    pub struct ResourceUsageFlags: u32 {
        /// Specifies no usage flags
        const NONE = 0x0;

        /// Specifies usage as the source of a copy operation
        const COPY_SOURCE = 0x1;

        /// Specifies usage as the destination of a copy operation
        const COPY_DEST = 0x2;

        /// Specifies usage as a vertex buffer
        const VERTEX_BUFFER = 0x4;

        /// Specifies usage as an index buffer
        const INDEX_BUFFER = 0x8;

        /// Specifies usage as a constant buffer through a constant buffer view
        const CONSTANT_BUFFER = 0x10;

        /// Specifies usage as a read-only shader resource
        const SHADER_RESOURCE = 0x20;

        /// Specifies usage through an unordered access view, implying writable access
        const UNORDERED_ACCESS = 0x40;

        /// Specifies read usage as the source buffer for indirect draw arguments
        const INDIRECT_DRAW_ARGS = 0x80;

        /// Specifies usage as the input for a raytracing acceleration structure build operation
        const ACCELERATION_STRUCTURE_BUILD_INPUT = 0x100;

        /// Specifies usage as a raytracing acceleration structure. This implys both read usage when
        /// tracing rays as well as being the target of an acceleration structure build operation.
        const ACCELERATION_STRUCTURE_STORAGE = 0x200;

        /// Specifies usage as a render target
        const RENDER_TARGET = 0x400;

        /// Specifies the texture can be used as a cubemap face
        const CUBE_FACE = 0x800;

        /// A mask of all the usage flags valid to use on buffers
        const BUFFER_USAGE_MASK =
            Self::COPY_SOURCE.bits()
            | Self::COPY_DEST.bits()
            | Self::VERTEX_BUFFER.bits()
            | Self::INDEX_BUFFER.bits()
            | Self::CONSTANT_BUFFER.bits()
            | Self::INDIRECT_DRAW_ARGS.bits()
            | Self::ACCELERATION_STRUCTURE_BUILD_INPUT.bits()
            | Self::ACCELERATION_STRUCTURE_STORAGE.bits()
            | Self::SHADER_RESOURCE.bits()
            | Self::UNORDERED_ACCESS.bits();

        /// A mask of all the usage flags valid to use on textures
        const TEXTURE_USAGE_MASK =
            Self::COPY_SOURCE.bits()
            | Self::COPY_DEST.bits()
            | Self::SHADER_RESOURCE.bits()
            | Self::UNORDERED_ACCESS.bits()
            | Self::RENDER_TARGET.bits()
            | Self::CUBE_FACE.bits();

        /// Mask that represents all read usage flags.
        const READ_USAGE_MASK =
            Self::COPY_SOURCE.bits()
            | Self::VERTEX_BUFFER.bits()
            | Self::INDEX_BUFFER.bits()
            | Self::CONSTANT_BUFFER.bits()
            | Self::INDIRECT_DRAW_ARGS.bits()
            | Self::ACCELERATION_STRUCTURE_BUILD_INPUT.bits()
            | Self::ACCELERATION_STRUCTURE_STORAGE.bits()
            | Self::SHADER_RESOURCE.bits()
            | Self::RENDER_TARGET.bits();

        /// Mask that represents all write usage flags.
        const WRITE_USAGE_MASK =
            Self::COPY_DEST.bits()
            | Self::ACCELERATION_STRUCTURE_STORAGE.bits()
            | Self::UNORDERED_ACCESS.bits()
            | Self::RENDER_TARGET.bits();
    }
}

impl Default for ResourceUsageFlags {
    #[inline(always)]
    fn default() -> Self {
        Self::NONE
    }
}

impl ResourceUsageFlags {
    pub const fn is_buffer_usage(&self) -> bool {
        ResourceUsageFlags::BUFFER_USAGE_MASK.contains(*self)
    }

    pub const fn is_texture_usage(&self) -> bool {
        ResourceUsageFlags::TEXTURE_USAGE_MASK.contains(*self)
    }

    pub const fn is_read_usage(&self) -> bool {
        ResourceUsageFlags::READ_USAGE_MASK.contains(*self)
    }

    pub const fn is_writable_usage(&self) -> bool {
        ResourceUsageFlags::WRITE_USAGE_MASK.contains(*self)
    }

    /// Gets the image layout that is compatible with the specified access flags
    ///
    /// - `read_only` Declares whether the access is read only.
    /// - `format` Provides a texture format for texture usages that need a format to resolve.
    ///     - `format` is only needed if 'self' contains [`ResourceUsageFlags::RENDER_TARGET`]
    pub const fn image_layout(&self, read_only: bool, format: Format) -> ImageLayout {
        debug_assert!(self.is_valid_texture_usage());

        if self.contains(Self::COPY_SOURCE) {
            return ImageLayout::CopySrc;
        }
        if self.contains(Self::COPY_DEST) {
            return ImageLayout::CopyDst;
        }
        if self.contains(Self::SHADER_RESOURCE) {
            return ImageLayout::ShaderReadOnly;
        }
        if self.contains(Self::UNORDERED_ACCESS) {
            return ImageLayout::UnorderedAccess;
        }
        if self.contains(Self::RENDER_TARGET) {
            return if format.is_depth_stencil() {
                if read_only {
                    ImageLayout::DepthStencilReadOnly
                } else {
                    ImageLayout::DepthStencilAttachment
                }
            } else {
                ImageLayout::ColorAttachment
            };
        }
        ImageLayout::Undefined
    }

    /// Returns the barrier sync stages that are valid for the given set of [`ResourceUsageFlags`].
    ///
    /// - `read_only` Declares whether the access is read only.
    /// - `format` Provides a texture format for texture usages that need a format to resolve.
    ///     - `format` is only needed if 'self' contains [`ResourceUsageFlags::RENDER_TARGET`]
    #[inline]
    pub fn default_barrier_sync(&self, read_only: bool, format: Format) -> BarrierSync {
        let mut sync = BarrierSync::NONE;
        if self.contains(Self::COPY_SOURCE) {
            sync |= BarrierSync::COPY;
        }
        if self.contains(Self::COPY_DEST) {
            sync |= BarrierSync::COPY;
        }
        if self.contains(Self::VERTEX_BUFFER) {
            sync |= BarrierSync::VERTEX_SHADING;
        }
        if self.contains(Self::INDEX_BUFFER) {
            sync |= BarrierSync::INDEX_INPUT;
        }
        if self.contains(Self::CONSTANT_BUFFER) {
            sync |= BarrierSync::PIXEL_SHADING
                | BarrierSync::VERTEX_SHADING
                // | BarrierSync::RAYTRACING // TODO: Validation on D3D12 flags this possibly erroneously
                | BarrierSync::COMPUTE_SHADING;
        }
        if self.contains(Self::SHADER_RESOURCE) {
            sync |= BarrierSync::PIXEL_SHADING
                | BarrierSync::VERTEX_SHADING
                // | BarrierSync::RAYTRACING // TODO: Validation on D3D12 flags this possibly erroneously
                | BarrierSync::COMPUTE_SHADING;
        }
        if self.contains(Self::UNORDERED_ACCESS) {
            sync |= BarrierSync::PIXEL_SHADING
                | BarrierSync::VERTEX_SHADING
                // | BarrierSync::RAYTRACING // TODO: Validation on D3D12 flags this possibly erroneously
                | BarrierSync::COMPUTE_SHADING;
        }
        if self.contains(Self::INDIRECT_DRAW_ARGS) {
            sync |= BarrierSync::EXECUTE_INDIRECT;
        }
        if self.contains(Self::ACCELERATION_STRUCTURE_BUILD_INPUT) {
            sync |= BarrierSync::BUILD_RAYTRACING_ACCELERATION_STRUCTURE;
        }
        if self.contains(Self::ACCELERATION_STRUCTURE_STORAGE) {
            sync |= BarrierSync::RAYTRACING;
            if !read_only {
                sync |= BarrierSync::BUILD_RAYTRACING_ACCELERATION_STRUCTURE;
            }
        }
        if self.contains(Self::RENDER_TARGET) {
            if format.is_depth_stencil() {
                sync |= BarrierSync::DEPTH_STENCIL;
            } else {
                sync |= BarrierSync::RENDER_TARGET;
            }
        }
        sync
    }

    /// Returns the set of [`BarrierAccess`] flags that covers all possible accesses applicable for
    /// the given set of [`ResourceUsageFlags`] assuming read-only access.
    ///
    /// - `format` Provides a texture format for texture usages that need a format to resolve.
    ///     - `format` is only needed if 'self' contains [`ResourceUsageFlags::RENDER_TARGET`]
    #[inline]
    pub fn barrier_access_for_read(&self, format: Format) -> BarrierAccess {
        let mut out = BarrierAccess::NONE;
        if self.contains(Self::COPY_SOURCE) {
            out |= BarrierAccess::COPY_READ
        }
        if self.contains(Self::VERTEX_BUFFER) {
            out |= BarrierAccess::VERTEX_BUFFER_READ
        }
        if self.contains(Self::INDEX_BUFFER) {
            out |= BarrierAccess::INDEX_BUFFER_READ
        }
        if self.contains(Self::CONSTANT_BUFFER) {
            out |= BarrierAccess::CONSTANT_BUFFER_READ
        }
        if self.contains(Self::INDIRECT_DRAW_ARGS) {
            out |= BarrierAccess::INDIRECT_COMMAND_READ
        }
        if self.contains(Self::ACCELERATION_STRUCTURE_BUILD_INPUT) {
            out |= BarrierAccess::RAYTRACING_ACCELERATION_STRUCTURE_READ
        }
        if self.contains(Self::ACCELERATION_STRUCTURE_STORAGE) {
            out |= BarrierAccess::RAYTRACING_ACCELERATION_STRUCTURE_READ
        }
        if self.contains(Self::SHADER_RESOURCE) {
            out |= BarrierAccess::SHADER_READ
        }
        if self.contains(Self::RENDER_TARGET) {
            if format.is_depth_stencil() {
                out |= BarrierAccess::DEPTH_STENCIL_READ
            } else {
                out |= BarrierAccess::RENDER_TARGET_READ
            }
        }
        out
    }

    /// Returns the set of [`BarrierAccess`] flags that covers all possible accesses applicable for
    /// the given set of [`ResourceUsageFlags`] assuming writable access.
    ///
    /// - `format` Provides a texture format for texture usages that need a format to resolve.
    ///     - `format` is only needed if 'self' contains [`ResourceUsageFlags::RENDER_TARGET`]
    #[inline]
    pub fn barrier_access_for_write(&self, format: Format) -> BarrierAccess {
        let mut out = BarrierAccess::NONE;
        if self.contains(Self::COPY_DEST) {
            out |= BarrierAccess::COPY_WRITE
        }
        if self.contains(Self::ACCELERATION_STRUCTURE_STORAGE) {
            out |= BarrierAccess::RAYTRACING_ACCELERATION_STRUCTURE_WRITE
        }
        if self.contains(Self::UNORDERED_ACCESS) {
            out |= BarrierAccess::SHADER_WRITE
        }
        if self.contains(Self::RENDER_TARGET) {
            if format.is_depth_stencil() {
                out |= BarrierAccess::DEPTH_STENCIL_WRITE
            } else {
                out |= BarrierAccess::RENDER_TARGET_WRITE
            }
        }
        out
    }

    /// Returns whether 'self' is a valid texture usage.
    ///
    /// That is, whether it is a subset of [`ResourceUsageFlags::TEXTURE_USAGE_MASK`] and does not
    /// contain [`ResourceUsageFlags::CUBE_FACE`].
    pub const fn is_valid_texture_usage(&self) -> bool {
        if !ResourceUsageFlags::TEXTURE_USAGE_MASK.contains(*self) {
            return false;
        }

        // We need to filter out any non-synchronizing access flags to allow this debug check
        // to make sense. We need to check that only a single usage has been specified as an
        // image within a pass can only be in a single image layout.
        let access_flags = self.bits() & (!Self::CUBE_FACE.bits());
        if access_flags.count_ones() > 1 {
            return false;
        }

        true
    }
}
