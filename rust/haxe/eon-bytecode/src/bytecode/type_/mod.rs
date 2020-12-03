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

use crate::bytecode::indexes::{StringIndex, TypeIndex};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Hash, Serialize, Deserialize)]
pub enum Type {
    // Primitive types
    Void,
    UI8,
    UI16,
    I32,
    I64,
    F32,
    F64,
    Bool,

    Bytes,
    Dynamic,
    Array,
    Type,
    DynObject,
    Function(TypeFunction),
    Method(TypeFunction),
    Ref(TypeParam),
    Null(TypeParam),
    Obj(TypeObject),
    Struct(TypeObject),
    Virtual(TypeVirtual),
    Abstract(TypeAbstract),
    Enum(TypeEnum),
}

impl From<hashlink_bytecode::Type> for Type {
    fn from(v: hashlink_bytecode::Type) -> Self {
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
            hashlink_bytecode::Type::Function(v) => Type::Function(v.into()),
            hashlink_bytecode::Type::Method(v) => Type::Method(v.into()),
            hashlink_bytecode::Type::Ref(v) => Type::Ref(v.into()),
            hashlink_bytecode::Type::Null(v) => Type::Null(v.into()),
            hashlink_bytecode::Type::Obj(v) => Type::Obj(v.into()),
            hashlink_bytecode::Type::Struct(v) => Type::Struct(v.into()),
            hashlink_bytecode::Type::Virtual(v) => Type::Virtual(v.into()),
            hashlink_bytecode::Type::Abstract(v) => Type::Abstract(v.into()),
            hashlink_bytecode::Type::Enum(v) => Type::Enum(v.into()),
        }
    }
}

impl Type {
    pub fn get_type_function(&self) -> Option<&TypeFunction> {
        match self {
            Type::Function(v) => Some(v),
            Type::Method(v) => Some(v),
            _ => None,
        }
    }

    pub fn get_type_param(&self) -> Option<&TypeParam> {
        match self {
            Type::Ref(v) => Some(v),
            Type::Null(v) => Some(v),
            _ => None,
        }
    }

    pub fn get_type_object(&self) -> Option<&TypeObject> {
        match self {
            Type::Obj(v) => Some(v),
            Type::Struct(v) => Some(v),
            _ => None,
        }
    }

    pub fn get_type_virtual(&self) -> Option<&TypeVirtual> {
        match self {
            Type::Virtual(v) => Some(v),
            _ => None,
        }
    }

    pub fn get_type_abstract(&self) -> Option<&TypeAbstract> {
        match self {
            Type::Abstract(v) => Some(v),
            _ => None,
        }
    }

    pub fn get_type_enum(&self) -> Option<&TypeEnum> {
        match self {
            Type::Enum(v) => Some(v),
            _ => None,
        }
    }

    pub fn get_type_function_mut(&mut self) -> Option<&mut TypeFunction> {
        match self {
            Type::Function(v) => Some(v),
            Type::Method(v) => Some(v),
            _ => None,
        }
    }

    pub fn get_type_param_mut(&mut self) -> Option<&mut TypeParam> {
        match self {
            Type::Ref(v) => Some(v),
            Type::Null(v) => Some(v),
            _ => None,
        }
    }

    pub fn get_type_object_mut(&mut self) -> Option<&mut TypeObject> {
        match self {
            Type::Obj(v) => Some(v),
            Type::Struct(v) => Some(v),
            _ => None,
        }
    }

    pub fn get_type_virtual_mut(&mut self) -> Option<&mut TypeVirtual> {
        match self {
            Type::Virtual(v) => Some(v),
            _ => None,
        }
    }

    pub fn get_type_abstract_mut(&mut self) -> Option<&mut TypeAbstract> {
        match self {
            Type::Abstract(v) => Some(v),
            _ => None,
        }
    }

    pub fn get_type_enum_mut(&mut self) -> Option<&mut TypeEnum> {
        match self {
            Type::Enum(v) => Some(v),
            _ => None,
        }
    }
}

#[derive(Clone, Debug, Hash, Serialize, Deserialize)]
pub struct ObjectProto {
    /// Index into string table for the name
    pub name: StringIndex,

    /// ?
    pub f_index: usize,

    /// ?
    pub p_index: usize,
}

impl From<hashlink_bytecode::ObjectProto> for ObjectProto {
    fn from(v: hashlink_bytecode::ObjectProto) -> Self {
        ObjectProto {
            name: StringIndex(v.name as usize),
            f_index: v.f_index as usize,
            p_index: v.p_index as usize,
        }
    }
}

