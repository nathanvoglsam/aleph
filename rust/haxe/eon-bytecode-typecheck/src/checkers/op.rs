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
use eon::indexes::*;
use eon::module::dump::get_type_name;
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
            OpCode::OpMov(op) => self.check_op_mov(op)?,
            OpCode::OpInt(op) => self.check_op_int(op)?,
            OpCode::OpFloat(op) => self.check_op_float(op)?,
            OpCode::OpBool(op) => self.check_op_bool(op)?,
            OpCode::OpBytes(op) => self.check_op_bytes(op)?,
            OpCode::OpString(op) => self.check_op_string(op)?,
            OpCode::OpNull(op) => self.check_op_null(op)?,
            OpCode::OpAdd(op) => self.check_op_arith_binop(op, true)?,
            OpCode::OpSub(op) => self.check_op_arith_binop(op, true)?,
            OpCode::OpMul(op) => self.check_op_arith_binop(op, true)?,
            OpCode::OpSDiv(op) => self.check_op_arith_binop(op, true)?,
            OpCode::OpUDiv(op) => self.check_op_arith_binop(op, false)?,
            OpCode::OpSMod(op) => self.check_op_arith_binop(op, false)?,
            OpCode::OpUMod(op) => self.check_op_arith_binop(op, false)?,
            OpCode::OpShl(op) => self.check_op_bitwise_binop(op)?,
            OpCode::OpSShr(op) => self.check_op_bitwise_binop(op)?,
            OpCode::OpUShr(op) => self.check_op_bitwise_binop(op)?,
            OpCode::OpAnd(op) => self.check_op_bitwise_binop(op)?,
            OpCode::OpOr(op) => self.check_op_bitwise_binop(op)?,
            OpCode::OpXor(op) => self.check_op_bitwise_binop(op)?,
            OpCode::OpNeg(op) => self.check_op_arith_unop(op, true)?,
            OpCode::OpNot(op) => self.check_op_bitwise_unop(op)?,
            OpCode::OpIncr(op) => self.check_op_arith_unop(op, false)?,
            OpCode::OpDecr(op) => self.check_op_arith_unop(op, false)?,
            OpCode::OpCall(op) => self.check_op_call(op)?,
            OpCode::OpCallMethod(op) => self.check_op_call_method(op)?,
            OpCode::OpCallClosure(op) => self.check_op_call_closure(op)?,
            OpCode::OpCallIntrinsic(op) => self.check_op_call_intrinsic(op)?,
            OpCode::OpInvoke(op) => self.check_op_invoke(op)?,
            OpCode::OpInvokeMethod(op) => self.check_op_invoke_method(op)?,
            OpCode::OpInvokeClosure(op) => self.check_op_invoke_closure(op)?,
            OpCode::OpInvokeIntrinsic(op) => self.check_op_invoke_intrinsic(op)?,
            OpCode::OpStaticClosure(_) => {}
            OpCode::OpInstanceClosure(_) => {}
            OpCode::OpVirtualClosure(_) => {}
            OpCode::OpGetGlobal(_) => {}
            OpCode::OpSetGlobal(_) => {}
            OpCode::OpGetField(_) => {}
            OpCode::OpSetField(_) => {}
            OpCode::OpDynGet(_) => {}
            OpCode::OpDynSet(_) => {}
            OpCode::OpJTrue(op) => self.check_op_cond_branch(op)?,
            OpCode::OpJFalse(op) => self.check_op_cond_branch(op)?,
            OpCode::OpJNull(op) => self.check_op_cond_branch_null_check(op)?,
            OpCode::OpJNotNull(op) => self.check_op_cond_branch_null_check(op)?,
            OpCode::OpCmp(op) => self.check_op_cmp(op)?,
            OpCode::OpJAlways(_) => {}
            OpCode::OpRet(_) => {}
            OpCode::OpSwitch(_) => {}
            OpCode::OpPhi(op) => self.check_op_phi(op)?,
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
        let module = self.module();
        let function = self.function();

        let assigns_val = &function[op.assigns];
        let source_val = &function[op.source];

        if assigns_val.type_ != source_val.type_ {
            let assigns_name = get_type_name(module, &module[assigns_val.type_]).unwrap();
            let source_name = get_type_name(module, &module[assigns_val.type_]).unwrap();
            let reason = format!(
                "The target and source of an OpMov must both be the same type. Actually {} and {}",
                assigns_name, source_name
            );

            return Err(self.instruction_type_error(reason));
        }

        Ok(())
    }

    fn check_op_int(&self, op: &LoadInt) -> TypeCheckResult<()> {
        let module = self.module();
        let function = self.function();

        let assigns_type = &function[op.assigns];
        let assigns_type = &module[assigns_type.type_];

        if !assigns_type.is_integer() {
            let assigns_name = get_type_name(module, assigns_type).unwrap();
            let reason = format!(
                "The target of an OpInt must be an integer type. Actually {}",
                assigns_name
            );
            return Err(self.instruction_type_error(reason));
        }

        Ok(())
    }

    fn check_op_float(&self, op: &LoadFloat) -> TypeCheckResult<()> {
        let module = self.module();
        let function = self.function();

        let assigns_type = &function[op.assigns];
        let assigns_type = &module[assigns_type.type_];

        if !assigns_type.is_floating_point() {
            let assigns_name = get_type_name(module, assigns_type).unwrap();
            let reason = format!(
                "The target of an OpFloat must be an F64 or F32 type. Actually {}",
                assigns_name
            );
            return Err(self.instruction_type_error(reason));
        }

        Ok(())
    }

    fn check_op_bool(&self, op: &LoadBool) -> TypeCheckResult<()> {
        let module = self.module();
        let function = self.function();

        let assigns_type = &function[op.assigns];
        let assigns_type = &module[assigns_type.type_];

        if assigns_type != &Type::Bool {
            let assigns_name = get_type_name(module, assigns_type).unwrap();
            let reason = format!(
                "The target of an OpBool must be a Bool type. Actually {}",
                assigns_name
            );
            return Err(self.instruction_type_error(reason));
        }

        Ok(())
    }

    fn check_op_bytes(&self, op: &LoadBytes) -> TypeCheckResult<()> {
        let module = self.module();
        let function = self.function();

        let assigns_type = &function[op.assigns];
        let assigns_type = &module[assigns_type.type_];

        if assigns_type != &Type::Bytes {
            let assigns_name = get_type_name(module, assigns_type).unwrap();
            let reason = format!(
                "The target of an OpBytes must be a Bytes type. Actually {}",
                assigns_name
            );
            return Err(self.instruction_type_error(reason));
        }

        Ok(())
    }

    fn check_op_string(&self, op: &LoadString) -> TypeCheckResult<()> {
        let module = self.module();
        let function = self.function();

        let assigns_type = &function[op.assigns];
        let assigns_type = &module[assigns_type.type_];

        if assigns_type != &Type::Bytes {
            let assigns_name = get_type_name(module, assigns_type).unwrap();
            let reason = format!(
                "The target of an OpString must be a Bytes type. Actually {}",
                assigns_name
            );
            return Err(self.instruction_type_error(reason));
        }

        Ok(())
    }

    fn check_op_null(&self, op: &ValueIndex) -> TypeCheckResult<()> {
        let module = self.module();
        let function = self.function();

        let assigns_type = &function[*op];
        let assigns_type = &module[assigns_type.type_];

        if !assigns_type.is_nullable() {
            let assigns_name = get_type_name(module, assigns_type).unwrap();
            let reason = format!(
                "The target of an OpNull must be a nullable type. Actually {}",
                assigns_name
            );
            return Err(self.instruction_type_error(reason));
        }

        Ok(())
    }

    fn check_op_arith_binop(&self, op: &Binop, allow_float: bool) -> TypeCheckResult<()> {
        let module = self.module();
        let function = self.function();

        let assigns_val = &function[op.assigns];
        let assigns_type = &module[assigns_val.type_];

        let lhs_val = &function[op.lhs];
        let lhs_type = &module[lhs_val.type_];

        let rhs_val = &function[op.rhs];
        let rhs_type = &module[rhs_val.type_];

        let assigns_name = get_type_name(module, assigns_type).unwrap();
        let lhs_name = get_type_name(module, lhs_type).unwrap();
        let rhs_name = get_type_name(module, rhs_type).unwrap();

        if !allow_float {
            if lhs_type.is_floating_point()
                || rhs_type.is_floating_point()
                || assigns_type.is_floating_point()
            {
                let reason = format!(
                    "The arithmetic opcode is not valid with floating point operands. Actually {}, {}, {}",
                    assigns_name, lhs_name, rhs_name
                );
                return Err(self.instruction_type_error(reason));
            }
        }

        if !lhs_type.is_numeric() || !rhs_type.is_numeric() || !assigns_type.is_numeric() {
            let reason = format!(
                "All operands for an arithmetic operation must be numeric. Actually {}, {}, {}",
                assigns_name, lhs_name, rhs_name
            );
            return Err(self.instruction_type_error(reason));
        }

        if lhs_val.type_ != rhs_val.type_
            || lhs_val.type_ != assigns_val.type_
            || rhs_val.type_ != assigns_val.type_
        {
            let reason = format!(
                "All operands for an arithmetic operation must be the same type. Actually {}, {}, {}",
                assigns_name, lhs_name, rhs_name
            );
            return Err(self.instruction_type_error(reason));
        }

        Ok(())
    }

    fn check_op_arith_unop(&self, op: &Unop, allow_float: bool) -> TypeCheckResult<()> {
        let module = self.module();
        let function = self.function();

        let assigns_val = &function[op.assigns];
        let assigns_type = &module[assigns_val.type_];

        let operand_val = &function[op.operand];
        let operand_type = &module[operand_val.type_];

        let assigns_name = get_type_name(module, assigns_type).unwrap();
        let operand_name = get_type_name(module, operand_type).unwrap();

        if !allow_float {
            if operand_type.is_floating_point() || assigns_type.is_floating_point() {
                let reason = format!(
                    "The arithmetic opcode is not valid with floating point operands. Actually {}, {}",
                    assigns_name, operand_name
                );
                return Err(self.instruction_type_error(reason));
            }
        }

        if !operand_type.is_numeric() || !assigns_type.is_numeric() {
            let reason = format!(
                "All operands for an arithmetic operation must be numeric. Actually {}, {}",
                assigns_name, operand_name
            );
            return Err(self.instruction_type_error(reason));
        }

        if operand_val.type_ != assigns_val.type_ {
            let reason = format!(
                "All operands for an arithmetic operation must be the same type. Actually {}, {}",
                assigns_name, operand_name
            );
            return Err(self.instruction_type_error(reason));
        }

        Ok(())
    }

    fn check_op_bitwise_binop(&self, op: &Binop) -> TypeCheckResult<()> {
        let module = self.module();
        let function = self.function();

        let assigns_val = &function[op.assigns];
        let assigns_type = &module[assigns_val.type_];

        let lhs_val = &function[op.lhs];
        let lhs_type = &module[lhs_val.type_];

        let rhs_val = &function[op.rhs];
        let rhs_type = &module[rhs_val.type_];

        let assigns_name = get_type_name(module, assigns_type).unwrap();
        let lhs_name = get_type_name(module, lhs_type).unwrap();
        let rhs_name = get_type_name(module, rhs_type).unwrap();

        if lhs_type.is_floating_point()
            || rhs_type.is_floating_point()
            || assigns_type.is_floating_point()
        {
            let reason = format!(
                "Bitwise opcodes not valid with floating point operands. Actually {}, {}, {}",
                assigns_name, lhs_name, rhs_name
            );
            return Err(self.instruction_type_error(reason));
        }

        if !lhs_type.is_numeric() || !rhs_type.is_numeric() || !assigns_type.is_numeric() {
            let reason = format!(
                "All operands for a bitwise operation must be numeric. Actually {}, {}, {}",
                assigns_name, lhs_name, rhs_name
            );
            return Err(self.instruction_type_error(reason));
        }

        if lhs_val.type_ != rhs_val.type_
            || lhs_val.type_ != assigns_val.type_
            || rhs_val.type_ != assigns_val.type_
        {
            let reason = format!(
                "All operands for a bitwise operation must be the same type. Actually {}, {}, {}",
                assigns_name, lhs_name, rhs_name
            );
            return Err(self.instruction_type_error(reason));
        }

        Ok(())
    }

    fn check_op_bitwise_unop(&self, op: &Unop) -> TypeCheckResult<()> {
        let module = self.module();
        let function = self.function();

        let assigns_val = &function[op.assigns];
        let assigns_type = &module[assigns_val.type_];

        let operand_val = &function[op.operand];
        let operand_type = &module[operand_val.type_];

        let assigns_name = get_type_name(module, assigns_type).unwrap();
        let operand_name = get_type_name(module, operand_type).unwrap();

        if operand_type.is_floating_point() || assigns_type.is_floating_point() {
            let reason = format!(
                "Bitwise opcodes not valid with floating point operands. Actually {}, {}",
                assigns_name, operand_name
            );
            return Err(self.instruction_type_error(reason));
        }

        if !operand_type.is_numeric() || !assigns_type.is_numeric() {
            let reason = format!(
                "All operands for a bitwise operation must be numeric. Actually {}, {}",
                assigns_name, operand_name
            );
            return Err(self.instruction_type_error(reason));
        }

        if operand_val.type_ != assigns_val.type_ {
            let reason = format!(
                "All operands for a bitwise operation must be the same type. Actually {}, {}",
                assigns_name, operand_name
            );
            return Err(self.instruction_type_error(reason));
        }

        Ok(())
    }

    fn check_op_call(&self, op: &Call) -> TypeCheckResult<()> {
        self.check_op_call_like(op.function, op.assigns, &op.fn_params)
    }

    fn check_op_invoke(&self, op: &Invoke) -> TypeCheckResult<()> {
        self.check_op_call_like(op.function, op.assigns, &op.fn_params)
    }

    fn check_op_call_method(&self, op: &CallMethod) -> TypeCheckResult<()> {
        Ok(())
    }

    fn check_op_invoke_method(&self, op: &InvokeMethod) -> TypeCheckResult<()> {
        Ok(())
    }

    fn check_op_call_closure(&self, op: &CallClosure) -> TypeCheckResult<()> {
        self.check_op_call_closure_like(op.closure, op.assigns, &op.fn_params)
    }

    fn check_op_invoke_closure(&self, op: &InvokeClosure) -> TypeCheckResult<()> {
        self.check_op_call_closure_like(op.closure, op.assigns, &op.fn_params)
    }

    fn check_op_call_intrinsic(&self, op: &CallIntrinsic) -> TypeCheckResult<()> {
        Ok(())
    }

    fn check_op_invoke_intrinsic(&self, op: &InvokeIntrinsic) -> TypeCheckResult<()> {
        Ok(())
    }

    fn check_op_call_like(
        &self,
        function: CallableIndex,
        assigns: ValueIndex,
        params: &[ValueIndex],
    ) -> TypeCheckResult<()> {
        let module = self.module();

        let callee_sig = match function {
            CallableIndex::Native(i) => &module[module[i].type_],
            CallableIndex::Function(i) => &module[module[i].type_],
        };

        let callee_sig = if let Some(callee_sig) = callee_sig.get_type_function() {
            callee_sig
        } else {
            return Err(self.instruction_type_error("OpCall's function has a non function type"));
        };

        self.check_call_sig(assigns, callee_sig.returns, params, &callee_sig.args)?;

        Ok(())
    }

    fn check_op_call_closure_like(
        &self,
        closure: ValueIndex,
        assigns: ValueIndex,
        params: &[ValueIndex],
    ) -> TypeCheckResult<()> {
        let module = self.module();
        let function = self.function();

        let callee = &function[closure];
        let callee_sig = &module[callee.type_];

        let callee_sig = if let Some(callee_sig) = callee_sig.get_type_function() {
            callee_sig
        } else {
            return Err(
                self.instruction_type_error("OpCallClosure's function has a non function type")
            );
        };

        self.check_call_sig(assigns, callee_sig.returns, params, &callee_sig.args)?;

        Ok(())
    }

    fn check_call_sig(
        &self,
        op_assigns: ValueIndex,
        sig_returns: TypeIndex,
        op_params: &[ValueIndex],
        sig_params: &[TypeIndex],
    ) -> TypeCheckResult<()> {
        let module = self.module();
        let function = self.function();

        if op_params.len() != sig_params.len() {
            let reason = format!(
                "A call like op is calling a function with the incorrect number of arguments.\
                 Expected {} args, called with {} args",
                sig_params.len(),
                op_params.len()
            );
            return Err(self.instruction_type_error(reason));
        }

        let op_assigns = &function[op_assigns];
        let op_assigns_type = &module[op_assigns.type_];
        let op_assigns_name = get_type_name(module, op_assigns_type).unwrap();

        let sig_returns_type = &module[sig_returns];
        let sig_returns_name = get_type_name(module, sig_returns_type).unwrap();

        if op_assigns.type_ != sig_returns {
            let reason = format!(
                "A call like op has parameters that do not match function signature. \
                     Expected {}, got {}",
                sig_returns_name, op_assigns_name
            );
            return Err(self.instruction_type_error(reason));
        }

        let param_types = op_params.iter().cloned().map(|v| function[v].type_);

        let callee_types = sig_params.iter().cloned();

        for (param_type, sig_type) in param_types.zip(callee_types) {
            if param_type != sig_type {
                let called_with_sig = self.function_sig_string(
                    op_assigns.type_,
                    op_params.iter().cloned().map(|v| function[v].type_),
                );

                let callee_sig = self.function_sig_string(sig_returns, sig_params.iter().cloned());

                let reason = format!(
                    "A call like op has parameters that do not match function signature. \
                     Expected {}, called with {}",
                    callee_sig, called_with_sig
                );
                return Err(self.instruction_type_error(reason));
            }
        }

        Ok(())
    }

    fn check_op_cond_branch(&self, op: &CondBranch) -> TypeCheckResult<()> {
        let module = self.module();
        let function = self.function();

        let cond_type = &function[op.check];
        let cond_type = &module[cond_type.type_];

        let cond_name = get_type_name(module, cond_type).unwrap();

        if cond_type != &Type::Bool {
            let reason = format!(
                "The condition operand for a conditional branch must always be of type Bool. Actually {}",
                cond_name
            );
            return Err(self.instruction_type_error(reason));
        }

        Ok(())
    }

    fn check_op_cond_branch_null_check(&self, op: &CondBranch) -> TypeCheckResult<()> {
        let module = self.module();
        let function = self.function();

        let cond_type = &function[op.check];
        let cond_type = &module[cond_type.type_];

        let cond_name = get_type_name(module, cond_type).unwrap();

        if !cond_type.is_nullable() {
            let reason = format!(
                "The condition operand for a null check branch must be a nullable type. Actually {}",
                cond_name
            );
            return Err(self.instruction_type_error(reason));
        }

        Ok(())
    }

    fn check_op_cmp(&self, op: &Comparison) -> TypeCheckResult<()> {
        let module = self.module();
        let function = self.function();

        let lhs_val = &function[op.lhs];
        let lhs_type = &module[lhs_val.type_];
        let rhs_val = &function[op.rhs];
        let rhs_type = &module[rhs_val.type_];

        let assigns_val = &function[op.assigns];
        let assigns_type = &module[assigns_val.type_];

        let assigns_name = get_type_name(module, assigns_type).unwrap();
        let lhs_name = get_type_name(module, lhs_type).unwrap();
        let rhs_name = get_type_name(module, rhs_type).unwrap();

        if lhs_val.type_ != rhs_val.type_ {
            let reason = format!(
                "The two comparison operands of an OpCmp must be the same type. Actually {} and {}",
                lhs_name, rhs_name
            );
            return Err(self.instruction_type_error(reason));
        }

        if assigns_type != &Type::Bool {
            let reason = format!(
                "The assigned value from an OpCmp must be a Bool type. Actually {}",
                assigns_name
            );
            return Err(self.instruction_type_error(reason));
        }

        Ok(())
    }

    fn check_op_phi(&self, op: &Phi) -> TypeCheckResult<()> {
        let function = self.function();

        let assigns_val = &function[op.assigns];

        for (v, _) in op.block_values.iter().cloned() {
            let v_val = &function[v];

            if assigns_val.type_ != v_val.type_ {
                return Err(
                    self.instruction_type_error("All operands of an OpPhi must be the same type")
                );
            }
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

    fn function_sig_string(
        &self,
        returns: TypeIndex,
        params: impl Iterator<Item = TypeIndex>,
    ) -> String {
        let module = self.module();

        let returns = &module[returns];
        let returns_name = get_type_name(module, returns).unwrap();

        let mut string = "(".to_string();
        for arg in params {
            let arg_type = &module[arg];
            let arg_name = get_type_name(module, arg_type).unwrap();
            string.push_str(&arg_name);
            string.push_str(", ");
        }
        string.push_str(") -> ");
        string.push_str(&returns_name);

        string
    }
}
