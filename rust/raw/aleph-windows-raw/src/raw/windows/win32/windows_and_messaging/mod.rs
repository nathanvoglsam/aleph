#![allow(unused_variables, non_upper_case_globals, non_snake_case)]
#[repr(C)]
#[allow(non_snake_case)]
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
pub struct HWND(pub isize);
impl HWND {}
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
impl ::std::cmp::PartialEq for HWND {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::std::cmp::Eq for HWND {}
unsafe impl ::windows::Abi for HWND {
    type Abi = Self;
}
