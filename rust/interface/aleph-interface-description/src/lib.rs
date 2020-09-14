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

use std::collections::HashMap;
use serde::{Serialize, Deserialize};

/// The supported set of types in the Aleph IDL
#[derive(Clone, Debug, Ord, PartialOrd, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub enum Type {
    U8,
    U16,
    U32,
    U64,
    I8,
    I16,
    I32,
    I64,
    F32,
    F64,
    Void,
    This,
    ConstSlice((Option<usize>, Box<Type>)),
    MutableSlice((Option<usize>, Box<Type>)),
    ConstReference(Box<Type>),
    MutableReference(Box<Type>),
    ConstPointer(Box<Type>),
    MutablePointer(Box<Type>),
    Struct(String),
}

impl Type {
    /// Returns if this type is exactly the `This` variant
    ///
    /// ```
    /// use aleph_interface_description::Type;
    ///
    /// assert!(Type::This.is_this());
    /// assert!(!Type::I8.is_this());
    /// ```
    pub const fn is_this(&self) -> bool {
        match self {
            Type::This => true,
            _ => false,
        }
    }

    /// Returns if this type is exactly either `Type::ConstReference(Type::This)` or
    /// `Type::MutableReference(Type::This)`
    ///
    /// ```
    /// use aleph_interface_description::Type;
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
    pub const fn is_this_ref(&self) -> bool {
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
    /// use aleph_interface_description::Type;
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
    pub const fn is_this_ptr(&self) -> bool {
        match self {
            Type::ConstPointer(v) => v.is_this(),
            Type::MutablePointer(v) => v.is_this(),
            _ => false,
        }
    }

    /// Returns if the outermost `Type` is mutable.
    ///
    /// Primitives and `Struct` are *not* mutable under the semantics of this interface.
    ///
    /// ```
    /// ```
    pub const fn is_mutable(&self) -> bool {
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
            | Type::ConstReference(_)
            | Type::ConstPointer(_)
            | Type::Struct(_)
            | Type::ConstSlice(_) => false,
            Type::MutableSlice(_) | Type::MutableReference(_) | Type::MutablePointer(_) => true,
        }
    }

    /// Does this type encode, at any level of nesting, a `This` variant.
    ///
    /// ```
    /// use aleph_interface_description::Type;
    ///
    /// assert!(Type::This.contains_this());
    ///
    /// let ref_this = Type::ConstReference(Box::new(Type::This));
    /// assert!(ref_this.contains_this());
    ///
    /// let mutref_ref_this = Type::MutableReference(Box::new(ref_this.clone()));
    /// assert!(mutref_ref_this.contains_this());
    ///
    /// let slice = (None, mutref_ref_this.boxed_clone());
    /// let slice_mutref_ref_this = Type::ConstSlice(slice);
    /// assert!(slice_mutref_ref_this.contains_this());
    ///
    /// assert!(!Type::U8.contains_this());
    /// ```
    pub const fn contains_this(&self) -> bool {
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
            | Type::Struct(_) => false,
            Type::This => true,
            Type::ConstSlice((_, v)) => v.contains_this(),
            Type::MutableSlice((_, v)) => v.contains_this(),
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
    /// use aleph_interface_description::Type;
    ///
    /// let t = Type::F32;
    /// let ref_t = Type::ConstReference(t.boxed_clone());
    ///
    /// assert_eq!(t, Type::F32);
    /// assert!(!ref_t.is_mutable());
    /// ```
    pub fn boxed_clone(&self) -> Box<Type> {
        Box::new(self.clone())
    }
}

#[derive(Clone, Debug, Ord, PartialOrd, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub struct Function {
    /// The name of the function
    pub name: String,

    /// The arguments of the function
    pub args: Vec<Type>,

    /// The return type of the function
    pub returns: Type,

    /// The fully qualified name of a `Class` this type is a member of
    pub member_of: Option<String>,
}

impl Function {
    /// Is this function a static function. This is similar to a function being a free function, but
    /// a static can still be associated with a `Class`
    pub fn is_static(&self) -> bool {
        if let Some(arg) = self.args.get(0) {
            match arg {
                Type::ConstReference(v) => !v.is_this(),
                Type::MutableReference(v) => !v.is_this(),
                _ => true,
            }
        } else {
            true
        }
    }

    /// Does this function have any arguments or a return value that encodes the `Type::This`
    /// variant
    pub fn contains_this(&self) -> bool {
        self.args.iter().any(Type::contains_this) || self.returns.contains_this()
    }

    /// Is this function not associated with a `Class`
    pub fn is_free(&self) -> bool {
        self.member_of.is_none()
    }
}

/// This struct represents a struct or class like object
#[derive(Clone, Debug, Ord, PartialOrd, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub struct Class {
    pub members: Vec<Type>,
}

impl Class {
    /// Does this class have no member variables
    pub fn is_zero_sized(&self) -> bool {
        self.members.is_empty()
    }

    /// Whether this class is a singleton object
    pub fn is_singleton(&self) -> bool {
        self.is_zero_sized()
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct InterfaceDescription {
    pub classes: HashMap<String, Class>,
    pub functions: Vec<Function>,
}
