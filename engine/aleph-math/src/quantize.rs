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

use num_traits::PrimInt;
use num_traits::Signed;
use num_traits::Unsigned;

use crate::traits::Real;

///
/// Converts a signed integer to a normalized floating point value in the range -1.0 - 1.0
///
pub fn snorm_to_float<In: Signed + PrimInt, Out: Real>(input: In) -> Out {
    let max_val: Out = Out::from(In::max_value()).unwrap();
    let factor: Out = Out::one() / max_val;
    Out::from(input).unwrap() * factor
}

///
/// Converts an unsigned integer to a normalized floating point value in the range 0 - 1.0
///
pub fn unorm_to_float<In: Unsigned + PrimInt, Out: Real>(input: In) -> Out {
    let max_val: Out = Out::from(In::max_value()).unwrap();
    Out::from(input).unwrap() / max_val
}

///
/// Converts a float in the range -1.0 - 1.0 to a signed integer
///
pub fn float_to_snorm<In: Real, Out: Signed + PrimInt>(input: In) -> Out {
    let clamped: In = if input < -In::one() {
        -In::one()
    } else if input > In::one() {
        In::one()
    } else {
        input
    };

    let mapped: In = clamped * In::from(Out::max_value() - Out::one()).unwrap();

    let half: In = In::one() / (In::one() + In::one());
    let output: In = if mapped >= In::zero() {
        mapped + half
    } else {
        mapped - half
    };

    Out::from(output.trunc()).unwrap()
}

///
/// Converts a float in the range 0 - 1.0 to an unsigned integer
///
pub fn float_to_unorm<In: Real, Out: Signed + PrimInt>(input: In) -> Out {
    let clamped: In = if input < In::zero() {
        In::zero()
    } else if input > In::one() {
        In::one()
    } else {
        input
    };

    let half: In = In::one() / (In::one() + In::one());

    let mapped: In = clamped * In::from(Out::max_value()).unwrap();
    let output: In = mapped + half;

    Out::from(output.trunc()).unwrap()
}
