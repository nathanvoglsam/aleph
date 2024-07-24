package aleph_rhi;

/**
 * All the supported RHI backends.
 */
enum abstract RhiBackend(String) {
    /** Direct3D 12 **/
    var D3D12 = "d3d12";

    /** Vulkan **/
    var Vulkan = "vulkan";
}

/**
 * Special options specific to the D3D12 backend.
 */
typedef D3D12Options = {}

/**
 * Special options specific to the Vulkan backend.
 */
typedef VulkanOptions = {
    /** Whether to disable sync2 and force the sync2 emulation path on. **/
    var denySync2: Bool;
}

/**
 * Options for configuring the RHI backend selection.
 */
typedef RhiBackendConfig = {
    /** The backend that is preferred. Optional. Defaults to a platform defined value. **/
    var variant: RhiBackend;

    /** Whether the backend specified is a hard requirement rather than a preference. **/
    var required: Bool;

    /** Any options to configure the Vulkan backend, if it is loaded. **/
    var ?vulkan: VulkanOptions;

    /** Any options to configure the D3D12 backend, if it is loaded. **/
    var ?d3d12: VulkanOptions;
}

/**
 * Options for configuring debugging options in the RHI.
 */
typedef RhiDebugConfig = {
    /** Whether to enable RHI and platform validation layers if they are available. **/
    var validation: Bool;

    /** 
     * Whether debuging utilities are allowed to be initialized. Different backends have debug
     * tools only available on dev machines.
     */
    var debug: Bool;
}

/**
 * Collection of all options for configuring the RHI.
 */
typedef RhiConfig = {
    var backend: RhiBackendConfig;
    var debug: RhiDebugConfig;
}
