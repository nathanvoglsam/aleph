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

use crate::code::Code;
use crate::utils::TestOpCodeIter;
use crate::OpCode;
use std::io::BufReader;
use std::path::PathBuf;

#[test]
pub fn parse_test_1() {
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

    Code::read(&mut file).unwrap();
}

#[test]
pub fn test_serialization() {
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

    let _string = serde_json::to_string_pretty(&code).unwrap();
    let _bytes = rmp_serde::to_vec(&code).unwrap();

    //std::fs::write(crate_root.join("out.json"), string.as_bytes()).unwrap();
    //std::fs::write(crate_root.join("out.msgpack"), &bytes).unwrap();
}

/// Force that only explicitly documented instructions do not perform a register read or write.
///
/// That is, we need to explicitly "whitelist" all opcodes that
#[test]
pub fn test_op_read_write_coverage() {
    for op in TestOpCodeIter::new() {
        match op {
            OpCode::OpJAlways(_)
            | OpCode::OpEndTrap(_)
            | OpCode::OpLabel
            | OpCode::OpAssert
            | OpCode::OpNop => {}
            _ => {
                let reads_register = op.register_reads().is_some();
                let writes_register = op.register_write().is_some();
                assert!(
                    reads_register || writes_register,
                    "Opcode doesn't read or write registers, {:?}",
                    op
                );
            }
        }
    }
}
