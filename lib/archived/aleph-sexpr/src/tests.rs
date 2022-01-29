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

use crate::lexer::{Error, Tok};

const INVALID_ROOT_ATOM: &'static str = r#"namespace

"#;

const INVALID_TERMINATING_TOO_MANY_LISTS: &'static str = r#"
(namespace aleph
    (hello world))
))

"#;

const INVALID_TERMINATING_UNOPENED_LIST: &'static str = r#"
)(namespace aleph
    (hello world)
)

\"#;

const INVALID_UNTERMINATED_LIST: &'static str = r#"(namespace aleph
    (- hello world
)

"#;

const INVALID_UNTERMINATED_STRING: &'static str = r#"(namespace aleph
    (default "default-monster)
)

\"#;

const VALID: &'static str = r#"
// Top level comment
(namespace aleph
    // Comment inside a list
    (def-struct Vector2
        (field x f32 (default 0.0))
        (field y f32 (default 0.0))
    )
    /* block comment inside a list */
    (def-struct Vector3
        (field x f32 (default 0.0))
        (field y f32 (default 0.0))
        (field z f32 (default 0.0))
    )
    /* multi line
     * block comment
     * inside a list
     */
    (def-table Monster
        (field position Vector3)
        (field target Vector3)
        (field name string (default "default-monster"))
    )
)
(namespace engine
    // Comment inside a list
    /* block comment inside a list */
    /* multi line
     * block comment
     * inside a list
     */
    (/* A comment*/ def-table Soldier
        (field position aleph::Vector3)
        (field target aleph::Vector3)
        (field health f64 (default 1_000_000.0))
    )
    ()
)

"#;

const VALID_DOC_COMMENT: &'static str = r#"
(namespace aleph
    /// This is a doc comment
    /// This is also a doc comment
    (def-table Monster
        (field position Vector3)
        (field target Vector3)
        (field name string (default "default-monster"))
    )
)
/// Root doc comment
"#;

const VALID_EMPTY_FILE: &'static str = r#""#;

const VALID_EMPTY_FILE_WITH_WHITESPACE: &'static str = r#"










"#;

const VALID_EMPTY_FILE_WITH_WHITESPACE_AND_COMMENTS: &'static str = r#"









// There's also comments
// yeet

/*
 * test pack please ignore
 *
 */
"#;

const VALID_ESCAPED_STRING: &'static str = r#"// Top level comment
("Test \x74ext please \"ignore\" me")
("Test text please \"ignore\" me")
("Test \
        \u{74}ext please \"ignore\" me")"#;

const VALID_ONLY_COMMENT: &'static str = r#"// This file consists purely of comments
/* Do not be alarmed
 * comments are good
 */"#;

const VALID_SPECIAL_IDENT: &'static str = r#"// Top level comment
(namespace aleph
    (#first)
    (?second?)
    (-third)
)
"#;

fn default(default: crate::ast::Atom) -> crate::ast::ListBuilder {
    crate::ast::ListBuilder::new()
        .add_word("default", None)
        .add_atom(default, None)
}

fn default_0_float() -> crate::ast::ListBuilder<'static> {
    default(crate::ast::Atom::word("0.0"))
}

fn field<'input>(name: &'input str, field_type: &'input str) -> crate::ast::ListBuilder<'input> {
    crate::ast::ListBuilder::new()
        .add_word("field", None)
        .add_word(name, None)
        .add_word(field_type, None)
}

fn field_default<'input, L: Into<crate::ast::List<'input>>>(
    name: &'input str,
    field_type: &'input str,
    default: L,
) -> crate::ast::ListBuilder<'input> {
    crate::ast::ListBuilder::new()
        .add_word("field", None)
        .add_word(name, None)
        .add_word(field_type, None)
        .add_list(default, None)
}

fn float_field_default_0(name: &str) -> crate::ast::ListBuilder {
    field_default(name, "f32", default_0_float())
}

