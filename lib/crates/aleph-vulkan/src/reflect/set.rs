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

use crate::reflect::structure::resolve_struct_block;
use crate::reflect::Struct;
use aleph_vulkan_core::erupt::vk1_0::{
    DescriptorSetLayoutBindingBuilder, DescriptorType, ShaderStageFlags,
};
use spirv_reflect::types::{ReflectDescriptorSet, ReflectDescriptorType, ReflectEntryPoint};

///
/// Type that represents the set of supported descriptor bindings that is currently supported
///
#[derive(Clone, Hash, PartialEq, Eq, Debug)]
pub enum BindingType {
    //Undefined,
    Sampler,
    CombinedImageSampler,
    SampledImage,
    StorageImage,
    //UniformTexelBuffer,
    //StorageTexelBuffer,
    UniformBuffer(Struct),
    //StorageBuffer(),
    InputAttachment,
    AccelerationStructureNV,
}

impl BindingType {
    ///
    /// Get the VkDescriptorType for this binding type
    ///
    pub fn descriptor_type(&self, buffer_type: BufferBindingType) -> DescriptorType {
        match self {
            BindingType::Sampler => DescriptorType::SAMPLER,
            BindingType::CombinedImageSampler => DescriptorType::COMBINED_IMAGE_SAMPLER,
            BindingType::SampledImage => DescriptorType::SAMPLED_IMAGE,
            BindingType::StorageImage => DescriptorType::STORAGE_IMAGE,
            BindingType::UniformBuffer(_) => match buffer_type {
                BufferBindingType::Static => DescriptorType::UNIFORM_BUFFER,
                BufferBindingType::Dynamic => DescriptorType::UNIFORM_BUFFER_DYNAMIC,
            },
            BindingType::InputAttachment => DescriptorType::INPUT_ATTACHMENT,
            BindingType::AccelerationStructureNV => DescriptorType::ACCELERATION_STRUCTURE_NV,
        }
    }
}

///
/// An enum for specifying whether a buffer descriptor binding should be dynamic or static
///
#[derive(Clone, Hash, PartialEq, Eq, Debug)]
pub enum BufferBindingType {
    ///
    /// A static buffer binding
    ///
    /// This maps to descriptor types:
    /// DescriptorType::STORAGE_BUFFER
    /// DescriptorType::UNIFORM_BUFFER
    ///
    Static,

    ///
    /// A uniform buffer binding
    ///
    /// This maps to descriptor types:
    /// DescriptorType::STORAGE_BUFFER_DYNAMIC
    /// DescriptorType::UNIFORM_BUFFER_DYNAMIC
    ///
    Dynamic,
}

impl Default for BufferBindingType {
    fn default() -> Self {
        BufferBindingType::Static
    }
}

///
/// A struct that represents a descriptor binding
///
#[derive(Clone, Hash, PartialEq, Eq, Debug)]
pub struct Binding {
    binding: u32,
    name: String,
    binding_type: BindingType,
}

impl Binding {
    ///
    /// Returns the name of the binding
    ///
    pub fn name(&self) -> &str {
        &self.name
    }

    ///
    /// Returns the type of this binding
    ///
    pub fn binding_type(&self) -> &BindingType {
        &self.binding_type
    }

    ///
    /// Get the binding index
    ///
    pub fn binding(&self) -> u32 {
        self.binding
    }
}

pub trait BindingMapperFn: Fn(&Binding) -> BufferBindingType {}
impl<T: Fn(&Binding) -> BufferBindingType> BindingMapperFn for T {}

///
/// Represents the reflection of a single descriptor set
///
#[derive(Clone, Hash, PartialEq, Eq, Debug)]
pub struct DescriptorSetReflection {
    set: u32,
    bindings: Vec<Binding>,
}

impl DescriptorSetReflection {
    ///
    /// Returns the list of bindings in this descriptor set
    ///
    pub fn bindings(&self) -> &[Binding] {
        &self.bindings
    }

    ///
    /// Get the set index
    ///
    pub fn set(&self) -> u32 {
        self.set
    }

    ///
    /// Builds a list of DescriptorSetLayoutBinding objects from the reflected data.
    ///
    /// Provides an optional parameter for mapping from a given binding to whether it should be a
    /// dynamic or static buffer binding. This is only useful for specifying whether a uniform
    /// buffer should be UNIFORM_BUFFER or UNIFORM_BUFFER_DYNAMIC as it must be done manually (can
    /// not be deduced from shader reflection)
    ///
    pub fn set_layout_bindings(
        &self,
        stage_flags: ShaderStageFlags,
        binding_mapper: Option<&dyn BindingMapperFn>,
    ) -> Vec<DescriptorSetLayoutBindingBuilder<'static>> {
        self.bindings
            .iter()
            .map(|binding| {
                // Let the user function decide whether the buffer is dynamic or static
                let binding_type = binding_mapper.map(|v| (*v)(binding)).unwrap_or_default();
                DescriptorSetLayoutBindingBuilder::new()
                    .binding(binding.binding())
                    .descriptor_type(binding.binding_type().descriptor_type(binding_type))
                    .descriptor_count(1)
                    .stage_flags(stage_flags)
            })
            .collect()
    }

    ///
    /// Consume the descriptor set information from a given entry point and produce the list of
    /// descriptor sets it uses
    ///
    pub fn reflect(entry_point: &mut ReflectEntryPoint) -> Vec<DescriptorSetReflection> {
        entry_point
            .descriptor_sets
            .drain(..)
            .map(Self::reflect_internal)
            .collect()
    }

    ///
    /// Create a new `Set` object from the output produced by `spirv-reflect`
    ///
    fn reflect_internal(mut set: ReflectDescriptorSet) -> DescriptorSetReflection {
        let bindings = set
            .bindings
            .drain(..)
            .map(|b| {
                let name = b.name;
                let binding_type = match b.descriptor_type {
                    ReflectDescriptorType::StorageBufferDynamic
                    | ReflectDescriptorType::StorageBuffer
                    | ReflectDescriptorType::UniformTexelBuffer
                    | ReflectDescriptorType::StorageTexelBuffer
                    | ReflectDescriptorType::Undefined => panic!("Unsupported descriptor type"),
                    ReflectDescriptorType::Sampler => BindingType::Sampler,
                    ReflectDescriptorType::CombinedImageSampler => {
                        BindingType::CombinedImageSampler
                    }
                    ReflectDescriptorType::SampledImage => BindingType::SampledImage,
                    ReflectDescriptorType::StorageImage => BindingType::StorageImage,
                    ReflectDescriptorType::UniformBuffer => BindingType::UniformBuffer(
                        resolve_struct_block(b.block)
                            .expect("Failed to reflect uniform buffer block"),
                    ),
                    ReflectDescriptorType::UniformBufferDynamic => {
                        unreachable!("It shouldn't be possible for spirv-reflect to emit this")
                    }
                    ReflectDescriptorType::InputAttachment => BindingType::InputAttachment,
                    ReflectDescriptorType::AccelerationStructureNV => {
                        BindingType::AccelerationStructureNV
                    }
                };
                let binding = b.binding;
                Binding {
                    binding,
                    name,
                    binding_type,
                }
            })
            .collect();

        let set = set.set;
        DescriptorSetReflection { set, bindings }
    }
}
