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

use eon_bytecode::constant::Constant;
use eon_bytecode::indexes::{GlobalIndex, StringIndex, TypeIndex, ValueIndex};
use eon_bytecode::native::Native;
use eon_bytecode::type_::{
    EnumConstruct, Field, ObjectProto, Type, TypeAbstract, TypeEnum, TypeFunction, TypeObject,
    TypeParam, TypeVirtual,
};

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

pub fn translate_type(v: hashlink_bytecode::Type) -> Type {
    match v {
        hashlink_bytecode::Type::Void => Type::Void,
        hashlink_bytecode::Type::UI8 => Type::UI8,
        hashlink_bytecode::Type::UI16 => Type::UI16,
        hashlink_bytecode::Type::I32 => Type::I32,
        hashlink_bytecode::Type::I64 => Type::I64,
        hashlink_bytecode::Type::F32 => Type::F32,
        hashlink_bytecode::Type::F64 => Type::F64,
        hashlink_bytecode::Type::Bool => Type::Bool,
        hashlink_bytecode::Type::Bytes => Type::Bytes,
        hashlink_bytecode::Type::Dynamic => Type::Dynamic,
        hashlink_bytecode::Type::Array => Type::Array,
        hashlink_bytecode::Type::Type => Type::Type,
        hashlink_bytecode::Type::DynObject => Type::DynObject,
        hashlink_bytecode::Type::Function(v) => Type::Function(translate_type_function(v)),
        hashlink_bytecode::Type::Method(v) => Type::Method(translate_type_function(v)),
        hashlink_bytecode::Type::Ref(v) => Type::Ref(translate_type_param(v)),
        hashlink_bytecode::Type::Null(v) => Type::Null(translate_type_param(v)),
        hashlink_bytecode::Type::Obj(v) => Type::Obj(translate_type_object(v)),
        hashlink_bytecode::Type::Struct(v) => Type::Struct(translate_type_object(v)),
        hashlink_bytecode::Type::Virtual(v) => Type::Virtual(translate_type_virtual(v)),
        hashlink_bytecode::Type::Abstract(v) => Type::Abstract(translate_type_abstract(v)),
        hashlink_bytecode::Type::Enum(v) => Type::Enum(translate_type_enum(v)),
    }
}

pub fn translate_native(v: hashlink_bytecode::Native) -> Native {
    Native {
        lib: translate_string_index(v.lib),
        name: translate_string_index(v.name),
        type_: translate_type_index(v.type_),
        f_index: v.f_index as usize,
    }
}

pub fn translate_constant(v: hashlink_bytecode::Constant) -> Constant {
    Constant {
        global: translate_global_index(v.global),
        fields: v.fields.into_iter().map(|v| v as usize).collect(),
    }
}

pub fn translate_object_proto(v: hashlink_bytecode::ObjectProto) -> ObjectProto {
    ObjectProto {
        name: translate_string_index(v.name),
        f_index: v.f_index as usize,
        p_index: v.p_index as usize,
    }
}

pub fn translate_field(v: hashlink_bytecode::Field) -> Field {
    Field {
        name: translate_string_index(v.name),
        type_: translate_type_index(v.type_),
    }
}

pub fn translate_enum_construct(v: hashlink_bytecode::EnumConstruct) -> EnumConstruct {
    EnumConstruct {
        name: translate_string_index(v.name),
        params: v.params.into_iter().map(translate_type_index).collect(),
    }
}

pub fn translate_type_function(v: hashlink_bytecode::TypeFunction) -> TypeFunction {
    TypeFunction {
        args: v.args.into_iter().map(translate_type_index).collect(),
        returns: translate_type_index(v.returns),
    }
}

pub fn translate_type_enum(v: hashlink_bytecode::TypeEnum) -> TypeEnum {
    TypeEnum {
        name: translate_string_index(v.name),
        constructs: v
            .constructs
            .into_iter()
            .map(translate_enum_construct)
            .collect(),
        global: v.global as usize,
    }
}

pub fn translate_type_object(v: hashlink_bytecode::TypeObject) -> TypeObject {
    TypeObject {
        name: translate_string_index(v.name),
        fields: v.fields.into_iter().map(translate_field).collect(),
        protos: v.protos.into_iter().map(translate_object_proto).collect(),
        bindings: v.bindings.into_iter().map(|v| v as usize).collect(),
        super_: v.super_.map(|v| v as usize),
        global: v.global as usize,
    }
}

pub fn translate_type_virtual(v: hashlink_bytecode::TypeVirtual) -> TypeVirtual {
    TypeVirtual {
        fields: v.fields.into_iter().map(translate_field).collect(),
    }
}

pub fn translate_type_param(v: hashlink_bytecode::TypeParam) -> TypeParam {
    TypeParam {
        type_: translate_type_index(v.type_),
    }
}

pub fn translate_type_abstract(v: hashlink_bytecode::TypeAbstract) -> TypeAbstract {
    TypeAbstract {
        name: translate_string_index(v.name),
    }
}

pub fn translate_types(input: Vec<hashlink_bytecode::Type>) -> Vec<Type> {
    input.into_iter().map(translate_type).collect()
}

pub fn translate_natives(input: Vec<hashlink_bytecode::Native>) -> Vec<Native> {
    input.into_iter().map(translate_native).collect()
}

pub fn translate_globals(input: Vec<i32>) -> Vec<TypeIndex> {
    input.into_iter().map(translate_type_index).collect()
}

pub fn translate_constants(input: Vec<hashlink_bytecode::Constant>) -> Vec<Constant> {
    input.into_iter().map(translate_constant).collect()
}
