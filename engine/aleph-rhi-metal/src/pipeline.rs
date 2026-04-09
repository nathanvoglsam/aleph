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
use std::sync::Arc;

use aleph_object_system::{Object, unsafe_impl_iobject};
use aleph_rhi_api::*;
use objc2::rc::Retained;
use objc2::runtime::ProtocolObject;
use objc2_foundation::{NSString, ns_string};
use objc2_metal::*;

use crate::binding_signature::BindingSignature;
use crate::device::Device;
use crate::internal::{conv, unwrap};

pub struct GraphicsPipeline {
    pub(crate) _device: Arc<Device>,
    pub(crate) _binding_signature: Arc<BindingSignature>,
    pub(crate) id: NonZeroU64,
    pub(crate) objects: GraphicsPipelineObjects,
    pub(crate) info: CachedGraphicsInfo,
}

unsafe_impl_iobject!(GraphicsPipeline, "01980753-5c4f-7ae3-be3b-9707082d77a7");

impl GraphicsPipeline {
    pub(crate) fn create(
        device: &Device,
        desc: &GraphicsPipelineDesc,
    ) -> Result<GraphicsPipelineHandle, PipelineCreateError> {
        let mtl_desc = MTLRenderPipelineDescriptor::new();

        for &stage in desc.shader_stages {
            match stage.shader_type() {
                ShaderType::Compute => panic!("Graphics pipelines can't use compute shaders!"),
                ShaderType::Vertex => unsafe {
                    let function = compile_function(device, stage)?;
                    mtl_desc.setVertexFunction(Some(&function));
                },
                ShaderType::Hull => unimplemented!(),
                ShaderType::Domain => unimplemented!(),
                ShaderType::Geometry => unimplemented!(),
                ShaderType::Fragment => unsafe {
                    let function = compile_function(device, stage)?;
                    mtl_desc.setFragmentFunction(Some(&function));
                },
                ShaderType::Amplification => unimplemented!(),
                ShaderType::Mesh => unimplemented!(),
            }
        }

        let binding_signature = unwrap::binding_signature(desc.binding_signature);

        if !desc.vertex_layout.input_bindings.is_empty()
            || desc.vertex_layout.input_attributes.is_empty()
        {
            let v_desc = MTLVertexDescriptor::new();
            let v_attrs = v_desc.attributes();
            for attribute in desc.vertex_layout.input_attributes.iter() {
                unsafe {
                    let a_desc = MTLVertexAttributeDescriptor::new();
                    a_desc.setBufferIndex(attribute.binding as usize + 10);
                    a_desc.setOffset(attribute.offset as usize);
                    a_desc.setFormat(conv::vertex_format_to_mtl(attribute.format));
                    v_attrs
                        .setObject_atIndexedSubscript(Some(&a_desc), attribute.location as usize);
                }
            }

            let v_layouts = v_desc.layouts();
            for binding in desc.vertex_layout.input_bindings {
                unsafe {
                    let b_desc = MTLVertexBufferLayoutDescriptor::new();
                    b_desc.setStride(binding.stride as usize);
                    b_desc.setStepFunction(conv::vertex_input_rate_to_mtl(binding.input_rate));
                    b_desc.setStepRate(1);
                    v_layouts
                        .setObject_atIndexedSubscript(Some(&b_desc), binding.binding as usize + 10);
                }
            }

            mtl_desc.setVertexDescriptor(Some(&v_desc));
        }

        let primitive_topology = desc.input_assembly_state.primitive_topology;
        let primitive_type = conv::primitive_topology_to_mtl(primitive_topology);
        unsafe {
            let mtl_topology = conv::primitive_topology_to_mtl_class(primitive_topology);
            mtl_desc.setInputPrimitiveTopology(mtl_topology);
        }

        let cull_mode = conv::cull_mode_to_mtl(desc.rasterizer_state.cull_mode);
        let front_face = conv::front_face_order_to_mtl(desc.rasterizer_state.front_face);
        let polygon_mode = conv::polygon_mode_to_mtl(desc.rasterizer_state.polygon_mode);
        let depth_bias = desc.rasterizer_state.depth_bias;
        let depth_bias_clamp = desc.rasterizer_state.depth_bias_clamp;
        let depth_bias_slope_factor = desc.rasterizer_state.depth_bias_slope_factor;

        // TODO depth bounds
        let mtl_depth_desc = MTLDepthStencilDescriptor::new();

        // 'depth_test = false' just decays to 'always'
        let compare_fn = match (
            desc.depth_stencil_state.depth_test,
            desc.depth_stencil_state.depth_compare_op,
        ) {
            (true, v) => conv::compare_op_to_mtl(v),
            (false, _) => MTLCompareFunction::Always,
        };
        mtl_depth_desc.setDepthCompareFunction(compare_fn);
        mtl_depth_desc.setDepthWriteEnabled(desc.depth_stencil_state.depth_write);

        fn apply_stencil_op_state(to: &MTLStencilDescriptor, v: &StencilOpState) {
            to.setStencilFailureOperation(conv::stencil_op_to_mtl(v.fail_op));
            to.setDepthStencilPassOperation(conv::stencil_op_to_mtl(v.pass_op));
            to.setDepthFailureOperation(conv::stencil_op_to_mtl(v.depth_fail_op));
            to.setStencilCompareFunction(conv::compare_op_to_mtl(v.compare_op));
        }
        if desc.depth_stencil_state.stencil_test {
            let front = MTLStencilDescriptor::new();
            front.setReadMask(desc.depth_stencil_state.stencil_read_mask as u32);
            front.setWriteMask(desc.depth_stencil_state.stencil_write_mask as u32);
            apply_stencil_op_state(&front, &desc.depth_stencil_state.stencil_front);
            mtl_depth_desc.setFrontFaceStencil(Some(&front));

            let back = MTLStencilDescriptor::new();
            back.setReadMask(desc.depth_stencil_state.stencil_read_mask as u32);
            back.setWriteMask(desc.depth_stencil_state.stencil_write_mask as u32);
            apply_stencil_op_state(&back, &desc.depth_stencil_state.stencil_back);
            mtl_depth_desc.setBackFaceStencil(Some(&back));
        }

        let depth_stencil_state = device
            .device
            .newDepthStencilStateWithDescriptor(&mtl_depth_desc);
        let depth_stencil_state = match depth_stencil_state {
            Some(v) => v,
            None => {
                log::error!(
                    "Failed to create depth stencil state for pipeline! Reason: {}",
                    "unknown"
                );
                return Err(PipelineCreateError::Platform);
            }
        };

        let attachments = mtl_desc.colorAttachments();
        for (i, (format, blend)) in desc
            .render_target_formats
            .iter()
            .zip(desc.blend_state.attachments)
            .enumerate()
        {
            let mtl_attachment = MTLRenderPipelineColorAttachmentDescriptor::new();

            mtl_attachment.setPixelFormat(conv::format_to_pixel_mtl(*format));
            mtl_attachment.setWriteMask(conv::write_mask_to_mtl(blend.color_write_mask));

            if blend.blend_enabled {
                mtl_attachment.setBlendingEnabled(blend.blend_enabled);

                let v = conv::blend_factor_to_mtl(blend.src_factor);
                mtl_attachment.setSourceRGBBlendFactor(v);

                let v = conv::blend_factor_to_mtl(blend.dst_factor);
                mtl_attachment.setDestinationRGBBlendFactor(v);

                let v = conv::alpha_blend_factor_to_mtl(blend.alpha_src_factor);
                mtl_attachment.setSourceAlphaBlendFactor(v);

                let v = conv::alpha_blend_factor_to_mtl(blend.alpha_dst_factor);
                mtl_attachment.setDestinationAlphaBlendFactor(v);

                let v = conv::blend_op_to_mtl(blend.blend_op);
                mtl_attachment.setRgbBlendOperation(v);

                let v = conv::blend_op_to_mtl(blend.alpha_blend_op);
                mtl_attachment.setAlphaBlendOperation(v);
            }

            unsafe {
                attachments.setObject_atIndexedSubscript(Some(&mtl_attachment), i);
            }
        }

        if let Some(fmt) = desc.depth_stencil_format {
            let mtl_format = conv::format_to_pixel_mtl(fmt);
            mtl_desc.setDepthAttachmentPixelFormat(mtl_format);

            if fmt.is_stencil() {
                mtl_desc.setStencilAttachmentPixelFormat(mtl_format);
            }
        }

        if let Some(name) = desc.name
            && device.context.debug
        {
            let mtl_name = NSString::from_str(name);
            mtl_desc.setLabel(Some(&mtl_name));
        }

        if device.context.validation {
            mtl_desc.setShaderValidation(MTLShaderValidation::Enabled);
        }

        let pipeline = device
            .device
            .newRenderPipelineStateWithDescriptor_error(&mtl_desc);
        let pipeline = match pipeline {
            Ok(v) => v,
            Err(err) => {
                log::error!("Failed to create render pipeline state! Reason: {}", err);
                return Err(PipelineCreateError::Platform);
            }
        };

        let out = GraphicsPipeline {
            _device: device.this.upgrade().unwrap(),
            _binding_signature: binding_signature.this.upgrade().unwrap(),
            id: device.object_counter.next_graphics_pipeline(),
            objects: GraphicsPipelineObjects {
                pipeline,
                depth_stencil_state,
            },
            info: CachedGraphicsInfo {
                primitive_type,
                cull_mode,
                front_face,
                polygon_mode,
                depth_bias,
                depth_bias_clamp,
                depth_bias_slope_factor,
            },
        };
        let out = Object::new_arc_opaque(out);
        unsafe { Ok(GraphicsPipelineHandle::new(out)) }
    }

