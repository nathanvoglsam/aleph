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

#[doc(hidden)]
pub extern crate aleph_nstr as nstr;

#[doc(hidden)]
pub extern crate uuid;

#[doc(hidden)]
pub extern crate ctor;

#[doc(hidden)]
pub extern crate crossbeam;

pub mod instrumentation;

pub use allocator_api2::boxed::Box as BBox;
pub use allocator_api2::vec::Vec as BVec;
pub use allocator_api2::*;
pub use blink_alloc::*;
pub use hashbrown::*;

pub type BHashMap<K, V, A = alloc::Global> = HashMap<K, V, DefaultHashBuilder, A>;
pub type BHashSet<T, A = alloc::Global> = HashSet<T, DefaultHashBuilder, A>;
