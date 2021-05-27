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

fn default(default: ast::untyped::Atom) -> ast::untyped::ListBuilder {
    ast::untyped::ListBuilder::new()
        .add_word("default", None)
        .add_atom(default, None)
}

fn default_0_float() -> ast::untyped::ListBuilder {
    default(ast::untyped::Atom::word("0.0"))
}

fn field(name: &str, field_type: &str) -> ast::untyped::ListBuilder {
    ast::untyped::ListBuilder::new()
        .add_word("field", None)
        .add_word(name, None)
        .add_word(field_type, None)
}

fn field_default<L: Into<ast::untyped::List>>(
    name: &str,
    field_type: &str,
    default: L,
) -> ast::untyped::ListBuilder {
    ast::untyped::ListBuilder::new()
        .add_word("field", None)
        .add_word(name, None)
        .add_word(field_type, None)
        .add_list(default, None)
}

fn float_field_default_0(name: &str) -> ast::untyped::ListBuilder {
    field_default(name, "f32", default_0_float())
}

fn vector2() -> ast::untyped::ListBuilder {
    ast::untyped::ListBuilder::new()
        .add_word("def-struct", None)
        .add_word("Vector2", None)
        .add_list(float_field_default_0("x"), None)
        .add_list(float_field_default_0("y"), None)
}

fn vector3() -> ast::untyped::ListBuilder {
    ast::untyped::ListBuilder::new()
        .add_word("def-struct", None)
        .add_word("Vector3", None)
        .add_list(float_field_default_0("x"), None)
        .add_list(float_field_default_0("y"), None)
        .add_list(float_field_default_0("z"), None)
}

fn monster() -> ast::untyped::ListBuilder {
    let default_name = default(ast::untyped::Atom::string("default-monster"));
    ast::untyped::ListBuilder::new()
        .add_word("def-table", None)
        .add_word("Monster", None)
        .add_list(field("position", "Vector3"), None)
        .add_list(field("target", "Vector3"), None)
        .add_list(field_default("name", "string", default_name), None)
}

fn soldier() -> ast::untyped::ListBuilder {
    let default_health = default(ast::untyped::Atom::word("1000000.0"));
    ast::untyped::ListBuilder::new()
        .add_word("def-table", None)
        .add_word("Soldier", None)
        .add_list(field("position", "aleph::Vector3"), None)
        .add_list(field("target", "aleph::Vector3"), None)
        .add_list(field_default("health", "f64", default_health), None)
}

#[test]
fn test_valid() {
    let namespace_aleph = ast::untyped::ListBuilder::new()
        .add_word("namespace", None)
        .add_word("aleph", None)
        .add_list(vector2(), None)
        .add_list(vector3(), None)
        .add_list(monster(), None);
    let namespace_engine = ast::untyped::ListBuilder::new()
        .add_word("namespace", None)
        .add_word("engine", None)
        .add_list(soldier(), None)
        .add_list([], None);
    let expected = ast::untyped::ListBuilder::new()
        .add_list(namespace_aleph, None)
        .add_list(namespace_engine, None);
    test_parses_valid("./schemas/valid.schema", expected);
}

#[test]
fn test_valid_empty_file() {
    let expected = ast::untyped::ListBuilder::new();
    test_parses_valid("./schemas/valid_empty_file.schema", expected);
}

#[test]
fn test_valid_empty_file_with_whitespace() {
    let expected = ast::untyped::ListBuilder::new();
    test_parses_valid(
        "./schemas/valid_empty_file_with_whitespace.schema",
        expected,
    );
}

#[test]
fn test_valid_escaped_string() {
    let string = "Test text please \"ignore\" me";
    let list = ast::untyped::ListBuilder::new().add_string(string, None);
    let expected = ast::untyped::ListBuilder::new().add_list(list, None);
    test_parses_valid("./schemas/valid_escaped_string.schema", expected);
}

#[test]
fn test_valid_special_idents() {
    let first = ast::untyped::ListBuilder::new().add_word("\\first", None);
    let second = ast::untyped::ListBuilder::new().add_word("/second", None);
    let third = ast::untyped::ListBuilder::new().add_word("-third", None);
    let namespace_aleph = ast::untyped::ListBuilder::new()
        .add_word("namespace", None)
        .add_word("aleph", None)
        .add_list(first, None)
        .add_list(second, None)
        .add_list(third, None);
    let expected = ast::untyped::ListBuilder::new().add_list(namespace_aleph, None);
    test_parses_valid("./schemas/valid_special_idents.schema", expected);
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

fn test_parses_valid<L: Into<ast::untyped::List>>(file_name: &str, expected: L) {
    let expected = expected.into();
    let text = std::fs::read_to_string(file_name).unwrap();
    match crate::parse(text.as_str()) {
        Ok(output) => {
            assert_eq!(
                &output, &expected,
                "ASTs do not match\nGOT:\n{:#?}\n---------------\nEXPECTED:\n{:#?}",
                &output, &expected
            );
        }
        Err(error) => {
            crate::print_error(&text, error);
            panic!("Failed to parse a valid input");
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