    pub(crate) fn get_owned(v: &GraphicsPipelineHandle) -> std::sync::Arc<Object<Self>> {
        v.clone()
            .into_inner()
            .downcast::<Self>()
            .expect("Unknown GraphicsPipeline implementation!")
    }

    pub(crate) fn get(v: &GraphicsPipelineHandle) -> &Self {
        v.get()
            .downcast_ref::<Self>()
            .expect("Unknown GraphicsPipeline implementation!")
    }
}

/// Wrapper type to limit the scope of our 'unsafe impl Send+Sync'
pub struct GraphicsPipelineObjects {
    pub pipeline: Retained<ProtocolObject<dyn MTLRenderPipelineState>>,
    pub depth_stencil_state: Retained<ProtocolObject<dyn MTLDepthStencilState>>,
}

/// Wrapper over all the pipeline data that is _not_ part of the MTLRenderPipelineState that the
/// RHI expects to be a part of the pipeline.
pub struct CachedGraphicsInfo {
    pub primitive_type: MTLPrimitiveType,

    pub cull_mode: MTLCullMode,
    pub front_face: MTLWinding,
    pub polygon_mode: MTLTriangleFillMode,
    pub depth_bias: i32, // If 0, depth bias is disabled
    pub depth_bias_clamp: f32,
    pub depth_bias_slope_factor: f32,
}

