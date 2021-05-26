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

#[test]
fn test_valid_simple() {
    test_parses_valid("./schemas/valid_simple.schema");
}

#[test]
fn test_valid_complex() {
    test_parses_valid("./schemas/valid_complex.schema");
}

#[test]
fn test_valid_empty_file() {
    test_parses_valid("./schemas/valid_empty_file.schema");
}

#[test]
fn test_valid_empty_file_with_whitespace() {
    test_parses_valid("./schemas/valid_empty_file_with_whitespace.schema");
}

#[test]
fn test_valid_escaped_string() {
    test_parses_valid("./schemas/valid_escaped_string.schema");
}

#[test]
fn test_valid_special_idents() {
    test_parses_valid("./schemas/valid_special_idents.schema");
}

#[test]
fn test_invalid_unterminated_list() {
    test_parses_invalid("./schemas/invalid_unterminated_list.schema");
}

#[test]
fn test_invalid_unterminated_string() {
    test_parses_invalid("./schemas/invalid_unterminated_string.schema");
}

#[test]
fn test_invalid_terminating_too_many_lists() {
    test_parses_invalid("./schemas/invalid_terminating_too_many_lists.schema");
}

#[test]
fn test_invalid_terminating_unopened_list() {
    test_parses_invalid("./schemas/invalid_terminating_unopened_list.schema");
}

#[test]
fn test_invalid_root_atom() {
    test_parses_invalid("./schemas/invalid_root_atom.schema");
}

fn test_parses_valid(file_name: &str) {
    let text = std::fs::read_to_string(file_name).unwrap();
    match crate::parse(text.as_str()) {
        Ok(output) => println!("{:#?}", output),
        Err(error) => {
            crate::print_error(&text, error);
            panic!("Failed to parse a valid input")
        }
    }
}

fn test_parses_invalid(file_name: &str) {
    let text = std::fs::read_to_string(file_name).unwrap();
    match crate::parse(text.as_str()) {
        Ok(output) => {
            println!("{:#?}", output);
            panic!("Successfully parsed an invalid input");
        }
        Err(error) => {
            crate::print_error(&text, error);
        }
    }
}
