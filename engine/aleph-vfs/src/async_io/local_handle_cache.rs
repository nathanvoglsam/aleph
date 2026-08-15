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

use std::cell::RefCell;
use std::fs::File;
use std::io;
use std::path::{Path, PathBuf};
use std::sync::Arc;

use aleph_alloc::BHashMap;
use aleph_alloc::instrumentation::IAllocationCategory;

use crate::async_io::top_level_handle_cache::TopLevelHandleCache;
use crate::async_io::{AsyncIo, AsyncIoSystem};

/// A bottom level cache, and sibling of [`TopLevelHandleCache`]. It exposes a similar interface.
///
/// `LocalHandleCache` does not own the [`File`] handles itself. It defers to the top level cache
/// to create the files. If the local cache is missing an entry then it fetches from the top level
/// cache. When fetching from the top level cache then we create a locally cached copy too so we
/// can serve future requests from the local cache instead.
///
/// This avoids contending the top level cache's internal mutex constantly. This bottom level cache
/// is `!Send` and `!Sync` and uses a [`RefCell`] internally.
pub struct LocalHandleCache {
    top_level: Arc<TopLevelHandleCache>,
    cache: RefCell<BHashMap<PathBuf, Arc<[File]>, AsyncIoSystem>>,
}

impl LocalHandleCache {
    /// Constructs a new [`LocalHandleCache`] that is a client of the given top level cache.
    pub fn new(top_level: Arc<TopLevelHandleCache>) -> Self {
        Self {
            top_level,
            cache: RefCell::new(Default::default()),
        }
    }

    /// Returns the configured `threads` value that was provided when the parent level cache was
    /// constructed.
    ///
    /// This will be the length of the `[File]` sets returned by this cache.
    pub fn num_threads(&self) -> usize {
        self.top_level.num_threads()
    }

    /// Fetch an existing entry, or create a new one if it's missing, for the given 'path'.
    ///
    /// The first time this is called the cache will open `threads` file handles for the file. This
    /// call will block.
    pub fn get_or_open(&self, path: impl AsRef<Path>) -> Result<Arc<[File]>, io::Error> {
        self.__get_or_open(path.as_ref())
    }

    fn __get_or_open(&self, path: &Path) -> Result<Arc<[File]>, io::Error> {
        let mut cache = self.cache.borrow_mut();

        if let Some(existing) = cache.get(path) {
            return Ok(existing.clone());
        }

        let existing = self.top_level.get_or_open(path)?;

        let path_buf = AsyncIo::with(|| path.to_path_buf());
        cache.insert(path_buf, existing.clone());

        Ok(existing)
    }
}
