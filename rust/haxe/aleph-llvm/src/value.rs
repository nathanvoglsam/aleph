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

use crate::{BasicBlock, Context};
use llvm_sys::core::*;
use llvm_sys::prelude::*;
use std::marker::PhantomData;

#[derive(Copy, Clone)]
#[repr(transparent)]
pub struct Value<'a> {
    pub(crate) inner: LLVMValueRef,
    pub(crate) phantom: PhantomData<&'a Context>,
}

impl<'a> Value<'a> {
    pub(crate) fn new(inner: LLVMValueRef) -> Self {
        Value {
            inner,
            phantom: Default::default(),
        }
    }

    pub fn get_param(&self, i: u32) -> Value {
        unsafe {
            let inner = LLVMGetParam(self.inner, i as _);
            Value {
                inner,
                phantom: Default::default(),
            }
        }
    }

    pub fn add_incoming(&self, values: &[Value], blocks: &[BasicBlock]) {
        unsafe {
            // Cast values pointer
            let vals_ptr = values.as_ptr();
            let vals_ptr = vals_ptr as *const LLVMValueRef;
            let vals_ptr = vals_ptr as *mut LLVMValueRef;

            // Cast blocks pointer
            let blocks_ptr = blocks.as_ptr();
            let blocks_ptr = blocks_ptr as *const LLVMBasicBlockRef;
            let blocks_ptr = blocks_ptr as *mut LLVMBasicBlockRef;

            // Select the shortest length of the two slices
            let len = values.len().min(blocks.len());

            // Assert that the length fits into the FFI type
            assert!(len < libc::c_uint::max_value() as usize);

            LLVMAddIncoming(self.inner, vals_ptr, blocks_ptr, len as _);
        }
    }
}
