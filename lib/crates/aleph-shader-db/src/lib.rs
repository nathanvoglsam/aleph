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

use rkyv::{Archive, Deserialize, Serialize};
use std::collections::HashMap;
use std::marker::PhantomData;

#[derive(
    Archive, Serialize, Deserialize, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug,
)]
pub enum ShaderType {
    Vertex,
    Fragment,
    Compute,
    Geometry,
}

pub trait ShaderStage {
    fn shader_type_matches(v: ShaderType) -> bool;
}

pub struct Vertex {}

impl ShaderStage for Vertex {
    fn shader_type_matches(v: ShaderType) -> bool {
        v == ShaderType::Vertex
    }
}

pub struct Fragment {}

impl ShaderStage for Fragment {
    fn shader_type_matches(v: ShaderType) -> bool {
        v == ShaderType::Fragment
    }
}

pub struct Compute {}

impl ShaderStage for Compute {
    fn shader_type_matches(v: ShaderType) -> bool {
        v == ShaderType::Compute
    }
}

pub struct Geometry {}

impl ShaderStage for Geometry {
    fn shader_type_matches(v: ShaderType) -> bool {
        v == ShaderType::Geometry
    }
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub struct ShaderName<'a, T: ShaderStage> {
    v: &'a str,
    _phantom: PhantomData<T>,
}

impl<'a, T: ShaderStage> ShaderName<'a, T> {
    pub unsafe fn new(v: &'a str) -> Self {
        Self {
            v,
            _phantom: PhantomData,
        }
    }
}

#[derive(Default, Archive, Serialize, Deserialize)]
pub struct ShaderDatabase {
    pub shaders: HashMap<String, ShaderEntry>,
}

#[derive(Archive, Serialize, Deserialize)]
pub struct ShaderEntry {
    pub shader_type: ShaderType,
    pub spirv: Vec<u8>,
    pub dxil: Vec<u8>,
}

pub struct ShaderEntryRef<'a> {
    pub shader_type: ShaderType,
    pub spirv: &'a [u8],
    pub dxil: &'a [u8],
}

pub trait IShaderDatabase {
    fn get_by_name(&self, name: &str) -> Option<ShaderEntryRef>;
}

pub trait IShaderDatabaseExt {
    fn get<T: ShaderStage>(&self, name: ShaderName<T>) -> Option<ShaderEntryRef>;
}

impl<T: IShaderDatabase + ?Sized> IShaderDatabaseExt for T {
    fn get<S: ShaderStage>(&self, name: ShaderName<S>) -> Option<ShaderEntryRef> {
        self.get_by_name(name.v)
            .map(|v| {
                if S::shader_type_matches(v.shader_type) {
                    Some(v)
                } else {
                    None
                }
            })
            .flatten()
    }
}

impl IShaderDatabase for ShaderDatabase {
    fn get_by_name(&self, name: &str) -> Option<ShaderEntryRef> {
        if let Some(v) = self.shaders.get(name) {
            Some(ShaderEntryRef {
                shader_type: v.shader_type,
                spirv: &v.spirv,
                dxil: &v.dxil,
            })
        } else {
            None
        }
    }
}

impl IShaderDatabase for ArchivedShaderDatabase {
    fn get_by_name(&self, name: &str) -> Option<ShaderEntryRef> {
        if let Some(v) = self.shaders.get(name) {
            Some(ShaderEntryRef {
                shader_type: v.shader_type.deserialize(&mut rkyv::Infallible).unwrap(),
                spirv: &v.spirv,
                dxil: &v.dxil,
            })
        } else {
            None
        }
    }
}
