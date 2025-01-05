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

use std::mem::size_of_val;

use aleph_any::AnyArc;
use aleph_rhi_api::*;

use crate::FrameGraphResources;

pub trait PassArgs: 'static {
    type Args<'a>: Send + Sync;
}

impl PassArgs for () {
    type Args<'a> = ();
}

pub trait IRenderPass<A: PassArgs>: Send + Sync + 'static {
    fn execute(
        &mut self,
        encoder: &mut dyn IGeneralEncoder,
        graph: &mut GraphChannel,
        resources: &FrameGraphResources,
        args: &A::Args<'_>,
    );
}

pub(crate) struct CallbackRenderPass<ExecFn> {
    /// The function that will be called on execute
    exec_fn: ExecFn,
}

impl<ExecFn> CallbackRenderPass<ExecFn> {
    pub fn new(exec_fn: ExecFn) -> Self {
        assert!(
            size_of_val(&exec_fn) < 1024,
            "Size limit for ExecFn closure exceeded"
        );
        Self { exec_fn }
    }
}

impl<A: PassArgs, ExecFn> IRenderPass<A> for CallbackRenderPass<ExecFn>
where
    ExecFn: FnMut(&mut dyn IGeneralEncoder, &mut GraphChannel, &FrameGraphResources, &A::Args<'_>)
        + Send
        + Sync
        + 'static,
{
    fn execute<'a>(
        &mut self,
        encoder: &mut dyn IGeneralEncoder,
        graph: &mut GraphChannel,
        resources: &FrameGraphResources,
        args: &'a A::Args<'_>,
    ) {
        (self.exec_fn)(encoder, graph, resources, args)
    }
}

/// Accessor struct that allows sending information to the graph during the execution of the graph.
///
/// # Deferred Barriers
///
/// Each pass is allowed to issue any number of deferred barriers. These barriers are collected from
/// the pass while its execute function is running. These barriers are not encoded immediately and
/// are instead collected into a big list which will be flushed later in a single batch.
///
/// Deferred barriers are guaranteed to be encoded before any passes that have an execution
/// dependency on the pass that submits them. This dependency could be incidental through access to
/// a transient resource or explicit via an execution token. We also guarantee that these barriers
/// will be encoded in the event that the submitting pass is a terminal pass.
pub struct GraphChannel {
    /// Flags whether a global or buffer barrier was provided
    pub(crate) has_global_or_buffer_barrier: bool,

    /// Accumulated barrier info for manually queued buffer barriers.
    pub(crate) global_barrier: GlobalBarrier,

    /// Accumulated list of texture handles held to keep the references in the texture barrier list
    /// live.
    pub(crate) texture_objects: Vec<Option<AnyArc<dyn ITexture>>>,

    /// Accumulated list of manually queued texture barriers. 'deferred_texture_objects' stores
    /// owned references to the texture objects. We do some unsafe casting hackery so we can use
    /// [`TextureBarrier`] directly. It should be safe as long as we're careful.
    pub(crate) texture_barriers: Vec<TextureBarrier<'static>>,
}

impl GraphChannel {
    /// Inserts a deferred global barrier
    ///
    /// # Safety
    ///
    /// This function is directly unsafe to call, but it does cause a resource barrier to be encoded
    /// which is in general unsafe. We feel it's more correct to make this unsafe to reflect this.
    pub unsafe fn deferred_global_barrier(&mut self, barrier: &GlobalBarrier) {
        self.has_global_or_buffer_barrier = true;
        self.global_barrier.before_sync |= barrier.before_sync;
        self.global_barrier.before_access |= barrier.before_access;
        self.global_barrier.after_sync |= barrier.after_sync;
        self.global_barrier.after_access |= barrier.after_access;
    }

    /// Inserts a list of global barriers
    ///
    /// # Safety
    ///
    /// This function is directly unsafe to call, but it does cause a resource barrier to be encoded
    /// which is in general unsafe. We feel it's more correct to make this unsafe to reflect this.
    pub unsafe fn deferred_global_barriers(&mut self, barriers: &[GlobalBarrier]) {
        self.has_global_or_buffer_barrier = true;
        for barrier in barriers {
            self.global_barrier.before_sync |= barrier.before_sync;
            self.global_barrier.before_access |= barrier.before_access;
            self.global_barrier.after_sync |= barrier.after_sync;
            self.global_barrier.after_access |= barrier.after_access;
        }
    }

    /// Inserts a deferred buffer barrier
    ///
    /// # Safety
    ///
    /// This function is directly unsafe to call, but it does cause a resource barrier to be encoded
    /// which is in general unsafe. We feel it's more correct to make this unsafe to reflect this.
    pub unsafe fn deferred_buffer_barrier(&mut self, barrier: &BufferBarrier) {
        self.has_global_or_buffer_barrier = true;

        // We collapse buffer barriers into global barriers
        self.global_barrier.before_sync |= barrier.before_sync;
        self.global_barrier.before_access |= barrier.before_access;
        self.global_barrier.after_sync |= barrier.after_sync;
        self.global_barrier.after_access |= barrier.after_access;
    }

    /// Inserts a list of deferred buffer barriers
    ///
    /// # Safety
    ///
    /// This function is directly unsafe to call, but it does cause a resource barrier to be encoded
    /// which is in general unsafe. We feel it's more correct to make this unsafe to reflect this.
    pub unsafe fn deferred_buffer_barriers(&mut self, barriers: &[BufferBarrier]) {
        self.has_global_or_buffer_barrier = true;

        for barrier in barriers {
            // We collapse buffer barriers into global barriers
            self.global_barrier.before_sync |= barrier.before_sync;
            self.global_barrier.before_access |= barrier.before_access;
            self.global_barrier.after_sync |= barrier.after_sync;
            self.global_barrier.after_access |= barrier.after_access;
        }
    }

    /// Inserts a deferred texture barrier
    ///
    /// # Safety
    ///
    /// This function is directly unsafe to call, but it does cause a resource barrier to be encoded
    /// which is in general unsafe. We feel it's more correct to make this unsafe to reflect this.
    pub unsafe fn deferred_texture_barrier(&mut self, barrier: TextureBarrier) {
        let object = barrier.texture.map(|v| v.upgrade());
        self.texture_objects.push(object);
        unsafe {
            // Cast away the lifetime as we keep the reference live manually in 'texture_objects'.
            let barrier: TextureBarrier<'static> = std::mem::transmute(barrier);
            self.texture_barriers.push(barrier);
        }
    }

    /// Inserts a list of deferred texture barriers
    ///
    /// # Safety
    ///
    /// This function is directly unsafe to call, but it does cause a resource barrier to be encoded
    /// which is in general unsafe. We feel it's more correct to make this unsafe to reflect this.
    pub unsafe fn deferred_texture_barriers(&mut self, barrier: &[TextureBarrier]) {
        self.texture_objects
            .extend(barrier.iter().map(|v| v.texture.map(|v| v.upgrade())));
        self.texture_barriers.extend(barrier.iter().map(|v| {
            unsafe {
                // Cast away the lifetime as we keep the reference live manually in
                // 'texture_objects'.
                let barrier: TextureBarrier<'static> = std::mem::transmute(v.clone());
                barrier
            }
        }));
    }
}
