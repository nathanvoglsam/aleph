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

use std::num::NonZeroU64;

/// sys_info module is only loaded for non-windows or aarch64 as CPUID isn't available
#[cfg(any(not(windows), target_arch = "aarch64"))]
mod sys_info;

/// We load the CPU info directly from CPUID on x86_64 as it's the simplest way
#[cfg(target_arch = "x86_64")]
mod raw_cpu_info;

#[cfg(windows)]
mod windows_memory;

#[cfg(unix)]
mod unix_memory;

mod null_memory;

///
/// Gets the vendor string for the current CPU
///
/// # Warning
///
/// At the moment this only works on x86 and x86_64. Otherwise it will just return an "Unknown CPU
/// Vendor" string.
///
#[allow(unreachable_code)]
pub fn cpu_vendor() -> &'static str {
    #[cfg(target_arch = "x86_64")]
    {
        return raw_cpu_info::CPU_VENDOR_STRING.as_str();
    }

    #[cfg(target_arch = "aarch64")]
    {
        return sys_info::SYSTEM_INFO.global_cpu_info().vendor_id();
    }

    unimplemented!()
}

///
/// Gets the brand string for the current CPU
///
/// # Warning
///
/// At the moment this only works on x86 and x86_64 that support an extended part of the __cpuid
/// instruction. Otherwise it will just return an "Unknown CPU" string.
///
#[allow(unreachable_code)]
pub fn cpu_brand() -> &'static str {
    #[cfg(target_arch = "x86_64")]
    {
        return raw_cpu_info::CPU_BRAND_STRING.as_str();
    }

    #[cfg(target_arch = "aarch64")]
    {
        return sys_info::SYSTEM_INFO.global_cpu_info().brand();
    }

    unimplemented!()
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
#[allow(unreachable_code)]
pub fn installed_memory() -> Option<NonZeroU64> {
    #[cfg(windows)]
    return *windows_memory::SYSTEM_MEMORY;

    #[cfg(unix)]
    return *unix_memory::SYSTEM_MEMORY;

    *null_memory::SYSTEM_MEMORY
}
