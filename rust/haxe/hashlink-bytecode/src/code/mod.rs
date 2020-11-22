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

use byteorder::{LittleEndian, ReadBytesExt};
use std::io::Read;

use crate::{
    CodeReadError, Constant, EnumConstruct, Field, Function, Native, ObjectProto, OpCallNParam,
    OpCode, OpCodeNumber, OpCodeType, OpFiveParam, OpFourParam, OpOneParam, OpSixParam,
    OpSwitchParam, OpThreeParam, OpTwoParam, Result, Type, TypeAbstract, TypeEnum, TypeFunction,
    TypeKind, TypeObject, TypeParam, TypeVariant, TypeVirtual, Version,
};
use serde::{Deserialize, Serialize};

const HEADER_H: u8 = 0x48;
const HEADER_L: u8 = 0x4C;
const HEADER_B: u8 = 0x42;

/// This struct is a direct representation of a hashlink module *as read from disk*. The original C
/// hashlink code deserializes directly into the datastructures used by the JIT and runtime. This
/// implementation is completely distinct from any runtime and serves purely as a utility for
/// reading, operating on and writing hashlink modules so any information that is not read directly
/// from a hashlink file or is only used by the runtime is not stored here.
///
/// This struct can be used as a component for reading hashlink modules to be consumed by a JIT
/// runtime but is not appropriate to be consumed directly by the runtime.
#[derive(Clone, Serialize, Deserialize)]
pub struct Code {
    /// The version of the bytecode file that was read from disk
    pub version: Version,

    /// A general flags field used by the file format
    pub flags: u32,

    /// The file's integer table
    pub ints: Vec<i32>,

    /// The file's float table
    pub floats: Vec<f64>,

    /// The file's string table
    pub strings: Vec<String>,

    /// The file's bytes blob
    pub bytes: Vec<u8>,

    /// The file's byte offets table
    pub byte_offsets: Vec<usize>,

    /// The file's debug file table
    pub debug_files: Vec<String>,

    /// The file's type table
    pub types: Vec<Type>,

    /// The file's natives table
    pub natives: Vec<Native>,

    /// The file's global table (list of indices into type table)
    pub globals: Vec<u32>,

    /// The file's function table
    pub functions: Vec<Function>,

    /// The file's constants table
    pub constants: Vec<Constant>,

    /// Index into the functions table for specifying which function is the entrypoint
    pub entrypoint: u32,
}

