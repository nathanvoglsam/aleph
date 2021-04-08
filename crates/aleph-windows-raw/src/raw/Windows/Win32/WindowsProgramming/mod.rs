#![allow(
    unused_variables,
    non_upper_case_globals,
    non_snake_case,
    unused_unsafe,
    non_camel_case_types,
    dead_code,
    clippy::all
)]
#[repr(C)]
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
pub struct FILETIME {
    pub dwLowDateTime: u32,
    pub dwHighDateTime: u32,
}
impl FILETIME {}
impl ::std::default::Default for FILETIME {
    fn default() -> Self {
        Self {
            dwLowDateTime: 0,
            dwHighDateTime: 0,
        }
    }
}
impl ::std::fmt::Debug for FILETIME {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("FILETIME")
            .field("dwLowDateTime", &format_args!("{:?}", self.dwLowDateTime))
            .field("dwHighDateTime", &format_args!("{:?}", self.dwHighDateTime))
            .finish()
    }
}
impl ::std::cmp::PartialEq for FILETIME {
    fn eq(&self, other: &Self) -> bool {
        self.dwLowDateTime == other.dwLowDateTime && self.dwHighDateTime == other.dwHighDateTime
    }
}
impl ::std::cmp::Eq for FILETIME {}
unsafe impl ::windows::Abi for FILETIME {
    type Abi = Self;
}
pub type LPFIBER_START_ROUTINE = extern "system" fn(lpfiberparameter: *mut ::std::ffi::c_void);
pub unsafe fn CloseHandle<'a, T0__: ::windows::IntoParam<'a, super::SystemServices::HANDLE>>(
    hobject: T0__,
) -> super::SystemServices::BOOL {
    #[link(name = "KERNEL32")]
    extern "system" {
        pub fn CloseHandle(hobject: super::SystemServices::HANDLE) -> super::SystemServices::BOOL;
    }
    CloseHandle(hobject.into_param().abi())
}
