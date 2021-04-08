#![allow(
    unused_variables,
    non_upper_case_globals,
    non_snake_case,
    unused_unsafe,
    non_camel_case_types,
    dead_code,
    clippy::all
)]
#[repr(C)]
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
pub struct LUID {
    pub LowPart: u32,
    pub HighPart: i32,
}
impl LUID {}
impl ::std::default::Default for LUID {
    fn default() -> Self {
        Self {
            LowPart: 0,
            HighPart: 0,
        }
    }
}
impl ::std::fmt::Debug for LUID {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("LUID")
            .field("LowPart", &format_args!("{:?}", self.LowPart))
            .field("HighPart", &format_args!("{:?}", self.HighPart))
            .finish()
    }
}
impl ::std::cmp::PartialEq for LUID {
    fn eq(&self, other: &Self) -> bool {
        self.LowPart == other.LowPart && self.HighPart == other.HighPart
    }
}
impl ::std::cmp::Eq for LUID {}
unsafe impl ::windows::Abi for LUID {
    type Abi = Self;
}
