#[repr(transparent)]
#[allow(non_camel_case_types)]
pub struct IMalloc(::windows::IUnknown);
impl ::std::clone::Clone for IMalloc {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::std::fmt::Debug for IMalloc {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        write!(f, "{:?}", self.0)
    }
}
impl ::std::cmp::PartialEq for IMalloc {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::std::cmp::Eq for IMalloc {}
unsafe impl ::windows::Interface for IMalloc {
    type Vtable = IMalloc_abi;
    const IID: ::windows::Guid = ::windows::Guid::from_values(2, 0, 0, [192, 0, 0, 0, 0, 0, 0, 70]);
}
#[repr(C)]
pub struct IMalloc_abi(
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        iid: &::windows::Guid,
        interface: *mut ::windows::RawPtr,
    ) -> ::windows::ErrorCode,
    pub unsafe extern "system" fn(this: ::windows::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::RawPtr, cb: usize) -> *mut ::std::ffi::c_void,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        pv: *mut ::std::ffi::c_void,
        cb: usize,
    ) -> *mut ::std::ffi::c_void,
    pub unsafe extern "system" fn(this: ::windows::RawPtr, pv: *mut ::std::ffi::c_void),
    pub unsafe extern "system" fn(this: ::windows::RawPtr, pv: *mut ::std::ffi::c_void) -> usize,
    pub unsafe extern "system" fn(this: ::windows::RawPtr, pv: *mut ::std::ffi::c_void) -> i32,
    pub unsafe extern "system" fn(this: ::windows::RawPtr),
);
#[allow(non_snake_case)]
impl IMalloc {
    pub unsafe fn Alloc(&self, cb: usize) -> *mut ::std::ffi::c_void {
        (::windows::Interface::vtable(self).3)(::windows::Abi::abi(self), cb)
    }
    pub unsafe fn Realloc(
        &self,
        pv: *mut ::std::ffi::c_void,
        cb: usize,
    ) -> *mut ::std::ffi::c_void {
        (::windows::Interface::vtable(self).4)(::windows::Abi::abi(self), pv, cb)
    }
    pub unsafe fn Free(&self, pv: *mut ::std::ffi::c_void) {
        (::windows::Interface::vtable(self).5)(::windows::Abi::abi(self), pv)
    }
    pub unsafe fn GetSize(&self, pv: *mut ::std::ffi::c_void) -> usize {
        (::windows::Interface::vtable(self).6)(::windows::Abi::abi(self), pv)
    }
    pub unsafe fn DidAlloc(&self, pv: *mut ::std::ffi::c_void) -> i32 {
        (::windows::Interface::vtable(self).7)(::windows::Abi::abi(self), pv)
    }
    pub unsafe fn HeapMinimize(&self) {
        (::windows::Interface::vtable(self).8)(::windows::Abi::abi(self))
    }
}
impl ::std::convert::From<IMalloc> for ::windows::IUnknown {
    fn from(value: IMalloc) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IMalloc> for ::windows::IUnknown {
    fn from(value: &IMalloc) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::std::convert::Into<::windows::Param<'a, ::windows::IUnknown>> for IMalloc {
    fn into(self) -> ::windows::Param<'a, ::windows::IUnknown> {
        ::windows::Param::Owned(::std::convert::Into::<::windows::IUnknown>::into(self))
    }
}
impl<'a> ::std::convert::Into<::windows::Param<'a, ::windows::IUnknown>> for &'a IMalloc {
    fn into(self) -> ::windows::Param<'a, ::windows::IUnknown> {
        ::windows::Param::Owned(::std::convert::Into::<::windows::IUnknown>::into(
            ::std::clone::Clone::clone(self),
        ))
    }
}
