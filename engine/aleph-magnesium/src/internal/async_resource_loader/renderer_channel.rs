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

use std::sync::Arc;

use crossbeam::channel::{Receiver, Sender, TryRecvError};
use thiserror::Error;

use crate::async_resource_loader::{BufferLoadResult, TextureLoadResult};
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

        /// Channel that will be notified
        sender: kanal::Sender<BufferLoadResult<C>>,
    },

    /// A texture upload was successfully completed in full. Provides the resource to make available
    /// and a cookie to notify the caller who spawned the request.
    TextureComplete {
        /// Cookie that should be dispatched to the downstream listener once the renderer has
        /// allocated a handle for the new resource.
        cookie: C,

        /// The fully initialized resource to be made available on the renderer's main queue.
        resource: rhi::TextureHandle,

        /// Channel that will be notified
        sender: kanal::Sender<TextureLoadResult<C>>,
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
    pub device: Arc<dyn rhi::IDevice>,

    /// Receives messages from a resource loader.
    pub renderer_receiver: LoaderReceiver<C>,
}

impl<C: Send + 'static> LoaderMessageDispatcher for GenericLoaderMessageDispatcher<C> {
    fn dispatch_messages(
        &self,
        last_uses: &mut LastUseTracker,
        bpool: &mut BufferObjectStore,
        tpool: &mut TextureObjectStore,
    ) -> Result<(), LoaderDispatcherError> {
        loop {
            match self.renderer_receiver.try_recv() {
                Ok(LoaderToRendererMessage::BufferComplete {
                    cookie,
                    resource,
                    sender,
                }) => {
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

                    let msg = (Ok(handle), cookie);
                    match sender.send(msg) {
                        Ok(_) => {}
                        Err(_) => {
                            bpool.pool.free(handle);
                            last_uses.buffers.remove(&handle);
                        }
                    }
                }
                Ok(LoaderToRendererMessage::TextureComplete {
                    cookie,
                    resource,
                    sender,
                }) => {
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

                    let msg = (Ok(handle), cookie);
                    match sender.send(msg) {
                        Ok(_) => {}
                        Err(_) => {
                            tpool.pool.free(handle);
                            last_uses.textures.remove(&handle);
                        }
                    }
                }
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
        }
    }
}

#[derive(Error, Debug)]
pub enum LoaderDispatcherError {
    #[error("The loader has disconnected from the channel.")]
    LoaderDisconnected,
}
