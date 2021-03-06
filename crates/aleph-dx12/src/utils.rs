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

#[macro_export]
macro_rules! device_child_impl {
    ($t:ident) => {
        impl $crate::D3D12DeviceChild for $t {
            #[inline]
            unsafe fn get_device(&self) -> $crate::Result<$crate::Device> {
                use windows_raw::{Abi, Interface};
                type D = $crate::windows_raw::Win32::Direct3D12::ID3D12Device4;
                let mut device: Option<D> = None;
                self.0
                    .GetDevice(&D::IID, device.set_abi())
                    .and_some(device)
                    .map(|v| $crate::Device(v))
            }
        }
    };
}

#[macro_export]
macro_rules! object_impl {
    ($t:ident) => {
        impl $crate::D3D12Object for $t {
            #[inline]
            unsafe fn set_name_raw(&self, name: &[u16]) -> $crate::Result<()> {
                use $crate::windows_raw::Win32::SystemServices::PWSTR;
                self.0.SetName(PWSTR(name.as_ptr() as *mut u16)).ok()
            }
        }
    };
}

#[macro_export]
macro_rules! owned_object {
    ($t:ident) => {
        unsafe impl Send for $t {}
    };
}

#[macro_export]
macro_rules! shared_object {
    ($t:ident) => {
        impl ::core::clone::Clone for $t {
            #[inline]
            fn clone(&self) -> Self {
                Self(self.0.clone())
            }
        }
        unsafe impl Send for $t {}
        unsafe impl Sync for $t {}
    };
}
