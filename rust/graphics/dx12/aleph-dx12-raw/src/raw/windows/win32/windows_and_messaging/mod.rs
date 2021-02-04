#[repr(C)]
#[allow(non_snake_case)]
pub struct HWND(pub isize);
#[repr(C)]
#[doc(hidden)]
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
impl ::std::cmp::PartialEq for HWND {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::std::cmp::Eq for HWND {}
