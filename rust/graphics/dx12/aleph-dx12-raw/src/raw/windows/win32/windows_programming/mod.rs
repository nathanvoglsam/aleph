#[repr(C)]
#[allow(non_snake_case)]
pub struct FILETIME {
    pub dw_low_date_time: u32,
    pub dw_high_date_time: u32,
}
impl FILETIME {}
#[repr(C)]
#[doc(hidden)]
pub struct FILETIME_abi(u32, u32);
unsafe impl ::windows::Abi for FILETIME {
    type Abi = FILETIME_abi;
}
impl ::std::default::Default for FILETIME {
    fn default() -> Self {
        Self {
            dw_low_date_time: 0,
            dw_high_date_time: 0,
        }
    }
}
impl ::std::fmt::Debug for FILETIME {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("FILETIME")
            .field(
                "dw_low_date_time",
                &format_args!("{:?}", self.dw_low_date_time),
            )
            .field(
                "dw_high_date_time",
                &format_args!("{:?}", self.dw_high_date_time),
            )
            .finish()
    }
}
impl ::std::clone::Clone for FILETIME {
    fn clone(&self) -> Self {
        Self {
            dw_low_date_time: self.dw_low_date_time,
            dw_high_date_time: self.dw_high_date_time,
        }
    }
}
impl ::std::cmp::PartialEq for FILETIME {
    fn eq(&self, other: &Self) -> bool {
        self.dw_low_date_time == other.dw_low_date_time
            && self.dw_high_date_time == other.dw_high_date_time
    }
}
impl ::std::cmp::Eq for FILETIME {}
#[link(name = "KERNEL32")]
extern "system" {
    pub fn CloseHandle(h_object: super::system_services::HANDLE) -> ::windows::BOOL;
}
