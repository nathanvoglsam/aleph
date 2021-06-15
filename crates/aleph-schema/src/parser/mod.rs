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

mod parsers;

use crate::ast::{
    Field, FieldType, Module, ModuleItem, ModuleItemType, PrimitiveType, Struct, Table,
};
use crate::parser::Error::UnexpectedWord;
use smartstring::alias::CompactString;
use std::collections::HashSet;
use std::hash::{Hash, Hasher};
use std::ops::Range;

#[derive(Clone, PartialEq, Eq, Debug)]
pub enum Error {
    ExpectedWord {
        span: Range<usize>,
    },
    ExpectedAttribute {
        span: Range<usize>,
    },
    UnexpectedWord {
        span: Range<usize>,
        expected: Vec<String>,
    },
    UnexpectedItem {
        span: Range<usize>,
        expected: Vec<sexpr::ast::ItemVariantType>,
    },
    DuplicateEntity {
        span: Range<usize>,
        duplicate: Range<usize>,
    },
}

pub struct Parser<'input> {
    input: sexpr::ast::List<'input>,
}

impl<'input> Parser<'input> {
    pub fn new(input: sexpr::ast::List<'input>) -> Self {
        Self { input }
    }

    pub fn parse(&self) -> Result<crate::ast::Module, Error> {
        // Use a module stack for parsing rather than recursive functions
        let mut module_stack = Vec::new();
        let top = (
            CompactString::new(),
            HashSet::new(),
            self.input.as_slice().iter(),
            Module {
                position: 0..0,
                children: Vec::new(),
                attributes: Vec::new(),
            },
        );
        module_stack.push(top);

        // A list we accumulate attributes into which will get applied to the next item
        let mut attributes = Vec::new();

        loop {
            let top = module_stack.last_mut().unwrap();
            if let Some(item) = top.2.next() {
                if let Some(list) = item.item.list() {
                    // If the word starts with '!' then the item is an attribute
                    if Self::try_parse_attribute(&mut attributes, list, item.span.clone()).is_ok() {
                        continue;
                    }

                    let first = Self::get_word(list, item.span.clone(), 0)?;
                    let item_type = Self::identify_module_item(first.0, first.1)?;

                    match item_type {
                        ModuleItemType::Module => {
                            let name: NameWithSpan<'input> =
                                Self::get_word(list, item.span.clone(), 1)?.into();

                            if !top.1.insert(name.clone()) {
                                let duplicate = top.1.get(&name).unwrap().span.clone();
                                let error = Error::DuplicateEntity {
                                    span: name.span.clone(),
                                    duplicate,
                                };
                                return Err(error);
                            }

                            let child = (
                                CompactString::from(name.name),
                                HashSet::new(),
                                list[2..].iter(),
                                Module {
                                    position: item.span.clone(),
                                    children: Vec::new(),
                                    attributes: std::mem::take(&mut attributes),
                                },
                            );
                            module_stack.push(child);
                        }
                        ModuleItemType::Struct => Self::parse_struct(
                            &mut top.3,
                            &mut top.1,
                            &mut attributes,
                            list,
                            item.span.clone(),
                        )?,
                        ModuleItemType::Table => Self::parse_table(
                            &mut top.3,
                            &mut top.1,
                            &mut attributes,
                            list,
                            item.span.clone(),
                        )?,
                        ModuleItemType::Enum => {}
                    }
                } else {
                    let error = Error::UnexpectedItem {
                        span: item.span.clone(),
                        expected: vec![sexpr::ast::ItemVariantType::List],
                    };
                    return Err(error);
                }
            } else {
                let top = module_stack.pop().unwrap();
                if let Some(parent) = module_stack.last_mut() {
                    let item = (top.0, ModuleItem::Module(top.3));
                    parent.3.children.push(item);
                } else {
                    module_stack.push(top);
                    break;
                }
            }
        }

        Ok(module_stack.pop().unwrap().3)
    }

    fn parse_struct(
        into: &mut Module<'input>,
        names: &mut HashSet<NameWithSpan<'input>>,
        attributes: &mut Vec<sexpr::ast::List<'input>>,
        input: &sexpr::ast::List<'input>,
        span: Range<usize>,
    ) -> Result<(), Error> {
        let name: NameWithSpan<'input> = Self::get_word(input, span.clone(), 1)?.into();

        if !names.insert(name.clone()) {
            let duplicate = names.get(&name).unwrap().span.clone();
            let error = Error::DuplicateEntity {
                span: name.span.clone(),
                duplicate,
            };
            return Err(error);
        }

        let name = CompactString::from(name.name);
        let mut v = Struct {
            position: span.clone(),
            fields: Vec::new(),
            attributes: std::mem::take(attributes),
        };

        Self::parse_struct_like(&mut v.fields, attributes, input, span.clone())?;

        let v = ModuleItem::Struct(v);
        into.children.push((name, v));
        Ok(())
    }

    fn parse_table(
        into: &mut Module<'input>,
        names: &mut HashSet<NameWithSpan<'input>>,
        attributes: &mut Vec<sexpr::ast::List<'input>>,
        input: &sexpr::ast::List<'input>,
        span: Range<usize>,
    ) -> Result<(), Error> {
        let name: NameWithSpan<'input> = Self::get_word(input, span.clone(), 1)?.into();

        if !names.insert(name.clone()) {
            let duplicate = names.get(&name).unwrap().span.clone();
            let error = Error::DuplicateEntity {
                span: name.span.clone(),
                duplicate,
            };
            return Err(error);
        }

        let name = CompactString::from(name.name);
        let mut v = Table {
            position: span.clone(),
            fields: Vec::new(),
            attributes: std::mem::take(attributes),
        };

        Self::parse_struct_like(&mut v.fields, attributes, input, span)?;

        let v = ModuleItem::Table(v);
        into.children.push((name, v));
        Ok(())
    }

