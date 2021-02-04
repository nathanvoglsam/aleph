#[link(name = "KERNEL32")]
extern "system" {
    pub fn CloseHandle(h_object: super::system_services::HANDLE) -> ::windows::BOOL;
}
