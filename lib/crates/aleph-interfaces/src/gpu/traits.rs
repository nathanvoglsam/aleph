use crate::gpu::{
    AcquireImageError, AdapterDescription, AdapterRequestOptions, BackendAPI, BufferCreateError,
    BufferDesc, ColorClearValue, CommandListBeginError, CommandListCreateError,
    CommandListSubmitError, CommandPoolCreateError, ContextCreateError, ContextOptions,
    DepthStencilClearValue, DrawIndexedOptions, DrawOptions, QueuePresentError, QueueType,
    RequestDeviceError, SamplerDesc, ShaderCreateError, ShaderOptions, ShaderType,
    SurfaceCreateError, SwapChainConfiguration, SwapChainCreateError, TextureCreateError,
    TextureDesc, TextureSubResourceSet,
};
use any::{AnyArc, IAny};
use raw_window_handle::HasRawWindowHandle;
use std::any::Any;

/// Entry point of the RHI. This interface is intended to be installed into a plugin registry where
/// some other use can request a handle to the [IContextProvider] instance and create the context.
pub trait IContextProvider: IAny + 'static {
    /// Creates the RHI [IContext] object. This can only succeed once. Calling this more than once
    /// will always return Err.
    fn make_context(
        &self,
        options: &ContextOptions,
    ) -> Result<AnyArc<dyn IContext>, ContextCreateError>;
}

/// Represents the underlying API context. Handles creating surfaces from window handles, and
/// retrieving.
pub trait IContext: IAny + 'static {
    fn upgrade(&self) -> AnyArc<dyn IContext>;

    /// Create an adapter that suitably meets the requested requirements and preferences specified
    /// by `options`. Will return `None` if no adapter meeting the requirements could be found.
    fn request_adapter(&self, options: &AdapterRequestOptions) -> Option<AnyArc<dyn IAdapter>>;

    /// Create a surface from the provided window handle.
    fn create_surface(
        &self,
        window: &dyn HasRawWindowHandle,
    ) -> Result<AnyArc<dyn ISurface>, SurfaceCreateError>;

    /// Returns the API used by the underlying backend implementation.
    fn get_backend_api(&self) -> BackendAPI;
}

/// Represents some GPU device installed in the system. An adapter is used to create an [IDevice].
pub trait IAdapter: IAny + 'static {
    fn upgrade(&self) -> AnyArc<dyn IAdapter>;

    /// Returns the [AdapterDescription] that provides information about this specific adapter.
    fn description(&self) -> AdapterDescription;

    /// Requests an IDevice
    fn request_device(&self) -> Result<AnyArc<dyn IDevice>, RequestDeviceError>;
}

/// Represents the graphics API's handle to the window or monitor surface. SwapChains are created
/// from surfaces.
///
/// A surface is not tied to a specific [IDevice], it represents an API level handle to a rendering
/// surface. As such [ISurface] is not created by an [IDevice], rather it is created by the
/// [IContext]. An [IDevice] will be selected and created based on its compatibility with an
/// [ISurface].
pub trait ISurface: IAny + 'static {
    fn upgrade(&self) -> AnyArc<dyn ISurface>;

    fn create_swap_chain(
        &self,
        device: &dyn IDevice,
        config: &SwapChainConfiguration,
    ) -> Result<AnyArc<dyn ISwapChain>, SwapChainCreateError>;
}

pub trait ISwapChain: INamedObject + IAny + 'static {
    fn upgrade(&self) -> AnyArc<dyn ISwapChain>;

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
    fn acquire_image(&self) -> Result<Box<dyn IAcquiredTexture>, AcquireImageError>;
}

pub trait IAcquiredTexture: IAny + Send + 'static {
    fn image(&self) -> &dyn ITexture;
}

pub trait IDevice: INamedObject + Send + Sync + IAny + Any + 'static {
    fn upgrade(&self) -> AnyArc<dyn IDevice>;

    /// Triggers a non blocking garbage collection cycle. This must be called for resources used in
    /// command lists to be freed. It is recommended to call this at least once per frame.
    fn garbage_collect(&self);

    /// Block the calling thread until all GPU queues are flushed of work. This is similar to
    /// vkDeviceWaitIdle.
    ///
    /// This will also trigger a GC cycle, freeing the releases from the now completed command
    /// lists.
    fn wait_idle(&self);

    fn create_shader(
        &self,
        options: &ShaderOptions,
    ) -> Result<AnyArc<dyn IShader>, ShaderCreateError>;

    fn create_buffer(&self, desc: &BufferDesc) -> Result<AnyArc<dyn IBuffer>, BufferCreateError>;

    fn create_texture(
        &self,
        desc: &TextureDesc,
    ) -> Result<AnyArc<dyn ITexture>, TextureCreateError>;

    fn create_sampler(&self, desc: &SamplerDesc) -> Result<AnyArc<dyn ISampler>, ()>;

    fn create_command_pool(&self) -> Result<AnyArc<dyn ICommandPool>, CommandPoolCreateError>;

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

    ///
    /// # Safety
    ///
    /// It is the caller's responsibility to ensure that the image that is being presented will be
    /// in the required resource state for presentation by the time this operation will be executed
    /// on the GPU timeline.
    ///
    unsafe fn general_queue_present(
        &self,
        image: Box<dyn IAcquiredTexture>,
    ) -> Result<(), QueuePresentError>;

    /// Returns the API used by the underlying backend implementation.
    fn get_backend_api(&self) -> BackendAPI;
}

