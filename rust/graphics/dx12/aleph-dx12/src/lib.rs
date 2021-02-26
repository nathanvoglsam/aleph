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

extern crate aleph_dx12_alloc_raw as alloc_raw;
extern crate aleph_dx12_raw as raw;
extern crate aleph_dxc_raw as dxc_raw;

#[cfg(feature = "pix")]
pub mod pix;

#[cfg(feature = "dxc")]
pub mod dxc;

#[cfg(feature = "alloc")]
pub mod alloc;

pub mod dxgi;

mod dx12;

pub use dx12::*;

mod utils;

#[cfg(test)]
mod tests;

pub use raw::windows::initialize_mta;
pub use raw::windows::initialize_sta;
pub use raw::windows::ErrorCode;
pub use raw::windows::Result;
pub use utils::name_thread_as_main_thread;
pub use utils::Bool;
