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

use crate::bytecode::indexes::{
    BasicBlockIndex, BytesIndex, ConstructorIndex, FieldIndex, FloatIndex, FunctionIndex,
    GlobalIndex, IntegerIndex, StringIndex, TypeIndex, ValueIndex,
};
use crate::bytecode::module::Module;
use crate::bytecode::opcode::OpCode;
use hashlink_bytecode::utils::TestOpCodeIter;
use hashlink_bytecode::Code;
use std::io::BufReader;
use std::path::PathBuf;

#[test]
pub fn test_translation_1() {
    let crate_root = std::env::var("CARGO_MANIFEST_DIR")
        .map(PathBuf::from)
        .unwrap_or(std::env::current_dir().unwrap());

    let path = crate_root.join("build.hl");
    let file = std::fs::OpenOptions::new()
        .read(true)
        .write(false)
        .create(false)
        .open(path)
        .unwrap();

    let mut file = BufReader::new(file);

    let code = Code::read(&mut file).unwrap();

    let module = Module::from_hashlink(code).unwrap();
}

#[test]
pub fn test_translation_coverage() {
    // Values used as templates
    let vi = ValueIndex(0);
    let ii = IntegerIndex(0);
    let fi = FloatIndex(0);
    let bi = BytesIndex(0);
    let si = StringIndex(0);
    let gi = GlobalIndex(0);
    let ti = TypeIndex(0);
    let ci = ConstructorIndex(0);
    let ff = FieldIndex(0);
    let fu = FunctionIndex(0);
    let bb = BasicBlockIndex(0);

    for op in TestOpCodeIter::new() {
        match op {
            hashlink_bytecode::OpCode::OpMov(_) => OpCode::translate_load(&op, vi, vi).unwrap(),
            hashlink_bytecode::OpCode::OpInt(_) => OpCode::translate_load_int(&op, vi, ii).unwrap(),
            hashlink_bytecode::OpCode::OpFloat(_) => {
                OpCode::translate_load_float(&op, vi, fi).unwrap()
            }
            hashlink_bytecode::OpCode::OpBool(_) => {
                OpCode::translate_load_bool(&op, vi, false).unwrap()
            }
            hashlink_bytecode::OpCode::OpBytes(_) => {
                OpCode::translate_load_bytes(&op, vi, bi).unwrap()
            }
            hashlink_bytecode::OpCode::OpString(_) => {
                OpCode::translate_load_string(&op, vi, si).unwrap()
            }

            hashlink_bytecode::OpCode::OpNull(_) => OpCode::translate_value_index(&op, vi).unwrap(),

            hashlink_bytecode::OpCode::OpAdd(_)
            | hashlink_bytecode::OpCode::OpSub(_)
            | hashlink_bytecode::OpCode::OpMul(_)
            | hashlink_bytecode::OpCode::OpSDiv(_)
            | hashlink_bytecode::OpCode::OpUDiv(_)
            | hashlink_bytecode::OpCode::OpSMod(_)
            | hashlink_bytecode::OpCode::OpUMod(_)
            | hashlink_bytecode::OpCode::OpShl(_)
            | hashlink_bytecode::OpCode::OpSShr(_)
            | hashlink_bytecode::OpCode::OpUShr(_)
            | hashlink_bytecode::OpCode::OpAnd(_)
            | hashlink_bytecode::OpCode::OpOr(_)
            | hashlink_bytecode::OpCode::OpXor(_) => {
                OpCode::translate_binop(&op, vi, vi, vi).unwrap()
            }

            hashlink_bytecode::OpCode::OpNeg(_)
            | hashlink_bytecode::OpCode::OpNot(_)
            | hashlink_bytecode::OpCode::OpIncr(_)
            | hashlink_bytecode::OpCode::OpDecr(_) => OpCode::translate_unop(&op, vi, vi).unwrap(),

            hashlink_bytecode::OpCode::OpCall0(_)
            | hashlink_bytecode::OpCode::OpCall1(_)
            | hashlink_bytecode::OpCode::OpCall2(_)
            | hashlink_bytecode::OpCode::OpCall3(_)
            | hashlink_bytecode::OpCode::OpCall4(_)
            | hashlink_bytecode::OpCode::OpCallN(_) => {
                OpCode::translate_call(&op, vi, fu, Vec::new()).unwrap()
            }

            hashlink_bytecode::OpCode::OpCallMethod(_)
            | hashlink_bytecode::OpCode::OpCallThis(_) => {
                OpCode::translate_call_field(&op, vi, vi, ff, Vec::new()).unwrap()
            }

            hashlink_bytecode::OpCode::OpCallClosure(_) => {
                OpCode::translate_call_closure(&op, vi, vi, Vec::new()).unwrap()
            }

            hashlink_bytecode::OpCode::OpStaticClosure(_) => {
                OpCode::translate_static_closure(&op, vi, fu).unwrap()
            }
            hashlink_bytecode::OpCode::OpInstanceClosure(_) => {
                OpCode::translate_instance_closure(&op, vi, fu, vi).unwrap()
            }
            hashlink_bytecode::OpCode::OpVirtualClosure(_) => {
                OpCode::translate_virtual_closure(&op, vi, vi, ff).unwrap()
            }

            hashlink_bytecode::OpCode::OpGetGlobal(_) => {
                OpCode::translate_load_global(&op, vi, gi).unwrap()
            }
            hashlink_bytecode::OpCode::OpSetGlobal(_) => {
                OpCode::translate_store_global(&op, vi, gi).unwrap()
            }

            hashlink_bytecode::OpCode::OpField(_)
            | hashlink_bytecode::OpCode::OpGetThis(_)
            | hashlink_bytecode::OpCode::OpDynGet(_) => {
                OpCode::translate_field_load(&op, vi, vi, ff).unwrap()
            }

            hashlink_bytecode::OpCode::OpSetField(_)
            | hashlink_bytecode::OpCode::OpSetThis(_)
            | hashlink_bytecode::OpCode::OpDynSet(_) => {
                OpCode::translate_field_store(&op, vi, ff, vi).unwrap()
            }

            hashlink_bytecode::OpCode::OpJTrue(_)
            | hashlink_bytecode::OpCode::OpJFalse(_)
            | hashlink_bytecode::OpCode::OpJNull(_)
            | hashlink_bytecode::OpCode::OpJNotNull(_) => {
                OpCode::translate_cond_branch(&op, vi, bb, bb).unwrap()
            }

            hashlink_bytecode::OpCode::OpJSLt(_)
            | hashlink_bytecode::OpCode::OpJSGte(_)
            | hashlink_bytecode::OpCode::OpJSGt(_)
            | hashlink_bytecode::OpCode::OpJSLte(_)
            | hashlink_bytecode::OpCode::OpJULt(_)
            | hashlink_bytecode::OpCode::OpJUGte(_)
            | hashlink_bytecode::OpCode::OpJNotLt(_)
            | hashlink_bytecode::OpCode::OpJNotGte(_)
            | hashlink_bytecode::OpCode::OpJEq(_)
            | hashlink_bytecode::OpCode::OpJNotEq(_) => {
                OpCode::translate_comp_branch(&op, vi, vi, bb, bb).unwrap()
            }

            hashlink_bytecode::OpCode::OpJAlways(_) => {
                OpCode::translate_unconditional_branch(&op, bb).unwrap()
            }

            hashlink_bytecode::OpCode::OpToDyn(_)
            | hashlink_bytecode::OpCode::OpToSFloat(_)
            | hashlink_bytecode::OpCode::OpToUFloat(_)
            | hashlink_bytecode::OpCode::OpToInt(_)
            | hashlink_bytecode::OpCode::OpSafeCast(_)
            | hashlink_bytecode::OpCode::OpUnsafeCast(_)
            | hashlink_bytecode::OpCode::OpToVirtual(_) => {
                OpCode::translate_cast(&op, vi, vi).unwrap()
            }

            hashlink_bytecode::OpCode::OpSwitch(_) => {
                OpCode::translate_switch(&op, vi, Vec::new(), bb).unwrap()
            }
            hashlink_bytecode::OpCode::OpTrap(_) => OpCode::translate_trap(&op, bb).unwrap(),
            hashlink_bytecode::OpCode::OpEndTrap(_) => {
                OpCode::translate_end_trap(&op, false).unwrap()
            }

            hashlink_bytecode::OpCode::OpGetI8(_)
            | hashlink_bytecode::OpCode::OpGetI16(_)
            | hashlink_bytecode::OpCode::OpGetMem(_)
            | hashlink_bytecode::OpCode::OpGetArray(_) => {
                OpCode::translate_read_memory(&op, vi, vi, vi).unwrap()
            }

            hashlink_bytecode::OpCode::OpSetI8(_)
            | hashlink_bytecode::OpCode::OpSetI16(_)
            | hashlink_bytecode::OpCode::OpSetMem(_)
            | hashlink_bytecode::OpCode::OpSetArray(_) => {
                OpCode::translate_write_memory(&op, vi, vi, vi).unwrap()
            }

            hashlink_bytecode::OpCode::OpType(_) => {
                OpCode::translate_load_type(&op, vi, ti).unwrap()
            }

            hashlink_bytecode::OpCode::OpRet(_)
            | hashlink_bytecode::OpCode::OpThrow(_)
            | hashlink_bytecode::OpCode::OpRethrow(_)
            | hashlink_bytecode::OpCode::OpNullCheck(_)
            | hashlink_bytecode::OpCode::OpNew(_) => {
                OpCode::translate_value_index(&op, vi).unwrap()
            }

            hashlink_bytecode::OpCode::OpArraySize(_)
            | hashlink_bytecode::OpCode::OpGetType(_)
            | hashlink_bytecode::OpCode::OpGetTID(_)
            | hashlink_bytecode::OpCode::OpRef(_)
            | hashlink_bytecode::OpCode::OpUnRef(_)
            | hashlink_bytecode::OpCode::OpEnumIndex(_) => {
                OpCode::translate_load(&op, vi, vi).unwrap()
            }

            hashlink_bytecode::OpCode::OpSetRef(_) => OpCode::translate_store(&op, vi, vi).unwrap(),
            hashlink_bytecode::OpCode::OpMakeEnum(_) => {
                OpCode::translate_make_enum(&op, vi, ci, Vec::new()).unwrap()
            }
            hashlink_bytecode::OpCode::OpEnumAlloc(_) => {
                OpCode::translate_alloc_enum(&op, vi, ci).unwrap()
            }
            hashlink_bytecode::OpCode::OpEnumField(_) => {
                OpCode::translate_load_enum_field(&op, vi, vi, ci, ff).unwrap()
            }
            hashlink_bytecode::OpCode::OpSetEnumField(_) => {
                OpCode::translate_store_enum_field(&op, vi, ff, vi).unwrap()
            }
            hashlink_bytecode::OpCode::OpRefData(_) => {
                OpCode::translate_ref_data(&op, vi, vi).unwrap()
            }
            hashlink_bytecode::OpCode::OpRefOffset(_) => {
                OpCode::translate_ref_offset(&op, vi, vi, vi).unwrap()
            }

            hashlink_bytecode::OpCode::OpAssert
            | hashlink_bytecode::OpCode::OpNop
            | hashlink_bytecode::OpCode::OpLabel => OpCode::translate_no_params(&op).unwrap(),
        };
    }
}
