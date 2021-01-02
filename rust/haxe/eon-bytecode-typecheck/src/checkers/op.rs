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

use crate::checkers::BasicBlockChecker;
use crate::error::{TypeCheckError, TypeCheckResult};
use eon::function::BasicBlock;
use eon::function::Function;
use eon::indexes::OpIndex;
use eon::module::Module;
use eon::opcode::*;
use eon::type_::*;

pub struct OpChecker<'module, 'module_checker, 'function_checker, 'basic_block_checker> {
    pub basic_block_checker:
        &'basic_block_checker BasicBlockChecker<'module, 'module_checker, 'function_checker>,
    pub op: &'module OpCode,
    pub op_index: OpIndex,
}

impl<'module, 'module_checker, 'function_checker, 'basic_block_checker>
    OpChecker<'module, 'module_checker, 'function_checker, 'basic_block_checker>
{
    pub fn new(
        basic_block_checker: &'module_checker BasicBlockChecker<
            'module,
            'module_checker,
            'function_checker,
        >,
        op: &'module eon::opcode::OpCode,
        op_index: OpIndex,
    ) -> Self {
        Self {
            basic_block_checker,
            op,
            op_index,
        }
    }

    pub fn check(&self) -> TypeCheckResult<()> {
        match self.op {
            OpCode::OpMov(v) => self.check_op_mov(v)?,
            OpCode::OpInt(v) => self.check_op_int(v)?,
            OpCode::OpFloat(v) => self.check_op_float(v)?,
            OpCode::OpBool(v) => self.check_op_bool(v)?,
            OpCode::OpBytes(_) => {}
            OpCode::OpString(_) => {}
            OpCode::OpNull(_) => {}
            OpCode::OpAdd(_) => {}
            OpCode::OpSub(_) => {}
            OpCode::OpMul(_) => {}
            OpCode::OpSDiv(_) => {}
            OpCode::OpUDiv(_) => {}
            OpCode::OpSMod(_) => {}
            OpCode::OpUMod(_) => {}
            OpCode::OpShl(_) => {}
            OpCode::OpSShr(_) => {}
            OpCode::OpUShr(_) => {}
            OpCode::OpAnd(_) => {}
            OpCode::OpOr(_) => {}
            OpCode::OpXor(_) => {}
            OpCode::OpNeg(_) => {}
            OpCode::OpNot(_) => {}
            OpCode::OpIncr(_) => {}
            OpCode::OpDecr(_) => {}
            OpCode::OpCall(_) => {}
            OpCode::OpCallMethod(_) => {}
            OpCode::OpCallClosure(_) => {}
            OpCode::OpCallIntrinsic(_) => {}
            OpCode::OpInvoke(_) => {}
            OpCode::OpInvokeMethod(_) => {}
            OpCode::OpInvokeClosure(_) => {}
            OpCode::OpInvokeIntrinsic(_) => {}
            OpCode::OpStaticClosure(_) => {}
            OpCode::OpInstanceClosure(_) => {}
            OpCode::OpVirtualClosure(_) => {}
            OpCode::OpGetGlobal(_) => {}
            OpCode::OpSetGlobal(_) => {}
            OpCode::OpGetField(_) => {}
            OpCode::OpSetField(_) => {}
            OpCode::OpDynGet(_) => {}
            OpCode::OpDynSet(_) => {}
            OpCode::OpJTrue(_) => {}
            OpCode::OpJFalse(_) => {}
            OpCode::OpJNull(_) => {}
            OpCode::OpJNotNull(_) => {}
            OpCode::OpCmp(_) => {}
            OpCode::OpJAlways(_) => {}
            OpCode::OpRet(_) => {}
            OpCode::OpRetVoid => {}
            OpCode::OpSwitch(_) => {}
            OpCode::OpPhi(_) => {}
            OpCode::OpToDyn(_) => {}
            OpCode::OpToSFloat(_) => {}
            OpCode::OpToUFloat(_) => {}
            OpCode::OpToInt(_) => {}
            OpCode::OpToVirtual(_) => {}
            OpCode::OpGetI8(_) => {}
            OpCode::OpGetI16(_) => {}
            OpCode::OpGetMem(_) => {}
            OpCode::OpGetArray(_) => {}
            OpCode::OpSetI8(_) => {}
            OpCode::OpSetI16(_) => {}
            OpCode::OpSetMem(_) => {}
            OpCode::OpSetArray(_) => {}
            OpCode::OpNew(_) => {}
            OpCode::OpArraySize(_) => {}
            OpCode::OpType(_) => {}
            OpCode::OpGetType(_) => {}
            OpCode::OpGetTID(_) => {}
            OpCode::OpRef(_) => {}
            OpCode::OpUnRef(_) => {}
            OpCode::OpSetRef(_) => {}
            OpCode::OpMakeEnum(_) => {}
            OpCode::OpEnumAlloc(_) => {}
            OpCode::OpEnumIndex(_) => {}
            OpCode::OpEnumField(_) => {}
            OpCode::OpSetEnumField(_) => {}
            OpCode::OpAssert => {}
            OpCode::OpRefData(_) => {}
            OpCode::OpRefOffset(_) => {}
            OpCode::OpUnreachable => {}
            OpCode::OpNop => {}
        }
        Ok(())
    }

    fn check_op_mov(&self, op: &Load) -> TypeCheckResult<()> {
        let function = self.function();

        let assigns_type = &function[op.assigns];
        let source_type = &function[op.source];

        if assigns_type.type_ != source_type.type_ {
            return Err(self.instruction_type_error(
                "The target and source of an OpMov must both be the same type",
            ));
        }

        Ok(())
    }

    fn check_op_int(&self, op: &LoadInt) -> TypeCheckResult<()> {
        let module = self.module();
        let function = self.function();

        let assigns_type = &function[op.assigns];
        let assigns_type = &module[assigns_type.type_];

        if assigns_type != &Type::I32 {
            return Err(self.instruction_type_error("The target of an OpInt must be an I32 type"));
        }

        Ok(())
    }

    fn check_op_float(&self, op: &LoadFloat) -> TypeCheckResult<()> {
        let module = self.module();
        let function = self.function();

        let assigns_type = &function[op.assigns];
        let assigns_type = &module[assigns_type.type_];

        if assigns_type != &Type::F64 {
            return Err(self.instruction_type_error("The target of an OpFloat must be an F64 type"));
        }

        Ok(())
    }

    fn check_op_bool(&self, op: &LoadBool) -> TypeCheckResult<()> {
        let module = self.module();
        let function = self.function();

        let assigns_type = &function[op.assigns];
        let assigns_type = &module[assigns_type.type_];

        if assigns_type != &Type::Bool {
            return Err(self.instruction_type_error("The target of an OpBool must be an Bool type"));
        }

        Ok(())
    }

    pub fn instruction_type_error(&self, reason: impl Into<String>) -> TypeCheckError {
        TypeCheckError::InstructionTypeError {
            function: self.basic_block_checker.function_checker.function_index,
            basic_block: self.basic_block_checker.basic_block_index,
            op: self.op_index,
            reason: reason.into(),
        }
    }

    pub fn module(&self) -> &'module Module {
        self.basic_block_checker.module()
    }

    pub fn function(&self) -> &'module Function {
        self.basic_block_checker.function()
    }

    pub fn basic_block(&self) -> &'module BasicBlock {
        self.basic_block_checker.basic_block
    }
}
