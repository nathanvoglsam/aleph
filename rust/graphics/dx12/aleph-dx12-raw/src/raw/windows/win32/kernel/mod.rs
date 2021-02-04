#[repr(C)]
#[allow(non_camel_case_types)]
pub struct LUID {
    pub low_part: u32,
    pub high_part: i32,
}
#[repr(C)]
#[doc(hidden)]
#[allow(non_camel_case_types)]
pub struct LUID_abi(u32, i32);
impl LUID {}
unsafe impl ::windows::Abi for LUID {
    type Abi = LUID_abi;
}
impl ::core::default::Default for LUID {
    fn default() -> Self {
        Self {
            low_part: 0,
            high_part: 0,
        }
    }
}
impl ::core::fmt::Debug for LUID {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("LUID")
            .field("low_part", &format_args!("{:?}", self.low_part))
            .field("high_part", &format_args!("{:?}", self.high_part))
            .finish()
    }
}
impl ::core::clone::Clone for LUID {
    fn clone(&self) -> Self {
        Self {
            low_part: self.low_part,
            high_part: self.high_part,
        }
    }
}
impl ::std::marker::Copy for LUID {}
