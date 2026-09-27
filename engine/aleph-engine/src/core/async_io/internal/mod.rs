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
use std::pin::Pin;
use std::ptr::NonNull;
use std::sync::Arc;
use std::task::{Context, Poll, RawWaker, RawWakerVTable, Waker};

use aleph_alloc::instrumentation::IAllocationCategory;
use aleph_gen_arena::RawHandle;
use aleph_io_queue::AsyncIo;
use aleph_io_queue::channel::IoWaker;
use crossbeam::channel::{Receiver, Sender, bounded};

use crate::core::async_io::TaskFuture;
use crate::core::async_io::context::IoContext;

/// Contains the object we send across to the async executor thread that invokes the async fn to
/// be executed on the async thread.
pub struct FutureSpawner {
    spawner: NonNull<SpawnerVTable>,
}

// Safety: It is illegal to construct a FutureSpawner that closes over a !Send type
unsafe impl Send for FutureSpawner {}

impl FutureSpawner {
    /// Takes the given async function and wrap it into a [`FutureSpawner`].
    pub fn new<T>(spawner: T) -> Self
    where
        for<'a> T: (AsyncFnOnce(IoContext<'a>) -> io::Result<()>) + Send + 'static,
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

    /// Consumes the spawner, calling the wrapped async function and returning a boxed future from
    /// it.
    pub fn spawn<'a>(self, io: IoContext<'a>) -> Pin<Box<TaskFuture<'a>>> {
        let unwrapper = unsafe { self.spawner.as_ref().unwrapper };
        let spawner = self.spawner;
        std::mem::forget(self);
        unsafe { unwrapper(io, spawner) }
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

type UnwrapperFn =
    for<'a> unsafe fn(IoContext<'a>, NonNull<SpawnerVTable>) -> Pin<Box<TaskFuture<'a>>>;

unsafe fn unwrapper<'aa, T>(
    ctx: IoContext<'aa>,
    f: NonNull<SpawnerVTable>,
) -> Pin<Box<TaskFuture<'aa>>>
where
    for<'a> T: (AsyncFnOnce(IoContext<'a>) -> io::Result<()>) + Send + 'static,
{
    let f: Box<SpawnerContainer<T>> =
        unsafe { Box::from_raw(f.cast::<SpawnerContainer<T>>().as_ptr()) };
    Box::pin((f.spawner)(ctx))
}

unsafe fn dropper<'aa, T>(f: NonNull<SpawnerVTable>)
where
    T: Send + 'static,
{
    let _drop: Box<SpawnerContainer<T>> =
        unsafe { Box::from_raw(f.cast::<SpawnerContainer<T>>().as_ptr()) };
}

/// [`Waker`] implementation that notifies our event loop to poll a task.
pub struct TaskWaker {
    handle: RawHandle,
    sender: Sender<RawHandle>,
}

impl TaskWaker {
    /// Constructs a new [`TaskWaker`] directly.
    pub fn new(handle: RawHandle, sender: Sender<RawHandle>) -> Self {
        Self { handle, sender }
    }

    /// Constructs a new [`TaskWaker`] and wraps it into the [`Waker`] object.
    pub fn new_waker(handle: RawHandle, sender: Sender<RawHandle>) -> Waker {
        let this = Self::new(handle, sender);
        let this = AsyncIo::with(|| Arc::new(this));
        let this = Arc::into_raw(this) as *const ();
        let raw_waker = RawWaker::new(this, &VTABLE);
        unsafe { Waker::from_raw(raw_waker) }
    }

    unsafe fn clone_callback(ptr: *const ()) -> RawWaker {
        let this = unsafe { Arc::from_raw(ptr as *const Self) };
        let clone = Arc::clone(&this);
        let _ = Arc::into_raw(this);
        let clone = Arc::into_raw(clone) as *const ();
        RawWaker::new(clone, &VTABLE)
    }
    unsafe fn wake_callback(ptr: *const ()) {
        let this = unsafe { Arc::from_raw(ptr as *const Self) };
        let _ = this.sender.send(this.handle);
        let _ = Arc::into_raw(this);
    }
    unsafe fn wake_by_ref_callback(ptr: *const ()) {
        let this = unsafe { Arc::from_raw(ptr as *const Self) };
        let _ = this.sender.send(this.handle);
        let _ = Arc::into_raw(this);
    }
    unsafe fn drop_callback(ptr: *const ()) {
        let this = unsafe { Arc::from_raw(ptr as *const Self) };
        drop(this);
    }
}

static VTABLE: RawWakerVTable = RawWakerVTable::new(
    TaskWaker::clone_callback,
    TaskWaker::wake_callback,
    TaskWaker::wake_by_ref_callback,
    TaskWaker::drop_callback,
);

/// Future type intended for use with [`IoWaker`] based async tasks.
///
/// Takes a closure that is invoked once to kick off the async task being wrapped. The [`IoTask`]
/// will then await a notification via the [`Waker`] and pop the message from an internally
/// managed channel.
pub struct IoTask<R, F> {
    receiver: Option<Receiver<io::Result<R>>>,
    f: F,
    phantom: PhantomData<fn() -> io::Result<R>>,
}

impl<R, F> IoTask<R, F>
where
    R: Unpin,
    F: (FnMut(IoWaker<io::Result<R>>) -> io::Result<()>) + Unpin,
{
    /// Constructs a new [`IoTask`] from the given closure.
    ///
    /// The closure will be called exactly once, on the first time the [`IoTask`] future is polled.
    /// The closure is expected to spawn an async request and pas the [`IoWaker`] along so the
    /// executor will be notified once it is complete.
    pub fn new(f: F) -> Self {
        Self {
            receiver: None,
            f,
            phantom: Default::default(),
        }
    }
}

impl<R, F> Future for IoTask<R, F>
where
    R: Unpin,
    F: (FnMut(IoWaker<io::Result<R>>) -> io::Result<()>) + Unpin,
{
    type Output = io::Result<R>;

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        match &self.receiver {
            Some(receiver) => match receiver.try_recv() {
                Ok(result) => Poll::Ready(result),
                Err(_) => Poll::Pending,
            },
            None => {
                let waker = cx.waker().clone();
                let (sender, receiver) = bounded::<io::Result<R>>(1);
                let io_waker = IoWaker::new(waker, sender);
                let result = (self.f)(io_waker);
                match result {
                    Ok(_) => {
                        self.receiver = Some(receiver);
                        Poll::Pending
                    }
                    Err(e) => Poll::Ready(Err(e)),
                }
            }
        }
    }
}
