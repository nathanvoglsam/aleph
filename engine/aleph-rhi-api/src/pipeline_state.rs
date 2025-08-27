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

use aleph_object_system::ArcObject;

use crate::*;

#[derive(Clone)]
pub struct GraphicsPipelineHandle {
    inner: ArcObject,
}

impl GraphicsPipelineHandle {
    /// # Safety
    ///
    /// It is the caller's responsibility to ensure that the given object refers to an object that
    /// the inner RHI implementation considers a semaphore objec.
    pub const unsafe fn new(inner: ArcObject) -> Self {
        Self { inner }
    }

    ///
    /// Gets the number of strong ([`GraphicsPipelineHandle`]) pointers to this allocation.
    ///
    /// # Safety
    ///
    /// This method by itself is safe, but using it correctly requires extra care.
    /// Another thread can change the strong count at any time,
    /// including potentially between calling this method and acting on the result.
    ///
    /// # Info
    ///
    /// This is just a wrapper around [`std::sync::Arc::strong_count`]
    ///
    #[inline]
    #[must_use]
    pub fn strong_count(&self) -> usize {
        self.inner.strong_count()
    }

    /// Unwrap the [`GraphicsPipelineHandle`] and get the inner [`ArcObject`]
    #[inline]
    pub fn into_inner(self) -> ArcObject {
        self.inner
    }

    /// Get the inner [`ArcObject`]
    pub const fn get(&self) -> &ArcObject {
        &self.inner
    }
}

#[derive(Clone)]
pub struct ComputePipelineHandle {
    inner: ArcObject,
}

impl ComputePipelineHandle {
    /// # Safety
    ///
    /// It is the caller's responsibility to ensure that the given object refers to an object that
    /// the inner RHI implementation considers a semaphore objec.
    pub const unsafe fn new(inner: ArcObject) -> Self {
        Self { inner }
    }

    ///
    /// Gets the number of strong ([`ComputePipelineHandle`]) pointers to this allocation.
    ///
    /// # Safety
    ///
    /// This method by itself is safe, but using it correctly requires extra care.
    /// Another thread can change the strong count at any time,
    /// including potentially between calling this method and acting on the result.
    ///
    /// # Info
    ///
    /// This is just a wrapper around [`std::sync::Arc::strong_count`]
    ///
    #[inline]
    #[must_use]
    pub fn strong_count(&self) -> usize {
        self.inner.strong_count()
    }

    /// Unwrap the [`ComputePipelineHandle`] and get the inner [`ArcObject`]
    #[inline]
    pub fn into_inner(self) -> ArcObject {
        self.inner
    }

    /// Get the inner [`ArcObject`]
    pub const fn get(&self) -> &ArcObject {
        &self.inner
    }
}

#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Debug)]
pub enum VertexInputRate {
    /// Specifies that vertex attribute addressing is a function of the vertex index
    PerVertex,

    /// Specifies that vertex attribute addressing is a function of the instance index
    PerInstance,
}

impl std::fmt::Display for VertexInputRate {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            VertexInputRate::PerVertex => f.write_str("PerVertex"),
            VertexInputRate::PerInstance => f.write_str("PerInstance"),
        }
    }
}

impl Default for VertexInputRate {
    #[inline(always)]
    fn default() -> Self {
        Self::PerVertex
    }
}

#[derive(Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Debug, Default)]
pub struct VertexInputBindingDesc {
    /// The binding number that this structure describes
    pub binding: u32,

    /// The byte stride between consecutive elements within the buffer
    pub stride: u32,

    /// Value specifying the rate at which this input binding is fetched (per-vertex or
    /// per-instance)
    pub input_rate: VertexInputRate,
}

#[derive(Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Debug, Default)]
pub struct VertexInputAttributeDesc {
    /// The shader input location number for this attribute
    pub location: u32,

    /// The binding number which this attribute takes its data from
    pub binding: u32,

    /// The format of the vertex attribute, describing size and layout
    pub format: Format,

    /// Byte offset of this attribute relative to the start of an element in the vertex input
    /// binding
    pub offset: u32,
}

