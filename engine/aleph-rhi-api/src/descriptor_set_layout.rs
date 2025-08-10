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

use std::num::NonZeroU32;

use aleph_object_system::ArcObject;

use crate::*;

#[derive(Clone)]
pub struct DescriptorSetLayoutHandle {
    inner: ArcObject,
}

impl DescriptorSetLayoutHandle {
    /// # Safety
    ///
    /// It is the caller's responsibility to ensure that the given object refers to an object that
    /// the inner RHI implementation considers a semaphore objec.
    pub const unsafe fn new(inner: ArcObject) -> Self {
        Self { inner }
    }

    ///
    /// Gets the number of strong ([`DescriptorSetLayoutHandle`]) pointers to this allocation.
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

    /// Unwrap the [`DescriptorSetLayoutHandle`] and get the inner [`ArcObject`]
    #[inline]
    pub fn into_inner(self) -> ArcObject {
        self.inner
    }

    /// Get the inner [`ArcObject`]
    pub const fn get(&self) -> &ArcObject {
        &self.inner
    }
}

#[derive(Clone, Default)]
pub struct DescriptorSetLayoutDesc<'a> {
    /// Specifies which shader stages can access a resource for this set
    pub visibility: DescriptorShaderVisibility,

    /// A list of all bindings that are a part of this descriptor set layout
    pub items: &'a [DescriptorSetLayoutBinding],

    /// The name of the object
    pub name: Option<&'a str>,
}

#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Debug)]
pub enum DescriptorType {
    /// A sampler descriptor. Maps as a sampler on both DX12 and Vulkan
    Sampler,

    /// A Texel Buffer, or typed buffer. Uniform texel buffer on Vulkan, 'Buffer' on DX12
    TexelBuffer,

    /// A Texel Buffer with read/write access. Storage texel buffer on Vulkan, 'BufferRW' on DX12.
    /// This is a UAV for DX12.
    TexelBufferRW,

    /// A Texture descriptor. Sampled Image on Vulkan, 'Texture' on DX12.
    Texture,

    /// A read/write Texture descriptor. Storage Image on Vulkan, 'RWTexture' on DX12. This is a
    /// UAV for DX12.
    TextureRW,

    /// A UniformBuffer/ConstantBuffer descriptor. Maps accordingly on Vulkan/D3D12. CBV on DX12.
    UniformBuffer,

    /// A dynamic UniformBuffer/ConstantBuffer descriptor. Maps accordingly on Vulkan/D3D12. CBV on
    /// DX12.
    UniformBufferDynamic,

    /// A buffer with a stride that represents N*stride items. Layout defined in the shader. On
    /// Vulkan this is just a storage buffer, D3D12 this is a SRV as it's read-only.
    StructuredBuffer,

    /// The same as [DescriptorType::StructuredBuffer], but read/write. Still a storage buffer on
    /// Vulkan but D3D12 requires a UAV for write access.
    StructuredBufferRW,

    /// A raw 'bag of bytes' like buffer. No stride info. Again a storage buffer on Vulkan, but it's
    /// a SRV on D3D12.
    ByteAddressBuffer,

    /// The same as [DescriptorType::ByteAddressBuffer] but read/write. Remains a storage buffer on
    /// Vulkan but D3D12 needs UAV again because of write access.
    ByteAddressBufferRW,

    /// An acceleration structure descriptor, which is a special kind of buffer descriptor that is
    /// used for raytracing acceleration structures.
    AccelerationStructure,

    /// UNIMPLEMENTED
    InputAttachment,
}

impl DescriptorType {
    pub const fn binding(self, num: u32) -> DescriptorSetLayoutBinding {
        DescriptorSetLayoutBinding {
            binding_num: num,
            binding_type: self,
            binding_count: None,
        }
    }
}

impl std::fmt::Display for DescriptorType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DescriptorType::Sampler => f.write_str("Sampler"),
            DescriptorType::TexelBuffer => f.write_str("TexelBuffer"),
            DescriptorType::TexelBufferRW => f.write_str("TexelBufferRW"),
            DescriptorType::Texture => f.write_str("Texture"),
            DescriptorType::TextureRW => f.write_str("TextureRW"),
            DescriptorType::UniformBuffer => f.write_str("UniformBuffer"),
            DescriptorType::UniformBufferDynamic => f.write_str("UniformBufferDynamic"),
            DescriptorType::StructuredBuffer => f.write_str("StructuredBuffer"),
            DescriptorType::StructuredBufferRW => f.write_str("StructuredBufferRW"),
            DescriptorType::ByteAddressBuffer => f.write_str("ByteAddressBuffer"),
            DescriptorType::ByteAddressBufferRW => f.write_str("ByteAddressBufferRW"),
            DescriptorType::AccelerationStructure => f.write_str("AccelerationStructure"),
            DescriptorType::InputAttachment => f.write_str("InputAttachment"),
        }
    }
}

impl Default for DescriptorType {
    #[inline(always)]
    fn default() -> Self {
        Self::Sampler
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

#[derive(Clone, Default)]
pub struct DescriptorSetLayoutBinding {
    /// The binding number of this entry and corresponds to a resource of the same binding number in
    /// the shader stages.
    pub binding_num: u32,

    /// Specifies which type of resource descriptors are used for this binding
    pub binding_type: DescriptorType,

    /// Specifies the number of descriptors contained in the binding. Should be 1 to declare a
    /// single binding, or >1 to declare an array of descriptors.
    pub binding_count: Option<NonZeroU32>,
}

impl DescriptorSetLayoutBinding {
    /// Constructs a new, defaulted [DescriptorSetLayoutBinding] with the given descriptor type.
    pub const fn with_type(descriptor_type: DescriptorType) -> Self {
        Self {
            binding_num: 0,
            binding_type: descriptor_type,
            binding_count: None,
        }
    }

    /// Takes the given desc and returns a new desc with [DescriptorSetLayoutBinding::binding_num]
    /// set to the given value
    pub const fn with_binding_num(mut self, binding_num: u32) -> Self {
        self.binding_num = binding_num;
        self
    }

    /// Takes the given desc and returns a new desc with [DescriptorSetLayoutBinding::binding_count]
    /// set to the given value
    pub const fn with_binding_count(mut self, binding_count: NonZeroU32) -> Self {
        self.binding_count = Some(binding_count);
        self
    }
}
