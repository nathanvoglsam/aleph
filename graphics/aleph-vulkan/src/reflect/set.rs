//
//
// This file is a part of Aleph
//
// <ALEPH_REPO_REPLACE>
//
// <ALEPH_LICENSE_REPLACE>
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
    UniformBufferDynamic(Struct),
    //StorageBufferDynamic(),
    InputAttachment,
    AccelerationStructureNV,
}

impl BindingType {
    ///
    /// Get the VkDescriptorType for this binding type
    ///
    pub fn descriptor_type(&self) -> DescriptorType {
        match self {
            BindingType::Sampler => DescriptorType::SAMPLER,
            BindingType::CombinedImageSampler => DescriptorType::COMBINED_IMAGE_SAMPLER,
            BindingType::SampledImage => DescriptorType::SAMPLED_IMAGE,
            BindingType::StorageImage => DescriptorType::STORAGE_IMAGE,
            BindingType::UniformBuffer(_) => DescriptorType::UNIFORM_BUFFER,
            BindingType::UniformBufferDynamic(_) => DescriptorType::UNIFORM_BUFFER_DYNAMIC,
            BindingType::InputAttachment => DescriptorType::INPUT_ATTACHMENT,
            BindingType::AccelerationStructureNV => DescriptorType::ACCELERATION_STRUCTURE_NV,
        }
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
    /// Builds a list of DescriptorSetLayoutBinding objects from the reflected data
    ///
    pub fn set_layout_bindings(
        &self,
        stage_flags: ShaderStageFlags,
    ) -> Vec<DescriptorSetLayoutBindingBuilder<'static>> {
        self.bindings
            .iter()
            .map(|binding| {
                DescriptorSetLayoutBindingBuilder::new()
                    .binding(binding.binding())
                    .descriptor_type(binding.binding_type().descriptor_type())
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
            .map(|d| Self::reflect_internal(d))
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
                        BindingType::UniformBufferDynamic(
                            resolve_struct_block(b.block)
                                .expect("Failed to reflect dynamic uniform buffer block"),
                        )
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
        let out_set = DescriptorSetReflection { set, bindings };

        out_set
    }
}
