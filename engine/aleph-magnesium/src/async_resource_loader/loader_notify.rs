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

use crossbeam::channel::Receiver;

use crate::common::channel::{RecvError, TryRecvError};
use crate::resource::buffer::BufferHandle;
use crate::resource::texture::TextureHandle;

/// Handle to the recevier half of a channel that the renderer/loader will issue messages onto.
///
/// Callers subscribe to this channel to receive completed [`BufferHandle`] and [`TextureHandle`]
/// handles for the resources they attempt to load asynchronously using a
/// [`crate::async_resource_loader::AsyncResourceLoader`].
#[derive(Clone)]
pub struct LoaderNotify<C: Send + 'static> {
    pub(crate) receiver: Receiver<LoaderMessage<C>>,
}

impl<C: Send + 'static> LoaderNotify<C> {
    /// Blocks the current thread until a message is received or the channel is empty and
    /// disconnected.
    ///
    /// If the channel is empty and not disconnected, this call will block until the receive
    /// operation can proceed. If the channel is empty and becomes disconnected, this call will wake
    /// up and return an error.
    ///
    /// If called on a zero-capacity channel, this method will wait for a send operation to appear
    /// on the other side of the channel.
    pub fn recv(&self) -> Result<LoaderMessage<C>, RecvError> {
        self.receiver.recv().map_err(|_| RecvError)
    }

    /// Attempts to receive a message from the channel without blocking.
    ///
    /// This method will either receive a message from the channel immediately or return an error if
    /// the channel is empty.
    ///
    /// If called on a zero-capacity channel, this method will receive a message only if there
    /// happens to be a send operation on the other side of the channel at the same time.
    pub fn try_recv(&self) -> Result<LoaderMessage<C>, TryRecvError> {
        self.receiver.try_recv().map_err(From::from)
    }
}

/// The set of messages that can be received on a [`LoaderNotify`] channel.
#[derive(Clone)]
pub enum LoaderMessage<C> {
    /// A resource upload has been completed successfully, and the resource is now accessible via
    /// the given handle.
    BufferComplete {
        /// Cookie value the request was spawned with.
        cookie: C,

        /// Valid resource handle, ready for use in recording frames immediately.
        resource: BufferHandle,
    },

    /// A resource upload has been completed successfully, and the resource is now accessible via
    /// the given handle.
    TextureComplete {
        /// Cookie value the request was spawned with.
        cookie: C,

        /// Valid resource handle, ready for use in recording frames immediately.
        resource: TextureHandle,
    },

    /// An upload request has failed. This is distinct from cancelation, which occurs when a
    /// request is intentionally ended while incomplete.
    Failed {
        /// Cookie value the request was spawned with.
        cookie: C,
    },

    /// An upload was canceled.
    Canceled {
        /// Cookie value the request was spawned with.
        cookie: C,
    },
}
