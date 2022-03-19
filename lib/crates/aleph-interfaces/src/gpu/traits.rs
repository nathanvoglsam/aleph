use crate::gpu::{
    AcquireImageError, AdapterDescription, AdapterRequestOptions, BackendAPI, BufferCreateError,
    BufferDesc, ClearValue, ContextCreateError, ContextOptions, DrawOptions, QueueType,
    RequestDeviceError, ShaderCreateError, ShaderOptions, ShaderType, SurfaceCreateError,
    SwapChainConfiguration, SwapChainCreateError, TextureCreateError, TextureDesc,
};
use any::IAny;
use raw_window_handle::HasRawWindowHandle;
use ref_ptr::{RefPtr, WeakRefPtr};
use std::any::Any;

/// Entry point of the RHI. This interface is intended to be installed into a plugin registry where
/// some other use can request a handle to the [IContextProvider] instance and create the context.
pub trait IContextProvider: IAny + 'static {
    /// Creates the RHI [IContext] object. This can only succeed once. Calling this more than once
    /// will always return Err.
    fn make_context(
        &self,
        options: &ContextOptions,
    ) -> Result<RefPtr<dyn IContext>, ContextCreateError>;
}

/// Represents the underlying API context. Handles creating surfaces from window handles, and
/// retrieving.
pub trait IContext: Any + 'static {
    /// Create an adapter that suitably meets the requested requirements and preferences specified
    /// by `options`. Will return `None` if no adapter meeting the requirements could be found.
    fn request_adapter(&self, options: &AdapterRequestOptions) -> Option<RefPtr<dyn IAdapter>>;

    /// Create a surface from the provided window handle.
    fn create_surface(
        &self,
        window: &dyn HasRawWindowHandle,
    ) -> Result<RefPtr<dyn ISurface>, SurfaceCreateError>;

    /// Returns the API used by the underlying backend implementation.
    fn get_backend_api(&self) -> BackendAPI;
}

/// Represents some GPU device installed in the system. An adapter is used to create an [IDevice].
pub trait IAdapter: Any + 'static {
    /// Returns the [AdapterDescription] that provides information about this specific adapter.
    fn description(&self) -> AdapterDescription;

    /// Requests an IDevice
    fn request_device(&self) -> Result<RefPtr<dyn IDevice>, RequestDeviceError>;
}

/// Represents the graphics API's handle to the window or monitor surface. SwapChains are created
/// from surfaces.
///
/// A surface is not tied to a specific [IDevice], it represents an API level handle to a rendering
/// surface. As such [ISurface] is not created by an [IDevice], rather it is created by the
/// [IContext]. An [IDevice] will be selected and created based on its compatibility with an
/// [ISurface].
pub trait ISurface: Any + 'static {
    fn create_swap_chain(
        &self,
        device: WeakRefPtr<dyn IDevice>,
        config: &SwapChainConfiguration,
    ) -> Result<RefPtr<dyn ISwapChain>, SwapChainCreateError>;
}

pub trait ISwapChain: Any + 'static {
    /// Returns whether support operations are supported on the given queue.
    fn present_supported_on_queue(&self, queue: QueueType) -> bool;

    /// Force a resize of the swap chain. Will block until the swap chain is no longer in use before
    /// performing the resize operation.
    fn queue_resize(&self, width: u32, height: u32);

    /// Returns a [SwapChainConfiguration] that describes the state of the swap chain at the time
    /// of the function being called.
    ///
    /// The state may change after this function is called. If a rebuild was needed internally in
    /// [ISwapChain::acquire_image] then the size may be different once the
    /// [ISwapChain::acquire_image] call returns.
    fn get_config(&self) -> SwapChainConfiguration;

    /// Acquire an image from the swap chain for use with rendering
    fn acquire_image(&self) -> Result<RefPtr<dyn ITexture>, AcquireImageError>;
}

pub trait IDevice: Send + Sync + Any + 'static {
    fn garbage_collect(&self);

    fn create_shader(
        &self,
        options: &ShaderOptions,
    ) -> Result<RefPtr<dyn IShader>, ShaderCreateError>;

    fn create_buffer(&self, desc: &BufferDesc) -> Result<RefPtr<dyn IBuffer>, BufferCreateError>;

    fn create_texture(
        &self,
        desc: &TextureDesc,
    ) -> Result<RefPtr<dyn ITexture>, TextureCreateError>;

    /// Returns the API used by the underlying backend implementation.
    fn get_backend_api(&self) -> BackendAPI;
}

pub trait IVertexInputLayout: Send + Sync + Any + 'static {}

pub trait IBuffer: Send + Sync + Any + 'static {
    fn desc(&self) -> &BufferDesc;
}

pub trait ITexture: Send + Sync + Any + 'static {
    fn desc(&self) -> &TextureDesc;
}

pub trait IShader: Send + Sync + Any + 'static {
    fn shader_type(&self) -> ShaderType;
    fn entry_point(&self) -> &str;
}

pub trait ISampler: Send + Sync + Any + 'static {}

pub trait IFramebufferLayout: Send + Sync + Any + 'static {}

pub trait IFramebuffer: Send + Sync + Any + 'static {}

pub trait IBindingLayout: Send + Sync + Any + 'static {}

pub trait IBindingSet: Send + Sync + Any + 'static {}

pub trait IGraphicsPipeline: Send + Sync + Any + 'static {}

pub trait IComputePipeline: Send + Sync + Any + 'static {}

pub trait ICommandPool: Send + Sync + Any + 'static {
    fn begin(&self) -> Result<Box<dyn IEncoder>, ()>;
}

pub trait IEncoder: Any + 'static {
    fn clear_texture(&self, texture: WeakRefPtr<dyn ITexture>, clear_color: ClearValue);
    fn clear_depth_stencil_texture(&self, texture: WeakRefPtr<dyn ITexture>, values: ClearValue);

    fn draw(&self, options: &DrawOptions);
    fn draw_indexed(&self, options: &DrawOptions);
}
