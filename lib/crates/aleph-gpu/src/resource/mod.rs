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

mod image;

pub use image::ImageFormat;

pub struct Resource<T: ResourceType> {
    state: T,
}

impl Resource<Any> {
    #[inline]
    pub fn to_buffer(self) -> Result<Resource<Buffer>, Self> {
        match self.state.0 {
            ResourceTypes::Buffer(v) => Ok(v),
            ResourceTypes::Image(_) => Err(self),
        }
    }

    #[inline]
    pub fn to_image(self) -> Result<Resource<Image>, Self> {
        match self.state.0 {
            ResourceTypes::Buffer(_) => Err(self),
            ResourceTypes::Image(v) => Ok(v),
        }
    }

    #[inline]
    pub fn to_type(self) -> ResourceTypes {
        self.state.0
    }

    #[inline]
    pub fn is_buffer(&self) -> bool {
        matches!(self.state.0, ResourceTypes::Buffer(_))
    }

    #[inline]
    pub fn is_image(&self) -> bool {
        matches!(self.state.0, ResourceTypes::Image(_))
    }
}

#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Debug, Hash)]
#[repr(u32)]
pub enum ResourceAccessType {
    /*--  BEGIN READ ACCESSES  --*/
    /// Read as an indirect buffer for drawing or dispatch
    IndirectBuffer = FIRST_READ_DISCRIMINATOR,

    /// Read as an index buffer for drawing
    IndexBuffer,

    /// Read as a vertex buffer for drawing
    VertexBuffer,

    /// Read as a uniform buffer in a vertex shader
    VertexShaderReadUniformBuffer,

    /// Read as a sampled image/uniform texel buffer in a vertex shader
    VertexShaderReadSampledImageOrUniformTexelBuffer,

    /// Read as any other resource in a vertex shader
    VertexShaderReadOther,

    /// Read as a uniform buffer in a tessellation control shader
    TessellationControlShaderReadUniformBuffer,

    /// Read as a sampled image/uniform texel buffer  in a tessellation control shader
    TessellationControlShaderReadSampledImageOrUniformTexelBuffer,

    /// Read as any other resource in a tessellation control shader
    TessellationControlShaderReadOther,

    /// Read as a uniform buffer in a tessellation evaluation shader
    TessellationEvaluationShaderReadUniformBuffer,

    /// Read as a sampled image/uniform texel buffer in a tessellation evaluation shader
    TessellationEvaluationShaderReadSampledImageOrUniformTexelBuffer,

    /// Read as any other resource in a tessellation evaluation shader
    TessellationEvaluationShaderReadOther,

    /// Read as a uniform buffer in a geometry shader
    GeometryShaderReadUniformBuffer,

    /// Read as a sampled image/uniform texel buffer  in a geometry shader
    GeometryShaderReadSampledImageOrUniformTexelBuffer,

    /// Read as any other resource in a geometry shader
    GeometryShaderReadOther,

    /// On Vulkan requires `VK_NV_mesh_shading` to be enabled
    /// Read as a uniform buffer in a task shader
    TaskShaderReadUniformBufferNv,

    /// On Vulkan requires `VK_NV_mesh_shading` to be enabled
    /// Read as a sampled image/uniform texel buffer in a task shader
    TaskShaderReadSampledImageOrUniformTexelBufferNv,

    /// On Vulkan requires `VK_NV_mesh_shading` to be enabled
    /// Read as any other resource in a task shader
    TaskShaderReadOtherNv,

    /// On Vulkan requires `VK_NV_mesh_shading` to be enabled
    /// Read as a uniform buffer in a mesh shader
    MeshShaderReadUniformBufferNv,

    /// On Vulkan requires `VK_NV_mesh_shading` to be enabled
    /// Read as a sampled image/uniform texel buffer in a mesh shader
    MeshShaderReadSampledImageOrUniformTexelBuffer,

    /// On Vulkan requires `VK_NV_mesh_shading` to be enabled
    /// Read as any other resource in a mesh shader
    MeshShaderReadOther,

