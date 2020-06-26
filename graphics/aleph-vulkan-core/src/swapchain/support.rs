//
//
// This file is a part of Aleph
//
// <ALEPH_REPO_REPLACE>
//
// <ALEPH_LICENSE_REPLACE>
//

use erupt::extensions::khr_surface::{PresentModeKHR, SurfaceCapabilitiesKHR, SurfaceFormatKHR};

///
/// Simple collection of some information for swapchain support
///
pub struct SwapChainSupport {
    pub capabilities: SurfaceCapabilitiesKHR,
    pub formats: Vec<SurfaceFormatKHR>,
    pub present_modes: Vec<PresentModeKHR>,
}