impl Code {
    /// Will attempt to read a hashlink module from the given stream
    pub fn read(stream: &mut impl Read) -> Result<Code> {
        //
        // READ AND VALIDATE FLE HEADER
        //
        let h = stream.read_u8()?;
        let l = stream.read_u8()?;
        let b = stream.read_u8()?;
        if h != HEADER_H || l != HEADER_L || b != HEADER_B {
            return Err(CodeReadError::InvalidFileHeader);
        }

        // Read and validate file version
        let version = stream.read_u8()?;
        let version = Version::from_raw(version).ok_or(CodeReadError::InvalidVersionUnknown)?;
        if !version.is_supported() {
            return Err(CodeReadError::InvalidVersionUnsupported);
        }

        // Load file description that lists flags and the sizes of various tables
        let flags = read_uindex(stream)? as u32;
        let num_ints = read_uindex(stream)?;
        let num_floats = read_uindex(stream)?;
        let num_strings = read_uindex(stream)?;
        let num_bytes = if version.has_bytes_table() {
            read_uindex(stream)?
        } else {
            0
        };
        let num_types = read_uindex(stream)?;
        let num_globals = read_uindex(stream)?;
        let num_natives = read_uindex(stream)?;
        let num_functions = read_uindex(stream)?;
        let num_constants = if version.has_constants_table() {
            read_uindex(stream)?
        } else {
            0
        };
        let entrypoint = read_uindex(stream)?;
        let has_debug = (flags & 1) != 0;

        // Read the integer table
        let mut ints = vec![0i32; num_ints as usize];
        stream.read_i32_into::<LittleEndian>(&mut ints)?;

        // Read the float table
        let mut floats = vec![0f64; num_floats as usize];
        stream.read_f64_into::<LittleEndian>(&mut floats)?;

        // Read the string table
        let strings = read_strings(num_strings as usize, stream)?;

        // Read the byte block and byte offset table
        let (bytes, byte_offsets) = if version.has_bytes_table() {
            read_bytes(num_bytes as usize, stream)?
        } else {
            (Vec::new(), Vec::new())
        };

        // Read off the debug file table, if it is marked as existing
        let debug_files = if has_debug {
            let num_debug_files = read_uindex(stream)?;
            let debug_files = read_strings(num_debug_files as usize, stream)?;
            debug_files
        } else {
            Vec::new()
        };

        // Read the type table
        let mut types = Vec::with_capacity(num_types as usize);
        for _ in 0..num_types {
            let type_ = read_type(stream, num_types, num_strings)?;
            types.push(type_);
        }

        // Read the globals table
        let mut globals = Vec::with_capacity(num_globals as usize);
        for _ in 0..num_globals {
            let global = get_type(stream, num_types)?;
            globals.push(global);
        }

        let mut natives = Vec::with_capacity(num_natives as usize);
        for _ in 0..num_natives {
            let lib = get_string(stream, num_strings)?;
            let name = get_string(stream, num_strings)?;
            let type_ = get_type(stream, num_types)?;
            let f_index = read_uindex(stream)?;
            natives.push(Native {
                lib,
                name,
                type_,
                f_index,
            });
        }

        let mut functions = Vec::with_capacity(num_functions as usize);
        for _ in 0..num_functions {
            functions.push(read_function(
                stream,
                num_types,
                debug_files.len() as _,
                has_debug,
            )?);
        }

        let mut constants = Vec::with_capacity(num_constants as usize);
        for _ in 0..num_constants {
            let global = read_uindex(stream)?;
            let num_fields = read_uindex(stream)?;

            let mut fields = Vec::with_capacity(num_fields as usize);
            for _ in 0..num_fields {
                fields.push(read_uindex(stream)?);
            }

            constants.push(Constant { global, fields });
        }

        let out = Code {
            version,
            flags,
            ints,
            floats,
            strings,
            bytes,
            byte_offsets,
            debug_files,
            types,
            natives,
            globals,
            functions,
            constants,
            entrypoint,
        };
        Ok(out)
    }
}

fn read_index(stream: &mut impl Read) -> Result<i32> {
    let b = stream.read_u8()? as i32;
    if (b & 0x80) == 0 {
        let out = b & 0x7F;
        Ok(out)
    } else if (b & 0x40) == 0 {
        let out = stream.read_u8()? as i32;
        let out = out | (b & 31) << 8;
        return if (b & 0x20) == 0 { Ok(out) } else { Ok(-out) };
    } else {
        let c = stream.read_u8()? as i32;
        let d = stream.read_u8()? as i32;
        let e = stream.read_u8()? as i32;
        let v = ((b & 31) << 24) | (c << 16) | (d << 8) | e;
        if (b & 0x20) == 0 {
            Ok(v)
        } else {
            Ok(-v)
        }
    }
}

fn read_uindex(stream: &mut impl Read) -> Result<u32> {
    let i = read_index(stream)?;
    if i < 0 {
        Err(CodeReadError::InvalidIndexUnsignedLessThanOne)
    } else {
        Ok(i as u32)
    }
}

