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

use interfaces::any::{declare_interfaces, AnyArc, AnyWeak};
use interfaces::gpu::{INamedObject, IShader, ShaderType};

pub struct Shader {
    pub(crate) this: AnyWeak<Self>,
    pub(crate) shader_type: ShaderType,
    pub(crate) data: Vec<u8>,
    pub(crate) entry_point: String,
}

declare_interfaces!(Shader, [IShader, IShaderExt]);

impl IShader for Shader {
    fn upgrade(&self) -> AnyArc<dyn IShader> {
        self.this.upgrade().unwrap().query_interface().unwrap()
    }

    fn shader_type(&self) -> ShaderType {
        self.shader_type
    }

    fn entry_point(&self) -> &str {
        &self.entry_point
    }
}

pub trait IShaderExt: IShader {
    fn get_raw_buffer(&self) -> &[u8];
}

impl IShaderExt for Shader {
    fn get_raw_buffer(&self) -> &[u8] {
        &self.data
    }
}

impl INamedObject for Shader {
    fn set_name(&self, _name: &str) {
        // Nothing to do on d3d12, d3d12 doesn't have an object to represent shader blobs
    }
}