    fn parse_struct_like(
        fields: &mut Vec<(CompactString, Field<'input>)>,
        attributes: &mut Vec<sexpr::ast::List<'input>>,
        input: &sexpr::ast::List<'input>,
        span: Range<usize>,
    ) -> Result<(), Error> {
        let mut field_names = HashSet::new();

        for item in &input[2..] {
            if let Some(list) = item.item.list() {
                if Self::try_parse_attribute(attributes, list, span.clone()).is_ok() {
                    continue;
                }

                let first = Self::get_word(list, span.clone(), 0)?;
                if first.1 == "field" {
                    Self::parse_field(
                        fields,
                        &mut field_names,
                        attributes,
                        list,
                        item.span.clone(),
                    )?;
                    continue;
                } else {
                    let error = UnexpectedWord {
                        span: first.0,
                        expected: vec!["field".to_string()],
                    };
                    return Err(error);
                }
            } else {
                let error = Error::UnexpectedItem {
                    span: item.span.clone(),
                    expected: vec![sexpr::ast::ItemVariantType::List],
                };
                return Err(error);
            }
        }

        Ok(())
    }

    fn parse_field(
        into: &mut Vec<(CompactString, Field<'input>)>,
        names: &mut HashSet<NameWithSpan<'input>>,
        attributes: &mut Vec<sexpr::ast::List<'input>>,
        input: &sexpr::ast::List<'input>,
        span: Range<usize>,
    ) -> Result<(), Error> {
        let name = Self::get_word(input, span.clone(), 1)?.into();

        if let Some(duplicate) = names.get(&name) {
            let duplicate = duplicate.span.clone();
            let error = Error::DuplicateEntity {
                span: name.span.clone(),
                duplicate,
            };
            return Err(error);
        }
        names.insert(name.clone());

        let name = CompactString::from(name.name);

        let r#type = Self::get_word(input, span.clone(), 2)?;
        let r#type = Self::parse_type_name(r#type.1);

        let v = Field {
            position: span,
            attributes: std::mem::take(attributes),
            r#type,
        };

        into.push((name, v));
        Ok(())
    }

    fn parse_type_name(input: &str) -> FieldType {
        let output = match input {
            "u8" => FieldType::Primitive(PrimitiveType::U8),
            "u16" => FieldType::Primitive(PrimitiveType::U16),
            "u32" => FieldType::Primitive(PrimitiveType::U32),
            "u64" => FieldType::Primitive(PrimitiveType::U64),
            "i8" => FieldType::Primitive(PrimitiveType::I8),
            "i16" => FieldType::Primitive(PrimitiveType::I16),
            "i32" => FieldType::Primitive(PrimitiveType::I32),
            "i64" => FieldType::Primitive(PrimitiveType::I64),
            "f32" => FieldType::Primitive(PrimitiveType::F32),
            "f64" => FieldType::Primitive(PrimitiveType::F64),
            "bool" => FieldType::Primitive(PrimitiveType::Bool),
            _ => FieldType::StructRef(CompactString::from(input)),
        };
        output
    }

    fn try_parse_attribute(
        into: &mut Vec<sexpr::ast::List<'input>>,
        input: &sexpr::ast::List<'input>,
        span: Range<usize>,
    ) -> Result<(), Error> {
        let first = Self::get_word(input, span.clone(), 0)?;

        // If the word starts with '!' then the item is an attribute
        if first.1.starts_with('!') {
            into.push(input.clone());
            Ok(())
        } else {
            let error = Error::ExpectedWord { span };
            Err(error)
        }
    }

    fn get_word(
        list: &sexpr::ast::List<'input>,
        span: Range<usize>,
        index: usize,
    ) -> Result<(Range<usize>, &'input str), Error> {
        let first = list.get(index).ok_or_else(|| Error::ExpectedWord {
            span: span.end..span.end,
        })?;
        let atom = first.item.atom().ok_or_else(|| Error::ExpectedWord {
            span: first.span.clone(),
        })?;
        atom.as_word()
            .ok_or_else(|| Error::ExpectedWord {
                span: first.span.clone(),
            })
            .map(|v| (first.span.clone(), v))
    }

    fn identify_module_item(
        span: Range<usize>,
        word: &str,
    ) -> Result<crate::ast::ModuleItemType, Error> {
        match word {
            "struct" => Ok(crate::ast::ModuleItemType::Struct),
            "table" => Ok(crate::ast::ModuleItemType::Table),
            "mod" => Ok(crate::ast::ModuleItemType::Module),
            "enum" => Ok(crate::ast::ModuleItemType::Enum),
            _ => Err(Error::UnexpectedWord {
                span,
                expected: vec![
                    "struct".to_string(),
                    "table".to_string(),
                    "mod".to_string(),
                    "enum".to_string(),
                ],
            }),
        }
    }
}

#[derive(Clone, Debug)]
struct NameWithSpan<'a> {
    pub span: Range<usize>,
    pub name: &'a str,
}

impl<'a> Hash for NameWithSpan<'a> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.name.hash(state)
    }
}

impl<'a> PartialEq for NameWithSpan<'a> {
    fn eq(&self, other: &Self) -> bool {
        self.name.eq(other.name)
    }
}

impl<'a> Eq for NameWithSpan<'a> {}

impl<'a> From<(Range<usize>, &'a str)> for NameWithSpan<'a> {
    fn from(v: (Range<usize>, &'a str)) -> Self {
        Self {
            span: v.0,
            name: v.1,
        }
    }
}
