//
//
// This file is a part of Aleph
//
// <ALEPH_REPO_REPLACE>
//
// <ALEPH_LICENSE_REPLACE>
//

///
/// An error enum to represent the ways that a call to `Swapchain::acquire_next` could fail
///
#[derive(Copy, Clone, PartialEq, Debug)]
pub enum AcquireError {
    Timeout,
    OutOfDate,
}
