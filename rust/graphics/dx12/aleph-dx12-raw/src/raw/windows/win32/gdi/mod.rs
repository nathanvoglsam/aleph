#![allow(unused_variables, non_upper_case_globals, non_snake_case)]
#[repr(C)]
#[allow(non_snake_case)]
#[derive(:: std :: clone :: Clone)]
pub struct HMONITOR(pub isize);
impl HMONITOR {}
impl ::std::default::Default for HMONITOR {
    fn default() -> Self {
        Self(0)
    }
}
impl ::std::fmt::Debug for HMONITOR {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("HMONITOR")
            .field("value", &format_args!("{:?}", self.0))
            .finish()
    }
}
impl ::std::cmp::PartialEq for HMONITOR {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::std::cmp::Eq for HMONITOR {}
unsafe impl ::windows::Abi for HMONITOR {
    type Abi = Self;
}
impl ::std::marker::Copy for HMONITOR {}
#[repr(C)]
#[allow(non_snake_case)]
#[derive(:: std :: clone :: Clone)]
pub struct HDC(pub isize);
impl HDC {}
impl ::std::default::Default for HDC {
    fn default() -> Self {
        Self(0)
    }
}
impl ::std::fmt::Debug for HDC {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("HDC")
            .field("value", &format_args!("{:?}", self.0))
            .finish()
    }
}
impl ::std::cmp::PartialEq for HDC {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::std::cmp::Eq for HDC {}
unsafe impl ::windows::Abi for HDC {
    type Abi = Self;
}
impl ::std::marker::Copy for HDC {}