#[derive(Clone, PartialEq, Eq, Hash, Debug, Default)]
pub struct VertexInputStateDesc<'a> {
    /// The list of input bindings. An input binding describes the access pattern of a single buffer
    /// bound at a specific binding slot. Each binding specifies the stride of a binding element
    /// as well as the input rate (per-vertex/per-instance) the elements are used at.
    pub input_bindings: &'a [VertexInputBindingDesc],

    /// The list of input attachments. An input attachment describes an individual vertex attribute.
    /// Conceptually it marks up a single 'field' within the input binding it is read from. Multiple
    /// attributes can be fetched from the same binding.
    pub input_attributes: &'a [VertexInputAttributeDesc],
}

#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Debug)]
pub enum PrimitiveTopology {
    /// Specifies a series of separate point primitives
    PointList,

    /// specifies a series of separate line primitives
    LineList,

    /// Specifies a series of connected line primitives with consecutive lines sharing a vertex
    LineStrip,

    /// Specifies a series of separate triangle primitives
    TriangleList,

    /// Specifies a series of connected triangle primitives with consecutive triangles sharing an
    /// edge
    TriangleStrip,
}

impl std::fmt::Display for PrimitiveTopology {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PrimitiveTopology::PointList => f.write_str("PointList"),
            PrimitiveTopology::LineList => f.write_str("LineList"),
            PrimitiveTopology::LineStrip => f.write_str("LineStrip"),
            PrimitiveTopology::TriangleList => f.write_str("TriangleList"),
            PrimitiveTopology::TriangleStrip => f.write_str("TriangleStrip"),
        }
    }
}

impl Default for PrimitiveTopology {
    #[inline(always)]
    fn default() -> Self {
        Self::PointList
    }
}

#[derive(Clone, PartialEq, Eq, Hash, Debug, Default)]
pub struct InputAssemblyStateDesc {
    pub primitive_topology: PrimitiveTopology,
}

#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Debug)]
pub enum PolygonMode {
    Fill,
    Line,
}

impl std::fmt::Display for PolygonMode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PolygonMode::Fill => f.write_str("Fill"),
            PolygonMode::Line => f.write_str("Line"),
        }
    }
}

impl Default for PolygonMode {
    #[inline(always)]
    fn default() -> Self {
        Self::Fill
    }
}

#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Debug)]
pub enum CullMode {
    /// Specifies that no triangles are discarded
    None,

    /// Specifies that back-facing triangles are discarded
    Back,

    /// Specifies that front-facing triangles are discarded
    Front,
}

impl std::fmt::Display for CullMode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CullMode::None => f.write_str("None"),
            CullMode::Back => f.write_str("Back"),
            CullMode::Front => f.write_str("Front"),
        }
    }
}

impl Default for CullMode {
    #[inline(always)]
    fn default() -> Self {
        Self::None
    }
}

#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Debug)]
pub enum FrontFaceOrder {
    /// Specifies that a front-facing triangle is defined as one with a counter-clockwise winding
    /// order
    CounterClockwise,

    /// Specifies that a front-facing triangle is defined as one with a clockwise winding order
    Clockwise,
}

impl std::fmt::Display for FrontFaceOrder {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            FrontFaceOrder::CounterClockwise => f.write_str("CounterClockwise"),
            FrontFaceOrder::Clockwise => f.write_str("Clockwise"),
        }
    }
}

impl Default for FrontFaceOrder {
    #[inline(always)]
    fn default() -> Self {
        Self::CounterClockwise
    }
}

#[derive(Clone, PartialEq, Debug, Default)]
pub struct RasterizerStateDesc {
    /// Specifies the triangle facing directions used for primitive culling
    pub cull_mode: CullMode,

    /// Specifies what winding order defines a 'front' facing triangle
    pub front_face: FrontFaceOrder,

    /// Specifies the triangle rendering mode
    pub polygon_mode: PolygonMode,

    /// Specifies the value to apply as a depth bias. If 0, depth bias is disabled
    pub depth_bias: i32,

    /// Specifies the depth bias clamp value, if depth bias is enabled
    pub depth_bias_clamp: f32,

    /// Specifies the depth bias slop factor value, if depth bias is enabled
    pub depth_bias_slope_factor: f32,
}

#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Debug)]
pub enum BlendFactor {
    Zero,
    One,
    SrcColor,
    OneMinusSrcColor,
    DstColor,
    OneMinusDstColor,
    SrcAlpha,
    OneMinusSrcAlpha,
    DstAlpha,
    OneMinusDstAlpha,
    SrcAlphaSaturate,
    BlendFactor,
    OneMinusBlendFactor,
}