fn read_strings(num_strings: usize, stream: &mut impl Read) -> Result<Vec<String>> {
    // The total number of bytes to read should be the first thing in the string table so we read
    // that first
    let byte_count = stream.read_i32::<LittleEndian>()? as usize;

    // Now we allocate a buffer and read that many bytes from the stream
    let mut bytes = vec![0u8; byte_count];
    stream.read_exact(&mut bytes)?;

    // Next is a table of the string lengths which we will want to read so we can extract the
    // strings
    let mut strings = Vec::with_capacity(num_strings);
    let mut head = 0;
    for _ in 0..num_strings {
        // Read the list
        let len = read_uindex(stream)?;
        let string = &bytes[head..head + len as usize];
        let string =
            std::str::from_utf8(string).map_err(|_| CodeReadError::InvalidStringNotValidUTF8)?;

        strings.push(string.to_string());

        head += len as usize + 1;
    }

    Ok(strings)
}

fn read_bytes(num_bytes: usize, stream: &mut impl Read) -> Result<(Vec<u8>, Vec<usize>)> {
    // First we read off the total number of bytes in the bytes block
    let byte_count = stream.read_i32::<LittleEndian>()? as usize;

    let mut bytes = vec![0u8; byte_count];
    stream.read_exact(&mut bytes)?;

    let mut offsets = Vec::with_capacity(num_bytes);
    for _ in 0..num_bytes {
        offsets.push(read_uindex(stream)? as usize);
    }

    Ok((bytes, offsets))
}

fn read_type(stream: &mut impl Read, num_types: u32, num_strings: u32) -> Result<Type> {
    let kind = stream.read_u8()? as i32;
    let kind = TypeKind::from_raw(kind).ok_or(CodeReadError::InvalidTypeKindDoesNotExist)?;
    let t = match kind {
        TypeKind::Method | TypeKind::Function => {
            // Read the function argument list
            let num_args = stream.read_u8()?;
            let mut args = Vec::with_capacity(num_args as usize);
            for _ in 0..num_args {
                let arg = get_type(stream, num_types)?;
                args.push(arg);
            }

            // Read the return type
            let returns = get_type(stream, num_types)?;

            // Package with the original type kind
            let variant = TypeVariant::Function(TypeFunction { args, returns });
            Type::new(kind, variant).ok_or(CodeReadError::InvalidType)?
        }
        TypeKind::Obj | TypeKind::Struct => {
            let name = get_string(stream, num_strings)?;
            let super_ = read_index(stream)?;
            let global = read_uindex(stream)?;
            let num_fields = read_uindex(stream)?;
            let num_protos = read_uindex(stream)?;
            let num_bindings = read_uindex(stream)?;

            let mut fields = Vec::with_capacity(num_fields as usize);
            for _ in 0..num_fields {
                let name = get_string(stream, num_strings)?;
                let type_ = get_type(stream, num_types)?;
                fields.push(Field { name, type_ });
            }

            let mut protos = Vec::with_capacity(num_protos as usize);
            for _ in 0..num_protos {
                let name = get_string(stream, num_strings)?;
                let f_index = read_uindex(stream)?;
                let p_index = read_index(stream)?;
                protos.push(ObjectProto {
                    name,
                    f_index,
                    p_index,
                });
            }

            let mut bindings = vec![0u32; num_bindings as usize * 2];
            for i in 0..num_bindings {
                let a = read_uindex(stream)?;
                let b = read_uindex(stream)?;
                let i1 = i * 2;
                let i2 = (i * 2) + 1;
                bindings[i1 as usize] = a;
                bindings[i2 as usize] = b;
            }

            let super_ = if super_ < 0 {
                None
            } else {
                Some(super_ as u32)
            };

            let variant = TypeVariant::Object(TypeObject {
                name,
                fields,
                protos,
                bindings,
                super_,
                global,
            });
            Type::new(kind, variant).ok_or(CodeReadError::InvalidType)?
        }
        TypeKind::Null | TypeKind::Ref => {
            let variant = TypeVariant::TypeParam(TypeParam {
                type_: get_type(stream, num_types)?,
            });
            Type::new(kind, variant).ok_or(CodeReadError::InvalidType)?
        }
        TypeKind::Abstract => {
            let variant = TypeVariant::Abstract(TypeAbstract {
                name: get_string(stream, num_strings)?,
            });
            Type::new(kind, variant).ok_or(CodeReadError::InvalidType)?
        }
        TypeKind::Virtual => {
            let num_fields = read_uindex(stream)?;
            let mut fields = Vec::with_capacity(num_fields as usize);
            for _ in 0..num_fields {
                let name = get_string(stream, num_strings)?;
                let type_ = get_type(stream, num_types)?;
                fields.push(Field { name, type_ });
            }
            let variant = TypeVariant::Virtual(TypeVirtual { fields });
            Type::new(kind, variant).ok_or(CodeReadError::InvalidType)?
        }
        TypeKind::Enum => {
            let name = get_string(stream, num_strings)?;
            let global = read_uindex(stream)?;
            let num_constructs = read_uindex(stream)?;
            let mut constructs = Vec::with_capacity(num_constructs as usize);
            for _ in 0..num_constructs {
                let name = get_string(stream, num_strings)?;
                let num_params = read_uindex(stream)?;
                let mut params = Vec::with_capacity(num_params as usize);
                for _ in 0..num_params {
                    params.push(get_type(stream, num_types)?);
                }
                constructs.push(EnumConstruct { name, params });
            }
            let variant = TypeVariant::Enum(TypeEnum {
                name,
                constructs,
                global,
            });
            Type::new(kind, variant).ok_or(CodeReadError::InvalidType)?
        }
        _ => Type::new(kind, TypeVariant::Other).ok_or(CodeReadError::InvalidType)?,
    };
    Ok(t)
}

