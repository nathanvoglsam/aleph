#![allow(unused_variables, non_upper_case_globals, non_snake_case)]
#[repr(C)]
#[allow(non_snake_case)]
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
pub struct HSTRING(pub isize);
impl HSTRING {}
impl ::std::default::Default for HSTRING {
    fn default() -> Self {
        Self(0)
    }
}
impl ::std::fmt::Debug for HSTRING {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("HSTRING")
            .field("value", &format_args!("{:?}", self.0))
            .finish()
    }
}
impl ::std::cmp::PartialEq for HSTRING {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::std::cmp::Eq for HSTRING {}
unsafe impl ::windows::Abi for HSTRING {
    type Abi = Self;
}
#[allow(non_camel_case_types)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: marker :: Copy,
    :: std :: clone :: Clone,
    :: std :: default :: Default,
    :: std :: fmt :: Debug,
)]
#[repr(transparent)]
pub struct TrustLevel(pub i32);
impl TrustLevel {
    #![allow(non_upper_case_globals)]
    pub const BaseTrust: Self = Self(0i32);
    pub const PartialTrust: Self = Self(1i32);
    pub const FullTrust: Self = Self(2i32);
}
impl ::std::convert::From<i32> for TrustLevel {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::Abi for TrustLevel {
    type Abi = Self;
}
#[repr(transparent)]
#[allow(non_camel_case_types)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct IInspectable(::windows::IUnknown);
impl IInspectable {}
unsafe impl ::windows::Interface for IInspectable {
    type Vtable = IInspectable_abi;
    const IID: ::windows::Guid = ::windows::Guid::from_values(
        2944852704,
        45357,
        19562,
        [156, 90, 215, 170, 101, 16, 30, 144],
    );
}
#[allow(non_snake_case)]
impl IInspectable {
    pub unsafe fn GetIids(
        &self,
        iid_count: *mut u32,
        iids: *mut *mut ::windows::Guid,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).3)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(iid_count),
            ::std::mem::transmute(iids),
        )
    }
    pub unsafe fn GetRuntimeClassName(&self, class_name: *mut HSTRING) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).4)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(class_name),
        )
    }
    pub unsafe fn GetTrustLevel(&self, trust_level: *mut TrustLevel) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).5)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(trust_level),
        )
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
impl<'a> ::windows::IntoParam<'a, ::windows::IUnknown> for IInspectable {
    fn into_param(self) -> ::windows::Param<'a, ::windows::IUnknown> {
        ::windows::Param::Owned(::std::convert::Into::<::windows::IUnknown>::into(self))
    }
}
impl<'a> ::windows::IntoParam<'a, ::windows::IUnknown> for &'a IInspectable {
    fn into_param(self) -> ::windows::Param<'a, ::windows::IUnknown> {
        ::windows::Param::Owned(::std::convert::Into::<::windows::IUnknown>::into(
            ::std::clone::Clone::clone(self),
        ))
    }
}
#[repr(C)]
#[doc(hidden)]
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
        class_name: *mut HSTRING,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        trust_level: *mut TrustLevel,
    ) -> ::windows::ErrorCode,
);
