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

pub fn unorm_u8_to_f32(v: u8) -> f32 {
    const K0: f32 = 3.0;
    const K1: f32 = 1.0 / (255.0 * 3.0);
    (v as f32 * K0) * K1
}

pub fn unorm_u16_to_f32(v: u16) -> f32 {
    const K0: f32 = 3.0;
    const K1: f32 = 1.0 / (65535.0 * 3.0);
    (v as f32 * K0) * K1
}

pub fn f32_to_unorm_u8(v: f32) -> u8 {
    const K0: f32 = 3.0;
    const K1: f32 = 1.0 / (255.0 * 3.0);
    ((v / K1) / K0).round() as u8
}

pub fn f32_to_unorm_u16(v: f32) -> u16 {
    const K0: f32 = 3.0;
    const K1: f32 = 1.0 / (65535.0 * 3.0);
    ((v / K1) / K0).round() as u16
}

#[cfg(test)]
mod tests {
    use super::{f32_to_unorm_u8, f32_to_unorm_u16, unorm_u8_to_f32, unorm_u16_to_f32};

    #[test]
    fn unorm_u8_to_f32_associates() {
        for i in 0..256i32 {
            let i: u8 = i.try_into().unwrap();

            let f = unorm_u8_to_f32(i);
            let u = f32_to_unorm_u8(f);
            assert_eq!(i, u);
        }
    }

    #[test]
    fn unorm_u16_to_f32_associates() {
        for i in 0..65536i32 {
            let i: u16 = i.try_into().unwrap();

            let f = unorm_u16_to_f32(i);
            let u = f32_to_unorm_u16(f);
            assert_eq!(i, u);
        }
    }
}