#[derive(Clone, Debug, Hash, Serialize, Deserialize)]
pub struct Field {
    /// Index into string table for the field name
    pub name: StringIndex,

    /// Index into type table for the type name
    pub type_: TypeIndex,
}

impl From<hashlink_bytecode::Field> for Field {
    fn from(v: hashlink_bytecode::Field) -> Self {
        Field {
            name: StringIndex(v.name as usize),
            type_: TypeIndex(v.type_ as usize),
        }
    }
}

#[derive(Clone, Debug, Hash, Serialize, Deserialize)]
pub struct EnumConstruct {
    /// Index into string table for the name
    pub name: StringIndex,

    /// List of indexes into the type table
    pub params: Vec<TypeIndex>,
}

impl From<hashlink_bytecode::EnumConstruct> for EnumConstruct {
    fn from(v: hashlink_bytecode::EnumConstruct) -> Self {
        EnumConstruct {
            name: StringIndex(v.name as usize),
            params: v
                .params
                .into_iter()
                .map(|v| TypeIndex(v as usize))
                .collect(),
        }
    }
}

#[derive(Clone, Debug, Hash, Serialize, Deserialize)]
pub struct TypeFunction {
    /// List of indexes into type table for the function arguments
    pub args: Vec<TypeIndex>,

    /// Index into the type table for the return type
    pub returns: TypeIndex,
}

impl From<hashlink_bytecode::TypeFunction> for TypeFunction {
    fn from(v: hashlink_bytecode::TypeFunction) -> Self {
        TypeFunction {
            args: v.args.into_iter().map(|v| TypeIndex(v as usize)).collect(),
            returns: TypeIndex(v.returns as usize),
        }
    }
}

#[derive(Clone, Debug, Hash, Serialize, Deserialize)]
pub struct TypeObject {
    /// Index into string table for the name
    pub name: StringIndex,

    /// List of fields on this object
    pub fields: Vec<Field>,

    /// ?
    pub protos: Vec<ObjectProto>,

    /// ?
    pub bindings: Vec<usize>,

    /// ?
    pub super_: Option<usize>,

    /// ?
    pub global: usize,
}

impl From<hashlink_bytecode::TypeObject> for TypeObject {
    fn from(v: hashlink_bytecode::TypeObject) -> Self {
        TypeObject {
            name: StringIndex(v.name as usize),
            fields: v.fields.into_iter().map(Field::from).collect(),
            protos: v.protos.into_iter().map(ObjectProto::from).collect(),
            bindings: v.bindings.into_iter().map(|v| v as usize).collect(),
            super_: v.super_.map(|v| v as usize),
            global: v.global as usize,
        }
    }
}

#[derive(Clone, Debug, Hash, Serialize, Deserialize)]
pub struct TypeEnum {
    /// Index into string table for the name
    pub name: StringIndex,

    /// ?
    pub constructs: Vec<EnumConstruct>,

    /// ?
    pub global: usize,
}

impl From<hashlink_bytecode::TypeEnum> for TypeEnum {
    fn from(v: hashlink_bytecode::TypeEnum) -> Self {
        TypeEnum {
            name: StringIndex(v.name as usize),
            constructs: v.constructs.into_iter().map(EnumConstruct::from).collect(),
            global: v.global as usize,
        }
    }
}

#[derive(Clone, Debug, Hash, Serialize, Deserialize)]
pub struct TypeVirtual {
    /// The list of fields on this virtual
    pub fields: Vec<Field>,
}

impl From<hashlink_bytecode::TypeVirtual> for TypeVirtual {
    fn from(v: hashlink_bytecode::TypeVirtual) -> Self {
        TypeVirtual {
            fields: v.fields.into_iter().map(Field::from).collect(),
        }
    }
}

#[derive(Clone, Debug, Hash, Serialize, Deserialize)]
pub struct TypeParam {
    /// Index into the type table
    pub type_: TypeIndex,
}

impl From<hashlink_bytecode::TypeParam> for TypeParam {
    fn from(v: hashlink_bytecode::TypeParam) -> Self {
        TypeParam {
            type_: TypeIndex(v.type_ as usize),
        }
    }
}

#[derive(Clone, Debug, Hash, Serialize, Deserialize)]
pub struct TypeAbstract {
    /// Index into the string table for the name
    pub name: StringIndex,
}

impl From<hashlink_bytecode::TypeAbstract> for TypeAbstract {
    fn from(v: hashlink_bytecode::TypeAbstract) -> Self {
        TypeAbstract {
            name: StringIndex(v.name as usize),
        }
    }
}
