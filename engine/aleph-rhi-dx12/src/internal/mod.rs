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

use aleph_rhi_api::*;
use raw_window_handle::{HasWindowHandle, RawWindowHandle};
use utf16_lit::utf16_null;
use windows::Win32::Foundation::{HWND, WAIT_EVENT};
use windows::Win32::Graphics::Direct3D::D3D_FEATURE_LEVEL;
use windows::Win32::Graphics::Direct3D12::*;
use windows::Win32::Graphics::Dxgi::*;
use windows::core::{GUID, IInspectable, IUnknown, Interface, PCWSTR};
use windows::utils::DynamicLoadCell;

pub mod conv;
pub mod debug_interface;
pub mod descriptor_allocator_cpu;
pub mod descriptor_chunk;
pub mod descriptor_heap;
pub mod descriptor_heaps;
pub mod dxgi_debug_interface;
pub mod feature_support;
pub mod graphics_pipeline_state_stream;
pub mod parameter_block;
pub mod parameter_block_pool;
pub mod register_message_callback;
pub mod root_signature_blob;
pub mod sampler_cache;
pub mod unwrap;

pub fn create_device<'a>(
    adapter: impl Into<Option<&'a IDXGIAdapter1>>,
    minimum_feature_level: D3D_FEATURE_LEVEL,
) -> windows::core::Result<ID3D12Device10> {
    pub static CREATE_FN: DynamicLoadCell<PFN_D3D12_CREATE_DEVICE> =
        DynamicLoadCell::new(&utf16_null!("d3d12.dll"), "D3D12CreateDevice\0");

    unsafe {
        let create_fn = CREATE_FN.get().expect("Failed to load d3d12.dll").unwrap();

        // create_fn won't decrement the reference counter if we clone the adapter (which
        // increments the counter). To work around this we use transmute_copy to make a copy without
        // incrementing the reference count. Otherwise we leak a reference to the adapter
        let adapter: Option<IUnknown> = adapter.into().map(|v| std::mem::transmute_copy(v));

        let mut device: Option<ID3D12Device10> = None;

        let ptr = &mut device;
        let ptr = ptr as *mut Option<ID3D12Device10>;
        let ptr = ptr as *mut *mut ::std::ffi::c_void;
        create_fn(adapter, minimum_feature_level, &ID3D12Device10::IID, ptr).map(|| device.unwrap())
    }
}

#[inline]
pub fn create_dxgi_factory(debug: bool) -> windows::core::Result<IDXGIFactory2> {
    type CreateFn = extern "system" fn(
        u32,
        *const GUID,
        *mut *mut ::std::ffi::c_void,
    ) -> windows::core::HRESULT;

    pub static CREATE_FN: DynamicLoadCell<CreateFn> =
        DynamicLoadCell::new(&utf16_null!("dxgi.dll"), "CreateDXGIFactory2\0");

    unsafe {
        let create_fn = *CREATE_FN.get().expect("Failed to load dxgi.dll");

        let flags = if debug { 0x1 } else { 0x0 };

        let mut dxgi_factory: Option<IDXGIFactory2> = None;

        let ptr = &mut dxgi_factory;
        let ptr = ptr as *mut Option<IDXGIFactory2>;
        let ptr = ptr as *mut *mut ::std::ffi::c_void;

        create_fn(flags, &IDXGIFactory2::IID, ptr).map(|| dxgi_factory.unwrap())
    }
}

pub const fn calc_subresource_index(
    mip_level: u32,
    array_layer: u32,
    plane_slice: u32,
    mip_levels: u32,
    array_size: u32,
) -> u32 {
    mip_level + (array_layer * mip_levels) + (plane_slice * mip_levels * array_size)
}

pub const fn plane_layer_for_aspect(format: Format, aspect: TextureCopyAspect) -> Option<u32> {
    match format {
        Format::Depth32Float => match aspect {
            TextureCopyAspect::Depth => Some(0),
            _ => None,
        },
        Format::Depth24Stencil8 | Format::Depth32FloatStencil8 => match aspect {
            TextureCopyAspect::Color => None,
            TextureCopyAspect::Depth => Some(0),
            TextureCopyAspect::Stencil => Some(1),
        },
        _ => match aspect {
            TextureCopyAspect::Color => Some(0),
            TextureCopyAspect::Depth => None,
            TextureCopyAspect::Stencil => None,
        },
    }
}