pub trait IVertexInputLayout: INamedObject + Send + Sync + IAny + Any + 'static {
    fn upgrade(&self) -> AnyArc<dyn IVertexInputLayout>;
}

pub trait IBuffer: INamedObject + Send + Sync + IAny + Any + 'static {
    fn upgrade(&self) -> AnyArc<dyn IBuffer>;
    fn desc(&self) -> &BufferDesc;
}

pub trait ITexture: INamedObject + Send + Sync + IAny + Any + 'static {
    fn upgrade(&self) -> AnyArc<dyn ITexture>;
    fn desc(&self) -> &TextureDesc;
}

pub trait IShader: INamedObject + Send + Sync + IAny + Any + 'static {
    fn upgrade(&self) -> AnyArc<dyn IShader>;
    fn shader_type(&self) -> ShaderType;
    fn entry_point(&self) -> &str;
}

pub trait ISampler: INamedObject + Send + Sync + IAny + Any + 'static {
    fn upgrade(&self) -> AnyArc<dyn ISampler>;
}

pub trait IFramebufferLayout: INamedObject + Send + Sync + IAny + Any + 'static {
    fn upgrade(&self) -> AnyArc<dyn IFramebufferLayout>;
}

pub trait IFramebuffer: INamedObject + Send + Sync + IAny + Any + 'static {
    fn upgrade(&self) -> AnyArc<dyn IFramebuffer>;
}

pub trait IBindingLayout: INamedObject + Send + Sync + IAny + Any + 'static {
    fn upgrade(&self) -> AnyArc<dyn IBindingLayout>;
}

pub trait IBindingSet: INamedObject + Send + Sync + IAny + Any + 'static {
    fn upgrade(&self) -> AnyArc<dyn IBindingSet>;
}

pub trait IGraphicsPipeline: INamedObject + Send + Sync + IAny + Any + 'static {
    fn upgrade(&self) -> AnyArc<dyn IGraphicsPipeline>;
}

pub trait IComputePipeline: INamedObject + Send + Sync + IAny + Any + 'static {
    fn upgrade(&self) -> AnyArc<dyn IComputePipeline>;
}

pub trait ICommandPool: INamedObject + Send + Sync + IAny + Any + 'static {
    fn upgrade(&self) -> AnyArc<dyn ICommandPool>;
    fn create_general_command_list(
        &self,
    ) -> Result<Box<dyn IGeneralCommandList>, CommandListCreateError>;
}

pub trait IGeneralCommandList: INamedObject + Send + IAny + Any + 'static {
    fn begin<'a>(&'a mut self) -> Result<Box<dyn IGeneralEncoder + 'a>, CommandListBeginError>;
}

pub trait IGeneralEncoder: IComputeEncoder + Send {
    fn clear_texture(
        &mut self,
        texture: &dyn ITexture,
        sub_resources: &TextureSubResourceSet,
        value: &ColorClearValue,
    );
    fn clear_depth_stencil_texture(
        &mut self,
        texture: &dyn ITexture,
        sub_resources: &TextureSubResourceSet,
        value: &DepthStencilClearValue,
    );
    fn draw(&mut self, options: &DrawOptions);
    fn draw_indexed(&mut self, options: &DrawIndexedOptions);
}

pub trait IComputeCommandList: INamedObject + Send + IAny + Any + 'static {
    fn begin<'a>(&'a mut self) -> Result<Box<dyn IComputeEncoder + 'a>, CommandListBeginError>;
}

pub trait IComputeEncoder: ITransferEncoder + Send {
    fn dispatch(&mut self, group_count_x: u32, group_count_y: u32, group_count_z: u32);
}

pub trait ITransferCommandList: INamedObject + Send + IAny + Any + 'static {
    fn begin<'a>(&'a mut self) -> Result<Box<dyn ITransferEncoder + 'a>, CommandListBeginError>;
}

pub trait ITransferEncoder: Send {}

pub trait INamedObject {
    fn set_name(&self, name: &str);
}
