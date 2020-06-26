//
//
// This file is a part of Aleph
//
// <ALEPH_REPO_REPLACE>
//
// <ALEPH_LICENSE_REPLACE>
//

pub extern crate aleph_vma_sys as vma_sys;
pub extern crate spirv_reflect;
pub extern crate erupt;

extern crate aleph_macros as macros;
extern crate aleph_app_info as app_info;
extern crate log;
extern crate console;
extern crate crossbeam;
extern crate once_cell;
extern crate palette;
extern crate parking_lot;
extern crate raw_window_handle;


pub(crate) mod debug;
pub(crate) mod surface;
pub mod defer;

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
pub use swapchain::Swapchain;
pub use swapchain::SwapchainBuilder;
pub use swapchain::SwapImage;
pub use vendor::VendorID;
