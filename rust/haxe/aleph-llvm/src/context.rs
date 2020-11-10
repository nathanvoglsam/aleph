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
use crate::{BasicBlock, Builder, Module, Type, Value};
use llvm_sys::core::*;
use llvm_sys::prelude::*;
use std::ffi::CStr;

///
/// Represents the app's singular connection to the global LLVM context. Provides a wrapper to
/// help ensure thread safety by making this not `Send` and `Sync` and enforcing only a single
/// instance of this object exists
///
#[repr(transparent)]
pub struct Context {
    pub(crate) inner: LLVMContextRef,
}

impl Context {
    pub fn init() -> Context {
        unsafe {
            let inner = LLVMContextCreate();
            Self { inner }
        }
    }

    pub fn create_module(&self, name: Option<&CStr>) -> Module {
        unsafe {
            let inner = LLVMModuleCreateWithNameInContext(cstr_or_empty(name), self.inner);
            Module {
                inner,
                phantom: Default::default(),
            }
        }
    }

    pub fn create_builder(&self) -> Builder {
        unsafe {
            let inner = LLVMCreateBuilderInContext(self.inner);
            Builder {
                inner,
                phantom: Default::default(),
            }
        }
    }

    pub fn create_function_type(&self, returns: Type, args: &[Type], is_vararg: bool) -> Type {
        unsafe {
            // Check that both types are of the same size, and that there aren't more args than fit
            // in the type the FFI is using
            assert_eq!(
                std::mem::size_of::<Type>(),
                std::mem::size_of::<LLVMTypeRef>()
            );
            assert!(args.len() < libc::c_uint::max_value() as usize);

            // Cast the args pointer in to the form the FFI expects
            let args_ptr = args.as_ptr();
            let args_ptr = args_ptr as *mut Type;
            let args_ptr = args_ptr as *mut LLVMTypeRef;

            // Convert bool arg into int for FFI
            let is_vararg = if is_vararg { 1 } else { 0 };

            let inner = LLVMFunctionType(returns.inner, args_ptr, args.len() as _, is_vararg);

            Type {
                inner,
                phantom: Default::default(),
            }
        }
    }

    pub fn create_void_type(&self) -> Type {
        unsafe { Type::new(LLVMVoidTypeInContext(self.inner)) }
    }

    pub fn create_ptr_type(&self, t: Type, address_space: u32) -> Type {
        unsafe { Type::new(LLVMPointerType(t.inner, address_space as _)) }
    }

    pub fn create_f32_type(&self) -> Type {
        unsafe { Type::new(LLVMFloatTypeInContext(self.inner)) }
    }

    pub fn create_f64_type(&self) -> Type {
        unsafe { Type::new(LLVMDoubleTypeInContext(self.inner)) }
    }

    pub fn create_int_type(&self, bits: u32) -> Type {
        unsafe { Type::new(LLVMIntTypeInContext(self.inner, bits as _)) }
    }

    pub fn create_i1_type(&self) -> Type {
        unsafe { Type::new(LLVMInt1TypeInContext(self.inner)) }
    }

    pub fn create_i8_type(&self) -> Type {
        unsafe { Type::new(LLVMInt8TypeInContext(self.inner)) }
    }

    pub fn create_i16_type(&self) -> Type {
        unsafe { Type::new(LLVMInt16TypeInContext(self.inner)) }
    }

    pub fn create_i32_type(&self) -> Type {
        unsafe { Type::new(LLVMInt32TypeInContext(self.inner)) }
    }

    pub fn create_i64_type(&self) -> Type {
        unsafe { Type::new(LLVMInt64TypeInContext(self.inner)) }
    }

    pub fn create_i128_type(&self) -> Type {
        unsafe { Type::new(LLVMInt128TypeInContext(self.inner)) }
    }

    pub fn create_int_constant(&self, t: Type, val: u64, sign_extend: bool) -> Value {
        unsafe {
            let sign_extend = if sign_extend { 1 } else { 0 };
            let inner = LLVMConstInt(t.inner, val as _, sign_extend);
            Value {
                inner,
                phantom: Default::default(),
            }
        }
    }

    pub fn append_basic_block(&self, value: Value, name: Option<&CStr>) -> BasicBlock {
        unsafe {
            let inner = LLVMAppendBasicBlockInContext(self.inner, value.inner, cstr_or_empty(name));
            BasicBlock {
                inner,
                phantom: Default::default(),
            }
        }
    }
}

impl Drop for Context {
    fn drop(&mut self) {
        unsafe {
            LLVMContextDispose(self.inner);
        }
    }
}
