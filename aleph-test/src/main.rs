//
//
// This file is a part of Aleph
//
// <ALEPH_REPO_REPLACE>
//
// <ALEPH_LICENSE_REPLACE>
//

extern crate aleph_engine as aleph;

use aleph::app::AppInfo;

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
    let app_info = AppInfo {
        name: "AlephTest".to_string(),
        major: 0,
        minor: 1,
        patch: 0,
    };
    aleph::app::Engine::start(app_info, AlephAppLogic::new());
}
