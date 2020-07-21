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

bitflags::bitflags! {
    ///
    /// The set of sample flags that can be found in a sample info block
    ///
    pub struct SampleFlags: u8 {
        const LINEAR = 0b0001;
        const EXPONENT = 0b0010;
        const SIGNED = 0b0100;
        const FLOAT = 0b1000;
    }
}
