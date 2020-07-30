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

bitflags::bitflags! {
    ///
    /// The set of DFD flags defined by the DFD spec
    ///
    pub struct DFDFlags: u8 {
        const ALPHA_PREMULTIPLIED = 0b00000001;
    }
}

bitflags::bitflags! {
    ///
    /// The set of sample flags that can be found in a sample info block
    ///
    pub struct SampleFlags: u8 {
        const LINEAR = 0b0001;
        const EXPONENT = 0b0010;
        const SIGNED = 0b0100;
        const FLOAT = 0b1000;
    }
}

impl SampleFlags {
    pub fn compatible_with(&self, other: SampleFlags) -> bool {
        let signed_match =
            self.contains(SampleFlags::SIGNED) == other.contains(SampleFlags::SIGNED);
        let float_match = self.contains(SampleFlags::FLOAT) == other.contains(SampleFlags::FLOAT);
        let exp_match =
            self.contains(SampleFlags::EXPONENT) == other.contains(SampleFlags::EXPONENT);
        signed_match && float_match && exp_match
    }
}

impl Default for SampleFlags {
    fn default() -> Self {
        Self::empty()
    }
}
