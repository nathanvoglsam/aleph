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

use std::io;
use std::marker::PhantomData;
use std::num::NonZero;
use std::ptr::NonNull;
use std::sync::Arc;

use aleph_io_queue::channel::{LoadChannel, ReadChannel};
use crossbeam::channel::SendError;
use smallbox::{SmallBox, smallbox};

use crate::path::VPath;

/// Analogue of [`std::fs::File`]. Represents a 'handle' to an open file within a virtual file
/// system.
///
/// Exposes a substantially reduced interface, mostly for reading bytes from the file. A 'VFile' is
/// stateless and _does not_ expose a cursor like an OS file handle. [`VFileReader`] implements a
/// [`Read`] interface atop the 'VFile' interface.
///
/// # `!Send`
///
/// We deny sending 'VFile' handles to other threads. Some OS file system APIs will serialize
/// concurrent synchronous reads to a file (they need to maintain coherence of the cursor). We don't
/// expose a cursor on VFile, but some OS APIs don't have a concurrent read_at call. To prevent
/// serializing reads across threads we ensure each thread gets its own OS file handle.
///
/// This would be especially problematic for package based layers as all files within the package
/// would share the same file handle, so any parallel reads would be serialized on some operating
/// systems.
pub struct VFile<'vfs> {
    /// A handle used to access an open 'virtual' file.
    ///
    /// This only needs to be unique on the thread the file handle is created on. File handles are
    /// !Send so they can only be used on their owning thread.
    pub(crate) handle: NonZero<u64>,

    /// Table of function pointers, references the implementations of the functions used for the
    /// interface 'VFile' exposes.
    pub(crate) vtable: &'static VFileVtable,

    /// Needed to force !Send.
    pub(crate) _no_send: PhantomData<*const ()>,

    /// Attach the vfile to the vfs handle it was created from.
    pub(crate) _vfs: PhantomData<&'vfs ()>,
}

impl<'vfs> VFile<'vfs> {
    /// Get the internal, thread-local vfile handle that [`VFile`] closes over.
    pub const fn handle(&self) -> NonZero<u64> {
        self.handle
    }

    /// Construct a [`VFileReader`] to get a [`Read`] interface over this file.
    pub fn reader(&self) -> VFileReader<'_> {
        VFileReader {
            file: self,
            cursor: 0,
        }
    }

    /// Will read up to `offset` bytes from the file into `buf`, returning the number of bytes
    /// written. In general this will function like [`std::fs::File::read`], with the same behavior
    /// and error conditions. More specifically [`std::fs::File::read_at`] (unix) or
    /// [`std::fs::File::seek_read`] (windows) depending on your host OS.
    pub fn read_at(&self, buf: &mut [u8], offset: u64) -> io::Result<usize> {
        (self.vtable.read_at)(self.handle, buf, offset)
    }

    /// Gets the length of the file, in bytes.
    ///
    /// # Note
    ///
    /// Most implementations will _cache_ this on their internal file handle representation. In most
    /// practical use cases this will be constant, but if someone is being naughty and modifying
    /// the mounted files when they shouldn't then you're going to have a bad time. The interface
    /// remains safe, but the behavior is unspecified.
    pub fn len(&self) -> io::Result<u64> {
        (self.vtable.size)(self.handle)
    }
}

impl<'vfs> Drop for VFile<'vfs> {
    fn drop(&mut self) {
        (self.vtable.close)(self.handle)
    }
}

/// Implements [`Read`] and [`Seek`] over a [`VFile`] by tracking our own cursor internally.
pub struct VFileReader<'a> {
    file: &'a VFile<'a>,
    cursor: u64,
}

impl<'vfs> io::Read for VFileReader<'vfs> {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        let read_bytes = self.file.read_at(buf, self.cursor)?;
        self.cursor += read_bytes as u64;
        Ok(read_bytes)
    }
}

