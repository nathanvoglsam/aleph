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

use crate::ast::Type;
use crate::interner::{Interner, StrId};
use std::fmt::Debug;
use std::hash::Hash;

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct Function {
    /// The arguments of the function
    pub args: Vec<Type>,

    /// The return type of the function
    pub returns: Type,
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

    /// Put `self` into a `Box`
    pub fn boxed(self) -> Box<Self> {
        Box::new(self)
    }

    /// Will convert drill down recursively until a `Type::Path` variant is reached, where it will
    /// convert the relative path to an absolute path
    pub(crate) fn relative_to_absolute_path(
        &mut self,
        name_stack: &[StrId],
        interner: &mut Interner,
    ) -> crate::ast::Result<()> {
        self.returns
            .relative_to_absolute_path(name_stack, interner)?;
        for arg in self.args.iter_mut() {
            arg.relative_to_absolute_path(name_stack, interner)?;
        }
        Ok(())
    }

    pub fn from_function_signature(interner: &mut Interner, sig: &syn::Signature) -> Option<Self> {
        let returns = Type::from_return_type(interner, &sig.output)?;

        let mut args = Vec::new();
        for arg in sig.inputs.iter() {
            args.push(Type::from_function_arg(interner, arg)?);
        }

        let out = Function { args, returns };

        Some(out)
    }
}
