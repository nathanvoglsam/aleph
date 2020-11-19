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

#[derive(Clone, Debug)]
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

#[derive(Clone, Debug)]
pub struct ObjectProto {
    /// Index into string table for the name
    pub name: usize,

    /// ?
    pub f_index: usize,

    /// ?
    pub p_index: usize,
}

impl From<hashlink_bytecode::ObjectProto> for ObjectProto {
    fn from(v: hashlink_bytecode::ObjectProto) -> Self {
        ObjectProto {
            name: v.name as usize,
            f_index: v.f_index as usize,
            p_index: v.p_index as usize,
        }
    }
}

#[derive(Clone, Debug)]
pub struct Field {
    /// Index into string table for the field name
    pub name: usize,

    /// Index into type table for the type name
    pub type_: usize,
}

impl From<hashlink_bytecode::Field> for Field {
    fn from(v: hashlink_bytecode::Field) -> Self {
        Field {
            name: v.name as usize,
            type_: v.type_ as usize,
        }
    }
}

#[derive(Clone, Debug)]
pub struct EnumConstruct {
    /// Index into string table for the name
    pub name: usize,

    /// List of indexes into the type table
    pub params: Vec<usize>,
}

impl From<hashlink_bytecode::EnumConstruct> for EnumConstruct {
    fn from(mut v: hashlink_bytecode::EnumConstruct) -> Self {
        EnumConstruct {
            name: v.name as usize,
            params: v.params.drain(..).map(|v| v as usize).collect(),
        }
    }
}

#[derive(Clone, Debug)]
pub struct TypeFunction {
    /// List of indexes into type table for the function arguments
    pub args: Vec<usize>,

    /// Index into the type table for the return type
    pub returns: usize,
}

impl From<hashlink_bytecode::TypeFunction> for TypeFunction {
    fn from(mut v: hashlink_bytecode::TypeFunction) -> Self {
        TypeFunction {
            args: v.args.drain(..).map(|v| v as usize).collect(),
            returns: v.returns as usize,
        }
    }
}

#[derive(Clone, Debug)]
pub struct TypeObject {
    /// Index into string table for the name
    pub name: usize,

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
    fn from(mut v: hashlink_bytecode::TypeObject) -> Self {
        TypeObject {
            name: v.name as usize,
            fields: v.fields.drain(..).map(Field::from).collect(),
            protos: v.protos.drain(..).map(ObjectProto::from).collect(),
            bindings: v.bindings.drain(..).map(|v| v as usize).collect(),
            super_: v.super_.map(|v| v as usize),
            global: v.global as usize,
        }
    }
}

#[derive(Clone, Debug)]
pub struct TypeEnum {
    /// Index into string table for the name
    pub name: usize,

    /// ?
    pub constructs: Vec<EnumConstruct>,

    /// ?
    pub global: usize,
}

impl From<hashlink_bytecode::TypeEnum> for TypeEnum {
    fn from(mut v: hashlink_bytecode::TypeEnum) -> Self {
        TypeEnum {
            name: v.name as usize,
            constructs: v.constructs.drain(..).map(EnumConstruct::from).collect(),
            global: v.global as usize,
        }
    }
}

#[derive(Clone, Debug)]
pub struct TypeVirtual {
    /// The list of fields on this virtual
    pub fields: Vec<Field>,
}

impl From<hashlink_bytecode::TypeVirtual> for TypeVirtual {
    fn from(mut v: hashlink_bytecode::TypeVirtual) -> Self {
        TypeVirtual {
            fields: v.fields.drain(..).map(Field::from).collect(),
        }
    }
}

#[derive(Clone, Debug)]
pub struct TypeParam {
    /// Index into the type table
    pub type_: usize,
}

impl From<hashlink_bytecode::TypeParam> for TypeParam {
    fn from(v: hashlink_bytecode::TypeParam) -> Self {
        TypeParam {
            type_: v.type_ as usize,
        }
    }
}

#[derive(Clone, Debug)]
pub struct TypeAbstract {
    /// Index into the string table for the name
    pub name: usize,
}

impl From<hashlink_bytecode::TypeAbstract> for TypeAbstract {
    fn from(v: hashlink_bytecode::TypeAbstract) -> Self {
        TypeAbstract {
            name: v.name as usize,
        }
    }
}
