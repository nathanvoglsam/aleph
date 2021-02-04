#[repr(C)]
#[allow(non_camel_case_types)]
pub struct HANDLE(pub isize);
#[repr(C)]
#[doc(hidden)]
#[allow(non_camel_case_types)]
pub struct HANDLE_abi(isize);
impl HANDLE {}
unsafe impl ::windows::Abi for HANDLE {
    type Abi = HANDLE_abi;
}
impl ::core::default::Default for HANDLE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for HANDLE {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("HANDLE")
            .field("value", &format_args!("{:?}", self.0))
            .finish()
    }
}
impl ::core::clone::Clone for HANDLE {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}
#[repr(C)]
#[allow(non_camel_case_types)]
pub struct SECURITY_ATTRIBUTES {
    pub n_length: u32,
    pub lp_security_descriptor: *mut ::std::ffi::c_void,
    pub b_inherit_handle: ::windows::BOOL,
}
#[repr(C)]
#[doc(hidden)]
#[allow(non_camel_case_types)]
pub struct SECURITY_ATTRIBUTES_abi(u32, *mut ::std::ffi::c_void, ::windows::BOOL);
impl SECURITY_ATTRIBUTES {}
unsafe impl ::windows::Abi for SECURITY_ATTRIBUTES {
    type Abi = SECURITY_ATTRIBUTES_abi;
}
impl ::core::default::Default for SECURITY_ATTRIBUTES {
    fn default() -> Self {
        Self {
            n_length: 0,
            lp_security_descriptor: ::std::ptr::null_mut(),
            b_inherit_handle: ::std::default::Default::default(),
        }
    }
}
impl ::core::fmt::Debug for SECURITY_ATTRIBUTES {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("SECURITY_ATTRIBUTES")
            .field("n_length", &format_args!("{:?}", self.n_length))
            .field(
                "lp_security_descriptor",
                &format_args!("{:?}", self.lp_security_descriptor),
            )
            .field(
                "b_inherit_handle",
                &format_args!("{:?}", self.b_inherit_handle),
            )
            .finish()
    }
}
impl ::core::clone::Clone for SECURITY_ATTRIBUTES {
    fn clone(&self) -> Self {
        Self {
            n_length: self.n_length,
            lp_security_descriptor: <*mut ::std::ffi::c_void as std::clone::Clone>::clone(
                &self.lp_security_descriptor,
            ),
            b_inherit_handle: <::windows::BOOL as std::clone::Clone>::clone(&self.b_inherit_handle),
        }
    }
}
#[link(name = "KERNEL32")]
extern "system" {
    pub fn CreateEventA(
        lp_event_attributes: *mut SECURITY_ATTRIBUTES,
        b_manual_reset: ::windows::BOOL,
        b_initial_state: ::windows::BOOL,
        lp_name: *const i8,
    ) -> HANDLE;
}
#[link(name = "KERNEL32")]
extern "system" {
    pub fn CreateEventW(
        lp_event_attributes: *mut SECURITY_ATTRIBUTES,
        b_manual_reset: ::windows::BOOL,
        b_initial_state: ::windows::BOOL,
        lp_name: *const u16,
    ) -> HANDLE;
}
#[link(name = "KERNEL32")]
extern "system" {
    pub fn WaitForSingleObject(h_handle: HANDLE, dw_milliseconds: u32) -> u32;
}
#[link(name = "KERNEL32")]
extern "system" {
    pub fn WaitForMultipleObjects(
        n_count: u32,
        lp_handles: *const isize,
        b_wait_all: ::windows::BOOL,
        dw_milliseconds: u32,
    ) -> u32;
}
#[link(name = "KERNEL32")]
extern "system" {
    pub fn ResetEvent(h_event: HANDLE) -> ::windows::BOOL;
}
pub const INFINITE: u32 = 4294967295u32;
