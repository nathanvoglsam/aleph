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

use crate::{Context, Module};
use core::mem;
use llvm_sys::execution_engine::*;
use llvm_sys::target::*;
use std::ffi::CStr;
use std::marker::PhantomData;
use std::str::Utf8Error;
use std::sync::atomic::{AtomicBool, Ordering};

static TARGET_INIT_GUARD: AtomicBool = AtomicBool::new(false);

#[derive(Clone, Debug)]
pub enum ExecutionEngineCreateError {
    /// This error occurs when the error string llvm provides is not valid UTF8
    Utf8Error(Utf8Error),

    /// This will 99.999% of the time be the error you actually get, and is the string llvm returns
    LLVMError(String),
}

impl From<Utf8Error> for ExecutionEngineCreateError {
    fn from(err: Utf8Error) -> Self {
        ExecutionEngineCreateError::Utf8Error(err)
    }
}

#[repr(transparent)]
pub struct ExecutionEngine<'a> {
    pub(crate) inner: LLVMExecutionEngineRef,
    pub(crate) phantom: PhantomData<&'a Context>,
}

impl<'a> ExecutionEngine<'a> {
    pub fn create_interpreter(
        module: Module<'a>,
    ) -> Result<ExecutionEngine, ExecutionEngineCreateError> {
        unsafe {
            // This is needed to bamboozle the linker in to not removing supposedly dead code, at
            // least according to the comments in the LLVM source code
            LLVMLinkInInterpreter();

            // build an execution engine
            let mut inner = mem::zeroed();
            let mut out_error = mem::zeroed();

            // The execution engine takes ownership of the pointer so we need to forget about it so
            // we don't attempt to free the module later as the execution engine will do this for
            // us
            let module_ptr = module.inner;
            mem::forget(module);

            // takes ownership of the module
            if LLVMCreateInterpreterForModule(&mut inner, module_ptr, &mut out_error) == 1 {
                let out_error = CStr::from_ptr(out_error).to_str()?;
                Err(ExecutionEngineCreateError::LLVMError(out_error.to_string()))
            } else {
                Ok(Self {
                    inner,
                    phantom: Default::default(),
                })
            }
        }
    }

    pub fn create_jit(
        module: Module<'a>,
        opt_level: u32,
    ) -> Result<ExecutionEngine, ExecutionEngineCreateError> {
        unsafe {
            // This is needed to bamboozle the linker in to not removing supposedly dead code, at
            // least according to the comments in the LLVM source code
            LLVMLinkInMCJIT();

            // Little wrapper function for initializing the native target
            target_initialization();

            // build an execution engine
            let mut inner = mem::zeroed();
            let mut out_error = mem::zeroed();

            // The execution engine takes ownership of the pointer so we need to forget about it so
            // we don't attempt to free the module later as the execution engine will do this for
            // us
            let module_ptr = module.inner;
            mem::forget(module);

            // takes ownership of the module
            if LLVMCreateJITCompilerForModule(
                &mut inner,
                module_ptr,
                opt_level as _,
                &mut out_error,
            ) == 1
            {
                let out_error = CStr::from_ptr(out_error).to_str()?;
                Err(ExecutionEngineCreateError::LLVMError(out_error.to_string()))
            } else {
                Ok(Self {
                    inner,
                    phantom: Default::default(),
                })
            }
        }
    }

    pub fn get_function_address(&self, name: &CStr) -> u64 {
        unsafe { LLVMGetFunctionAddress(self.inner, name.as_ptr()) }
    }
}

impl<'a> Drop for ExecutionEngine<'a> {
    fn drop(&mut self) {
        unsafe { LLVMDisposeExecutionEngine(self.inner) }
    }
}

unsafe fn target_initialization() {
    if cfg!(any(target_arch = "x86", target_arch = "x86_64")) {
        LLVMInitializeX86Target();
        LLVMInitializeX86TargetMC();
        LLVMInitializeX86TargetInfo();
        LLVMInitializeX86Disassembler();
        LLVMInitializeX86AsmParser();
        LLVMInitializeX86AsmPrinter();
    } else {
        panic!("Unsupported platform");
    }
}
