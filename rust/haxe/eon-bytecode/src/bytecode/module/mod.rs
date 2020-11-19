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

use crate::bytecode::constant::Constant;
use crate::bytecode::function::Function;
use crate::bytecode::native::Native;
use crate::bytecode::type_::{
    Type, TypeAbstract, TypeEnum, TypeFunction, TypeObject, TypeParam, TypeVirtual,
};

/// Set of all errors that can occur when transpiling from hashlink bytecode
#[derive(Clone, PartialEq, PartialOrd, Eq, Ord, Debug, Hash)]
pub enum TranspileError {
    /// This occurs when there is an error when translating the type definitions. Generally this
    /// error will never actually happen as it's not possible to encode an invalid type in the
    /// on-disk hashlink format but one could be made after being loaded from disk.
    InvalidType,
}

pub type TranspileResult<T> = Result<T, TranspileError>;

/// This struct is a direct representation of a hashlink module *as read from disk*. The original C
/// hashlink code deserializes directly into the datastructures used by the JIT and runtime. This
/// implementation is completely distinct from any runtime and serves purely as a utility for
/// reading, operating on and writing hashlink modules so any information that is not read directly
/// from a hashlink file or is only used by the runtime is not stored here.
///
/// This struct can be used as a component for reading hashlink modules to be consumed by a JIT
/// runtime but is not appropriate to be consumed directly by the runtime.
#[derive(Clone, Debug)]
pub struct Module {
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
    pub globals: Vec<usize>,

    /// The file's function table
    pub functions: Vec<Function>,

    /// The file's constants table
    pub constants: Vec<Constant>,

    /// Index into the functions table for specifying which function is the entrypoint
    pub entrypoint: usize,
}

impl Module {
    pub fn from_hashlink(mut code: hashlink_bytecode::Code) -> TranspileResult<Self> {
        let out = Self {
            ints: code.ints,
            floats: code.floats,
            strings: code.strings,
            bytes: code.bytes,
            byte_offsets: code.byte_offsets,
            debug_files: code.debug_files,
            types: translate_types(code.types)?,
            natives: translate_natives(code.natives)?,
            globals: translate_globals(code.globals)?,
            functions: vec![],
            constants: translate_constants(code.constants)?,
            entrypoint: code.entrypoint as usize,
        };
        Ok(out)
    }
}

fn translate_types(mut input: Vec<hashlink_bytecode::Type>) -> TranspileResult<Vec<Type>> {
    let mut out = Vec::with_capacity(input.len());
    for v in input.drain(..) {
        let new = match v.kind {
            hashlink_bytecode::TypeKind::Void => Type::Void,
            hashlink_bytecode::TypeKind::UI8 => Type::UI8,
            hashlink_bytecode::TypeKind::UI16 => Type::UI16,
            hashlink_bytecode::TypeKind::I32 => Type::I32,
            hashlink_bytecode::TypeKind::I64 => Type::I64,
            hashlink_bytecode::TypeKind::F32 => Type::F32,
            hashlink_bytecode::TypeKind::F64 => Type::F64,
            hashlink_bytecode::TypeKind::Bool => Type::Bool,
            hashlink_bytecode::TypeKind::Bytes => Type::Bytes,
            hashlink_bytecode::TypeKind::Dynamic => Type::Dynamic,
            hashlink_bytecode::TypeKind::Function => {
                if let hashlink_bytecode::TypeVariant::Function(f) = v.variant {
                    Type::Function(TypeFunction::from(f))
                } else {
                    return Err(TranspileError::InvalidType);
                }
            }
            hashlink_bytecode::TypeKind::Obj => {
                if let hashlink_bytecode::TypeVariant::Object(f) = v.variant {
                    Type::Obj(TypeObject::from(f))
                } else {
                    return Err(TranspileError::InvalidType);
                }
            }
            hashlink_bytecode::TypeKind::Array => Type::Array,
            hashlink_bytecode::TypeKind::Type => Type::Type,
            hashlink_bytecode::TypeKind::Ref => {
                if let hashlink_bytecode::TypeVariant::TypeParam(f) = v.variant {
                    Type::Ref(TypeParam::from(f))
                } else {
                    return Err(TranspileError::InvalidType);
                }
            }
            hashlink_bytecode::TypeKind::Virtual => {
                if let hashlink_bytecode::TypeVariant::Virtual(f) = v.variant {
                    Type::Virtual(TypeVirtual::from(f))
                } else {
                    return Err(TranspileError::InvalidType);
                }
            }
            hashlink_bytecode::TypeKind::DynObject => Type::DynObject,
            hashlink_bytecode::TypeKind::Abstract => {
                if let hashlink_bytecode::TypeVariant::Abstract(f) = v.variant {
                    Type::Abstract(TypeAbstract::from(f))
                } else {
                    return Err(TranspileError::InvalidType);
                }
            }
            hashlink_bytecode::TypeKind::Enum => {
                if let hashlink_bytecode::TypeVariant::Enum(f) = v.variant {
                    Type::Enum(TypeEnum::from(f))
                } else {
                    return Err(TranspileError::InvalidType);
                }
            }
            hashlink_bytecode::TypeKind::Null => {
                if let hashlink_bytecode::TypeVariant::TypeParam(f) = v.variant {
                    Type::Null(TypeParam::from(f))
                } else {
                    return Err(TranspileError::InvalidType);
                }
            }
            hashlink_bytecode::TypeKind::Method => {
                if let hashlink_bytecode::TypeVariant::Function(f) = v.variant {
                    Type::Method(TypeFunction::from(f))
                } else {
                    return Err(TranspileError::InvalidType);
                }
            }
            hashlink_bytecode::TypeKind::Struct => {
                if let hashlink_bytecode::TypeVariant::Object(f) = v.variant {
                    Type::Struct(TypeObject::from(f))
                } else {
                    return Err(TranspileError::InvalidType);
                }
            }
        };
        out.push(new);
    }
    Ok(out)
}

fn translate_natives(mut input: Vec<hashlink_bytecode::Native>) -> TranspileResult<Vec<Native>> {
    Ok(input.drain(..).map(Native::from).collect())
}

fn translate_globals(mut input: Vec<u32>) -> TranspileResult<Vec<usize>> {
    Ok(input.drain(..).map(|v| v as usize).collect())
}

fn translate_constants(
    mut input: Vec<hashlink_bytecode::Constant>,
) -> TranspileResult<Vec<Constant>> {
    Ok(input.drain(..).map(Constant::from).collect())
}
