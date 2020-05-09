//
//
// This file is a part of Aleph
//
// <ALEPH_REPO_REPLACE>
//
// <ALEPH_LICENSE_REPLACE>
//

///
/// A user's main interface with Aleph. This represents the life cycle of the entire application.
///
pub trait AppLogic: Send {
    ///
    /// This method will be called once on application start up. Anything that needs to be called
    /// once for the entire life cycle of the application goes here.
    ///
    /// This will be called after every engine subsystem has been initialized, right before
    /// `on_update` would have been called for the first time. Trust that the engine is fully
    /// initialized by the time this is called.
    ///
    fn on_init(&mut self);

    ///
    /// This method will be called once per frame. Prior to rendering and before any other `Logic`
    /// instances.
    ///
    fn on_update(&mut self);

    ///
    /// This method will be called once on application exit. This would usually be used for closing
    /// resources and de-initializing whatever had been done in `on_init`.
    ///
    /// This will be called before any engine subsystems begin shutdown so it can be trusted that
    /// they are still safe to use.
    ///
    fn on_exit(&mut self);
}

///
/// The entry point for Aleph.
///
pub struct Engine {}

impl Engine {
    ///
    /// This function is **THE** entry point for Aleph. It all begins here. This manages setting up
    /// a window or drawing surface, setting up input, asset systems, rendering, scripting, etc.
    ///
    /// Once everything is set up it hands
    ///
    pub fn start(mut app: impl AppLogic) {
        // First thing we do is initialize the log backend so everything can log from now on
        crate::logger::init();

        log::info!("Entering into AppLogic::on_init");
        app.on_init();
        log::info!("Exiting AppLogic::on_init");

        app.on_update();

        log::info!("Entering into AppLogic::on_exit");
        app.on_exit();
        log::info!("Exiting AppLogic::on_exit");
    }
}
