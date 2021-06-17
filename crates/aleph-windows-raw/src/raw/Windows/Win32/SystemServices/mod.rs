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
    fn drop_param(param: &mut ::windows::Param<Self>) {
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
        !(self.0 == 0)
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
    fn drop_param(param: &mut ::windows::Param<Self>) {
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
            .field("Value", &format_args!("{:?}", self.0))
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
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
pub struct SECURITY_ATTRIBUTES {
    pub nLength: u32,
    pub lpSecurityDescriptor: *mut ::std::ffi::c_void,
    pub bInheritHandle: BOOL,
}
impl SECURITY_ATTRIBUTES {}
impl ::std::default::Default for SECURITY_ATTRIBUTES {
    fn default() -> Self {
        Self {
            nLength: 0,
            lpSecurityDescriptor: ::std::ptr::null_mut(),
            bInheritHandle: ::std::default::Default::default(),
        }
    }
}
impl ::std::fmt::Debug for SECURITY_ATTRIBUTES {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("SECURITY_ATTRIBUTES")
            .field("nLength", &format_args!("{:?}", self.nLength))
            .field(
                "lpSecurityDescriptor",
                &format_args!("{:?}", self.lpSecurityDescriptor),
            )
            .field("bInheritHandle", &format_args!("{:?}", self.bInheritHandle))
            .finish()
    }
}
impl ::std::cmp::PartialEq for SECURITY_ATTRIBUTES {
    fn eq(&self, other: &Self) -> bool {
        self.nLength == other.nLength
            && self.lpSecurityDescriptor == other.lpSecurityDescriptor
            && self.bInheritHandle == other.bInheritHandle
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
    lpeventattributes: *mut SECURITY_ATTRIBUTES,
    bmanualreset: T1__,
    binitialstate: T2__,
    lpname: T3__,
) -> HANDLE {
    #[link(name = "KERNEL32")]
    extern "system" {
        pub fn CreateEventA(
            lpeventattributes: *mut SECURITY_ATTRIBUTES,
            bmanualreset: BOOL,
            binitialstate: BOOL,
            lpname: PSTR,
        ) -> HANDLE;
    }
    CreateEventA(
        ::std::mem::transmute(lpeventattributes),
        bmanualreset.into_param().abi(),
        binitialstate.into_param().abi(),
        lpname.into_param().abi(),
    )
}
pub unsafe fn CreateEventW<
    'a,
    T1__: ::windows::IntoParam<'a, BOOL>,
    T2__: ::windows::IntoParam<'a, BOOL>,
    T3__: ::windows::IntoParam<'a, PWSTR>,
>(
    lpeventattributes: *mut SECURITY_ATTRIBUTES,
    bmanualreset: T1__,
    binitialstate: T2__,
    lpname: T3__,
) -> HANDLE {
    #[link(name = "KERNEL32")]
    extern "system" {
        pub fn CreateEventW(
            lpeventattributes: *mut SECURITY_ATTRIBUTES,
            bmanualreset: BOOL,
            binitialstate: BOOL,
            lpname: PWSTR,
        ) -> HANDLE;
    }
    CreateEventW(
        ::std::mem::transmute(lpeventattributes),
        bmanualreset.into_param().abi(),
        binitialstate.into_param().abi(),
        lpname.into_param().abi(),
    )
}
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: marker :: Copy,
    :: std :: clone :: Clone,
    :: std :: default :: Default,
    :: std :: fmt :: Debug,
)]
#[repr(transparent)]
pub struct WAIT_RETURN_CAUSE(pub u32);
impl WAIT_RETURN_CAUSE {
    pub const WAIT_OBJECT_0: Self = Self(0u32);
    pub const WAIT_ABANDONED: Self = Self(128u32);
    pub const WAIT_ABANDONED_0: Self = Self(128u32);
    pub const WAIT_IO_COMPLETION: Self = Self(192u32);
    pub const WAIT_TIMEOUT: Self = Self(258u32);
    pub const WAIT_FAILED: Self = Self(4294967295u32);
}
impl ::std::convert::From<u32> for WAIT_RETURN_CAUSE {
    fn from(value: u32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::Abi for WAIT_RETURN_CAUSE {
    type Abi = Self;
}
impl ::std::ops::BitOr for WAIT_RETURN_CAUSE {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}
impl ::std::ops::BitAnd for WAIT_RETURN_CAUSE {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}
impl ::std::ops::BitOrAssign for WAIT_RETURN_CAUSE {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0.bitor_assign(rhs.0)
    }
}
impl ::std::ops::BitAndAssign for WAIT_RETURN_CAUSE {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0.bitand_assign(rhs.0)
    }
}
pub unsafe fn WaitForSingleObject<'a, T0__: ::windows::IntoParam<'a, HANDLE>>(
    hhandle: T0__,
    dwmilliseconds: u32,
) -> WAIT_RETURN_CAUSE {
    #[link(name = "KERNEL32")]
    extern "system" {
        pub fn WaitForSingleObject(hhandle: HANDLE, dwmilliseconds: u32) -> WAIT_RETURN_CAUSE;
    }
    WaitForSingleObject(
        hhandle.into_param().abi(),
        ::std::mem::transmute(dwmilliseconds),
    )
}
pub unsafe fn WaitForMultipleObjects<'a, T2__: ::windows::IntoParam<'a, BOOL>>(
    ncount: u32,
    lphandles: *const HANDLE,
    bwaitall: T2__,
    dwmilliseconds: u32,
) -> WAIT_RETURN_CAUSE {
    #[link(name = "KERNEL32")]
    extern "system" {
        pub fn WaitForMultipleObjects(
            ncount: u32,
            lphandles: *const HANDLE,
            bwaitall: BOOL,
            dwmilliseconds: u32,
        ) -> WAIT_RETURN_CAUSE;
    }
    WaitForMultipleObjects(
        ::std::mem::transmute(ncount),
        ::std::mem::transmute(lphandles),
        bwaitall.into_param().abi(),
        ::std::mem::transmute(dwmilliseconds),
    )
}
pub unsafe fn ResetEvent<'a, T0__: ::windows::IntoParam<'a, HANDLE>>(hevent: T0__) -> BOOL {
    #[link(name = "KERNEL32")]
    extern "system" {
        pub fn ResetEvent(hevent: HANDLE) -> BOOL;
    }
    ResetEvent(hevent.into_param().abi())
}
pub unsafe fn LoadLibraryW<'a, T0__: ::windows::IntoParam<'a, PWSTR>>(
    lplibfilename: T0__,
) -> isize {
    #[link(name = "KERNEL32")]
    extern "system" {
        pub fn LoadLibraryW(lplibfilename: PWSTR) -> isize;
    }
    LoadLibraryW(lplibfilename.into_param().abi())
}
pub unsafe fn LoadLibraryA<'a, T0__: ::windows::IntoParam<'a, PSTR>>(lplibfilename: T0__) -> isize {
    #[link(name = "KERNEL32")]
    extern "system" {
        pub fn LoadLibraryA(lplibfilename: PSTR) -> isize;
    }
    LoadLibraryA(lplibfilename.into_param().abi())
}
pub type FARPROC = extern "system" fn() -> isize;
pub unsafe fn GetProcAddress<'a, T1__: ::windows::IntoParam<'a, PSTR>>(
    hmodule: isize,
    lpprocname: T1__,
) -> ::std::option::Option<FARPROC> {
    #[link(name = "KERNEL32")]
    extern "system" {
        pub fn GetProcAddress(hmodule: isize, lpprocname: PSTR) -> ::std::option::Option<FARPROC>;
    }
    GetProcAddress(
        ::std::mem::transmute(hmodule),
        lpprocname.into_param().abi(),
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
    hthread: T0__,
    lpthreaddescription: T1__,
) -> ::windows::ErrorCode {
    #[link(name = "KERNEL32")]
    extern "system" {
        pub fn SetThreadDescription(
            hthread: HANDLE,
            lpthreaddescription: PWSTR,
        ) -> ::windows::ErrorCode;
    }
    SetThreadDescription(
        hthread.into_param().abi(),
        lpthreaddescription.into_param().abi(),
    )
}
pub unsafe fn ConvertThreadToFiberEx(
    lpparameter: *mut ::std::ffi::c_void,
    dwflags: u32,
) -> *mut ::std::ffi::c_void {
    #[link(name = "KERNEL32")]
    extern "system" {
        pub fn ConvertThreadToFiberEx(
            lpparameter: *mut ::std::ffi::c_void,
            dwflags: u32,
        ) -> *mut ::std::ffi::c_void;
    }
    ConvertThreadToFiberEx(
        ::std::mem::transmute(lpparameter),
        ::std::mem::transmute(dwflags),
    )
}
pub unsafe fn CreateFiberEx(
    dwstackcommitsize: usize,
    dwstackreservesize: usize,
    dwflags: u32,
    lpstartaddress: ::std::option::Option<super::WindowsProgramming::LPFIBER_START_ROUTINE>,
    lpparameter: *mut ::std::ffi::c_void,
) -> *mut ::std::ffi::c_void {
    #[link(name = "KERNEL32")]
    extern "system" {
        pub fn CreateFiberEx(
            dwstackcommitsize: usize,
            dwstackreservesize: usize,
            dwflags: u32,
            lpstartaddress: ::windows::RawPtr,
            lpparameter: *mut ::std::ffi::c_void,
        ) -> *mut ::std::ffi::c_void;
    }
    CreateFiberEx(
        ::std::mem::transmute(dwstackcommitsize),
        ::std::mem::transmute(dwstackreservesize),
        ::std::mem::transmute(dwflags),
        ::std::mem::transmute(lpstartaddress),
        ::std::mem::transmute(lpparameter),
    )
}
pub unsafe fn DeleteFiber(lpfiber: *mut ::std::ffi::c_void) {
    #[link(name = "KERNEL32")]
    extern "system" {
        pub fn DeleteFiber(lpfiber: *mut ::std::ffi::c_void);
    }
    DeleteFiber(::std::mem::transmute(lpfiber))
}
pub unsafe fn ConvertFiberToThread() -> BOOL {
    #[link(name = "KERNEL32")]
    extern "system" {
        pub fn ConvertFiberToThread() -> BOOL;
    }
    ConvertFiberToThread()
}
pub unsafe fn SwitchToFiber(lpfiber: *mut ::std::ffi::c_void) {
    #[link(name = "KERNEL32")]
    extern "system" {
        pub fn SwitchToFiber(lpfiber: *mut ::std::ffi::c_void);
    }
    SwitchToFiber(::std::mem::transmute(lpfiber))
}
pub unsafe fn GetPhysicallyInstalledSystemMemory(totalmemoryinkilobytes: *mut u64) -> BOOL {
    #[link(name = "KERNEL32")]
    extern "system" {
        pub fn GetPhysicallyInstalledSystemMemory(totalmemoryinkilobytes: *mut u64) -> BOOL;
    }
    GetPhysicallyInstalledSystemMemory(::std::mem::transmute(totalmemoryinkilobytes))
}
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: marker :: Copy,
    :: std :: clone :: Clone,
    :: std :: default :: Default,
    :: std :: fmt :: Debug,
)]
#[repr(transparent)]
pub struct VirtualAlloc_flAllocationType(pub u32);
impl VirtualAlloc_flAllocationType {
    pub const MEM_COMMIT: Self = Self(4096u32);
    pub const MEM_RESERVE: Self = Self(8192u32);
    pub const MEM_RESET: Self = Self(524288u32);
    pub const MEM_RESET_UNDO: Self = Self(16777216u32);
    pub const MEM_REPLACE_PLACEHOLDER: Self = Self(16384u32);
    pub const MEM_LARGE_PAGES: Self = Self(536870912u32);
    pub const MEM_RESERVE_PLACEHOLDER: Self = Self(262144u32);
    pub const MEM_FREE: Self = Self(65536u32);
}
impl ::std::convert::From<u32> for VirtualAlloc_flAllocationType {
    fn from(value: u32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::Abi for VirtualAlloc_flAllocationType {
    type Abi = Self;
}
impl ::std::ops::BitOr for VirtualAlloc_flAllocationType {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}
impl ::std::ops::BitAnd for VirtualAlloc_flAllocationType {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}
impl ::std::ops::BitOrAssign for VirtualAlloc_flAllocationType {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0.bitor_assign(rhs.0)
    }
}
impl ::std::ops::BitAndAssign for VirtualAlloc_flAllocationType {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0.bitand_assign(rhs.0)
    }
}
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: marker :: Copy,
    :: std :: clone :: Clone,
    :: std :: default :: Default,
    :: std :: fmt :: Debug,
)]
#[repr(transparent)]
pub struct PAGE_TYPE(pub u32);
impl PAGE_TYPE {
    pub const PAGE_NOACCESS: Self = Self(1u32);
    pub const PAGE_READONLY: Self = Self(2u32);
    pub const PAGE_READWRITE: Self = Self(4u32);
    pub const PAGE_WRITECOPY: Self = Self(8u32);
    pub const PAGE_EXECUTE: Self = Self(16u32);
    pub const PAGE_EXECUTE_READ: Self = Self(32u32);
    pub const PAGE_EXECUTE_READWRITE: Self = Self(64u32);
    pub const PAGE_EXECUTE_WRITECOPY: Self = Self(128u32);
    pub const PAGE_GUARD: Self = Self(256u32);
    pub const PAGE_NOCACHE: Self = Self(512u32);
    pub const PAGE_WRITECOMBINE: Self = Self(1024u32);
    pub const PAGE_TARGETS_INVALID: Self = Self(1073741824u32);
    pub const PAGE_TARGETS_NO_UPDATE: Self = Self(1073741824u32);
}
impl ::std::convert::From<u32> for PAGE_TYPE {
    fn from(value: u32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::Abi for PAGE_TYPE {
    type Abi = Self;
}
impl ::std::ops::BitOr for PAGE_TYPE {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}
impl ::std::ops::BitAnd for PAGE_TYPE {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}
impl ::std::ops::BitOrAssign for PAGE_TYPE {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0.bitor_assign(rhs.0)
    }
}
impl ::std::ops::BitAndAssign for PAGE_TYPE {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0.bitand_assign(rhs.0)
    }
}
pub unsafe fn VirtualAlloc(
    lpaddress: *mut ::std::ffi::c_void,
    dwsize: usize,
    flallocationtype: VirtualAlloc_flAllocationType,
    flprotect: PAGE_TYPE,
) -> *mut ::std::ffi::c_void {
    #[link(name = "KERNEL32")]
    extern "system" {
        pub fn VirtualAlloc(
            lpaddress: *mut ::std::ffi::c_void,
            dwsize: usize,
            flallocationtype: VirtualAlloc_flAllocationType,
            flprotect: PAGE_TYPE,
        ) -> *mut ::std::ffi::c_void;
    }
    VirtualAlloc(
        ::std::mem::transmute(lpaddress),
        ::std::mem::transmute(dwsize),
        ::std::mem::transmute(flallocationtype),
        ::std::mem::transmute(flprotect),
    )
}
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: marker :: Copy,
    :: std :: clone :: Clone,
    :: std :: default :: Default,
    :: std :: fmt :: Debug,
)]
#[repr(transparent)]
pub struct VirtualFree_dwFreeType(pub u32);
impl VirtualFree_dwFreeType {
    pub const MEM_DECOMMIT: Self = Self(16384u32);
    pub const MEM_RELEASE: Self = Self(32768u32);
}
impl ::std::convert::From<u32> for VirtualFree_dwFreeType {
    fn from(value: u32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::Abi for VirtualFree_dwFreeType {
    type Abi = Self;
}
impl ::std::ops::BitOr for VirtualFree_dwFreeType {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}
impl ::std::ops::BitAnd for VirtualFree_dwFreeType {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}
impl ::std::ops::BitOrAssign for VirtualFree_dwFreeType {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0.bitor_assign(rhs.0)
    }
}
impl ::std::ops::BitAndAssign for VirtualFree_dwFreeType {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0.bitand_assign(rhs.0)
    }
}
pub unsafe fn VirtualFree(
    lpaddress: *mut ::std::ffi::c_void,
    dwsize: usize,
    dwfreetype: VirtualFree_dwFreeType,
) -> BOOL {
    #[link(name = "KERNEL32")]
    extern "system" {
        pub fn VirtualFree(
            lpaddress: *mut ::std::ffi::c_void,
            dwsize: usize,
            dwfreetype: VirtualFree_dwFreeType,
        ) -> BOOL;
    }
    VirtualFree(
        ::std::mem::transmute(lpaddress),
        ::std::mem::transmute(dwsize),
        ::std::mem::transmute(dwfreetype),
    )
}
