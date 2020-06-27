//
//
// This file is a part of Aleph
//
// <ALEPH_REPO_REPLACE>
//
// <ALEPH_LICENSE_REPLACE>
//

#[cfg(target_os = "android")]
fn init_internal() {
    use android_logger::Config;
    android_logger::init_once(Config::default().with_min_level(log::Level::Trace));
}

#[cfg(any(target_os = "windows", target_os = "linux"))]
fn init_internal() {
    env_logger::Builder::from_default_env()
        .filter(None, log::LevelFilter::Trace)
        .init();
}

///
/// Initializes whatever log backend is being used for the platform
///
pub fn init() {
    init_internal();
}
