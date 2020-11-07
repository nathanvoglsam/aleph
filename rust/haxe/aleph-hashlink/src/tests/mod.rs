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
use llvm_sys::prelude::*;
use llvm_sys::target::*;
use llvm_sys::LLVMIntPredicate;
use std::os::raw::c_char;

const EMPTY_STRING: *const c_char = b"\0".as_ptr() as *const c_char;

#[test]
pub fn llvm_test() {
    unsafe {
        // Set up a context and module in that context.
        let context = LLVMContextCreate();
        let module = LLVMModuleCreateWithNameInContext(EMPTY_STRING, context);

        llvm_test_1(context, module);
        llvm_test_2(context, module);

        // Dump the module as IR to stdout.
        LLVMDumpModule(module);

        // build an execution engine
        let mut ee = mem::zeroed();
        let mut out = mem::zeroed();

        // robust code should check that these calls complete successfully
        // each of these calls is necessary to setup an execution engine which compiles to native
        // code
        LLVMLinkInMCJIT();
        LLVMInitializeX86Target();
        LLVMInitializeX86TargetMC();
        LLVMInitializeX86TargetInfo();
        LLVMInitializeX86Disassembler();
        LLVMInitializeX86AsmParser();
        LLVMInitializeX86AsmPrinter();

        // takes ownership of the module
        LLVMCreateExecutionEngineForModule(&mut ee, module, &mut out);

        llvm_test_1_exec(ee);
        llvm_test_2_exec(ee);

        // Clean up the rest.
        LLVMDisposeExecutionEngine(ee);
        LLVMContextDispose(context);
    }
}

unsafe fn llvm_test_1(context: LLVMContextRef, module: LLVMModuleRef) {
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

    let sum = LLVMBuildAdd(builder, x, y, EMPTY_STRING);
    let sum = LLVMBuildAdd(builder, sum, z, EMPTY_STRING);

    // Emit a `ret void` into the function
    LLVMBuildRet(builder, sum);

    LLVMDisposeBuilder(builder);
}

unsafe fn llvm_test_1_exec(ee: LLVMExecutionEngineRef) {
    let addr = LLVMGetFunctionAddress(ee, b"sum\0".as_ptr() as *const _);

    let f: extern "C" fn(u64, u64, u64) -> u64 = mem::transmute(addr);

    let x: u64 = 1;
    let y: u64 = 1;
    let z: u64 = 1;
    let res = f(x, y, z);

    assert_eq!(res, x + y + z);
}

