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

use crate::device::Device;
use crate::surface::Surface;
use crate::swap_texture::SwapTexture;
use crossbeam::atomic::AtomicCell;
use dx12::dxgi;
use interfaces::gpu::{
    AcquireImageError, IDevice, INamedObject, ISwapChain, ITexture, QueueType, ResourceStates,
    SwapChainConfiguration, TextureDesc, TextureDimension,
};
use interfaces::ref_ptr::{ref_ptr_init, ref_ptr_object, RefPtr, RefPtrObject};
use parking_lot::Mutex;
use std::ops::Deref;
use std::sync::atomic::Ordering;

ref_ptr_object! {
    pub struct SwapChain: ISwapChain, ISwapChainExt {
        pub(crate) swap_chain: dxgi::SwapChain,
        pub(crate) device: RefPtr<Device>,
        pub(crate) surface: RefPtr<Surface>,
        pub(crate) queue_support: QueueType,
        pub(crate) inner: Mutex<SwapChainState>,
        pub(crate) queued_resize: AtomicCell<Option<Box<(u32, u32)>>>,
    }
}

pub struct SwapChainState {
    pub config: SwapChainConfiguration,
    pub acquired: bool,
    pub images: Vec<(dx12::Resource, dx12::CPUDescriptorHandle)>,
    pub dxgi_format: dxgi::Format,
    pub dxgi_view_format: dxgi::Format,
}

impl SwapChain {
    pub unsafe fn handle_resize(&self, inner: &mut SwapChainState, width: u32, height: u32) {
        // D3D12 requires releasing all references to the D3D12_RESOURCE handles associated with a
        // swap chain *before* calling ResizeBuffers. In order to meet this requirement we will
        // force a full device queue flush and garbage collection cycle.
        //
        // This way we know the only places that can be holding a reference to any of the swap chain
        // resources is the swap chain itself. That means we now have exclusive ownership of the
        // images and releasing the handles should leave them all freed. Assuming no implementation
        // bugs anyway.
        self.device.wait_idle();

        // Empty the images array as, assuming the rest of the code is correct, that array will
        // hold the only remaining references to the swap chain images
        inner.images.clear();

        let queues = &self.device.queues;
        let queue = match self.queue_support {
            QueueType::General => queues.general.as_ref().unwrap().read().handle.clone(),
            QueueType::Compute => queues.compute.as_ref().unwrap().read().handle.clone(),
            QueueType::Transfer => queues.transfer.as_ref().unwrap().read().handle.clone(),
        };
        let queues: Vec<dx12::CommandQueue> = inner
            .images
            .drain(..)
            .map(|(_, view)| {
                self.device.rtv_heap.free(view);
                queue.clone()
            })
            .collect();

        self.swap_chain
            .resize_buffers(
                queues.len() as u32,
                width,
                height,
                dxgi::Format::Unknown,
                dxgi::SwapChainFlags::NONE,
                None,
                &queues,
            )
            .unwrap();

        let images = self
            .device
            .create_views_for_swap_images(
                &self.swap_chain,
                inner.dxgi_view_format,
                queues.len() as u32,
            )
            .unwrap();

        inner.images = images;
        inner.config.width = width;
        inner.config.height = height;
    }
}

impl ISwapChain for SwapChain {
    fn present_supported_on_queue(&self, queue: QueueType) -> bool {
        queue == self.queue_support
    }

    fn queue_resize(&self, width: u32, height: u32) {
        let resize = Box::new((width, height));
        self.queued_resize.store(Some(resize));
    }

    fn get_config(&self) -> SwapChainConfiguration {
        self.inner.lock().config.clone()
    }

    fn acquire_image(&self) -> Result<RefPtr<dyn ITexture>, AcquireImageError> {
        let mut inner = self.inner.lock();

        if inner.acquired {
            return Err(AcquireImageError::ImageNotAvailable);
        }

        if let Some(dimensions) = self.queued_resize.take() {
            // TODO: We need to ensure that the swap chain is no longer in use before we do any of
            //       this. We also need to make sure all dx12::Resource handles to the swap images
            //       have been dropped.
            let new_width = dimensions.deref().0;
            let new_height = dimensions.deref().1;
            unsafe {
                self.handle_resize(&mut inner, new_width, new_height);
            }
        }

        let image = self.swap_chain.get_current_back_buffer_index();
        let image = ref_ptr_init! {
            SwapTexture {
                resource: inner.images[image as usize].0.clone(),
                view: inner.images[image as usize].1,
                desc: TextureDesc {
                    width: inner.config.width,
                    height: inner.config.height,
                    depth: 1,
                    format: inner.config.format,
                    dimension: TextureDimension::Texture2D,
                    initial_state: ResourceStates::PRESENT,
                    clear_value: None,
                    array_size: 1,
                    mip_levels: 1,
                    sample_count: 1,
                    sample_quality: 0,
                    allow_unordered_access: false,
                    allow_cube_face: false,
                    is_render_target: true
                },
                swap_chain: self.as_ref_ptr(),
            }
        };
        let image: RefPtr<SwapTexture> = RefPtr::new(image);

        inner.acquired = true;

        Ok(image.query_interface().unwrap())
    }
}

impl Drop for SwapChain {
    fn drop(&mut self) {
        // Release the surface as the swap chain no longer owns it
        debug_assert!(self.surface.has_swap_chain.swap(false, Ordering::SeqCst));

        let mut inner = self.inner.lock();
        for (_, view) in inner.images.drain(..) {
            self.device.rtv_heap.free(view);
        }
        assert!(!inner.acquired);
    }
}

pub trait ISwapChainExt: ISwapChain {
    fn get_raw_handle(&self) -> dxgi::SwapChain;

    fn get_raw_in_memory_format(&self) -> dxgi::Format;

    fn get_raw_view_format(&self) -> dxgi::Format;
}

impl ISwapChainExt for SwapChain {
    fn get_raw_handle(&self) -> dxgi::SwapChain {
        self.swap_chain.clone()
    }

    fn get_raw_in_memory_format(&self) -> dxgi::Format {
        self.inner.lock().dxgi_format
    }

    fn get_raw_view_format(&self) -> dxgi::Format {
        self.inner.lock().dxgi_view_format
    }
}

impl INamedObject for SwapChain {
    fn set_name(&self, _name: &str) {
        // Nothing to do on d3d12 as swap chains can't be named. SwapChain comes from dxgi which
        // doesn't implement D3D12Object.
    }
}
