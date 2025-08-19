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

use crate::*;

#[derive(serde::Deserialize, serde::Serialize, Debug)]
#[serde(rename_all = "camelCase")]
#[serde(tag = "kind")]
pub enum TypeLayout<'a> {
    #[serde(rename_all = "camelCase")]
    SamplerState {},

    #[serde(rename_all = "camelCase")]
    Scalar { scalar_type: ScalarType },

    #[serde(rename_all = "camelCase")]
    Vector {
        #[serde(default)]
        element_count: u64,

        #[serde(borrow)]
        element_type: Box<Type<'a>>,
    },

    #[serde(rename_all = "camelCase")]
    Matrix {
        #[serde(default)]
        row_count: u64,

        #[serde(default)]
        column_count: u64,

        #[serde(borrow)]
        element_type: Box<Type<'a>>,
    },

    #[serde(rename_all = "camelCase")]
    Pointer {
        #[serde(borrow)]
        #[serde(default)]
        value_type: Cow<'a, str>,
    },

    #[serde(rename_all = "camelCase")]
    Array {
        #[serde(default)]
        element_count: u64,

        #[serde(borrow)]
        element_type: Box<TypeLayout<'a>>,

        #[serde(default)]
        uniform_stride: u64,
    },

    #[serde(rename_all = "camelCase")]
    Struct {
        #[serde(borrow)]
        name: Cow<'a, str>,

        #[serde(borrow)]
        #[serde(default)]
        fields: Vec<VariableLayout<'a>>,

        #[serde(borrow)]
        #[serde(default)]
        user_attribs: Vec<UserAttribute<'a>>,
    },

    #[serde(rename_all = "camelCase")]
    ConstantBuffer {
        #[serde(borrow)]
        element_type: Box<TypeLayout<'a>>,

        #[serde(borrow)]
        container_var_layout: VariableBindingInfo<'a>,

        #[serde(borrow)]
        element_var_layout: Box<VariableLayout<'a>>,
    },

    #[serde(rename_all = "camelCase")]
    ParameterBlock {
        #[serde(borrow)]
        element_type: Box<TypeLayout<'a>>,

        #[serde(borrow)]
        container_var_layout: VariableBindingInfo<'a>,

        #[serde(borrow)]
        element_var_layout: Box<VariableLayout<'a>>,
    },

    #[serde(rename_all = "camelCase")]
    TextureBuffer {
        #[serde(borrow)]
        element_type: Box<TypeLayout<'a>>,

        #[serde(borrow)]
        container_var_layout: VariableBindingInfo<'a>,

        #[serde(borrow)]
        element_var_layout: Box<VariableLayout<'a>>,
    },

    #[serde(rename_all = "camelCase")]
    ShaderStorageBuffer {
        #[serde(borrow)]
        element_type: Box<TypeLayout<'a>>,
    },

    #[serde(rename_all = "camelCase")]
    Resource {
        #[serde(borrow)]
        #[serde(flatten)]
        info: ResourceTypeBase<'a>,

        #[serde(borrow)]
        #[serde(default)]
        user_attribs: Vec<UserAttribute<'a>>,
    },
}

impl<'a> TypeLayout<'a> {
    pub fn normalize(&mut self) {
        match self {
            TypeLayout::ConstantBuffer {
                container_var_layout,
                element_var_layout,
                ..
            }
            | TypeLayout::ParameterBlock {
                container_var_layout,
                element_var_layout,
                ..
            }
            | TypeLayout::TextureBuffer {
                container_var_layout,
                element_var_layout,
                ..
            } => {
                container_var_layout.normalize();
                element_var_layout.normalize();
            }
            _ => {}
        }
    }
}

macro_rules! scalar_type_def {
    ($n: ident, $t: path) => {
        pub const $n: Self = Self::Scalar { scalar_type: $t };
    };
}

impl TypeLayout<'static> {
    scalar_type_def!(VOID, ScalarType::Void);
    scalar_type_def!(BOOL, ScalarType::Bool);
    scalar_type_def!(FLOAT16, ScalarType::Float16);
    scalar_type_def!(FLOAT32, ScalarType::Float32);
    scalar_type_def!(FLOAT64, ScalarType::Float64);
    scalar_type_def!(UINT8, ScalarType::Uint8);
    scalar_type_def!(INT8, ScalarType::Int8);
    scalar_type_def!(UINT16, ScalarType::Uint16);
    scalar_type_def!(INT16, ScalarType::Int16);
    scalar_type_def!(UINT32, ScalarType::Uint32);
    scalar_type_def!(INT32, ScalarType::Int32);
    scalar_type_def!(UINT64, ScalarType::Uint64);
    scalar_type_def!(INT64, ScalarType::Int64);
    scalar_type_def!(INTPTR, ScalarType::Intptr);
    scalar_type_def!(UINTPTR, ScalarType::Uintptr);
}

#[derive(serde::Deserialize, serde::Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct VariableLayout<'a> {
    #[serde(borrow)]
    pub name: Option<Cow<'a, str>>,

    #[serde(default)]
    pub shared: bool,

    #[serde(borrow)]
    #[serde(default)]
    pub user_attribs: Vec<UserAttribute<'a>>,

    #[serde(borrow)]
    #[serde(flatten)]
    pub binding_info: VariableBindingInfo<'a>,

    #[serde(borrow)]
    pub r#type: TypeLayout<'a>,
}

impl<'a> VariableLayout<'a> {
    pub fn normalize(&mut self) {
        self.binding_info.normalize();
        self.r#type.normalize();
    }
}
