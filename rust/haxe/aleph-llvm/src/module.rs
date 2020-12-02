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

use crate::utils::cstr_or_empty;
use crate::{Context, Type, Value};
use llvm_sys::core::*;
use llvm_sys::prelude::*;
use std::ffi::CStr;
use std::marker::PhantomData;

/// A safe rusty wrapper around the `LLVMModule` type
#[repr(transparent)]
pub struct Module<'a> {
    pub(crate) inner: LLVMModuleRef,
    pub(crate) phantom: PhantomData<&'a Context>,
}

impl<'a> Module<'a> {
    /// Adds a new function with the given name and type
    pub fn add_function(&self, name: Option<&CStr>, t: Type) -> Value {
        unsafe {
            let inner = LLVMAddFunction(self.inner, cstr_or_empty(name), t.inner);
            Value {
                inner,
                phantom: Default::default(),
            }
        }
    }

    /// Wraps the `LLVMDumpModule` function to dump the LLVM-IR to stdout
    pub fn dump(&self) {
        unsafe { LLVMDumpModule(self.inner) }
    }

    /// Returns the module's current target triple
    pub fn get_target(&self) -> String {
        unsafe {
            let result = LLVMGetTarget(self.inner);
            let result = CStr::from_ptr(result);
            let result = result.to_str().unwrap();
            result.to_string()
        }
    }

    /// Sets the module's target triple to the provided value
    pub fn set_target(&self, triple: &CStr) {
        unsafe {
            LLVMSetTarget(self.inner, triple.as_ptr());
        }
    }

    /// Get the module's data layout as a string
    pub fn get_data_layout_string(&self) -> String {
        unsafe {
            let result = LLVMGetDataLayoutStr(self.inner);
            let result = CStr::from_ptr(result);
            let result = result.to_str().unwrap();
            result.to_string()
        }
    }

    /// Set the module's data layout based on the string provided
    pub fn set_data_layout_from_string(&self, layout: &CStr) {
        unsafe {
            LLVMSetDataLayout(self.inner,  layout.as_ptr());
        }
    }
}

impl<'a> Drop for Module<'a> {
    fn drop(&mut self) {
        unsafe {
            LLVMDisposeModule(self.inner);
        }
    }
}