fn vector2() -> crate::ast::ListBuilder<'static> {
    crate::ast::ListBuilder::new()
        .add_word("def-struct", None)
        .add_word("Vector2", None)
        .add_list(float_field_default_0("x"), None)
        .add_list(float_field_default_0("y"), None)
}

fn vector3() -> crate::ast::ListBuilder<'static> {
    crate::ast::ListBuilder::new()
        .add_word("def-struct", None)
        .add_word("Vector3", None)
        .add_list(float_field_default_0("x"), None)
        .add_list(float_field_default_0("y"), None)
        .add_list(float_field_default_0("z"), None)
}

fn monster() -> crate::ast::ListBuilder<'static> {
    let default_name = default(crate::ast::Atom::string("default-monster"));
    crate::ast::ListBuilder::new()
        .add_word("def-table", None)
        .add_word("Monster", None)
        .add_list(field("position", "Vector3"), None)
        .add_list(field("target", "Vector3"), None)
        .add_list(field_default("name", "string", default_name), None)
}

fn soldier() -> crate::ast::ListBuilder<'static> {
    let default_health = default(crate::ast::Atom::word("1_000_000.0"));
    crate::ast::ListBuilder::new()
        .add_word("def-table", None)
        .add_word("Soldier", None)
        .add_list(field("position", "aleph::Vector3"), None)
        .add_list(field("target", "aleph::Vector3"), None)
        .add_list(field_default("health", "f64", default_health), None)
}

#[test]
fn test_valid() {
    let namespace_aleph = crate::ast::ListBuilder::new()
        .add_word("namespace", None)
        .add_word("aleph", None)
        .add_list(vector2(), None)
        .add_list(vector3(), None)
        .add_list(monster(), None);
    let namespace_engine = crate::ast::ListBuilder::new()
        .add_word("namespace", None)
        .add_word("engine", None)
        .add_list(soldier(), None)
        .add_list([], None);
    let expected = crate::ast::ListBuilder::new()
        .add_list(namespace_aleph, None)
        .add_list(namespace_engine, None);
    test_parses_valid(VALID, expected);
}

#[test]
fn test_valid_empty_file() {
    let expected = crate::ast::ListBuilder::new();
    test_parses_valid(VALID_EMPTY_FILE, expected);
}

#[test]
fn test_valid_empty_file_with_whitespace_and_comments() {
    let expected = crate::ast::ListBuilder::new();
    test_parses_valid(VALID_EMPTY_FILE_WITH_WHITESPACE_AND_COMMENTS, expected);
}

#[test]
fn test_valid_only_comments() {
    let expected = crate::ast::ListBuilder::new();
    test_parses_valid(VALID_ONLY_COMMENT, expected);
}

#[test]
fn test_valid_empty_file_with_whitespace() {
    let expected = crate::ast::ListBuilder::new();
    test_parses_valid(VALID_EMPTY_FILE_WITH_WHITESPACE, expected);
}

#[test]
fn test_valid_escaped_string() {
    let string_a = "Test \\x74ext please \\\"ignore\\\" me";
    let string_b = "Test text please \\\"ignore\\\" me";
    let string_c = "Test \\\n        \\u{74}ext please \\\"ignore\\\" me";
    let list_a = crate::ast::ListBuilder::new()
        .add_string(string_a, None)
        .build();
    let list_b = crate::ast::ListBuilder::new()
        .add_string(string_b, None)
        .build();
    let list_c = crate::ast::ListBuilder::new()
        .add_string(string_c, None)
        .build();
    let expected = crate::ast::ListBuilder::new()
        .add_list(list_a, None)
        .add_list(list_b, None)
        .add_list(list_c, None);
    test_parses_valid(VALID_ESCAPED_STRING, expected);
}

#[test]
fn test_valid_special_idents() {
    let first = crate::ast::ListBuilder::new().add_word("#first", None);
    let second = crate::ast::ListBuilder::new().add_word("?second?", None);
    let third = crate::ast::ListBuilder::new().add_word("-third", None);
    let namespace_aleph = crate::ast::ListBuilder::new()
        .add_word("namespace", None)
        .add_word("aleph", None)
        .add_list(first, None)
        .add_list(second, None)
        .add_list(third, None);
    let expected = crate::ast::ListBuilder::new().add_list(namespace_aleph, None);
    test_parses_valid(VALID_SPECIAL_IDENT, expected);
}

