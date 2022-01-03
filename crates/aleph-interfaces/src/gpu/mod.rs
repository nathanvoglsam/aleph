//
//
// This file is a part of Aleph
//
// https://github.com/nathanvoglsam/aleph
//
// MIT License
//
// Copyright (c) 2020 Aleph Engine
//
// Permission is hereby granted, free of charge, to any person obtaining a copy
// of this software and associated documentation files (the "Software"), to deal
// in the Software without restriction, including without limitation the rights
// to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
// copies of the Software, and to permit persons to whom the Software is
// furnished to do so, subject to the following conditions:
//
// The above copyright notice and this permission notice shall be included in all
// copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
// AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
// LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
// OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
// SOFTWARE.
//

use any::IAny;
use raw_window_handle::HasRawWindowHandle;
use std::fmt::Debug;

pub const API_VERSION_MAJOR: &'static str = env!("CARGO_PKG_VERSION_MAJOR");
pub const API_VERSION_MINOR: &'static str = env!("CARGO_PKG_VERSION_MINOR");
pub const API_VERSION_PATCH: &'static str = env!("CARGO_PKG_VERSION_PATCH");

/// Entry point of the RHI. This interface is intended to be installed into a plugin registry where
/// some other use can request a handle to the [IContextProvider] instance and create the context.
pub trait IGpuContextProvider: IAny + 'static {
    /// Creates the RHI [IContext] object. This can only succeed once. Calling this more than once
    /// will always return Err.
    fn make_context(
        &self,
        options: &ContextOptions,
    ) -> Result<Box<dyn IGpuContext>, ContextCreateError>;
}

/// Represents the underlying API context. Handles creating surfaces from window handles, and
/// retrieving
///
/// TODO: This doesn't need to be kept alive as the implementation should handle extending the
///       lifetime until all objects are destroyed
pub trait IGpuContext: IAny + Send + 'static {
    /// Create an adapter that suitably meets the requested requirements and preferences specified
    /// by `options`. Will return `None` if no adapter meeting the requirements could be found.
    fn request_adapter(&self, options: &AdapterRequestOptions) -> Option<Box<dyn IGpuAdapter>>;

    /// Create a surface from the provided window handle.
    fn create_surface(&self, window: &dyn HasRawWindowHandle) -> Box<dyn IGpuSurface>;
}

/// Represents some GPU device installed in the system. An adapter is used to create an [IDevice].
pub trait IGpuAdapter: IAny + Send + 'static {
    /// Returns the [AdapterDescription] that provides information about this specific adapter.
    fn description(&mut self) -> AdapterDescription;
    fn request_device(&mut self) -> Result<Box<dyn IGpuDevice>, RequestDeviceError>;
}

pub trait IGpuSurface: IAny + Send + 'static {
    fn create_swap_chain(
        &self,
        device: &dyn IGpuDevice,
        config: &SwapChainConfiguration,
    ) -> Result<Box<dyn IGpuSwapChain>, SwapChainCreateError>;
}

pub trait IGpuSwapChain: IAny + 'static {}

pub trait IGpuDevice: IAny + Send + Sync + 'static {
    fn create_sampler(&self);
}

pub trait IGpuMemoryPool: IAny + 'static {
    fn create_buffer(&self);
    fn create_texture(&self);
}

/// Options provided when a context is created
pub struct ContextOptions {
    /// Whether backend API validation should be enabled.
    ///
    /// Will implicitly force the `debug` option to true if `validation` is also true as on some
    /// backends the `validation` option requires loading the same `debug` utilities to function.
    ///
    /// This flag requests that the backend should enable their backend specific API validation.
    ///
    /// This will add massive amounts of overhead and should never be enabled unless debugging the
    /// backends themselves.
    ///
    /// # Detail
    ///
    /// This is will enable w/e API validation and debug tools that are available to the backend.
    ///
    /// For Vulkan this will enable the validation layers and install a debug messenger the uses
    /// the rust `log` framework.
    ///
    /// For Direct3D 12 this will enable API validation.
    pub validation: bool,

    /// Whether backend debug utilities should be enabled. This enables debug integrations for
    /// naming objects and marking code sections to the backend's API for markup in debug tools.
    ///
    /// # Detail
    ///
    /// Basically just a request to enable `VK_EXT_debug_utils` for Vulkan without enabling
    /// validation layers.
    pub debug: bool,
}

