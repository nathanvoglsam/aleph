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

#[cfg(any(target_os = "macos", target_os = "ios"))]
mod apple {
    pub fn get_allocated_bytes() -> usize {
        use std::mem::MaybeUninit;
        use std::ptr::null_mut;

        unsafe {
            let mut stats = MaybeUninit::uninit();
            libc::malloc_zone_statistics(null_mut(), stats.as_mut_ptr());
            let stats = stats.assume_init();
            stats.size_in_use
        }
    }
}

#[cfg(target_os = "linux")]
mod linux {
    pub fn get_allocated_bytes() -> usize {
        unsafe {
            let stats = libc::mallinfo2();
            stats.uordblks
        }
    }
}

#[cfg(target_os = "android")]
mod android {
    pub fn get_allocated_bytes() -> usize {
        unsafe {
            let stats = libc::mallinfo();
            stats.uordblks
        }
    }
}

#[cfg(target_os = "windows")]
mod windows {
    pub fn get_allocated_bytes() -> usize {
        use aleph_windows::Win32::Foundation::TRUE;
        use aleph_windows::Win32::System::Memory::GetProcessHeap;
        use aleph_windows::Win32::System::Memory::HeapSummary;
        use aleph_windows::Win32::System::Memory::HEAP_SUMMARY;

        unsafe {
            let heap = GetProcessHeap().unwrap();

            let mut stats = HEAP_SUMMARY {
                cb: size_of::<HEAP_SUMMARY>() as u32,
                cbAllocated: 0,
                cbCommitted: 0,
                cbReserved: 0,
                cbMaxReserve: 0,
            };
            let success = HeapSummary(heap, 0, &mut stats);

            if success == TRUE {
                stats.cbAllocated
            } else {
                panic!("Failed call to 'HeapSummary'");
            }
        }
    }
}

pub fn get_allocated_bytes() -> usize {
    #[cfg(any(target_os = "macos", target_os = "ios"))]
    return apple::get_allocated_bytes();

    #[cfg(target_os = "linux")]
    return unix::get_allocated_bytes();

    #[cfg(target_os = "android")]
    return android::get_allocated_bytes();

    #[cfg(target_os = "windows")]
    return windows::get_allocated_bytes();

    #[allow(unreachable_code)]
    {
        unreachable!()
    }
}

#[cfg(test)]
mod tests {
    use crate::get_allocated_bytes;

    #[test]
    pub fn get_test() {
        let bytes = get_allocated_bytes();
        println!("Got: {bytes} bytes!");
    }
}
