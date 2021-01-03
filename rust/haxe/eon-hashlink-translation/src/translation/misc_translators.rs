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

use eon::{Constant, GlobalIndex, Native, StringIndex, TypeIndex, ValueIndex};

pub fn translate_native(v: hashlink::Native) -> Native {
    Native {
        lib: translate_string_index(v.lib),
        name: translate_string_index(v.name),
        type_: translate_type_index(v.type_),
    }
}

pub fn translate_constant(v: hashlink::Constant) -> Constant {
    Constant {
        global: translate_global_index(v.global),
        fields: v.fields.into_iter().map(|v| v as usize).collect(),
    }
}

pub fn translate_global_index(v: i32) -> GlobalIndex {
    GlobalIndex(v as usize)
}

pub fn translate_string_index(v: i32) -> StringIndex {
    StringIndex(v as usize)
}

pub fn translate_type_index(v: i32) -> TypeIndex {
    TypeIndex(v as usize)
}

pub fn translate_value_index(v: i32) -> ValueIndex {
    ValueIndex(v as usize)
}

pub fn translate_natives(input: Vec<hashlink::Native>) -> Vec<Native> {
    input.into_iter().map(translate_native).collect()
}

pub fn translate_globals(input: Vec<i32>) -> Vec<TypeIndex> {
    input.into_iter().map(translate_type_index).collect()
}

pub fn translate_constants(input: Vec<hashlink::Constant>) -> Vec<Constant> {
    input.into_iter().map(translate_constant).collect()
}
