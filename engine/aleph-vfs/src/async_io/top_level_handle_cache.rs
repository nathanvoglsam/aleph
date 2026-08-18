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

use std::fs::File;
use std::io;
use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex};

use aleph_alloc::BHashMap;
use aleph_alloc::instrumentation::IAllocationCategory;

use crate::async_io::{AsyncIo, AsyncIoSystem};

/// A global, send+sync file handle cache.
///
/// This is, effectively, a `Mutex<HashMap>` that maps `Path -> File`.
///
/// # Threads
///
/// Each cache is configured with a number of `threads`. The cache will create `threads` unique
/// file handles for each path used with the cache. Some operating systems serialize concurrent
/// IO done by separate threads to the same file handle. To avoid this we create a set of handles
/// instead and distribute threads across them to reduce (OS level) contention.
pub struct TopLevelHandleCache {
    /// The number of parallel file handles to open for each path. This is used so thread pools can
    /// avoid sharing the same handle on multiple threads. Some operating systems will serialize
    /// file IO to the same handle across threads.
    threads: usize,

    /// Maps a path to a set of file handles for that file.
    cache: Mutex<BHashMap<PathBuf, Arc<[File]>, AsyncIoSystem>>,
}

impl TopLevelHandleCache {
    /// Constructs a new [`TopLevelHandleCache`] configured with the given `threads` value.
    pub fn new(threads: usize) -> Arc<Self> {
        AsyncIo::with(|| {
            Arc::new(Self {
                threads,
                cache: Default::default(),
            })
        })
    }

    /// Returns the configured `threads` value that was provided when the cache was constructed.
    ///
    /// This will be the length of the `[File]` sets returned by this cache.
    pub const fn num_threads(&self) -> usize {
        self.threads
    }

    /// Fetch an existing entry, or create a new one if it's missing, for the given 'path'.
    ///
    /// The first time this is called the cache will open `threads` file handles for the file. This
    /// call will block.
    pub fn get_or_open(&self, path: impl AsRef<Path>) -> Result<Arc<[File]>, io::Error> {
        self.__get_or_open(path.as_ref())
    }

    fn __get_or_open(&self, path: &Path) -> Result<Arc<[File]>, io::Error> {
        let cache = self.cache.lock().map_err(|_| {
            io::Error::new(
                io::ErrorKind::BrokenPipe,
                "The top level cache mutex was poisoned.",
            )
        })?;
        if let Some(existing) = cache.get(path) {
            Ok(existing.clone())
        } else {
            // Release the lock so we don't block other threads while we open the file handles.
            drop(cache);
            let handles = AsyncIo::with(|| -> Result<Arc<[File]>, io::Error> {
                let mut handles = Vec::with_capacity(self.threads);
                for _ in 0..self.threads {
                    let file = File::options().read(true).open(path)?;
                    let metadata = file.metadata()?;
                    if !metadata.is_file() {
                        return Err(io::Error::new(
                            io::ErrorKind::IsADirectory,
                            "TopLevelHandleCache can only open files.",
                        ));
                    }
                    handles.push(file);
                }

                let handles = handles.into_boxed_slice();
                Ok(handles.into())
            })?;

            let path_buf = AsyncIo::with(|| path.to_path_buf());

            // We don't care if there was an existing entry. All that would mean is that there was
            // another thread racing to create the file handles. We don't want to block other cached
            // queries while we do the slow part (opening file handles).
            //
            // This means we can do wasted work instead, where we open file handles while racing
            // and immediately close them. The reduced contention is more important than perfect
            // compute efficiency.
            let mut cache = self.cache.lock().map_err(|_| {
                io::Error::new(
                    io::ErrorKind::BrokenPipe,
                    "The top level cache mutex was poisoned.",
                )
            })?;
            let _ = cache.insert(path_buf, handles.clone());
            Ok(handles)
        }
    }
}
