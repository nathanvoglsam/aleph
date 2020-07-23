//
//
// This file is a part of Aleph
//
// <ALEPH_REPO_REPLACE>
//
// <ALEPH_LICENSE_REPLACE>
//

use crate::format::{is_format_prohibited, is_format_unsupported, ALLOWED_FORMATS};
use crate::{format_sample_info_count, SampleInfo};
use aleph_vk_format::{VkFormat, ALL_FORMATS};

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
