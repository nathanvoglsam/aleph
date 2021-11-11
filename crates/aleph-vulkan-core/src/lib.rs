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

pub extern crate erupt;
pub extern crate raw_window_handle;

mod app_info;
mod debug;
mod device;
mod engine_info;
mod entry;
mod gpu_info;
mod instance;
mod queue_family;
mod surface;
mod swapchain;
mod vendor;

pub use app_info::AppInfo;
pub use debug::DebugName;
pub use device::Device;
pub use device::DeviceBuilder;
pub use engine_info::EngineInfo;
pub use entry::Entry;
pub use gpu_info::GPUInfo;
pub use instance::Instance;
pub use instance::InstanceBuilder;
pub use queue_family::QueueFamily;
pub use queue_family::QueueFamilyType;
pub use swapchain::AcquireError;
pub use swapchain::RebuildError;
pub use swapchain::SwapChainSupport;
pub use swapchain::SwapImage;
pub use swapchain::Swapchain;
pub use swapchain::SwapchainBuilder;
pub use vendor::VendorID;
