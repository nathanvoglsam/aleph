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

/// Represents the `ID3D12Object` interface mapped to a rust trait
pub trait D3D12Object {
    /// A simple function that takes a native rust string. This will allocate to convert it into a
    /// UTF16 string to pass to the underlying d3d12 API
    fn set_name(&self, name: &str) -> crate::Result<()> {
        unsafe {
            let utf16: Vec<u16> = name.encode_utf16().chain(std::iter::once(0)).collect();
            self.set_name_raw(&utf16)
        }
    }

    /// A lower level version of `set_name` which allows for manually supplying a null terminated
    /// UTF16 string
    unsafe fn set_name_raw(&self, name: &[u16]) -> crate::Result<()>;
}

/// Represents the `ID3D12DeviceChild` interface mapped to a rust trait
pub trait D3D12DeviceChild {
    unsafe fn get_device(&self) -> crate::Result<crate::Device>;
}
