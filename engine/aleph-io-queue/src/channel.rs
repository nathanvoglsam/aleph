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

use std::io;

/// Message format that the IO queue will respond with.
pub enum IoMessage {
    ReadSuccess {
        /// Offset in the file that the data was read from.
        offset: u64,

        /// The total number of bytes that were successfully transferred. This may not equal the
        /// number of bytes _requested_.
        bytes_transferred: usize,

        /// An opaque tag that can be used to associate the message with a particular request.
        opaque: u64,
    },
    ReadFail {
        /// Offset in the file that the data was read from.
        offset: u64,

        /// The specific IO error that was thrown that caused the request to fail.
        err: io::Error,

        /// An opaque tag that can be used to associate the message with a particular request.
        opaque: u64,
    },
    LoadSuccess {
        /// Buffer that contains the complete contents of the file.
        data: Vec<u8>,

        /// An opaque tag that can be used to associate the message with a particular request.
        opaque: u64,
    },
    LoadFail {
        /// The specific IO error that was thrown that caused the request to fail.
        err: io::Error,

        /// An opaque tag that can be used to associate the message with a particular request.
        opaque: u64,
    },
    OpenSuccess {
        /// An opaque tag that can be used to associate the message with a particular request.
        opaque: u64,
    },
    OpenFail {
        /// The specific IO error that was thrown that caused the request to fail.
        err: io::Error,

        /// An opaque tag that can be used to associate the message with a particular request.
        opaque: u64,
    },
}

impl IoMessage {
    /// Get the opaque tag from whichever message variant `self` contains.
    pub const fn opaque(&self) -> u64 {
        match self {
            IoMessage::ReadSuccess { opaque, .. } => *opaque,
            IoMessage::ReadFail { opaque, .. } => *opaque,
            IoMessage::LoadSuccess { opaque, .. } => *opaque,
            IoMessage::LoadFail { opaque, .. } => *opaque,
            IoMessage::OpenSuccess { opaque, .. } => *opaque,
            IoMessage::OpenFail { opaque, .. } => *opaque,
        }
    }
}
