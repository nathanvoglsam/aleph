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

use interfaces::platform::{HasRawWindowHandle, RawWindowHandle};
use windows::core::{IInspectable, IUnknown, Interface};
use windows::Win32::Foundation::HWND;
use windows::Win32::Graphics::Direct3D12::*;
use windows::Win32::Graphics::Dxgi::*;

pub unsafe fn dxgi_create_swap_chain(
    factory: &IDXGIFactory2,
    queue: &ID3D12CommandQueue,
    window_handle: &impl HasRawWindowHandle,
    swap_chain_desc: &DXGI_SWAP_CHAIN_DESC1,
) -> windows::core::Result<IDXGISwapChain4> {
    let window_handle = window_handle.raw_window_handle();

    // Create the swapchain
    let swapchain = match window_handle {
        RawWindowHandle::Win32(hwnd) => {
            assert!(!hwnd.hwnd.is_null());
            let hwnd = HWND(hwnd.hwnd as isize);
            dxgi_create_swap_chain_for_hwnd(factory, queue, hwnd, swap_chain_desc)?
        }
        RawWindowHandle::WinRt(core_window) => {
            assert!(!core_window.core_window.is_null());
            let core_window = core_window.core_window;
            let core_window: IInspectable = std::mem::transmute(core_window);
            dxgi_create_swap_chain_for_core_window(factory, queue, core_window, swap_chain_desc)?
        }
        _ => panic!("Unsupported window handle"),
    };

    Ok(swapchain)
}

pub unsafe fn dxgi_create_swap_chain_for_hwnd(
    factory: &IDXGIFactory2,
    queue: &ID3D12CommandQueue,
    hwnd: HWND,
    desc: &DXGI_SWAP_CHAIN_DESC1,
) -> windows::core::Result<IDXGISwapChain4> {
    let swapchain = factory.CreateSwapChainForHwnd(queue, hwnd, desc, std::ptr::null(), None)?;
    swapchain.cast::<IDXGISwapChain4>()
}

pub unsafe fn dxgi_create_swap_chain_for_core_window(
    factory: &IDXGIFactory2,
    queue: &ID3D12CommandQueue,
    core_window: IInspectable,
    desc: &DXGI_SWAP_CHAIN_DESC1,
) -> windows::core::Result<IDXGISwapChain4> {
    let core_window = core_window.cast::<IUnknown>()?;
    let swapchain = factory.CreateSwapChainForCoreWindow(queue, &core_window, desc, None)?;
    swapchain.cast::<IDXGISwapChain4>()
}
