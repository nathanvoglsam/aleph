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

use std::borrow::Cow;

use aleph_slang_reflection::{
    ResourceAccess, ResourceTypeBase, Root, TypeLayout, UserAttribute, VariableLayout,
};

use crate::shader_system::reflection::{CurrentContainer, Diagnostic, Node, TypeClass};

pub fn walk_parameter_tree<'a>(diagnostics: &mut Vec<Diagnostic<'a>>, root: &'a Root) -> Vec<Node> {
    let mut name_stack = Vec::new();
    let mut nodes = Vec::new();

    name_stack.push("__$global");
    walk_global_root(
        diagnostics,
        &mut name_stack,
        &mut nodes,
        "__ImplicitGlobal",
        &root.parameters,
    );
    name_stack.pop();

    for entry_point in root.entry_points.iter() {
        name_stack.push("__$entry");
        walk_global_root(
            diagnostics,
            &mut name_stack,
            &mut nodes,
            "__ImplicitEntry",
            &entry_point.parameters,
        );
        name_stack.pop();
    }

    nodes
}

fn walk_global_root<'a>(
    diagnostics: &mut Vec<Diagnostic<'a>>,
    name_stack: &mut Vec<&'a str>,
    nodes: &mut Vec<Node>,
    container_type_name: &'a str,
    fields: &'a [VariableLayout<'a>],
) {
    for field in fields {
        walk_global_root_field(diagnostics, name_stack, nodes, container_type_name, field);
    }
}

fn walk_global_root_field<'a>(
    diagnostics: &mut Vec<Diagnostic<'a>>,
    name_stack: &mut Vec<&'a str>,
    nodes: &mut Vec<Node>,
    container_type_name: &'a str,
    field: &'a VariableLayout<'a>,
) {
    let no_bindings = field.binding_info.bindings.is_empty();
    let is_semantic = no_bindings && field.binding_info.semantic.name.is_some();
    let is_varying_input = field.binding_info.bindings.iter().all(|v| match v {
        aleph_slang_reflection::VariableBinding::VaryingInput { .. } => true,
        aleph_slang_reflection::VariableBinding::VaryingOutput { .. } => true,
        _ => false,
    });
    // Ignore fields if they have no bindings (semantic inputs) or varying input/outputs. They
    // do not consume any binding space and aren't important for binding signature layout.
    if is_semantic || is_varying_input {
        return;
    }

    name_stack.push(field.name.as_deref().unwrap_or("<unnamed>"));
    match &field.r#type {
        TypeLayout::Struct { name, fields, .. } => {
            walk_global_root(diagnostics, name_stack, nodes, name, fields)
        }
        TypeLayout::ConstantBuffer {
            element_var_layout, ..
        } => {
            if field.user_attribs.contains(&PUSH_CONSTANT_ATTR) {
                // Get the size from the reflection data.
                let bytes = element_var_layout
                    .binding_info
                    .bindings
                    .iter()
                    .find_map(|v| match v {
                        aleph_slang_reflection::VariableBinding::Uniform { size, .. } => {
                            Some(*size)
                        }
                        _ => None,
                    })
                    .unwrap();
                nodes.push(Node::PushConstantBlock { bytes });
            } else {
                let diag = Diagnostic::BadField {
                    full_path: full_path_from_name_stack(name_stack),
                    struct_name: container_type_name,
                    class: TypeClass::PushConstantBlock,
                    container: CurrentContainer::Global,
                };
                diagnostics.push(diag);
            }

            walk_constant_buffer(
                diagnostics,
                name_stack,
                container_type_name,
                element_var_layout,
            );
        }
        TypeLayout::ParameterBlock {
            element_var_layout, ..
        } => {
            let child = walk_parameter_block(
                diagnostics,
                name_stack,
                container_type_name,
                element_var_layout,
            );
            nodes.push(child);
        }
        _ => {
            let class = TypeClass::for_type_layout(&field.r#type);
            let diag = Diagnostic::BadField {
                full_path: full_path_from_name_stack(name_stack),
                struct_name: container_type_name,
                class,
                container: CurrentContainer::Global,
            };
            diagnostics.push(diag);
        }
    }
    name_stack.pop();
}

