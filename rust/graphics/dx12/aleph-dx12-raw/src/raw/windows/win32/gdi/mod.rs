#[repr(C)]
#[allow(non_snake_case)]
pub struct HDC(pub isize);
#[repr(C)]
#[doc(hidden)]
pub struct HDC_abi(isize);
unsafe impl ::windows::Abi for HDC {
    type Abi = HDC_abi;
}
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
impl ::std::clone::Clone for HDC {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}
impl ::std::cmp::PartialEq for HDC {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::std::cmp::Eq for HDC {}
impl ::std::marker::Copy for HDC {}
