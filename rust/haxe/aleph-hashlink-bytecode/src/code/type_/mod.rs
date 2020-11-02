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

use crate::code::Code;

#[repr(i32)]
#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Debug)]
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

pub struct Type {
    /// The type kind of this type
    pub kind: TypeKind,

    /// Extra data associated with some type variants
    pub variant: TypeVariant,
}

pub enum TypeVariant {
    Function(Function),
    Object(Object),
    Enum(Enum),
    Virtual(Virtual),
    TypeParam(TypeParam),
    Abstract(Abstract),
    Other,
}

impl TypeVariant {
    pub fn to_string(&self, code: &Code) -> String {
        match self {
            TypeVariant::Function(f) => {
                let mut string = "(".to_string();
                for arg_idx in f.args.iter() {
                    let arg_idx = *arg_idx as usize;
                    let arg = &code.types[arg_idx];
                    if let Some(name) = arg.variant.name(code) {
                        std::fmt::write(&mut string, format_args!("{},", name)).unwrap();
                    } else if let TypeVariant::Other = &arg.variant {
                        std::fmt::write(&mut string, format_args!("{:?},", arg.kind)).unwrap();
                    } else {
                        std::fmt::write(&mut string, format_args!("{},", arg_idx)).unwrap();
                    }
                }
                let returns_idx = f.returns as usize;
                let returns = &code.types[returns_idx];
                if let Some(name) = returns.variant.name(code) {
                    std::fmt::write(&mut string, format_args!(") -> {})", name)).unwrap();
                } else if let TypeVariant::Other = &returns.variant {
                    std::fmt::write(&mut string, format_args!(") -> {:?})", returns.kind)).unwrap();
                } else {
                    std::fmt::write(&mut string, format_args!(") -> {})", returns_idx)).unwrap();
                }
                string
            }
            TypeVariant::Object(o) => "".to_string(),
            TypeVariant::Enum(e) => "".to_string(),
            TypeVariant::Virtual(v) => "".to_string(),
            TypeVariant::TypeParam(t) => "".to_string(),
            TypeVariant::Abstract(a) => "".to_string(),
            TypeVariant::Other => "".to_string(),
        }
    }

    pub fn name<'a, 'b>(&'a self, code: &'b Code) -> Option<&'b str> {
        match self {
            TypeVariant::Function(_) => None,
            TypeVariant::Object(o) => Some(&code.strings[o.name as usize]),
            TypeVariant::Enum(e) => Some(&code.strings[e.name as usize]),
            TypeVariant::Virtual(_) => None,
            TypeVariant::TypeParam(_) => None,
            TypeVariant::Abstract(a) => Some(&code.strings[a.name as usize]),
            TypeVariant::Other => None,
        }
    }
}

pub struct ObjectProto {
    /// Index into string table for the name
    pub name: u32,

    /// ?
    pub f_index: u32,

    /// ?
    pub p_index: i32,
}

pub struct Field {
    /// Index into string table for the field name
    pub name: u32,

    /// Index into type table for the type name
    pub type_: u32,
}

pub struct EnumConstruct {
    /// Index into string table for the name
    pub name: u32,

    /// List of indexes into the type table
    pub params: Vec<u32>,
}

pub struct Function {
    /// List of indexes into type table for the function arguments
    pub args: Vec<u32>,

    /// Index into the type table for the return type
    pub returns: u32,
}

pub struct Object {
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

pub struct Enum {
    /// Index into string table for the name
    pub name: u32,

    /// ?
    pub constructs: Vec<EnumConstruct>,

    /// ?
    pub global: u32,
}

pub struct Virtual {
    /// The list of fields on this virtual
    pub fields: Vec<Field>,
}

pub struct TypeParam {
    /// Index into the type table
    pub type_: u32,
}

pub struct Abstract {
    /// Index into the string table for the name
    pub name: u32,
}
