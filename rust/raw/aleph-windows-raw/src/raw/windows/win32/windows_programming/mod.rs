#![allow(unused_variables, non_upper_case_globals, non_snake_case)]
#[repr(C)]
#[allow(non_snake_case)]
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
pub struct FILETIME {
    pub dw_low_date_time: u32,
    pub dw_high_date_time: u32,
}
impl FILETIME {}
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
impl ::std::cmp::PartialEq for FILETIME {
    fn eq(&self, other: &Self) -> bool {
        self.dw_low_date_time == other.dw_low_date_time
            && self.dw_high_date_time == other.dw_high_date_time
    }
}
impl ::std::cmp::Eq for FILETIME {}
unsafe impl ::windows::Abi for FILETIME {
    type Abi = Self;
}
pub unsafe fn CloseHandle<'a, T0__: ::windows::IntoParam<'a, super::system_services::HANDLE>>(
    h_object: T0__,
) -> super::system_services::BOOL {
    #[link(name = "KERNEL32")]
    extern "system" {
        pub fn CloseHandle(
            h_object: super::system_services::HANDLE,
        ) -> super::system_services::BOOL;
    }
    CloseHandle(h_object.into_param().abi())
}
