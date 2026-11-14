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
use sysinfo::{CpuRefreshKind, MemoryRefreshKind, RefreshKind, System};

///
/// Only need to look this up once so wrap it in a lazy static
///
#[allow(clippy::identity_op)]
pub static SYSTEM_INFO: Lazy<System> = Lazy::new(|| {
    let refreshes = RefreshKind::nothing()
        .with_cpu(CpuRefreshKind::everything())
        .with_memory(MemoryRefreshKind::nothing().with_ram());
    System::new_with_specifics(refreshes)
});

#[cfg(any(target_arch = "aarch64", test))]
pub(crate) fn cpu_vendor(system: &System) -> &str {
    cpu_info_or_unknown(
        system.cpus().first().map(|cpu| cpu.vendor_id()),
        "Unknown CPU Vendor",
    )
}

#[cfg(any(target_arch = "aarch64", test))]
pub(crate) fn cpu_brand(system: &System) -> &str {
    cpu_info_or_unknown(system.cpus().first().map(|cpu| cpu.brand()), "Unknown CPU")
}

#[cfg(any(target_arch = "aarch64", test))]
fn cpu_info_or_unknown<'a>(value: Option<&'a str>, unknown: &'static str) -> &'a str {
    value
        .filter(|value| !value.trim().is_empty())
        .unwrap_or(unknown)
}

#[cfg(test)]
mod tests {
    use sysinfo::System;

    use super::{cpu_brand, cpu_info_or_unknown, cpu_vendor};

    #[test]
    fn empty_cpu_list_reports_unknown() {
        let system = System::new();
        assert!(system.cpus().is_empty());
        assert_eq!(cpu_vendor(&system), "Unknown CPU Vendor");
        assert_eq!(cpu_brand(&system), "Unknown CPU");
    }

    #[test]
    fn cpu_info_reports_unknown_for_missing_or_empty_values() {
        assert_eq!(cpu_info_or_unknown(None, "Unknown"), "Unknown");
        assert_eq!(cpu_info_or_unknown(Some(""), "Unknown"), "Unknown");
        assert_eq!(cpu_info_or_unknown(Some("  "), "Unknown"), "Unknown");
        assert_eq!(
            cpu_info_or_unknown(Some("Known CPU"), "Unknown"),
            "Known CPU"
        );
    }
}