fn walk_parameter_block<'a>(
    diagnostics: &mut Vec<Diagnostic<'a>>,
    name_stack: &mut Vec<&'a str>,
    container_type_name: &'a str,
    element: &'a VariableLayout<'a>,
) -> Node {
    let mut nodes = Vec::new();

    walk_parameter_block_field(
        diagnostics,
        name_stack,
        &mut nodes,
        container_type_name,
        element,
    );

    Node::ParameterBlock { children: nodes }
}

fn walk_parameter_block_field<'a>(
    diagnostics: &mut Vec<Diagnostic<'a>>,
    name_stack: &mut Vec<&'a str>,
    nodes: &mut Vec<Node>,
    container_type_name: &'a str,
    element: &'a VariableLayout<'a>,
) {
    match &element.r#type {
        TypeLayout::SamplerState {} => {
            nodes.push(Node::Parameter {
                parameter_type: aleph_shader_db::ParameterType::SamplerState,
                array_size: 0,
            });
        }
        TypeLayout::Array {
            element_count,
            element_type,
            ..
        } => {
            walk_parameter_block_array_field(
                diagnostics,
                name_stack,
                nodes,
                container_type_name,
                *element_count,
                element_type,
            );
        }
        TypeLayout::Struct { name, fields, .. } => {
            walk_parameter_block_struct(diagnostics, name_stack, nodes, name, fields)
        }
        TypeLayout::ConstantBuffer {
            element_var_layout, ..
        } => {
            if !element.user_attribs.contains(&PUSH_CONSTANT_ATTR) {
                nodes.push(Node::Parameter {
                    parameter_type: aleph_shader_db::ParameterType::ConstantBuffer,
                    array_size: 0,
                });
            } else {
                let diag = Diagnostic::BadField {
                    full_path: full_path_from_name_stack(name_stack),
                    struct_name: container_type_name,
                    class: TypeClass::PushConstantBlock,
                    container: CurrentContainer::ParameterBlock,
                };
                diagnostics.push(diag);
            }
            walk_constant_buffer(
                diagnostics,
                name_stack,
                container_type_name,
                element_var_layout,
            );
        }
        TypeLayout::ParameterBlock {
            element_var_layout, ..
        } => {
            let child = walk_parameter_block(
                diagnostics,
                name_stack,
                container_type_name,
                element_var_layout,
            );
            nodes.push(child);
        }
        TypeLayout::TextureBuffer { .. } => {
            let diag = Diagnostic::UnsupportedField {
                full_path: full_path_from_name_stack(name_stack),
                struct_name: container_type_name,
                field_type: "TextureBuffer",
            };
            diagnostics.push(diag);
        }
        TypeLayout::ShaderStorageBuffer { .. } => {
            let diag = Diagnostic::UnsupportedField {
                full_path: full_path_from_name_stack(name_stack),
                struct_name: container_type_name,
                field_type: "ShaderStorageBuffer",
            };
            diagnostics.push(diag);
        }
        TypeLayout::Resource { info, .. } => {
            nodes.push(Node::Parameter {
                parameter_type: parameter_type_from_resource_info(
                    diagnostics,
                    name_stack,
                    container_type_name,
                    info,
                ),
                array_size: 0,
            });
        }
        _ => {
            let class = TypeClass::for_type_layout(&element.r#type);
            let diag = Diagnostic::BadField {
                full_path: full_path_from_name_stack(name_stack),
                struct_name: container_type_name,
                class,
                container: CurrentContainer::ParameterBlock,
            };
            diagnostics.push(diag);
        }
    }
}

