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
use std::pin::Pin;
use std::ptr::NonNull;

use mg::async_resource_loader::AsyncResourceLoader;

use crate::core::async_io::context::IoContext;
use crate::render::async_loader::resources::async_loader_requests::ResourceLoadHandle;

/// Interface of our tasks futures (once boxed, hence the `dyn`).
pub type TaskFuture<'a> = dyn Future<Output = io::Result<()>> + 'a;

/// Contains the object we send across to the async executor thread that invokes the async fn to
/// be executed on the async thread.
pub(crate) struct FutureSpawner {
    spawner: NonNull<SpawnerVTable>,
}

// Safety: It is illegal to construct a FutureSpawner that closes over a !Send type
unsafe impl Send for FutureSpawner {}

impl FutureSpawner {
    pub(crate) fn new<T>(spawner: T) -> Self
    where
        for<'a> T: (AsyncFnOnce(
                IoContext<'a>,
                &'a AsyncResourceLoader<ResourceLoadHandle>,
            ) -> io::Result<()>)
            + Send
            + 'static,
    {
        let spawner = SpawnerContainer::<T> {
            vtable: SpawnerVTable {
                unwrapper: unwrapper::<T>,
                dropper: dropper::<T>,
            },
            spawner,
        };
        let boxed: Box<SpawnerContainer<T>> = Box::new(spawner);
        let raw = Box::into_raw(boxed);
        let raw = unsafe { NonNull::new(raw).unwrap_unchecked() };

        FutureSpawner {
            spawner: raw.cast(),
        }
    }

    pub(crate) fn spawn<'a>(
        self,
        io: IoContext<'a>,
        loader: &'a AsyncResourceLoader<ResourceLoadHandle>,
    ) -> Pin<Box<TaskFuture<'a>>> {
        let unwrapper = unsafe { self.spawner.as_ref().unwrapper };
        let spawner = self.spawner;
        std::mem::forget(self);
        unsafe { unwrapper(io, loader, spawner) }
    }
}

impl Drop for FutureSpawner {
    fn drop(&mut self) {
        unsafe {
            let dropper = { self.spawner.as_ref().dropper };
            dropper(self.spawner);
        }
    }
}

#[repr(C)]
struct SpawnerContainer<T: Send + 'static> {
    vtable: SpawnerVTable,
    spawner: T,
}

#[repr(C)]
struct SpawnerVTable {
    unwrapper: UnwrapperFn,
    dropper: unsafe fn(NonNull<SpawnerVTable>),
}

type UnwrapperFn = for<'a> unsafe fn(
    IoContext<'a>,
    &'a AsyncResourceLoader<ResourceLoadHandle>,
    NonNull<SpawnerVTable>,
) -> Pin<Box<TaskFuture<'a>>>;

unsafe fn unwrapper<'aa, T>(
    ctx: IoContext<'aa>,
    loader: &'aa AsyncResourceLoader<ResourceLoadHandle>,
    f: NonNull<SpawnerVTable>,
) -> Pin<Box<TaskFuture<'aa>>>
where
    for<'a> T: (AsyncFnOnce(IoContext<'a>, &'a AsyncResourceLoader<ResourceLoadHandle>) -> io::Result<()>)
        + Send
        + 'static,
{
    let f: Box<SpawnerContainer<T>> =
        unsafe { Box::from_raw(f.cast::<SpawnerContainer<T>>().as_ptr()) };
    Box::pin((f.spawner)(ctx, loader))
}

unsafe fn dropper<'aa, T>(f: NonNull<SpawnerVTable>)
where
    T: Send + 'static,
{
    let _drop: Box<SpawnerContainer<T>> =
        unsafe { Box::from_raw(f.cast::<SpawnerContainer<T>>().as_ptr()) };
}
