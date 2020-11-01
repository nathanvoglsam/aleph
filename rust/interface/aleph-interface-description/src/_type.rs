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

use crate::Function;
use serde::{Deserialize, Serialize};
use std::fmt::Debug;
use std::hash::Hash;

/// The supported set of types in the Aleph IDL
#[derive(Clone, Debug, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub enum Type<T: Clone + Debug + Eq + PartialEq + Hash + AsRef<str>> {
    /// u8 primitive type
    U8,

    /// u16 primitive type
    U16,

    /// u32 primitive type
    U32,

    /// u64 primitive type
    U64,

    /// i8 primitive type
    I8,

    /// i16 primitive type
    I16,

    /// i32 primitive type
    I32,

    /// i64 primitive type
    I64,

    /// f32 primitive type
    F32,

    /// f32 primitive type
    F64,

    /// void primitive type (only really useful for return type)
    Void,

    /// special "this" or "self" type to be used by member functions. This is distinct from `Self_`
    /// as self is purely for resolving a concrete type while `This` is for representing a
    /// "this ptr".
    This,

    /// rust's "Self" type that is used to refer to the type the function is defined on
    SelfType,

    /// A function pointer, with the given signature
    FunctionPointer(Box<Function<T>>),

    /// A slice of a given type. This matches the semantics of a slice in rust
    Slice(Box<Type<T>>),

    /// An array of a given type. This matches semantics of an array in rust
    Array((u64, Box<Type<T>>)),

    /// A non-nullable const pointer. This matches the semantics of a shared reference in rust
    ConstReference(Box<Type<T>>),

    /// A non-nullable mutable pointer. This matches the semantics of a mutable reference in rust
    MutableReference(Box<Type<T>>),

    /// A const pointer. The mutability semantics should match that of a shared reference in rust
    ConstPointer(Box<Type<T>>),

    /// A mutable pointer. The mutability semantics should match that of a mutable reference in rust
    MutablePointer(Box<Type<T>>),

    /// A reference to another type
    Path(T),
}

impl<T: Clone + Debug + Eq + PartialEq + Hash + AsRef<str>> Type<T> {
    /// Returns if this type is exactly the `This` variant
    ///
    /// ```
    /// type Type = aleph_interface_description::Type<String>;
    ///
    /// assert!(Type::This.is_this());
    /// assert!(!Type::I8.is_this());
    /// ```
    pub fn is_this(&self) -> bool {
        match self {
            Type::This => true,
            _ => false,
        }
    }

    /// Returns if this type is exactly either `Type::ConstReference(Type::This)` or
    /// `Type::MutableReference(Type::This)`
    ///
    /// ```
    /// type Type = aleph_interface_description::Type<String>;
    ///
    /// let this_type = Type::This;
    ///
    /// let t = Type::ConstReference(Box::new(Type::This));
    /// assert!(t.is_this_ref());
    ///
    /// let t = Type::MutableReference(Box::new(Type::This));
    /// assert!(t.is_this_ref());
    ///
    /// assert!(!Type::This.is_this_ref());
    /// assert!(!Type::I8.is_this_ref());
    /// ```
    pub fn is_this_ref(&self) -> bool {
        match self {
            Type::ConstReference(v) => v.is_this(),
            Type::MutableReference(v) => v.is_this(),
            _ => false,
        }
    }

    /// Returns if this type is exactly either `Type::ConstPointer(Type::This)` or
    /// `Type::MutablePointer(Type::This)`
    ///
    /// ```
    /// type Type = aleph_interface_description::Type<String>;
    ///
    /// let this_type = Type::This;
    ///
    /// let t = Type::ConstPointer(Box::new(Type::This));
    /// assert!(t.is_this_ptr());
    ///
    /// let t = Type::MutablePointer(Box::new(Type::This));
    /// assert!(t.is_this_ptr());
    ///
    /// assert!(!Type::This.is_this_ptr());
    /// assert!(!Type::I8.is_this_ptr());
    /// ```
    pub fn is_this_ptr(&self) -> bool {
        match self {
            Type::ConstPointer(v) => v.is_this(),
            Type::MutablePointer(v) => v.is_this(),
            _ => false,
        }
    }

