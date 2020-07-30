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

use crate::reflect::structure::StructResolutionError;
use crate::reflect::Struct;
use spirv_reflect::types::ReflectBlockVariable;
use std::ops::Deref;

///
/// Represents a push constant layout reflected from a shader module
///
#[derive(Clone, Hash, PartialEq, Eq, Debug)]
pub struct PushConstantReflection {
    inner: Struct,
}

impl PushConstantReflection {
    ///
    /// Reflect a push constant layout from the given block variable
    ///
    pub fn reflect(block: ReflectBlockVariable) -> Result<Self, StructResolutionError> {
        let inner = super::structure::resolve_struct_block(block)?;
        Ok(Self { inner })
    }
}

impl Deref for PushConstantReflection {
    type Target = Struct;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
