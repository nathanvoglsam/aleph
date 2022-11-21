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

use crate::acquired_texture::AcquiredTexture;
use crate::device::Device;
use crate::surface::Surface;
use crate::texture::{SwapTexture, Texture, TextureInner};
use crossbeam::atomic::AtomicCell;
use interfaces::any::{declare_interfaces, AnyArc, AnyWeak};
use interfaces::gpu::{
    AcquireImageError, IAcquiredTexture, IDevice, INamedObject, ISwapChain, ITexture, QueueType,
    SwapChainConfiguration, TextureDesc, TextureDimension,
};
use parking_lot::Mutex;
use std::ops::Deref;
use std::sync::atomic::{AtomicBool, AtomicU32, Ordering};
use windows::core::IUnknown;
use windows::utils::CPUDescriptorHandle;
use windows::Win32::Graphics::Direct3D12::*;
use windows::Win32::Graphics::Dxgi::Common::*;
use windows::Win32::Graphics::Dxgi::*;

pub struct SwapChain {
    pub(crate) this: AnyWeak<Self>,
    pub(crate) device: AnyArc<Device>,
    pub(crate) surface: AnyArc<Surface>,
    pub(crate) swap_chain: IDXGISwapChain4,
    pub(crate) queue_support: QueueType,
    pub(crate) inner: Mutex<SwapChainState>,
    pub(crate) queued_resize: AtomicCell<Option<Box<(u32, u32)>>>,
    pub(crate) acquired: AtomicBool,
    pub(crate) images_in_flight: AtomicU32,
}

declare_interfaces!(SwapChain, [ISwapChain, ISwapChainExt]);

pub struct SwapChainState {
    pub config: SwapChainConfiguration,
    pub images: Vec<(ID3D12Resource, CPUDescriptorHandle)>,
    pub dxgi_format: DXGI_FORMAT,
    pub dxgi_view_format: DXGI_FORMAT,
}

