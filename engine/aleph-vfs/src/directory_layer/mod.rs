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

mod async_file;
mod sync_file;

use std::io;
use std::path::Path;
use std::sync::Arc;

use aleph_alloc::BBox;
use aleph_alloc::instrumentation::IAllocationCategory;
use aleph_gen_arena::HandleType;
use aleph_io_queue::IoQueue;
use aleph_io_queue::channel::OpenChannel;
use camino::Utf8PathBuf;
use smallbox::SmallBox;

use crate::directory_layer::async_file::{AsyncVFile, RemapOpenSender};
use crate::directory_layer::sync_file::{POOL, PooledFile, VTABLE};
use crate::file::{IAsyncVFile, VFile};
use crate::path::{Component, VPath};
use crate::{ILayer, Vfs, VfsSystem, box_layer};

struct LocalVFile;
aleph_gen_arena::make_handle_id!(LocalVFile);

pub struct DirectoryLayer {
    /// The directory in the filesystem that this layer is mounting into the vfs.
    mounted_path: Utf8PathBuf,

    /// The [`IoQueue`] to push async file requests onto.
    io_queue: Option<Arc<IoQueue>>,
}

impl DirectoryLayer {
    pub fn new(mounted_path: Utf8PathBuf) -> BBox<dyn ILayer, VfsSystem> {
        box_layer(Self {
            mounted_path,
            io_queue: None,
        })
    }

    pub fn new_with_io_queue(
        mounted_path: Utf8PathBuf,
        io_queue: Arc<IoQueue>,
    ) -> BBox<dyn ILayer, VfsSystem> {
        box_layer(Self {
            mounted_path,
            io_queue: Some(io_queue),
        })
    }

    fn sanitize_and_translate_path(&self, path: &VPath) -> io::Result<Utf8PathBuf> {
        // clone and pre-reserve space for the path + some slop for redundant characters
        let mut combined = self.mounted_path.clone();
        combined.reserve(path.len() + 8);

        // Push the path in
        for component in path.components() {
            match component {
                // We just skip a root segment, an absolute path here is absolute _within_ the
                // layer mount.
                //
                // If we don't get a root component at all we're okay because relative paths
                // within this call are relative to the layer mount's root, not the outer vfs
                // root.
                Component::Root => continue,
                Component::Segment(seg) => {
                    // We intentionally skip ".." components. No good can come from trying to
                    // handle '..'. It's not a sane name for a file, so they only real time
                    // you'll see this is either a bug or someone intentionally trying to break
                    // things.
                    //
                    // We skip "." components too because they are likely to cause problems too.
                    if matches!(seg, ".." | ".") {
                        return Err(io::Error::new(
                            io::ErrorKind::InvalidFilename,
                            "Path contains '..' | '.' segments.",
                        ));
                    }
                    combined.push(seg);
                }
            }
        }
        Ok(combined)
    }
}

impl ILayer for DirectoryLayer {
    fn install(&mut self, _mount_name: &str) -> io::Result<()> {
        // We want a fully normalized, canonical path for 'mounted_path' to remove all ambiguity
        // when resolving.
        self.mounted_path = self.mounted_path.canonicalize_utf8()?;

        // Make sure that it's not a file too
        let metadata = self.mounted_path.metadata()?;
        if metadata.is_file() {
            return Err(io::Error::new(
                io::ErrorKind::NotADirectory,
                "Can't mount files as a DirectoryLayer",
            ));
        }
        assert!(metadata.is_dir());

        Ok(())
    }

    fn query_entity(&self, path: &VPath) -> io::Result<VFile<'_>> {
        let file = Vfs::with(|| -> io::Result<_> {
            let combined = self.sanitize_and_translate_path(path)?;

            // First we open a standard file handle for synchronous file IO
            let file = std::fs::OpenOptions::new()
                .read(true)
                .write(false)
                .create(false)
                .open(&combined)?;

            Ok(file)
        })?;

        let metadata = file.metadata()?;

        if !metadata.is_file() {
            return Err(io::Error::new(
                io::ErrorKind::IsADirectory,
                "DirectoryLayer can only open files.",
            ));
        }

        let handle = POOL.with_borrow_mut(|pool| {
            pool.alloc(PooledFile {
                file,
                len: metadata.len(),
            })
        });
        let handle = handle.to_bare_handle().into_int();

        let out = VFile {
            handle,
            vtable: &VTABLE,
            _no_send: Default::default(),
            _vfs: Default::default(),
        };

        Ok(out)
    }

    fn sync_query_entity_async_io(&self, path: &VPath) -> io::Result<Arc<dyn IAsyncVFile>> {
        let io_queue = match self.io_queue.as_ref() {
            Some(io_queue) => io_queue,
            None => {
                return Err(io::Error::new(
                    io::ErrorKind::Unsupported,
                    "Can't open async io for a directory layer with no async io queue.",
                ));
            }
        };
        let file = Vfs::with(|| -> io::Result<_> {
            let combined = self.sanitize_and_translate_path(path)?;

            let path: Arc<VPath> = {
                let arc: Arc<str> = Arc::from(path.to_str());
                unsafe { Arc::from_raw(Arc::into_raw(arc) as *const VPath) }
            };

            // Prime the handle cache, or error out if we failed to open the file.
            io_queue.open(combined.as_std_path())?;

            let out = AsyncVFile {
                queue: io_queue.clone(),
                virtual_path: path,
                path: Arc::from(combined.into_std_path_buf()),
            };
            let out = Arc::new(out);

            Ok(out)
        })?;

        Ok(file)
    }

    fn async_query_entity_async_io(
        &self,
        sender: SmallBox<dyn OpenChannel<Arc<dyn IAsyncVFile>>, [u128; 1]>,
        path: &VPath,
        opaque: u64,
    ) -> io::Result<()> {
        let io_queue = match self.io_queue.as_ref() {
            Some(io_queue) => io_queue,
            None => {
                return Err(io::Error::new(
                    io::ErrorKind::Unsupported,
                    "Can't open async io for a directory layer with no async io queue.",
                ));
            }
        };
        let _ = Vfs::with(|| -> io::Result<_> {
            let combined = self.sanitize_and_translate_path(path)?;

            let std_path: Arc<Path> = Arc::from(combined.as_std_path());

            let virtual_path: Arc<VPath> = {
                let arc: Arc<str> = Arc::from(path.to_str());
                unsafe { Arc::from_raw(Arc::into_raw(arc) as *const VPath) }
            };
            let file = AsyncVFile {
                queue: io_queue.clone(),
                virtual_path,
                path: Arc::from(combined.into_std_path_buf()),
            };
            let file = Arc::new(file);
            let remap_sender = RemapOpenSender { file, sender };

            // Dispatch a job onto the io queue to open the given file
            match io_queue.open_async(std_path, remap_sender, opaque) {
                Ok(_) => Ok(()),
                Err(_) => Err(io::Error::new(
                    io::ErrorKind::ConnectionAborted,
                    "The IoQueue workers have disconnected.",
                )),
            }
        })?;

        Ok(())
    }
}
