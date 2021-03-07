#![allow(unused_variables, non_upper_case_globals, non_snake_case)]
#[repr(C)]
#[allow(non_snake_case)]
#[derive(
    :: std :: clone :: Clone, :: std :: marker :: Copy, :: std :: cmp :: Eq, :: std :: fmt :: Debug,
)]
pub struct PSTR(pub *mut u8);
impl ::std::default::Default for PSTR {
    fn default() -> Self {
        Self(::std::ptr::null_mut())
    }
}
impl ::std::cmp::PartialEq for PSTR {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
unsafe impl ::windows::Abi for PSTR {
    type Abi = Self;
    fn drop_param<'a>(param: &mut ::windows::Param<'a, Self>) {
        if let ::windows::Param::Boxed(value) = param {
            if !value.0.is_null() {
                unsafe {
                    ::std::boxed::Box::from_raw(value.0);
                }
            }
        }
    }
}
impl<'a> ::windows::IntoParam<'a, PSTR> for &'a str {
    fn into_param(self) -> ::windows::Param<'a, PSTR> {
        ::windows::Param::Boxed(PSTR(::std::boxed::Box::<[u8]>::into_raw(
            self.bytes()
                .chain(::std::iter::once(0))
                .collect::<std::vec::Vec<u8>>()
                .into_boxed_slice(),
        ) as _))
    }
}
impl<'a> ::windows::IntoParam<'a, PSTR> for String {
    fn into_param(self) -> ::windows::Param<'a, PSTR> {
        ::windows::Param::Boxed(PSTR(::std::boxed::Box::<[u8]>::into_raw(
            self.bytes()
                .chain(::std::iter::once(0))
                .collect::<std::vec::Vec<u8>>()
                .into_boxed_slice(),
        ) as _))
    }
}
#[repr(C)]
#[allow(non_snake_case)]
#[derive(
    :: std :: clone :: Clone,
    :: std :: marker :: Copy,
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: default :: Default,
)]
pub struct BOOL(pub i32);
impl BOOL {
    #[inline]
    pub fn as_bool(self) -> bool {
        if self.0 == 0 {
            false
        } else {
            true
        }
    }
    #[inline]
    pub fn ok(self) -> ::windows::Result<()> {
        if self.as_bool() {
            Ok(())
        } else {
            Err(::windows::ErrorCode::from_thread().into())
        }
    }
    #[inline]
    pub fn unwrap(self) {
        self.ok().unwrap();
    }
    #[inline]
    pub fn expect(self, msg: &str) {
        self.ok().expect(msg);
    }
}
impl ::std::fmt::Debug for BOOL {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        let msg = if self.as_bool() { "true" } else { "false" };
        fmt.write_str(msg)
    }
}
unsafe impl ::windows::Abi for BOOL {
    type Abi = Self;
}
impl ::std::convert::From<BOOL> for bool {
    fn from(value: BOOL) -> Self {
        value.as_bool()
    }
}
impl ::std::convert::From<&BOOL> for bool {
    fn from(value: &BOOL) -> Self {
        value.as_bool()
    }
}
impl ::std::convert::From<bool> for BOOL {
    fn from(value: bool) -> Self {
        if value {
            BOOL(1)
        } else {
            BOOL(0)
        }
    }
}
impl ::std::convert::From<&bool> for BOOL {
    fn from(value: &bool) -> Self {
        (*value).into()
    }
}
impl ::std::cmp::PartialEq<bool> for BOOL {
    fn eq(&self, other: &bool) -> bool {
        self.as_bool() == *other
    }
}
impl ::std::cmp::PartialEq<BOOL> for bool {
    fn eq(&self, other: &BOOL) -> bool {
        *self == other.as_bool()
    }
}
impl std::ops::Not for BOOL {
    type Output = Self;
    fn not(self) -> Self::Output {
        if self.as_bool() {
            BOOL(0)
        } else {
            BOOL(1)
        }
    }
}
impl<'a> ::windows::IntoParam<'a, BOOL> for bool {
    fn into_param(self) -> ::windows::Param<'a, BOOL> {
        ::windows::Param::Owned(self.into())
    }
}
#[repr(C)]
#[allow(non_snake_case)]
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
pub struct HANDLE(pub isize);
impl HANDLE {}
impl ::std::default::Default for HANDLE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::std::fmt::Debug for HANDLE {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("HANDLE")
            .field("value", &format_args!("{:?}", self.0))
            .finish()
    }
}
impl ::std::cmp::PartialEq for HANDLE {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::std::cmp::Eq for HANDLE {}
unsafe impl ::windows::Abi for HANDLE {
    type Abi = Self;
}
#[repr(C)]
#[allow(non_snake_case)]
#[derive(
    :: std :: clone :: Clone, :: std :: marker :: Copy, :: std :: cmp :: Eq, :: std :: fmt :: Debug,
)]
pub struct PWSTR(pub *mut u16);
impl ::std::default::Default for PWSTR {
    fn default() -> Self {
        Self(::std::ptr::null_mut())
    }
}
impl ::std::cmp::PartialEq for PWSTR {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
unsafe impl ::windows::Abi for PWSTR {
    type Abi = Self;
    fn drop_param<'a>(param: &mut ::windows::Param<'a, Self>) {
        if let ::windows::Param::Boxed(value) = param {
            if !value.0.is_null() {
                unsafe {
                    ::std::boxed::Box::from_raw(value.0);
                }
            }
        }
    }
}
impl<'a> ::windows::IntoParam<'a, PWSTR> for &'a str {
    fn into_param(self) -> ::windows::Param<'a, PWSTR> {
        ::windows::Param::Boxed(PWSTR(::std::boxed::Box::<[u16]>::into_raw(
            self.encode_utf16()
                .chain(::std::iter::once(0))
                .collect::<std::vec::Vec<u16>>()
                .into_boxed_slice(),
        ) as _))
    }
}
impl<'a> ::windows::IntoParam<'a, PWSTR> for String {
    fn into_param(self) -> ::windows::Param<'a, PWSTR> {
        ::windows::Param::Boxed(PWSTR(::std::boxed::Box::<[u16]>::into_raw(
            self.encode_utf16()
                .chain(::std::iter::once(0))
                .collect::<std::vec::Vec<u16>>()
                .into_boxed_slice(),
        ) as _))
    }
}
#[repr(C)]
#[allow(non_snake_case)]
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
pub struct SECURITY_ATTRIBUTES {
    pub n_length: u32,
    pub lp_security_descriptor: *mut ::std::ffi::c_void,
    pub b_inherit_handle: BOOL,
}
impl SECURITY_ATTRIBUTES {}
impl ::std::default::Default for SECURITY_ATTRIBUTES {
    fn default() -> Self {
        Self {
            n_length: 0,
            lp_security_descriptor: ::std::ptr::null_mut(),
            b_inherit_handle: ::std::default::Default::default(),
        }
    }
}
impl ::std::fmt::Debug for SECURITY_ATTRIBUTES {
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
impl ::std::cmp::PartialEq for SECURITY_ATTRIBUTES {
    fn eq(&self, other: &Self) -> bool {
        self.n_length == other.n_length
            && self.lp_security_descriptor == other.lp_security_descriptor
            && self.b_inherit_handle == other.b_inherit_handle
    }
}
impl ::std::cmp::Eq for SECURITY_ATTRIBUTES {}
unsafe impl ::windows::Abi for SECURITY_ATTRIBUTES {
    type Abi = Self;
}
pub unsafe fn CreateEventA<
    'a,
    T1__: ::windows::IntoParam<'a, BOOL>,
    T2__: ::windows::IntoParam<'a, BOOL>,
    T3__: ::windows::IntoParam<'a, PSTR>,
>(
    lp_event_attributes: *mut SECURITY_ATTRIBUTES,
    b_manual_reset: T1__,
    b_initial_state: T2__,
    lp_name: T3__,
) -> HANDLE {
    #[link(name = "KERNEL32")]
    extern "system" {
        pub fn CreateEventA(
            lp_event_attributes: *mut SECURITY_ATTRIBUTES,
            b_manual_reset: BOOL,
            b_initial_state: BOOL,
            lp_name: PSTR,
        ) -> HANDLE;
    }
    CreateEventA(
        ::std::mem::transmute(lp_event_attributes),
        b_manual_reset.into_param().abi(),
        b_initial_state.into_param().abi(),
        lp_name.into_param().abi(),
    )
}
pub unsafe fn CreateEventW<
    'a,
    T1__: ::windows::IntoParam<'a, BOOL>,
    T2__: ::windows::IntoParam<'a, BOOL>,
    T3__: ::windows::IntoParam<'a, PWSTR>,
>(
    lp_event_attributes: *mut SECURITY_ATTRIBUTES,
    b_manual_reset: T1__,
    b_initial_state: T2__,
    lp_name: T3__,
) -> HANDLE {
    #[link(name = "KERNEL32")]
    extern "system" {
        pub fn CreateEventW(
            lp_event_attributes: *mut SECURITY_ATTRIBUTES,
            b_manual_reset: BOOL,
            b_initial_state: BOOL,
            lp_name: PWSTR,
        ) -> HANDLE;
    }
    CreateEventW(
        ::std::mem::transmute(lp_event_attributes),
        b_manual_reset.into_param().abi(),
        b_initial_state.into_param().abi(),
        lp_name.into_param().abi(),
    )
}
pub unsafe fn WaitForSingleObject<'a, T0__: ::windows::IntoParam<'a, HANDLE>>(
    h_handle: T0__,
    dw_milliseconds: u32,
) -> u32 {
    #[link(name = "KERNEL32")]
    extern "system" {
        pub fn WaitForSingleObject(h_handle: HANDLE, dw_milliseconds: u32) -> u32;
    }
    WaitForSingleObject(
        h_handle.into_param().abi(),
        ::std::mem::transmute(dw_milliseconds),
    )
}
pub unsafe fn WaitForMultipleObjects<'a, T2__: ::windows::IntoParam<'a, BOOL>>(
    n_count: u32,
    lp_handles: *const HANDLE,
    b_wait_all: T2__,
    dw_milliseconds: u32,
) -> u32 {
    #[link(name = "KERNEL32")]
    extern "system" {
        pub fn WaitForMultipleObjects(
            n_count: u32,
            lp_handles: *const HANDLE,
            b_wait_all: BOOL,
            dw_milliseconds: u32,
        ) -> u32;
    }
    WaitForMultipleObjects(
        ::std::mem::transmute(n_count),
        ::std::mem::transmute(lp_handles),
        b_wait_all.into_param().abi(),
        ::std::mem::transmute(dw_milliseconds),
    )
}
pub unsafe fn ResetEvent<'a, T0__: ::windows::IntoParam<'a, HANDLE>>(h_event: T0__) -> BOOL {
    #[link(name = "KERNEL32")]
    extern "system" {
        pub fn ResetEvent(h_event: HANDLE) -> BOOL;
    }
    ResetEvent(h_event.into_param().abi())
}
pub const INFINITE: u32 = 4294967295u32;
pub unsafe fn LoadLibraryW<'a, T0__: ::windows::IntoParam<'a, PWSTR>>(
    lp_lib_file_name: T0__,
) -> isize {
    #[link(name = "KERNEL32")]
    extern "system" {
        pub fn LoadLibraryW(lp_lib_file_name: PWSTR) -> isize;
    }
    LoadLibraryW(lp_lib_file_name.into_param().abi())
}
pub unsafe fn LoadLibraryA<'a, T0__: ::windows::IntoParam<'a, PSTR>>(
    lp_lib_file_name: T0__,
) -> isize {
    #[link(name = "KERNEL32")]
    extern "system" {
        pub fn LoadLibraryA(lp_lib_file_name: PSTR) -> isize;
    }
    LoadLibraryA(lp_lib_file_name.into_param().abi())
}
#[allow(non_camel_case_types)]
pub type FARPROC = extern "system" fn() -> i32;
pub unsafe fn GetProcAddress<'a, T1__: ::windows::IntoParam<'a, PSTR>>(
    h_module: isize,
    lp_proc_name: T1__,
) -> ::std::option::Option<FARPROC> {
    #[link(name = "KERNEL32")]
    extern "system" {
        pub fn GetProcAddress(
            h_module: isize,
            lp_proc_name: PSTR,
        ) -> ::std::option::Option<FARPROC>;
    }
    GetProcAddress(
        ::std::mem::transmute(h_module),
        lp_proc_name.into_param().abi(),
    )
}
pub unsafe fn GetCurrentThread() -> HANDLE {
    #[link(name = "KERNEL32")]
    extern "system" {
        pub fn GetCurrentThread() -> HANDLE;
    }
    GetCurrentThread()
}
pub unsafe fn SetThreadDescription<
    'a,
    T0__: ::windows::IntoParam<'a, HANDLE>,
    T1__: ::windows::IntoParam<'a, PWSTR>,
>(
    h_thread: T0__,
    lp_thread_description: T1__,
) -> ::windows::ErrorCode {
    #[link(name = "KERNEL32")]
    extern "system" {
        pub fn SetThreadDescription(
            h_thread: HANDLE,
            lp_thread_description: PWSTR,
        ) -> ::windows::ErrorCode;
    }
    SetThreadDescription(
        h_thread.into_param().abi(),
        lp_thread_description.into_param().abi(),
    )
}
