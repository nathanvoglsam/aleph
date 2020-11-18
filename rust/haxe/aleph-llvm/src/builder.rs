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
use crate::{BasicBlock, Context, IntPredicate, Type, Value};
use llvm_sys::core::*;
use llvm_sys::prelude::*;
use std::ffi::CStr;
use std::marker::PhantomData;

#[repr(transparent)]
pub struct Builder<'a> {
    pub(crate) inner: LLVMBuilderRef,
    pub(crate) phantom: PhantomData<&'a Context>,
}

impl<'a> Builder<'a> {
    pub fn position_at_end(&self, basic_block: BasicBlock) {
        unsafe { LLVMPositionBuilderAtEnd(self.inner, basic_block.inner) }
    }

    pub fn build_add(&self, lhs: Value, rhs: Value, name: Option<&CStr>) -> Value {
        unsafe {
            let inner = LLVMBuildAdd(self.inner, lhs.inner, rhs.inner, cstr_or_empty(name));
            Value::new(inner)
        }
    }

    pub fn build_sub(&self, lhs: Value, rhs: Value, name: Option<&CStr>) -> Value {
        unsafe {
            let inner = LLVMBuildSub(self.inner, lhs.inner, rhs.inner, cstr_or_empty(name));
            Value::new(inner)
        }
    }

    pub fn build_mul(&self, lhs: Value, rhs: Value, name: Option<&CStr>) -> Value {
        unsafe {
            let inner = LLVMBuildMul(self.inner, lhs.inner, rhs.inner, cstr_or_empty(name));
            Value::new(inner)
        }
    }

    pub fn build_sdiv(&self, lhs: Value, rhs: Value, name: Option<&CStr>) -> Value {
        unsafe {
            let inner = LLVMBuildSDiv(self.inner, lhs.inner, rhs.inner, cstr_or_empty(name));
            Value::new(inner)
        }
    }

    pub fn build_udiv(&self, lhs: Value, rhs: Value, name: Option<&CStr>) -> Value {
        unsafe {
            let inner = LLVMBuildUDiv(self.inner, lhs.inner, rhs.inner, cstr_or_empty(name));
            Value::new(inner)
        }
    }

    pub fn build_load(&self, val: Value, name: Option<&CStr>) -> Value {
        unsafe {
            let inner = LLVMBuildLoad(self.inner, val.inner, cstr_or_empty(name));
            Value::new(inner)
        }
    }

    pub fn build_store(&self, val: Value, ptr: Value) -> Value {
        unsafe {
            let inner = LLVMBuildStore(self.inner, val.inner, ptr.inner);
            Value::new(inner)
        }
    }

    pub fn build_br(&self, dest: BasicBlock) -> Value {
        unsafe {
            let inner = LLVMBuildBr(self.inner, dest.inner);
            Value::new(inner)
        }
    }

    pub fn build_cond_br(&self, cond: Value, pass: BasicBlock, fail: BasicBlock) -> Value {
        unsafe {
            let inner = LLVMBuildCondBr(self.inner, cond.inner, pass.inner, fail.inner);
            Value::new(inner)
        }
    }

    pub fn build_icmp(
        &self,
        op: IntPredicate,
        lhs: Value,
        rhs: Value,
        name: Option<&CStr>,
    ) -> Value {
        unsafe {
            let inner = LLVMBuildICmp(self.inner, op, lhs.inner, rhs.inner, cstr_or_empty(name));
            Value::new(inner)
        }
    }

    pub fn build_phi(&self, t: Type, name: Option<&CStr>) -> Value {
        unsafe {
            let inner = LLVMBuildPhi(self.inner, t.inner, cstr_or_empty(name));
            Value {
                inner,
                phantom: Default::default(),
            }
        }
    }

    pub fn build_ret(&self, val: Value) -> Value {
        unsafe {
            let inner = LLVMBuildRet(self.inner, val.inner);
            Value {
                inner,
                phantom: Default::default(),
            }
        }
    }

    pub fn build_ret_void(&self) -> Value {
        unsafe {
            let inner = LLVMBuildRetVoid(self.inner);
            Value {
                inner,
                phantom: Default::default(),
            }
        }
    }
}

impl<'a> Drop for Builder<'a> {
    fn drop(&mut self) {
        unsafe {
            LLVMDisposeBuilder(self.inner);
        }
    }
}
