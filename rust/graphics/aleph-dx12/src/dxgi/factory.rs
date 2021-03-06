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

use crate::dxgi::{Adapter, SwapChain, SwapChainDesc1};
use crate::windows_raw::win32::direct3d12::ID3D12Device4;
use crate::windows_raw::win32::dxgi::{
    IDXGIAdapter1, IDXGIFactory2, IDXGIFactory6, IDXGISwapChain1, IDXGISwapChain4,
    DXGI_ADAPTER_DESC1, DXGI_ADAPTER_FLAG, DXGI_GPU_PREFERENCE, DXGI_SWAP_CHAIN_DESC1,
};
use crate::windows_raw::win32::windows_and_messaging::HWND;
use crate::{CommandQueue, FeatureLevel};
use raw_window_handle::{HasRawWindowHandle, RawWindowHandle};
use std::mem::{transmute, transmute_copy};
use std::ops::Deref;
use utf16_lit::utf16_null;
use windows_raw::utils::DynamicLoadCell;
use windows_raw::win32::winrt::IInspectable;
use windows_raw::{Abi, Interface};

type CreateFn = extern "system" fn(
    u32,
    *const windows_raw::Guid,
    *mut *mut ::std::ffi::c_void,
) -> crate::ErrorCode;

static CREATE_FN: DynamicLoadCell<CreateFn> =
    DynamicLoadCell::new(&utf16_null!("dxgi.dll"), "CreateDXGIFactory2\0");

#[repr(transparent)]
pub struct Factory(pub(crate) IDXGIFactory2);

impl Factory {
    pub fn new(debug: bool) -> crate::Result<Factory> {
        unsafe {
            let create_fn = *CREATE_FN.get().expect("Failed to load dxgi.dll");
            let flags = if debug { 0x1 } else { 0x0 };
            let mut dxgi_factory: Option<IDXGIFactory2> = None;
            create_fn(flags, &IDXGIFactory2::IID, dxgi_factory.set_abi())
                .and_some(dxgi_factory)
                .map(|v| Self(v))
        }
    }

    pub fn create_swap_chain(
        &mut self,
        queue: &CommandQueue,
        window_handle: &impl HasRawWindowHandle,
        swap_chain_desc: &SwapChainDesc1,
    ) -> crate::Result<SwapChain> {
        unsafe {
            let desc = swap_chain_desc.clone();
            let desc = transmute(desc);
            let window_handle = window_handle.raw_window_handle();

            // Create the swapchain
            let swapchain = match window_handle {
                RawWindowHandle::Windows(hwnd) => {
                    let hwnd = hwnd.hwnd.unwrap().as_ptr() as isize;
                    let hwnd = HWND(hwnd);
                    self.create_swap_chain_for_hwnd(queue, hwnd, desc)?
                }
                RawWindowHandle::WinRT(core_window) => {
                    let core_window = core_window.core_window.unwrap();
                    let core_window: IInspectable = std::mem::transmute(core_window);
                    self.create_swap_chain_for_core_window(queue, core_window, desc)?
                }
                _ => panic!("Unsupported window handle"),
            };

            Ok(SwapChain(swapchain))
        }
    }

    unsafe fn create_swap_chain_for_hwnd(
        &mut self,
        queue: &CommandQueue,
        hwnd: HWND,
        desc: DXGI_SWAP_CHAIN_DESC1,
    ) -> crate::Result<IDXGISwapChain4> {
        let mut swapchain: Option<IDXGISwapChain1> = None;
        let swapchain = self
            .0
            .CreateSwapChainForHwnd(
                queue.get_raw_shared().deref(),
                hwnd,
                &desc,
                std::ptr::null(),
                None,
                &mut swapchain,
            )
            .and_some(swapchain)?;
        swapchain.cast::<IDXGISwapChain4>()
    }

    unsafe fn create_swap_chain_for_core_window(
        &mut self,
        queue: &CommandQueue,
        core_window: IInspectable,
        desc: DXGI_SWAP_CHAIN_DESC1,
    ) -> crate::Result<IDXGISwapChain4> {
        let mut swapchain: Option<IDXGISwapChain1> = None;
        let swapchain = self
            .0
            .CreateSwapChainForCoreWindow(
                queue.get_raw_shared().deref(),
                core_window,
                &desc,
                None,
                &mut swapchain,
            )
            .and_some(swapchain)?;
        swapchain.cast::<IDXGISwapChain4>()
    }

    pub fn enumerate_adapters(&mut self, i: u32) -> crate::Result<Adapter> {
        unsafe { Self::enum_edapter_old(&self.0, i).map(|v| Adapter(v)) }
    }

    pub fn select_hardware_adapter(
        &mut self,
        minimum_feature_level: FeatureLevel,
    ) -> Option<Adapter> {
        unsafe {
            // If possible we can explicitly ask for a "high performance" device.
            let factory_6 = self.0.cast::<IDXGIFactory6>().ok();

            let create_fn = *crate::dx12::device::CREATE_FN
                .get()
                .expect("Failed to load dxgi.dll");

            // Loop over all the available adapters
            let mut i = 0;
            loop {
                // Use the newest available interface to enumerate the adapter
                let adapter = if let Some(factory) = factory_6.as_ref() {
                    Self::enum_edapter_new(factory, i)
                } else {
                    Self::enum_edapter_old(&self.0, i)
                };

                // Check if we've gotten an adapter, or break from the loop if we don't as we've either hit
                // a big error or enumerated all of them already
                if let Ok(adapter) = adapter {
                    // Get the adapter description so we can decide if we want to use it or not
                    let mut desc = DXGI_ADAPTER_DESC1::default();
                    adapter.GetDesc1(&mut desc).unwrap();

                    // We want to skip software adapters as they're going to be *very* slow
                    if (desc.flags & DXGI_ADAPTER_FLAG::DXGI_ADAPTER_FLAG_SOFTWARE.0) != 0 {
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
    unsafe fn enum_edapter_new(factory: &IDXGIFactory6, i: u32) -> crate::Result<IDXGIAdapter1> {
        let mut ppv_adapter: Option<IDXGIAdapter1> = None;
        factory
            .EnumAdapterByGpuPreference(
                i,
                DXGI_GPU_PREFERENCE::DXGI_GPU_PREFERENCE_HIGH_PERFORMANCE,
                &IDXGIAdapter1::IID,
                ppv_adapter.set_abi(),
            )
            .and_some(ppv_adapter)
    }

    unsafe fn enum_edapter_old(factory: &IDXGIFactory2, i: u32) -> crate::Result<IDXGIAdapter1> {
        let mut pp_adapter: Option<IDXGIAdapter1> = None;
        factory
            .EnumAdapters1(i, &mut pp_adapter)
            .and_some(pp_adapter)
    }
}
