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

use crate::reflect::BindingMapperFn;
use crate::shader::ShaderModule;
use aleph_vulkan_core::erupt::vk1_0::{
    DescriptorSetLayout, DescriptorSetLayoutCreateInfoBuilder, PipelineLayoutCreateInfoBuilder,
    PushConstantRangeBuilder, ShaderStageFlags,
};
use aleph_vulkan_core::{DebugName, Device};
use std::ffi::CStr;

///
/// An enum to represent the possible errors that can be encountered when creating a pipeline layout
///
#[derive(Copy, Clone, Hash, Ord, PartialOrd, Eq, PartialEq, Debug)]
pub enum PipelineLayoutBuildError {
    ///
    /// When multiple shader modules from the same stage are trying to be bound together. This is
    /// invalid as it is not possible to bind multiple of the same shader type. I.e two fragment
    /// shaders.
    ///
    MultipleShadersInSameStage,

    ///
    /// No modules were specified to the builder.
    ///
    NoModulesSpecified,

    ///
    /// This wrapper requires reflection information for all the shaders.
    ///
    NoReflectionInformation,

    ///
    /// Multiple shaders bind a descriptor at the same set index but expect different layouts.
    ///
    IncompatibleDescriptorLayouts,
}

///
/// A wrapper for building pipeline layouts from reflected shader information
///
pub struct PipelineLayoutBuilder<'a> {
    modules: Option<&'a [(&'a ShaderModule, Option<&'a dyn BindingMapperFn>)]>,
    debug_name: Option<&'a CStr>,
}

impl<'a> PipelineLayoutBuilder<'a> {
    ///
    /// Creates a new builder
    ///
    pub fn new() -> Self {
        Self {
            modules: None,
            debug_name: None,
        }
    }

    ///
    /// Adds a debug name to be applied to the pipeline layout handle with the VK_EXT_debug_utils
    /// extension
    ///
    pub fn debug_name(mut self, debug_name: &'a CStr) -> Self {
        self.debug_name = Some(debug_name);
        self
    }

    ///
    /// Provide the list of shader modules to build from, with an optionally attached binding mapper
    /// closure for each module
    ///
    pub fn modules(
        mut self,
        modules: &'a [(&'a ShaderModule, Option<&'a dyn BindingMapperFn>)],
    ) -> Self {
        self.modules = Some(modules);
        self
    }

    ///
    /// Builds a pipeline layout based on the provided list of modules
    ///
    pub fn build(self, device: &Device) -> Result<PipelineLayout, PipelineLayoutBuildError> {
        // Get the list of modules, this must exist
        let modules = self.modules.expect("No modules provided");

        //
        // CHECK WE DONT HAVE MULTIPLE SHADERS FROM THE SAME STAGE
        //

        // Accumulate a list of all stages that are specified by the list of modules
        let mut stages = ShaderStageFlags::default();
        for v in modules.iter() {
            stages |= v.0.shader_stage_flags();

            // Already looping here so may as well assert and not iterate again just for this
            if !v.0.reflected() {
                return Err(PipelineLayoutBuildError::NoReflectionInformation);
            }
        }

        // Now we iterate over the modules again and remove the stage flags if they're specified.
        // If two modules are of the same stage then one will remove the flag before the other so
        // if a module tries to remove a stage flag but it isn't there then multiple modules are
        // of the same stage
        for v in modules.iter() {
            if stages.contains(v.0.shader_stage_flags()) {
                stages.set(v.0.shader_stage_flags(), false);
            } else {
                return Err(PipelineLayoutBuildError::MultipleShadersInSameStage);
            }
        }

        //
        // BUILD LIST OF PUSH CONSTANT RANGES
        //

        // Extract push constant reflection from all modules that specify one
        let mut push_constants = Vec::new();
        modules.iter().for_each(|v| {
            if let Some(p) = v.0.push_constants() {
                push_constants.push((p, v.0.shader_stage_flags()));
            }
        });

        // Merge identical push constant layouts into single ranges that are valid across multiple
        // shader stages
        let mut i = 0;
        loop {
            // Have to capture length each iteration as it can change between iterations
            if i == push_constants.len() {
                break;
            }

            // Remove the current element we're looking for duplicates of
            let mut current = push_constants.remove(0);

            // Iterate through the rest of the list
            let mut j = 0;
            'push_inner: loop {
                // Break from the inner loop once we've been through the whole list
                if j == push_constants.len() {
                    break 'push_inner;
                }

                // Get the element we're checking against
                let other = &push_constants[j];

                // If two push constant blocks are identical then we can should merge them into one
                // that is accessible from both shader stages as a single range
                //
                // That is, if they have the same layout and resource name then we interpret that
                // as trying to use the same resource and so merge the two blocks to once push
                // constant block accessible from multiple stages
                if current.0 == other.0 {
                    // Add the stage usage flag
                    current.1 |= other.1;

                    // Remove the duplicate
                    push_constants.remove(j);

                    // Skip the index increment
                    continue 'push_inner;
                }
                j += 1;
            }
            push_constants.push(current);
            i += 1;
        }

