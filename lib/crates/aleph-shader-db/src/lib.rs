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

use std::collections::HashMap;
use std::marker::PhantomData;

use rkyv::{Archive, Deserialize, Serialize};

#[derive(
    Archive, Serialize, Deserialize, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug,
)]
pub enum ShaderType {
    Compute,
    Vertex,
    Hull,
    Domain,
    Geometry,
    Fragment,
    Amplification,
    Mesh,
}

pub trait ShaderStage: Copy + Clone {
    const SHADER_TYPE: ShaderType;
}

macro_rules! shader_stage_variant {
    ($v_name: ident) => {
        #[derive(Copy, Clone)]
        pub struct $v_name {}

        impl ShaderStage for $v_name {
            const SHADER_TYPE: ShaderType = ShaderType::$v_name;
        }
    };
}

shader_stage_variant!(Compute);
shader_stage_variant!(Vertex);
shader_stage_variant!(Hull);
shader_stage_variant!(Domain);
shader_stage_variant!(Geometry);
shader_stage_variant!(Fragment);
shader_stage_variant!(Amplification);
shader_stage_variant!(Mesh);

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub struct ShaderName<'a, T> {
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

impl<'a, T> Into<&'a str> for ShaderName<'a, T> {
    fn into(self) -> &'a str {
        self.v
    }
}

#[derive(Archive, Serialize, Deserialize)]
pub struct ShaderDatabase {
    pub magic_number: u64,
    pub format_version: u64,
    pub shaders: HashMap<String, ShaderEntry>,
}

impl ShaderDatabase {
    const MAGIC_NUMBER: u64 = 569420;
    const EXPECTED_VERSION: u64 = 1;

    pub fn is_header_valid(&self) -> bool {
        self.magic_number == Self::MAGIC_NUMBER && self.format_version == Self::EXPECTED_VERSION
    }

    pub fn validate_header(&self) {
        assert!(
            self.is_header_valid(),
            "Invalid header detected, incompatible shader db"
        );
    }
}

impl ArchivedShaderDatabase {
    pub fn is_header_valid(&self) -> bool {
        self.magic_number == ShaderDatabase::MAGIC_NUMBER
            && self.format_version == ShaderDatabase::EXPECTED_VERSION
    }

    pub fn validate_header(&self) {
        assert!(
            self.is_header_valid(),
            "Invalid header detected, incompatible shader db"
        );
    }
}

impl Default for ShaderDatabase {
    fn default() -> Self {
        Self {
            magic_number: 569420,
            shaders: Default::default(),
            format_version: Self::EXPECTED_VERSION,
        }
    }
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
                if S::SHADER_TYPE == v.shader_type {
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
