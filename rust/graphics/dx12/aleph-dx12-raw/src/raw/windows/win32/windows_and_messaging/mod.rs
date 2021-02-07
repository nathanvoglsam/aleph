#[repr(C)]
#[allow(non_camel_case_types)]
pub struct HWND(pub isize);
#[repr(C)]
#[doc(hidden)]
#[allow(non_camel_case_types)]
pub struct HWND_abi(isize);
impl HWND {}
unsafe impl ::windows::Abi for HWND {
    type Abi = HWND_abi;
}
impl ::std::default::Default for HWND {
    fn default() -> Self {
        Self(0)
    }
}
impl ::std::fmt::Debug for HWND {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("HWND")
            .field("value", &format_args!("{:?}", self.0))
            .finish()
    }
}
impl ::std::clone::Clone for HWND {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}
