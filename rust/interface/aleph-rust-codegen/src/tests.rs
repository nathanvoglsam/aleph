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

use crate::FunctionSignatureBuilder;
use crate::Statement;
use proc_macro2::Span;
use quote::ToTokens;
use syn::{Expr, ExprLit, Local, Pat, PatIdent};

#[test]
fn build_function_1() {
    // Build the function signature
    let mut signature = FunctionSignatureBuilder::new("test_func");
    signature.external(Some("C"));
    signature.unsafety(true);
    signature.visibility_pub();

    // Build a function with a body
    let mut function = signature.clone().build_function();
    function.statement(Statement::null());

    // Build the right hand side of the statement "let test = true"
    let right = syn::LitBool {
        value: true,
        span: Span::call_site(),
    };
    let right = syn::Lit::Bool(right);
    let right = ExprLit {
        attrs: Vec::new(),
        lit: right,
    };
    let right = Expr::Lit(right);
    let right = Box::new(right);

    // Build the left hand side of the statement "let test = true"
    let local = syn::Ident::new("test", Span::call_site());
    let local = PatIdent {
        attrs: vec![],
        by_ref: None,
        mutability: None,
        ident: local,
        subpat: None,
    };
    let local = Local {
        attrs: vec![],
        let_token: Default::default(),
        pat: Pat::Ident(local),
        init: Some((Default::default(), right)),
        semi_token: Default::default(),
    };

    // Build the statement "let test = true"
    let statement = Statement::local(local);

    // Add the "test = true" statement to the function
    function.statement(statement);

    let function = function.build();

    // Build a function without a body
    let ffi_function = signature.clone().build_ffi_function();

    let stream = function.to_token_stream();
    println!("{}", stream);

    let stream = ffi_function.to_token_stream();
    println!("{}", stream);
}
