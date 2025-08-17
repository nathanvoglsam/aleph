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
use std::ops::Deref;

use crate::*;

#[derive(serde::Deserialize, serde::Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct VariableBindingInfo<'a> {
    #[serde(default)]
    pub stage: ShaderStage,

    #[serde(flatten)]
    pub bindings: VariableBindings,

    #[serde(borrow)]
    #[serde(flatten)]
    pub semantic: Semantic<'a>,

    #[serde(borrow)]
    #[serde(default)]
    pub format: Option<Cow<'a, str>>,
}

impl<'a> VariableBindingInfo<'a> {
    pub fn normalize(&mut self) {
        self.bindings.normalize();
    }
}

#[derive(serde::Deserialize, serde::Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct VariableBindings {
    #[serde(default)]
    pub binding: Option<VariableBinding>,

    #[serde(default)]
    pub bindings: Vec<VariableBinding>,
}

impl VariableBindings {
    pub fn normalize(&mut self) {
        // The format is a bit painful and will write out a single binding into the 'binding'
        // field if there is only a single one. If there are multiple bindings it will instead write
        // a 'bindings' field as an array.
        //
        // This code normalizes the strange special case encoding by moving the binding from the
        // singular field into the empty array.
        let has_binding = self.binding.is_some();
        let has_bindings = !self.bindings.is_empty();
        match (has_binding, has_bindings) {
            (true, false) => {
                let binding = self.binding.take().unwrap();
                self.bindings.push(binding);
            }
            (false, true) => {}
            (true, true) => panic!(),
            (false, false) => {}
        }
    }
}

impl Deref for VariableBindings {
    type Target = [VariableBinding];

    fn deref(&self) -> &Self::Target {
        self.bindings.as_ref()
    }
}

#[derive(serde::Deserialize, serde::Serialize, Debug)]
#[serde(rename_all = "camelCase")]
#[serde(tag = "kind")]
pub enum VariableBinding {
    #[serde(rename_all = "camelCase")]
    Uniform {
        #[serde(default)]
        offset: u64,

        #[serde(default)]
        size: u64,

        #[serde(default)]
        element_stride: u64,

        #[serde(default)]
        #[serde(with = "used_serde")]
        used: Option<bool>,
    },

    #[serde(rename_all = "camelCase")]
    RegisterSpace {
        #[serde(default)]
        space: u64,

        #[serde(default)]
        index: u64,

        #[serde(default)]
        count: u64,

        #[serde(default)]
        #[serde(with = "used_serde")]
        used: Option<bool>,
    },

    #[serde(rename_all = "camelCase")]
    ConstantBuffer {
        #[serde(default)]
        index: u64,

        #[serde(default)]
        count: u64,

        #[serde(default)]
        #[serde(with = "used_serde")]
        used: Option<bool>,
    },

    #[serde(rename_all = "camelCase")]
    ShaderResource {
        #[serde(default)]
        index: u64,

        #[serde(default)]
        count: u64,

        #[serde(default)]
        #[serde(with = "used_serde")]
        used: Option<bool>,
    },

    #[serde(rename_all = "camelCase")]
    UnorderedAccess {
        #[serde(default)]
        index: u64,

        #[serde(default)]
        count: u64,

        #[serde(default)]
        #[serde(with = "used_serde")]
        used: Option<bool>,
    },

    #[serde(rename_all = "camelCase")]
    VaryingInput {
        #[serde(default)]
        index: u64,

        #[serde(default)]
        count: u64,

        #[serde(default)]
        #[serde(with = "used_serde")]
        used: Option<bool>,
    },

    #[serde(rename_all = "camelCase")]
    VaryingOutput {
        #[serde(default)]
        index: u64,

        #[serde(default)]
        count: u64,

        #[serde(default)]
        #[serde(with = "used_serde")]
        used: Option<bool>,
    },

    #[serde(rename_all = "camelCase")]
    SamplerState {
        #[serde(default)]
        index: u64,

        #[serde(default)]
        count: u64,

        #[serde(default)]
        #[serde(with = "used_serde")]
        used: Option<bool>,
    },

    #[serde(rename_all = "camelCase")]
    PushConstantBuffer {
        #[serde(default)]
        index: u64,

        #[serde(default)]
        count: u64,

        #[serde(default)]
        #[serde(with = "used_serde")]
        used: Option<bool>,
    },

    #[serde(rename_all = "camelCase")]
    DescriptorTableSlot {
        #[serde(default)]
        index: u64,

        #[serde(default)]
        count: u64,

        #[serde(default)]
        #[serde(with = "used_serde")]
        used: Option<bool>,
    },

    #[serde(rename_all = "camelCase")]
    SpecializationConstant {
        #[serde(default)]
        index: u64,

        #[serde(default)]
        count: u64,

        #[serde(default)]
        #[serde(with = "used_serde")]
        used: Option<bool>,
    },

    #[serde(rename_all = "camelCase")]
    Mixed {
        #[serde(default)]
        index: u64,

        #[serde(default)]
        count: u64,

        #[serde(default)]
        #[serde(with = "used_serde")]
        used: Option<bool>,
    },

    #[serde(rename_all = "camelCase")]
    SubElementRegisterSpace {
        #[serde(default)]
        index: u64,

        #[serde(default)]
        count: u64,

        #[serde(default)]
        #[serde(with = "used_serde")]
        used: Option<bool>,
    },

    #[serde(rename_all = "camelCase")]
    Generic {
        #[serde(default)]
        index: u64,

        #[serde(default)]
        count: u64,

        #[serde(with = "used_serde")]
        used: Option<bool>,
    },

    #[serde(rename_all = "camelCase")]
    MetalArgumentBufferElement {
        #[serde(default)]
        index: u64,

        #[serde(default)]
        count: u64,

        #[serde(default)]
        #[serde(with = "used_serde")]
        used: Option<bool>,
    },
}

mod used_serde {
    use serde::{Deserialize, Serialize};

    pub fn serialize<S>(v: &Option<bool>, s: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let v = v.map(|v| if v { 1 } else { 0 });
        v.serialize(s)
    }

    pub fn deserialize<'de, D>(d: D) -> Result<Option<bool>, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let v: Option<u64> = Deserialize::deserialize(d)?;
        let v = v.map(|v| if v != 0 { true } else { false });
        Ok(v)
    }
}

#[derive(serde::Deserialize, serde::Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Semantic<'a> {
    #[serde(borrow)]
    #[serde(rename = "semanticName")]
    pub name: Option<Cow<'a, str>>,

    #[serde(default)]
    #[serde(rename = "semanticIndex")]
    pub index: usize,
}
