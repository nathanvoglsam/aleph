//
//
// This file is a part of Aleph
//
// <ALEPH_REPO_REPLACE>
//
// <ALEPH_LICENSE_REPLACE>
//

pub(crate) mod debug;
pub(crate) mod surface;

pub mod alloc;
pub mod pipeline;
pub mod reflect;
pub mod render;

mod device;
mod gpu_info;
mod imgui;
mod instance;
mod pipeline_cache;
mod queue_family;
mod swapchain;
mod vendor;

pub use device::Device;
pub use device::DeviceBuilder;
pub use gpu_info::GPUInfo;
pub use instance::Instance;
pub use instance::InstanceBuilder;
pub use pipeline_cache::PipelineCache;
pub use queue_family::QueueFamily;
pub use queue_family::QueueFamilyType;
pub use swapchain::AcquireError;
pub use swapchain::RebuildError;
pub use swapchain::SwapChainSupport;
pub use swapchain::Swapchain;
pub use swapchain::SwapchainBuilder;
pub use vendor::VendorID;

pub(crate) use self::imgui::ImguiRenderer;
