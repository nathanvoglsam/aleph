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

use core::mem;
use llvm_sys::core::*;
use llvm_sys::execution_engine::*;
use llvm_sys::target::*;

#[test]
pub fn llvm_test() {
    unsafe {
        // Set up a context, module and builder in that context.
        let context = LLVMContextCreate();
        let module = LLVMModuleCreateWithNameInContext(b"sum\0".as_ptr() as *const _, context);
        let builder = LLVMCreateBuilderInContext(context);

        // get a type for sum function
        let i64t = LLVMInt64TypeInContext(context);
        let mut argts = [i64t, i64t, i64t];
        let function_type = LLVMFunctionType(i64t, argts.as_mut_ptr(), argts.len() as u32, 0);

        // add it to our module
        let function = LLVMAddFunction(module, b"sum\0".as_ptr() as *const _, function_type);

        // Create a basic block in the function and set our builder to generate
        // code in it.
        let bb = LLVMAppendBasicBlockInContext(context, function, b"entry\0".as_ptr() as *const _);

        LLVMPositionBuilderAtEnd(builder, bb);

        // get the function's arguments
        let x = LLVMGetParam(function, 0);
        let y = LLVMGetParam(function, 1);
        let z = LLVMGetParam(function, 2);

        let sum = LLVMBuildAdd(builder, x, y, b"sum.1\0".as_ptr() as *const _);
        let sum = LLVMBuildAdd(builder, sum, z, b"sum.2\0".as_ptr() as *const _);

        // Emit a `ret void` into the function
        LLVMBuildRet(builder, sum);

        // done building
        LLVMDisposeBuilder(builder);

        // Dump the module as IR to stdout.
        LLVMDumpModule(module);

        // build an execution engine
        let mut ee = mem::zeroed();
        let mut out = mem::zeroed();

        // robust code should check that these calls complete successfully
        // each of these calls is necessary to setup an execution engine which compiles to native
        // code
        LLVMLinkInMCJIT();
        LLVM_InitializeNativeTarget();
        LLVM_InitializeNativeAsmPrinter();

        // takes ownership of the module
        LLVMCreateExecutionEngineForModule(&mut ee, module, &mut out);

        let addr = LLVMGetFunctionAddress(ee, b"sum\0".as_ptr() as *const _);

        let f: extern "C" fn(u64, u64, u64) -> u64 = mem::transmute(addr);

        let x: u64 = 1;
        let y: u64 = 1;
        let z: u64 = 1;
        let res = f(x, y, z);

        assert_eq!(res, x + y + z);

        // Clean up the rest.
        LLVMDisposeExecutionEngine(ee);
        LLVMContextDispose(context);
    }
}
