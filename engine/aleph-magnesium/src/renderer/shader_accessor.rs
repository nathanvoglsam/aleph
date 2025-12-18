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

pub trait IShaderAccessor {
    fn get_stage_by_name(&self, name: &str) -> Option<&dyn rhi::IShaderCodeSource>;
}

#[derive(Clone)]
pub struct ShaderAccessor<'a, T: sdb::IShaderDatabase> {
    db: &'a T,
}

impl<'a, T: sdb::IShaderDatabase> ShaderAccessor<'a, T> {
    pub fn new(db: &'a T) -> Self {
        Self { db }
    }
}

impl<'a, T: sdb::IShaderDatabase> IShaderAccessor for ShaderAccessor<'a, T> {
    fn get_stage_by_name(&self, name: &str) -> Option<&dyn rhi::IShaderCodeSource> {
        let out = self.db.get_by_name(name)?;
        Some(out)
    }
}

pub trait IShaderAccessorExt: IShaderAccessor {
    fn load_stage<'a, S: sdb::ShaderStage>(
        &'a self,
        name: sdb::ShaderName<'_, S>,
    ) -> Option<&'a dyn rhi::IShaderCodeSource>;
}

impl<T: IShaderAccessor + ?Sized> IShaderAccessorExt for T {
    fn load_stage<'a, S: sdb::ShaderStage>(
        &'a self,
        name: sdb::ShaderName<'_, S>,
    ) -> Option<&'a dyn rhi::IShaderCodeSource> {
        let loaded = self.get_stage_by_name(name.into())?;
        if S::API_SHADER_TYPE == loaded.shader_type() {
            Some(loaded)
        } else {
            None
        }
    }
}