        // Transform our flattened and merged list into a list of PushConstantRangeBuilders
        let push_constant_ranges: Vec<PushConstantRangeBuilder> = push_constants
            .drain(..)
            .map(|v| {
                PushConstantRangeBuilder::new()
                    .stage_flags(v.1)
                    .offset(v.0.offset())
                    .size(v.0.size_padded())
            })
            .collect();

        //
        // BUILD LIST OF DESCRIPTOR SET LAYOUTS
        //

        // Build a flat list of all descriptor sets from the shader modules
        let mut sets = Vec::new();
        modules.iter().for_each(|v| {
            v.0.descriptor_sets().iter().for_each(|s| {
                sets.push((s, v.0.shader_stage_flags(), v.1));
            })
        });

        // Deduplicate the sets
        let mut i = 0;
        loop {
            // Have to capture length each iteration as it can change between iterations
            if i == sets.len() {
                break;
            }

            // Remove the element we're checking duplicates for
            let mut current = sets.remove(0);

            // Iterate through the rest of the list
            let mut j = 0;
            'sets_inner: loop {
                // Break from the inner loop once we've been through the whole list
                if j == sets.len() {
                    break 'sets_inner;
                }

                // Get the element we're checking against
                let other = &sets[j];

                // If the set index is the same then these sets must have the same layout as they
                // point to the same resources
                if current.0.set() == other.0.set() {
                    // Assert that the two sets share the same layout
                    if current.0 != other.0 {
                        return Err(PipelineLayoutBuildError::IncompatibleDescriptorLayouts);
                    }
                    // Add stage usage
                    current.1 |= other.1;

                    // Remove duplicate set
                    sets.remove(j);

                    // Skip index increment
                    continue 'sets_inner;
                }
                j += 1;
            }
            sets.push(current);
            i += 1;
        }

        // Create DescriptorSetLayouts for the final deduplicated layouts
        let set_layouts: Vec<DescriptorSetLayout> = sets
            .drain(..)
            .map(|v| {
                let bindings = v.0.set_layout_bindings(v.1, v.2);
                let create_info = DescriptorSetLayoutCreateInfoBuilder::new().bindings(&bindings);
                unsafe {
                    device
                        .create_descriptor_set_layout(&create_info, None)
                        .expect("Failed to create descriptor set layout")
                }
            })
            .collect();

        let create_info = PipelineLayoutCreateInfoBuilder::new()
            .push_constant_ranges(&push_constant_ranges)
            .set_layouts(&set_layouts);
        let pipeline_layout = unsafe {
            let pipeline_layout = device
                .create_pipeline_layout(&create_info, None)
                .expect("Failed to create pipeline layout");

            if let Some(name) = self.debug_name {
                pipeline_layout.add_debug_name(device, name);
            }

            pipeline_layout
        };

        Ok(PipelineLayout {
            pipeline_layout,
            set_layouts,
        })
    }
}

///
///
///
pub struct PipelineLayout {
    pipeline_layout: aleph_vulkan_core::erupt::vk1_0::PipelineLayout,
    set_layouts: Vec<DescriptorSetLayout>,
}

impl PipelineLayout {
    ///
    /// Creates a builder
    ///
    pub fn builder<'a>() -> PipelineLayoutBuilder<'a> {
        PipelineLayoutBuilder::new()
    }

    ///
    /// Returns the underlying pipeline layout handle
    ///
    pub fn pipeline_layout(&self) -> aleph_vulkan_core::erupt::vk1_0::PipelineLayout {
        self.pipeline_layout
    }

    ///
    /// Returns the list of descriptor set layouts this pipeline layout was built with
    ///
    pub fn set_layouts(&self) -> &[DescriptorSetLayout] {
        &self.set_layouts
    }

    ///
    /// Destroys all resources associated with this PipelineLayout object
    ///
    /// Is unsafe because destruction is not synchronized
    ///
    pub unsafe fn destroy(&self, device: &Device) {
        device.destroy_pipeline_layout(Some(self.pipeline_layout), None);
        self.set_layouts
            .iter()
            .for_each(|v| device.destroy_descriptor_set_layout(Some(*v), None));
    }
}