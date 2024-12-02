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

use crate::HiBitSet;

#[test]
pub fn test_create_empty() {
    let set = HiBitSet::new();

    assert_eq!(set.bytes_allocated(), 0);
    assert_eq!(set.level_0.len(), 0);
    assert_eq!(set.level_1.len(), 0);
    assert_eq!(set.level_2.len(), 0);
    assert_eq!(set.len, 0);
}

#[test]
pub fn test_create_resize() {
    let mut set = HiBitSet::new();
    set.resize(1);

    // Can't test this as behaviour is allocator dependent
    // assert_eq!(set.bytes_allocated(), 8 * 3);

    assert_eq!(set.level_0.len(), 1);
    assert_eq!(set.level_1.len(), 1);
    assert_eq!(set.level_2.len(), 1);
    assert_eq!(set.len, 1);
}

#[test]
pub fn test_create_resize_64() {
    let mut set = HiBitSet::new();
    set.resize(64);

    assert_eq!(set.level_0.len(), 1);
    assert_eq!(set.level_1.len(), 1);
    assert_eq!(set.level_2.len(), 1);
    assert_eq!(set.len, 64);
}

#[test]
pub fn test_create_resize_65() {
    let mut set = HiBitSet::new();
    set.resize(65);

    assert_eq!(set.level_0.len(), 2);
    assert_eq!(set.level_1.len(), 1);
    assert_eq!(set.level_2.len(), 1);
    assert_eq!(set.len, 65);
}

#[test]
pub fn test_create_resize_max() {
    let mut set = HiBitSet::new();
    set.resize(HiBitSet::MAX_BITS);

    assert_eq!(set.level_0.len(), HiBitSet::MAX_BITS / 64);
    assert_eq!(set.level_1.len(), HiBitSet::MAX_BITS / 64 / 64);
    assert_eq!(set.level_2.len(), HiBitSet::MAX_BITS / 64 / 64 / 64);
    assert_eq!(set.len, HiBitSet::MAX_BITS);
}

#[test]
#[should_panic]
pub fn test_create_resize_shrink() {
    let mut set = HiBitSet::new();
    set.resize(2);
    set.resize(1);
}

#[test]
pub fn test_create_set_check() {
    let mut set = HiBitSet::new();
    set.resize(64 * 2);

    // Check the hierarchy is set correctly
    assert_eq!(set.level_1[0], 0);
    assert_eq!(set.level_2[0], 0);
    assert_eq!(set.level_3, 0);

    //
    // NEXT
    set.set_bit(0);
    assert!(set.has_bit(0));

    // Check the hierarchy is set correctly
    assert_eq!(set.level_1[0], 1);
    assert_eq!(set.level_2[0], 1);
    assert_eq!(set.level_3, 1);

    //
    // NEXT
    set.set_bit(1);
    assert!(set.has_bit(1));

    // Check the hierarchy is set correctly
    assert_eq!(set.level_1[0], 1);
    assert_eq!(set.level_2[0], 1);
    assert_eq!(set.level_3, 1);

    //
    // NEXT
    set.unset_bit(0);
    assert!(!set.has_bit(0));

    // Check the hierarchy is set correctly
    assert_eq!(set.level_1[0], 1);
    assert_eq!(set.level_2[0], 1);
    assert_eq!(set.level_3, 1);

    //
    // NEXT
    set.unset_bit(1);
    assert!(!set.has_bit(1));

    // Check the hierarchy is set correctly
    assert_eq!(set.level_1[0], 0);
    assert_eq!(set.level_2[0], 0);
    assert_eq!(set.level_3, 0);

    //
    // NEXT
    for i in 0..64 {
        set.set_bit(i);
    }

    for i in 0..64 {
        assert!(set.has_bit(i))
    }
    // Check the hierarchy is set correctly
    assert_eq!(set.level_1[0], 1);
    assert_eq!(set.level_2[0], 1);
    assert_eq!(set.level_3, 1);

    //
    // NEXT
    set.set_bit(64);
    assert!(set.has_bit(64));

    // Check the hierarchy is set correctly
    assert_eq!(set.level_1[0], 3);
    assert_eq!(set.level_2[0], 1);
    assert_eq!(set.level_3, 1);

    //
    // NEXT
    set.unset_bit(64);
    assert!(!set.has_bit(64));

    // Check the hierarchy is set correctly
    assert_eq!(set.level_1[0], 1);
    assert_eq!(set.level_2[0], 1);
    assert_eq!(set.level_3, 1);
}

