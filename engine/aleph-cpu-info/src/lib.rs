//
//
// This file is a part of Aleph
//
// <ALEPH_REPO_REPLACE>
//
// <ALEPH_LICENSE_REPLACE>
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
pub fn physical_core_count() -> usize {
    num_cpus::get_physical()
}

///
/// Returns the number of logical cores (physical + SMT cores) on the current host
///
pub fn logical_core_count() -> usize {
    num_cpus::get()
}
