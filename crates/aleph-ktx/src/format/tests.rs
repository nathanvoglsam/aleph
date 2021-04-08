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

use crate::format::{is_format_prohibited, is_format_unsupported, ALLOWED_FORMATS};
use crate::{format_sample_info_count, SampleInfo};
use aleph_vk_format::ALL_FORMATS;

#[test]
fn all_formats_handled_prohibited() {
    ALL_FORMATS.iter().for_each(|format| {
        let allowed = !is_format_prohibited(*format);
        let in_allowed_list = ALLOWED_FORMATS.contains(format);
        match (allowed, in_allowed_list) {
            (true, false) => panic!("Allowed format not in allowed list: {:#?}", format),
            (false, true) => panic!("Prohibited format in allowed list: {:#?}", format),
            _ => (),
        };
    });
}

#[test]
fn all_formats_sample_count() {
    ALLOWED_FORMATS.iter().for_each(|format| {
        if !is_format_unsupported(*format) {
            let format_covered = format_sample_info_count(*format).is_some();
            assert!(format_covered, "{:#?}", format);
        }
    });
}

#[test]
fn allowed_formats_sample_info() {
    let mut sample_infos: [SampleInfo; 16] = Default::default();
    ALLOWED_FORMATS.iter().for_each(|format| {
        if !is_format_unsupported(*format) {
            let format_covered = SampleInfo::for_format(*format, &mut sample_infos).is_some();
            assert!(format_covered, "{:#?}", format);
        }
    });
}