    /// Returns whether this type is a data reference and mutable.
    ///
    /// Will return `None` if the type is not a data reference (ConstReference, MutablePointer, etc)
    ///
    /// Will return `Some(bool)` if the type is a data reference where bool depends on whether the
    /// reference is mutable.
    ///
    /// # Warning
    ///
    /// `Slice` and `Array` are not data references they are concrete types that are only sane to
    /// expose through a reference, so a bare `Array` or `Slice` will return `None`.
    ///
    /// `FunctionPointer`, while being a reference, does not refer to "data" that can be read and so
    /// would strictly always be immutable. `None` will be returned for a `FunctionPointer` as I
    /// believe it is enough of an edge case to not consider in this function.
    ///
    /// ```
    /// ```
    pub fn is_mutable_ref(&self) -> Option<bool> {
        match self {
            Type::U8
            | Type::U16
            | Type::U32
            | Type::U64
            | Type::I8
            | Type::I16
            | Type::I32
            | Type::I64
            | Type::F32
            | Type::F64
            | Type::Void
            | Type::This
            | Type::SelfType
            | Type::Path(_)
            | Type::FunctionPointer(_)
            | Type::Array(_)
            | Type::Slice(_) => None,
            Type::ConstReference(_) | Type::ConstPointer(_) => Some(false),
            Type::MutableReference(_) | Type::MutablePointer(_) => Some(true),
        }
    }

    /// Does this type encode, at any level of nesting, a `This` variant.
    ///
    /// ```
    /// type Type = aleph_interface_description::Type<String>;
    ///
    /// assert!(Type::This.contains_this());
    ///
    /// let ref_this = Type::ConstReference(Box::new(Type::This));
    /// assert!(ref_this.contains_this());
    ///
    /// let mutref_ref_this = Type::MutableReference(Box::new(ref_this.clone()));
    /// assert!(mutref_ref_this.contains_this());
    ///
    /// let slice_mutref_ref_this = Type::Slice(ref_this.boxed_clone());
    /// assert!(slice_mutref_ref_this.contains_this());
    ///
    /// assert!(!Type::U8.contains_this());
    /// ```
    pub fn contains_this(&self) -> bool {
        match self {
            Type::U8
            | Type::U16
            | Type::U32
            | Type::U64
            | Type::I8
            | Type::I16
            | Type::I32
            | Type::I64
            | Type::F32
            | Type::F64
            | Type::Void
            | Type::SelfType
            | Type::Path(_) => false,
            Type::This => true,
            Type::FunctionPointer(v) => v.contains_this(),
            Type::Array((_, v)) => v.contains_this(),
            Type::Slice(v) => v.contains_this(),
            Type::ConstReference(v) => v.contains_this(),
            Type::MutableReference(v) => v.contains_this(),
            Type::ConstPointer(v) => v.contains_this(),
            Type::MutablePointer(v) => v.contains_this(),
        }
    }

    /// Clones the `Type` into a `Box`
    ///
    /// Useful if you're constructing a `Type` but need the `Type` being boxed multiple times and
    /// don't want to chain functions or nest functions
    ///
    /// ```
    /// type Type = aleph_interface_description::Type<String>;
    ///
    /// let t = Type::F32;
    /// let ref_t = Type::ConstReference(t.boxed_clone());
    ///
    /// assert_eq!(t, Type::F32);
    /// assert!(ref_t.is_mutable_ref().is_some());
    /// ```
    pub fn boxed_clone(&self) -> Box<Self> {
        Box::new(self.clone())
    }

    /// Put `self` into a `Box`
    pub fn boxed(self) -> Box<Self> {
        Box::new(self)
    }
}
