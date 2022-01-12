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

use crate::VirtualBuffer;

#[test]
fn test_individual_commits() {
    let mut buffer = VirtualBuffer::reserve(2).unwrap();

    // Commit the first page
    buffer.commit(0..1).unwrap();

    // Commit the second page
    buffer.commit(5000..5001).unwrap();

    // This is safe as both pages should be committed now
    let slice = unsafe { buffer.as_slice_mut() };

    slice[5000] = 21;
    slice[1] = 56;

    assert_eq!(slice[5000], 21);
    assert_eq!(slice[1], 56);

    // Release the first page
    buffer.commit(0..1).unwrap();

    // Release the second page
    buffer.commit(5000..5001).unwrap();
}

#[test]
fn test_stradling_commit() {
    let mut buffer = VirtualBuffer::reserve(2).unwrap();

    // Commit both pages
    buffer.commit(4095..4097).unwrap();

    // This is safe as both pages should be committed now
    let slice = unsafe { buffer.as_slice_mut() };

    slice[5000] = 21;
    slice[1] = 56;

    assert_eq!(slice[5000], 21);
    assert_eq!(slice[1], 56);

    // Release both pages
    buffer.release(4095..4097).unwrap();
}
