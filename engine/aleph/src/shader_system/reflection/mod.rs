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

use aleph_shader_db::ParameterBlockDesc;
use aleph_slang_reflection::{Root, TypeLayout};

mod phase1;
mod phase2;
mod phase3;

pub fn build_shader_db_reflection<'a>(
    root: &'a Root,
) -> Option<(Vec<ParameterBlockDesc>, Option<u64>)> {
    let (diagnostics, mut nodes) = phase1::walk_parameter_tree(root);

    for diag in diagnostics.iter() {
        match diag {
            Diagnostic::BadField {
                full_path,
                struct_name,
                class,
                container,
            } => log::error!(
                "Field '{full_path}' in type '{struct_name}' of class '{class:?}' is not allowed inside '{container:?}'."
            ),
            Diagnostic::UnsupportedField {
                full_path,
                struct_name,
                field_type,
            } => log::error!(
                "Field '{full_path}' in type '{struct_name}' of is of unsupported type '{field_type}'."
            ),
            Diagnostic::UnsupportedResource {
                full_path,
                struct_name,
                field_type,
            } => log::error!(
                "Field '{full_path}' in type '{struct_name}' of is of unsupported resoruce type '{field_type}'."
            ),
        }
    }
    if !diagnostics.is_empty() {
        return None;
    }

    let push_constant_block = phase2::find_push_constant_block(&mut nodes);
    let descs = phase3::flatten_parameter_tree(&nodes);
    Some((descs, push_constant_block))
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub enum CurrentContainer {
    ConstantBuffer,
    ParameterBlock,
    Global,
}

/// Represents the different classes of type for the purposes of our reflection processing.
/// Different classes of type are only allowed to appear in certain parts of the parameter tree
/// of a shder module and this type defines the different classes for each.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub enum TypeClass {
    /// Types of this class are descriptor type resources. They take a descriptor/parameter slot and
    /// are the only allowed class of type in parameter blocks (by our rules)
    Descriptor,

    /// A parameter block is a special, distinct class of type similar to a struct. However
    /// parameter blocks have unique behavior w.r.t. introducing new register/spaces descriptor set
    /// indices.
    ParameterBlock,

    /// A special kind of type, typically a specially attributed ConstantBuffer. It maps to the
    /// underlying APIs closest concept to 'push constants'. The binding behavior of these is
    /// special.
    PushConstantBlock,

    /// This class of type encompases types which are structural or organizational and do not
    /// consume data or descriptor space on their own. For example: structs and arrays.
    Structure,

    /// This class of type encompases the most basic types: scalars, vectors, matrices and any other
    /// type that consumes space in a constant buffer. These are only allowed inside buffer
    /// descriptors and should only be placed inside a buffer descriptor of some kind.
    Data,
}

impl TypeClass {
    pub fn for_type_layout(v: &TypeLayout) -> Self {
        match v {
            TypeLayout::SamplerState { .. } => Self::Descriptor,
            TypeLayout::Resource { .. } => Self::Descriptor,
            TypeLayout::ConstantBuffer { .. } => Self::Descriptor,
            TypeLayout::ParameterBlock { .. } => Self::ParameterBlock,
            TypeLayout::TextureBuffer { .. } => Self::Descriptor,
            TypeLayout::ShaderStorageBuffer { .. } => Self::Descriptor,
            TypeLayout::Scalar { .. } => Self::Data,
            TypeLayout::Vector { .. } => Self::Data,
            TypeLayout::Matrix { .. } => Self::Data,
            TypeLayout::Array { .. } => Self::Structure,
            TypeLayout::Pointer { .. } => Self::Data,
            TypeLayout::Struct { .. } => Self::Structure,
        }
    }
}

pub enum Diagnostic<'a> {
    BadField {
        full_path: String,
        struct_name: &'a str,
        class: TypeClass,
        container: CurrentContainer,
    },
    UnsupportedField {
        full_path: String,
        struct_name: &'a str,
        field_type: &'static str,
    },
    UnsupportedResource {
        full_path: String,
        struct_name: &'a str,
        field_type: &'static str,
    },
}

#[derive(Debug)]
pub enum Node {
    ParameterBlock {
        children: Vec<Node>,
    },
    Parameter {
        parameter_type: aleph_shader_db::ParameterType,
        array_size: u32,
    },
    PushConstantBlock {
        bytes: u64,
    },
}
