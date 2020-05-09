//
//
// This file is a part of Aleph
//
// <ALEPH_REPO_REPLACE>
//
// <ALEPH_LICENSE_REPLACE>
//

extern crate aleph_engine as aleph;

struct AlephAppLogic {}

impl AlephAppLogic {
    pub fn new() -> Self {
        Self {}
    }
}

impl aleph::app::AppLogic for AlephAppLogic {
    fn on_init(&mut self) {}

    fn on_update(&mut self) {}

    fn on_exit(&mut self) {}
}

fn main() {
    aleph::app::Engine::start(AlephAppLogic::new());
}