#[test]
fn test_valid_doc_comment() {
    let first = crate::ast::ListBuilder::new()
        .add_word("!doc", None)
        .add_string(" This is a doc comment", None);
    let second = crate::ast::ListBuilder::new()
        .add_word("!doc", None)
        .add_string(" This is also a doc comment", None);
    let last = crate::ast::ListBuilder::new()
        .add_word("!doc", None)
        .add_string(" Root doc comment", None);
    let namespace_aleph = crate::ast::ListBuilder::new()
        .add_word("namespace", None)
        .add_word("aleph", None)
        .add_list(first, None)
        .add_list(second, None)
        .add_list(monster(), None);
    let expected = crate::ast::ListBuilder::new()
        .add_list(namespace_aleph, None)
        .add_list(last, None);
    test_parses_valid(VALID_DOC_COMMENT, expected);
}

#[test]
fn test_invalid_unterminated_list() {
    let expected = lalrpop_util::ParseError::UnrecognizedEOF {
        location: 37,
        expected: vec![
            "\"(\"".to_string(),
            "\")\"".to_string(),
            "\"string\"".to_string(),
            "\"word\"".to_string(),
        ],
    };
    test_parses_invalid(INVALID_UNTERMINATED_LIST, expected);
}

#[test]
fn test_invalid_unterminated_string() {
    let expected = lalrpop_util::ParseError::User {
        error: Error::UnclosedStringLiteral { begin: 31 },
    };
    test_parses_invalid(INVALID_UNTERMINATED_STRING, expected);
}

#[test]
fn test_invalid_terminating_too_many_lists() {
    let expected = lalrpop_util::ParseError::UnrecognizedToken {
        token: (37, Tok::ParenClose, 38),
        expected: vec!["\"(\"".to_string()],
    };
    test_parses_invalid(INVALID_TERMINATING_TOO_MANY_LISTS, expected);
}

#[test]
fn test_invalid_terminating_unopened_list() {
    let expected = lalrpop_util::ParseError::UnrecognizedToken {
        token: (1, Tok::ParenClose, 2),
        expected: vec!["\"(\"".to_string()],
    };
    test_parses_invalid(INVALID_TERMINATING_UNOPENED_LIST, expected);
}

#[test]
fn test_invalid_root_atom() {
    let expected = lalrpop_util::ParseError::UnrecognizedToken {
        token: (0, Tok::Word("namespace"), 9),
        expected: vec!["\"(\"".to_string()],
    };
    test_parses_invalid(INVALID_ROOT_ATOM, expected);
}

fn test_parses_valid<'input, L: Into<crate::ast::List<'input>>>(text: &'input str, expected: L) {
    let expected = expected.into();
    let lexer = crate::lexer::Lexer::new(&text);
    let parser = crate::parser::FileParser::new();
    match parser.parse(lexer) {
        Ok(output) => {
            assert_eq!(
                &output, &expected,
                "ASTs do not match\nGOT:\n{:#?}\n---------------\nEXPECTED:\n{:#?}",
                &output, &expected
            );
        }
        Err(error) => {
            panic!("{:#?}\nCorrectly failed to parse a valid input", error);
        }
    }
}

fn test_parses_invalid(text: &str, expected: lalrpop_util::ParseError<usize, Tok, Error>) {
    let lexer = crate::lexer::Lexer::new(text);
    let parser = crate::parser::FileParser::new();
    match parser.parse(lexer) {
        Ok(output) => {
            println!("{:#?}", output);
            panic!("Successfully parsed an invalid input");
        }
        Err(error) => {
            assert!(error.eq(&expected),
                "Failed to parse an invalid input, but in the incorrect way.\nGOT:\n{:#?}\n---------------\nEXPECTED:\n{:#?}",
                &error, &expected
            );
        }
    }
}
