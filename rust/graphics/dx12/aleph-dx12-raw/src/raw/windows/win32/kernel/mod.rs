#[repr(C)]
#[allow(non_snake_case)]
pub struct LUID {
    pub low_part: u32,
    pub high_part: i32,
}
#[repr(C)]
#[doc(hidden)]
pub struct LUID_abi(u32, i32);
unsafe impl ::windows::Abi for LUID {
    type Abi = LUID_abi;
}
impl ::std::default::Default for LUID {
    fn default() -> Self {
        Self {
            low_part: 0,
            high_part: 0,
        }
    }
}
impl ::std::fmt::Debug for LUID {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("LUID")
            .field("low_part", &format_args!("{:?}", self.low_part))
            .field("high_part", &format_args!("{:?}", self.high_part))
            .finish()
    }
}
impl ::std::clone::Clone for LUID {
    fn clone(&self) -> Self {
        Self {
            low_part: self.low_part,
            high_part: self.high_part,
        }
    }
}
impl ::std::cmp::PartialEq for LUID {
    fn eq(&self, other: &Self) -> bool {
        self.low_part == other.low_part && self.high_part == other.high_part
    }
}
impl ::std::cmp::Eq for LUID {}
