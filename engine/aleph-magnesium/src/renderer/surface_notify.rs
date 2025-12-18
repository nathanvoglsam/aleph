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

use crate::common::channel::TryRecvError;

/// Generic channel interface that should be pumped with messages from the host application. We
/// only expose the 'receive' half of the channel through this interface as that is the minimum
/// that the renderer needs. This interface is _consumed_ by the renderer. We don't care how the
/// host sends the messages.
///
/// The host application should implement this trait and provide an instance to the renderer for
/// each rendering surface it shares. The host application should then send any relevant
/// messages regarding that surface in a way that calling [`ISurfaceNotify::try_recv`] will be able
/// to receive those messages.
///
/// A specific [`ISurfaceNotify`] channel is directly associated with a specific surface.
/// Notifications relevant to other surface should be sent through their respective channel.
pub trait ISurfaceNotify: Send + Sync {
    /// Called by the renderer to receive surface messages from the host.
    fn try_recv(&self) -> Result<SurfaceNotification, TryRecvError>;
}

/// Enumeration of the notifications and events a host application should send about a specific
/// surface.
pub enum SurfaceNotification {
    /// The host application has flagged that the surface has been resized. It's likely we need
    /// to rebuild the swap chain for this surface next time we draw a frame.
    ///
    /// This includes the size, in pixels, that the surface was resized to. This _should_ be the
    /// back buffer size we try to resize to, though depending on platform and backend it may be
    /// treated as a hint.
    Resized(rhi::Extent2D),
}

impl ISurfaceNotify for crossbeam::channel::Receiver<SurfaceNotification> {
    fn try_recv(&self) -> Result<SurfaceNotification, TryRecvError> {
        let result = crossbeam::channel::Receiver::try_recv(self)?;
        Ok(result)
    }
}
