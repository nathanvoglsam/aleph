use crate::gpu::{
    AcquireImageError, AdapterDescription, AdapterRequestOptions, ContextCreateError,
    ContextOptions, QueueType, RequestDeviceError, SurfaceCreateError, SwapChainConfiguration,
    SwapChainCreateError, TextureFormat,
};
use any::IAny;
use raw_window_handle::HasRawWindowHandle;
use ref_ptr::{RefPtr, WeakRefPtr};

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
pub trait IContext: 'static {
    /// Create an adapter that suitably meets the requested requirements and preferences specified
    /// by `options`. Will return `None` if no adapter meeting the requirements could be found.
    fn request_adapter(&self, options: &AdapterRequestOptions) -> Option<RefPtr<dyn IAdapter>>;

    /// Create a surface from the provided window handle.
    fn create_surface(
        &self,
        window: &dyn HasRawWindowHandle,
    ) -> Result<RefPtr<dyn ISurface>, SurfaceCreateError>;
}

/// Represents some GPU device installed in the system. An adapter is used to create an [IDevice].
pub trait IAdapter: 'static {
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
pub trait ISurface: 'static {
    fn create_swap_chain(
        &self,
        device: WeakRefPtr<dyn IDevice>,
        config: &SwapChainConfiguration,
    ) -> Result<RefPtr<dyn ISwapChain>, SwapChainCreateError>;
}

pub trait ISwapChain: 'static {
    /// Returns whether support operations are supported on the given queue.
    fn present_supported_on_queue(&self, queue: QueueType) -> bool;

    /// Force a resize of the swap chain. Will block until the swap chain is no longer in use before
    /// performing the resize operation.
    fn queue_resize(&self, width: u32, height: u32);

    /// Acquire an image from the swap chain for use with rendering
    fn acquire_image(&self) -> Result<RefPtr<dyn ISwapTexture>, AcquireImageError>;
}

pub trait IDevice: Send + Sync + 'static {
    fn garbage_collect(&self);
}

pub trait IVertexInputLayout: Send + Sync + 'static {}

pub trait IBuffer: Send + Sync + 'static {}

pub trait ITexture: Send + Sync + 'static {
    fn size(&self) -> (u32, u32);
    fn width(&self) -> u32;
    fn height(&self) -> u32;
    fn format(&self) -> TextureFormat;
}

pub trait ISwapTexture: ITexture + Send + Sync + 'static {}

pub trait IShader: Send + Sync + 'static {}

pub trait ISampler: Send + Sync + 'static {}

pub trait IFramebufferLayout: Send + Sync + 'static {}

pub trait IFramebuffer: Send + Sync + 'static {}

pub trait IBindingLayout: Send + Sync + 'static {}

pub trait IBindingSet: Send + Sync + 'static {}

pub trait IGraphicsPipeline: Send + Sync + 'static {}

pub trait IComputePipeline: Send + Sync + 'static {}

pub trait ICommandList: Send + Sync + 'static {}
