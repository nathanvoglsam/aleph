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

use crate::reflect::{
    DescriptorSetReflection, PushConstantReflection, StructResolutionError, VertexLayoutReflection,
    VertexLayoutResolutionError,
};
use aleph_vulkan_core::erupt::vk1_0::{
    PipelineShaderStageCreateInfoBuilder, ShaderModuleCreateInfoBuilder, ShaderStageFlagBits,
    ShaderStageFlags,
};
use aleph_vulkan_core::{DebugName, Device};
use spirv_reflect::types::{ReflectEntryPoint, ReflectShaderStageFlags};
use std::ffi::CStr;

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub enum ShaderModuleBuildError {
    ///
    /// Error when the actual spirv module is for a different shader stage than expected
    ///
    ExpectedDifferentModuleStage,

    ///
    /// When the shader loaded is not for a supported shader stage
    ///
    UnsupportedShaderStage,

    ///
    /// Loaded module is for a shader stage we don't know about and haven't explicitly supported or
    /// not supported.
    ///
    UnknownShaderStage,

    ///
    /// When the number of entry points in the shader module is not exactly one
    ///
    InvalidEntryPointCount,

    ///
    /// It is invalid for a shader to specify more than one push constant block per shader stage or
    /// entry point
    ///
    MoreThanOnePushConstantBlock,

    ///
    /// A vertex shader was loaded that has no vertex inputs, we treat this as invalid. There are
    /// some cases where this is needed so maybe we should explicitly support this in the future
    /// with a flag to ignore vertex input reflection
    ///
    VertexShaderHasNoVertexInputs,

    ///
    /// An error was encountered generatin reflection information
    ///
    FailedGeneratingReflection(&'static str),

    ///
    /// Failed to enumerate the entry points in the spirv module
    ///
    FailedEnumeratingEntryPoints(&'static str),

    ///
    /// Failed to generate reflection information for a push constant block
    ///
    FailedReflectingPushConstants(&'static str),

    ///
    /// An error was encountered when resolving a struct as part of the shader module reflection
    ///
    StructResolutionError(StructResolutionError),

    ///
    /// An error was encountered when resolving a vertex layout
    ///
    VertexLayoutResolutionError(VertexLayoutResolutionError),
}

impl From<StructResolutionError> for ShaderModuleBuildError {
    fn from(other: StructResolutionError) -> Self {
        ShaderModuleBuildError::StructResolutionError(other)
    }
}

impl From<VertexLayoutResolutionError> for ShaderModuleBuildError {
    fn from(other: VertexLayoutResolutionError) -> Self {
        ShaderModuleBuildError::VertexLayoutResolutionError(other)
    }
}

///
/// Builder struct for creating a shader module
///
/// Defaults to not generating reflection information and compiling to a vk::ShaderModule
///
pub struct ShaderModuleBuilder<'a> {
    reflect: bool,
    compile: bool,
    stage_flags: ReflectShaderStageFlags,
    words: Option<&'a [u32]>,
    debug_name: Option<&'a CStr>,
}

impl<'a> ShaderModuleBuilder<'a> {
    ///
    /// Creates a new shader module builder
    ///
    /// Defaults to not generating reflection information and compiling to a vk::ShaderModule
    ///
    pub fn new() -> Self {
        Self {
            reflect: false,
            compile: true,
            stage_flags: ReflectShaderStageFlags::UNDEFINED,
            words: None,
            debug_name: None,
        }
    }

    ///
    /// Adds a debug name to be applied to the shader module handle with the VK_EXT_debug_utils
    /// extension
    ///
    pub fn debug_name(mut self, debug_name: &'a CStr) -> Self {
        self.debug_name = Some(debug_name);
        self
    }

    ///
    /// Sets this as a fragment shader
    ///
    /// This is optional if generating reflection information. If this is specified when generating
    /// reflection information `build` will panic if the shader is of the incorrect stage so this
    /// can be used to assert the shader is of a certain stage
    ///
    pub fn fragment(mut self) -> Self {
        self.stage_flags = ReflectShaderStageFlags::FRAGMENT;
        self
    }

    ///
    /// Sets this as a vertex shader
    ///
    /// This is optional if generating reflection information. If this is specified when generating
    /// reflection information `build` will panic if the shader is of the incorrect stage so this
    /// can be used to assert the shader is of a certain stage
    ///
    pub fn vertex(mut self) -> Self {
        self.stage_flags = ReflectShaderStageFlags::VERTEX;
        self
    }

    ///
    /// Sets this as a geometry shader
    ///
    /// This is optional if generating reflection information. If this is specified when generating
    /// reflection information `build` will panic if the shader is of the incorrect stage so this
    /// can be used to assert the shader is of a certain stage
    ///
    pub fn geometry(mut self) -> Self {
        self.stage_flags = ReflectShaderStageFlags::GEOMETRY;
        self
    }

    ///
    /// Sets this as a tessellation control shader
    ///
    /// This is optional if generating reflection information. If this is specified when generating
    /// reflection information `build` will panic if the shader is of the incorrect stage so this
    /// can be used to assert the shader is of a certain stage
    ///
    pub fn tessellation_control(mut self) -> Self {
        self.stage_flags = ReflectShaderStageFlags::TESSELLATION_CONTROL;
        self
    }

    ///
    /// Sets this as a tessellation evaluation shader
    ///
    /// This is optional if generating reflection information. If this is specified when generating
    /// reflection information `build` will panic if the shader is of the incorrect stage so this
    /// can be used to assert the shader is of a certain stage
    ///
    pub fn tessellation_evaluation(mut self) -> Self {
        self.stage_flags = ReflectShaderStageFlags::TESSELLATION_EVALUATION;
        self
    }

    ///
    /// Sets this as a compute shader
    ///
    /// This is optional if generating reflection information. If this is specified when generating
    /// reflection information `build` will panic if the shader is of the incorrect stage so this
    /// can be used to assert the shader is of a certain stage
    ///
    pub fn compute(mut self) -> Self {
        self.stage_flags = ReflectShaderStageFlags::COMPUTE;
        self
    }

    ///
    /// Sets whether reflection information should be generated
    ///
    pub fn reflect(mut self, reflect: bool) -> Self {
        self.reflect = reflect;
        self
    }

    ///
    /// Sets whether the shader should be compiled to a vulkan ShaderModule object
    ///
    pub fn compile(mut self, compile: bool) -> Self {
        self.compile = compile;
        self
    }

    ///
    /// Sets the words the shader will be loaded from. This is u32 because of the vulkan headers so
    /// have fun getting a u32 slice from a file without any unsafe
    ///
    pub fn words(mut self, words: &'a [u32]) -> Self {
        self.words = Some(words);
        self
    }

    ///
    /// Internal function for generating push constant reflection
    ///
    fn generate_push_constants(
        reflection: &spirv_reflect::ShaderModule,
        entry_point: &ReflectEntryPoint,
    ) -> Result<Option<Box<PushConstantReflection>>, ShaderModuleBuildError> {
        let mut push_constants =
            match reflection.enumerate_push_constant_blocks(Some(&entry_point.name)) {
                Ok(v) => v,
                Err(err) => return Err(ShaderModuleBuildError::FailedReflectingPushConstants(err)),
            };

        let push_constants = if push_constants.is_empty() {
            None
        } else {
            if push_constants.len() != 1 {
                return Err(ShaderModuleBuildError::MoreThanOnePushConstantBlock);
            }
            let push_constants = push_constants.drain(..).next().unwrap();

            let push_constants = PushConstantReflection::reflect(push_constants)?;
            let push_constants = Box::new(push_constants);
            Some(push_constants)
        };

        Ok(push_constants)
    }

    ///
    /// Internal function for generating vertex layout reflection
    ///
    fn generate_vertex_layout(
        entry_point: &mut ReflectEntryPoint,
    ) -> Result<Option<Box<VertexLayoutReflection>>, ShaderModuleBuildError> {
        let vertex_layout = if entry_point
            .shader_stage
            .contains(ReflectShaderStageFlags::VERTEX)
        {
            if entry_point.input_variables.is_empty() {
                return Err(ShaderModuleBuildError::VertexShaderHasNoVertexInputs);
            }
            let iterator = entry_point.input_variables.drain(..);
            let vertex_layout = VertexLayoutReflection::reflect(iterator)?;
            let vertex_layout = Box::new(vertex_layout);
            Some(vertex_layout)
        } else {
            None
        };
        Ok(vertex_layout)
    }

    ///
    /// Builds the shader module. Depending on the options provided this will
    ///
    /// - Generate reflection information about the shader
    /// - Compiled the shader to a vulkan ShaderModule object
    ///
    /// If compiling the shader the device arg must provide a device to compile with. Otherwise it
    /// is not needed.
    ///
    /// This way reflection can be generated without having to compiled the shader and this wrapper
    /// can be used in a headless environment (no gpu)
    ///
    pub fn build(self, device: Option<&Device>) -> Result<ShaderModule, ShaderModuleBuildError> {
        // Get the shader words
        let words = self.words.expect("Shader bytes weren't specified");

        // We'll also need a byte slice of the words
        let bytes = unsafe {
            core::slice::from_raw_parts(
                words.as_ptr() as *const u8,
                words.len() * core::mem::size_of::<u32>(),
            )
        };

        // At minimum we need to check the shader stage so we need this no matter what settings
        let reflection = match spirv_reflect::create_shader_module(bytes) {
            Ok(v) => v,
            Err(err) => return Err(ShaderModuleBuildError::FailedGeneratingReflection(err)),
        };

        // Load the entry points
        let mut entry_point = match reflection.enumerate_entry_points() {
            Ok(v) => v,
            Err(err) => return Err(ShaderModuleBuildError::FailedEnumeratingEntryPoints(err)),
        };

        // Make sure there's only one entry point
        if entry_point.len() != 1 {
            return Err(ShaderModuleBuildError::InvalidEntryPointCount);
        }

        // Extract the single expected entry point
        let mut entry_point = entry_point.drain(..).next().unwrap();

        // Extract the shader stage of the entry point
        let stage_flags = entry_point.shader_stage;

        // If we've specified an expected shader stage assert this shader is of the right type.
        // otherwise the shader stage will just be set by reflection information
        if self.stage_flags != ReflectShaderStageFlags::UNDEFINED && self.stage_flags != stage_flags
        {
            return Err(ShaderModuleBuildError::ExpectedDifferentModuleStage);
        }

        // Map the internal shader stage flags to vulkan form
        let stage_flags = match stage_flags {
            ReflectShaderStageFlags::FRAGMENT => ShaderStageFlags::FRAGMENT,
            ReflectShaderStageFlags::VERTEX => ShaderStageFlags::VERTEX,
            ReflectShaderStageFlags::GEOMETRY => ShaderStageFlags::GEOMETRY,
            ReflectShaderStageFlags::TESSELLATION_CONTROL => ShaderStageFlags::TESSELLATION_CONTROL,
            ReflectShaderStageFlags::TESSELLATION_EVALUATION => {
                ShaderStageFlags::TESSELLATION_EVALUATION
            }
            ReflectShaderStageFlags::COMPUTE => ShaderStageFlags::COMPUTE,
            ReflectShaderStageFlags::ANY_HIT_BIT_NV
            | ReflectShaderStageFlags::CALLABLE_BIT_NV
            | ReflectShaderStageFlags::CLOSEST_HIT_BIT_NV
            | ReflectShaderStageFlags::INTERSECTION_BIT_NV
            | ReflectShaderStageFlags::MISS_BIT_NV
            | ReflectShaderStageFlags::RAYGEN_BIT_NV => {
                return Err(ShaderModuleBuildError::UnsupportedShaderStage)
            }
            _ => return Err(ShaderModuleBuildError::UnknownShaderStage),
        };

        // Generate reflection information if requested
        let push_constants = if self.reflect {
            Self::generate_push_constants(&reflection, &entry_point)?
        } else {
            None
        };

        // Generate reflection information if requested
        let vertex_layout = if self.reflect {
            Self::generate_vertex_layout(&mut entry_point)?
        } else {
            None
        };

        // Generate reflection information if requested
        let descriptor_sets = if self.reflect {
            DescriptorSetReflection::reflect(&mut entry_point)
        } else {
            Vec::new()
        };

        // Generate reflection information if requested
        let entry_point_name = if self.reflect {
            // Get the entry point with having to reallocate
            let mut entry_point_name = String::new();
            std::mem::swap(&mut entry_point_name, &mut entry_point.name);

            // Push a null terminator to do some unsafe fuckery so we can hand out CStr references
            // to the string rather than forcing allocations
            entry_point_name.push('\0');

            entry_point_name
        } else {
            "main\0".to_string()
        };

        // Compiled the module if requested
        let module = if self.compile {
            let device = device.expect("Need a device ref to compile shader");
            let create_info = ShaderModuleCreateInfoBuilder::new().code(words);
            unsafe {
                let module = device
                    .create_shader_module(&create_info, None)
                    .expect("Failed to create shader module");
                if let Some(name) = self.debug_name {
                    module.add_debug_name(device, name);
                }
                module
            }
        } else {
            aleph_vulkan_core::erupt::vk1_0::ShaderModule::null()
        };

        let module = ShaderModule {
            module,
            entry_point_name,
            push_constants,
            vertex_layout,
            descriptor_sets,
            stage_flags,
            reflected: self.reflect,
        };

        Ok(module)
    }
}

impl<'a> Default for ShaderModuleBuilder<'a> {
    fn default() -> Self {
        Self::new()
    }
}

///
/// Represents a higher level wrapper around a shader module that can hold reflected information
/// about a shader module
///
pub struct ShaderModule {
    module: aleph_vulkan_core::erupt::vk1_0::ShaderModule,
    entry_point_name: String,
    push_constants: Option<Box<PushConstantReflection>>,
    vertex_layout: Option<Box<VertexLayoutReflection>>,
    descriptor_sets: Vec<DescriptorSetReflection>,
    stage_flags: ShaderStageFlags,
    reflected: bool,
}

impl ShaderModule {
    ///
    /// Gets a builder
    ///
    pub fn builder<'a>() -> ShaderModuleBuilder<'a> {
        ShaderModuleBuilder::new()
    }

    ///
    /// Checks whether this module was compiled and has a valid vulkan ShaderModule handle
    ///
    pub fn compiled(&self) -> bool {
        !self.module.is_null()
    }

    ///
    /// Gets the internal vulkan ShaderModule handle if it was compiled on object creation.
    /// This returns None if the module wasn't compiled
    ///
    pub fn module(&self) -> Option<aleph_vulkan_core::erupt::vk1_0::ShaderModule> {
        if self.module.is_null() {
            None
        } else {
            Some(self.module)
        }
    }

    ///
    /// Gets the name of the shader's entry point
    ///
    pub fn entry_point_name(&self) -> &str {
        let len = self.entry_point_name.len();
        &self.entry_point_name[0..len - 1]
    }

    ///
    /// Gets the name of the shader's entry point as a cstr for use with raw vulkan calls
    ///
    pub fn entry_point_name_cstr(&self) -> &CStr {
        // This is safe because the string is guaranteed to have a null terminator because of the
        // how the type is constructed
        unsafe { CStr::from_ptr(self.entry_point_name.as_ptr() as *const _) }
    }

    ///
    /// Creates a `PipelineShaderStageCreateInfo` for this shader module
    ///
    pub fn pipeline_shader_stage(&self) -> Option<PipelineShaderStageCreateInfoBuilder> {
        self.module().map(|module| {
            let stage = ShaderStageFlagBits(self.stage_flags.bits());
            PipelineShaderStageCreateInfoBuilder::new()
                .stage(stage)
                .module(module)
                .name(self.entry_point_name_cstr())
        })
    }

    ///
    /// Gets whether reflection information was generated for this shader module
    ///
    pub fn reflected(&self) -> bool {
        self.reflected
    }

    ///
    /// Gets the list of reflected push constant blocks.
    ///
    /// This will be None if no reflection was generated but may also be None even with reflection
    /// generated so make sure to check with `Self::reflected`
    ///
    pub fn push_constants(&self) -> Option<&PushConstantReflection> {
        match &self.push_constants {
            None => None,
            Some(push_constants) => Some(push_constants),
        }
    }

    ///
    /// Gets the vertex layout reflection for the struct if this is a vertex shader. Otherwise
    /// returns None
    ///
    /// This will be None if no reflection was generated but may also be None even with reflection
    /// generated so make sure to check with `Self::reflected`
    ///
    pub fn vertex_layout(&self) -> Option<&VertexLayoutReflection> {
        match &self.vertex_layout {
            None => None,
            Some(vertex_layout) => Some(vertex_layout),
        }
    }

    ///
    /// Gets the list of reflected descriptor sets
    ///
    /// This will be empty if no reflection was generated but may also be empty even with reflection
    /// generated so make sure to check with `Self::reflected`
    ///
    pub fn descriptor_sets(&self) -> &[DescriptorSetReflection] {
        &self.descriptor_sets
    }

    ///
    /// Get the internal shader stage flags in case it is needed
    ///
    pub fn shader_stage_flags(&self) -> ShaderStageFlags {
        self.stage_flags
    }

    ///
    /// Gets whether this is a fragment shader.
    ///
    pub fn is_fragment_shader(&self) -> bool {
        self.stage_flags == ShaderStageFlags::FRAGMENT
    }

    ///
    /// Gets whether this is a vertex shader.
    ///
    pub fn is_vertex_shader(&self) -> bool {
        self.stage_flags == ShaderStageFlags::VERTEX
    }

    ///
    /// Gets whether this is a geometry shader.
    ///
    pub fn is_geometry_shader(&self) -> bool {
        self.stage_flags == ShaderStageFlags::GEOMETRY
    }

    ///
    /// Gets whether this is a tessellation control shader.
    ///
    pub fn is_tessellation_control_shader(&self) -> bool {
        self.stage_flags == ShaderStageFlags::TESSELLATION_CONTROL
    }

    ///
    /// Gets whether this is a tessellation evaluation shader.
    ///
    pub fn is_tessellation_evaluation_shader(&self) -> bool {
        self.stage_flags == ShaderStageFlags::TESSELLATION_EVALUATION
    }

    ///
    /// Gets whether this is a compute shader.
    ///
    pub fn is_compute_shader(&self) -> bool {
        self.stage_flags == ShaderStageFlags::COMPUTE
    }

    ///
    /// Destroys a compiled module. Will do nothing if the module wasn't compiled but is marked
    /// unsafe because the destroy is not synchronized
    ///
    /// # Safety
    ///
    /// This doesn't perform any synchronization for destroying the underlying VkShaderModule. The
    /// caller must ensure it is safe to destroy the shader module.
    ///
    pub unsafe fn destroy(&self, device: &Device) {
        if !self.module.is_null() {
            device.destroy_shader_module(Some(self.module), None)
        }
    }
}
