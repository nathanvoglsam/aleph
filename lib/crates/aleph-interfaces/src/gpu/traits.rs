use crate::gpu::{
    AcquireImageError, AdapterDescription, AdapterRequestOptions, BackendAPI, BufferCreateError,
    BufferDesc, ClearValue, CommandListBeginError, CommandListCreateError, CommandListSubmitError,
    CommandPoolCreateError, ContextCreateError, ContextOptions, DrawIndexedOptions, DrawOptions,
    QueueType, RequestDeviceError, ShaderCreateError, ShaderOptions, ShaderType,
    SurfaceCreateError, SwapChainConfiguration, SwapChainCreateError, TextureCreateError,
    TextureDesc, TextureSubresourceSet,
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

pub trait ISwapChain: INamedObject + Any + 'static {
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

pub trait IDevice: INamedObject + Send + Sync + Any + 'static {
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

    fn create_command_pool(&self) -> Result<RefPtr<dyn ICommandPool>, CommandPoolCreateError>;

    ///
    /// # Safety
    ///
    /// It is the caller's responsibility to ensure that the command lists submitted to the GPU
    /// contain a valid command stream.
    ///
    /// The GPU interfaces will uphold resource lifetime requirements and CPU synchronization
    /// requirements, but makes a very limited effort to handle GPU synchronization. It is up to the
    /// caller to record correct barriers.
    ///
    unsafe fn general_queue_submit_list(
        &self,
        command_list: Box<dyn IGeneralCommandList>,
    ) -> Result<(), CommandListSubmitError>;

    ///
    /// # Safety
    ///
    /// It is the caller's responsibility to ensure that the command lists submitted to the GPU
    /// contain a valid command stream.
    ///
    /// The GPU interfaces will uphold resource lifetime requirements and CPU synchronization
    /// requirements, but makes a very limited effort to handle GPU synchronization. It is up to the
    /// caller to record correct barriers.
    ///
    unsafe fn general_queue_submit_lists(
        &self,
        command_lists: &mut dyn Iterator<Item = Box<dyn IGeneralCommandList>>,
    ) -> Result<(), CommandListSubmitError>;

    /// Returns the API used by the underlying backend implementation.
    fn get_backend_api(&self) -> BackendAPI;
}

pub trait IVertexInputLayout: INamedObject + Send + Sync + Any + 'static {}

pub trait IBuffer: INamedObject + Send + Sync + Any + 'static {
    fn desc(&self) -> &BufferDesc;
}

pub trait ITexture: INamedObject + Send + Sync + Any + 'static {
    fn desc(&self) -> &TextureDesc;
}

pub trait IShader: INamedObject + Send + Sync + Any + 'static {
    fn shader_type(&self) -> ShaderType;
    fn entry_point(&self) -> &str;
}

pub trait ISampler: INamedObject + Send + Sync + Any + 'static {}

pub trait IFramebufferLayout: INamedObject + Send + Sync + Any + 'static {}

pub trait IFramebuffer: INamedObject + Send + Sync + Any + 'static {}

pub trait IBindingLayout: INamedObject + Send + Sync + Any + 'static {}

pub trait IBindingSet: INamedObject + Send + Sync + Any + 'static {}

pub trait IGraphicsPipeline: INamedObject + Send + Sync + Any + 'static {}

pub trait IComputePipeline: INamedObject + Send + Sync + Any + 'static {}

pub trait ICommandPool: INamedObject + Send + Sync + Any + 'static {
    fn create_general_command_list(
        &self,
    ) -> Result<Box<dyn IGeneralCommandList>, CommandListCreateError>;
}

pub trait IGeneralCommandList: IAny + INamedObject + Send + 'static {
    fn begin<'a>(&'a mut self) -> Result<Box<dyn IGeneralEncoder + 'a>, CommandListBeginError>;
}

pub trait IGeneralEncoder: IComputeEncoder + Send {
    fn clear_texture(
        &mut self,
        texture: WeakRefPtr<dyn ITexture>,
        subresources: &TextureSubresourceSet,
        value: ClearValue,
    );
    fn clear_depth_stencil_texture(
        &mut self,
        texture: WeakRefPtr<dyn ITexture>,
        subresources: &TextureSubresourceSet,
        value: ClearValue,
    );

    fn draw(&mut self, options: &DrawOptions);
    fn draw_indexed(&mut self, options: &DrawIndexedOptions);
}

pub trait IComputeCommandList: IAny + INamedObject + Send + 'static {
    fn begin<'a>(&'a mut self) -> Result<Box<dyn IComputeEncoder + 'a>, CommandListBeginError>;
}

pub trait IComputeEncoder: Send {
    fn dispatch(&mut self, group_count_x: u32, group_count_y: u32, group_count_z: u32);
}

pub trait INamedObject {
    fn set_name(&self, name: &str);
}
