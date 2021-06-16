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

use crate::ast::{
    Enum, EnumVariant, Field, FieldType, Module, ModuleItem, PrimitiveType, Struct, Table,
};
use crate::parser::Error;

#[test]
pub fn test_valid_enum() {
    let mut first_variant = EnumVariant::default();
    first_variant
        .fields
        .push(FieldType::Primitive(PrimitiveType::I16).into());

    let mut second_variant = EnumVariant::default();
    second_variant
        .fields
        .push(FieldType::Primitive(PrimitiveType::I32).into());
    second_variant
        .fields
        .push(FieldType::Primitive(PrimitiveType::F64).into());

    let mut static_mesh_enum = Enum::default();
    static_mesh_enum.attributes.push(
        sexpr::ast::ListBuilder::new()
            .add_word("!doc", None)
            .add_string(" A static mesh", None)
            .build(),
    );
    static_mesh_enum
        .variants
        .push(("I16".into(), first_variant));
    static_mesh_enum
        .variants
        .push(("I32".into(), second_variant));

    let mut aleph_module = Module::default();
    aleph_module
        .children
        .push(("StaticMesh".into(), ModuleItem::from(static_mesh_enum)));

    let mut expected = Module::default();
    expected
        .children
        .push(("aleph".into(), ModuleItem::from(aleph_module)));

    test_valid_file("./schemas/valid_enum.schema", expected);
}

#[test]
pub fn test_valid_nested_modules() {
    let mesh = Module::default();

    let mut aleph = Module::default();
    aleph.children.push(("mesh".into(), mesh.into()));

    let mut expected = Module::default();
    expected.children.push(("aleph".into(), aleph.into()));

    test_valid_file("./schemas/valid_nested_modules.schema", expected);
}

#[test]
pub fn test_valid_struct() {
    let f32_field: Field<'_> = FieldType::Primitive(PrimitiveType::F32).into();
    let mut vector2 = Struct::default();
    vector2.fields.push(("x".into(), f32_field.clone()));
    vector2.fields.push(("y".into(), f32_field.clone()));
    vector2.attributes.push(
        sexpr::ast::ListBuilder::new()
            .add_word("!doc", None)
            .add_string(" A two component vector of floats", None)
            .build(),
    );

    let mut aleph = Module::default();
    aleph.children.push(("Vector2".into(), vector2.into()));

    let mut expected = Module::default();
    expected.children.push(("aleph".into(), aleph.into()));

    test_valid_file("./schemas/valid_struct.schema", expected);
}

#[test]
pub fn test_valid_table() {
    let mut transform_field: Field<'_> = FieldType::StructRef("Transform".into()).into();
    transform_field.attributes.push(
        sexpr::ast::ListBuilder::new()
            .add_word("!doc", None)
            .add_string(" The human\'s position in world space", None)
            .build(),
    );

    let mut string_field: Field<'_> = FieldType::StructRef("string".into()).into();
    string_field.attributes.push(
        sexpr::ast::ListBuilder::new()
            .add_word("!doc", None)
            .add_string(" The human\'s name", None)
            .build(),
    );

    let mut f32_field: Field<'_> = FieldType::Primitive(PrimitiveType::F32).into();
    f32_field.attributes.push(
        sexpr::ast::ListBuilder::new()
            .add_word("!doc", None)
            .add_string(" The human\'s health", None)
            .build(),
    );

    let mut human = Table::default();
    human.fields.push(("transform".into(), transform_field));
    human.fields.push(("name".into(), string_field));
    human.fields.push(("health".into(), f32_field));
    human.attributes.push(
        sexpr::ast::ListBuilder::new()
            .add_word("!doc", None)
            .add_string(" A human", None)
            .build(),
    );

    let mut aleph = Module::default();
    aleph.children.push(("Human".into(), human.into()));

    let mut expected = Module::default();
    expected.children.push(("aleph".into(), aleph.into()));

    test_valid_file("./schemas/valid_table.schema", expected);
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

    assert_eq!(
        &err, &expected,
        "EXPECTED:\n{:#?}\nGOT:\n{:#?}",
        &expected, &err
    );
}

fn test_valid_file(file: &str, expected: crate::ast::Module) {
    let text = std::fs::read_to_string(file).unwrap();
    let text = text.replace("\r\n", "\n");

    let sexpr_lexer = sexpr::lexer::Lexer::new(text.as_str());
    let sexpr_parser = sexpr::parser::FileParser::new();
    let sexpr_tree = sexpr_parser.parse(sexpr_lexer).unwrap();

    let ast_parser = crate::parser::Parser::new(sexpr_tree);
    let ast = ast_parser.parse().unwrap();

    assert_eq!(
        &ast, &expected,
        "EXPECTED:\n{:#?}\nGOT:\n{:#?}",
        &expected, &ast
    );
}
