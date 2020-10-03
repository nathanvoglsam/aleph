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

use crate::function::Function;
use crate::Type;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fmt::Debug;
use std::hash::Hash;

/// This struct represents a struct or class like object
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct Class<T: Clone + Debug + Eq + PartialEq + Hash + AsRef<str>> {
    pub fields: HashMap<String, Type<T>>,
    pub functions: HashMap<String, Function<T>>,
}

impl<T: Clone + Debug + Eq + PartialEq + Hash + AsRef<str>> Class<T> {
    /// Does this class have no member variables
    pub fn has_fields(&self) -> bool {
        !self.fields.is_empty()
    }

    /// Whether this class has any methods (member functions)
    pub fn has_methods(&self) -> bool {
        !self.functions.is_empty()
    }

    /// Does this class have any static methods. Will return false if it has no methods at all
    pub fn has_static_methods(&self) -> bool {
        self.functions.iter().any(|(_, v)| v.is_static())
    }

    /// Does this class have any non static methods. Will return false if it has no methods at all
    pub fn has_non_static_methods(&self) -> bool {
        self.functions.iter().any(|(_, v)| !v.is_static())
    }

    /// Does this class only have static methods. Will return false if it has no methods at all
    pub fn has_only_static_methods(&self) -> bool {
        self.has_static_methods() && !self.has_non_static_methods()
    }

    /// Does this class only have non static methods. Will return false if it has no methods at all
    pub fn has_only_non_static_methods(&self) -> bool {
        !self.has_static_methods() && self.has_non_static_methods()
    }

    /// Whether this class is a singleton object
    pub fn is_singleton(&self) -> bool {
        self.has_only_static_methods() && !self.has_fields()
    }
}
