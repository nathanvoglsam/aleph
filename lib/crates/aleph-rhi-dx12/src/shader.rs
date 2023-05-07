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

use crate::device::Device;
use aleph_any::{declare_interfaces, AnyArc, AnyWeak};
use aleph_rhi_api::*;
use std::any::TypeId;

pub struct Shader {
    pub(crate) this: AnyWeak<Self>,
    pub(crate) _device: AnyArc<Device>,
    pub(crate) shader_type: ShaderType,
    pub(crate) data: Vec<u8>,
    pub(crate) entry_point: String,
}

declare_interfaces!(Shader, [IShader]);

impl IGetPlatformInterface for Shader {
    unsafe fn __query_platform_interface(&self, target: TypeId, out: *mut ()) -> Option<()> {
        if target == TypeId::of::<ShaderData>() {
            let out = out as *mut ShaderData;
            out.write(ShaderData(self.data.clone()));

            Some(())
        } else {
            None
        }
    }
}

impl IShader for Shader {
    fn upgrade(&self) -> AnyArc<dyn IShader> {
        AnyArc::map::<dyn IShader, _>(self.this.upgrade().unwrap(), |v| v)
    }

    fn strong_count(&self) -> usize {
        self.this.strong_count()
    }

    fn weak_count(&self) -> usize {
        self.this.weak_count()
    }

    fn shader_type(&self) -> ShaderType {
        self.shader_type
    }

    fn entry_point(&self) -> &str {
        &self.entry_point
    }
}

/// A new-type wrapper over a `Vec<u8>` to provide a new type-id for use with the
/// [IGetPlatformInterface] interface.
pub struct ShaderData(pub Vec<u8>);
