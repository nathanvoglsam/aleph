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

use once_cell::sync::Lazy;

///
/// Only need to look this up once so wrap it in a lazy static
///
#[allow(clippy::identity_op)]
pub static CPU_VENDOR_STRING: Lazy<String> = Lazy::new(|| {
    let result = raw_cpuid::CpuId::new();
    match result.get_vendor_info() {
        None => String::from("Unknown CPU Vendor"),
        Some(vendor) => vendor.as_str().to_string(),
    }
});

///
/// Only need to look this up once so wrap it in a lazy static
///
#[allow(clippy::identity_op)]
pub static CPU_BRAND_STRING: Lazy<String> = Lazy::new(|| {
    let result = raw_cpuid::CpuId::new();
    match result.get_processor_brand_string() {
        None => "Unknown CPU".to_string(),
        Some(ext) => ext.as_str().to_string(),
    }
});
