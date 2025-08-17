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
pub enum Type<'a> {
    #[serde(rename_all = "camelCase")]
    SamplerState {
        #[serde(borrow)]
        #[serde(default)]
        user_attribs: Vec<UserAttribute<'a>>,
    },

    #[serde(rename_all = "camelCase")]
    Resource {
        #[serde(borrow)]
        #[serde(flatten)]
        info: ResourceTypeBase<'a>,

        #[serde(borrow)]
        result_type: Option<Box<Type<'a>>>,

        #[serde(borrow)]
        #[serde(default)]
        user_attribs: Vec<UserAttribute<'a>>,
    },

    #[serde(rename_all = "camelCase")]
    ConstantBuffer {
        #[serde(borrow)]
        element_type: Box<Type<'a>>,

        #[serde(borrow)]
        #[serde(default)]
        user_attribs: Vec<UserAttribute<'a>>,
    },

    #[serde(rename_all = "camelCase")]
    ParameterBlock {
        #[serde(borrow)]
        element_type: Box<Type<'a>>,

        #[serde(borrow)]
        #[serde(default)]
        user_attribs: Vec<UserAttribute<'a>>,
    },

    #[serde(rename_all = "camelCase")]
    TextureBuffer {
        #[serde(borrow)]
        element_type: Box<Type<'a>>,

        #[serde(borrow)]
        #[serde(default)]
        user_attribs: Vec<UserAttribute<'a>>,
    },

    #[serde(rename_all = "camelCase")]
    ShaderStorageBuffer {
        #[serde(borrow)]
        element_type: Box<Type<'a>>,

        #[serde(borrow)]
        #[serde(default)]
        user_attribs: Vec<UserAttribute<'a>>,
    },

    #[serde(rename_all = "camelCase")]
    Scalar {
        scalar_type: ScalarType,

        #[serde(borrow)]
        #[serde(default)]
        user_attribs: Vec<UserAttribute<'a>>,
    },

    #[serde(rename_all = "camelCase")]
    Vector {
        #[serde(default)]
        element_count: u64,

        #[serde(borrow)]
        element_type: Box<Type<'a>>,

        #[serde(borrow)]
        #[serde(default)]
        user_attribs: Vec<UserAttribute<'a>>,
    },

    #[serde(rename_all = "camelCase")]
    Matrix {
        #[serde(default)]
        row_count: u64,

        #[serde(default)]
        column_count: u64,

        #[serde(borrow)]
        element_type: Box<Type<'a>>,

        #[serde(borrow)]
        #[serde(default)]
        user_attribs: Vec<UserAttribute<'a>>,
    },

    #[serde(rename_all = "camelCase")]
    Array {
        #[serde(default)]
        element_count: u64,

        #[serde(borrow)]
        element_type: Box<Type<'a>>,

        #[serde(borrow)]
        #[serde(default)]
        user_attribs: Vec<UserAttribute<'a>>,
    },

    #[serde(rename_all = "camelCase")]
    Pointer {
        #[serde(borrow)]
        target_type: Box<Type<'a>>,

        #[serde(borrow)]
        #[serde(default)]
        user_attribs: Vec<UserAttribute<'a>>,
    },

    #[serde(rename_all = "camelCase")]
    Struct {
        #[serde(borrow)]
        #[serde(default)]
        fields: Vec<Variable<'a>>,

        #[serde(borrow)]
        #[serde(default)]
        user_attribs: Vec<UserAttribute<'a>>,
    },
}

macro_rules! scalar_type_def {
    ($n: ident, $t: path) => {
        pub const $n: Self = Self::Scalar {
            scalar_type: $t,
            user_attribs: vec![],
        };
    };
}

impl Type<'static> {
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
pub struct Variable<'a> {
    #[serde(borrow)]
    pub name: Cow<'a, str>,

    #[serde(default)]
    pub shared: bool,

    #[serde(borrow)]
    #[serde(default)]
    pub user_attribs: Vec<UserAttribute<'a>>,

    #[serde(borrow)]
    pub r#type: Type<'a>,
}