pub const fn plane_layer_for_aspect_flag(format: Format, aspect: TextureAspect) -> Option<u32> {
    match format {
        Format::Depth32Float => {
            if aspect.contains(TextureAspect::DEPTH) {
                Some(0)
            } else {
                None
            }
        }
        Format::Depth24Stencil8 | Format::Depth32FloatStencil8 => {
            if aspect.contains(TextureAspect::DEPTH) {
                Some(0)
            } else if aspect.contains(TextureAspect::STENCIL) {
                Some(1)
            } else {
                None
            }
        }
        _ => {
            if aspect.contains(TextureAspect::COLOR) {
                Some(0)
            } else {
                None
            }
        }
    }
}

///
/// Handle result from a `WaitForSingleObject` call. Will panic on error.
///
/// - Returns `true` if the `WaitForSingleObject` completed
/// - Returns `false` if the `WaitForSingleObject` timed out
///
/// # Safety
///
/// Calls `GetLastError` internally on error
///
pub unsafe fn handle_wait_result(result: WAIT_EVENT) -> bool {
    unsafe {
        use windows::Win32::Foundation::*;

        // Successfully waited on the event
        if result == WAIT_OBJECT_0 {
            return true;
        }

        // Timeout is an error as we're supposed to block until the event is signalled
        if result == WAIT_TIMEOUT {
            return false;
        }

        // Handle the error case
        if result == WAIT_FAILED {
            GetLastError().to_hresult().unwrap();
            unreachable!("WaitForSingleObject failed");
        }

        // This shouldn't even be possible to observe as the event is thread-local so can't
        // event be observed across threads. But handle it anyway as you never know
        if result == WAIT_ABANDONED {
            panic!("Event was abandoned by owning thread");
        }

        unreachable!("Unexpected result value");
    }
}

pub fn adapter_description_string(desc: &DXGI_ADAPTER_DESC1) -> Option<String> {
    let sub_slice = desc.Description.split(|v| *v == 0).next()?;
    String::from_utf16(sub_slice).ok()
}

pub unsafe fn dxgi_create_swap_chain(
    factory: &IDXGIFactory2,
    queue: &ID3D12CommandQueue,
    window_handle: &impl HasWindowHandle,
    swap_chain_desc: &DXGI_SWAP_CHAIN_DESC1,
) -> windows::core::Result<IDXGISwapChain4> {
    unsafe {
        let window_handle = window_handle.window_handle().unwrap().as_raw();

        // Create the swapchain
        let swapchain = match window_handle {
            RawWindowHandle::Win32(hwnd) => {
                let hwnd = HWND(hwnd.hwnd.get() as *mut _);
                dxgi_create_swap_chain_for_hwnd(factory, queue, hwnd, swap_chain_desc)?
            }
            RawWindowHandle::WinRt(core_window) => {
                let core_window = core_window.core_window;
                let core_window: IInspectable = std::mem::transmute(core_window);
                dxgi_create_swap_chain_for_core_window(
                    factory,
                    queue,
                    core_window,
                    swap_chain_desc,
                )?
            }
            _ => panic!("Unsupported window handle"),
        };

        Ok(swapchain)
    }
}

pub unsafe fn dxgi_create_swap_chain_for_hwnd(
    factory: &IDXGIFactory2,
    queue: &ID3D12CommandQueue,
    hwnd: HWND,
    desc: &DXGI_SWAP_CHAIN_DESC1,
) -> windows::core::Result<IDXGISwapChain4> {
    unsafe {
        let swapchain = factory.CreateSwapChainForHwnd(queue, hwnd, desc, None, None)?;
        swapchain.cast::<IDXGISwapChain4>()
    }
}

pub unsafe fn dxgi_create_swap_chain_for_core_window(
    factory: &IDXGIFactory2,
    queue: &ID3D12CommandQueue,
    core_window: IInspectable,
    desc: &DXGI_SWAP_CHAIN_DESC1,
) -> windows::core::Result<IDXGISwapChain4> {
    unsafe {
        let core_window = core_window.cast::<IUnknown>()?;
        let swapchain = factory.CreateSwapChainForCoreWindow(queue, &core_window, desc, None)?;
        swapchain.cast::<IDXGISwapChain4>()
    }
}

pub fn set_name<'a, T: Into<&'a ID3D12Object>>(object: T, name: &str) -> windows::core::Result<()> {
    let object = object.into();

    let utf16: Vec<u16> = name.encode_utf16().chain(std::iter::once(0)).collect();
    let name = PCWSTR::from_raw(utf16.as_ptr());

    // Safety: this function is internally synchronized
    unsafe { object.SetName(name) }
}