fn read_function(
    stream: &mut impl Read,
    num_types: u32,
    num_debug_files: u32,
    has_debug: bool,
) -> Result<Function> {
    let type_ = get_type(stream, num_types)?;
    let f_index = read_uindex(stream)?;
    let num_regs = read_uindex(stream)?;
    let num_ops = read_uindex(stream)?;

    let mut registers = Vec::with_capacity(num_regs as usize);
    for _ in 0..num_regs {
        registers.push(get_type(stream, num_types)?);
    }

    let mut ops = Vec::with_capacity(num_ops as usize);
    for _ in 0..num_ops {
        let op = stream.read_u8()?;
        let op = OpCodeNumber::from_raw(op).ok_or(CodeReadError::InvalidOpCodeUnknown)?;

        match op.opcode_type() {
            OpCodeType::OpNoParam => {
                let op = OpCode::from_no_param(op).unwrap();
                ops.push(op);
            }
            OpCodeType::OpOneParam => {
                let params = OpOneParam {
                    param_1: read_index(stream)?,
                };
                let op = OpCode::from_one_param(op, params).unwrap();
                ops.push(op);
            }
            OpCodeType::OpTwoParam => {
                let params = OpTwoParam {
                    param_1: read_index(stream)?,
                    param_2: read_index(stream)?,
                };
                let op = OpCode::from_two_param(op, params).unwrap();
                ops.push(op);
            }
            OpCodeType::OpThreeParam => {
                let params = OpThreeParam {
                    param_1: read_index(stream)?,
                    param_2: read_index(stream)?,
                    param_3: read_index(stream)?,
                };
                let op = OpCode::from_three_param(op, params).unwrap();
                ops.push(op);
            }
            OpCodeType::OpFourParam => {
                let params = OpFourParam {
                    param_1: read_index(stream)?,
                    param_2: read_index(stream)?,
                    param_3: read_index(stream)?,
                    param_4: read_index(stream)?,
                };
                let op = OpCode::from_four_param(op, params).unwrap();
                ops.push(op);
            }
            OpCodeType::OpFiveParam => {
                let params = OpFiveParam {
                    param_1: read_index(stream)?,
                    param_2: read_index(stream)?,
                    param_3: read_index(stream)?,
                    param_4: read_index(stream)?,
                    param_5: read_index(stream)?,
                };
                let op = OpCode::from_five_param(op, params).unwrap();
                ops.push(op);
            }
            OpCodeType::OpSixParam => {
                let params = OpSixParam {
                    param_1: read_index(stream)?,
                    param_2: read_index(stream)?,
                    param_3: read_index(stream)?,
                    param_4: read_index(stream)?,
                    param_5: read_index(stream)?,
                    param_6: read_index(stream)?,
                };
                let op = OpCode::from_six_param(op, params).unwrap();
                ops.push(op);
            }
            OpCodeType::OpCallNParam => {
                let param_1 = read_index(stream)?;
                let param_2 = read_index(stream)?;
                let param_3 = stream.read_u8()?;

                let mut extra = Vec::with_capacity(param_3 as usize);
                for _ in 0..param_3 {
                    extra.push(read_index(stream)?);
                }

                let params = OpCallNParam {
                    param_1,
                    param_2,
                    param_3,
                    extra,
                };
                let op = OpCode::from_call_n_param(op, params).unwrap();
                ops.push(op);
            }
            OpCodeType::OpSwitchParam => {
                let param_1 = read_uindex(stream)?;
                let param_2 = read_uindex(stream)?;
                let mut extra = Vec::with_capacity(param_2 as usize);
                for _ in 0..param_2 {
                    extra.push(read_uindex(stream)?);
                }
                let param_3 = read_uindex(stream)?;
                let params = OpSwitchParam {
                    param_1,
                    param_2,
                    extra,
                    param_3,
                };
                let op = OpCode::from_switch_param(op, params).unwrap();
                ops.push(op);
            }
        }
    }

    let mut debug_infos = Vec::with_capacity(num_ops as usize * 2);
    if has_debug {
        let mut current_file: i32 = -1;
        let mut current_line: i32 = 0;
        while debug_infos.len() < num_ops as usize {
            let c = stream.read_u8()? as i32;
            if (c & 1) != 0 {
                // Read in the current file

                // Unpack from file
                let byte = stream.read_u8()? as i32;
                current_file = c >> 1;
                current_file = (current_file << 8) | byte;

                // Validate refers to a file in the table
                if current_file >= num_debug_files as i32 {
                    return Err(CodeReadError::InvalidDebugInfoBadFileIndex);
                }
            } else if (c & 2) != 0 {
                // Read in a packed form of `n` copies of the current `current_line`

                // Unpack from the file
                let delta = c >> 6;
                let count = (c >> 2) & 15;

                // Validate that it does not produce more entries than opcodes in the function
                if (debug_infos.len() + count as usize) > num_ops as usize {
                    return Err(CodeReadError::InvalidDebugInfoOutsideRange);
                }

                // Push `n` (current_file, current_line) into the list
                for _ in 0..count {
                    debug_infos.push((current_file, current_line));
                }

                // Update current line
                current_line += delta;
            } else if (c & 4) != 0 {
                // Read in a current line shift value and push a new debug info set
                current_line += c >> 3;
                debug_infos.push((current_file, current_line));
            } else {
                // Read a different form of packed number for updating the current line and push
                let b2 = stream.read_u8()? as i32;
                let b3 = stream.read_u8()? as i32;
                current_line = (c >> 3) | (b2 << 5) | (b3 << 13);
                debug_infos.push((current_file, current_line));
            }
        }
        let num_assigns = read_uindex(stream)?;
        for _ in 0..num_assigns {
            read_uindex(stream)?;
            read_index(stream)?;
        }
    }

    Ok(Function {
        type_,
        f_index,
        registers,
        ops,
        debug_infos,
    })
}

fn get_type(stream: &mut impl Read, num_types: u32) -> Result<u32> {
    let t = read_index(stream)?;
    if t < 0 || t as u32 >= num_types {
        return Err(CodeReadError::InvalidTypeBadIndex);
    } else {
        Ok(t as u32)
    }
}

fn get_string(stream: &mut impl Read, num_strings: u32) -> Result<u32> {
    let string = read_index(stream)?;
    if string < 0 || string as u32 >= num_strings {
        return Err(CodeReadError::InvalidStringBadIndex);
    } else {
        Ok(string as u32)
    }
}
