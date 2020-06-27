//
//
// This file is a part of Aleph
//
// <ALEPH_REPO_REPLACE>
//
// <ALEPH_LICENSE_REPLACE>
//

use std::ffi::{c_void, CStr, CString};

///
/// Namespace struct for getting the clipboard text
///
pub struct Clipboard {}

impl Clipboard {
    ///
    /// Gets the current clipboard text, if there is some.
    ///
    /// Will perform an allocation to re-box the string with the rust allocator. This is so a
    /// `String` object can be passed around safely as there would be no safe way to constrain the
    /// lifetime of a string reference returned from this function
    ///
    pub fn get() -> Option<String> {
        unsafe {
            let buf = sdl2::sys::SDL_GetClipboardText();

            if buf.is_null() {
                None
            } else {
                let cstr = CStr::from_ptr(buf as *const _);
                let cstr = cstr.to_str().ok()?;
                let string = cstr.to_string();

                sdl2::sys::SDL_free(buf as *mut c_void);

                Some(string)
            }
        }
    }

    ///
    /// Gets the current clipboard text, if there is some. This will still allocate, but the null
    /// terminator will be preserved in case it is needed.
    ///
    /// Will perform an allocation to re-box the string with the rust allocator. This is so a
    /// `CString` object can be passed around safely as there would be no safe way to constrain the
    /// lifetime of a string reference returned from this function
    ///
    pub fn get_null_terminated() -> Option<CString> {
        unsafe {
            let buf = sdl2::sys::SDL_GetClipboardText();

            if buf.is_null() {
                None
            } else {
                let cstr = CStr::from_ptr(buf as *const _);
                let cstring = cstr.to_owned();

                sdl2::sys::SDL_free(buf as *mut c_void);

                Some(cstring)
            }
        }
    }

    ///
    /// Sets the current clipboard text
    ///
    pub fn set(value: &str) {
        unsafe {
            let mut string = value.to_string();
            string.push('\0');

            sdl2::sys::SDL_SetClipboardText(string.as_ptr() as *const _);
        }
    }

    ///
    /// Sets the current clipboard text with an already null terminated string (saves on an alloc)
    ///
    pub fn set_null_terminated(value: &CStr) {
        unsafe {
            sdl2::sys::SDL_SetClipboardText(value.as_ptr() as *const _);
        }
    }
}
