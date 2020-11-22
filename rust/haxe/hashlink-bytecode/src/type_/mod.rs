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

use serde::{Deserialize, Serialize};

#[repr(i32)]
#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Debug, Serialize, Deserialize)]
pub enum TypeKind {
    Void = 0,
    UI8 = 1,
    UI16 = 2,
    I32 = 3,
    I64 = 4,
    F32 = 5,
    F64 = 6,
    Bool = 7,
    Bytes = 8,
    Dynamic = 9,
    Function = 10,
    Obj = 11,
    Array = 12,
    Type = 13,
    Ref = 14,
    Virtual = 15,
    DynObject = 16,
    Abstract = 17,
    Enum = 18,
    Null = 19,
    Method = 20,
    Struct = 21,
}

impl TypeKind {
    pub fn from_raw(raw: i32) -> Option<TypeKind> {
        match raw {
            0 => Some(TypeKind::Void),
            1 => Some(TypeKind::UI8),
            2 => Some(TypeKind::UI16),
            3 => Some(TypeKind::I32),
            4 => Some(TypeKind::I64),
            5 => Some(TypeKind::F32),
            6 => Some(TypeKind::F64),
            7 => Some(TypeKind::Bool),
            8 => Some(TypeKind::Bytes),
            9 => Some(TypeKind::Dynamic),
            10 => Some(TypeKind::Function),
            11 => Some(TypeKind::Obj),
            12 => Some(TypeKind::Array),
            13 => Some(TypeKind::Type),
            14 => Some(TypeKind::Ref),
            15 => Some(TypeKind::Virtual),
            16 => Some(TypeKind::DynObject),
            17 => Some(TypeKind::Abstract),
            18 => Some(TypeKind::Enum),
            19 => Some(TypeKind::Null),
            20 => Some(TypeKind::Method),
            21 => Some(TypeKind::Struct),
            _ => None,
        }
    }

    pub fn to_raw(&self) -> i32 {
        *self as i32
    }
}

#[derive(Clone, Serialize, Deserialize)]
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

impl Type {
    pub fn new(kind: TypeKind, variant: TypeVariant) -> Option<Self> {
        match kind {
            TypeKind::Void => Some(Type::Void),
            TypeKind::UI8 => Some(Type::UI8),
            TypeKind::UI16 => Some(Type::UI16),
            TypeKind::I32 => Some(Type::I32),
            TypeKind::I64 => Some(Type::I64),
            TypeKind::F32 => Some(Type::F32),
            TypeKind::F64 => Some(Type::F64),
            TypeKind::Bool => Some(Type::Bool),
            TypeKind::Bytes => Some(Type::Bytes),
            TypeKind::Dynamic => Some(Type::Dynamic),
            TypeKind::Function => {
                if let TypeVariant::Function(f) = variant {
                    Some(Type::Function(f))
                } else {
                    None
                }
            }
            TypeKind::Obj => {
                if let TypeVariant::Object(f) = variant {
                    Some(Type::Obj(f))
                } else {
                    None
                }
            }
            TypeKind::Array => Some(Type::Array),
            TypeKind::Type => Some(Type::Type),
            TypeKind::Ref => {
                if let TypeVariant::TypeParam(f) = variant {
                    Some(Type::Ref(f))
                } else {
                    None
                }
            }
            TypeKind::Virtual => {
                if let TypeVariant::Virtual(f) = variant {
                    Some(Type::Virtual(f))
                } else {
                    None
                }
            }
            TypeKind::DynObject => Some(Type::DynObject),
            TypeKind::Abstract => {
                if let TypeVariant::Abstract(f) = variant {
                    Some(Type::Abstract(f))
                } else {
                    None
                }
            }
            TypeKind::Enum => {
                if let TypeVariant::Enum(f) = variant {
                    Some(Type::Enum(f))
                } else {
                    None
                }
            }
            TypeKind::Null => {
                if let TypeVariant::TypeParam(f) = variant {
                    Some(Type::Null(f))
                } else {
                    None
                }
            }
            TypeKind::Method => {
                if let TypeVariant::Function(f) = variant {
                    Some(Type::Method(f))
                } else {
                    None
                }
            }
            TypeKind::Struct => {
                if let TypeVariant::Object(f) = variant {
                    Some(Type::Struct(f))
                } else {
                    None
                }
            }
        }
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub enum TypeVariant {
    Function(TypeFunction),
    Object(TypeObject),
    Enum(TypeEnum),
    Virtual(TypeVirtual),
    TypeParam(TypeParam),
    Abstract(TypeAbstract),
    Other,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct ObjectProto {
    /// Index into string table for the name
    pub name: u32,

    /// ?
    pub f_index: u32,

    /// ?
    pub p_index: i32,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct Field {
    /// Index into string table for the field name
    pub name: u32,

    /// Index into type table for the type name
    pub type_: u32,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct EnumConstruct {
    /// Index into string table for the name
    pub name: u32,

    /// List of indexes into the type table
    pub params: Vec<u32>,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct TypeFunction {
    /// List of indexes into type table for the function arguments
    pub args: Vec<u32>,

    /// Index into the type table for the return type
    pub returns: u32,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct TypeObject {
    /// Index into string table for the name
    pub name: u32,

    /// List of fields on this object
    pub fields: Vec<Field>,

    /// ?
    pub protos: Vec<ObjectProto>,

    /// ?
    pub bindings: Vec<u32>,

    /// ?
    pub super_: Option<u32>,

    /// ?
    pub global: u32,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct TypeEnum {
    /// Index into string table for the name
    pub name: u32,

    /// ?
    pub constructs: Vec<EnumConstruct>,

    /// ?
    pub global: u32,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct TypeVirtual {
    /// The list of fields on this virtual
    pub fields: Vec<Field>,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct TypeParam {
    /// Index into the type table
    pub type_: u32,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct TypeAbstract {
    /// Index into the string table for the name
    pub name: u32,
}
