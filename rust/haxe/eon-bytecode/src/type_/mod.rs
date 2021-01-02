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

use crate::indexes::{StringIndex, TypeIndex};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Hash, PartialEq, Eq, Serialize, Deserialize)]
pub enum Type {
    /// Similar to void in C, represents a "nothing type". i.e a function that returns void returns
    /// no value.
    Void,

    /// 8-bit unsigned integer
    U8,

    /// 16-bit unsigned integer
    U16,

    /// 32-bit signed integer
    I32,

    /// 64-bit signed integer
    I64,

    /// 32-bit single-precision floating point
    F32,

    /// 64-bit double-precision floating point
    F64,

    /// An abstract boolean type with a platform defined size
    Bool,

    /// A plain, untyped, nullable pointer
    Bytes,

    /// Represents a boxed, dynamically typed object. This could also be thought of as an `Any`
    /// type.
    ///
    /// This is essentially a pointer to a dynamically allocated block of memory that that contains
    /// a type value pair. The type describes the runtime type of the value, and the value is the
    /// value itself.
    Dynamic,

    /// Currently I'm keeping the old HashLink like `Array` type variant around until I have a solid
    /// story for replacing it.
    ///
    /// This type represents an array of elements with a single type. The representation is very
    /// straight forward. It's just a pointer + element count pair.
    Array(TypeParam),

    /// This type represents a value that holds a type identifier
    Type,

    /// This represents a pointer to a dynamically typed object. This type also has the semantics of
    /// a pointer. `DynObject` is similar to an object but is closer to what JavaScript or Lua would
    /// call an object where all fields are dynamic to add and remove. This can be sort of thought
    /// of as a hash-table (as that's basically what JS objects are) mapping a name to a value. The
    /// value could be any type (primitive, an object, a closure)
    DynObject,

    /// Represents a static function paired with its signature
    Function(TypeFunction),

    /// Represents a member function for an object paired with its signature
    Method(TypeFunction),

    /// This type wraps another underlying type to apply reference semantics to it. This would
    /// primarily have the use case of passing references to value types through function arguments.
    ///
    /// A value of this type would have the semantics of a pointer
    Ref(TypeParam),

    /// `Null` wraps an underlying type to make that underlying type capable of being nullable, if
    /// it otherwise wouldn't be.
    ///
    /// The semantics of the type *DO NOT CHANGE* when wrapped by this, other than the value now
    /// being able to represent `null`. An integer will remain a value type, will continue to be
    /// passed by copying into functions, etc.
    ///
    /// The primary use case for this is optional values, like optional members or optional function
    /// arguments.
    Null(TypeParam),

    /// Represents a regular object, paired with its definition. A value of this type has the
    /// semantics of a pointer
    Obj(TypeObject),

    /// Represents an anonymous struct, paired with its definition. A value of this type has the
    /// semantics of a pointer
    Struct(TypeObject),

    /// This type represents a virtual interface that can be stamped out from an object or dyn-obj.
    /// This is essentially just a bare vtable and this-pointer pair.
    Virtual(TypeVirtual),

    /// This represents an object type with a definition external to the compiled HashLink module.
    /// This is a tool for allowing an object's memory layout to be opaque and defined by the host
    /// application. It's a tool for integrating with native code.
    Abstract(TypeAbstract),

    /// Represents an enum type. An enum type is a type which represents a collection of "variants"
    /// that the type can contain.
    ///
    /// An enum is a *value* type and *NOT* a pointer. The whole structure is passed through
    /// functions by *value*.
    Enum(TypeEnum),
}

impl Type {
    /// Returns whether the type is a numeric type. (i.e, u8, f32, etc)
    pub fn is_numeric(&self) -> bool {
        match self {
            Type::U8 | Type::U16 | Type::I32 | Type::I64 | Type::F32 | Type::F64 => true,
            _ => false,
        }
    }

    /// Returns whether the type is a numeric floating point type. (i.e f32, f64)
    pub fn is_floating_point(&self) -> bool {
        match self {
            Type::F32 | Type::F64 => true,
            _ => false,
        }
    }

    /// Returns whether the type is a numeric integer type. (i.e i32, u16)
    pub fn is_integer(&self) -> bool {
        match self {
            Type::I32 | Type::I64 | Type::U16 | Type::U8 => true,
            _ => false,
        }
    }

    /// Returns whether the type is nullable.
    pub fn is_nullable(&self) -> bool {
        match self {
            Type::Bytes => true,
            Type::Dynamic => true,
            Type::Array(_) => true,
            Type::Type => false, // TODO: Double check if this is valid
            Type::DynObject => true,
            Type::Function(_) => true,
            Type::Method(_) => true,
            Type::Ref(_) => true,
            Type::Null(_) => true,
            Type::Obj(_) => true,
            Type::Struct(_) => true,
            Type::Virtual(_) => true,
            Type::Abstract(_) => true,
            Type::Enum(_) => false, // TODO: Double check if this is valid
            _ => false,
        }
    }

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

#[derive(Clone, Debug, Hash, PartialEq, Eq, Serialize, Deserialize)]
pub struct ObjectProto {
    /// Index into string table for the name
    pub name: StringIndex,

    /// ?
    pub f_index: usize,

    /// ?
    pub p_index: usize,
}

#[derive(Clone, Debug, Hash, PartialEq, Eq, Serialize, Deserialize)]
pub struct Field {
    /// Index into string table for the field name
    pub name: StringIndex,

    /// Index into type table for the type name
    pub type_: TypeIndex,
}

#[derive(Clone, Debug, Hash, PartialEq, Eq, Serialize, Deserialize)]
pub struct EnumConstruct {
    /// Index into string table for the name
    pub name: StringIndex,

    /// List of indexes into the type table
    pub params: Vec<TypeIndex>,
}

#[derive(Clone, Debug, Hash, PartialEq, Eq, Serialize, Deserialize)]
pub struct TypeFunction {
    /// List of indexes into type table for the function arguments
    pub args: Vec<TypeIndex>,

    /// Index into the type table for the return type
    pub returns: TypeIndex,
}

#[derive(Clone, Debug, Hash, PartialEq, Eq, Serialize, Deserialize)]
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

#[derive(Clone, Debug, Hash, PartialEq, Eq, Serialize, Deserialize)]
pub struct TypeEnum {
    /// Index into string table for the name
    pub name: StringIndex,

    /// ?
    pub constructs: Vec<EnumConstruct>,

    /// ?
    pub global: usize,
}

#[derive(Clone, Debug, Hash, PartialEq, Eq, Serialize, Deserialize)]
pub struct TypeVirtual {
    /// The list of fields on this virtual
    pub fields: Vec<Field>,
}

#[derive(Clone, Debug, Hash, PartialEq, Eq, Serialize, Deserialize)]
pub struct TypeParam {
    /// Index into the type table
    pub type_: TypeIndex,
}

#[derive(Clone, Debug, Hash, PartialEq, Eq, Serialize, Deserialize)]
pub struct TypeAbstract {
    /// Index into the string table for the name
    pub name: StringIndex,
}
