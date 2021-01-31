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

use erupt::utils::loading::EntryLoaderError;
use std::sync::Arc;

///
/// Wrapper around the erupt API for dynamically loading the vulkan library that plays nicer with
/// the patterns set by other wrappers in this crate
///
pub struct Entry {
    entry_loader: erupt::DefaultEntryLoader,
}

impl Entry {
    pub fn new() -> Result<Arc<Self>, EntryLoaderError> {
        aleph_log::trace!("Initializing Vulkan Entry Loader");
        let entry_loader = erupt::DefaultEntryLoader::new()?;
        let out = Self { entry_loader };
        Ok(Arc::new(out))
    }
}

impl Entry {
    ///
    /// Get a reference to the ref-counted loader instance
    ///
    pub fn loader(&self) -> &erupt::DefaultEntryLoader {
        &self.entry_loader
    }
}
