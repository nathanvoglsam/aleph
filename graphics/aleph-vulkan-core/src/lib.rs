//
//
// This file is a part of Aleph
//
// <ALEPH_REPO_REPLACE>
//
// <ALEPH_LICENSE_REPLACE>
//

pub extern crate erupt;

extern crate aleph_app_info as app_info;
extern crate aleph_macros as macros;
extern crate console;
extern crate crossbeam;
extern crate log;
extern crate raw_window_handle;

pub(crate) mod debug;
pub mod defer;
pub(crate) mod surface;

mod device;
mod gpu_info;
mod instance;
mod queue_family;
mod swapchain;
mod vendor;

pub use device::Device;
pub use device::DeviceBuilder;
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
