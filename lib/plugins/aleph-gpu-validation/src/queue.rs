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
    ValidationSwapChain, ValidationTexture,
};
use interfaces::any::{AnyArc, AnyWeak, QueryInterface};
use interfaces::gpu::*;

pub struct ValidationQueue {
    pub(crate) _this: AnyWeak<Self>,
    pub(crate) _device: AnyWeak<ValidationDevice>,
    pub(crate) inner: AnyArc<dyn IQueue>,
    pub(crate) queue_type: QueueType,
}

interfaces::any::declare_interfaces!(ValidationQueue, [IQueue]);

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

    unsafe fn submit(&self, desc: &QueueSubmitDesc) -> Result<(), QueueSubmitError> {
        for v in desc.command_lists {
            let v = v
                .query_interface::<ValidationCommandList>()
                .expect("Unknown ICommandList implementation");
            self.validate_command_list_submission(v);
        }

        for v in desc.wait_semaphores {
            let v = v
                .query_interface::<ValidationSemaphore>()
                .expect("Unknown ISemaphore implementation");
            let semaphore_state = v.state.load();

            assert_eq!(
                semaphore_state,
                SemaphoreState::Waiting,
                "It is invalid to submit a wait semaphore after using it in a previous submission"
            );
        }

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

    unsafe fn acquire(
        &self,
        desc: &QueueAcquireDesc,
    ) -> Result<AnyArc<dyn ITexture>, AcquireImageError> {
        let swap_chain = desc
            .swap_chain
            .query_interface::<ValidationSwapChain>()
            .expect("Unknown ISwapChain implementation");

        assert_eq!(
            self.queue_type, swap_chain.queue_support,
            "Tried to use a swap chain on an unsupported queue"
        );

        let result = get_as_unwrapped::queue_acquire_desc(desc, |inner_desc| {
            let result = self.inner.acquire(inner_desc);

            for v in desc.signal_semaphores {
                let v = v
                    .query_interface::<ValidationSemaphore>()
                    .expect("Unknown ISemaphore implementation");

                v.state.store(SemaphoreState::Waiting);
            }

            result
        });

        result.map(|inner| {
            let texture = AnyArc::new_cyclic(move |v| ValidationTexture {
                _this: v.clone(),
                _device: self._device.upgrade().unwrap(),
                inner,
            });
            AnyArc::map::<dyn ITexture, _>(texture, |v| v)
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

        get_as_unwrapped::queue_present_desc(desc, |inner_desc| {
            let result = self.inner.present(inner_desc);

            // for v in desc.wait_semaphores {
            //     let v = v
            //         .query_interface::<ValidationSemaphore>()
            //         .expect("Unknown ISemaphore implementation");
            //
            //     v.state.store(SemaphoreState::Waited);
            // }

            result
        })
    }

    unsafe fn set_marker(&mut self, _color: Color, _message: &str) {
        unimplemented!();
        // self.inner.set_marker(color, message);
    }

    unsafe fn begin_event(&mut self, _color: Color, _message: &str) {
        unimplemented!();
        // self.inner.begin_event(color, message);
    }

    unsafe fn end_event(&mut self) {
        unimplemented!();
        // self.inner.end_event();
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
