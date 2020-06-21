//
//
// This file is a part of Aleph
//
// <ALEPH_REPO_REPLACE>
//
// <ALEPH_LICENSE_REPLACE>
//

use std::ffi::CStr;
use std::mem;

#[inline]
#[allow(dead_code)]
pub unsafe fn str_to_cstr(string: &'static str) -> &'static CStr {
    mem::transmute(string)
}

#[macro_export]
macro_rules! cstr {
    ($strval:expr) => {
        unsafe { $crate::macros::str_to_cstr(concat!($strval, "\0")) }
    };
}

#[repr(C)] // guarantee 'bytes' comes after '_align'
pub struct AlignedAs<Align, Bytes: ?Sized> {
    pub _align: [Align; 0],
    pub bytes: Bytes,
}

#[macro_export]
macro_rules! include_bytes_aligned_as {
    ($align_ty:ty, $path:literal) => {{
        // const block expression to encapsulate the static
        use $crate::macros::AlignedAs;

        // this assignment is made possible by CoerceUnsized
        static ALIGNED: &AlignedAs<$align_ty, [u8]> = &AlignedAs {
            _align: [],
            bytes: *include_bytes!($path),
        };

        &ALIGNED.bytes
    }};
}
