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

use crate::utils::{drill_through_parens, path_to_string};
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

    /// special "this" or "self" type to be used by member functions
    This,

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
    /// use aleph_interface_description::Type;
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
    pub fn is_this_ptr(&self) -> bool {
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
    pub fn is_mutable(&self) -> bool {
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
            | Type::Path(_)
            | Type::FunctionPointer(_) => false,
            Type::Array(_)
            | Type::Slice(_)
            | Type::MutableReference(_)
            | Type::MutablePointer(_) => true,
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
    /// use aleph_interface_description::Type;
    ///
    /// let t = Type::F32;
    /// let ref_t = Type::ConstReference(t.boxed_clone());
    ///
    /// assert_eq!(t, Type::F32);
    /// assert!(!ref_t.is_mutable());
    /// ```
    pub fn boxed_clone(&self) -> Box<Self> {
        Box::new(self.clone())
    }

    /// Put `self` into a `Box`
    pub fn boxed(self) -> Box<Self> {
        Box::new(self)
    }
}

impl Type<String> {
    /// Takes a `syn::Path` and converts it to a `Type`. This will check for and coerce to one of
    /// the primitive type variants if one is detected, and will return `None` on unsupported
    /// primitive types
    pub fn from_path(path: &syn::Path) -> Option<Self> {
        let our_path = path_to_string(path);
        match our_path.as_str() {
            "u8" => Some(Type::U8),
            "u16" => Some(Type::U16),
            "u32" => Some(Type::U32),
            "u64" => Some(Type::U64),
            "i8" => Some(Type::I8),
            "i16" => Some(Type::I16),
            "i32" => Some(Type::I32),
            "i64" => Some(Type::I64),
            "f32" => Some(Type::F32),
            "f64" => Some(Type::F64),
            "usize" => None, // Unsupported type
            "isize" => None, // Unsupported type
            "i128" => None,  // Unsupported type
            "u128" => None,  // Unsupported type
            "bool" => None,  // Unsupported type
            "str" => None,   // Unsupported type
            _ => Some(Type::Path(our_path)),
        }
    }

    /// Creates a `Type`
    pub fn from_ptr_like_inner(ty: &syn::Type) -> Option<Self> {
        match ty {
            syn::Type::Array(t) => Self::from_array(t),
            syn::Type::BareFn(t) => Self::from_bare_fn(t),
            syn::Type::Path(t) => Self::from_path(&t.path),
            syn::Type::Ptr(t) => Self::from_ptr(t),
            syn::Type::Reference(t) => Self::from_ref(t),
            syn::Type::Slice(t) => Self::from_slice(t),
            _ => None,
        }
    }

    /// Creates a `Type` from a `syn::TypePtr`. This should be called where the root type is a
    /// pointer
    pub fn from_ptr(ptr: &syn::TypePtr) -> Option<Self> {
        let ty = drill_through_parens(ptr.elem.as_ref());
        let inner = Self::from_ptr_like_inner(ty);
        if ptr.mutability.is_some() {
            Some(Type::MutablePointer(inner?.boxed()))
        } else {
            Some(Type::ConstPointer(inner?.boxed()))
        }
    }

    /// Creates a `Type` from a `syn::TypePtr`. This should be called where the root type is a
    /// pointer
    pub fn from_ref(reference: &syn::TypeReference) -> Option<Self> {
        let ty = drill_through_parens(reference.elem.as_ref());
        let inner = Self::from_ptr_like_inner(ty);
        if reference.mutability.is_some() {
            Some(Type::MutableReference(inner?.boxed()))
        } else {
            Some(Type::ConstReference(inner?.boxed()))
        }
    }

    /// Produces a `Type` from a `syn::FnArg`
    pub fn from_bare_function_arg(arg: &syn::BareFnArg) -> Option<Self> {
        match drill_through_parens(&arg.ty) {
            //syn::Type::Array(_) => unimplemented!(),
            syn::Type::BareFn(t) => Self::from_bare_fn(t),
            syn::Type::Path(t) => Self::from_path(&t.path),
            syn::Type::Ptr(t) => Self::from_ptr(t),
            syn::Type::Reference(t) => Self::from_ref(t),
            //syn::Type::Slice(_) => unimplemented!(),
            _ => None,
        }
    }

    pub fn from_struct_field(ty: &syn::Type) -> Option<Self> {
        let ty = drill_through_parens(ty);
        match ty {
            syn::Type::Array(t) => Self::from_array(t),
            syn::Type::BareFn(t) => Self::from_bare_fn(t),
            syn::Type::Path(t) => Self::from_path(&t.path),
            syn::Type::Ptr(t) => Self::from_ptr(t),
            syn::Type::Reference(t) => Self::from_ref(t),
            _ => None,
        }
    }

    pub fn from_bare_fn(bare: &syn::TypeBareFn) -> Option<Self> {
        let returns = Type::from_return_type(&bare.output)?;

        let mut args = Vec::new();
        for arg in bare.inputs.iter() {
            args.push(Type::from_bare_function_arg(arg)?);
        }

        let function = Function { args, returns };
        let out = Type::FunctionPointer(function.boxed());

        Some(out)
    }

    /// Creates a `Type` from the `syn::Type` of an array like primitive. This should be the root
    /// type of an array or slice type
    pub fn from_array_like(ty: &syn::Type) -> Option<Self> {
        let ty = drill_through_parens(ty);
        match ty {
            syn::Type::Array(t) => Self::from_array(t),
            syn::Type::BareFn(t) => Self::from_bare_fn(t),
            syn::Type::Path(t) => Self::from_path(&t.path),
            syn::Type::Ptr(t) => Self::from_ptr(t),
            syn::Type::Reference(t) => Self::from_ref(t),
            syn::Type::Slice(t) => Self::from_slice(t),
            _ => None,
        }
    }

    /// Produces a `Type` from a `syn::FnArg`
    pub fn from_function_arg(arg: &syn::FnArg) -> Option<Self> {
        match arg {
            syn::FnArg::Receiver(v) => {
                let mutable = v.mutability.is_some();
                let reference = v.reference.is_some();

                match (mutable, reference) {
                    (true, true) => Some(Type::MutableReference(Type::This.boxed())),
                    (false, true) => Some(Type::ConstReference(Type::This.boxed())),
                    (_, _) => None,
                }
            }
            syn::FnArg::Typed(v) => match drill_through_parens(v.ty.as_ref()) {
                //syn::Type::Array(_) => unimplemented!(),
                syn::Type::BareFn(t) => Self::from_bare_fn(t),
                syn::Type::Path(t) => Self::from_path(&t.path),
                syn::Type::Ptr(t) => Self::from_ptr(t),
                syn::Type::Reference(t) => Self::from_ref(t),
                //syn::Type::Slice(_) => unimplemented!(),
                _ => None,
            },
        }
    }

    /// Creates a `Type` from a `syn::TypeArray`
    pub fn from_array(array: &syn::TypeArray) -> Option<Self> {
        let ty = Self::from_array_like(array.elem.as_ref())?;

        let size: Option<u64> = match &array.len {
            syn::Expr::Lit(lit) => match &lit.lit {
                syn::Lit::Int(int) => int.base10_parse().ok(),
                _ => None,
            },
            //syn::Expr::Paren(_) => unimplemented!(),
            _ => None,
        };

        let inner = (size?, ty.boxed());
        Some(Type::Array(inner))
    }

    /// Creates a `Type` from a `syn::TypeSlice`
    pub fn from_slice(slice: &syn::TypeSlice) -> Option<Self> {
        let ty = Self::from_array_like(slice.elem.as_ref())?;
        Some(Type::Slice(ty.boxed()))
    }

    /// Produces a `Type` from a `syn::ReturnType`
    pub fn from_return_type(output: &syn::ReturnType) -> Option<Self> {
        match output {
            syn::ReturnType::Default => Some(Type::Void),
            syn::ReturnType::Type(_, t) => Self::from_type_for_return_value(t.as_ref()),
        }
    }

    /// Produces a `Type` from a `syn::Type` in the context of a return value.
    ///
    /// This function is only *sane* to call on the root `syn::Type` from a `syn::ReturnType` for a
    /// `syn::Type::BareFn`.
    ///
    /// # Errors
    ///
    /// This will fail on any `syn::Type` value that does not make sense in the context of a
    /// function return value across the FFI boundary.
    pub fn from_type_for_return_value(ty: &syn::Type) -> Option<Self> {
        let ty = drill_through_parens(ty);
        match ty {
            //syn::Type::Array(_) => unimplemented!(),
            syn::Type::BareFn(t) => Self::from_bare_fn(t),
            syn::Type::Path(t) => Self::from_path(&t.path),
            syn::Type::Ptr(t) => Self::from_ptr(t),
            syn::Type::Reference(t) => Self::from_ref(t),
            _ => None,
        }
    }
}
