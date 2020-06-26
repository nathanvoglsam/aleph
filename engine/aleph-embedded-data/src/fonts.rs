//
//
// This file is a part of Aleph
//
// <ALEPH_REPO_REPLACE>
//
// <ALEPH_LICENSE_REPLACE>
//

///
/// Embedded bytes of CascadiaCode.ttf
///
pub fn cascadia_code() -> &'static [u8] {
    include_bytes!("../fonts/CascadiaCode.ttf")
}

///
/// Embedded bytes of JetBrainsMono-Regular.ttf
///
pub fn jetbrains_mono_regular() -> &'static [u8] {
    include_bytes!("../fonts/JetBrainsMono-Regular.ttf")
}
