//
//
// This file is a part of Aleph
//
// <ALEPH_REPO_REPLACE>
//
// <ALEPH_LICENSE_REPLACE>
//

use crate::gpu::vk::core::Device;
use crate::gpu::vk::reflect::{DescriptorSetReflection, PushConstantReflection};
use erupt::vk1_0::{
    PipelineShaderStageCreateInfoBuilder, ShaderModuleCreateInfoBuilder, ShaderStageFlagBits,
    ShaderStageFlags, Vk10DeviceLoaderExt,
};
use spirv_reflect::types::ReflectShaderStageFlags;
use std::ffi::CStr;

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
        }
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
    pub fn build(self, device: Option<&Device>) -> ShaderModule {
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
        let reflection = spirv_reflect::create_shader_module(bytes)
            .expect("Failed to generate reflection information");

        // Load the entry points
        let mut entry_point = reflection
            .enumerate_entry_points()
            .expect("Failed to enumerate entry points");

        // Make sure there's only one entry point
        assert_eq!(
            entry_point.len(),
            1,
            "Only support SPIRV files with a single entry point"
        );

        // Extract the single expected entry point
        let mut entry_point = entry_point.drain(..).nth(0).unwrap();

        // Extract the shader stage of the entry point
        let stage_flags = entry_point.shader_stage;

        // If we've specified an expected shader stage assert this shader is of the right type.
        // otherwise the shader stage will just be set by reflection information
        if self.stage_flags != ReflectShaderStageFlags::UNDEFINED {
            assert_eq!(
                self.stage_flags, stage_flags,
                "Shader not of expected stage"
            );
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
            | ReflectShaderStageFlags::RAYGEN_BIT_NV => panic!("Unsupported shader stage"),
            _ => panic!("Unknown shader stage"),
        };

        // Generate reflection information if requested
        let (push_constants, descriptor_sets, entry_point_name) = if self.reflect {
            let mut push_constants = reflection
                .enumerate_push_constant_blocks(Some(&entry_point.name))
                .expect("Failed to reflect push constant information");

            let push_constants = if push_constants.is_empty() {
                None
            } else {
                assert_eq!(
                    push_constants.len(),
                    1,
                    "More than one push constant block is invalid"
                );
                push_constants
                    .drain(..)
                    .map(|v| Some(Box::new(PushConstantReflection::reflect(v))))
                    .nth(0)
                    .unwrap()
            };

            let descriptor_sets: Vec<DescriptorSetReflection> =
                DescriptorSetReflection::reflect(&mut entry_point);

            // Get the entry point with having to reallocate
            let mut entry_point_name = String::new();
            std::mem::swap(&mut entry_point_name, &mut entry_point.name);

            // Push a null terminator to do some unsafe fuckery so we can hand out CStr references
            // to the string rather than forcing allocations
            entry_point_name.push('\0');

            (push_constants, descriptor_sets, entry_point_name)
        } else {
            // No reflection information generated so generate defaults and assume entry point name
            // is "main"
            (None, Vec::new(), "main\0".to_string())
        };

        // Compiled the module if requested
        let module = if self.compile {
            let loader = device
                .expect("Need a device ref to compile shader")
                .loader();
            let create_info = ShaderModuleCreateInfoBuilder::new().code(words);
            unsafe {
                loader
                    .create_shader_module(&create_info, None, None)
                    .expect("Failed to create shader module")
            }
        } else {
            erupt::vk1_0::ShaderModule::null()
        };

        ShaderModule {
            module,
            entry_point_name,
            push_constants,
            descriptor_sets,
            stage_flags,
            reflected: self.reflect,
        }
    }
}

///
/// Represents a higher level wrapper around a shader module that can hold reflected information
/// about a shader module
///
pub struct ShaderModule {
    module: erupt::vk1_0::ShaderModule,
    entry_point_name: String,
    push_constants: Option<Box<PushConstantReflection>>,
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
    pub fn module(&self) -> Option<erupt::vk1_0::ShaderModule> {
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
    pub fn pipeline_shader_stage(&self) -> PipelineShaderStageCreateInfoBuilder {
        let stage = ShaderStageFlagBits(self.stage_flags.bits());
        PipelineShaderStageCreateInfoBuilder::new()
            .stage(stage)
            .module(self.module)
            .name(self.entry_point_name_cstr())
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
            Some(push_constants) => Some(&push_constants),
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
    pub unsafe fn destroy(&self, device: &Device) {
        if !self.module.is_null() {
            device.loader().destroy_shader_module(self.module, None)
        }
    }
}
