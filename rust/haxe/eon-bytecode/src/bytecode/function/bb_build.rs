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

use crate::bytecode::function::bb_graph::BBGraph;
use crate::bytecode::function::{BasicBlock, Function, Register, RegisterMetadata, SSAValue};
use crate::bytecode::indexes::{BasicBlockIndex, InstructionIndex, RegisterIndex, TypeIndex, ValueIndex};
use crate::bytecode::module::Module;
use std::collections::{HashMap, HashSet};

pub fn build_bb(
    out: &mut Function,
    module: &Module,
    f: &hashlink_bytecode::Function,
    bb_graph: BBGraph,
    mut spans: Vec<(InstructionIndex, InstructionIndex)>,
) -> Option<()> {
    // Get the actual function type value, checking to ensure it is of the correct type category
    // (Function or Method)
    let fn_ty = &module.types[out.type_.0];
    let fn_ty = fn_ty.get_type_function()?;

    // As we go we'll be generating various bits of metadata about the transcoded instructions
    let registers = vec![Register::default(); f.registers.len()];
    let register_map = Vec::new();
    let basic_block_registers_read = Vec::new();
    let basic_block_registers_written = Vec::new();
    let mut reg_meta = RegisterMetadata {
        registers,
        register_map,
        basic_block_registers_read,
        basic_block_registers_written,
    };

    // Go over the function arguments and check that the types in the signature match the registers
    // in the actual function definition while inserting the SSA values for them at the same time
    for (i, arg_ty) in fn_ty.args.iter().enumerate() {
        // Get the type for the register that matches the function argument
        let reg_ty = f.registers[i] as usize;

        // Error if the types don't match
        if *arg_ty != reg_ty {
            return None;
        }

        // Insert an SSA value for this argument that points to the first instruction in the first
        // basic block. The first instruction will always be a special no-op type instruction so
        // that bb: 0 and instr: 0 can be used as a marker for function arguments.
        out.ssa_values.push(SSAValue {
            type_: TypeIndex(reg_ty),
            basic_block: BasicBlockIndex::default(),
            instruction: InstructionIndex::default(),
        });

        // Insert the information to map the SSA value back to the register it refers to
        reg_meta.register_map.push(RegisterIndex(i));
    }

    // The spans array is a list of ranges into the source hashlink bytecode. Each entry encodes the
    // span of instructions that should be encoded into a basic block.
    //
    // Instruction index 0 in the source bytecode is special as it **must** be the first instruction
    // of the first basic block (the function entry point). The algorithm that generates the spans
    // list does not guarantee that the first span corresponds to the first instruction in the
    // source.
    //
    // To fix this we sort the array so that the span for instruction 0 will be the first item in
    // the array, meaning we can transparently handle all of them and it will always be the first
    // basic block
    spans.sort_unstable_by(|a, b| a.0.cmp(&b.0));

    // Now we need to build information about the registers read and written by each basic block so
    // we can use it to produce the final SSA form instruction stream
    for span in &spans {
        let lower_bound = span.0.0;
        let upper_bound = span.1.0;
        let ops = &f.ops[lower_bound..=upper_bound];
        for op in ops {
            let mut reg_reads = HashSet::new();
            let mut reg_writes = HashMap::new();

            if let Some(reads) = op.register_reads() {
                for read in reads {
                    reg_reads.insert(RegisterIndex(read as usize));
                }
            }
            if let Some(write) = op.register_write() {
                reg_writes.insert(RegisterIndex(write as usize), ValueIndex(0));
            }

            reg_meta.basic_block_registers_read.push(reg_reads);
            reg_meta.basic_block_registers_written.push(reg_writes);
        }
    }

    // The next phase requires us to identify all distinct SSA form values for each register in the
    // source bytecode. This requires iterating over the flat source code (no basic block info yet)
    // and identifying any write instructions.
    //
    // The original bytecode has mutable registers so any write to a given register must be
    // translated into the creation of a new SSA value that is associated with that register. Any
    // usage of the register after a write should be directed at the new SSA value created by the
    // write.
    //
    // Later we will also need to identify the last write made to a register within each basic block
    // so that we can correctly insert our phi instructions.
    for op in &f.ops {
        if let Some(write) = op.register_write() {}
    }

    out.metadata.reg_data = Some(reg_meta);

    Some(())
}

/// Find the span, in the given list, that holds the given instruction index
fn find_source_span(spans: &[(usize, usize)], i: usize) -> Option<usize> {
    spans
        .iter()
        .enumerate()
        .find(|(_, v)| v.0 <= i && v.1 >= i)
        .map(|(i, _)| i)
}

/// Simple function that handles creating and adding SSA values for instructions
fn handle_ssa_write(out: &mut Function, f: &hashlink_bytecode::Function, v: i32) {
    let type_ = f.registers[v as usize] as usize;
    out.ssa_values.push(SSAValue {
        type_: TypeIndex(type_),
        basic_block: Default::default(),
        instruction: Default::default(),
    });
}