impl std::fmt::Display for BlendFactor {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            BlendFactor::Zero => f.write_str("Zero"),
            BlendFactor::One => f.write_str("One"),
            BlendFactor::SrcColor => f.write_str("SrcColor"),
            BlendFactor::OneMinusSrcColor => f.write_str("OneMinusSrcColor"),
            BlendFactor::DstColor => f.write_str("DstColor"),
            BlendFactor::OneMinusDstColor => f.write_str("OneMinusDstColor"),
            BlendFactor::SrcAlpha => f.write_str("SrcAlpha"),
            BlendFactor::OneMinusSrcAlpha => f.write_str("OneMinusSrcAlpha"),
            BlendFactor::DstAlpha => f.write_str("DstAlpha"),
            BlendFactor::OneMinusDstAlpha => f.write_str("OneMinusDstAlpha"),
            BlendFactor::SrcAlphaSaturate => f.write_str("SrcAlphaSaturate"),
            BlendFactor::BlendFactor => f.write_str("BlendFactor"),
            BlendFactor::OneMinusBlendFactor => f.write_str("OneMinusBlendFactor"),
        }
    }
}

impl Default for BlendFactor {
    #[inline(always)]
    fn default() -> Self {
        Self::Zero
    }
}

#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Debug)]
pub enum BlendOp {
    Add,
    Subtract,
    ReverseSubtract,
    Min,
    Max,
}

impl std::fmt::Display for BlendOp {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            BlendOp::Add => f.write_str("Add"),
            BlendOp::Subtract => f.write_str("Subtract"),
            BlendOp::ReverseSubtract => f.write_str("ReverseSubtract"),
            BlendOp::Min => f.write_str("Min"),
            BlendOp::Max => f.write_str("Max"),
        }
    }
}

impl Default for BlendOp {
    #[inline(always)]
    fn default() -> Self {
        Self::Add
    }
}

#[derive(Clone, PartialEq, Eq, Hash, Debug, Default)]
pub struct AttachmentBlendState {
    /// Enables color blending for matching attachment. All other fields will be ignored if this
    /// value is `false`.
    pub blend_enabled: bool,

    /// Selects which blend factor is used to determine the source factors (Sr,Sg,Sb)
    pub src_factor: BlendFactor,

    /// Selects which blend factor is used to determine the destination factors (Dr,Dg,Db)
    pub dst_factor: BlendFactor,

    /// Selects which blend operation is used to calculate the RGB values to write to the color
    /// attachment
    pub blend_op: BlendOp,

    /// Selects which blend factor is used to determine the source factor (Sa)
    pub alpha_src_factor: BlendFactor,

    /// Selects which blend factor is used to determine the destination factor (Da)
    pub alpha_dst_factor: BlendFactor,

    /// Selects which blend operation is use to calculate the alpha values to write to the color
    /// attachment
    pub alpha_blend_op: BlendOp,

    /// Is a bitmask of [ColorComponentFlags] specifying which of the R, G, B, and/or A components
    /// are enabled for writing
    pub color_write_mask: ColorComponentFlags,
}

#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Debug)]
pub enum StencilOp {
    /// Keeps the current value
    Keep,

    /// Sets the value to 0
    Zero,

    /// Sets the value to reference
    Replace,

    /// Increments the current value and clamps to the maximum representable unsigned value
    IncrementClamp,

    /// Decrements the current value and clamps to 0
    DecrementClamp,

    /// Bitwise-inverts the current value
    Invert,

    /// Increments the current value and wraps to 0 when the maximum value would have been exceeded
    IncrementWrap,

    /// Decrements the current value and wraps to the maximum possible value when the value would go
    /// below 0
    DecrementWrap,
}

impl std::fmt::Display for StencilOp {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            StencilOp::Keep => f.write_str("Keep"),
            StencilOp::Zero => f.write_str("Zero"),
            StencilOp::Replace => f.write_str("Replace"),
            StencilOp::IncrementClamp => f.write_str("IncrementClamp"),
            StencilOp::DecrementClamp => f.write_str("DecrementClamp"),
            StencilOp::Invert => f.write_str("Invert"),
            StencilOp::IncrementWrap => f.write_str("IncrementWrap"),
            StencilOp::DecrementWrap => f.write_str("DecrementWrap"),
        }
    }
}

