//
//
// This file is a part of Aleph
//
// https://github.com/nathanvoglsam/aleph
//
// MIT License
//
// Copyright (c) 2020 Aleph Engine
//
// Permission is hereby granted, free of charge, to any person obtaining a copy
// of this software and associated documentation files (the "Software"), to deal
// in the Software without restriction, including without limitation the rights
// to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
// copies of the Software, and to permit persons to whom the Software is
// furnished to do so, subject to the following conditions:
//
// The above copyright notice and this permission notice shall be included in all
// copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
// AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
// LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
// OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
// SOFTWARE.
//

use once_cell::sync::Lazy;

///
/// Only need to look this up once so wrap it in a lazy static
///
#[allow(clippy::identity_op)]
static CPU_VENDOR_STRING: Lazy<String> = Lazy::new(|| {
    let result = raw_cpuid::CpuId::new();
    match result.get_vendor_info() {
        None => String::from("Unknown CPU Vendor"),
        Some(vendor) => vendor.as_string().to_string(),
    }
});

///
/// Only need to look this up once so wrap it in a lazy static
///
#[allow(clippy::identity_op)]
static CPU_BRAND_STRING: Lazy<String> = Lazy::new(|| {
    let result = raw_cpuid::CpuId::new();
    match result.get_extended_function_info() {
        None => "Unknown CPU".to_string(),
        Some(ext) => ext
            .processor_brand_string()
            .unwrap_or("Unknown CPU")
            .to_string(),
    }
});

///
/// Only need to look this up once so wrap it in a lazy static
///
#[allow(clippy::identity_op)]
static SYSTEM_MEMORY: Lazy<Option<u64>> = Lazy::new(get_memory);

#[cfg(target_os = "windows")]
fn get_memory() -> Option<u64> {
    use aleph_windows_raw::Win32::SystemServices::GetPhysicallyInstalledSystemMemory;

    unsafe {
        let mut memory = 0;
        if GetPhysicallyInstalledSystemMemory(&mut memory) != false {
            Some(memory * 1024)
        } else {
            None
        }
    }
}

#[cfg(target_os = "linux")]
fn get_memory() -> Option<u64> {
    unsafe {
        use std::mem::MaybeUninit;
        let mut sysinfo = MaybeUninit::uninit();

        if libc::sysinfo(sysinfo.as_mut_ptr()) == 0 {
            let sysinfo = sysinfo.assume_init();
            Some(sysinfo.totalram as u64)
        } else {
            None
        }
    }
}

#[cfg(not(any(target_os = "windows", target_os = "linux")))]
fn get_memory() -> Option<u64> {
    None
}

///
/// Gets the vendor string for the current CPU
///
/// # Warning
///
/// At the moment this only works on x86 and x86_64. Otherwise it will just return an "Unknown CPU
/// Vendor" string.
///
pub fn cpu_vendor() -> &'static str {
    CPU_VENDOR_STRING.as_str()
}

///
/// Gets the brand string for the current CPU
///
/// # Warning
///
/// At the moment this only works on x86 and x86_64 that support an extended part of the __cpuid
/// instruction. Otherwise it will just return an "Unknown CPU" string.
///
pub fn cpu_brand() -> &'static str {
    CPU_BRAND_STRING.as_str()
}

///
/// Returns the number of physical cores (non SMT cores) on the current host
///
pub fn physical_core_count() -> u64 {
    num_cpus::get_physical() as u64
}

///
/// Returns the number of logical cores (physical + SMT cores) on the current host
///
pub fn logical_core_count() -> u64 {
    num_cpus::get() as u64
}

///
/// Returns the amount of memory installed in the system in bytes. A `None` value indicates that
/// the information could not be retrieved successfully
///
pub fn installed_memory() -> Option<u64> {
    *SYSTEM_MEMORY
}