fn walk_parameter_block_array_field<'a>(
    diagnostics: &mut Vec<Diagnostic<'a>>,
    name_stack: &mut Vec<&'a str>,
    nodes: &mut Vec<Node>,
    container_type_name: &'a str,
    element_count: u64,
    element_type: &'a TypeLayout<'a>,
) {
    match element_type {
        TypeLayout::SamplerState {} => {
            let array_size = if element_count == 0 {
                u64::MAX
            } else {
                element_count
            };
            nodes.push(Node::Parameter {
                parameter_type: aleph_shader_db::ParameterType::SamplerState,
                array_size,
            });
        }
        TypeLayout::Array {
            element_count: inner_element_count,
            element_type,
            ..
        } => {
            let element_count = *inner_element_count * element_count;
            walk_parameter_block_array_field(
                diagnostics,
                name_stack,
                nodes,
                container_type_name,
                element_count,
                element_type,
            );
        }
        TypeLayout::Struct { name, fields, .. } => {
            for _ in 0..element_count {
                walk_parameter_block_struct(diagnostics, name_stack, nodes, name, fields);
            }
        }
        TypeLayout::ConstantBuffer { .. } => {
            let diag = Diagnostic::UnsupportedField {
                full_path: full_path_from_name_stack(name_stack),
                struct_name: container_type_name,
                field_type: "ConstantBuffer Array",
            };
            diagnostics.push(diag);
        }
        TypeLayout::ParameterBlock { .. } => {
            let diag = Diagnostic::UnsupportedField {
                full_path: full_path_from_name_stack(name_stack),
                struct_name: container_type_name,
                field_type: "ParameterBlockArray",
            };
            diagnostics.push(diag);
        }
        TypeLayout::TextureBuffer { .. } => {
            let diag = Diagnostic::UnsupportedField {
                full_path: full_path_from_name_stack(name_stack),
                struct_name: container_type_name,
                field_type: "TextureBuffer",
            };
            diagnostics.push(diag);
        }
        TypeLayout::ShaderStorageBuffer { .. } => {
            let diag = Diagnostic::UnsupportedField {
                full_path: full_path_from_name_stack(name_stack),
                struct_name: container_type_name,
                field_type: "ShaderStorageBuffer",
            };
            diagnostics.push(diag);
        }
        TypeLayout::Resource { info, .. } => {
            let parameter_type = parameter_type_from_resource_info(
                diagnostics,
                name_stack,
                container_type_name,
                info,
            );
            let array_size = if element_count == 0 {
                u64::MAX
            } else {
                element_count
            };
            nodes.push(Node::Parameter {
                parameter_type,
                array_size,
            });
        }
        _ => {
            let class = TypeClass::for_type_layout(element_type);
            let diag = Diagnostic::BadField {
                full_path: full_path_from_name_stack(name_stack),
                struct_name: container_type_name,
                class,
                container: CurrentContainer::ParameterBlock,
            };
            diagnostics.push(diag);
        }
    }
}

fn walk_parameter_block_struct<'a>(
    diagnostics: &mut Vec<Diagnostic<'a>>,
    name_stack: &mut Vec<&'a str>,
    nodes: &mut Vec<Node>,
    container_type_name: &'a str,
    fields: &'a [VariableLayout<'a>],
) {
    for field in fields {
        name_stack.push(field.name.as_deref().unwrap_or("<unnamed>"));
        walk_parameter_block_field(diagnostics, name_stack, nodes, container_type_name, field);
        name_stack.pop();
    }
}

fn walk_constant_buffer<'a>(
    diagnostics: &mut Vec<Diagnostic<'a>>,
    name_stack: &mut Vec<&'a str>,
    container_type_name: &'a str,
    element: &'a VariableLayout<'a>,
) {
    walk_constant_buffer_field_type(
        diagnostics,
        name_stack,
        container_type_name,
        &element.r#type,
    );
}

fn walk_constant_buffer_struct<'a>(
    diagnostics: &mut Vec<Diagnostic<'a>>,
    name_stack: &mut Vec<&'a str>,
    container_type_name: &'a str,
    fields: &'a [VariableLayout<'a>],
) {
    for field in fields {
        walk_constant_buffer_field(diagnostics, name_stack, container_type_name, field);
    }
}

