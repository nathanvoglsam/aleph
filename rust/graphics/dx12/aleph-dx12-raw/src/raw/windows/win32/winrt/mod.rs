#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct TrustLevel(pub i32);
impl ::std::convert::From<i32> for TrustLevel {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
impl ::std::clone::Clone for TrustLevel {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}
impl ::std::default::Default for TrustLevel {
    fn default() -> Self {
        Self(0)
    }
}
impl ::std::fmt::Debug for TrustLevel {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        write!(f, "{:?}", self.0)
    }
}
impl ::std::cmp::PartialEq for TrustLevel {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::std::cmp::Eq for TrustLevel {}
impl ::std::marker::Copy for TrustLevel {}
impl TrustLevel {
    #![allow(non_upper_case_globals)]
    pub const BaseTrust: Self = Self(0i32);
    pub const PartialTrust: Self = Self(1i32);
    pub const FullTrust: Self = Self(2i32);
}
unsafe impl ::windows::Abi for TrustLevel {
    type Abi = Self;
}
#[repr(transparent)]
#[allow(non_camel_case_types)]
pub struct IInspectable(::windows::IUnknown);
impl ::std::clone::Clone for IInspectable {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::std::fmt::Debug for IInspectable {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        write!(f, "{:?}", self.0)
    }
}
impl ::std::cmp::PartialEq for IInspectable {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::std::cmp::Eq for IInspectable {}
unsafe impl ::windows::Interface for IInspectable {
    type Vtable = IInspectable_abi;
    const IID: ::windows::Guid = ::windows::Guid::from_values(
        2944852704,
        45357,
        19562,
        [156, 90, 215, 170, 101, 16, 30, 144],
    );
}
#[repr(C)]
pub struct IInspectable_abi(
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        iid: &::windows::Guid,
        interface: *mut ::windows::RawPtr,
    ) -> ::windows::ErrorCode,
    pub unsafe extern "system" fn(this: ::windows::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::RawPtr) -> u32,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        iid_count: *mut u32,
        iids: *mut *mut ::windows::Guid,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        class_name: *mut isize,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        trust_level: *mut TrustLevel,
    ) -> ::windows::ErrorCode,
);
#[allow(non_snake_case)]
impl IInspectable {
    pub unsafe fn GetIids(
        &self,
        iid_count: *mut u32,
        iids: *mut *mut ::windows::Guid,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).3)(::windows::Abi::abi(self), iid_count, iids)
    }
    pub unsafe fn GetRuntimeClassName(&self, class_name: *mut isize) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).4)(::windows::Abi::abi(self), class_name)
    }
    pub unsafe fn GetTrustLevel(&self, trust_level: *mut TrustLevel) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).5)(::windows::Abi::abi(self), trust_level)
    }
}
impl ::std::convert::From<IInspectable> for ::windows::IUnknown {
    fn from(value: IInspectable) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IInspectable> for ::windows::IUnknown {
    fn from(value: &IInspectable) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::std::convert::Into<::windows::Param<'a, ::windows::IUnknown>> for IInspectable {
    fn into(self) -> ::windows::Param<'a, ::windows::IUnknown> {
        ::windows::Param::Owned(::std::convert::Into::<::windows::IUnknown>::into(self))
    }
}
impl<'a> ::std::convert::Into<::windows::Param<'a, ::windows::IUnknown>> for &'a IInspectable {
    fn into(self) -> ::windows::Param<'a, ::windows::IUnknown> {
        ::windows::Param::Owned(::std::convert::Into::<::windows::IUnknown>::into(
            ::std::clone::Clone::clone(self),
        ))
    }
}
