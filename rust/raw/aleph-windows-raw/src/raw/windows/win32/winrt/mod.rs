#![allow(unused_variables, non_upper_case_globals, non_snake_case)]
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
    pub unsafe extern "system" fn(),
    pub unsafe extern "system" fn(),
);