impl SwapChain {
    pub unsafe fn handle_resize(
        &self,
        inner: &mut SwapChainState,
        width: u32,
        height: u32,
    ) -> Result<(), AcquireImageError> {
        // D3D12 requires releasing all references to the D3D12_RESOURCE handles associated with a
        // swap chain *before* calling ResizeBuffers. In order to meet this requirement we will
        // force a full device queue flush and garbage collection cycle.
        //
        // This way we know the only places that can be holding a reference to any of the swap chain
        // resources is the swap chain itself. That means we now have exclusive ownership of the
        // images and releasing the handles should leave them all freed. Assuming no implementation
        // bugs anyway.
        self.device.wait_idle();

        // The GPU API requires that no swap images are in use and there are no acquired images for
        // a resize to be possible.
        //
        // This is because of D3D12's requirements on ResizeBuffers.
        if self.images_in_flight.load(Ordering::Relaxed) > 0 {
            return Err(AcquireImageError::QueuedResizeFailed);
        }

        let queue = match self.queue_support {
            QueueType::General => self.device.general_queue.as_ref().unwrap().handle.clone(),
            QueueType::Compute => self.device.compute_queue.as_ref().unwrap().handle.clone(),
            QueueType::Transfer => self.device.transfer_queue.as_ref().unwrap().handle.clone(),
        };
        // Empty the images array as, assuming the rest of the code is correct, that array will
        // hold the only remaining references to the swap chain images.
        //
        // This also handles creating the list of queues we pass to ResizeBuffers
        let queues: Vec<ID3D12CommandQueue> = inner
            .images
            .drain(..)
            .map(|(_, view)| {
                self.device.descriptor_heaps.cpu_rtv_heap().free(view);
                queue.clone() // TODO: We should find a way to avoid this
            })
            .collect();

        self.resize_buffers(
            queues.len() as u32,
            width,
            height,
            DXGI_FORMAT_UNKNOWN,
            0,
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

        Ok(())
    }

    /// Wrapper for `IDXGISwapChain3::ResizeBuffers1`
    #[allow(clippy::too_many_arguments)]
    pub unsafe fn resize_buffers(
        &self,
        buffer_count: u32,
        width: u32,
        height: u32,
        format: DXGI_FORMAT,
        flags: u32,
        queues: &[ID3D12CommandQueue],
    ) -> windows::core::Result<()> {
        // Input validation
        debug_assert!(
            queues.len() <= DXGI_MAX_SWAP_CHAIN_BUFFERS as usize,
            "queues len must be <= 16"
        );
        debug_assert_eq!(
            queues.len(),
            buffer_count as usize,
            "queues len must == buffer count if buffer count != 0"
        );
        debug_assert!(
            buffer_count <= DXGI_MAX_SWAP_CHAIN_BUFFERS,
            "can't have more than 16 swap chain buffers"
        );

        static NODE_MASKS: [u32; 16] = [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
        let p_creation_node_mask = NODE_MASKS.as_ptr();

        // Arg unpack
        let pp_present_queue = queues.as_ptr() as *mut ID3D12CommandQueue;
        let pp_present_queue = pp_present_queue as *mut Option<IUnknown>;

        self.swap_chain.ResizeBuffers1(
            buffer_count,
            width,
            height,
            format.into(),
            flags,
            p_creation_node_mask,
            pp_present_queue,
        )
    }
}

impl ISwapChain for SwapChain {
    fn upgrade(&self) -> AnyArc<dyn ISwapChain> {
        AnyArc::map::<dyn ISwapChain, _>(self.this.upgrade().unwrap(), |v| v)
    }

    fn strong_count(&self) -> usize {
        self.this.strong_count()
    }

    fn weak_count(&self) -> usize {
        self.this.weak_count()
    }

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

    fn acquire_image(&self) -> Result<Box<dyn IAcquiredTexture>, AcquireImageError> {
        if self
            .acquired
            .compare_exchange(false, true, Ordering::Acquire, Ordering::Relaxed)
            .is_ok()
        {
            let mut inner = self.inner.lock();

            if let Some(dimensions) = self.queued_resize.take() {
                let new_width = dimensions.deref().0;
                let new_height = dimensions.deref().1;
                unsafe {
                    self.handle_resize(&mut inner, new_width, new_height)?;
                }
            }

            let index = unsafe { self.swap_chain.GetCurrentBackBufferIndex() };
            let image = AnyArc::new_cyclic(move |v| Texture {
                this: v.clone(),
                inner: TextureInner::Swap(SwapTexture {
                    swap_chain: self.this.upgrade().unwrap(),
                    resource: inner.images[index as usize].0.clone(),
                    view: inner.images[index as usize].1,
                    desc: TextureDesc {
                        width: inner.config.width,
                        height: inner.config.height,
                        depth: 1,
                        format: inner.config.format,
                        dimension: TextureDimension::Texture2D,
                        clear_value: None,
                        array_size: 1,
                        mip_levels: 1,
                        sample_count: 1,
                        sample_quality: 0,
                        allow_unordered_access: false,
                        allow_cube_face: false,
                        is_render_target: true,
                    },
                }),
            });
            let image = AnyArc::map::<dyn ITexture, _>(image, |v| v);

            self.images_in_flight.fetch_add(1, Ordering::Acquire);

            let acquired = Box::new(AcquiredTexture {
                swap_chain: self.this.upgrade().unwrap(),
                image,
            });

            Ok(acquired)
        } else {
            Err(AcquireImageError::ImageNotAvailable)
        }
    }
}

impl Drop for SwapChain {
    fn drop(&mut self) {
        // Release the surface as the swap chain no longer owns it
        debug_assert!(self.surface.has_swap_chain.swap(false, Ordering::SeqCst));

        let mut inner = self.inner.lock();
        for (_, view) in inner.images.drain(..) {
            self.device.descriptor_heaps.cpu_rtv_heap().free(view);
        }
        assert_eq!(self.images_in_flight.load(Ordering::Relaxed), 0);
        assert!(!self.acquired.load(Ordering::Relaxed));
    }
}

pub trait ISwapChainExt: ISwapChain {
    fn get_raw_handle(&self) -> IDXGISwapChain4;

    fn get_raw_in_memory_format(&self) -> DXGI_FORMAT;

    fn get_raw_view_format(&self) -> DXGI_FORMAT;
}

impl ISwapChainExt for SwapChain {
    fn get_raw_handle(&self) -> IDXGISwapChain4 {
        self.swap_chain.clone()
    }

    fn get_raw_in_memory_format(&self) -> DXGI_FORMAT {
        self.inner.lock().dxgi_format
    }

    fn get_raw_view_format(&self) -> DXGI_FORMAT {
        self.inner.lock().dxgi_view_format
    }
}

impl INamedObject for SwapChain {
    fn set_name(&self, _name: &str) {
        // Nothing to do on d3d12 as swap chains can't be named. SwapChain comes from dxgi which
        // doesn't implement D3D12Object.
    }
}