fn walk_constant_buffer_field<'a>(
    diagnostics: &mut Vec<Diagnostic<'a>>,
    name_stack: &mut Vec<&'a str>,
    container_type_name: &'a str,
    field: &'a VariableLayout<'a>,
) {
    name_stack.push(field.name.as_deref().unwrap_or("<unnamed>"));
    walk_constant_buffer_field_type(diagnostics, name_stack, container_type_name, &field.r#type);
    name_stack.pop();
}

fn walk_constant_buffer_field_type<'a>(
    diagnostics: &mut Vec<Diagnostic<'a>>,
    name_stack: &mut Vec<&'a str>,
    container_type_name: &'a str,
    field_type: &'a TypeLayout<'a>,
) {
    match field_type {
        TypeLayout::Scalar { .. } => {}
        TypeLayout::Vector { .. } => {}
        TypeLayout::Matrix { .. } => {}
        TypeLayout::Pointer { .. } => {}
        TypeLayout::Array { element_type, .. } => {
            walk_constant_buffer_field_type(
                diagnostics,
                name_stack,
                container_type_name,
                element_type,
            );
        }
        TypeLayout::Struct { name, fields, .. } => {
            walk_constant_buffer_struct(diagnostics, name_stack, name, fields);
        }
        _ => {
            let class = TypeClass::for_type_layout(field_type);
            let diag = Diagnostic::BadField {
                full_path: full_path_from_name_stack(name_stack),
                struct_name: container_type_name,
                class,
                container: CurrentContainer::ConstantBuffer,
            };
            diagnostics.push(diag);
        }
    }
}

fn full_path_from_name_stack(name_stack: &[&str]) -> String {
    use std::fmt::Write;

    let mut full_path = String::new();

    let mut iter = name_stack.iter();

    let first = iter.next();
    if let Some(first) = first {
        write!(&mut full_path, "{first}").unwrap();
        while let Some(next) = iter.next() {
            write!(&mut full_path, ".{next}").unwrap();
        }
    }
    full_path
}

