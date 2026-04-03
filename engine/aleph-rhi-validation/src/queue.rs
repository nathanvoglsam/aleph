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

use std::sync::atomic::Ordering;
use std::sync::{Arc, Weak};

use aleph_rhi_api::*;

use crate::internal::{get_as_unwrapped, unwrap};
use crate::{ValidationCommandList, ValidationDevice, ValidationTexture};

pub struct ValidationQueue {
    pub(crate) _this: Weak<Self>,
    pub(crate) _device: Weak<ValidationDevice>,
    pub(crate) inner: Arc<dyn IQueue>,
    pub(crate) queue_type: QueueType,
}

crate::impl_platform_interface_passthrough!(ValidationQueue);

impl IQueue for ValidationQueue {
    fn upgrade(&self) -> Arc<dyn IQueue> {
        self._this.upgrade().unwrap()
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

    fn garbage_collect(&self) -> Result<(), QueueGarbageCollectError> {
        self.inner.garbage_collect()
    }

    fn wait_idle(&self) -> Result<(), QueueWaitError> {
        self.inner.wait_idle()
    }

    unsafe fn submit(&self, desc: &QueueSubmitDesc) -> Result<(), QueueSubmitError> {
        for v in desc.command_lists {
            let list_box = v.take().unwrap();
            self.validate_command_list_submission(unwrap::command_list(list_box.as_ref()));
            v.set(Some(list_box));
        }

        get_as_unwrapped::queue_submit_desc(desc, |inner_desc| {
            let result = unsafe { self.inner.submit(inner_desc) };
            result
        })
    }

    unsafe fn present(&self, swap_image: Arc<dyn ISwapImage>) -> Result<(), QueuePresentError> {
        let mut swap_image = {
            let v = swap_image;
            unwrap::swap_image_owned(v)
        };
        let swap_image = Arc::get_mut(&mut swap_image).unwrap();

        {
            let swap_texture = swap_image.texture.take().unwrap();
            let swap_texture = swap_texture
                .into_inner()
                .downcast::<ValidationTexture>()
                .unwrap();
            assert_eq!(Arc::strong_count(&swap_texture), 1);
        }

        let swap_chain = &swap_image._swap_chain;
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

        let inner_swap_image = swap_image.inner.take().unwrap();

        unsafe { self.inner.present(inner_swap_image) }
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
