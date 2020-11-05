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

use std::io::Error;

/// Represents the set of errors that can be encountered when reading a hashlink module
#[derive(Debug)]
pub enum CodeReadError {
    IOError(std::io::Error),
    InvalidFileHeader,
    InvalidIndexUnsignedLessThanOne,
    InvalidStringNotValidUTF8,
    InvalidTypeKindDoesNotExist,
    InvalidTypeKindInvalidType,
    InvalidTypeBadIndex,
    InvalidStringBadIndex,
    InvalidOpCodeUnknown,
    InvalidVersionUnknown,
    InvalidVersionUnsupported,
    InvalidDebugInfoBadFileIndex,
    InvalidDebugInfoOutsideRange,
}

impl From<std::io::Error> for CodeReadError {
    fn from(err: Error) -> Self {
        CodeReadError::IOError(err)
    }
}

/// Result for module reading
pub type Result<T> = std::result::Result<T, CodeReadError>;