impl Default for ContextOptions {
    fn default() -> Self {
        Self {
            validation: false,
            debug: false,
        }
    }
}

/// Set of errors that can occur when creating an [IContext]
#[derive(Debug)]
pub enum ContextCreateError {
    /// A context has already been created.
    ContextAlreadyCreated,

    /// Some platform error occurred while creating the API context.
    Platform(Box<dyn Debug>),
}

#[derive(Debug)]
pub enum RequestDeviceError {
    /// Some platform error occurred while creating the device.
    Platform(Box<dyn Debug>),
}

pub struct AdapterRequestOptions<'a> {
    /// A handle to an [ISurface] which the device adapter must be able to render and present to.
    ///
    /// Can be set to `None` to indicate we aren't going to present. Useful for compute-only
    /// workloads.
    pub surface: Option<&'a dyn IGpuSurface>,

    /// Specifies the preferred power class of the adapter the context should return. See
    /// [AdapterPowerClass] for the meaning of each power class.
    ///
    /// This only specifies a preference. There is no guarantee that the returned adapter will be
    /// of any particular power class, only that the context will chose the best available match
    /// out of the set of compatible adapters.
    ///
    /// e.g. If a system only has a single dedicated GPU and the preferred power class is low-power
    /// then the context will still yield the dedicated GPU.
    pub power_class: AdapterPowerClass,
}

impl<'a> Default for AdapterRequestOptions<'a> {
    fn default() -> Self {
        Self {
            // We can't make a "default" surface so just default to no surface.
            surface: None,

            // 99.9999% of the time this will be HighPower so we default to that.
            power_class: AdapterPowerClass::HighPower,
        }
    }
}

/// The set of adapter power classes. Primarily used as part of requesting an adapter from the
/// [IContext].
#[derive(Copy, Clone, Debug)]
pub enum AdapterPowerClass {
    /// A low-power adapter refers to the most power efficient GPU installed in the host system.
    ///
    /// e.g. In a laptop with an integrated and discrete GPU, low-power refers to the integrated
    /// GPU as it will almost certainly use less power than the discrete GPU.
    LowPower,

    /// A high-power adapter refers to the highest performance GPU installed in the host system.
    ///
    /// e.g. In a laptop with an integrated and discrete GPU, high-power refers to the discrete GPU
    /// as it will almost certainly be faster than the integrated GPU (otherwise why would it be
    /// installed in the system?).
    HighPower,
}

#[derive(Debug)]
pub struct AdapterDescription<'a> {
    /// The name of the adapter
    pub name: &'a str,
}

#[derive(Debug)]
pub enum SwapChainCreateError {
    UnsupportedFormat(TextureFormat),
    UnsupportedUsage(()),
    UnsupportedWidth(u32),
    UnsupportedHeight(u32),
    UnsupportedPresentMode(PresentationMode),
    Platform(Box<dyn Debug>),
}

pub struct SwapChainConfiguration {
    pub usage: (),
    pub format: TextureFormat,
    pub width: u32,
    pub height: u32,
    pub present_mode: PresentationMode,
}

#[derive(Copy, Clone, Debug)]
pub enum PresentationMode {
    Immediate,
    Mailbox,
    Fifo,
}

#[derive(Copy, Clone, Debug)]
pub enum TextureFormat {
    R8Unorm,
    R8Snorm,
    R8Uint,
    R8Sint,
    R16Uint,
    R16Sint,
    R16Unorm,
    R16Snorm,
    R16Float,
    Rg8Unorm,
    Rg8Snorm,
    Rg8Uint,
    Rg8Sint,
    R32Uint,
    R32Sint,
    R32Float,
    Rg16Uint,
    Rg16Sint,
    Rg16Unorm,
    Rg16Snorm,
    Rg16Float,
    Rgba8Unorm,
    Rgba8UnormSrgb,
    Rgba8Snorm,
    Rgba8Uint,
    Rgba8Sint,
    Bgra8Unorm,
    Bgra8UnormSrgb,
    Rgb10a2Unorm,
    Rg11b10Float,
    Rg32Uint,
    Rg32Sint,
    Rg32Float,
    Rgba16Uint,
    Rgba16Sint,
    Rgba16Unorm,
    Rgba16Snorm,
    Rgba16Float,
    Rgba32Uint,
    Rgba32Sint,
    Rgba32Float,
    Depth32Float,
    Depth24Stencil8,
}
