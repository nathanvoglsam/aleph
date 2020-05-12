//
//
// This file is a part of Aleph
//
// <ALEPH_REPO_REPLACE>
//
// <ALEPH_LICENSE_REPLACE>
//

#[cfg(target_arch = "x86_64")]
use core::arch::x86_64::*;

#[cfg(target_arch = "x86")]
use core::arch::x86::*;

use once_cell::sync::Lazy;
use std::ffi::CStr;
use std::os::raw::c_char;

///
/// Only need to look this up once so wrap it in a lazy static
///
static CPU_VENDOR_STRING: Lazy<String> = Lazy::new(|| {
    if cfg!(target_arch = "x86_64") || cfg!(target_arch = "x86") {
        unsafe {
            let mut data = Vec::new();

            let n_ids = __cpuid(0).eax;

            for i in 0..n_ids {
                data.push(__cpuid_count(i, 0));
            }

            let mut vendor_buffer = [0u8; 0x20];

            let vendor_ptr = vendor_buffer.as_mut_ptr() as *mut u32;
            vendor_ptr.add(0).write(data[0].ebx);
            vendor_ptr.add(1).write(data[0].edx);
            vendor_ptr.add(2).write(data[0].ecx);

            let vendor = CStr::from_ptr(vendor_buffer.as_ptr() as *const c_char);

            let vendor = vendor.to_str().expect("Vendor String not UTF8");
            let vendor = vendor.to_string();
            vendor
        }
    } else {
        String::from("Unknown CPU Vendor")
    }
});

///
/// Only need to look this up once so wrap it in a lazy static
///
static CPU_BRAND_STRING: Lazy<String> = Lazy::new(|| {
    if cfg!(target_arch = "x86_64") || cfg!(target_arch = "x86") {
        unsafe {
            let mut data = Vec::new();

            let n_ext_ids = __cpuid(0x80000000).eax;

            for i in 0x80000000..n_ext_ids {
                data.push(__cpuid_count(i, 0));
            }

            if n_ext_ids > 0x80000000 {
                let mut brand_buffer = [0u8; 0x40];

                let brand_ptr = brand_buffer.as_mut_ptr() as *mut u32;

                let offset = 0;
                brand_ptr.add((offset * 4) + 0).write(data[offset + 2].eax);
                brand_ptr.add((offset * 4) + 1).write(data[offset + 2].ebx);
                brand_ptr.add((offset * 4) + 2).write(data[offset + 2].ecx);
                brand_ptr.add((offset * 4) + 3).write(data[offset + 2].edx);

                let offset = 1;
                brand_ptr.add((offset * 4) + 0).write(data[offset + 2].eax);
                brand_ptr.add((offset * 4) + 1).write(data[offset + 2].ebx);
                brand_ptr.add((offset * 4) + 2).write(data[offset + 2].ecx);
                brand_ptr.add((offset * 4) + 3).write(data[offset + 2].edx);

                let offset = 2;
                brand_ptr.add((offset * 4) + 0).write(data[offset + 2].eax);
                brand_ptr.add((offset * 4) + 1).write(data[offset + 2].ebx);
                brand_ptr.add((offset * 4) + 2).write(data[offset + 2].ecx);
                brand_ptr.add((offset * 4) + 3).write(data[offset + 2].edx);

                let brand = CStr::from_ptr(brand_buffer.as_ptr() as *const c_char);

                let brand = brand.to_str().expect("brand String not UTF8");
                let brand = brand.trim();
                let brand = brand.to_string();
                brand
            } else {
                "CPU BRAND NOT SUPPORTED".to_string()
            }
        }
    } else {
        String::from("Unknown CPU Brand")
    }
});

///
/// Gets the vendor string for the current CPU
///
pub fn cpu_vendor() -> &'static str {
    CPU_VENDOR_STRING.as_str()
}

///
/// Gets the brand string for the current CPU
///
pub fn cpu_brand() -> &'static str {
    CPU_BRAND_STRING.as_str()
}
