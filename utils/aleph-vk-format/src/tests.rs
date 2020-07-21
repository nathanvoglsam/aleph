//
//
// This file is a part of Aleph
//
// <ALEPH_REPO_REPLACE>
//
// <ALEPH_LICENSE_REPLACE>
//

pub use super::VkFormat;
use crate::ALL_FORMATS;

#[test]
fn all_formats_enumerated() {
    ALL_FORMATS.iter().for_each(|format| {
        assert!(format.is_known(), "All formats in the list must be known");
    });
}

#[test]
fn all_formats_have_debug_name() {
    ALL_FORMATS.iter().for_each(|format| {
        let format_str = format!("{:#?}", format);
        assert_ne!(
            format_str, "(unknown)",
            "All formats must have a debug name"
        );
    });
}

//#[test]
//fn quick() {
//    ALL_FORMATS.iter().for_each(|format| {
//        let a = ColorModel::for_format(*format);
//        let b = ColorModel::for_format_small(*format);
//        assert_eq!(a, b, "A:B -> {:#?}", format);
//    })
//}
