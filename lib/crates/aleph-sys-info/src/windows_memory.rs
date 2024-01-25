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

use once_cell::sync::Lazy;

///
/// Only need to look this up once so wrap it in a lazy static
///
#[allow(clippy::identity_op)]
pub static SYSTEM_MEMORY: Lazy<Option<NonZeroU64>> = Lazy::new(get_memory);

#[cfg(target_vendor = "pc")]
fn get_memory() -> Option<NonZeroU64> {
    use aleph_windows::Win32::System::SystemInformation::GetPhysicallyInstalledSystemMemory;

    unsafe {
        let mut memory = 0;
        if GetPhysicallyInstalledSystemMemory(&mut memory) != false {
            NonZeroU64::new(memory * 1024)
        } else {
            None
        }
    }
}

#[cfg(target_vendor = "uwp")]
fn get_memory() -> Option<NonZeroU64> {
    use aleph_windows::Win32::System::SystemInformation::GetPhysicallyInstalledSystemMemory;

    unsafe {
        let mut memory = 0;
        if GetPhysicallyInstalledSystemMemory(&mut memory) != false {
            NonZeroU64::new(memory * 1024)
        } else {
            use aleph_windows::System::MemoryManager;
            MemoryManager::AppMemoryUsageLimit()
                .ok()
                .map(NonZeroU64::new)
                .flatten()
        }
    }
}
