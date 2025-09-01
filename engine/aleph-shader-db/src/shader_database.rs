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

use aleph_rhi_api::IShaderCodeSource;

use crate::{ArchivedShaderEntry, ShaderEntry};

pub trait IShaderDatabase {
    type Entry: IShaderCodeSource;
    fn get_by_name(&self, name: &str) -> Option<&Self::Entry>;
}

#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize)]
pub struct ShaderDatabase {
    pub magic_number: u64,
    pub format_version: u64,
    pub shaders: HashMap<String, ShaderEntry>,
}

impl ShaderDatabase {
    const MAGIC_NUMBER: u64 = 569420;
    const EXPECTED_VERSION: u64 = 1;

    pub const fn is_header_valid(&self) -> bool {
        self.magic_number == Self::MAGIC_NUMBER && self.format_version == Self::EXPECTED_VERSION
    }

    pub const fn validate_header(&self) {
        assert!(
            self.is_header_valid(),
            "Invalid header detected, incompatible shader db"
        );
    }
}

impl ArchivedShaderDatabase {
    pub const fn is_header_valid(&self) -> bool {
        self.magic_number.to_native() == ShaderDatabase::MAGIC_NUMBER
            && self.format_version.to_native() == ShaderDatabase::EXPECTED_VERSION
    }

    pub const fn validate_header(&self) {
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

impl IShaderDatabase for ArchivedShaderDatabase {
    type Entry = ArchivedShaderEntry;
    fn get_by_name(&self, name: &str) -> Option<&Self::Entry> {
        self.shaders.get(name)
    }
}

impl IShaderDatabase for ShaderDatabase {
    type Entry = ShaderEntry;
    fn get_by_name(&self, name: &str) -> Option<&Self::Entry> {
        self.shaders.get(name)
    }
}
