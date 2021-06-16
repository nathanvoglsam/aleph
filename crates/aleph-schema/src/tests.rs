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

use crate::parser::Error;

#[test]
pub fn test_valid_enum() {
    test_valid_file("./schemas/valid_enum.schema");
}

#[test]
pub fn test_valid_nested_modules() {
    test_valid_file("./schemas/valid_nested_modules.schema");
}

#[test]
pub fn test_valid_struct() {
    test_valid_file("./schemas/valid_struct.schema");
}

#[test]
pub fn test_valid_table() {
    test_valid_file("./schemas/valid_table.schema");
}

#[test]
pub fn test_invalid_duplicate_field() {
    test_invalid_file(
        "./schemas/invalid_duplicate_field.schema",
        Error::DuplicateEntity {
            span: 81..82,
            duplicate: 59..60,
        },
    );
}

fn test_invalid_file(file: &str, expected: crate::parser::Error) {
    let text = std::fs::read_to_string(file).unwrap();
    let text = text.replace("\r\n", "\n");

    let sexpr_lexer = sexpr::lexer::Lexer::new(text.as_str());
    let sexpr_parser = sexpr::parser::FileParser::new();
    let sexpr_tree = sexpr_parser.parse(sexpr_lexer).unwrap();

    let ast_parser = crate::parser::Parser::new(sexpr_tree);
    let err = ast_parser.parse().unwrap_err();

    assert_eq!(err, expected);
}

fn test_valid_file(file: &str) {
    let text = std::fs::read_to_string(file).unwrap();
    let text = text.replace("\r\n", "\n");

    let sexpr_lexer = sexpr::lexer::Lexer::new(text.as_str());
    let sexpr_parser = sexpr::parser::FileParser::new();
    let sexpr_tree = sexpr_parser.parse(sexpr_lexer).unwrap();

    let ast_parser = crate::parser::Parser::new(sexpr_tree);
    let ast = ast_parser.parse().unwrap();

    println!("{:#?}", ast);
}
