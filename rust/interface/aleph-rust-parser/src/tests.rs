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

use crate::ast::Module;
use crate::crate_resolver::ParserError;
use crate::interner::Interner;
use std::path::PathBuf;

#[test]
fn test_parse_crate_valid() {
    let crate_root = std::env::var("CARGO_MANIFEST_DIR").unwrap();
    let mut crate_root = PathBuf::from(crate_root);
    crate_root.push("test_crate_roots");
    crate_root.push("valid_1");
    let file = crate::crate_resolver::resolve_crate(&crate_root).unwrap();
}

#[test]
fn test_parse_crate_malformed_graph() {
    let crate_root = std::env::var("CARGO_MANIFEST_DIR").unwrap();
    let mut crate_root = PathBuf::from(crate_root);
    crate_root.push("test_crate_roots");
    crate_root.push("malformed_graph");
    let file = crate::crate_resolver::resolve_crate(&crate_root);
    if let Err(e) = file {
        match e {
            ParserError::ModuleGraphInvalid => {}
            _ => panic!(
                "parse_crate failed, but failed for the wrong reason: {:#?}",
                e
            ),
        }
    } else {
        panic!("parse_crate should fail, but did not")
    }
}

#[test]
fn test_parse_to_internal_ast() {
    let crate_root = std::env::var("CARGO_MANIFEST_DIR").unwrap();
    let mut crate_root = PathBuf::from(crate_root);
    crate_root.push("test_crate_roots");
    crate_root.push("valid_2");
    let file = crate::crate_resolver::resolve_crate(&crate_root).unwrap();

    let mut interner = Interner::with_capacity(1024);
    let interface = Module::from_file(&mut interner, &file).unwrap();
    interface.debug_print(&interner).unwrap();
}
