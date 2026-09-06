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
use std::num::NonZero;

use aleph_gen_arena::{GenArena, Handle, HandleType, RawHandle};

use crate::VfsSystem;
use crate::directory_layer::LocalVFile;
use crate::file::VFileVtable;

pub static VTABLE: VFileVtable = VFileVtable {
    read_at: read_at_vfile,
    size: size_vfile,
    close: close_vfile,
};

fn read_at_vfile(handle: NonZero<u64>, buf: &mut [u8], offset: u64) -> io::Result<usize> {
    let handle = RawHandle::from_int(handle);
    let handle = Handle::from_bare_handle(handle);

    POOL.with_borrow_mut(|pool| {
        let file = pool.get_mut(handle).unwrap();

        cfg_select! {
            unix => {
                use std::os::unix::fs::FileExt;
            }
            windows => {
                use std::os::windows::fs::FileExt;
            }
            _ => {}
        }

        let result: io::Result<usize> = cfg_select! {
            unix => file.file.read_at(buf, offset),
            windows => file.file.seek_read(buf, offset),
            _ => {
                unimplemented!()
            }
        };
        result
    })
}

fn size_vfile(handle: NonZero<u64>) -> io::Result<u64> {
    let handle = RawHandle::from_int(handle);
    let handle = Handle::from_bare_handle(handle);
    POOL.with_borrow(|pool| {
        let file = pool.get_ref(handle).unwrap();
        Ok(file.len)
    })
}

fn close_vfile(handle: NonZero<u64>) {
    let handle = RawHandle::from_int(handle);
    let handle = Handle::from_bare_handle(handle);
    POOL.with_borrow_mut(|pool| {
        let _ = pool.free(handle);
    })
}

pub struct PooledFile {
    pub file: File,
    pub len: u64,
}

type Pool = RefCell<GenArena<PooledFile, Handle<LocalVFile>, VfsSystem>>;
thread_local! {
    pub static POOL: Pool = const { RefCell::new(GenArena::new_in()) };
}
