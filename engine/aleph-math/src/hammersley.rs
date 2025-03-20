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

use ultraviolet::Vec2;

pub const fn radical_inverse_vdc(bits: u32) -> f32 {
    let bits = (bits << 16) | (bits >> 16);
    let bits = ((bits & 0x55555555) << 1) | ((bits & 0xAAAAAAAA) >> 1);
    let bits = ((bits & 0x33333333) << 2) | ((bits & 0xCCCCCCCC) >> 2);
    let bits = ((bits & 0x0F0F0F0F) << 4) | ((bits & 0xF0F0F0F0) >> 4);
    let bits = ((bits & 0x00FF00FF) << 8) | ((bits & 0xFF00FF00) >> 8);
    return bits as f32 * 2.3283064365386963e-10; // / 0x100000000
}

pub const fn hammersley(i: u32, n: u32) -> Vec2 {
    let x = i as f32 / n as f32;
    let y = radical_inverse_vdc(i);
    return Vec2::new(x, y);
}