    /// On Vulkan requires `VK_EXT_transform_feedback` to be enabled
    /// Read as a transform feedback counter buffer
    TransformFeedbackCounterRead,

    /// On Vulkan requires `VK_EXT_fragment_density_map` to be enabled
    /// Read as a fragment density map image
    FragmentDensityMapRead,

    /// On Vulkan requires `VK_NV_shading_rate_image` to be enabled
    /// Read as a shading rate image
    ShadingRateRead,

    /// Read as a uniform buffer in a fragment shader
    FragmentShaderReadUniformBuffer,

    /// Read as a sampled image/uniform texel buffer  in a fragment shader
    FragmentShaderReadSampledImageOrUniformTexelBuffer,

    /// Read as an input attachment with a color format in a fragment shader
    FragmentShaderReadColorInputAttachment,

    /// Read as an input attachment with a depth/stencil format in a fragment shader
    FragmentShaderReadDepthStencilInputAttachment,

    /// Read as any other resource in a fragment shader
    FragmentShaderReadOther,

    /// Read by standard blending/logic operations or subpass load operations
    ColorAttachmentRead,

    /// On Vulkan requires `VK_EXT_blend_operation_advanced` to be enabled
    /// Read by advanced blending, standard blending, logic operations, or subpass load operations
    ColorAttachmentAdvancedBlending,

    /// Read by depth/stencil tests or subpass load operations
    DepthStencilAttachmentRead,

    /// Read as a uniform buffer in a compute shader
    ComputeShaderReadUniformBuffer,

    /// Read as a sampled image/uniform texel buffer in a compute shader
    ComputeShaderReadSampledImageOrUniformTexelBuffer,

    /// Read as any other resource in a compute shader
    ComputeShaderReadOther,

    /// Read as a uniform buffer in any shader
    AnyShaderReadUniformBuffer,

    /// Read as a uniform buffer in any shader, or a vertex buffer
    AnyShaderReadUniformBufferOrVertexBuffer,

    /// Read as a sampled image in any shader
    AnyShaderReadSampledImageOrUniformTexelBuffer,

    /// Read as any other resource (excluding attachments) in any shader
    AnyShaderReadOther,

    /// Read as the source of a transfer operation
    TransferRead,

    /// Read on the host
    HostRead,

    /// On Vulkan requires `VK_KHR_swapchain` to be enabled
    /// Read by the presentation engine (i.e. vkQueuePresentKHR)
    Present,

    /// On Vulkan requires `VK_EXT_conditional_rendering` to be enabled
    /// Read by conditional rendering
    ConditionalRenderingRead,

    /// On Vulkan requires `VK_NV_ray_tracing` or `VK_KHR_acceleration_structure` to be enabled
    /// Read by a ray tracing shader as an acceleration structure
    RayTracingShaderAccelerationStructureRead,

    /// On Vulkan requires `VK_NV_ray_tracing` to be enabled
    /// Read as an acceleration structure during a build
    AccelerationStructureBuildRead,

    /// On Vulkan requires `VK_NV_ray_tracing` or `VK_KHR_ray_tracing_pipeline` to be enabled
    /// Read as shader binding table
    ShaderBindingTable,

    /*--  BEGIN WRITE ACCESSES  --*/
    /// Requires `VK_NV_device_generated_commands` to be enabled
    /// Command buffer write operation
    CommandBufferWrite = FIRST_WRITE_DISCRIMINATOR,

    /// Written as any resource in a vertex shader
    VertexShaderWrite,

    /// Written as any resource in a tessellation control shader
    TessellationControlShaderWrite,

    /// Written as any resource in a tessellation evaluation shader
    TessellationEvaluationShaderWrite,

    /// Written as any resource in a geometry shader
    GeometryShaderWrite,

    /// On Vulkan requires `VK_NV_mesh_shading` to be enabled
    /// Written as any resource in a task shader
    TaskShaderWrite,

    /// On Vulkan requires `VK_NV_mesh_shading` to be enabled
    /// Written as any resource in a mesh shader
    MeshShaderWrite,