#[test]
pub fn test_create_set_check_max() {
    let mut set = HiBitSet::new();
    set.resize(HiBitSet::MAX_BITS);

    // Check the hierarchy is set correctly
    assert_eq!(set.level_1[0], 0);
    assert_eq!(set.level_2[0], 0);
    assert_eq!(set.level_3, 0);

    //
    // NEXT
    set.set_bit(0);
    assert!(set.has_bit(0));

    // Check the hierarchy is set correctly
    assert_eq!(set.level_1[0], 1);
    assert_eq!(set.level_2[0], 1);
    assert_eq!(set.level_3, 1);

    //
    // NEXT
    set.set_bit(HiBitSet::MAX_BITS - 1);
    assert!(set.has_bit(HiBitSet::MAX_BITS - 1));

    // Check the hierarchy is set correctly
    assert_eq!(set.level_1[0], 1);
    assert_eq!(set.level_2[0], 1);
    assert_eq!(set.level_3, 1 | (1 << 63));
}

#[test]
pub fn test_create_iter_1() {
    let mut set = HiBitSet::new();
    set.resize(HiBitSet::MAX_BITS);

    set.set_bit(0);
    assert!(set.has_bit(0));

    set.set_bit(HiBitSet::MAX_BITS - 1);
    assert!(set.has_bit(HiBitSet::MAX_BITS - 1));

    let bits: Vec<usize> = set.iter().collect();
    assert_eq!(bits.len(), 2);
    assert_eq!(bits[0], 0);
    assert_eq!(bits[1], HiBitSet::MAX_BITS - 1);
}

#[test]
pub fn test_create_iter_2() {
    let mut set = HiBitSet::new();
    set.resize(64);

    set.set_bit(0);
    assert!(set.has_bit(0));

    set.set_bit(63);
    assert!(set.has_bit(63));

    let bits: Vec<usize> = set.iter().collect();
    assert_eq!(bits.len(), 2);
    assert_eq!(bits[0], 0);
    assert_eq!(bits[1], 63);
}

#[test]
pub fn test_create_iter_3() {
    let mut set = HiBitSet::new();
    set.resize(64);

    let bits: Vec<usize> = set.iter().collect();
    assert_eq!(bits.len(), 0);
}

#[test]
pub fn test_create_iter_4() {
    let mut set = HiBitSet::new();
    set.resize(HiBitSet::MAX_BITS);

    let expected = [0, 1, 64, 96, 1052, HiBitSet::MAX_BITS - 1].to_vec();

    for b in expected.iter() {
        set.set_bit(*b);
    }

    for b in expected.iter() {
        assert!(set.has_bit(*b));
    }

    let actual: Vec<usize> = set.iter().collect();

    assert_eq!(expected.len(), actual.len());
    for (expect, actual) in expected.iter().zip(actual.iter()) {
        assert_eq!(*expect, *actual);
    }
}

#[test]
pub fn test_create_iter_5() {
    let mut set = HiBitSet::new();
    set.resize(HiBitSet::MAX_BITS);

    let expected = [
        0,
        1,
        64,
        96,
        1052,
        HiBitSet::MAX_BITS / 2,
        HiBitSet::MAX_BITS / 2 + 1,
        HiBitSet::MAX_BITS - 3,
        HiBitSet::MAX_BITS - 2,
        HiBitSet::MAX_BITS - 1,
    ]
    .to_vec();

    for b in expected.iter() {
        set.set_bit(*b);
    }

    for b in expected.iter() {
        assert!(set.has_bit(*b));
    }

    let actual: Vec<usize> = set.iter().collect();

    assert_eq!(expected.len(), actual.len());
    for (expect, actual) in expected.iter().zip(actual.iter()) {
        assert_eq!(*expect, *actual);
    }
}

#[test]
pub fn test_create_iter_6() {
    let mut set = HiBitSet::new();
    set.resize(64);

    let expected: Vec<usize> = (0..63).collect();

    for b in expected.iter() {
        set.set_bit(*b);
    }

    for b in expected.iter() {
        assert!(set.has_bit(*b));
    }

    let actual: Vec<usize> = set.iter().collect();

    assert_eq!(expected.len(), actual.len());
    for (expect, actual) in expected.iter().zip(actual.iter()) {
        assert_eq!(*expect, *actual);
    }
}

#[test]
pub fn test_create_iter_7() {
    let mut set = HiBitSet::new();
    set.resize(HiBitSet::MAX_BITS);

    let expected: Vec<usize> = (1..HiBitSet::MAX_BITS).collect();

    for b in expected.iter() {
        set.set_bit(*b);
    }

    for b in expected.iter() {
        assert!(set.has_bit(*b));
    }

    let actual: Vec<usize> = set.iter().collect();

    assert_eq!(expected.len(), actual.len());
    for (expect, actual) in expected.iter().zip(actual.iter()) {
        assert_eq!(*expect, *actual);
    }
}
