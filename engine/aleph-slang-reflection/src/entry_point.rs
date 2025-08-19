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

#[derive(serde::Deserialize, serde::Serialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct EntryPoint<'a> {
    #[serde(borrow)]
    pub name: Cow<'a, str>,

    #[serde(default)]
    pub stage: ShaderStage,

    #[serde(borrow)]
    #[serde(default)]
    pub user_attribs: Vec<UserAttribute<'a>>,

    #[serde(borrow)]
    #[serde(default)]
    pub parameters: Vec<VariableLayout<'a>>,

    #[serde(default)]
    pub uses_any_sample_rate_input: bool,

    #[serde(borrow)]
    pub result: Option<VariableLayout<'a>>,

    pub thread_group_size: Option<[u32; 3]>,

    #[serde(borrow)]
    #[serde(default)]
    pub bindings: Vec<EntryPointParam<'a>>,
}

impl<'a> EntryPoint<'a> {
    pub fn normalize(&mut self) {
        for binding in self.bindings.iter_mut() {
            binding.normalize();
        }
    }
}

#[derive(serde::Deserialize, serde::Serialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct EntryPointParam<'a> {
    #[serde(borrow)]
    pub name: Cow<'a, str>,

    #[serde(borrow)]
    #[serde(flatten)]
    pub binding: VariableBindingInfo<'a>,
}

impl<'a> EntryPointParam<'a> {
    pub fn normalize(&mut self) {
        self.binding.normalize();
    }
}