fn parameter_type_from_resource_info<'a>(
    diagnostics: &mut Vec<Diagnostic<'a>>,
    name_stack: &mut Vec<&'a str>,
    container_type_name: &'a str,
    info: &ResourceTypeBase,
) -> aleph_shader_db::ParameterType {
    match info {
        ResourceTypeBase::Texture1D {
            array,
            feedback,
            access,
            combined,
        } => {
            let mut base = aleph_shader_db::ParameterType::Texture1D;

            if *combined {
                push_unsupported_resource_error(
                    diagnostics,
                    name_stack,
                    container_type_name,
                    "Combined Image/Sampler",
                );
                return base;
            }
            if *feedback {
                push_unsupported_resource_error(
                    diagnostics,
                    name_stack,
                    container_type_name,
                    "Feedback",
                );
                return base;
            }

            if *array {
                base = parameter_type_to_array(base);
            }

            map_parameter_type_access(diagnostics, name_stack, container_type_name, access, base)
        }
        ResourceTypeBase::Texture2D {
            array,
            feedback,
            access,
            combined,
            multisample,
        } => {
            let mut base = if *multisample {
                aleph_shader_db::ParameterType::Texture2DMS
            } else {
                aleph_shader_db::ParameterType::Texture2D
            };

            if *combined {
                push_unsupported_resource_error(
                    diagnostics,
                    name_stack,
                    container_type_name,
                    "Combined Image/Sampler",
                );
                return base;
            }
            if *feedback {
                push_unsupported_resource_error(
                    diagnostics,
                    name_stack,
                    container_type_name,
                    "Feedback",
                );
                return base;
            }

            if *array {
                base = parameter_type_to_array(base);
            }

            map_parameter_type_access(diagnostics, name_stack, container_type_name, access, base)
        }
        ResourceTypeBase::Texture3D {
            array,
            feedback,
            access,
            combined,
        } => {
            let mut base = aleph_shader_db::ParameterType::Texture3D;

            if *combined {
                push_unsupported_resource_error(
                    diagnostics,
                    name_stack,
                    container_type_name,
                    "Combined Image/Sampler",
                );
                return base;
            }
            if *feedback {
                push_unsupported_resource_error(
                    diagnostics,
                    name_stack,
                    container_type_name,
                    "Feedback",
                );
                return base;
            }

            if *array {
                base = parameter_type_to_array(base);
            }

            map_parameter_type_access(diagnostics, name_stack, container_type_name, access, base)
        }
        ResourceTypeBase::TextureCube {
            array,
            feedback,
            access,
            combined,
        } => {
            let mut base = aleph_shader_db::ParameterType::TextureCube;

            if *combined {
                push_unsupported_resource_error(
                    diagnostics,
                    name_stack,
                    container_type_name,
                    "Combined Image/Sampler",
                );
                return base;
            }
            if *feedback {
                push_unsupported_resource_error(
                    diagnostics,
                    name_stack,
                    container_type_name,
                    "Feedback",
                );
                return base;
            }

            if *array {
                base = parameter_type_to_array(base);
            }

            map_parameter_type_access(diagnostics, name_stack, container_type_name, access, base)
        }
        ResourceTypeBase::TextureBuffer {
            array,
            feedback,
            access,
            combined,
        } => {
            let base = aleph_shader_db::ParameterType::Buffer;

            if *array {
                push_unsupported_resource_error(
                    diagnostics,
                    name_stack,
                    container_type_name,
                    "Buffer Array",
                );
                return base;
            }
            if *combined {
                push_unsupported_resource_error(
                    diagnostics,
                    name_stack,
                    container_type_name,
                    "Combined Image/Sampler",
                );
                return base;
            }
            if *feedback {
                push_unsupported_resource_error(
                    diagnostics,
                    name_stack,
                    container_type_name,
                    "Feedback",
                );
                return base;
            }

            map_parameter_type_access(diagnostics, name_stack, container_type_name, access, base)
        }
        ResourceTypeBase::StructuredBuffer {
            array,
            feedback,
            access,
            ..
        } => {
            let base = aleph_shader_db::ParameterType::ByteAddressBuffer;

            if *array {
                push_unsupported_resource_error(
                    diagnostics,
                    name_stack,
                    container_type_name,
                    "StructuredBuffer Array",
                );
                return base;
            }
            if *feedback {
                push_unsupported_resource_error(
                    diagnostics,
                    name_stack,
                    container_type_name,
                    "Feedback",
                );
                return base;
            }

            map_parameter_type_access(diagnostics, name_stack, container_type_name, access, base)
        }
        ResourceTypeBase::ByteAddressBuffer {
            array,
            feedback,
            access,
        } => {
            let base = aleph_shader_db::ParameterType::ByteAddressBuffer;

            if *array {
                push_unsupported_resource_error(
                    diagnostics,
                    name_stack,
                    container_type_name,
                    "ByteAddressBuffer Array",
                );
                return base;
            }
            if *feedback {
                push_unsupported_resource_error(
                    diagnostics,
                    name_stack,
                    container_type_name,
                    "Feedback",
                );
                return base;
            }

            map_parameter_type_access(diagnostics, name_stack, container_type_name, access, base)
        }
        ResourceTypeBase::AccelerationStructure {
            array,
            feedback,
            access,
        } => {
            let base = aleph_shader_db::ParameterType::AccelerationStructure;

            if *array {
                push_unsupported_resource_error(
                    diagnostics,
                    name_stack,
                    container_type_name,
                    "AccelerationStructure Array",
                );
                return base;
            }
            if *feedback {
                push_unsupported_resource_error(
                    diagnostics,
                    name_stack,
                    container_type_name,
                    "Feedback",
                );
                return base;
            }

            map_parameter_type_access(diagnostics, name_stack, container_type_name, access, base)
        }
    }
}

fn push_unsupported_resource_error<'a>(
    diagnostics: &mut Vec<Diagnostic<'a>>,
    name_stack: &mut Vec<&'a str>,
    container_type_name: &'a str,
    field_type: &'static str,
) {
    diagnostics.push(Diagnostic::UnsupportedResource {
        full_path: full_path_from_name_stack(name_stack),
        struct_name: container_type_name,
        field_type,
    });
}

fn push_unsupported_resource_access_error<'a>(
    diagnostics: &mut Vec<Diagnostic<'a>>,
    name_stack: &mut Vec<&'a str>,
    container_type_name: &'a str,
    access: &aleph_slang_reflection::ResourceAccess,
) {
    match access {
        ResourceAccess::RasterOrdered => push_unsupported_resource_error(
            diagnostics,
            name_stack,
            container_type_name,
            "RasterOrdererView",
        ),
        ResourceAccess::Append => push_unsupported_resource_error(
            diagnostics,
            name_stack,
            container_type_name,
            "Append Buffer",
        ),
        ResourceAccess::Consume => push_unsupported_resource_error(
            diagnostics,
            name_stack,
            container_type_name,
            "Consume Buffer",
        ),
        ResourceAccess::Feedback => push_unsupported_resource_error(
            diagnostics,
            name_stack,
            container_type_name,
            "Feedback",
        ),
        _ => {}
    }
}

