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

use aleph_any::AnyArc;
use crossbeam::channel::{Receiver, Sender, TryRecvError};
use thiserror::Error;

use crate::async_resource_loader::loader_notify::LoaderMessage;
use crate::internal::buffer::{BufferObject, BufferObjectStore};
use crate::internal::renderer::last_use_tracker::{LastBufferUse, LastTextureUse, LastUseTracker};
use crate::internal::texture::{TextureObject, TextureObjectStore};

pub type LoaderSender<C> = Sender<LoaderToRendererMessage<C>>;
pub type LoaderReceiver<C> = Receiver<LoaderToRendererMessage<C>>;

/// Enumeration of all possible messages that the loader may send to the host renderer.
pub enum LoaderToRendererMessage<C: Send + 'static> {
    /// A buffer upload was successfully completed in full. Provides the resource to make available
    /// and a cookie to notify the caller who spawned the request.
    BufferComplete {
        /// Cookie that should be dispatched to the downstream listener once the renderer has
        /// allocated a handle for the new resource.
        cookie: C,

        /// The fully initialized resource to be made available on the renderer's main queue.
        resource: rhi::BufferHandle,
    },

    /// A texture upload was successfully completed in full. Provides the resource to make available
    /// and a cookie to notify the caller who spawned the request.
    TextureComplete {
        /// Cookie that should be dispatched to the downstream listener once the renderer has
        /// allocated a handle for the new resource.
        cookie: C,

        /// The fully initialized resource to be made available on the renderer's main queue.
        resource: rhi::TextureHandle,
    },

    /// A resource upload has been completed successfully, and the resource is now accessible via
    /// the given handle.
    Failed {
        /// Cookie that should be dispatched to the downstream listener to notify them of the
        /// canceled request.
        cookie: C,
    },

    /// A resource upload was canceled, either explicitly or implicitly (e.g. the loader was
    /// dropped). Provides the request cookie so the caller who spawned the request can be notified
    /// the request is canceled.
    Canceled {
        /// Cookie that should be dispatched to the downstream listener to notify them of the
        /// canceled request.
        cookie: C,
    },
}

/// Generic, virtual dispatch interface implemented by [`GenericLoaderMessageDispatcher`] used to
/// erase the generic type for a loader's channel.
///
/// Each loader instance can have a distinct 'cookie' type. The renderer needs to erase that type,
/// but still needs to interact with the loaders so we guard the interaction behind dynamic
/// dispatch.
pub trait LoaderMessageDispatcher: Send + Sync + 'static {
    fn dispatch_messages(
        &self,
        last_uses: &mut LastUseTracker,
        bpool: &mut BufferObjectStore,
        tpool: &mut TextureObjectStore,
    ) -> Result<(), LoaderDispatcherError>;
}

/// Implementation of [`LoaderMessageDispatcher`]. Handles any logic that must know the concrete
/// type 'C' for a loader instance via the trait impl. Enables the renderer to type erase the 'C'
/// and handle loaders uniformly. The renderer doesn't care about the cookies.
pub struct GenericLoaderMessageDispatcher<C: Send + 'static> {
    /// The GPU we're rendering with.
    pub device: AnyArc<dyn rhi::IDevice>,

    /// Receives messages from a resource loader.
    pub renderer_receiver: LoaderReceiver<C>,

    /// Sends messages to any outside listeners that need to know once resources they asked for are
    /// made available.
    pub renderer_sender: Sender<LoaderMessage<C>>,
}

impl<C: Send + 'static> LoaderMessageDispatcher for GenericLoaderMessageDispatcher<C> {
    fn dispatch_messages(
        &self,
        last_uses: &mut LastUseTracker,
        bpool: &mut BufferObjectStore,
        tpool: &mut TextureObjectStore,
    ) -> Result<(), LoaderDispatcherError> {
        loop {
            let send_result = match self.renderer_receiver.try_recv() {
                Ok(LoaderToRendererMessage::BufferComplete { cookie, resource }) => {
                    let object = BufferObject {
                        object: Some(resource),
                    };
                    let handle = bpool.pool.alloc(object);

                    last_uses.buffers.insert(
                        handle,
                        LastBufferUse {
                            sync: Default::default(),
                            access: Default::default(),
                            queue_transition: Some(rhi::QueueTransition {
                                before_queue: rhi::QueueType::Transfer,
                                after_queue: rhi::QueueType::General,
                            }),
                        },
                    );

                    self.renderer_sender.send(LoaderMessage::BufferComplete {
                        cookie,
                        resource: handle,
                    })
                }
                Ok(LoaderToRendererMessage::TextureComplete { cookie, resource }) => {
                    let rhi_desc = self.device.get_texture_desc(&resource);
                    let subresource_all = rhi::TextureSubResourceSet::all(&rhi_desc);
                    let format = rhi_desc.format;
                    let mut object = TextureObject {
                        object: Some(resource),
                        default_view: None,
                        subresource_all,
                        format,
                    };
                    object.recreate_default_view(self.device.as_ref());
                    let handle = tpool.pool.alloc(object);

                    last_uses.textures.insert(
                        handle,
                        LastTextureUse {
                            sync: Default::default(),
                            access: Default::default(),
                            layout: rhi::ImageLayout::ShaderReadOnly,
                            queue_transition: Some(rhi::QueueTransition {
                                before_queue: rhi::QueueType::Transfer,
                                after_queue: rhi::QueueType::General,
                            }),
                        },
                    );

                    self.renderer_sender.send(LoaderMessage::TextureComplete {
                        cookie,
                        resource: handle,
                    })
                }
                Ok(LoaderToRendererMessage::Failed { cookie }) => {
                    self.renderer_sender.send(LoaderMessage::Failed { cookie })
                }
                Ok(LoaderToRendererMessage::Canceled { cookie }) => self
                    .renderer_sender
                    .send(LoaderMessage::Canceled { cookie }),
                Err(TryRecvError::Empty) => {
                    // Messages are flushed, return with success
                    return Ok(());
                }
                Err(TryRecvError::Disconnected) => {
                    // Loader disconnected, notify the caller so the renderer can dispose of the
                    // dispatcher for the now dead loader.
                    return Err(LoaderDispatcherError::LoaderDisconnected);
                }
            };
            match send_result {
                Ok(_) => {
                    // Complete success, loop again and try pop another message to process.
                }
                Err(err) => {
                    // If we failed to send the message then we should destroy the resource we just
                    // created and clear it from the last use tracker. We have to cancel the
                    // resource because there's nobody to give the handle to. If we left the
                    // resource in place then we would leak it, nobody would have the handle to
                    // free the resource with.
                    match err.0 {
                        LoaderMessage::BufferComplete { resource, .. } => {
                            bpool.pool.free(resource);
                            last_uses.buffers.remove(&resource);
                        }
                        LoaderMessage::TextureComplete { resource, .. } => {
                            tpool.pool.free(resource);
                            last_uses.textures.remove(&resource);
                        }
                        LoaderMessage::Failed { .. } => {}
                        LoaderMessage::Canceled { .. } => {}
                    }

                    // If the listener disconnected we should notify the renderer. The loader should
                    // be destroyed because it's impossible to use any resources it loads because
                    // the handles will never be made available outside the renderer internals.
                    return Err(LoaderDispatcherError::ListenerDisconnected);
                }
            }
        }
    }
}

#[derive(Error, Debug)]
pub enum LoaderDispatcherError {
    #[error("The loader has disconnected from the channel.")]
    LoaderDisconnected,

    #[error("The 'notify' listener has disconnected from the channel.")]
    ListenerDisconnected,
}
