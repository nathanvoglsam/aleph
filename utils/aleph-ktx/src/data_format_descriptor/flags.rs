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

impl SampleFlags {
    pub fn compatible_with(&self, other: SampleFlags) -> bool {
        let signed_match = self.contains(SampleFlags::SIGNED) == other.contains(SampleFlags::SIGNED);
        let float_match = self.contains(SampleFlags::FLOAT) == other.contains(SampleFlags::FLOAT);
        let exp_match = self.contains(SampleFlags::EXPONENT) == other.contains(SampleFlags::EXPONENT);
        signed_match && float_match && exp_match
    }
}

impl Default for SampleFlags {
    fn default() -> Self {
        Self::empty()
    }
}
