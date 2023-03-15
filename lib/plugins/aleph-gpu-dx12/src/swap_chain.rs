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
use crate::internal::{try_clone_value_into_slot, unwrap};
use crate::surface::Surface;
use crate::texture::Texture;
use interfaces::any::{declare_interfaces, AnyArc, AnyWeak};
use interfaces::anyhow::anyhow;
use interfaces::gpu::*;
use parking_lot::{Mutex, RwLock};
use std::any::TypeId;
use std::collections::HashMap;
use std::sync::atomic::{AtomicBool, Ordering};
use windows::core::IUnknown;
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
    pub(crate) acquired: AtomicBool,
}

declare_interfaces!(SwapChain, [ISwapChain]);

impl IGetPlatformInterface for SwapChain {
    unsafe fn __query_platform_interface(&self, target: TypeId, out: *mut ()) -> Option<()> {
        try_clone_value_into_slot::<IDXGISwapChain4>(&self.swap_chain, out, target)
    }
}

pub struct SwapChainState {
    pub config: SwapChainConfiguration,
    pub current: i32,
    pub textures: Vec<AnyArc<Texture>>,
    pub dxgi_format: DXGI_FORMAT,
    pub dxgi_view_format: DXGI_FORMAT,
}

impl SwapChain {
    pub unsafe fn recreate_swap_images(
        &self,
        state: &mut SwapChainState,
        count: u32,
    ) -> windows::core::Result<()> {
        state.textures.clear();
        for i in 0..count {
            let resource = self.swap_chain.GetBuffer::<ID3D12Resource>(i)?;
            let desc = TextureDesc {
                width: state.config.width,
                height: state.config.height,
                depth: 1,
                format: state.config.format,
                dimension: TextureDimension::Texture2D,
                clear_value: None,
                array_size: 1,
                mip_levels: 1,
                sample_count: 1,
                sample_quality: 0,
                allow_unordered_access: false,
                allow_cube_face: false,
                is_render_target: true,
                name: None,
            };
            let dxgi_format = state.dxgi_format;
            let texture = AnyArc::new_cyclic(move |v| Texture {
                this: v.clone(),
                device: self.device.clone(),
                resource,
                desc,
                name: None,
                dxgi_format,
                rtv_cache: RwLock::new(HashMap::new()),
                dsv_cache: RwLock::new(HashMap::new()),
                srv_cache: RwLock::new(HashMap::new()),
                uav_cache: RwLock::new(HashMap::new()),
            });

            state.textures.push(texture);
        }

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
            format,
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

    fn get_config(&self) -> SwapChainConfiguration {
        self.inner.lock().config.clone()
    }

    fn rebuild(
        &self,
        new_size: Option<Extent2D>,
    ) -> Result<SwapChainConfiguration, SwapChainRebuildError> {
        let mut inner = self.inner.lock();

        let (width, height) = if let Some(Extent2D { width, height }) = new_size {
            (width, height)
        } else {
            (0, 0)
        };

        // D3D12 requires releasing all references to the D3D12_RESOURCE handles associated with a
        // swap chain *before* calling ResizeBuffers. In order to meet this requirement we will
        // force a full device queue flush and garbage collection cycle.
        //
        // This way we know the only places that can be holding a reference to any of the swap chain
        // resources is the swap chain itself. That means we now have exclusive ownership of the
        // images and releasing the handles should leave them all freed. Assuming no implementation
        // bugs anyway.
        self.device.wait_idle();

        // Assert that we have the only reference to the swap chain textures. D3D12 requires
        // that ID3D12Resources created from the swap chain are fully released before resizing
        // the swap chain
        #[cfg(debug_assertions)]
        for v in inner.textures.iter_mut() {
            assert!(
                v.weak_count() == 1 && v.strong_count() == 1,
                "It is invalid to resize a swap chain while still holding references to its images"
            )
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
            .textures
            .drain(..)
            .map(|_| {
                queue.clone() // TODO: We should find a way to avoid this
            })
            .collect();

        unsafe {
            self.resize_buffers(
                queues.len() as u32,
                width,
                height,
                DXGI_FORMAT_UNKNOWN,
                0,
                &queues,
            )
            .unwrap();

            inner.config.width = width;
            inner.config.height = height;
            self.recreate_swap_images(&mut inner, queues.len() as u32)
                .map_err(|v| anyhow!(v))?;
        }

        self.acquired.store(false, Ordering::SeqCst);

        Ok(inner.config.clone())
    }

    fn get_images(&self, images: &mut [Option<AnyArc<dyn ITexture>>]) {
        let lock = self.inner.lock();
        let textures = &lock.textures;

        for (out, v) in images.iter_mut().zip(textures.iter()) {
            *out = Some(v.upgrade());
        }
    }

    unsafe fn acquire_next_image(&self, desc: &AcquireDesc) -> Result<u32, ImageAcquireError> {
        if self
            .acquired
            .compare_exchange(false, true, Ordering::SeqCst, Ordering::SeqCst)
            .is_err()
        {
            panic!("Attempted to acquire an image while one is already acquired");
        }

        let index = self.swap_chain.GetCurrentBackBufferIndex();

        for semaphore in unwrap::semaphore_iter(desc.signal_semaphores) {
            semaphore.signal_from_cpu().map_err(|v| anyhow!(v))?;
        }

        Ok(index)
    }
}

impl Drop for SwapChain {
    fn drop(&mut self) {
        let mut has_swap_chain = self.surface.has_swap_chain.lock();

        // Release the surface as the swap chain no longer owns it
        assert!(*has_swap_chain);
        *has_swap_chain = false;
    }
}
