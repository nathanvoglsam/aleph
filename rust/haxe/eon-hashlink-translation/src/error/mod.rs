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

/// Set of all errors that can occur when transpiling from hashlink bytecode
#[derive(Clone, Debug)]
pub enum TranspileError {
    /// This occurs when there is an error when translating the type definitions. Generally this
    /// error will never actually happen as it's not possible to encode an invalid type in the
    /// on-disk hashlink format but one could be made after being loaded from disk.
    InvalidType,

    /// This error occurs when transpiling a function from the hashlink module fails
    InvalidFunction(InvalidFunctionReason),
}

pub type TranspileResult<T> = Result<T, TranspileError>;

/// An enum representing the set of reasons an `InvalidFunction` error can be emitted
#[derive(Clone, Debug)]
pub enum InvalidFunctionReason {
    /// This error occurs when we are extracting basic block information and find a stream of
    /// instructions without a block terminator instruction.
    ///
    /// This is invalid as all basic blocks must end with a terminator instruction, and is also
    /// indicative of an invalid function in HashLink too as every function should at least have one
    /// OpRet (OpRet is a terminator). If there are exactly 0 terminator instructions in a function
    /// then it is not valid in either bytecode's semantics.
    SpanFoundNoBlockTerminator { func: hashlink::Function },

    /// When an error occurs when actually trying to convert a jump offset into an index. May be the
    /// result of integer overflow/underflow
    JumpInvalidOffset {
        i_index: usize,
        func: hashlink::Function,
    },

    /// When a function encodes a jump offset in an opcode that will jump out of bounds of the
    /// opcode array
    JumpOffsetOutOfBounds {
        i_index: usize,
        func: hashlink::Function,
    },

    /// This occurs when a jump instruction uses a negative offset but the target instruction is not
    /// an OpLabel (which is a requirement of HashLink bytecode)
    JumpNegativeOffsetNotTargetingLabel {
        i_index: usize,
        func: hashlink::Function,
    },

    /// This occurs when a function's type index refers to a type that is not a function type
    TypeIndexNotFunction { func: hashlink::Function },

    /// This occurs when a function argument's signature does not match the registers the function
    /// declares. That is, if the signature declares the argument as one type, but the register says
    /// it is another
    FunctionSignatureArgNotMatchRegister {
        a_index: usize,
        func: hashlink::Function,
    },

    /// This occurs when a trap handling basic block has more than one predecessor. Because of how
    /// exceptions work in Eon, this is invalid as jumping into this block from anything other than
    /// an exception being thrown will mean there is no exception value that the block should use.
    TrapHandlerHasMultiplePredecessors { func: hashlink::Function },

    /// This is a similar, but more specific, form of the `TrapHandlerHasMultiplePredecessors`. This
    /// error is thrown when multiple `OpTrap` instructions mark a basic block as their trap
    /// handler.
    ///
    /// While it would be possible for the runtime and codegen to support this, the problem has not
    /// been given much thought as of yet so it will be treated as an error until a more rigorous
    /// investigation is made.
    TrapHandlerHasMultipleTrapPredecessors { func: hashlink::Function },
}