// Safety: Needed for 'MTLRenderPipelineState'
unsafe impl Send for GraphicsPipelineObjects {}
unsafe impl Sync for GraphicsPipelineObjects {}

pub struct ComputePipeline {
    pub(crate) _device: Arc<Device>,
    pub(crate) _binding_signature: Arc<BindingSignature>,
    pub(crate) id: NonZeroU64,
    pub(crate) objects: ComputePipelineObjects,
    pub(crate) workgroup_size: MTLSize,
}

unsafe_impl_iobject!(ComputePipeline, "01980753-5c4f-7ae3-be3b-9719259cfbc3");

impl ComputePipeline {
    pub(crate) fn create(
        device: &Device,
        desc: &ComputePipelineDesc,
    ) -> Result<ComputePipelineHandle, PipelineCreateError> {
        let mtl_desc = MTLComputePipelineDescriptor::new();

        let function = unsafe { compile_function(device, desc.shader_module)? };
        mtl_desc.setComputeFunction(Some(&function));

        let workgroup_size = desc.shader_module.get_compute_workgroup_size();
        let workgroup_size = MTLSize {
            width: workgroup_size.0 as usize,
            height: workgroup_size.1 as usize,
            depth: workgroup_size.2 as usize,
        };

        let binding_signature = unwrap::binding_signature(desc.binding_signature);

        if let Some(name) = desc.name
            && device.context.debug
        {
            let mtl_name = NSString::from_str(name);
            mtl_desc.setLabel(Some(&mtl_name));
        }

        if device.context.validation {
            mtl_desc.setShaderValidation(MTLShaderValidation::Enabled);
        }

        let pipeline = device
            .device
            .newComputePipelineStateWithDescriptor_options_reflection_error(
                &mtl_desc,
                MTLPipelineOption::empty(),
                None,
            );

        let pipeline = match pipeline {
            Ok(v) => v,
            Err(err) => {
                log::error!("Failed to create render pipeline state! Reason: {}", err,);
                return Err(PipelineCreateError::Platform);
            }
        };

        let out = ComputePipeline {
            _device: device.this.upgrade().unwrap(),
            _binding_signature: binding_signature.this.upgrade().unwrap(),
            id: device.object_counter.next_compute_pipeline(),
            objects: ComputePipelineObjects { pipeline },
            workgroup_size,
        };
        let out = Object::new_arc_opaque(out);
        unsafe { Ok(ComputePipelineHandle::new(out)) }
    }

