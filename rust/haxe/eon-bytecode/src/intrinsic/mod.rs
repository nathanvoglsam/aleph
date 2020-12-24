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

use serde::{Deserialize, Serialize};

/// This represents the set of intrinsic functions available to the Eon VM/compiler
#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Debug, Serialize, Deserialize)]
pub enum Intrinsic {
    /// Checks if the given pointer is null, throwing an exception if it does
    NullCheck,

    /// Attempts to cast one value to another type, throwing an exception if it is not possible
    /// to do so
    SafeCast,

    /// Attempts to cast a type from one value to another, triggering UB if it is not possible
    /// TODO: I really don't like the idea of this intrinsic as it has a very, very narrow reason to
    ///       exist. This intrinsic only exists for cases when two values are statically guaranteed
    ///       to have compatible types. This should be replaced with static analysis
    UnsafeCast,

    /// Marks when an exception handler comes into scope (needed for managing sjlj exceptions)
    BeginTrap,

    /// Marks when an exception handler goes out of scope (needed for managing sjlj exceptions)
    EndTrap,

    /// Call used for actually throwing an exception
    Throw,

    /// Call used for rethrowing an exception when inside an exception handler so it holds on to the
    /// stack trace from the previous exception
    Rethrow,

    /// Intrinsic function for assigning a value with the exception that has been thrown inside an
    /// exception handler
    ReceiveException,
}

impl Intrinsic {
    pub fn mnemonic(&self) -> &'static str {
        match self {
            Intrinsic::NullCheck => "@eon.null_check",
            Intrinsic::SafeCast => "@eon.safe_cast",
            Intrinsic::UnsafeCast => "@eon.unsafe_cast",
            Intrinsic::BeginTrap => "@eon.begin_trap",
            Intrinsic::EndTrap => "@eon.end_trap",
            Intrinsic::Throw => "@eon.throw",
            Intrinsic::Rethrow => "@eon.rethrow",
            Intrinsic::ReceiveException => "@eon.receive_exception",
        }
    }
}
