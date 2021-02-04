#[repr(C)]
#[allow(non_camel_case_types)]
pub struct HDC(pub isize);
#[repr(C)]
#[doc(hidden)]
#[allow(non_camel_case_types)]
pub struct HDC_abi(isize);
impl HDC {}
unsafe impl ::windows::Abi for HDC {
    type Abi = HDC_abi;
}
impl ::core::default::Default for HDC {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for HDC {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("HDC")
            .field("value", &format_args!("{:?}", self.0))
            .finish()
    }
}
impl ::core::clone::Clone for HDC {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}