unsafe fn llvm_test_2(context: LLVMContextRef, module: LLVMModuleRef) {
    let builder = LLVMCreateBuilderInContext(context);

    // Create the three types we care about, int64, void and a pointer to int64
    let i64t = LLVMInt64TypeInContext(context);
    let voidt = LLVMVoidTypeInContext(context);
    let i64_ptr = LLVMPointerType(i64t, 0);

    // Build the function signature
    let mut argts = [i64_ptr, i64t, i64t];
    let function_type = LLVMFunctionType(voidt, argts.as_mut_ptr(), argts.len() as u32, 0);

    // Add the function to the module
    let function = LLVMAddFunction(module, b"add_loop\0".as_ptr() as *const _, function_type);

    // Create the four basic blocks we need
    let entry = LLVMAppendBasicBlockInContext(context, function, EMPTY_STRING);
    let loop_check = LLVMAppendBasicBlockInContext(context, function, EMPTY_STRING);
    let loop_body = LLVMAppendBasicBlockInContext(context, function, EMPTY_STRING);
    let end = LLVMAppendBasicBlockInContext(context, function, EMPTY_STRING);

    // Create the constants we need
    let constant_one = LLVMConstInt(i64t, 1, 1);
    let constant_zero = LLVMConstInt(i64t, 0, 1);

    // Retrieve the LLVMValue pointers for the function parameters
    let ptr_arg = LLVMGetParam(function, 0);
    let add_arg = LLVMGetParam(function, 1);
    let loop_arg = LLVMGetParam(function, 2);

    // Build the entry basic block, which just loads the value from the pointer and jumps to the
    // start of the loop
    LLVMPositionBuilderAtEnd(builder, entry);
    let initial_load = LLVMBuildLoad(builder, ptr_arg, EMPTY_STRING);
    LLVMBuildBr(builder, loop_check);

    // Build the loop condition check block, which loads values with phi instructions and then
    // checks if the loop condition is no longer valid where it decides to continue looping or
    // to jump to the end
    LLVMPositionBuilderAtEnd(builder, loop_check);
    let check_phi_1 = LLVMBuildPhi(builder, i64t, EMPTY_STRING);
    let check_phi_2 = LLVMBuildPhi(builder, i64t, EMPTY_STRING);
    let compare = LLVMBuildICmp(
        builder,
        LLVMIntPredicate::LLVMIntSLT,
        check_phi_2,
        loop_arg,
        EMPTY_STRING,
    );
    LLVMBuildCondBr(builder, compare, loop_body, end);

    // Build the loop block, which gets values with phi instructions, adds arg[1] to the
    // accumulator, increments the loop counter then jumps back to the loop condition check
    LLVMPositionBuilderAtEnd(builder, loop_body);
    let loop_phi_1 = LLVMBuildPhi(builder, i64t, EMPTY_STRING);
    let loop_phi_2 = LLVMBuildPhi(builder, i64t, EMPTY_STRING);
    let add = LLVMBuildAdd(builder, loop_phi_1, add_arg, EMPTY_STRING);
    let loop_counter = LLVMBuildAdd(builder, loop_phi_2, constant_one, EMPTY_STRING);
    LLVMBuildBr(builder, loop_check);

    // Build the exit block, which stores the final value back in to the pointer and then
    // returns from the function
    LLVMPositionBuilderAtEnd(builder, end);
    let end_phi = LLVMBuildPhi(builder, i64t, EMPTY_STRING);
    LLVMBuildStore(builder, end_phi, ptr_arg);
    LLVMBuildRetVoid(builder);

    // Now we need to patch all our phi instructions
    let mut check_phi_blocks = [entry, loop_body];
    let mut check_phi_1_vals = [initial_load, add];
    let mut check_phi_2_vals = [constant_zero, loop_counter];
    LLVMAddIncoming(
        check_phi_1,
        check_phi_1_vals.as_mut_ptr(),
        check_phi_blocks.as_mut_ptr(),
        2,
    );
    LLVMAddIncoming(
        check_phi_2,
        check_phi_2_vals.as_mut_ptr(),
        check_phi_blocks.as_mut_ptr(),
        2,
    );
    let mut loop_phi_blocks = [loop_check];
    let mut loop_phi_1_vals = [check_phi_1];
    let mut loop_phi_2_vals = [check_phi_2];
    LLVMAddIncoming(
        loop_phi_1,
        loop_phi_1_vals.as_mut_ptr(),
        loop_phi_blocks.as_mut_ptr(),
        1,
    );
    LLVMAddIncoming(
        loop_phi_2,
        loop_phi_2_vals.as_mut_ptr(),
        loop_phi_blocks.as_mut_ptr(),
        1,
    );
    let mut end_phi_blocks = [loop_check];
    let mut end_phi_vals = [check_phi_1];
    LLVMAddIncoming(
        end_phi,
        end_phi_vals.as_mut_ptr(),
        end_phi_blocks.as_mut_ptr(),
        1,
    );

    LLVMDisposeBuilder(builder);
}

unsafe fn llvm_test_2_exec(ee: LLVMExecutionEngineRef) {
    let addr = LLVMGetFunctionAddress(ee, b"add_loop\0".as_ptr() as *const _);

    let f: extern "C" fn(*mut i64, i64, i64) = mem::transmute(addr);

    let old_val = 1;
    let mut val = old_val;
    let add = 1;
    let loop_count = 1;
    f(&mut val as *mut _, add, loop_count);

    assert_eq!(val, old_val + (add * loop_count));
}
