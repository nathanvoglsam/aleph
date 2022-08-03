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

use crate::dxgi::gpu_preference::GpuPreference;
use crate::dxgi::{Adapter, SwapChain, SwapChainDesc1};
use crate::windows::core::Interface;
use crate::{CommandQueue, FeatureLevel};
use raw_window_handle::{HasRawWindowHandle, RawWindowHandle};
use std::mem::{transmute, transmute_copy};
use utf16_lit::utf16_null;
use windows::core::{IInspectable, IUnknown, GUID};
use windows::utils::DynamicLoadCell;
use windows::Win32::Foundation::HWND;
use windows::Win32::Graphics::Direct3D12::ID3D12Device4;
use windows::Win32::Graphics::Dxgi::{
    IDXGIAdapter1, IDXGIFactory2, IDXGIFactory6, IDXGISwapChain4, DXGI_ADAPTER_FLAG_SOFTWARE,
    DXGI_SWAP_CHAIN_DESC1,
};

type CreateFn =
    extern "system" fn(u32, *const GUID, *mut *mut ::std::ffi::c_void) -> windows::core::HRESULT;

static CREATE_FN: DynamicLoadCell<CreateFn> =
    DynamicLoadCell::new(&utf16_null!("dxgi.dll"), "CreateDXGIFactory2\0");

#[repr(transparent)]
pub struct Factory(pub(crate) IDXGIFactory2);

impl Factory {
    #[inline]
    pub fn new(debug: bool) -> windows::core::Result<Factory> {
        unsafe {
            let create_fn = *CREATE_FN.get().expect("Failed to load dxgi.dll");
            let flags = if debug { 0x1 } else { 0x0 };
            let mut dxgi_factory: Option<IDXGIFactory2> = None;
            let ptr = &mut dxgi_factory;
            let ptr = ptr as *mut Option<IDXGIFactory2>;
            let ptr = ptr as *mut *mut ::std::ffi::c_void;
            create_fn(flags, &IDXGIFactory2::IID, ptr)
                .and_some(dxgi_factory)
                .map(Self)
        }
    }

    pub fn create_swap_chain(
        &self,
        queue: &CommandQueue,
        window_handle: &impl HasRawWindowHandle,
        swap_chain_desc: &SwapChainDesc1,
    ) -> windows::core::Result<SwapChain> {
        unsafe {
            let desc = swap_chain_desc.clone();
            let desc = transmute(desc);
            let window_handle = window_handle.raw_window_handle();

            // Create the swapchain
            let swapchain = match window_handle {
                RawWindowHandle::Win32(hwnd) => {
                    assert!(!hwnd.hwnd.is_null());
                    let hwnd = HWND(hwnd.hwnd as isize);
                    self.create_swap_chain_for_hwnd(queue, hwnd, desc)?
                }
                RawWindowHandle::WinRt(core_window) => {
                    assert!(!core_window.core_window.is_null());
                    let core_window = core_window.core_window;
                    let core_window: IInspectable = std::mem::transmute(core_window);
                    self.create_swap_chain_for_core_window(queue, core_window, desc)?
                }
                _ => panic!("Unsupported window handle"),
            };

            Ok(SwapChain(swapchain))
        }
    }

    #[inline]
    unsafe fn create_swap_chain_for_hwnd(
        &self,
        queue: &CommandQueue,
        hwnd: HWND,
        desc: DXGI_SWAP_CHAIN_DESC1,
    ) -> windows::core::Result<IDXGISwapChain4> {
        let swapchain =
            self.0
                .CreateSwapChainForHwnd(queue.as_raw(), hwnd, &desc, std::ptr::null(), None)?;
        swapchain.cast::<IDXGISwapChain4>()
    }

    #[inline]
    unsafe fn create_swap_chain_for_core_window(
        &self,
        queue: &CommandQueue,
        core_window: IInspectable,
        desc: DXGI_SWAP_CHAIN_DESC1,
    ) -> windows::core::Result<IDXGISwapChain4> {
        let core_window = core_window.cast::<IUnknown>()?;
        let swapchain =
            self.0
                .CreateSwapChainForCoreWindow(queue.as_raw(), &core_window, &desc, None)?;
        swapchain.cast::<IDXGISwapChain4>()
    }

    #[inline]
    pub fn enumerate_adapters(&mut self, i: u32) -> windows::core::Result<Adapter> {
        unsafe { Self::enum_edapter_old(&self.0, i).map(Adapter) }
    }

    pub fn select_hardware_adapter(
        &self,
        minimum_feature_level: FeatureLevel,
        gpu_preference: GpuPreference,
    ) -> Option<Adapter> {
        unsafe {
            // If possible we can explicitly ask for a "high performance" device.
            let factory_6 = self.0.cast::<IDXGIFactory6>().ok();

            let create_fn = crate::dx12::device::CREATE_FN
                .get()
                .expect("Failed to load dxgi.dll")
                .unwrap();

            // Loop over all the available adapters
            let mut i = 0;
            loop {
                // Use the newest available interface to enumerate the adapter
                let adapter = if let Some(factory) = factory_6.as_ref() {
                    Self::enum_edapter_new(factory, i, gpu_preference)
                } else {
                    Self::enum_edapter_old(&self.0, i)
                };

                // Check if we've gotten an adapter, or break from the loop if we don't as we've either hit
                // a big error or enumerated all of them already
                if let Ok(adapter) = adapter {
                    // Get the adapter description so we can decide if we want to use it or not
                    let desc = adapter.GetDesc1().unwrap();

                    // We want to skip software adapters as they're going to be *very* slow
                    if (desc.Flags & DXGI_ADAPTER_FLAG_SOFTWARE.0) != 0 {
                        i += 1;
                        continue;
                    }

                    // Check if the device supports the feature level we want by trying to create a device
                    let result = create_fn(
                        transmute_copy(&adapter),
                        minimum_feature_level.into(),
                        &ID3D12Device4::IID,
                        std::ptr::null_mut(),
                    );

                    // If we succeeded then use the adapter
                    if result.is_ok() {
                        return Some(Adapter(adapter));
                    }
                } else {
                    break;
                }

                i += 1;
            }

            None
        }
    }
}

impl Factory {
    #[inline]
    unsafe fn enum_edapter_new(
        factory: &IDXGIFactory6,
        i: u32,
        gpu_preference: GpuPreference,
    ) -> windows::core::Result<IDXGIAdapter1> {
        factory.EnumAdapterByGpuPreference(i, gpu_preference.into())
    }

    #[inline]
    unsafe fn enum_edapter_old(
        factory: &IDXGIFactory2,
        i: u32,
    ) -> windows::core::Result<IDXGIAdapter1> {
        factory.EnumAdapters1(i)
    }
}

impl Clone for Factory {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
