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
pub struct STATSTG {
    pub pwcsName: super::SystemServices::PWSTR,
    pub r#type: u32,
    pub cbSize: u64,
    pub mtime: super::WindowsProgramming::FILETIME,
    pub ctime: super::WindowsProgramming::FILETIME,
    pub atime: super::WindowsProgramming::FILETIME,
    pub grfMode: u32,
    pub grfLocksSupported: u32,
    pub clsid: ::windows::Guid,
    pub grfStateBits: u32,
    pub reserved: u32,
}
impl STATSTG {}
impl ::std::default::Default for STATSTG {
    fn default() -> Self {
        Self {
            pwcsName: ::std::default::Default::default(),
            r#type: 0,
            cbSize: 0,
            mtime: ::std::default::Default::default(),
            ctime: ::std::default::Default::default(),
            atime: ::std::default::Default::default(),
            grfMode: 0,
            grfLocksSupported: 0,
            clsid: ::std::default::Default::default(),
            grfStateBits: 0,
            reserved: 0,
        }
    }
}
impl ::std::fmt::Debug for STATSTG {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("STATSTG")
            .field("pwcsName", &format_args!("{:?}", self.pwcsName))
            .field("r#type", &format_args!("{:?}", self.r#type))
            .field("cbSize", &format_args!("{:?}", self.cbSize))
            .field("mtime", &format_args!("{:?}", self.mtime))
            .field("ctime", &format_args!("{:?}", self.ctime))
            .field("atime", &format_args!("{:?}", self.atime))
            .field("grfMode", &format_args!("{:?}", self.grfMode))
            .field(
                "grfLocksSupported",
                &format_args!("{:?}", self.grfLocksSupported),
            )
            .field("clsid", &format_args!("{:?}", self.clsid))
            .field("grfStateBits", &format_args!("{:?}", self.grfStateBits))
            .field("reserved", &format_args!("{:?}", self.reserved))
            .finish()
    }
}
impl ::std::cmp::PartialEq for STATSTG {
    fn eq(&self, other: &Self) -> bool {
        self.pwcsName == other.pwcsName
            && self.r#type == other.r#type
            && self.cbSize == other.cbSize
            && self.mtime == other.mtime
            && self.ctime == other.ctime
            && self.atime == other.atime
            && self.grfMode == other.grfMode
            && self.grfLocksSupported == other.grfLocksSupported
            && self.clsid == other.clsid
            && self.grfStateBits == other.grfStateBits
            && self.reserved == other.reserved
    }
}
impl ::std::cmp::Eq for STATSTG {}
unsafe impl ::windows::Abi for STATSTG {
    type Abi = Self;
}
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct ISequentialStream(::windows::IUnknown);
impl ISequentialStream {}
unsafe impl ::windows::Interface for ISequentialStream {
    type Vtable = ISequentialStream_abi;
    const IID: ::windows::Guid =
        ::windows::Guid::from_values(208878128, 10780, 4558, [173, 229, 0, 170, 0, 68, 119, 61]);
}
impl ISequentialStream {
    pub unsafe fn Read(
        &self,
        pv: *mut ::std::ffi::c_void,
        cb: u32,
        pcbread: *mut u32,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).3)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(pv),
            ::std::mem::transmute(cb),
            ::std::mem::transmute(pcbread),
        )
    }
    pub unsafe fn Write(
        &self,
        pv: *const ::std::ffi::c_void,
        cb: u32,
        pcbwritten: *mut u32,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).4)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(pv),
            ::std::mem::transmute(cb),
            ::std::mem::transmute(pcbwritten),
        )
    }
}
impl ::std::convert::From<ISequentialStream> for ::windows::IUnknown {
    fn from(value: ISequentialStream) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&ISequentialStream> for ::windows::IUnknown {
    fn from(value: &ISequentialStream) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::IntoParam<'a, ::windows::IUnknown> for ISequentialStream {
    fn into_param(self) -> ::windows::Param<'a, ::windows::IUnknown> {
        ::windows::Param::Owned(::std::convert::Into::<::windows::IUnknown>::into(self))
    }
}
impl<'a> ::windows::IntoParam<'a, ::windows::IUnknown> for &'a ISequentialStream {
    fn into_param(self) -> ::windows::Param<'a, ::windows::IUnknown> {
        ::windows::Param::Owned(::std::convert::Into::<::windows::IUnknown>::into(
            ::std::clone::Clone::clone(self),
        ))
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ISequentialStream_abi(
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        iid: &::windows::Guid,
        interface: *mut ::windows::RawPtr,
    ) -> ::windows::ErrorCode,
    pub unsafe extern "system" fn(this: ::windows::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::RawPtr) -> u32,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        pv: *mut ::std::ffi::c_void,
        cb: u32,
        pcbread: *mut u32,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        pv: *const ::std::ffi::c_void,
        cb: u32,
        pcbwritten: *mut u32,
    ) -> ::windows::ErrorCode,
);
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct IStream(::windows::IUnknown);
impl IStream {}
unsafe impl ::windows::Interface for IStream {
    type Vtable = IStream_abi;
    const IID: ::windows::Guid =
        ::windows::Guid::from_values(12, 0, 0, [192, 0, 0, 0, 0, 0, 0, 70]);
}
impl IStream {
    pub unsafe fn Read(
        &self,
        pv: *mut ::std::ffi::c_void,
        cb: u32,
        pcbread: *mut u32,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).3)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(pv),
            ::std::mem::transmute(cb),
            ::std::mem::transmute(pcbread),
        )
    }
    pub unsafe fn Write(
        &self,
        pv: *const ::std::ffi::c_void,
        cb: u32,
        pcbwritten: *mut u32,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).4)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(pv),
            ::std::mem::transmute(cb),
            ::std::mem::transmute(pcbwritten),
        )
    }
    pub unsafe fn SetSize(&self, libnewsize: u64) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).6)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(libnewsize),
        )
    }
    pub unsafe fn CopyTo<'a, T0__: ::windows::IntoParam<'a, IStream>>(
        &self,
        pstm: T0__,
        cb: u64,
        pcbread: *mut u64,
        pcbwritten: *mut u64,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).7)(
            ::windows::Abi::abi(self),
            pstm.into_param().abi(),
            ::std::mem::transmute(cb),
            ::std::mem::transmute(pcbread),
            ::std::mem::transmute(pcbwritten),
        )
    }
    pub unsafe fn Commit(&self, grfcommitflags: u32) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).8)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(grfcommitflags),
        )
    }
    pub unsafe fn Revert(&self) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).9)(::windows::Abi::abi(self))
    }
    pub unsafe fn LockRegion(
        &self,
        liboffset: u64,
        cb: u64,
        dwlocktype: u32,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).10)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(liboffset),
            ::std::mem::transmute(cb),
            ::std::mem::transmute(dwlocktype),
        )
    }
    pub unsafe fn UnlockRegion(
        &self,
        liboffset: u64,
        cb: u64,
        dwlocktype: u32,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).11)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(liboffset),
            ::std::mem::transmute(cb),
            ::std::mem::transmute(dwlocktype),
        )
    }
    pub unsafe fn Stat(&self, pstatstg: *mut STATSTG, grfstatflag: u32) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).12)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(pstatstg),
            ::std::mem::transmute(grfstatflag),
        )
    }
    pub unsafe fn Clone(&self, ppstm: *mut ::std::option::Option<IStream>) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).13)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(ppstm),
        )
    }
}
impl ::std::convert::From<IStream> for ::windows::IUnknown {
    fn from(value: IStream) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IStream> for ::windows::IUnknown {
    fn from(value: &IStream) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::IntoParam<'a, ::windows::IUnknown> for IStream {
    fn into_param(self) -> ::windows::Param<'a, ::windows::IUnknown> {
        ::windows::Param::Owned(::std::convert::Into::<::windows::IUnknown>::into(self))
    }
}
impl<'a> ::windows::IntoParam<'a, ::windows::IUnknown> for &'a IStream {
    fn into_param(self) -> ::windows::Param<'a, ::windows::IUnknown> {
        ::windows::Param::Owned(::std::convert::Into::<::windows::IUnknown>::into(
            ::std::clone::Clone::clone(self),
        ))
    }
}
impl ::std::convert::From<IStream> for ISequentialStream {
    fn from(value: IStream) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IStream> for ISequentialStream {
    fn from(value: &IStream) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::IntoParam<'a, ISequentialStream> for IStream {
    fn into_param(self) -> ::windows::Param<'a, ISequentialStream> {
        ::windows::Param::Owned(::std::convert::Into::<ISequentialStream>::into(self))
    }
}
impl<'a> ::windows::IntoParam<'a, ISequentialStream> for &'a IStream {
    fn into_param(self) -> ::windows::Param<'a, ISequentialStream> {
        ::windows::Param::Owned(::std::convert::Into::<ISequentialStream>::into(
            ::std::clone::Clone::clone(self),
        ))
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IStream_abi(
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        iid: &::windows::Guid,
        interface: *mut ::windows::RawPtr,
    ) -> ::windows::ErrorCode,
    pub unsafe extern "system" fn(this: ::windows::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::RawPtr) -> u32,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        pv: *mut ::std::ffi::c_void,
        cb: u32,
        pcbread: *mut u32,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        pv: *const ::std::ffi::c_void,
        cb: u32,
        pcbwritten: *mut u32,
    ) -> ::windows::ErrorCode,
    pub unsafe extern "system" fn(),
    pub unsafe extern "system" fn(this: ::windows::RawPtr, libnewsize: u64) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        pstm: ::windows::RawPtr,
        cb: u64,
        pcbread: *mut u64,
        pcbwritten: *mut u64,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        grfcommitflags: u32,
    ) -> ::windows::ErrorCode,
    pub unsafe extern "system" fn(this: ::windows::RawPtr) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        liboffset: u64,
        cb: u64,
        dwlocktype: u32,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        liboffset: u64,
        cb: u64,
        dwlocktype: u32,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        pstatstg: *mut STATSTG,
        grfstatflag: u32,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        ppstm: *mut ::windows::RawPtr,
    ) -> ::windows::ErrorCode,
);
