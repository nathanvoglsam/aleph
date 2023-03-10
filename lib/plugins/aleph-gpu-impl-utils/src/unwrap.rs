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

#[macro_export]
macro_rules! conversion_function {
    ($from: path, $to: path, $name: ident, $name_d: ident) => {
        #[allow(unused)]
        /// Converts the given dynamic object to a concrete type, panicking if it is not the
        /// expected concrete type.
        pub fn $name(v: &dyn $from) -> &$to {
            use $crate::aleph_interfaces::any::*;
            use $crate::aleph_interfaces::gpu::*;
            v.query_interface::<$to>().expect(concat!(
                "Unknown ",
                stringify!($from),
                " implementation"
            ))
        }

        #[allow(unused)]
        /// Converts the given dynamic object to a concrete type, panicking if it is not the
        /// expected concrete type. Accepts a double-reference, denoted by the `_d` suffix
        pub fn $name_d<'a>(v: &'a &'a dyn $from) -> &'a $to {
            use $crate::aleph_interfaces::any::*;
            use $crate::aleph_interfaces::gpu::*;
            v.query_interface::<$to>().expect(concat!(
                "Unknown ",
                stringify!($from),
                " implementation"
            ))
        }
    };
}