fn map_parameter_type_access<'a>(
    diagnostics: &mut Vec<Diagnostic<'a>>,
    name_stack: &mut Vec<&'a str>,
    container_type_name: &'a str,
    access: &aleph_slang_reflection::ResourceAccess,
    base: aleph_shader_db::ParameterType,
) -> aleph_shader_db::ParameterType {
    match access {
        ResourceAccess::Read => {}
        ResourceAccess::Write => return parameter_type_to_rw(base),
        ResourceAccess::ReadWrite => return parameter_type_to_rw(base),
        v @ _ => {
            push_unsupported_resource_access_error(diagnostics, name_stack, container_type_name, v)
        }
    }
    base
}

fn parameter_type_to_rw(v: aleph_shader_db::ParameterType) -> aleph_shader_db::ParameterType {
    match v {
        aleph_shader_db::ParameterType::StructuredBuffer => {
            aleph_shader_db::ParameterType::RWStructuredBuffer
        }
        aleph_shader_db::ParameterType::ByteAddressBuffer => {
            aleph_shader_db::ParameterType::RWByteAddressBuffer
        }
        aleph_shader_db::ParameterType::Buffer => aleph_shader_db::ParameterType::RWBuffer,
        aleph_shader_db::ParameterType::Texture1D => aleph_shader_db::ParameterType::RWTexture1D,
        aleph_shader_db::ParameterType::Texture2D => aleph_shader_db::ParameterType::RWTexture2D,
        aleph_shader_db::ParameterType::Texture3D => aleph_shader_db::ParameterType::RWTexture3D,
        aleph_shader_db::ParameterType::Texture1DArray => {
            aleph_shader_db::ParameterType::RWTexture1DArray
        }
        aleph_shader_db::ParameterType::Texture2DArray => {
            aleph_shader_db::ParameterType::RWTexture2DArray
        }
        aleph_shader_db::ParameterType::Texture3DArray => {
            aleph_shader_db::ParameterType::RWTexture3DArray
        }
        aleph_shader_db::ParameterType::Texture2DMS => {
            aleph_shader_db::ParameterType::RWTexture2DMS
        }
        aleph_shader_db::ParameterType::Texture2DMSArray => {
            aleph_shader_db::ParameterType::RWTexture2DMSArray
        }
        v @ _ => v,
    }
}

fn parameter_type_to_array(v: aleph_shader_db::ParameterType) -> aleph_shader_db::ParameterType {
    match v {
        aleph_shader_db::ParameterType::Texture1D => aleph_shader_db::ParameterType::Texture1DArray,
        aleph_shader_db::ParameterType::RWTexture1D => {
            aleph_shader_db::ParameterType::RWTexture1DArray
        }
        aleph_shader_db::ParameterType::Texture2D => aleph_shader_db::ParameterType::Texture2DArray,
        aleph_shader_db::ParameterType::RWTexture2D => {
            aleph_shader_db::ParameterType::RWTexture2DArray
        }
        aleph_shader_db::ParameterType::Texture3D => aleph_shader_db::ParameterType::Texture3DArray,
        aleph_shader_db::ParameterType::RWTexture3D => {
            aleph_shader_db::ParameterType::RWTexture3DArray
        }
        aleph_shader_db::ParameterType::Texture2DMS => {
            aleph_shader_db::ParameterType::Texture2DMSArray
        }
        aleph_shader_db::ParameterType::RWTexture2DMS => {
            aleph_shader_db::ParameterType::RWTexture2DMSArray
        }
        aleph_shader_db::ParameterType::TextureCube => {
            aleph_shader_db::ParameterType::TextureCubeArray
        }
        v @ _ => v,
    }
}

const PUSH_CONSTANT_ATTR: UserAttribute = UserAttribute {
    name: Cow::Borrowed("aleph_PushConstantBlock"),
};