    /// On Vulkan requires `VK_EXT_transform_feedback` to be enabled
    /// Written as a transform feedback buffer
    TransformFeedbackWrite,

    /// On Vulkan requires `VK_EXT_transform_feedback` to be enabled
    /// Written as a transform feedback counter buffer
    TransformFeedbackCounterWrite,

    /// Written as any resource in a fragment shader
    FragmentShaderWrite,

    /// Written as a color attachment during rendering, or via a subpass store op
    ColorAttachmentWrite,

    /// Written as a depth/stencil attachment during rendering, or via a subpass store op
    DepthStencilAttachmentWrite,

    /// On Vulkan requires `VK_KHR_maintenance2` to be enabled
    /// Written as a depth aspect of a depth/stencil attachment during rendering, whilst the stencil
    /// aspect is read-only
    DepthAttachmentWriteStencilReadOnly,

    /// On Vulkan requires `VK_KHR_maintenance2` to be enabled
    /// Written as a stencil aspect of a depth/stencil attachment during rendering, whilst the depth
    /// aspect is read-only
    StencilAttachmentWriteDepthReadOnly,

    /// Written as any resource in a compute shader
    ComputeShaderWrite,

    /// Written as any resource in any shader
    AnyShaderWrite,

    /// Written as the destination of a transfer operation
    TransferWrite,

    /// Data pre-filled by host before device access starts
    HostPreInitialized,

    /// Written on the host
    HostWrite,

    /// On Vulkan requires `VK_NV_ray_tracing` or `VK_KHR_acceleration_structure` to be enabled
    /// Written as an acceleration structure during a build
    AccelerationStructureBuildWrite,

    /// Read or written as a color attachment during rendering
    ColorAttachmentReadWrite,

    /// General access
    /// Covers any access - useful for debug, generally avoid for performance reasons
    General,
}

impl ResourceAccessType {
    /// Returns if the resource access is a read-only access
    pub fn is_read(&self) -> bool {
        (*self as u32) < FIRST_WRITE_DISCRIMINATOR
    }

    /// Returns if the resource access is a write access
    pub fn is_write(&self) -> bool {
        (*self as u32) >= FIRST_WRITE_DISCRIMINATOR
    }
}

/// Typed wrapper that holds a [ResourceAccessType] that is guaranteed to hold a write access
/// type
#[repr(transparent)]
#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Debug, Hash)]
pub struct WriteResourceAccessType(ResourceAccessType);

impl TryFrom<ResourceAccessType> for WriteResourceAccessType {
    type Error = ();

    fn try_from(value: ResourceAccessType) -> Result<Self, Self::Error> {
        if value.is_write() {
            Ok(Self(value))
        } else {
            Err(())
        }
    }
}

/// Typed wrapper that holds a [ResourceAccessType] that is guaranteed to hold a read-only access
/// type
#[repr(transparent)]
#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Debug, Hash)]
pub struct ReadResourceAccessType(ResourceAccessType);

impl TryFrom<ResourceAccessType> for ReadResourceAccessType {
    type Error = ();

    fn try_from(value: ResourceAccessType) -> Result<Self, Self::Error> {
        if value.is_read() {
            Ok(Self(value))
        } else {
            Err(())
        }
    }
}

/// Discriminator base used for read-only accesses
///
/// Allows for checking read-only access by integer comparison
const FIRST_READ_DISCRIMINATOR: u32 = 0;

/// Discriminator base used for write access
///
/// Allows for checking read-only access by integer comparison
const FIRST_WRITE_DISCRIMINATOR: u32 = 2_147_483_648;

/// Trait used for restricting set of markers used for resource type
pub trait ResourceType {}

/// Marker used for buffer resource type
pub struct Buffer();

/// Marker used for image resource type
pub struct Image();

/// Marker + state used for a resource of any type
pub struct Any(ResourceTypes);

impl ResourceType for Buffer {}
impl ResourceType for Image {}
impl ResourceType for Any {}

/// Enum that enumerates all resource types
pub enum ResourceTypes {
    Buffer(Resource<Buffer>),
    Image(Resource<Image>),
}
