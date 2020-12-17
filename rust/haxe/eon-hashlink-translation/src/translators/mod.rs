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

/// This translates a HashLink type into the matching Eon type definition. The return value may be
/// a potentially invalid type that needs further processing into the correct underlying type.
///
/// # Warning
///
/// As stated above, the type returned from this may be invalid. Currently this is only the case
/// when translating the following HashLink types:
///
/// - `hashlink_byteocde::Type::Array`
///
/// ## `Type::Array`
///
/// HashLink's bytecode erases the type of the elements stored in an array from the type system and
/// leaves it to be handled at runtime. This is, to be frank, fucking stupid. What's the point of an
/// array type that doesn't even have its element type in the type signature, it may as well just be
/// a raw pointer.
///
/// HashLink doesn't even take advantage of the flexibility this could provide, where the type of
/// the elements could be changed at runtime.
///
/// To remedy this we need to try and reconstruct this information and rebuild a sane type
/// definition for arrays. That algorithm will be explained elsewhere. The important information is
/// that `Type::Array` will be translated to `Type::Array(0)` in every case. We then need to pass
/// over the module and emit a new type for every Array -> Element Type pair and remap all uses of
/// the types accordingly.
///
pub fn translate_type(v: hashlink::Type) -> Type {
    match v {
        hashlink::Type::Void => Type::Void,
        hashlink::Type::UI8 => Type::U8,
        hashlink::Type::UI16 => Type::U16,
        hashlink::Type::I32 => Type::I32,
        hashlink::Type::I64 => Type::I64,
        hashlink::Type::F32 => Type::F32,
        hashlink::Type::F64 => Type::F64,
        hashlink::Type::Bool => Type::Bool,
        hashlink::Type::Bytes => Type::Bytes,
        hashlink::Type::Dynamic => Type::Dynamic,
        hashlink::Type::Array => Type::Array(TypeParam {
            type_: TypeIndex(0),
        }),
        hashlink::Type::Type => Type::Type,
        hashlink::Type::DynObject => Type::DynObject,
        hashlink::Type::Function(v) => Type::Function(translate_type_function(v)),
        hashlink::Type::Method(v) => Type::Method(translate_type_function(v)),
        hashlink::Type::Ref(v) => Type::Ref(translate_type_param(v)),
        hashlink::Type::Null(v) => Type::Null(translate_type_param(v)),
        hashlink::Type::Obj(v) => Type::Obj(translate_type_object(v)),
        hashlink::Type::Struct(v) => Type::Struct(translate_type_object(v)),
        hashlink::Type::Virtual(v) => Type::Virtual(translate_type_virtual(v)),
        hashlink::Type::Abstract(v) => Type::Abstract(translate_type_abstract(v)),
        hashlink::Type::Enum(v) => Type::Enum(translate_type_enum(v)),
    }
}

pub fn translate_native(v: hashlink::Native) -> Native {
    Native {
        lib: translate_string_index(v.lib),
        name: translate_string_index(v.name),
        type_: translate_type_index(v.type_),
        f_index: v.f_index as usize,
    }
}

pub fn translate_constant(v: hashlink::Constant) -> Constant {
    Constant {
        global: translate_global_index(v.global),
        fields: v.fields.into_iter().map(|v| v as usize).collect(),
    }
}

pub fn translate_object_proto(v: hashlink::ObjectProto) -> ObjectProto {
    ObjectProto {
        name: translate_string_index(v.name),
        f_index: v.f_index as usize,
        p_index: v.p_index as usize,
    }
}

pub fn translate_field(v: hashlink::Field) -> Field {
    Field {
        name: translate_string_index(v.name),
        type_: translate_type_index(v.type_),
    }
}

pub fn translate_enum_construct(v: hashlink::EnumConstruct) -> EnumConstruct {
    EnumConstruct {
        name: translate_string_index(v.name),
        params: v.params.into_iter().map(translate_type_index).collect(),
    }
}

pub fn translate_type_function(v: hashlink::TypeFunction) -> TypeFunction {
    TypeFunction {
        args: v.args.into_iter().map(translate_type_index).collect(),
        returns: translate_type_index(v.returns),
    }
}

pub fn translate_type_enum(v: hashlink::TypeEnum) -> TypeEnum {
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

pub fn translate_type_object(v: hashlink::TypeObject) -> TypeObject {
    TypeObject {
        name: translate_string_index(v.name),
        fields: v.fields.into_iter().map(translate_field).collect(),
        protos: v.protos.into_iter().map(translate_object_proto).collect(),
        bindings: v.bindings.into_iter().map(|v| v as usize).collect(),
        super_: v.super_.map(|v| v as usize),
        global: v.global as usize,
    }
}

pub fn translate_type_virtual(v: hashlink::TypeVirtual) -> TypeVirtual {
    TypeVirtual {
        fields: v.fields.into_iter().map(translate_field).collect(),
    }
}

pub fn translate_type_param(v: hashlink::TypeParam) -> TypeParam {
    TypeParam {
        type_: translate_type_index(v.type_),
    }
}

pub fn translate_type_abstract(v: hashlink::TypeAbstract) -> TypeAbstract {
    TypeAbstract {
        name: translate_string_index(v.name),
    }
}

pub fn translate_types(input: Vec<hashlink::Type>) -> Vec<Type> {
    input.into_iter().map(translate_type).collect()
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
