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

use crate::error::TranspileResult;
use crate::indexes::InstructionIndex;
use eon_bytecode::{
    Alloca, BasicBlock, BasicBlockIndex, Function, Module, OpCode, RegisterIndex, SSAValue,
    TypeIndex, TypeParam,
};
use std::collections::{HashMap, HashSet};

struct PrePassInfo {
    pub on_stack_registers: HashMap<RegisterIndex, (TypeIndex, TypeIndex)>,
}

struct TranslatePassInfo {
    pub function: Function,
}

struct QueueItem {
    pub predecessor: Option<InstructionIndex>,
    pub start: InstructionIndex,
    pub trap_stack: Vec<InstructionIndex>,
}

impl QueueItem {
    fn initial() -> Self {
        Self {
            predecessor: None,
            start: 0.into(),
            trap_stack: Vec::new(),
        }
    }

    fn next(&self, predecessor: InstructionIndex, start: InstructionIndex) -> Self {
        Self {
            predecessor: Some(predecessor),
            start,
            trap_stack: self.trap_stack.clone(),
        }
    }

    fn next_no_predecessor(&self, start: InstructionIndex) -> Self {
        Self {
            predecessor: None,
            start,
            trap_stack: self.trap_stack.clone(),
        }
    }

    fn next_popped(&self, predecessor: InstructionIndex, start: InstructionIndex) -> Self {
        let mut trap_stack = self.trap_stack.clone();
        trap_stack.pop();

        Self {
            predecessor: Some(predecessor),
            start,
            trap_stack,
        }
    }

    fn next_pushed(
        &self,
        predecessor: InstructionIndex,
        start: InstructionIndex,
        handler: InstructionIndex,
    ) -> NextItem {
        let mut trap_stack = self.trap_stack.clone();
        trap_stack.push(handler);

        Self {
            predecessor: Some(predecessor),
            start,
            trap_stack,
        }
    }
}

/// This is an internal struct that implements the function translation
pub struct FunctionTranslator<'module, 'callable_table> {
    pub module: &'module Module,
    pub callable_table: &'callable_table [hashlink::Callable],
}

impl<'module, 'callable_table> FunctionTranslator<'module, 'callable_table> {
    pub fn new(
        module: &'module Module,
        callable_table: &'callable_table [hashlink::Callable],
    ) -> Self {
        Self {
            module,
            callable_table,
        }
    }

    pub fn translate_function(&self, mut source: hashlink::Function) -> TranspileResult<Function> {
        // This is a very, very, very, very hacky thing we inject into the instruction stream of
        // every function we translate.
        //
        // An unconditional jump with an offset of 0 is a no-op as the execution semantics are
        // exactly equal to OpNop. This little hack serves one purpose, it guarantees that every
        // function begins with a basic block with no predecessors and ends with an unconditional
        // jump.
        //
        // The purpose of this hack is to simplify our "mem2reg" implementation when dealing with
        // function arguments. HashLink implicitly uses the 0..n registers for 0..n function
        // arguments. There is no sane way to represent this kind of assignment semantics in the SSA
        // graph we calculate in the next step without forcing the entire algorithm to work around
        // this one edge case.
        //
        // The problem stems from the fact that it is invalid, under our translation, for the
        // *entry* basic block to be the target of a branch, as we can't encode phi instructions
        // that import the values from the function arguments without introducing more painful edge
        // cases to the instruction encoding. HashLink bytecode makes no guarantee, and actively has
        // code generated, that violates this requirement of our translation (if we translated
        // directly without this hack anyway).
        //
        // The solution, just add a no-op branch at the start of the function. This, essentially,
        // just explicitly encodes the edge of the execution graph that was otherwise only
        // implicitly represented. There is an implicit execution graph edge from the caller into
        // the callee, which is the execution edge that the function argument's SSA values are
        // imported from. This empty basic block will encode that edge explicitly so we don't need
        // to handle any edge cases when emitting phi instructions for branch target blocks (there's
        // no sane way to handle having the first instruction be a branch target otherwise).
        let noop_jump = hashlink::OpOneParam { param_1: 0 };
        let noop_jump = hashlink::OpCode::OpJAlways(noop_jump);
        source.ops.insert(0, noop_jump);

        let pre_pass_info = self.pre_pass(&source)?;
        let translate_pass_info = self.translate_pass(pre_pass_info, source)?;
        let function = self.remap_pass(translate_pass_info)?;

        Ok(function)
    }

