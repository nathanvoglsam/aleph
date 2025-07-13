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

use crate::LONGEST_DFD;
use crate::table::VK_TO_DFD;

#[test]
fn sizes_are_sensible() {
    for (_, value) in VK_TO_DFD.entries() {
        assert_eq!(value[0], value.len() as u32 * 4);
        assert!(value.len() >= 7);
    }
}

#[test]
fn longest_dfd_is_correct() {
    let mut real_longest = 0;
    for (_, value) in VK_TO_DFD.entries() {
        assert!(value.len() <= LONGEST_DFD);

        // Swap out 'real_longest' if we find a bigger dfd
        real_longest = value.len().max(real_longest);
    }

    // LONGEST_DFD should be exactly equal to the length of the longest DFD with no extra space
    assert_eq!(real_longest, LONGEST_DFD);
}

#[test]
fn lookups_work() {
    for (format, value) in VK_TO_DFD.entries() {
        assert_eq!(*value, crate::vk2dfd(*format).unwrap());
    }
}
