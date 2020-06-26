//
//
// This file is a part of Aleph
//
// <ALEPH_REPO_REPLACE>
//
// <ALEPH_LICENSE_REPLACE>
//

///
/// Represents the errors that could be produced when rebuilding a swapchain
///
#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Debug)]
pub enum RebuildError {
    ExtentsZero,
}
