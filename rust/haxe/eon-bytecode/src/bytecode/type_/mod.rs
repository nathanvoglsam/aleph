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

#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Debug)]
pub enum TypeKind {
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
    Function,
    Obj,
    Array,
    Type,
    Ref,
    Virtual,
    DynObject,
    Abstract,
    Enum,
    Null,
    Method,
    Struct,
}

#[derive(Clone, Debug)]
pub struct Type {
    /// The type kind of this type
    pub kind: TypeKind,

    /// Extra data associated with some type variants
    pub variant: TypeVariant,
}

#[derive(Clone, Debug)]
pub enum TypeVariant {
    Function(TypeFunction),
    Object(TypeObject),
    Enum(TypeEnum),
    Virtual(TypeVirtual),
    TypeParam(TypeParam),
    Abstract(TypeAbstract),
    Other,
}

#[derive(Clone, Debug)]
pub struct ObjectProto {
    /// Index into string table for the name
    pub name: u32,

    /// ?
    pub f_index: u32,

    /// ?
    pub p_index: i32,
}

#[derive(Clone, Debug)]
pub struct Field {
    /// Index into string table for the field name
    pub name: u32,

    /// Index into type table for the type name
    pub type_: u32,
}

#[derive(Clone, Debug)]
pub struct EnumConstruct {
    /// Index into string table for the name
    pub name: u32,

    /// List of indexes into the type table
    pub params: Vec<u32>,
}

#[derive(Clone, Debug)]
pub struct TypeFunction {
    /// List of indexes into type table for the function arguments
    pub args: Vec<u32>,

    /// Index into the type table for the return type
    pub returns: u32,
}

#[derive(Clone, Debug)]
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

#[derive(Clone, Debug)]
pub struct TypeEnum {
    /// Index into string table for the name
    pub name: u32,

    /// ?
    pub constructs: Vec<EnumConstruct>,

    /// ?
    pub global: u32,
}

#[derive(Clone, Debug)]
pub struct TypeVirtual {
    /// The list of fields on this virtual
    pub fields: Vec<Field>,
}

#[derive(Clone, Debug)]
pub struct TypeParam {
    /// Index into the type table
    pub type_: u32,
}

#[derive(Clone, Debug)]
pub struct TypeAbstract {
    /// Index into the string table for the name
    pub name: u32,
}
