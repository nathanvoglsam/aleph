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
        unsafe { $crate::str_to_cstr(concat!($strval, "\0")) }
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
        // this assignment is made possible by CoerceUnsized
        static ALIGNED: &$crate::AlignedAs<$align_ty, [u8]> = &$crate::AlignedAs {
            _align: [],
            bytes: *include_bytes!($path),
        };

        &ALIGNED.bytes
    }};
}

///
/// Internal function used by `include_spirv_bytes` macro
///
#[inline]
#[allow(dead_code)]
pub fn spirv_bytes_map(bytes: &'static [u8]) -> (&'static [u8], &'static [u32]) {
    unsafe {
        let words = core::slice::from_raw_parts(bytes.as_ptr() as *const u32, bytes.len() / 4);
        (bytes, words)
    }
}

#[macro_export]
macro_rules! include_spirv_bytes {
    ($path:literal) => {{
        $crate::spirv_bytes_map($crate::include_bytes_aligned_as!(u32, $path))
    }};
}
