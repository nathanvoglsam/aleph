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

use crate::gpu::{
    BackendAPI, BufferCreateError, BufferDesc, CommandPoolCreateError, DescriptorSetLayoutDesc,
    IAcquiredTexture, IBuffer, ICommandPool, IGeneralCommandList, INamedObject, ISampler, IShader,
    ITexture, QueuePresentError, QueueSubmitError, SamplerCreateError, SamplerDesc,
    ShaderCreateError, ShaderOptions, TextureCreateError, TextureDesc,
};
use any::{AnyArc, IAny};
use std::any::Any;
use thiserror::Error;

pub trait IDevice: INamedObject + Send + Sync + IAny + Any + 'static {
    any_arc_trait_utils_decl!(IDevice);

    /// Triggers a non blocking garbage collection cycle. This must be called for resources used in
    /// command lists to be freed. It is recommended to call this at least once per frame.
    fn garbage_collect(&self);

    /// Block the calling thread until all GPU queues are flushed of work. This is similar to
    /// vkDeviceWaitIdle.
    ///
    /// This will also trigger a GC cycle, freeing the releases from the now completed command
    /// lists.
    fn wait_idle(&self);

    fn create_shader(
        &self,
        options: &ShaderOptions,
    ) -> Result<AnyArc<dyn IShader>, ShaderCreateError>;

    fn create_descriptor_set_layout(&self, desc: &DescriptorSetLayoutDesc);

    fn create_buffer(&self, desc: &BufferDesc) -> Result<AnyArc<dyn IBuffer>, BufferCreateError>;

    fn create_texture(
        &self,
        desc: &TextureDesc,
    ) -> Result<AnyArc<dyn ITexture>, TextureCreateError>;

    fn create_sampler(
        &self,
        desc: &SamplerDesc,
    ) -> Result<AnyArc<dyn ISampler>, SamplerCreateError>;

    fn create_command_pool(&self) -> Result<AnyArc<dyn ICommandPool>, CommandPoolCreateError>;

    ///
    /// # Safety
    ///
    /// It is the caller's responsibility to ensure that the command lists submitted to the GPU
    /// contain a valid command stream.
    ///
    /// The GPU interfaces will uphold resource lifetime requirements and CPU synchronization
    /// requirements, but makes a very limited effort to handle GPU synchronization. It is up to the
    /// caller to record correct barriers.
    ///
    unsafe fn general_queue_submit_list(
        &self,
        command_list: Box<dyn IGeneralCommandList>,
    ) -> Result<(), QueueSubmitError>;

    ///
    /// # Safety
    ///
    /// It is the caller's responsibility to ensure that the command lists submitted to the GPU
    /// contain a valid command stream.
    ///
    /// The GPU interfaces will uphold resource lifetime requirements and CPU synchronization
    /// requirements, but makes a very limited effort to handle GPU synchronization. It is up to the
    /// caller to record correct barriers.
    ///
    unsafe fn general_queue_submit_lists(
        &self,
        command_lists: &mut dyn Iterator<Item = Box<dyn IGeneralCommandList>>,
    ) -> Result<(), QueueSubmitError>;

    ///
    /// # Safety
    ///
    /// It is the caller's responsibility to ensure that the image that is being presented will be
    /// in the required resource state for presentation by the time this operation will be executed
    /// on the GPU timeline.
    ///
    unsafe fn general_queue_present(
        &self,
        image: Box<dyn IAcquiredTexture>,
    ) -> Result<(), QueuePresentError>;

    /// Returns the API used by the underlying backend implementation.
    fn get_backend_api(&self) -> BackendAPI;
}

/// Set of errors that can occur when creating an [IDevice]
#[derive(Error, Debug)]
#[non_exhaustive]
pub enum RequestDeviceError {
    #[error("An internal backend error has occurred '{0}'")]
    Platform(#[from] anyhow::Error),
}
