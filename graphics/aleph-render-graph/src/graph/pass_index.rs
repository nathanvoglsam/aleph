//
//
// This file is a part of Aleph
//
// <ALEPH_REPO_REPLACE>
//
// <ALEPH_LICENSE_REPLACE>
//

///
/// Wrapper type for pass index to make it a little more safe and explicit
///
#[derive(Copy, Clone, Default, PartialOrd, PartialEq, Ord, Eq, Hash, Debug)]
pub struct PassIndex {
    index: usize,
}

impl PassIndex {
    ///
    /// A marker value for specifying an "external" pass. An external pass refers to any use of a
    /// resource outside of the render graph
    ///
    pub const EXTERNAL: PassIndex = Self::new(usize::max_value());

    ///
    /// Creates a new PassIndex
    ///
    pub const fn new(index: usize) -> Self {
        Self { index }
    }

    ///
    /// Function wrapper for checking if this PassIndex refers to an external pass
    ///
    pub fn is_external_pass(self) -> bool {
        self == Self::EXTERNAL
    }

    ///
    /// Wrapper for getting the actual index. Returns `None` if `self == PassIndex::EXTERNAL` as
    /// this is not a valid pass index if used to index into the pass list
    ///
    pub fn get(self) -> Option<usize> {
        if self != Self::EXTERNAL {
            Some(self.index)
        } else {
            None
        }
    }
}
