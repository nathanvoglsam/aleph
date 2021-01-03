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

use crate::drivers::translate_hashlink_module;
use crate::opcode_translators::{
    translate_alloc_enum, translate_binop, translate_call, translate_call_closure,
    translate_call_field, translate_cast, translate_comp_branch, translate_cond_branch,
    translate_field_load, translate_field_store, translate_instance_closure, translate_intrinsic,
    translate_intrinsic_invoke, translate_load, translate_load_bool, translate_load_bytes,
    translate_load_enum_field, translate_load_float, translate_load_global, translate_load_int,
    translate_load_string, translate_load_type, translate_make_enum, translate_no_params,
    translate_read_memory, translate_ref_data, translate_ref_offset, translate_static_closure,
    translate_store, translate_store_enum_field, translate_store_global, translate_switch,
    translate_unconditional_branch, translate_unop, translate_value_index,
    translate_virtual_closure, translate_write_memory,
};
use eon_bytecode::indexes::{
    BasicBlockIndex, BytesIndex, CallableIndex, ConstructorIndex, FieldIndex, FloatIndex,
    GlobalIndex, IntegerIndex, StringIndex, TypeIndex, ValueIndex,
};
use hashlink::utils::TestOpCodeIter;
use hashlink::Code;
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

    let module = translate_hashlink_module(code).unwrap();

    let _string = eon_bytecode::module::dump::dump_to_string(&module).unwrap();
    let _json = serde_json::to_string_pretty(&module).unwrap();
    let _bytes = rmp_serde::to_vec(&module).unwrap();

    std::fs::write(crate_root.join("out.dump"), _string).unwrap();
    //std::fs::write(crate_root.join("out.json"), _json.as_bytes()).unwrap();
    //std::fs::write(crate_root.join("out.msgpack"), &_bytes).unwrap();
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
    let fu = CallableIndex::Native(0.into());
    let bb = BasicBlockIndex(0);

    for op in TestOpCodeIter::new() {
        match op {
            hashlink::OpCode::OpMov(_) => translate_load(&op, vi, vi).unwrap(),
            hashlink::OpCode::OpInt(_) => translate_load_int(&op, vi, ii).unwrap(),
            hashlink::OpCode::OpFloat(_) => translate_load_float(&op, vi, fi).unwrap(),
            hashlink::OpCode::OpBool(_) => translate_load_bool(&op, vi, false).unwrap(),
            hashlink::OpCode::OpBytes(_) => translate_load_bytes(&op, vi, bi).unwrap(),
            hashlink::OpCode::OpString(_) => translate_load_string(&op, vi, si).unwrap(),

            hashlink::OpCode::OpNull(_) => translate_value_index(&op, vi).unwrap(),

            hashlink::OpCode::OpAdd(_)
            | hashlink::OpCode::OpSub(_)
            | hashlink::OpCode::OpMul(_)
            | hashlink::OpCode::OpSDiv(_)
            | hashlink::OpCode::OpUDiv(_)
            | hashlink::OpCode::OpSMod(_)
            | hashlink::OpCode::OpUMod(_)
            | hashlink::OpCode::OpShl(_)
            | hashlink::OpCode::OpSShr(_)
            | hashlink::OpCode::OpUShr(_)
            | hashlink::OpCode::OpAnd(_)
            | hashlink::OpCode::OpOr(_)
            | hashlink::OpCode::OpXor(_) => translate_binop(&op, vi, vi, vi).unwrap(),

            hashlink::OpCode::OpNeg(_)
            | hashlink::OpCode::OpNot(_)
            | hashlink::OpCode::OpIncr(_)
            | hashlink::OpCode::OpDecr(_) => translate_unop(&op, vi, vi).unwrap(),

            hashlink::OpCode::OpCall0(_)
            | hashlink::OpCode::OpCall1(_)
            | hashlink::OpCode::OpCall2(_)
            | hashlink::OpCode::OpCall3(_)
            | hashlink::OpCode::OpCall4(_)
            | hashlink::OpCode::OpCallN(_) => translate_call(&op, vi, fu, Vec::new()).unwrap(),

            hashlink::OpCode::OpCallMethod(_) | hashlink::OpCode::OpCallThis(_) => {
                translate_call_field(&op, vi, vi, ff, Vec::new()).unwrap()
            }

            hashlink::OpCode::OpCallClosure(_) => {
                translate_call_closure(&op, vi, vi, Vec::new()).unwrap()
            }

            hashlink::OpCode::OpStaticClosure(_) => translate_static_closure(&op, vi, fu).unwrap(),
            hashlink::OpCode::OpInstanceClosure(_) => {
                translate_instance_closure(&op, vi, fu, vi).unwrap()
            }
            hashlink::OpCode::OpVirtualClosure(_) => {
                translate_virtual_closure(&op, vi, vi, ff).unwrap()
            }

            hashlink::OpCode::OpGetGlobal(_) => translate_load_global(&op, vi, gi).unwrap(),
            hashlink::OpCode::OpSetGlobal(_) => translate_store_global(&op, gi, vi).unwrap(),

            hashlink::OpCode::OpField(_)
            | hashlink::OpCode::OpGetThis(_)
            | hashlink::OpCode::OpDynGet(_) => translate_field_load(&op, vi, vi, ff).unwrap(),

            hashlink::OpCode::OpSetField(_)
            | hashlink::OpCode::OpSetThis(_)
            | hashlink::OpCode::OpDynSet(_) => translate_field_store(&op, vi, ff, vi).unwrap(),

            hashlink::OpCode::OpJTrue(_)
            | hashlink::OpCode::OpJFalse(_)
            | hashlink::OpCode::OpJNull(_)
            | hashlink::OpCode::OpJNotNull(_) => translate_cond_branch(&op, vi, bb, bb).unwrap(),

            hashlink::OpCode::OpJSLt(_)
            | hashlink::OpCode::OpJSGte(_)
            | hashlink::OpCode::OpJSGt(_)
            | hashlink::OpCode::OpJSLte(_)
            | hashlink::OpCode::OpJULt(_)
            | hashlink::OpCode::OpJUGte(_)
            | hashlink::OpCode::OpJNotLt(_)
            | hashlink::OpCode::OpJNotGte(_)
            | hashlink::OpCode::OpJEq(_)
            | hashlink::OpCode::OpJNotEq(_) => translate_comp_branch(&op, vi, vi, vi).unwrap(),

            hashlink::OpCode::OpJAlways(_) => translate_unconditional_branch(&op, bb).unwrap(),

            hashlink::OpCode::OpToDyn(_)
            | hashlink::OpCode::OpToSFloat(_)
            | hashlink::OpCode::OpToUFloat(_)
            | hashlink::OpCode::OpToInt(_)
            | hashlink::OpCode::OpSafeCast(_)
            | hashlink::OpCode::OpUnsafeCast(_)
            | hashlink::OpCode::OpToVirtual(_) => translate_cast(&op, vi, vi).unwrap(),

            hashlink::OpCode::OpSwitch(_) => translate_switch(&op, vi, Vec::new(), bb).unwrap(),

            hashlink::OpCode::OpTrap(_) | hashlink::OpCode::OpEndTrap(_) => {
                translate_intrinsic_invoke(&op, vi, Vec::new(), bb, bb).unwrap()
            }

            hashlink::OpCode::OpGetI8(_)
            | hashlink::OpCode::OpGetI16(_)
            | hashlink::OpCode::OpGetMem(_)
            | hashlink::OpCode::OpGetArray(_) => translate_read_memory(&op, vi, vi, vi).unwrap(),

            hashlink::OpCode::OpSetI8(_)
            | hashlink::OpCode::OpSetI16(_)
            | hashlink::OpCode::OpSetMem(_)
            | hashlink::OpCode::OpSetArray(_) => translate_write_memory(&op, vi, vi, vi).unwrap(),

            hashlink::OpCode::OpType(_) => translate_load_type(&op, vi, ti).unwrap(),

            hashlink::OpCode::OpRet(_) | hashlink::OpCode::OpNew(_) => {
                translate_value_index(&op, vi).unwrap()
            }

            hashlink::OpCode::OpThrow(_)
            | hashlink::OpCode::OpRethrow(_)
            | hashlink::OpCode::OpNullCheck(_) => {
                translate_intrinsic(&op, vi, [].iter().cloned()).unwrap()
            }

            hashlink::OpCode::OpArraySize(_)
            | hashlink::OpCode::OpGetType(_)
            | hashlink::OpCode::OpGetTID(_)
            | hashlink::OpCode::OpRef(_)
            | hashlink::OpCode::OpUnRef(_)
            | hashlink::OpCode::OpEnumIndex(_) => translate_load(&op, vi, vi).unwrap(),

            hashlink::OpCode::OpSetRef(_) => translate_store(&op, vi, vi).unwrap(),
            hashlink::OpCode::OpMakeEnum(_) => {
                translate_make_enum(&op, vi, ci, Vec::new()).unwrap()
            }
            hashlink::OpCode::OpEnumAlloc(_) => translate_alloc_enum(&op, vi, ci).unwrap(),
            hashlink::OpCode::OpEnumField(_) => {
                translate_load_enum_field(&op, vi, vi, ci, ff).unwrap()
            }
            hashlink::OpCode::OpSetEnumField(_) => {
                translate_store_enum_field(&op, vi, ff, vi).unwrap()
            }
            hashlink::OpCode::OpRefData(_) => translate_ref_data(&op, vi, vi).unwrap(),
            hashlink::OpCode::OpRefOffset(_) => translate_ref_offset(&op, vi, vi, vi).unwrap(),

            hashlink::OpCode::OpAssert | hashlink::OpCode::OpNop | hashlink::OpCode::OpLabel => {
                translate_no_params(&op).unwrap()
            }
        };
    }
}
