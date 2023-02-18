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
use crate::internal::try_clone_value_into_slot;
use crate::surface::Surface;
use crate::texture::Texture;
use crossbeam::atomic::AtomicCell;
use interfaces::any::{declare_interfaces, AnyArc, AnyWeak};
use interfaces::anyhow::anyhow;
use interfaces::gpu::{
    AcquireImageError, IDevice, IGetPlatformInterface, ISwapChain, ITexture, QueueType,
    SwapChainConfiguration, TextureDesc, TextureDimension,
};
use parking_lot::{Mutex, RwLock};
use std::any::TypeId;
use std::collections::HashMap;
use std::ops::Deref;
use std::sync::atomic::Ordering;
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
    pub(crate) queued_resize: AtomicCell<Option<Box<(u32, u32)>>>,
}

declare_interfaces!(SwapChain, [ISwapChain]);

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

        // TODO: Should this be done in the validation layer?
        #[cfg(debug_assertions)]
        {
            // Assert that we have the only reference to the swap chain textures. D3D12 requires
            // that ID3D12Resources created from the swap chain are fully released before resizing
            // the swap chain
            for texture in inner.textures.iter() {
                // Both strong and weak count must be one to prove exclusive ownership of 'texture'
                //
                // Weak count will be 1 as the texture has an internal Weak reference to itself.
                debug_assert_eq!(texture.strong_count(), 1);
                debug_assert_eq!(texture.weak_count(), 1);
            }
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
        self.recreate_swap_images(inner, queues.len() as u32)
            .map_err(|v| anyhow!(v))?;

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

    fn get_config(&self) -> SwapChainConfiguration {
        self.inner.lock().config.clone()
    }

    fn queue_resize(&self, width: u32, height: u32) {
        let resize = Box::new((width, height));
        self.queued_resize.store(Some(resize));
    }

    unsafe fn acquire_image(&self) -> Result<AnyArc<dyn ITexture>, AcquireImageError> {
        let mut inner = self.inner.lock();

        if let Some(dimensions) = self.queued_resize.take() {
            let new_width = dimensions.deref().0;
            let new_height = dimensions.deref().1;
            unsafe {
                self.handle_resize(&mut inner, new_width, new_height)?;
            }
        }

        inner.current = unsafe { self.swap_chain.GetCurrentBackBufferIndex() as i32 };
        let image = inner.textures[inner.current as usize].clone();
        let image = AnyArc::map::<dyn ITexture, _>(image, |v| v);

        Ok(image)
    }

    fn get_current_image(&self) -> Option<AnyArc<dyn ITexture>> {
        let inner = self.inner.lock();

        if inner.current < 0 {
            None
        } else {
            let texture = inner.textures[inner.current as usize].clone();
            Some(AnyArc::map::<dyn ITexture, _>(texture, |v| v))
        }
    }
}

impl Drop for SwapChain {
    fn drop(&mut self) {
        // Release the surface as the swap chain no longer owns it
        debug_assert!(self.surface.has_swap_chain.swap(false, Ordering::SeqCst));
    }
}

impl IGetPlatformInterface for SwapChain {
    unsafe fn __query_platform_interface(&self, target: TypeId, out: *mut ()) -> Option<()> {
        try_clone_value_into_slot::<IDXGISwapChain4>(&self.swap_chain, out, target)
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
