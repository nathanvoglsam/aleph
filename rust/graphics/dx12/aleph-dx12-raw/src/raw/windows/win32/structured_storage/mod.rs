#[repr(C)]
#[allow(non_snake_case)]
pub struct STATSTG {
    pub pwcs_name: *mut u16,
    pub r#type: u32,
    pub cb_size: u64,
    pub mtime: super::windows_programming::FILETIME,
    pub ctime: super::windows_programming::FILETIME,
    pub atime: super::windows_programming::FILETIME,
    pub grf_mode: u32,
    pub grf_locks_supported: u32,
    pub clsid: ::windows::Guid,
    pub grf_state_bits: u32,
    pub reserved: u32,
}
impl STATSTG {}
#[repr(C)]
#[doc(hidden)]
pub struct STATSTG_abi(
    *mut u16,
    u32,
    u64,
    super::windows_programming::FILETIME_abi,
    super::windows_programming::FILETIME_abi,
    super::windows_programming::FILETIME_abi,
    u32,
    u32,
    ::windows::Guid,
    u32,
    u32,
);
unsafe impl ::windows::Abi for STATSTG {
    type Abi = STATSTG_abi;
}
impl ::std::default::Default for STATSTG {
    fn default() -> Self {
        Self {
            pwcs_name: ::std::ptr::null_mut(),
            r#type: 0,
            cb_size: 0,
            mtime: ::std::default::Default::default(),
            ctime: ::std::default::Default::default(),
            atime: ::std::default::Default::default(),
            grf_mode: 0,
            grf_locks_supported: 0,
            clsid: ::windows::Guid::zeroed(),
            grf_state_bits: 0,
            reserved: 0,
        }
    }
}
impl ::std::fmt::Debug for STATSTG {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("STATSTG")
            .field("pwcs_name", &format_args!("{:?}", self.pwcs_name))
            .field("type", &format_args!("{:?}", self.r#type))
            .field("cb_size", &format_args!("{:?}", self.cb_size))
            .field("mtime", &format_args!("{:?}", self.mtime))
            .field("ctime", &format_args!("{:?}", self.ctime))
            .field("atime", &format_args!("{:?}", self.atime))
            .field("grf_mode", &format_args!("{:?}", self.grf_mode))
            .field(
                "grf_locks_supported",
                &format_args!("{:?}", self.grf_locks_supported),
            )
            .field("clsid", &format_args!("{:?}", self.clsid))
            .field("grf_state_bits", &format_args!("{:?}", self.grf_state_bits))
            .field("reserved", &format_args!("{:?}", self.reserved))
            .finish()
    }
}
impl ::std::clone::Clone for STATSTG {
    fn clone(&self) -> Self {
        Self {
            pwcs_name: self.pwcs_name,
            r#type: self.r#type,
            cb_size: self.cb_size,
            mtime: <super::windows_programming::FILETIME as std::clone::Clone>::clone(&self.mtime),
            ctime: <super::windows_programming::FILETIME as std::clone::Clone>::clone(&self.ctime),
            atime: <super::windows_programming::FILETIME as std::clone::Clone>::clone(&self.atime),
            grf_mode: self.grf_mode,
            grf_locks_supported: self.grf_locks_supported,
            clsid: <::windows::Guid as std::clone::Clone>::clone(&self.clsid),
            grf_state_bits: self.grf_state_bits,
            reserved: self.reserved,
        }
    }
}
impl ::std::cmp::PartialEq for STATSTG {
    fn eq(&self, other: &Self) -> bool {
        self.pwcs_name == other.pwcs_name
            && self.r#type == other.r#type
            && self.cb_size == other.cb_size
            && self.mtime == other.mtime
            && self.ctime == other.ctime
            && self.atime == other.atime
            && self.grf_mode == other.grf_mode
            && self.grf_locks_supported == other.grf_locks_supported
            && self.clsid == other.clsid
            && self.grf_state_bits == other.grf_state_bits
            && self.reserved == other.reserved
    }
}
impl ::std::cmp::Eq for STATSTG {}
#[repr(transparent)]
#[allow(non_camel_case_types)]
pub struct ISequentialStream(::windows::IUnknown);
impl ::std::clone::Clone for ISequentialStream {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::std::fmt::Debug for ISequentialStream {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        write!(f, "{:?}", self.0)
    }
}
impl ::std::cmp::PartialEq for ISequentialStream {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::std::cmp::Eq for ISequentialStream {}
unsafe impl ::windows::Interface for ISequentialStream {
    type Vtable = ISequentialStream_abi;
    const IID: ::windows::Guid =
        ::windows::Guid::from_values(208878128, 10780, 4558, [173, 229, 0, 170, 0, 68, 119, 61]);
}
#[repr(C)]
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
        pcb_read: *mut u32,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        pv: *const ::std::ffi::c_void,
        cb: u32,
        pcb_written: *mut u32,
    ) -> ::windows::ErrorCode,
);
#[allow(non_snake_case)]
impl ISequentialStream {
    pub unsafe fn Read(
        &self,
        pv: *mut ::std::ffi::c_void,
        cb: u32,
        pcb_read: *mut u32,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).3)(::windows::Abi::abi(self), pv, cb, pcb_read)
    }
    pub unsafe fn Write(
        &self,
        pv: *const ::std::ffi::c_void,
        cb: u32,
        pcb_written: *mut u32,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).4)(::windows::Abi::abi(self), pv, cb, pcb_written)
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
impl<'a> ::std::convert::Into<::windows::Param<'a, ::windows::IUnknown>> for ISequentialStream {
    fn into(self) -> ::windows::Param<'a, ::windows::IUnknown> {
        ::windows::Param::Owned(::std::convert::Into::<::windows::IUnknown>::into(self))
    }
}
impl<'a> ::std::convert::Into<::windows::Param<'a, ::windows::IUnknown>> for &'a ISequentialStream {
    fn into(self) -> ::windows::Param<'a, ::windows::IUnknown> {
        ::windows::Param::Owned(::std::convert::Into::<::windows::IUnknown>::into(
            ::std::clone::Clone::clone(self),
        ))
    }
}
#[repr(transparent)]
#[allow(non_camel_case_types)]
pub struct IStream(::windows::IUnknown);
impl ::std::clone::Clone for IStream {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::std::fmt::Debug for IStream {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        write!(f, "{:?}", self.0)
    }
}
impl ::std::cmp::PartialEq for IStream {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::std::cmp::Eq for IStream {}
unsafe impl ::windows::Interface for IStream {
    type Vtable = IStream_abi;
    const IID: ::windows::Guid =
        ::windows::Guid::from_values(12, 0, 0, [192, 0, 0, 0, 0, 0, 0, 70]);
}
#[repr(C)]
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
        pcb_read: *mut u32,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        pv: *const ::std::ffi::c_void,
        cb: u32,
        pcb_written: *mut u32,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        dlib_move: i64,
        dw_origin: u32,
        plib_new_position: *mut u64,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        lib_new_size: u64,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        pstm: ::windows::RawPtr,
        cb: u64,
        pcb_read: *mut u64,
        pcb_written: *mut u64,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        grf_commit_flags: u32,
    ) -> ::windows::ErrorCode,
    pub unsafe extern "system" fn(this: ::windows::RawPtr) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        lib_offset: u64,
        cb: u64,
        dw_lock_type: u32,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        lib_offset: u64,
        cb: u64,
        dw_lock_type: u32,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        pstatstg: *mut STATSTG,
        grf_stat_flag: u32,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        ppstm: *mut ::std::option::Option<IStream>,
    ) -> ::windows::ErrorCode,
);
#[allow(non_snake_case)]
impl IStream {
    pub unsafe fn Read(
        &self,
        pv: *mut ::std::ffi::c_void,
        cb: u32,
        pcb_read: *mut u32,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).3)(::windows::Abi::abi(self), pv, cb, pcb_read)
    }
    pub unsafe fn Write(
        &self,
        pv: *const ::std::ffi::c_void,
        cb: u32,
        pcb_written: *mut u32,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).4)(::windows::Abi::abi(self), pv, cb, pcb_written)
    }
    pub unsafe fn Seek(
        &self,
        dlib_move: i64,
        dw_origin: u32,
        plib_new_position: *mut u64,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).5)(
            ::windows::Abi::abi(self),
            dlib_move,
            dw_origin,
            plib_new_position,
        )
    }
    pub unsafe fn SetSize(&self, lib_new_size: u64) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).6)(::windows::Abi::abi(self), lib_new_size)
    }
    pub unsafe fn CopyTo<'a, T0__: ::std::convert::Into<::windows::Param<'a, IStream>>>(
        &self,
        pstm: T0__,
        cb: u64,
        pcb_read: *mut u64,
        pcb_written: *mut u64,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).7)(
            ::windows::Abi::abi(self),
            pstm.into().abi(),
            cb,
            pcb_read,
            pcb_written,
        )
    }
    pub unsafe fn Commit(&self, grf_commit_flags: u32) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).8)(::windows::Abi::abi(self), grf_commit_flags)
    }
    pub unsafe fn Revert(&self) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).9)(::windows::Abi::abi(self))
    }
    pub unsafe fn LockRegion(
        &self,
        lib_offset: u64,
        cb: u64,
        dw_lock_type: u32,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).10)(
            ::windows::Abi::abi(self),
            lib_offset,
            cb,
            dw_lock_type,
        )
    }
    pub unsafe fn UnlockRegion(
        &self,
        lib_offset: u64,
        cb: u64,
        dw_lock_type: u32,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).11)(
            ::windows::Abi::abi(self),
            lib_offset,
            cb,
            dw_lock_type,
        )
    }
    pub unsafe fn Stat(&self, pstatstg: *mut STATSTG, grf_stat_flag: u32) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).12)(::windows::Abi::abi(self), pstatstg, grf_stat_flag)
    }
    pub unsafe fn Clone(&self, ppstm: *mut ::std::option::Option<IStream>) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).13)(::windows::Abi::abi(self), ppstm)
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
impl<'a> ::std::convert::Into<::windows::Param<'a, ::windows::IUnknown>> for IStream {
    fn into(self) -> ::windows::Param<'a, ::windows::IUnknown> {
        ::windows::Param::Owned(::std::convert::Into::<::windows::IUnknown>::into(self))
    }
}
impl<'a> ::std::convert::Into<::windows::Param<'a, ::windows::IUnknown>> for &'a IStream {
    fn into(self) -> ::windows::Param<'a, ::windows::IUnknown> {
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
impl<'a> ::std::convert::Into<::windows::Param<'a, ISequentialStream>> for IStream {
    fn into(self) -> ::windows::Param<'a, ISequentialStream> {
        ::windows::Param::Owned(::std::convert::Into::<ISequentialStream>::into(self))
    }
}
impl<'a> ::std::convert::Into<::windows::Param<'a, ISequentialStream>> for &'a IStream {
    fn into(self) -> ::windows::Param<'a, ISequentialStream> {
        ::windows::Param::Owned(::std::convert::Into::<ISequentialStream>::into(
            ::std::clone::Clone::clone(self),
        ))
    }
}