    pub(crate) fn get_owned(v: &ComputePipelineHandle) -> std::sync::Arc<Object<Self>> {
        v.clone()
            .into_inner()
            .downcast::<Self>()
            .expect("Unknown ComputePipeline implementation!")
    }

    pub(crate) fn get(v: &ComputePipelineHandle) -> &Self {
        v.get()
            .downcast_ref::<Self>()
            .expect("Unknown ComputePipeline implementation!")
    }
}

/// Wrapper type to limit the scope of our 'unsafe impl Send+Sync'
pub struct ComputePipelineObjects {
    pub pipeline: Retained<ProtocolObject<dyn MTLComputePipelineState>>,
}

// Safety: Needed for 'MTLRenderPipelineState
unsafe impl Send for ComputePipelineObjects {}
unsafe impl Sync for ComputePipelineObjects {}

pub unsafe fn compile_function(
    device: &Device,
    code: &dyn IShaderCodeSource,
) -> Result<Retained<ProtocolObject<dyn MTLFunction>>, PipelineCreateError> {
    let source = str::from_utf8(code.get_msl())
        .inspect_err(|v| log::error!("{v}"))
        .map_err(|_| PipelineCreateError::InvalidShaderCode(code.shader_type()))?;
    let source = NSString::from_str(source);

    let mtl_options = MTLCompileOptions::new();

    mtl_options.setEnableLogging(false);
    mtl_options.setMathMode(MTLMathMode::Safe);
    mtl_options.setMathFloatingPointFunctions(MTLMathFloatingPointFunctions::Precise);
    mtl_options.setPreserveInvariance(true);
    mtl_options.setLanguageVersion(MTLLanguageVersion::Version3_2);
    mtl_options.setOptimizationLevel(MTLLibraryOptimizationLevel::Default);

    let library = device
        .device
        .newLibraryWithSource_options_error(&source, Some(&mtl_options))
        .inspect_err(|v| log::error!("{v}"))
        .map_err(|_| PipelineCreateError::Platform)?;

    let function = library
        .newFunctionWithName(ns_string!("main_0"))
        .ok_or_else(|| {
            log::error!("No 'main' function found in MTLLibrary");
            PipelineCreateError::Platform
        })?;

    Ok(function)
}
