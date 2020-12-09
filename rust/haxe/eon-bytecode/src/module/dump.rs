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

use crate::module::Module;
use crate::native::Native;
use crate::type_::Type;
use crate::indexes::TypeIndex;
use crate::function::Function;
use std::fmt::Write;

pub fn dump_to_string(module: &Module) -> Result<String, std::fmt::Error> {
    let mut string = String::new();

    dump_int_table(&mut string, 4, &module.ints)?;
    dump_float_table(&mut string, 4, &module.floats)?;
    dump_string_table(&mut string, 4, &module.strings)?;

    // SKIP BYTES SECTION FOR NOW

    dump_debug_files(&mut string, 4, &module.debug_files)?;
    dump_types(&mut string, module, 4, &module.types)?;
    dump_natives(&mut string, module, 4, &module.natives)?;
    dump_globals(&mut string, module, 4, &module.globals)?;
    dump_functions(&mut string, module, 4, &module.functions)?;

    Ok(string)
}

fn dump_int_table(string: &mut String, indent: usize, table: &[i32]) -> std::fmt::Result {
    let len = table.len() as f64;
    let len = len.log10() as usize + 1;

    writeln!(string, "[Int Table]")?;
    for (i, v) in table.iter().enumerate() {
        writeln!(
            string,
            "{space:indent$}#{index:<align$} {val}",
            space = " ",
            indent = indent,
            index = i,
            align = len,
            val = *v
        )?;
    }
    writeln!(string)?;
    Ok(())
}

fn dump_float_table(string: &mut String, indent: usize, table: &[f64]) -> std::fmt::Result {
    let len = table.len() as f64;
    let len = len.log10() as usize + 1;

    writeln!(string, "[Float Table]")?;
    for (i, v) in table.iter().enumerate() {
        writeln!(
            string,
            "{space:indent$}#{index:<align$} {val}",
            space = " ",
            indent = indent,
            index = i,
            align = len,
            val = *v
        )?;
    }
    writeln!(string)?;
    Ok(())
}

fn dump_string_table(string: &mut String, indent: usize, table: &[String]) -> std::fmt::Result {
    let len = table.len() as f64;
    let len = len.log10() as usize + 1;

    writeln!(string, "[String Table]")?;
    for (i, v) in table.iter().enumerate() {
        writeln!(
            string,
            "{space:indent$}#{index:<align$} {val}",
            space = " ",
            indent = indent,
            index = i,
            align = len,
            val = *v
        )?;
    }
    writeln!(string)?;
    Ok(())
}

fn dump_debug_files(string: &mut String, indent: usize, files: &[String]) -> std::fmt::Result {
    let len = files.len() as f64;
    let len = len.log10() as usize + 1;

    writeln!(string, "[Debug Files]")?;
    for (i, v) in files.iter().enumerate() {
        writeln!(
            string,
            "{space:indent$}#{index:<align$} {val}",
            space = " ",
            indent = indent,
            index = i,
            align = len,
            val = *v
        )?;
    }
    writeln!(string)?;
    Ok(())
}

fn dump_natives(
    string: &mut String,
    module: &Module,
    indent: usize,
    natives: &[Native],
) -> std::fmt::Result {
    let len = natives.len();
    let len = len.to_string();
    let len = len.chars().count();

    writeln!(string, "[Natives]")?;
    for (i, v) in natives.iter().enumerate() {
        writeln!(
            string,
            "{space:indent$}#{index:<align$} {lib}::{name} = ({ty}) {func}",
            space = " ",
            indent = indent,
            index = i,
            align = len,
            lib = module.strings[v.lib.0],
            name = module.strings[v.name.0],
            ty = v.type_.0,
            func = v.f_index,
        )?;
    }
    writeln!(string)?;
    Ok(())
}

fn dump_types(
    string: &mut String,
    module: &Module,
    indent: usize,
    types: &[Type],
) -> std::fmt::Result {
    let len = types.len();
    let len = len.to_string();
    let len = len.chars().count();

    writeln!(string, "[Types]")?;
    for (i, v) in types.iter().enumerate() {
        writeln!(
            string,
            "{space:indent$}#{index:<align$} {name}",
            space = " ",
            indent = indent,
            index = i,
            align = len,
            name = get_type_name(module, v).unwrap(),
        )?;
    }
    writeln!(string)?;
    Ok(())
}

fn dump_globals(
    string: &mut String,
    module: &Module,
    indent: usize,
    globals: &[TypeIndex],
) -> std::fmt::Result {
    let len = globals.len();
    let len = len.to_string();
    let len = len.chars().count();

    writeln!(string, "[Globals]")?;
    for (i, v) in globals.iter().enumerate() {
        writeln!(
            string,
            "{space:indent$}#{index:<align$} {name}",
            space = " ",
            indent = indent,
            index = i,
            align = len,
            name = get_type_name(module, &module.types[v.0]).unwrap(),
        )?;
    }
    writeln!(string)?;
    Ok(())
}

