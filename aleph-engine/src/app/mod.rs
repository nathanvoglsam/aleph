//
//
// This file is a part of Aleph
//
// <ALEPH_REPO_REPLACE>
//
// <ALEPH_LICENSE_REPLACE>
//


///
///
///
pub trait AppLogic: Send {
    fn on_init(&mut self);

    fn on_update(&mut self);

    fn on_exit(&mut self);
}

///
/// The entry point for Aleph.
///
pub struct Engine {

}

impl Engine {
    pub fn start(app: impl AppLogic) {

    }
}
