//
//
// This file is a part of Aleph
//
// <ALEPH_REPO_REPLACE>
//
// <ALEPH_LICENSE_REPLACE>
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
