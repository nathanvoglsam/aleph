//
//
// This file is a part of Aleph
//
// <ALEPH_REPO_REPLACE>
//
// <ALEPH_LICENSE_REPLACE>
//

///
/// A struct that holds information about the particular game the engine will be running
///
pub struct AppInfo {
    ///
    /// The name of the App.
    ///
    /// # Info
    ///
    /// This will be the window title
    ///
    pub name: String,

    ///
    /// The major version of the app, following semver conventions
    ///
    pub major: u32,

    ///
    /// The minor version of the app, following semver conventions
    ///
    pub minor: u32,

    ///
    /// The patch version of the app, following semver conventions
    ///
    pub patch: u32,
}

impl AppInfo {
    pub fn log_info(&self) {
        log::info!("=== Game Info ===");
        log::info!("Name    : {}", &self.name);
        log::info!(
            "Version : {}.{}.{}",
            self.major,
            self.minor,
            self.patch
        );
        log::info!("=== Game Info ===");
    }
}
