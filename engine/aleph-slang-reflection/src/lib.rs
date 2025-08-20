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

mod common;
mod entry_point;
mod resource_type;
mod r#type;
mod type_layout;
mod type_parameter;
mod user_attribute;
mod variable_binding;

pub use common::{ResourceAccess, ScalarType, ShaderStage};
pub use entry_point::EntryPoint;
pub use resource_type::ResourceTypeBase;
pub use r#type::{Type, Variable};
pub use type_layout::{TypeLayout, VariableLayout};
pub use user_attribute::UserAttribute;
pub use variable_binding::{VariableBinding, VariableBindingInfo, VariableBindings};

use crate::type_parameter::TypeParameter;

#[derive(serde::Deserialize, serde::Serialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Root<'a> {
    #[serde(borrow)]
    #[serde(default)]
    pub parameters: Vec<VariableLayout<'a>>,

    #[serde(borrow)]
    #[serde(default)]
    pub entry_points: Vec<EntryPoint<'a>>,

    #[serde(borrow)]
    #[serde(default)]
    pub type_params: Vec<TypeParameter<'a>>,
}

impl<'a> Root<'a> {
    pub fn normalize(&mut self) {
        for parameter in self.parameters.iter_mut() {
            parameter.normalize();
        }

        for entry_point in self.entry_points.iter_mut() {
            entry_point.normalize();
        }

        for type_param in self.type_params.iter_mut() {
            type_param.normalize();
        }
    }
}