fn dump_functions(
    string: &mut String,
    module: &Module,
    indent: usize,
    functions: &[Function],
) -> std::fmt::Result {
    let len = functions.len();
    let len = len.to_string();
    let len = len.chars().count();

    writeln!(string, "[Functions]")?;
    for (i, v) in functions.iter().enumerate() {
        writeln!(
            string,
            "#{index:<align$} = [{fidx}] {ty}",
            index = i,
            align = len,
            fidx = v.f_index,
            ty = get_type_name(module, &module.types[v.type_.0]).unwrap()
        )?;

        let ssa_len = v.ssa_values.len();
        let ssa_len = ssa_len.to_string();
        let ssa_len = ssa_len.chars().count();
        writeln!(
            string,
            "{space:indent$}[Values]",
            space = " ",
            indent = indent
        )?;
        for (i, ssa_val) in v.ssa_values.iter().enumerate() {
            writeln!(
                string,
                "{space:indent$}#{index:<align$} {name}",
                space = " ",
                indent = indent * 2,
                index = i,
                align = ssa_len,
                name = get_type_name(module, &module.types[ssa_val.type_.0]).unwrap()
            )?;
        }

        writeln!(
            string,
            "{space:indent$}[Code]",
            space = " ",
            indent = indent
        )?;
        for (i, bb) in v.basic_blocks.iter().enumerate() {
            writeln!(
                string,
                "{space:indent$}:${bb:<align$}",
                space = " ",
                indent = indent * 2,
                bb = i,
                align = ssa_len,
            )?;
            for op in bb.ops.iter() {
                writeln!(
                    string,
                    "{space:indent$}{op}",
                    space = " ",
                    indent = indent * 3,
                    op = op.to_pretty_string(module)
                )?;
            }
        }
    }
    writeln!(string)?;
    Ok(())
}

fn get_type_name(module: &Module, t: &Type) -> Option<String> {
    match t {
        Type::Void => Some("void".to_string()),
        Type::U8 => Some("u8".to_string()),
        Type::U16 => Some("u16".to_string()),
        Type::I32 => Some("i32".to_string()),
        Type::I64 => Some("i64".to_string()),
        Type::F32 => Some("f32".to_string()),
        Type::F64 => Some("f64".to_string()),
        Type::Bool => Some("bool".to_string()),
        Type::Bytes => Some("bytes".to_string()),
        Type::Dynamic => Some("dyn".to_string()),
        Type::Array(v) => Some(format!(
            "array<{}>",
            get_type_name(module, &module.types[v.type_.0])?
        )),
        Type::Type => Some("type".to_string()),
        Type::DynObject => Some("dyn_object".to_string()),
        Type::Function(v) => {
            let mut string = String::new();
            write!(&mut string, "fn(").ok()?;
            for a in &v.args {
                write!(
                    &mut string,
                    "{}, ",
                    get_type_name(module, &module.types[a.0])?
                )
                    .ok()?;
            }
            write!(
                &mut string,
                ") -> {}",
                get_type_name(module, &module.types[v.returns.0])?
            )
                .ok()?;
            Some(string)
        }
        Type::Method(v) => {
            let mut string = String::new();
            write!(&mut string, "method(").ok()?;
            for a in &v.args {
                write!(
                    &mut string,
                    "{}, ",
                    get_type_name(module, &module.types[a.0])?
                )
                    .ok()?;
            }
            write!(
                &mut string,
                ") -> {}",
                get_type_name(module, &module.types[v.returns.0])?
            )
                .ok()?;
            Some(string)
        }
        Type::Ref(v) => Some(format!(
            "ref<{}>",
            get_type_name(module, &module.types[v.type_.0])?
        )),
        Type::Null(v) => Some(format!(
            "null<{}>",
            get_type_name(module, &module.types[v.type_.0])?
        )),
        Type::Obj(v) => Some(format!("obj<{}>", &module.strings[v.name.0])),
        Type::Struct(v) => Some(format!("struct<{}>", &module.strings[v.name.0])),
        Type::Virtual(v) => {
            let mut string = String::new();
            write!(&mut string, "virtual(").ok()?;
            for a in &v.fields {
                let type_str = get_type_name(module, &module.types[a.type_.0])?;
                let name_str = &module.strings[a.name.0];
                write!(&mut string, "{}: {}, ", name_str, type_str).ok()?;
            }
            write!(&mut string, ")").ok()?;
            Some(string)
        }
        Type::Abstract(v) => Some(format!("abstract<{}>", &module.strings[v.name.0])),
        Type::Enum(v) => Some(format!("enum<{}>", &module.strings[v.name.0])),
    }
}