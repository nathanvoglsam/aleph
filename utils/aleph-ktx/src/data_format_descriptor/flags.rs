//
//
// This file is a part of Aleph
//
// <ALEPH_REPO_REPLACE>
//
// <ALEPH_LICENSE_REPLACE>
//

bitflags::bitflags! {
    ///
    /// The set of DFD flags defined by the DFD spec
    ///
    pub struct DFDFlags: u8 {
        const ALPHA_PREMULTIPLIED = 0b00000001;
    }
}