impl<'vfs> io::Seek for VFileReader<'vfs> {
    fn seek(&mut self, pos: io::SeekFrom) -> io::Result<u64> {
        match pos {
            io::SeekFrom::Start(offset) => {
                self.cursor = offset;
                Ok(self.cursor)
            }
            io::SeekFrom::End(offset) => {
                let len = self.file.len()?;
                self.cursor = len.checked_add_signed(offset).ok_or(io::Error::new(
                    io::ErrorKind::InvalidInput,
                    "Out of bounds offset",
                ))?;
                Ok(self.cursor)
            }
            io::SeekFrom::Current(offset) => {
                self.cursor = self
                    .cursor
                    .checked_add_signed(offset)
                    .ok_or(io::Error::new(
                        io::ErrorKind::InvalidInput,
                        "Out of bounds offset",
                    ))?;
                Ok(self.cursor)
            }
        }
    }
}

pub trait IAsyncVFile: Send + Sync + 'static {
    unsafe fn __read_at(
        &self,
        buf: NonNull<[u8]>,
        offset: u64,
        sender: SmallBox<dyn ReadChannel<Arc<VPath>>, [u128; 1]>,
        opaque: u64,
    ) -> Result<(), SendError<()>>;

    unsafe fn __read_exact_at(
        &self,
        buf: NonNull<[u8]>,
        offset: u64,
        sender: SmallBox<dyn ReadChannel<Arc<VPath>>, [u128; 1]>,
        opaque: u64,
    ) -> Result<(), SendError<()>>;

    fn __load(
        &self,
        sender: SmallBox<dyn LoadChannel<Arc<VPath>>, [u128; 1]>,
        opaque: u64,
    ) -> Result<(), SendError<()>>;

    fn path(&self) -> &VPath;
}

pub trait IAsyncVFileExt {
    unsafe fn read_at<T: ReadChannel<Arc<VPath>>>(
        &self,
        buf: NonNull<[u8]>,
        offset: u64,
        sender: T,
        opaque: u64,
    ) -> Result<(), SendError<()>>;

    unsafe fn read_exact_at<T: ReadChannel<Arc<VPath>>>(
        &self,
        buf: NonNull<[u8]>,
        offset: u64,
        sender: T,
        opaque: u64,
    ) -> Result<(), SendError<()>>;

    fn load<T: LoadChannel<Arc<VPath>>>(&self, sender: T, opaque: u64)
    -> Result<(), SendError<()>>;
}

impl<TT: IAsyncVFile + ?Sized> IAsyncVFileExt for TT {
    unsafe fn read_at<T: ReadChannel<Arc<VPath>>>(
        &self,
        buf: NonNull<[u8]>,
        offset: u64,
        sender: T,
        opaque: u64,
    ) -> Result<(), SendError<()>> {
        let sender: SmallBox<dyn ReadChannel<Arc<VPath>>, _> = smallbox!(sender);
        unsafe { self.__read_at(buf, offset, sender, opaque) }
    }

    unsafe fn read_exact_at<T: ReadChannel<Arc<VPath>>>(
        &self,
        buf: NonNull<[u8]>,
        offset: u64,
        sender: T,
        opaque: u64,
    ) -> Result<(), SendError<()>> {
        let sender: SmallBox<dyn ReadChannel<Arc<VPath>>, _> = smallbox!(sender);
        unsafe { self.__read_exact_at(buf, offset, sender, opaque) }
    }

    fn load<T: LoadChannel<Arc<VPath>>>(
        &self,
        sender: T,
        opaque: u64,
    ) -> Result<(), SendError<()>> {
        let sender: SmallBox<dyn LoadChannel<Arc<VPath>>, _> = smallbox!(sender);
        self.__load(sender, opaque)
    }
}

pub(crate) struct VFileVtable {
    pub(crate) read_at: fn(NonZero<u64>, &mut [u8], u64) -> io::Result<usize>,
    pub(crate) size: fn(NonZero<u64>) -> io::Result<u64>,
    pub(crate) close: fn(NonZero<u64>),
}
