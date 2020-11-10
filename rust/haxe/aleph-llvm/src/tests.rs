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

use crate::{Context, ExecutionEngine, IntPredicate, Module};
use core::mem;
use std::ffi::CString;

#[test]
pub fn llvm_test_jit() {
    let context = Context::init();
    let module = context.create_module(None);

    llvm_test_1(&context, &module);
    llvm_test_2(&context, &module);

    module.dump();
    let ee = ExecutionEngine::create_jit(module, 3).unwrap();

    llvm_test_1_exec_jit(&ee);
    llvm_test_2_exec_jit(&ee);
}

#[test]
pub fn llvm_test_interp() {
    let context = Context::init();
    let module = context.create_module(None);

    llvm_test_1(&context, &module);
    llvm_test_2(&context, &module);

    module.dump();
    let ee = ExecutionEngine::create_jit(module, 3).unwrap();

    llvm_test_1_exec_jit(&ee);
    llvm_test_2_exec_jit(&ee);
}

fn llvm_test_1(context: &Context, module: &Module) {
    let builder = context.create_builder();

    // get a type for sum function
    let i64t = context.create_i64_type();
    let args = [i64t, i64t, i64t];
    let func_type = context.create_function_type(i64t, &args, false);

    // add it to our module
    let name = String::from("sum");
    let name = CString::new(name).unwrap();
    let func = module.add_function(Some(&name), func_type);

    // Create a basic block in the function and set our builder to generate
    // code in it.
    let basic_block = context.append_basic_block(func, None);
    builder.position_at_end(basic_block);

    // get the function's arguments
    let x = func.get_param(0);
    let y = func.get_param(1);
    let z = func.get_param(2);

    let sum = builder.build_add(x, y, None);
    let sum = builder.build_add(sum, z, None);

    // Emit a `ret void` into the function
    builder.build_ret(sum);
}

fn llvm_test_1_exec_jit(ee: &ExecutionEngine) {
    let name = String::from("sum");
    let name = CString::new(name).unwrap();
    let addr = ee.get_function_address(&name);

    let f: extern "C" fn(u64, u64, u64) -> u64 = unsafe { mem::transmute(addr) };

    let x: u64 = 1;
    let y: u64 = 1;
    let z: u64 = 1;
    let res = f(x, y, z);

    assert_eq!(res, x + y + z);
}

fn llvm_test_2(context: &Context, module: &Module) {
    let builder = context.create_builder();

    // Create the three types we care about, int64, void and a pointer to int64
    let i64t = context.create_i64_type();
    let voidt = context.create_void_type();
    let i64_ptr = context.create_ptr_type(i64t, 0);

    // Build the function signature
    let argts = [i64_ptr, i64t, i64t];
    let func_type = context.create_function_type(voidt, &argts, false);

    // Add the function to the module
    let name = String::from("add_loop");
    let name = CString::new(name).unwrap();
    let func = module.add_function(Some(&name), func_type);

    // Create the four basic blocks we need
    let entry = context.append_basic_block(func, None);
    let loop_check = context.append_basic_block(func, None);
    let loop_body = context.append_basic_block(func, None);
    let end = context.append_basic_block(func, None);

    // Create the constants we need
    let constant_one = context.create_int_constant(i64t, 1, true);
    let constant_zero = context.create_int_constant(i64t, 0, true);

    // Retrieve the LLVMValue pointers for the function parameters
    let ptr_arg = func.get_param(0);
    let add_arg = func.get_param(1);
    let loop_arg = func.get_param(2);

    // Build the entry basic block, which just loads the value from the pointer and jumps to the
    // start of the loop
    builder.position_at_end(entry);
    let initial_load = builder.build_load(ptr_arg, None);
    builder.build_br(loop_check);

    // Build the loop condition check block, which loads values with phi instructions and then
    // checks if the loop condition is no longer valid where it decides to continue looping or
    // to jump to the end
    builder.position_at_end(loop_check);
    let check_phi_1 = builder.build_phi(i64t, None);
    let check_phi_2 = builder.build_phi(i64t, None);
    let compare = builder.build_icmp(IntPredicate::LLVMIntSLT, check_phi_2, loop_arg, None);
    builder.build_cond_br(compare, loop_body, end);

    // Build the loop block, which gets values with phi instructions, adds arg[1] to the
    // accumulator, increments the loop counter then jumps back to the loop condition check
    builder.position_at_end(loop_body);
    let loop_phi_1 = builder.build_phi(i64t, None);
    let loop_phi_2 = builder.build_phi(i64t, None);
    let add = builder.build_add(loop_phi_1, add_arg, None);
    let loop_counter = builder.build_add(loop_phi_2, constant_one, None);
    builder.build_br(loop_check);

    // Build the exit block, which stores the final value back in to the pointer and then
    // returns from the function
    builder.position_at_end(end);
    let end_phi = builder.build_phi(i64t, None);
    builder.build_store(end_phi, ptr_arg);
    builder.build_ret_void();

    // Now we need to patch all our phi instructions
    let check_phi_blocks = [entry, loop_body];
    let check_phi_1_values = [initial_load, add];
    let check_phi_2_values = [constant_zero, loop_counter];
    check_phi_1.add_incoming(&check_phi_1_values, &check_phi_blocks);
    check_phi_2.add_incoming(&check_phi_2_values, &check_phi_blocks);

    let loop_phi_blocks = [loop_check];
    let loop_phi_1_values = [check_phi_1];
    let loop_phi_2_values = [check_phi_2];
    loop_phi_1.add_incoming(&loop_phi_1_values, &loop_phi_blocks);
    loop_phi_2.add_incoming(&loop_phi_2_values, &loop_phi_blocks);

    let end_phi_blocks = [loop_check];
    let end_phi_values = [check_phi_1];
    end_phi.add_incoming(&end_phi_values, &end_phi_blocks);
}

fn llvm_test_2_exec_jit(ee: &ExecutionEngine) {
    let name = String::from("add_loop");
    let name = CString::new(name).unwrap();
    let addr = ee.get_function_address(&name);

    let f: extern "C" fn(*mut i64, i64, i64) = unsafe { mem::transmute(addr) };

    let old_val = 1;
    let mut val = old_val;
    let add = 1;
    let loop_count = 1;
    f(&mut val as *mut _, add, loop_count);

    assert_eq!(val, old_val + (add * loop_count));
}