impl Default for StencilOp {
    #[inline(always)]
    fn default() -> Self {
        Self::Keep
    }
}

#[derive(Clone, PartialEq, Eq, Hash, Debug, Default)]
pub struct StencilOpState {
    /// Value specifying the action performed on samples that fail the stencil test
    pub fail_op: StencilOp,

    /// Value specifying the action performed on samples that pass both the depth and stencil tests
    pub pass_op: StencilOp,

    /// Value specifying the action performed on samples that pass the stencil test and fail the
    /// depth test
    pub depth_fail_op: StencilOp,

    /// Value specifying the comparison operator used in the stencil test
    pub compare_op: CompareOp,
}

#[derive(Clone, PartialEq, Debug, Default)]
pub struct DepthStencilStateDesc {
    /// Controls whether depth testing is enabled
    pub depth_test: bool,

    /// Controls whether depth writes are enabled when 'depth_test' is true. Depth writes are always
    /// disabled when 'depth_test' is false
    pub depth_write: bool,

    /// Specifies the comparison operator to use in the 'comparison' step of the depth test
    pub depth_compare_op: CompareOp,

    /// Controls whether stencil testing is enabled
    pub stencil_test: bool,

    /// Selects the bits of the unsigned integer stencil values participating in the stencil test
    pub stencil_read_mask: u8,

    /// Selects the bits of the unsigned integer stencil values updated by the stencil test in the
    /// stencil framebuffer attachment
    pub stencil_write_mask: u8,

    /// Control the 'front' parameters of the stencil test
    pub stencil_front: StencilOpState,

    /// Control the 'back' parameters of the stencil test
    pub stencil_back: StencilOpState,

    /// Enables the depth bounds check. Requires a device capability for support.
    pub depth_bounds_enable: bool,

    /// Specifies the min range of the depth bounds, if the depth bound test is enabled. Ignored if
    /// depth bounds is not enabled.
    pub min_depth_bounds: f32,

    /// Specifies the max range of the depth bounds, if the depth bound test is enabled. Ignored if
    /// depth bounds is not enabled.
    pub max_depth_bounds: f32,
}

#[derive(Clone, PartialEq, Eq, Hash, Debug, Default)]
pub struct BlendStateDesc<'a> {
    /// An array of blend state descriptions that will be applied to each matching output attachment
    pub attachments: &'a [AttachmentBlendState],
}

#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub struct ShaderStage<'a> {
    pub data: ShaderBinary<'a>,
    pub stage: ShaderType,
}

#[derive(Clone)]
pub struct GraphicsPipelineDesc<'a> {
    /// The list of shader modules that the pipeline configuration will use. The shader stage for
    /// each module is specified on the [ShaderStage] object.
    pub shader_stages: &'a [ShaderStage<'a>],

    /// The description of binding locations used by both the pipeline and parameter blocks used
    /// with the pipeline
    pub binding_signature: &'a dyn IBindingSignature,

    /// Structure that describes the vertex input piece of the graphics pipeline
    pub vertex_layout: &'a VertexInputStateDesc<'a>,

    /// Structure that describes the input assembly piece of the graphics pipeline
    pub input_assembly_state: &'a InputAssemblyStateDesc,

    /// Structure that describes the rasterizer piece of the graphics pipeline
    pub rasterizer_state: &'a RasterizerStateDesc,

    /// Structure that describes the depth/stencil test piece of the graphics pipeline
    pub depth_stencil_state: &'a DepthStencilStateDesc,

    /// Structure that describes the color blending piece of the graphics pipeline
    pub blend_state: &'a BlendStateDesc<'a>,

    /// Specifies the number of and format of render target attachments
    pub render_target_formats: &'a [Format],

    /// Specifies the format of the depth stencil attachment, if any.
    pub depth_stencil_format: Option<Format>,

    /// The name of the object
    pub name: Option<&'a str>,
}

#[derive(Clone)]
pub struct ComputePipelineDesc<'a> {
    /// The compute shader module that will be used by the compute pipeline being created.
    pub shader_module: ShaderBinary<'a>,

    /// The description of binding locations used by both the pipeline and parameter blocks used
    /// with the pipeline
    pub binding_signature: &'a dyn IBindingSignature,

    /// The name of the object
    pub name: Option<&'a str>,
}
