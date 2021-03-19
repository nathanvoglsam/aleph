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

use any::*;
use std::ffi::{CStr, CString};

///
/// This interface should be used by plugins that wish to register themselves as the engine's
/// clipboard provider. Anything that implements this should correctly handle creating and
/// destroying whatever is needed to access the system's clipboard, and should be able to give out
/// an `AnyArc<IClipboard>` to allow others to retrieve information about and manipulate the
/// clipboard.
///
pub trait IClipboardProvider: IAny + 'static {
    ///
    /// Returns an `AnyArc` that holds an `IClipboard` interface.
    ///
    /// This will always return the same `IClipboard` instance as `IClipboardProvider` only supports
    /// handling a single mouse device.
    ///
    /// A return value of `None` should signal that the functionality is not supported.
    ///
    fn get_clipboard(&self) -> Option<AnyArc<dyn IClipboard>>;
}

///
/// This interface represents the API expected of something that gives the engine access to a
/// device's clipboard.
///
pub trait IClipboard: IAny + Send + Sync + 'static {
    ///
    /// Gets the current clipboard text, if there is some.
    ///
    fn get(&self) -> Option<String>;

    ///
    /// Gets the current clipboard text, if there is some. This will still allocate, but the null
    /// terminator will be preserved in case it is needed.
    ///
    fn get_null_terminated(&self) -> Option<CString>;

    ///
    /// Sets the current clipboard text
    ///
    fn set(&self, value: &str);

    ///
    /// Sets the current clipboard text with an already null terminated string.
    ///
    /// This could potentially save on an allocation if needed
    ///
    fn set_null_terminated(&self, value: &CStr);
}
