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

use aleph_alloc::BVec;
use aleph_alloc::instrumentation::system;

use crate::internal::MgSystem;

/// Manager object that handles extending rhi resource lifetimes until they are no longer in use on
/// the GPU.
///
/// Takes bundles of rhi handles + a fence that will be signaled to end those handle's lifetime.
/// Polling the deletion manager will check all the bundles, see if the fence is signaled, and drop
/// all the handles if it is.
///
/// The expected use case for this is... well... the renderer. The renderer will, as a rule, need
/// to keep resources alive as long as they can be in use. This is a tool to ensure this. Things
/// that will end up in here include:
///
/// - Upload buffers
/// - Textures
/// - GPU buffers
/// - Descriptor pools/descriptor arenas
pub struct DeferredDeletionManager {
    /// GPU device handle
    device: Arc<dyn rhi::IDevice>,

    /// Collection of deletion bundles. These will be destroyed when their associated fence is
    /// found to be signaled.
    bundles: BVec<DeletionBundle, MgSystem>,
}

impl DeferredDeletionManager {
    /// Creates a new manager instance that manages resources for the given device.
    pub fn new(device: Arc<dyn rhi::IDevice>) -> Self {
        Self {
            device,
            bundles: BVec::new_in(system()),
        }
    }

    /// Push the provided bundle in to the manager's set of active bundles.
    pub fn push_bundle(&mut self, bundle: DeletionBundle) {
        self.bundles.push(bundle);
    }

    /// Poll the inner bundle set and release any bundles where the fence is signaled.
    ///
    /// This will not block, and merely polls the fences. You would typically call this once per
    /// frame to ensure resources are released in a timely fashion.
    pub fn delete_retired_bundles(&mut self) {
        let mut i = 0;
        loop {
            if i >= self.bundles.len() {
                break;
            }

            let bundle = &self.bundles[i];
            let result = self.device.get_fence_signaled_value(&bundle.fence);
            let logical_value = match result {
                // Use the real value on success
                Ok(value) => value,

                // Project 'device lost' to be u64::MAX so it's always signaled
                Err(rhi::FencePollError::DeviceLost) => u64::MAX,

                // Panic for other errors, we're hosed.
                Err(_) => {
                    // TODO: surface an error?
                    panic!("Unrecoverable 'get_fence_signaled_value' error");
                }
            };
            if logical_value >= bundle.value {
                let DeletionBundle {
                    fence,
                    buffers,
                    textures,
                    ..
                } = self.bundles.swap_remove(i);
                drop(buffers);
                drop(textures);
                drop(fence);
                continue;
            }

            // Only increment if we didn't remove a bundle.
            i += 1;
        }
    }

    /// Intended for use when dropping the renderer object. Will assert all bundles are complete and
    /// drop them immediately.
    ///
    /// # Stall
    ///
    /// This will stall waiting on the active bundle's to be signaled. It is critical that this is
    /// called in a way that active bundles may become signaled, as otherwise you will deadlock in
    /// this call.
    ///
    /// # Panics
    ///
    /// This will panic if any of the fences in the active bundles fail to complete when waited on.
    pub fn assert_delete_all(&mut self) {
        for bundle in self.bundles.drain(..) {
            // Wait for the fence to be signaled so we know for sure the work is done
            match self
                .device
                .wait_fences(&[&bundle.fence], &[bundle.value], true, u32::MAX)
            {
                // The work is considered complete if the fence is waited on successfully, or if
                // we get a 'device lost' error. In either case we can drop the resources safely.
                //
                // If we time out or fail for another reason we should panic. There's nothing we can
                // do.
                Ok(rhi::FenceWaitResult::Complete) | Err(rhi::FenceWaitError::DeviceLost) => {
                    drop(bundle);
                }
                Ok(_) => {
                    panic!("Unexpected 'wait_fences' timeout");
                }
                Err(_) => {
                    panic!("Unrecoverable 'wait_fences' error");
                }
            }
        }
    }
}

impl Drop for DeferredDeletionManager {
    fn drop(&mut self) {
        self.assert_delete_all();
    }
}

/// A set of resource handles that are associated to a given fence.
///
/// Once submitted to a [`DeferredDeletionManager`], the resources in a deletion bundle will be
/// kept alive until after the associated fence is signaled. The manager will poll its live bundles
/// and drop any bundles it finds with a signaled fence.
///
/// # Warning
///
/// If a bundle is dropped without being submitted to a deletion manager then the handles ___will
/// immediately drop too___. This means the resources lifetime will __not__ be extended. It is
/// almost certainly an error for this to happen, so don't let it.
///
/// This code isn't marked unsafe because it only becomes unsafe once GPU work is submitted, which
/// _is_ marked as an unsafe operation.
pub struct DeletionBundle {
    /// Fence that will be polled. Once it is polled as signaled a deletion manager will release
    /// all the handles stored in this bundle.
    fence: rhi::FenceHandle,

    /// The value that should be waited on with the provided fence.
    value: u64,

    /// List of buffer handles to be kept alive in a deletion manager
    pub buffers: BVec<rhi::BufferHandle, MgSystem>,

    /// List of texture handles to be kept alive in a deletion manager
    pub textures: BVec<rhi::TextureHandle, MgSystem>,
}

impl DeletionBundle {
    /// Constructs a new, empty bundle that as associated with the given fence.
    pub const fn new(fence: rhi::FenceHandle, value: u64) -> DeletionBundle {
        DeletionBundle {
            fence,
            value,
            buffers: BVec::new_in(system()),
            textures: BVec::new_in(system()),
        }
    }
}