    fn pre_pass(&self, source: &hashlink::Function) -> TranspileResult<PrePassInfo> {
        let on_stack_registers = self.on_stack_registers(source);

        Ok(PrePassInfo { on_stack_registers })
    }

    fn translate_pass(
        &self,
        pre_pass_info: PrePassInfo,
        source: hashlink::Function,
    ) -> TranspileResult<TranslatePassInfo> {
        // A set/map of virtual registers that are created when a source HashLink opcode must be
        // translated into multiple eon opcodes
        let mut virtual_registers = HashMap::new();

        // A set/map of the branch destinations that have already been handled once.
        let mut destination_to_block = HashMap::new();

        let mut values = Vec::new();
        let mut basic_blocks = Vec::new();

        // Create the item queue and push the initial entry
        let mut queue = Vec::new();
        queue.push(QueueItem::initial());

        while let Some(current) = queue.pop() {
            let mut ops = Vec::new();

            if let Some(existing_block) = self.is_handled(&destination_to_block, &current) {
                self.add_to_predecessor_set(existing_block, &current);
                continue;
            }

            // When we handle the first basic block we need to inject stack allocations
            if current.start.0 == 0 {
                self.generate_alloca_ops(&pre_pass_info, &mut values, &mut ops);
            }

            for (i, source_op) in source.ops[current.start.0..].iter().enumerate() {}

            self.mark_handled(&mut destination_to_block, &current, Default::default());
            basic_blocks.push(BasicBlock { ops });
        }

        Ok(TranslatePassInfo {
            function: Function {
                type_: TypeIndex(source.type_ as usize),
                ssa_values: values,
                basic_blocks,
            },
        })
    }

    fn remap_pass(&self, translate_pass_info: TranslatePassInfo) -> TranspileResult<Function> {
        Ok(translate_pass_info.function)
    }

    /// This function calculates the set of registers that *must* be stored in stack memory.
    ///
    /// This could be for a variety of reasons, but mostly for when the address of the register
    /// needs to be taken
    fn on_stack_registers(
        &self,
        source: &hashlink::Function,
    ) -> HashMap<RegisterIndex, (TypeIndex, TypeIndex)> {
        source
            .ops
            .iter()
            .filter_map(|v| match v {
                hashlink::OpCode::OpRef(v) => {
                    let addr = RegisterIndex(v.param_1 as usize);
                    let addr_of = RegisterIndex(v.param_2 as usize);

                    let addr_type = TypeIndex(source.registers[addr.0] as usize);
                    let addr_of_type = TypeIndex(source.registers[addr_of.0] as usize);

                    Some((addr_of, (addr_type, addr_of_type)))
                }
                _ => None,
            })
            .collect()
    }

    fn generate_alloca_ops(
        &self,
        pre_pass_info: &PrePassInfo,
        values: &mut Vec<SSAValue>,
        ops: &mut Vec<OpCode>,
    ) {
        for (addr_of, (addr_type, addr_of_type)) in pre_pass_info.on_stack_registers {
            let op = OpCode::OpAlloca(Alloca {
                assigns: Default::default(),
                type_: addr_of_type,
            });
            ops.push(op);
        }
    }

    fn mark_handled(
        &self,
        destination_to_block: &mut HashMap<InstructionIndex, BasicBlockIndex>,
        current: &QueueItem,
        block: BasicBlockIndex,
    ) {
        let previous = destination_to_block.insert(current.start, block);
        debug_assert!(previous.is_none());
    }

    fn is_handled(
        &self,
        destination_to_block: &HashMap<InstructionIndex, BasicBlockIndex>,
        current: &QueueItem,
    ) -> Option<BasicBlockIndex> {
        destination_to_block.get(&current.start).cloned()
    }

    /// This function is called when we attempt to handle a branch target a second time (i.e, the
    /// same instruction is a target for multiple branches).
    ///
    /// When we reach the same instruction from a branch a second time we do not need to translate
    /// it a second time, but we do need to add the block that got us here to the predecessor set.
    ///
    /// TODO: Might not need this in new algorithm
    fn add_to_predecessor_set(&self, _existing_block: BasicBlockIndex, _current: &QueueItem) {
        //if let Some(predecessor) = _current.predecessor {
        //    predecessors[_existing_block.0].insert(predecessor);
        //}
    }
}
