//
//
// This file is a part of Aleph
//
// <ALEPH_REPO_REPLACE>
//
// <ALEPH_LICENSE_REPLACE>
//

use crate::format::{
    is_format_prohibited, is_format_unsupported, ALLOWED_FORMATS, SUPPORTED_FORMATS,
};
use crate::format_sample_info_count;
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
fn all_formats_handled_supported() {
    ALL_FORMATS.iter().for_each(|format| {
        let supported = !is_format_prohibited(*format) && !is_format_unsupported(*format);
        let in_supported_list = SUPPORTED_FORMATS.contains(format);
        match (supported, in_supported_list) {
            (true, false) => panic!("Supported format not in supported list: {:#?}", format),
            (false, true) => panic!("Unsupported format in supported list: {:#?}", format),
            _ => (),
        };
    });
}

#[test]
fn all_formats_sample_count() {
    ALL_FORMATS.iter().for_each(|format| {
        let allowed = !is_format_prohibited(*format) && !is_format_unsupported(*format);
        let sample_count_option = format_sample_info_count(*format);
        match (allowed, sample_count_option) {
            (true, None) => panic!(
                "Allowed format doesn't describe sample count: {:#?}",
                format
            ),
            (false, Some(_)) => panic!("Prohibited format describes sample count: {:#?}", format),
            _ => (),
        };
    });
}
