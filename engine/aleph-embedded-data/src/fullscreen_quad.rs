//
//
// This file is a part of Aleph
//
// <ALEPH_REPO_REPLACE>
//
// <ALEPH_LICENSE_REPLACE>
//

///
/// A namespace struct for a fullscreen quad
///
pub struct FullscreenQuad {}

impl FullscreenQuad {
    ///
    /// Gets the vertex position buffer for a fullscreen quad
    ///
    pub fn positions() -> &'static [[f32; 2]; 4] {
        static POS: [[f32; 2]; 4] = [[-1.0, -1.0], [1.0, -1.0], [1.0, 1.0], [-1.0, 1.0]];
        &POS
    }

    ///
    /// Gets the index buffer for a fullscreen quad
    ///
    pub fn indices() -> &'static [u16; 6] {
        static IND: [u16; 6] = [0, 1, 2, 3, 0, 2];
        &IND
    }
}
