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

use crate::fence::FenceState;
use crate::internal::get_as_unwrapped;
use crate::semaphore::SemaphoreState;
use crate::{
    ValidationCommandList, ValidationDevice, ValidationFence, ValidationSemaphore,
    ValidationSwapChain,
};
use interfaces::any::{AnyArc, AnyWeak, IAny, QueryInterface, TraitObject};
use interfaces::gpu::*;
use std::any::TypeId;
use std::mem::transmute;
use std::ptr;
use std::ptr::NonNull;
use std::sync::atomic::Ordering;

pub struct ValidationQueue {
    pub(crate) _this: AnyWeak<Self>,
    pub(crate) _device: AnyWeak<ValidationDevice>,
    pub(crate) inner: AnyArc<dyn IQueue>,
    pub(crate) inner_debug: Option<AnyArc<dyn IQueueDebug>>,
    pub(crate) queue_type: QueueType,
}

// Unwrapped declare_interfaces as we need to inject a custom condition for returning IQueueDebug
impl IAny for ValidationQueue {
    #[allow(bare_trait_objects)]
    fn __query_interface(&self, target: TypeId) -> Option<TraitObject> {
        unsafe {
            if target == TypeId::of::<dyn IQueue>() {
                return Some(transmute(self as &dyn IQueue));
            }
            if target == TypeId::of::<dyn IQueueDebug>() && self.inner_debug.is_some() {
                return Some(transmute(self as &dyn IQueueDebug));
            }
            if target == TypeId::of::<dyn IAny>() {
                return Some(transmute(self as &dyn IAny));
            }
        }
        unsafe {
            if target == TypeId::of::<ValidationQueue>() {
                Some(TraitObject {
                    data: NonNull::new_unchecked(self as *const _ as *mut ()),
                    vtable: ptr::null_mut(),
                    phantom: Default::default(),
                })
            } else {
                None
            }
        }
    }
}

crate::impl_platform_interface_passthrough!(ValidationQueue);

impl IQueue for ValidationQueue {
    fn upgrade(&self) -> AnyArc<dyn IQueue> {
        AnyArc::map::<dyn IQueue, _>(self._this.upgrade().unwrap(), |v| v)
    }

    fn strong_count(&self) -> usize {
        self._this.strong_count()
    }

    fn weak_count(&self) -> usize {
        self._this.weak_count()
    }

    fn queue_properties(&self) -> QueueProperties {
        self.inner.queue_properties()
    }

    fn garbage_collect(&self) {
        self.inner.garbage_collect()
    }

    fn wait_idle(&self) {
        self.inner.wait_idle()
    }

    unsafe fn submit(&self, desc: &QueueSubmitDesc) -> Result<(), QueueSubmitError> {
        for v in desc.command_lists {
            let v = v
                .query_interface::<ValidationCommandList>()
                .expect("Unknown ICommandList implementation");
            self.validate_command_list_submission(v);
        }

        // TODO: Validating semaphore usage requires tracking the submissions to fully track a
        //       semaphore's state. Otherwise we can only know whether a semaphore has been queued
        //       for signalling.
        // for v in desc.wait_semaphores {
        //     let v = v
        //         .query_interface::<ValidationSemaphore>()
        //         .expect("Unknown ISemaphore implementation");
        //     let semaphore_state = v.state.load();
        //
        //     assert_eq!(
        //         semaphore_state,
        //         SemaphoreState::Waiting,
        //         "It is invalid to submit a wait semaphore after using it in a previous submission"
        //     );
        // }
        //
        // for v in desc.signal_semaphores {
        //     let v = v
        //         .query_interface::<ValidationSemaphore>()
        //         .expect("Unknown ISemaphore implementation");
        //     let semaphore_state = v.state.load();
        //
        //     assert_eq!(
        //         semaphore_state,
        //         SemaphoreState::Reset,
        //         "It is invalid to submit a signal semaphore before use in a previous submission"
        //     );
        // }

        if let Some(v) = desc.fence {
            let v = v
                .query_interface::<ValidationFence>()
                .expect("Unknown IFence implementation");
            let fence_state = v.state.load();

            assert_eq!(
                fence_state,
                FenceState::Reset,
                "It is invalid to submit a wait fence without resetting it from prior use"
            );
        }

        get_as_unwrapped::queue_submit_desc(desc, |inner_desc| {
            let result = self.inner.submit(inner_desc);

            // for v in desc.wait_semaphores {
            //     let v = v
            //         .query_interface::<ValidationSemaphore>()
            //         .expect("Unknown ISemaphore implementation");
            //
            //     v.state.store(SemaphoreState::Waited);
            // }

            for v in desc.signal_semaphores {
                let v = v
                    .query_interface::<ValidationSemaphore>()
                    .expect("Unknown ISemaphore implementation");

                v.state.store(SemaphoreState::Waiting);
            }

            if let Some(v) = desc.fence {
                let v = v
                    .query_interface::<ValidationFence>()
                    .expect("Unknown IFence implementation");
                v.state.store(FenceState::Waiting);
            }

            result
        })
    }

    unsafe fn present(&self, desc: &QueuePresentDesc) -> Result<(), QueuePresentError> {
        let swap_chain = desc
            .swap_chain
            .query_interface::<ValidationSwapChain>()
            .expect("Unknown ISwapChain implementation");

        assert_eq!(
            self.queue_type, swap_chain.queue_support,
            "Tried to use a swap chain on an unsupported queue"
        );

        if swap_chain
            .acquired
            .compare_exchange(true, false, Ordering::SeqCst, Ordering::SeqCst)
            .is_err()
        {
            panic!("Attempted to present an image without having first acquired one");
        }

        for v in desc.wait_semaphores {
            let v = v
                .query_interface::<ValidationSemaphore>()
                .expect("Unknown ISemaphore implementation");

            assert_eq!(
                v.state.load(),
                SemaphoreState::Waiting,
                "Attempted to wait on a semaphore that has not been queued for signalling"
            );
        }

        get_as_unwrapped::queue_present_desc(desc, |inner_desc| {
            let result = self.inner.present(inner_desc);

            result
        })
    }
}

impl IQueueDebug for ValidationQueue {
    fn set_marker(&self, color: Color, message: &str) {
        let debug = self.inner_debug.as_deref().unwrap();
        debug.set_marker(color, message)
    }

    fn begin_event(&self, color: Color, message: &str) {
        let debug = self.inner_debug.as_deref().unwrap();
        debug.begin_event(color, message)
    }

    fn end_event(&self) {
        let debug = self.inner_debug.as_deref().unwrap();
        debug.end_event()
    }
}

impl ValidationQueue {
    pub fn validate_command_list_submission<'b>(
        &self,
        list: &'b ValidationCommandList,
    ) -> &'b ValidationCommandList {
        match self.queue_type {
            QueueType::General => {
                assert!(
                    matches!(list.list_type, QueueType::General),
                    "Can only submit 'General' command lists to 'General' queues"
                )
            }
            QueueType::Compute => {
                assert!(
                    matches!(list.list_type, QueueType::General | QueueType::Compute),
                    "Can only submit 'Compute' command lists to 'General' or 'Compute' queues"
                )
            }
            QueueType::Transfer => {
                // Transfer command lists can be submitted to any queue type
            }
        }
        list
    }
}
