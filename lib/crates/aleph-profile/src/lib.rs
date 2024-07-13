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

// This crate is a fork of aclysma/profiling
//
// MIT License
//
// Copyright (c) 2020 Philip Degarmo and other contributors
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

use std::ffi::{ CStr, CString };

#[cfg(feature = "procmacros")]
pub use aleph_profile_procmacros::all_functions;

#[cfg(feature = "procmacros")]
pub use aleph_profile_procmacros::function;

#[cfg(feature = "procmacros")]
pub use aleph_profile_procmacros::skip;

#[cfg(feature = "profile-with-superluminal")]
pub use superluminal_perf;

#[cfg(feature = "profile-with-tracy")]
pub use tracy_client;

#[cfg(feature = "profile-with-superluminal")]
mod impl_superluminal;
#[cfg(feature = "profile-with-superluminal")]
#[allow(unused_imports)]
pub use impl_superluminal::*;

#[cfg(feature = "profile-with-tracy")]
mod impl_tracy;
#[cfg(feature = "profile-with-tracy")]
#[allow(unused_imports)]
pub use impl_tracy::*;

#[cfg(feature = "profile-with-pix")]
mod impl_pix;
#[cfg(feature = "profile-with-pix")]
#[allow(unused_imports)]
pub use impl_pix::*;

#[cfg(not(any(
    feature = "profile-with-superluminal",
    feature = "profile-with-tracy",
    feature = "profile-with-pix"
)))]
mod impl_empty;

#[cfg(not(any(
    feature = "profile-with-superluminal",
    feature = "profile-with-tracy",
    feature = "profile-with-pix"
)))]
#[allow(unused_imports)]
pub use impl_empty::*;

pub trait ProfileDataParam {
    fn as_str(&self) -> &str;
    fn as_cstr(&self) -> Option<&CStr>;
    fn to_cstr(&self) -> CString;
}

impl ProfileDataParam for &str {
    #[inline(always)]
    fn as_str(&self) -> &str {
        *self
    }

    #[inline(always)]
    fn as_cstr(&self) -> Option<&CStr> {
        CStr::from_bytes_with_nul(self.as_bytes()).ok()
    }

    #[inline(always)]
    fn to_cstr(&self) -> CString {
        CString::new(*self).unwrap()
    }
}

impl ProfileDataParam for &CStr {
    #[inline(always)]
    fn as_str(&self) -> &str {
        self.to_str().unwrap()
    }

    #[inline(always)]
    fn as_cstr(&self) -> Option<&CStr> {
        Some(*self)
    }

    #[inline(always)]
    fn to_cstr(&self) -> CString {
        CString::from(*self)
    }
}
