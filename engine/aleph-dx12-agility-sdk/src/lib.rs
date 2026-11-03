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

///
/// Macro that will define the `D3D12SDKVersion` and `D3D12SDKPath` symbols.
///
/// They will be defined with the following values
///
/// - `D3D12SDKVersion` = 618
///     - This is the version identifier of the SDK version bundled with this crate
/// - `D3D12SDKPath` = ".\"
///     - This allows 'D3D12Core.dll' to be next to the app executable and is compatible with the
///       results of the [extract_agility_sdk_binaries] function, which will copy the SDK binaries
///       into the 'target' directory next to the output .exe file.
///
/// To use custom values declare the symbols manually like this:
///
/// ```ignore
/// #[used]
/// #[no_mangle]
/// #[allow(non_upper_case_globals)]
/// /// Replace 618 with your minimum required SDK version
/// pub static D3D12SDKVersion: u32 = 618;
///
/// #[used]
/// #[no_mangle]
/// #[allow(non_upper_case_globals)]
/// /// Replace b".\\\0" with your directory to search for D3D12Core.dll
/// pub static D3D12SDKPath: &[u8; 9] = b".\\D3D12\\\0";
/// ```
///
/// See the D3D12 Agility SDK documentation for information on what these values mean and can be
/// set to.
///
#[macro_export]
macro_rules! export_standard_agility_sdk_symbols {
    () => {
        #[used]
        #[unsafe(no_mangle)]
        #[allow(non_upper_case_globals)]
        pub static D3D12SDKVersion: u32 = 618;

        #[used]
        #[unsafe(no_mangle)]
        #[allow(non_upper_case_globals)]
        pub static D3D12SDKPath: &[u8; 9] = b".\\D3D12\\\0";
    };
}
