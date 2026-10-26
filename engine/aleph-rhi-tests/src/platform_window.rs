use std::cell::RefCell;
use std::sync::Arc;

use aleph_rhi_api::*;

use crate::SwapChainRequest;

/// SDL and native window setup, performed before initializing the RHI.
pub struct SdlWindow {
    pub event_pump: RefCell<sdl3::EventPump>,
    pub window: sdl3::video::Window,
    pub video: sdl3::VideoSubsystem,
    pub sdl: sdl3::Sdl,
    #[cfg(any(target_os = "ios", target_os = "macos"))]
    metal_view: MetalView,
}

impl SdlWindow {
    pub fn new(request: &SwapChainRequest) -> Self {
        let sdl = sdl3::init().expect("Failed to initialize SDL3");
        let video = sdl.video().expect("Failed to initialize SDL3 video");
        #[allow(unused_mut)]
        let mut builder = video.window(&request.title, request.width, request.height);
        #[cfg(any(target_os = "ios", target_os = "macos"))]
        builder.metal_view();
        let window = builder.build().expect("Failed to create SDL3 window");

        #[cfg(any(target_os = "ios", target_os = "macos"))]
        let metal_view = MetalView::new(&window);
        let event_pump = RefCell::new(
            sdl.event_pump()
                .expect("Failed to initialize SDL3 event pump"),
        );

        Self {
            event_pump,
            window,
            video,
            sdl,
            #[cfg(any(target_os = "ios", target_os = "macos"))]
            metal_view,
        }
    }
}

pub struct PlatformWindow {
    pub surface: Arc<dyn ISurface>,
    pub event_pump: RefCell<sdl3::EventPump>,
    pub window: sdl3::video::Window,
    pub video: sdl3::VideoSubsystem,
    pub sdl: sdl3::Sdl,
    #[cfg(any(target_os = "ios", target_os = "macos"))]
    _metal_view: MetalView,
}

impl PlatformWindow {
    pub fn new(context: &dyn IContext, sdl_window: SdlWindow) -> Self {
        #[cfg(any(target_os = "ios", target_os = "macos"))]
        let surface = create_surface(context, &sdl_window);
        #[cfg(not(any(target_os = "ios", target_os = "macos")))]
        let surface = create_surface(context, &sdl_window.window);

        Self {
            surface,
            event_pump: sdl_window.event_pump,
            window: sdl_window.window,
            video: sdl_window.video,
            sdl: sdl_window.sdl,
            #[cfg(any(target_os = "ios", target_os = "macos"))]
            _metal_view: sdl_window.metal_view,
        }
    }
}

#[cfg(any(target_os = "ios", target_os = "macos"))]
struct MetalView {
    view: std::ptr::NonNull<std::ffi::c_void>,
    layer: std::ptr::NonNull<std::ffi::c_void>,
}

#[cfg(any(target_os = "ios", target_os = "macos"))]
impl MetalView {
    fn new(window: &sdl3::video::Window) -> Self {
        use sdl3::sys::metal::{SDL_Metal_CreateView, SDL_Metal_GetLayer};

        let view = unsafe {
            std::ptr::NonNull::new(SDL_Metal_CreateView(window.raw()))
                .expect("Failed to create SDL3 Metal view")
        };
        let layer = unsafe { SDL_Metal_GetLayer(view.as_ptr().cast()) };
        let layer =
            std::ptr::NonNull::new(layer).expect("Failed to get CAMetalLayer from SDL3 Metal view");
        Self { view, layer }
    }
}

#[cfg(any(target_os = "ios", target_os = "macos"))]
impl Drop for MetalView {
    fn drop(&mut self) {
        unsafe {
            sdl3::sys::metal::SDL_Metal_DestroyView(self.view.as_ptr().cast());
        }
    }
}

#[cfg(any(target_os = "ios", target_os = "macos"))]
fn create_surface(context: &dyn IContext, window: &SdlWindow) -> Arc<dyn ISurface> {
    context
        .create_surface_for_metal_layer(window.metal_view.layer)
        .expect("Failed to create surface for Metal layer")
}

#[cfg(not(any(target_os = "ios", target_os = "macos")))]
fn create_surface(context: &dyn IContext, window: &sdl3::video::Window) -> Arc<dyn ISurface> {
    context
        .create_surface(window, window)
        .expect("Failed to create surface")
}
