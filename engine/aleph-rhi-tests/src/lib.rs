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

extern crate aleph_sdl3_link;

mod platform_window;
pub mod swap_chain;

use std::sync::Arc;

use aleph_rhi_api::*;
use aleph_rhi_loader::{BackendConfigs, LoaderOptions, RhiLoader};

use crate::platform_window::{PlatformWindow, SdlWindow};

#[derive(Clone, Debug)]
pub struct SwapChainRequest {
    pub title: String,
    pub width: u32,
    pub height: u32,
    pub format: Format,
    pub present_mode: PresentationMode,
    pub buffer_count: u32,
}

impl Default for SwapChainRequest {
    fn default() -> Self {
        Self {
            title: "aleph-rhi-tests".to_string(),
            width: 640,
            height: 480,
            format: Format::Bgra8Unorm,
            present_mode: PresentationMode::Fifo,
            buffer_count: 3,
        }
    }
}

#[derive(Clone, Debug)]
pub struct HarnessOptions {
    pub backend: BackendAPI,
    pub swap_chain: Option<SwapChainRequest>,
}

impl Default for HarnessOptions {
    fn default() -> Self {
        Self {
            backend: default_backend(),
            swap_chain: None,
        }
    }
}

pub struct WindowState {
    pub swap_chain: Arc<dyn ISwapChain>,
    pub platform: PlatformWindow,
}

pub struct TestHarness {
    pub window_state: Option<WindowState>,
    pub device: Arc<dyn IDevice>,
    pub adapter: Arc<dyn IAdapter>,
    pub context: Arc<dyn IContext>,
}

impl TestHarness {
    pub fn new(options: &HarnessOptions) -> Self {
        let sdl_window = options.swap_chain.as_ref().map(SdlWindow::new);

        let loader = RhiLoader::new();
        let context = loader
            .make_context(&LoaderOptions {
                backend: options.backend,
                options: ContextOptions {
                    profiling: false,
                    debug: true,
                    validation: true,
                },
                config: BackendConfigs::default(),
            })
            .expect("Failed to create RHI context");

        let platform_window =
            sdl_window.map(|window| PlatformWindow::new(context.as_ref(), window));

        let adapter = context
            .request_adapter(&AdapterRequestOptions {
                surface: platform_window.as_ref().map(|v| v.surface.as_ref()),
                ..Default::default()
            })
            .expect("No suitable GPU adapter found");
        let device = adapter
            .request_device()
            .expect("Failed to create GPU device");

        let window_state = platform_window.map(|platform| {
            let request = options
                .swap_chain
                .as_ref()
                .expect("window requires request");
            let (width, height) = platform.window.size_in_pixels();
            let swap_chain = platform
                .surface
                .create_swap_chain(
                    device.as_ref(),
                    &SwapChainConfiguration {
                        format: request.format,
                        width,
                        height,
                        present_mode: request.present_mode,
                        buffer_count: request.buffer_count,
                        present_queue: QueueType::General,
                    },
                )
                .expect("Failed to create swap chain");
            WindowState {
                swap_chain,
                platform,
            }
        });

        Self {
            window_state,
            device,
            adapter,
            context,
        }
    }

    pub fn swap_chain(&self) -> Option<&Arc<dyn ISwapChain>> {
        self.window_state.as_ref().map(|v| &v.swap_chain)
    }
}

/// Returns the backend to test. Overridable with `ALEPH_RHI_BACKEND` (vulkan, d3d12, metal).
/// Without an override, defaults to Vulkan on Linux, D3D12 on Windows, and Metal on macOS.
fn default_backend() -> BackendAPI {
    let backend = std::env::var("ALEPH_RHI_BACKEND").map(|v| v.to_ascii_lowercase());
    match backend.as_deref() {
        Ok("d3d12") => BackendAPI::D3D12,
        Ok("metal") => BackendAPI::Metal,
        Ok("vulkan") => BackendAPI::Vulkan,
        _ => platform_default_backend(),
    }
}

fn platform_default_backend() -> BackendAPI {
    cfg_select! {
        target_os = "windows" => BackendAPI::D3D12,
        any(target_os = "ios", target_os = "macos") => BackendAPI::Metal,
        target_os = "linux" => BackendAPI::Vulkan,
        _ => BackendAPI::Vulkan,
    }
}
