#![allow(unused_variables, non_upper_case_globals, non_snake_case)]
#[allow(non_camel_case_types)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: marker :: Copy,
    :: std :: clone :: Clone,
    :: std :: default :: Default,
    :: std :: fmt :: Debug,
)]
#[repr(transparent)]
pub struct DXGI_FORMAT(pub u32);
impl DXGI_FORMAT {
    #![allow(non_upper_case_globals)]
    pub const DXGI_FORMAT_UNKNOWN: Self = Self(0u32);
    pub const DXGI_FORMAT_R32G32B32A32_TYPELESS: Self = Self(1u32);
    pub const DXGI_FORMAT_R32G32B32A32_FLOAT: Self = Self(2u32);
    pub const DXGI_FORMAT_R32G32B32A32_UINT: Self = Self(3u32);
    pub const DXGI_FORMAT_R32G32B32A32_SINT: Self = Self(4u32);
    pub const DXGI_FORMAT_R32G32B32_TYPELESS: Self = Self(5u32);
    pub const DXGI_FORMAT_R32G32B32_FLOAT: Self = Self(6u32);
    pub const DXGI_FORMAT_R32G32B32_UINT: Self = Self(7u32);
    pub const DXGI_FORMAT_R32G32B32_SINT: Self = Self(8u32);
    pub const DXGI_FORMAT_R16G16B16A16_TYPELESS: Self = Self(9u32);
    pub const DXGI_FORMAT_R16G16B16A16_FLOAT: Self = Self(10u32);
    pub const DXGI_FORMAT_R16G16B16A16_UNORM: Self = Self(11u32);
    pub const DXGI_FORMAT_R16G16B16A16_UINT: Self = Self(12u32);
    pub const DXGI_FORMAT_R16G16B16A16_SNORM: Self = Self(13u32);
    pub const DXGI_FORMAT_R16G16B16A16_SINT: Self = Self(14u32);
    pub const DXGI_FORMAT_R32G32_TYPELESS: Self = Self(15u32);
    pub const DXGI_FORMAT_R32G32_FLOAT: Self = Self(16u32);
    pub const DXGI_FORMAT_R32G32_UINT: Self = Self(17u32);
    pub const DXGI_FORMAT_R32G32_SINT: Self = Self(18u32);
    pub const DXGI_FORMAT_R32G8X24_TYPELESS: Self = Self(19u32);
    pub const DXGI_FORMAT_D32_FLOAT_S8X24_UINT: Self = Self(20u32);
    pub const DXGI_FORMAT_R32_FLOAT_X8X24_TYPELESS: Self = Self(21u32);
    pub const DXGI_FORMAT_X32_TYPELESS_G8X24_UINT: Self = Self(22u32);
    pub const DXGI_FORMAT_R10G10B10A2_TYPELESS: Self = Self(23u32);
    pub const DXGI_FORMAT_R10G10B10A2_UNORM: Self = Self(24u32);
    pub const DXGI_FORMAT_R10G10B10A2_UINT: Self = Self(25u32);
    pub const DXGI_FORMAT_R11G11B10_FLOAT: Self = Self(26u32);
    pub const DXGI_FORMAT_R8G8B8A8_TYPELESS: Self = Self(27u32);
    pub const DXGI_FORMAT_R8G8B8A8_UNORM: Self = Self(28u32);
    pub const DXGI_FORMAT_R8G8B8A8_UNORM_SRGB: Self = Self(29u32);
    pub const DXGI_FORMAT_R8G8B8A8_UINT: Self = Self(30u32);
    pub const DXGI_FORMAT_R8G8B8A8_SNORM: Self = Self(31u32);
    pub const DXGI_FORMAT_R8G8B8A8_SINT: Self = Self(32u32);
    pub const DXGI_FORMAT_R16G16_TYPELESS: Self = Self(33u32);
    pub const DXGI_FORMAT_R16G16_FLOAT: Self = Self(34u32);
    pub const DXGI_FORMAT_R16G16_UNORM: Self = Self(35u32);
    pub const DXGI_FORMAT_R16G16_UINT: Self = Self(36u32);
    pub const DXGI_FORMAT_R16G16_SNORM: Self = Self(37u32);
    pub const DXGI_FORMAT_R16G16_SINT: Self = Self(38u32);
    pub const DXGI_FORMAT_R32_TYPELESS: Self = Self(39u32);
    pub const DXGI_FORMAT_D32_FLOAT: Self = Self(40u32);
    pub const DXGI_FORMAT_R32_FLOAT: Self = Self(41u32);
    pub const DXGI_FORMAT_R32_UINT: Self = Self(42u32);
    pub const DXGI_FORMAT_R32_SINT: Self = Self(43u32);
    pub const DXGI_FORMAT_R24G8_TYPELESS: Self = Self(44u32);
    pub const DXGI_FORMAT_D24_UNORM_S8_UINT: Self = Self(45u32);
    pub const DXGI_FORMAT_R24_UNORM_X8_TYPELESS: Self = Self(46u32);
    pub const DXGI_FORMAT_X24_TYPELESS_G8_UINT: Self = Self(47u32);
    pub const DXGI_FORMAT_R8G8_TYPELESS: Self = Self(48u32);
    pub const DXGI_FORMAT_R8G8_UNORM: Self = Self(49u32);
    pub const DXGI_FORMAT_R8G8_UINT: Self = Self(50u32);
    pub const DXGI_FORMAT_R8G8_SNORM: Self = Self(51u32);
    pub const DXGI_FORMAT_R8G8_SINT: Self = Self(52u32);
    pub const DXGI_FORMAT_R16_TYPELESS: Self = Self(53u32);
    pub const DXGI_FORMAT_R16_FLOAT: Self = Self(54u32);
    pub const DXGI_FORMAT_D16_UNORM: Self = Self(55u32);
    pub const DXGI_FORMAT_R16_UNORM: Self = Self(56u32);
    pub const DXGI_FORMAT_R16_UINT: Self = Self(57u32);
    pub const DXGI_FORMAT_R16_SNORM: Self = Self(58u32);
    pub const DXGI_FORMAT_R16_SINT: Self = Self(59u32);
    pub const DXGI_FORMAT_R8_TYPELESS: Self = Self(60u32);
    pub const DXGI_FORMAT_R8_UNORM: Self = Self(61u32);
    pub const DXGI_FORMAT_R8_UINT: Self = Self(62u32);
    pub const DXGI_FORMAT_R8_SNORM: Self = Self(63u32);
    pub const DXGI_FORMAT_R8_SINT: Self = Self(64u32);
    pub const DXGI_FORMAT_A8_UNORM: Self = Self(65u32);
    pub const DXGI_FORMAT_R1_UNORM: Self = Self(66u32);
    pub const DXGI_FORMAT_R9G9B9E5_SHAREDEXP: Self = Self(67u32);
    pub const DXGI_FORMAT_R8G8_B8G8_UNORM: Self = Self(68u32);
    pub const DXGI_FORMAT_G8R8_G8B8_UNORM: Self = Self(69u32);
    pub const DXGI_FORMAT_BC1_TYPELESS: Self = Self(70u32);
    pub const DXGI_FORMAT_BC1_UNORM: Self = Self(71u32);
    pub const DXGI_FORMAT_BC1_UNORM_SRGB: Self = Self(72u32);
    pub const DXGI_FORMAT_BC2_TYPELESS: Self = Self(73u32);
    pub const DXGI_FORMAT_BC2_UNORM: Self = Self(74u32);
    pub const DXGI_FORMAT_BC2_UNORM_SRGB: Self = Self(75u32);
    pub const DXGI_FORMAT_BC3_TYPELESS: Self = Self(76u32);
    pub const DXGI_FORMAT_BC3_UNORM: Self = Self(77u32);
    pub const DXGI_FORMAT_BC3_UNORM_SRGB: Self = Self(78u32);
    pub const DXGI_FORMAT_BC4_TYPELESS: Self = Self(79u32);
    pub const DXGI_FORMAT_BC4_UNORM: Self = Self(80u32);
    pub const DXGI_FORMAT_BC4_SNORM: Self = Self(81u32);
    pub const DXGI_FORMAT_BC5_TYPELESS: Self = Self(82u32);
    pub const DXGI_FORMAT_BC5_UNORM: Self = Self(83u32);
    pub const DXGI_FORMAT_BC5_SNORM: Self = Self(84u32);
    pub const DXGI_FORMAT_B5G6R5_UNORM: Self = Self(85u32);
    pub const DXGI_FORMAT_B5G5R5A1_UNORM: Self = Self(86u32);
    pub const DXGI_FORMAT_B8G8R8A8_UNORM: Self = Self(87u32);
    pub const DXGI_FORMAT_B8G8R8X8_UNORM: Self = Self(88u32);
    pub const DXGI_FORMAT_R10G10B10_XR_BIAS_A2_UNORM: Self = Self(89u32);
    pub const DXGI_FORMAT_B8G8R8A8_TYPELESS: Self = Self(90u32);
    pub const DXGI_FORMAT_B8G8R8A8_UNORM_SRGB: Self = Self(91u32);
    pub const DXGI_FORMAT_B8G8R8X8_TYPELESS: Self = Self(92u32);
    pub const DXGI_FORMAT_B8G8R8X8_UNORM_SRGB: Self = Self(93u32);
    pub const DXGI_FORMAT_BC6H_TYPELESS: Self = Self(94u32);
    pub const DXGI_FORMAT_BC6H_UF16: Self = Self(95u32);
    pub const DXGI_FORMAT_BC6H_SF16: Self = Self(96u32);
    pub const DXGI_FORMAT_BC7_TYPELESS: Self = Self(97u32);
    pub const DXGI_FORMAT_BC7_UNORM: Self = Self(98u32);
    pub const DXGI_FORMAT_BC7_UNORM_SRGB: Self = Self(99u32);
    pub const DXGI_FORMAT_AYUV: Self = Self(100u32);
    pub const DXGI_FORMAT_Y410: Self = Self(101u32);
    pub const DXGI_FORMAT_Y416: Self = Self(102u32);
    pub const DXGI_FORMAT_NV12: Self = Self(103u32);
    pub const DXGI_FORMAT_P010: Self = Self(104u32);
    pub const DXGI_FORMAT_P016: Self = Self(105u32);
    pub const DXGI_FORMAT_420_OPAQUE: Self = Self(106u32);
    pub const DXGI_FORMAT_YUY2: Self = Self(107u32);
    pub const DXGI_FORMAT_Y210: Self = Self(108u32);
    pub const DXGI_FORMAT_Y216: Self = Self(109u32);
    pub const DXGI_FORMAT_NV11: Self = Self(110u32);
    pub const DXGI_FORMAT_AI44: Self = Self(111u32);
    pub const DXGI_FORMAT_IA44: Self = Self(112u32);
    pub const DXGI_FORMAT_P8: Self = Self(113u32);
    pub const DXGI_FORMAT_A8P8: Self = Self(114u32);
    pub const DXGI_FORMAT_B4G4R4A4_UNORM: Self = Self(115u32);
    pub const DXGI_FORMAT_P208: Self = Self(130u32);
    pub const DXGI_FORMAT_V208: Self = Self(131u32);
    pub const DXGI_FORMAT_V408: Self = Self(132u32);
    pub const DXGI_FORMAT_SAMPLER_FEEDBACK_MIN_MIP_OPAQUE: Self = Self(189u32);
    pub const DXGI_FORMAT_SAMPLER_FEEDBACK_MIP_REGION_USED_OPAQUE: Self = Self(190u32);
    pub const DXGI_FORMAT_FORCE_UINT: Self = Self(4294967295u32);
}
impl ::std::convert::From<u32> for DXGI_FORMAT {
    fn from(value: u32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::Abi for DXGI_FORMAT {
    type Abi = Self;
}
impl ::std::ops::BitOr for DXGI_FORMAT {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}
impl ::std::ops::BitAnd for DXGI_FORMAT {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}
#[repr(C)]
#[allow(non_snake_case)]
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
pub struct DXGI_SAMPLE_DESC {
    pub count: u32,
    pub quality: u32,
}
impl DXGI_SAMPLE_DESC {}
impl ::std::default::Default for DXGI_SAMPLE_DESC {
    fn default() -> Self {
        Self {
            count: 0,
            quality: 0,
        }
    }
}
impl ::std::fmt::Debug for DXGI_SAMPLE_DESC {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("DXGI_SAMPLE_DESC")
            .field("count", &format_args!("{:?}", self.count))
            .field("quality", &format_args!("{:?}", self.quality))
            .finish()
    }
}
impl ::std::cmp::PartialEq for DXGI_SAMPLE_DESC {
    fn eq(&self, other: &Self) -> bool {
        self.count == other.count && self.quality == other.quality
    }
}
impl ::std::cmp::Eq for DXGI_SAMPLE_DESC {}
unsafe impl ::windows::Abi for DXGI_SAMPLE_DESC {
    type Abi = Self;
}
pub struct Apis {}
impl Apis {}
impl ::windows::RuntimeName for Apis {
    const NAME: &'static str = "Windows.Win32.Dxgi.Apis";
}
pub unsafe fn CreateDXGIFactory(
    riid: *const ::windows::Guid,
    pp_factory: *mut *mut ::std::ffi::c_void,
) -> ::windows::ErrorCode {
    #[link(name = "dxgi")]
    extern "system" {
        pub fn CreateDXGIFactory(
            riid: *const ::windows::Guid,
            pp_factory: *mut *mut ::std::ffi::c_void,
        ) -> ::windows::ErrorCode;
    }
    CreateDXGIFactory(
        ::std::mem::transmute(riid),
        ::std::mem::transmute(pp_factory),
    )
}
pub unsafe fn CreateDXGIFactory1(
    riid: *const ::windows::Guid,
    pp_factory: *mut *mut ::std::ffi::c_void,
) -> ::windows::ErrorCode {
    #[link(name = "dxgi")]
    extern "system" {
        pub fn CreateDXGIFactory1(
            riid: *const ::windows::Guid,
            pp_factory: *mut *mut ::std::ffi::c_void,
        ) -> ::windows::ErrorCode;
    }
    CreateDXGIFactory1(
        ::std::mem::transmute(riid),
        ::std::mem::transmute(pp_factory),
    )
}
pub unsafe fn CreateDXGIFactory2(
    flags: u32,
    riid: *const ::windows::Guid,
    pp_factory: *mut *mut ::std::ffi::c_void,
) -> ::windows::ErrorCode {
    #[link(name = "dxgi")]
    extern "system" {
        pub fn CreateDXGIFactory2(
            flags: u32,
            riid: *const ::windows::Guid,
            pp_factory: *mut *mut ::std::ffi::c_void,
        ) -> ::windows::ErrorCode;
    }
    CreateDXGIFactory2(
        ::std::mem::transmute(flags),
        ::std::mem::transmute(riid),
        ::std::mem::transmute(pp_factory),
    )
}
pub unsafe fn DXGIDeclareAdapterRemovalSupport() -> ::windows::ErrorCode {
    #[link(name = "dxgi")]
    extern "system" {
        pub fn DXGIDeclareAdapterRemovalSupport() -> ::windows::ErrorCode;
    }
    DXGIDeclareAdapterRemovalSupport()
}
pub unsafe fn DXGIGetDebugInterface1(
    flags: u32,
    riid: *const ::windows::Guid,
    p_debug: *mut *mut ::std::ffi::c_void,
) -> ::windows::ErrorCode {
    #[link(name = "dxgi")]
    extern "system" {
        pub fn DXGIGetDebugInterface1(
            flags: u32,
            riid: *const ::windows::Guid,
            p_debug: *mut *mut ::std::ffi::c_void,
        ) -> ::windows::ErrorCode;
    }
    DXGIGetDebugInterface1(
        ::std::mem::transmute(flags),
        ::std::mem::transmute(riid),
        ::std::mem::transmute(p_debug),
    )
}
#[repr(C)]
#[allow(non_snake_case)]
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
pub struct DXGI_ADAPTER_DESC {
    pub description: [u16; 128],
    pub vendor_id: u32,
    pub device_id: u32,
    pub sub_sys_id: u32,
    pub revision: u32,
    pub dedicated_video_memory: usize,
    pub dedicated_system_memory: usize,
    pub shared_system_memory: usize,
    pub adapter_luid: super::kernel::LUID,
}
impl DXGI_ADAPTER_DESC {}
impl ::std::default::Default for DXGI_ADAPTER_DESC {
    fn default() -> Self {
        Self {
            description: [0; 128],
            vendor_id: 0,
            device_id: 0,
            sub_sys_id: 0,
            revision: 0,
            dedicated_video_memory: 0,
            dedicated_system_memory: 0,
            shared_system_memory: 0,
            adapter_luid: ::std::default::Default::default(),
        }
    }
}
impl ::std::fmt::Debug for DXGI_ADAPTER_DESC {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("DXGI_ADAPTER_DESC")
            .field("description", &format_args!("{:?}", self.description))
            .field("vendor_id", &format_args!("{:?}", self.vendor_id))
            .field("device_id", &format_args!("{:?}", self.device_id))
            .field("sub_sys_id", &format_args!("{:?}", self.sub_sys_id))
            .field("revision", &format_args!("{:?}", self.revision))
            .field(
                "dedicated_video_memory",
                &format_args!("{:?}", self.dedicated_video_memory),
            )
            .field(
                "dedicated_system_memory",
                &format_args!("{:?}", self.dedicated_system_memory),
            )
            .field(
                "shared_system_memory",
                &format_args!("{:?}", self.shared_system_memory),
            )
            .field("adapter_luid", &format_args!("{:?}", self.adapter_luid))
            .finish()
    }
}
impl ::std::cmp::PartialEq for DXGI_ADAPTER_DESC {
    fn eq(&self, other: &Self) -> bool {
        self.description == other.description
            && self.vendor_id == other.vendor_id
            && self.device_id == other.device_id
            && self.sub_sys_id == other.sub_sys_id
            && self.revision == other.revision
            && self.dedicated_video_memory == other.dedicated_video_memory
            && self.dedicated_system_memory == other.dedicated_system_memory
            && self.shared_system_memory == other.shared_system_memory
            && self.adapter_luid == other.adapter_luid
    }
}
impl ::std::cmp::Eq for DXGI_ADAPTER_DESC {}
unsafe impl ::windows::Abi for DXGI_ADAPTER_DESC {
    type Abi = Self;
}
#[repr(C)]
#[allow(non_snake_case)]
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
pub struct DXGI_ADAPTER_DESC1 {
    pub description: [u16; 128],
    pub vendor_id: u32,
    pub device_id: u32,
    pub sub_sys_id: u32,
    pub revision: u32,
    pub dedicated_video_memory: usize,
    pub dedicated_system_memory: usize,
    pub shared_system_memory: usize,
    pub adapter_luid: super::kernel::LUID,
    pub flags: u32,
}
impl DXGI_ADAPTER_DESC1 {}
impl ::std::default::Default for DXGI_ADAPTER_DESC1 {
    fn default() -> Self {
        Self {
            description: [0; 128],
            vendor_id: 0,
            device_id: 0,
            sub_sys_id: 0,
            revision: 0,
            dedicated_video_memory: 0,
            dedicated_system_memory: 0,
            shared_system_memory: 0,
            adapter_luid: ::std::default::Default::default(),
            flags: 0,
        }
    }
}
impl ::std::fmt::Debug for DXGI_ADAPTER_DESC1 {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("DXGI_ADAPTER_DESC1")
            .field("description", &format_args!("{:?}", self.description))
            .field("vendor_id", &format_args!("{:?}", self.vendor_id))
            .field("device_id", &format_args!("{:?}", self.device_id))
            .field("sub_sys_id", &format_args!("{:?}", self.sub_sys_id))
            .field("revision", &format_args!("{:?}", self.revision))
            .field(
                "dedicated_video_memory",
                &format_args!("{:?}", self.dedicated_video_memory),
            )
            .field(
                "dedicated_system_memory",
                &format_args!("{:?}", self.dedicated_system_memory),
            )
            .field(
                "shared_system_memory",
                &format_args!("{:?}", self.shared_system_memory),
            )
            .field("adapter_luid", &format_args!("{:?}", self.adapter_luid))
            .field("flags", &format_args!("{:?}", self.flags))
            .finish()
    }
}
impl ::std::cmp::PartialEq for DXGI_ADAPTER_DESC1 {
    fn eq(&self, other: &Self) -> bool {
        self.description == other.description
            && self.vendor_id == other.vendor_id
            && self.device_id == other.device_id
            && self.sub_sys_id == other.sub_sys_id
            && self.revision == other.revision
            && self.dedicated_video_memory == other.dedicated_video_memory
            && self.dedicated_system_memory == other.dedicated_system_memory
            && self.shared_system_memory == other.shared_system_memory
            && self.adapter_luid == other.adapter_luid
            && self.flags == other.flags
    }
}
impl ::std::cmp::Eq for DXGI_ADAPTER_DESC1 {}
unsafe impl ::windows::Abi for DXGI_ADAPTER_DESC1 {
    type Abi = Self;
}
#[allow(non_camel_case_types)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: marker :: Copy,
    :: std :: clone :: Clone,
    :: std :: default :: Default,
    :: std :: fmt :: Debug,
)]
#[repr(transparent)]
pub struct DXGI_GRAPHICS_PREEMPTION_GRANULARITY(pub i32);
impl DXGI_GRAPHICS_PREEMPTION_GRANULARITY {
    #![allow(non_upper_case_globals)]
    pub const DXGI_GRAPHICS_PREEMPTION_DMA_BUFFER_BOUNDARY: Self = Self(0i32);
    pub const DXGI_GRAPHICS_PREEMPTION_PRIMITIVE_BOUNDARY: Self = Self(1i32);
    pub const DXGI_GRAPHICS_PREEMPTION_TRIANGLE_BOUNDARY: Self = Self(2i32);
    pub const DXGI_GRAPHICS_PREEMPTION_PIXEL_BOUNDARY: Self = Self(3i32);
    pub const DXGI_GRAPHICS_PREEMPTION_INSTRUCTION_BOUNDARY: Self = Self(4i32);
}
impl ::std::convert::From<i32> for DXGI_GRAPHICS_PREEMPTION_GRANULARITY {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::Abi for DXGI_GRAPHICS_PREEMPTION_GRANULARITY {
    type Abi = Self;
}
#[allow(non_camel_case_types)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: marker :: Copy,
    :: std :: clone :: Clone,
    :: std :: default :: Default,
    :: std :: fmt :: Debug,
)]
#[repr(transparent)]
pub struct DXGI_COMPUTE_PREEMPTION_GRANULARITY(pub i32);
impl DXGI_COMPUTE_PREEMPTION_GRANULARITY {
    #![allow(non_upper_case_globals)]
    pub const DXGI_COMPUTE_PREEMPTION_DMA_BUFFER_BOUNDARY: Self = Self(0i32);
    pub const DXGI_COMPUTE_PREEMPTION_DISPATCH_BOUNDARY: Self = Self(1i32);
    pub const DXGI_COMPUTE_PREEMPTION_THREAD_GROUP_BOUNDARY: Self = Self(2i32);
    pub const DXGI_COMPUTE_PREEMPTION_THREAD_BOUNDARY: Self = Self(3i32);
    pub const DXGI_COMPUTE_PREEMPTION_INSTRUCTION_BOUNDARY: Self = Self(4i32);
}
impl ::std::convert::From<i32> for DXGI_COMPUTE_PREEMPTION_GRANULARITY {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::Abi for DXGI_COMPUTE_PREEMPTION_GRANULARITY {
    type Abi = Self;
}
#[repr(C)]
#[allow(non_snake_case)]
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
pub struct DXGI_ADAPTER_DESC2 {
    pub description: [u16; 128],
    pub vendor_id: u32,
    pub device_id: u32,
    pub sub_sys_id: u32,
    pub revision: u32,
    pub dedicated_video_memory: usize,
    pub dedicated_system_memory: usize,
    pub shared_system_memory: usize,
    pub adapter_luid: super::kernel::LUID,
    pub flags: u32,
    pub graphics_preemption_granularity: DXGI_GRAPHICS_PREEMPTION_GRANULARITY,
    pub compute_preemption_granularity: DXGI_COMPUTE_PREEMPTION_GRANULARITY,
}
impl DXGI_ADAPTER_DESC2 {}
impl ::std::default::Default for DXGI_ADAPTER_DESC2 {
    fn default() -> Self {
        Self {
            description: [0; 128],
            vendor_id: 0,
            device_id: 0,
            sub_sys_id: 0,
            revision: 0,
            dedicated_video_memory: 0,
            dedicated_system_memory: 0,
            shared_system_memory: 0,
            adapter_luid: ::std::default::Default::default(),
            flags: 0,
            graphics_preemption_granularity: ::std::default::Default::default(),
            compute_preemption_granularity: ::std::default::Default::default(),
        }
    }
}
impl ::std::fmt::Debug for DXGI_ADAPTER_DESC2 {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("DXGI_ADAPTER_DESC2")
            .field("description", &format_args!("{:?}", self.description))
            .field("vendor_id", &format_args!("{:?}", self.vendor_id))
            .field("device_id", &format_args!("{:?}", self.device_id))
            .field("sub_sys_id", &format_args!("{:?}", self.sub_sys_id))
            .field("revision", &format_args!("{:?}", self.revision))
            .field(
                "dedicated_video_memory",
                &format_args!("{:?}", self.dedicated_video_memory),
            )
            .field(
                "dedicated_system_memory",
                &format_args!("{:?}", self.dedicated_system_memory),
            )
            .field(
                "shared_system_memory",
                &format_args!("{:?}", self.shared_system_memory),
            )
            .field("adapter_luid", &format_args!("{:?}", self.adapter_luid))
            .field("flags", &format_args!("{:?}", self.flags))
            .field(
                "graphics_preemption_granularity",
                &format_args!("{:?}", self.graphics_preemption_granularity),
            )
            .field(
                "compute_preemption_granularity",
                &format_args!("{:?}", self.compute_preemption_granularity),
            )
            .finish()
    }
}
impl ::std::cmp::PartialEq for DXGI_ADAPTER_DESC2 {
    fn eq(&self, other: &Self) -> bool {
        self.description == other.description
            && self.vendor_id == other.vendor_id
            && self.device_id == other.device_id
            && self.sub_sys_id == other.sub_sys_id
            && self.revision == other.revision
            && self.dedicated_video_memory == other.dedicated_video_memory
            && self.dedicated_system_memory == other.dedicated_system_memory
            && self.shared_system_memory == other.shared_system_memory
            && self.adapter_luid == other.adapter_luid
            && self.flags == other.flags
            && self.graphics_preemption_granularity == other.graphics_preemption_granularity
            && self.compute_preemption_granularity == other.compute_preemption_granularity
    }
}
impl ::std::cmp::Eq for DXGI_ADAPTER_DESC2 {}
unsafe impl ::windows::Abi for DXGI_ADAPTER_DESC2 {
    type Abi = Self;
}
#[allow(non_camel_case_types)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: marker :: Copy,
    :: std :: clone :: Clone,
    :: std :: default :: Default,
    :: std :: fmt :: Debug,
)]
#[repr(transparent)]
pub struct DXGI_ADAPTER_FLAG3(pub u32);
impl DXGI_ADAPTER_FLAG3 {
    #![allow(non_upper_case_globals)]
    pub const DXGI_ADAPTER_FLAG3_NONE: Self = Self(0u32);
    pub const DXGI_ADAPTER_FLAG3_REMOTE: Self = Self(1u32);
    pub const DXGI_ADAPTER_FLAG3_SOFTWARE: Self = Self(2u32);
    pub const DXGI_ADAPTER_FLAG3_ACG_COMPATIBLE: Self = Self(4u32);
    pub const DXGI_ADAPTER_FLAG3_SUPPORT_MONITORED_FENCES: Self = Self(8u32);
    pub const DXGI_ADAPTER_FLAG3_SUPPORT_NON_MONITORED_FENCES: Self = Self(16u32);
    pub const DXGI_ADAPTER_FLAG3_KEYED_MUTEX_CONFORMANCE: Self = Self(32u32);
    pub const DXGI_ADAPTER_FLAG3_FORCE_DWORD: Self = Self(4294967295u32);
}
impl ::std::convert::From<u32> for DXGI_ADAPTER_FLAG3 {
    fn from(value: u32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::Abi for DXGI_ADAPTER_FLAG3 {
    type Abi = Self;
}
impl ::std::ops::BitOr for DXGI_ADAPTER_FLAG3 {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}
impl ::std::ops::BitAnd for DXGI_ADAPTER_FLAG3 {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}
#[repr(C)]
#[allow(non_snake_case)]
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
pub struct DXGI_ADAPTER_DESC3 {
    pub description: [u16; 128],
    pub vendor_id: u32,
    pub device_id: u32,
    pub sub_sys_id: u32,
    pub revision: u32,
    pub dedicated_video_memory: usize,
    pub dedicated_system_memory: usize,
    pub shared_system_memory: usize,
    pub adapter_luid: super::kernel::LUID,
    pub flags: DXGI_ADAPTER_FLAG3,
    pub graphics_preemption_granularity: DXGI_GRAPHICS_PREEMPTION_GRANULARITY,
    pub compute_preemption_granularity: DXGI_COMPUTE_PREEMPTION_GRANULARITY,
}
impl DXGI_ADAPTER_DESC3 {}
impl ::std::default::Default for DXGI_ADAPTER_DESC3 {
    fn default() -> Self {
        Self {
            description: [0; 128],
            vendor_id: 0,
            device_id: 0,
            sub_sys_id: 0,
            revision: 0,
            dedicated_video_memory: 0,
            dedicated_system_memory: 0,
            shared_system_memory: 0,
            adapter_luid: ::std::default::Default::default(),
            flags: ::std::default::Default::default(),
            graphics_preemption_granularity: ::std::default::Default::default(),
            compute_preemption_granularity: ::std::default::Default::default(),
        }
    }
}
impl ::std::fmt::Debug for DXGI_ADAPTER_DESC3 {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("DXGI_ADAPTER_DESC3")
            .field("description", &format_args!("{:?}", self.description))
            .field("vendor_id", &format_args!("{:?}", self.vendor_id))
            .field("device_id", &format_args!("{:?}", self.device_id))
            .field("sub_sys_id", &format_args!("{:?}", self.sub_sys_id))
            .field("revision", &format_args!("{:?}", self.revision))
            .field(
                "dedicated_video_memory",
                &format_args!("{:?}", self.dedicated_video_memory),
            )
            .field(
                "dedicated_system_memory",
                &format_args!("{:?}", self.dedicated_system_memory),
            )
            .field(
                "shared_system_memory",
                &format_args!("{:?}", self.shared_system_memory),
            )
            .field("adapter_luid", &format_args!("{:?}", self.adapter_luid))
            .field("flags", &format_args!("{:?}", self.flags))
            .field(
                "graphics_preemption_granularity",
                &format_args!("{:?}", self.graphics_preemption_granularity),
            )
            .field(
                "compute_preemption_granularity",
                &format_args!("{:?}", self.compute_preemption_granularity),
            )
            .finish()
    }
}
impl ::std::cmp::PartialEq for DXGI_ADAPTER_DESC3 {
    fn eq(&self, other: &Self) -> bool {
        self.description == other.description
            && self.vendor_id == other.vendor_id
            && self.device_id == other.device_id
            && self.sub_sys_id == other.sub_sys_id
            && self.revision == other.revision
            && self.dedicated_video_memory == other.dedicated_video_memory
            && self.dedicated_system_memory == other.dedicated_system_memory
            && self.shared_system_memory == other.shared_system_memory
            && self.adapter_luid == other.adapter_luid
            && self.flags == other.flags
            && self.graphics_preemption_granularity == other.graphics_preemption_granularity
            && self.compute_preemption_granularity == other.compute_preemption_granularity
    }
}
impl ::std::cmp::Eq for DXGI_ADAPTER_DESC3 {}
unsafe impl ::windows::Abi for DXGI_ADAPTER_DESC3 {
    type Abi = Self;
}
#[allow(non_camel_case_types)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: marker :: Copy,
    :: std :: clone :: Clone,
    :: std :: default :: Default,
    :: std :: fmt :: Debug,
)]
#[repr(transparent)]
pub struct DXGI_ADAPTER_FLAG(pub u32);
impl DXGI_ADAPTER_FLAG {
    #![allow(non_upper_case_globals)]
    pub const DXGI_ADAPTER_FLAG_NONE: Self = Self(0u32);
    pub const DXGI_ADAPTER_FLAG_REMOTE: Self = Self(1u32);
    pub const DXGI_ADAPTER_FLAG_SOFTWARE: Self = Self(2u32);
}
impl ::std::convert::From<u32> for DXGI_ADAPTER_FLAG {
    fn from(value: u32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::Abi for DXGI_ADAPTER_FLAG {
    type Abi = Self;
}
impl ::std::ops::BitOr for DXGI_ADAPTER_FLAG {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}
impl ::std::ops::BitAnd for DXGI_ADAPTER_FLAG {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}
#[allow(non_camel_case_types)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: marker :: Copy,
    :: std :: clone :: Clone,
    :: std :: default :: Default,
    :: std :: fmt :: Debug,
)]
#[repr(transparent)]
pub struct DXGI_ALPHA_MODE(pub u32);
impl DXGI_ALPHA_MODE {
    #![allow(non_upper_case_globals)]
    pub const DXGI_ALPHA_MODE_UNSPECIFIED: Self = Self(0u32);
    pub const DXGI_ALPHA_MODE_PREMULTIPLIED: Self = Self(1u32);
    pub const DXGI_ALPHA_MODE_STRAIGHT: Self = Self(2u32);
    pub const DXGI_ALPHA_MODE_IGNORE: Self = Self(3u32);
    pub const DXGI_ALPHA_MODE_FORCE_DWORD: Self = Self(4294967295u32);
}
impl ::std::convert::From<u32> for DXGI_ALPHA_MODE {
    fn from(value: u32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::Abi for DXGI_ALPHA_MODE {
    type Abi = Self;
}
impl ::std::ops::BitOr for DXGI_ALPHA_MODE {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}
impl ::std::ops::BitAnd for DXGI_ALPHA_MODE {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}
pub const DXGI_CENTER_MULTISAMPLE_QUALITY_PATTERN: u32 = 4294967294u32;
#[allow(non_camel_case_types)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: marker :: Copy,
    :: std :: clone :: Clone,
    :: std :: default :: Default,
    :: std :: fmt :: Debug,
)]
#[repr(transparent)]
pub struct DXGI_COLOR_SPACE_TYPE(pub i32);
impl DXGI_COLOR_SPACE_TYPE {
    #![allow(non_upper_case_globals)]
    pub const DXGI_COLOR_SPACE_RGB_FULL_G22_NONE_P709: Self = Self(0i32);
    pub const DXGI_COLOR_SPACE_RGB_FULL_G10_NONE_P709: Self = Self(1i32);
    pub const DXGI_COLOR_SPACE_RGB_STUDIO_G22_NONE_P709: Self = Self(2i32);
    pub const DXGI_COLOR_SPACE_RGB_STUDIO_G22_NONE_P2020: Self = Self(3i32);
    pub const DXGI_COLOR_SPACE_RESERVED: Self = Self(4i32);
    pub const DXGI_COLOR_SPACE_YCBCR_FULL_G22_NONE_P709_X601: Self = Self(5i32);
    pub const DXGI_COLOR_SPACE_YCBCR_STUDIO_G22_LEFT_P601: Self = Self(6i32);
    pub const DXGI_COLOR_SPACE_YCBCR_FULL_G22_LEFT_P601: Self = Self(7i32);
    pub const DXGI_COLOR_SPACE_YCBCR_STUDIO_G22_LEFT_P709: Self = Self(8i32);
    pub const DXGI_COLOR_SPACE_YCBCR_FULL_G22_LEFT_P709: Self = Self(9i32);
    pub const DXGI_COLOR_SPACE_YCBCR_STUDIO_G22_LEFT_P2020: Self = Self(10i32);
    pub const DXGI_COLOR_SPACE_YCBCR_FULL_G22_LEFT_P2020: Self = Self(11i32);
    pub const DXGI_COLOR_SPACE_RGB_FULL_G2084_NONE_P2020: Self = Self(12i32);
    pub const DXGI_COLOR_SPACE_YCBCR_STUDIO_G2084_LEFT_P2020: Self = Self(13i32);
    pub const DXGI_COLOR_SPACE_RGB_STUDIO_G2084_NONE_P2020: Self = Self(14i32);
    pub const DXGI_COLOR_SPACE_YCBCR_STUDIO_G22_TOPLEFT_P2020: Self = Self(15i32);
    pub const DXGI_COLOR_SPACE_YCBCR_STUDIO_G2084_TOPLEFT_P2020: Self = Self(16i32);
    pub const DXGI_COLOR_SPACE_RGB_FULL_G22_NONE_P2020: Self = Self(17i32);
    pub const DXGI_COLOR_SPACE_YCBCR_STUDIO_GHLG_TOPLEFT_P2020: Self = Self(18i32);
    pub const DXGI_COLOR_SPACE_YCBCR_FULL_GHLG_TOPLEFT_P2020: Self = Self(19i32);
    pub const DXGI_COLOR_SPACE_RGB_STUDIO_G24_NONE_P709: Self = Self(20i32);
    pub const DXGI_COLOR_SPACE_RGB_STUDIO_G24_NONE_P2020: Self = Self(21i32);
    pub const DXGI_COLOR_SPACE_YCBCR_STUDIO_G24_LEFT_P709: Self = Self(22i32);
    pub const DXGI_COLOR_SPACE_YCBCR_STUDIO_G24_LEFT_P2020: Self = Self(23i32);
    pub const DXGI_COLOR_SPACE_YCBCR_STUDIO_G24_TOPLEFT_P2020: Self = Self(24i32);
    pub const DXGI_COLOR_SPACE_CUSTOM: Self = Self(-1i32);
}
impl ::std::convert::From<i32> for DXGI_COLOR_SPACE_TYPE {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::Abi for DXGI_COLOR_SPACE_TYPE {
    type Abi = Self;
}
pub const DXGI_CPU_ACCESS_DYNAMIC: u32 = 1u32;
pub const DXGI_CPU_ACCESS_FIELD: u32 = 15u32;
pub const DXGI_CPU_ACCESS_NONE: u32 = 0u32;
pub const DXGI_CPU_ACCESS_READ_WRITE: u32 = 2u32;
pub const DXGI_CPU_ACCESS_SCRATCH: u32 = 3u32;
pub const DXGI_CREATE_FACTORY_DEBUG: u32 = 1u32;
pub const DXGI_DEBUG_BINARY_VERSION: u32 = 1u32;
#[allow(non_camel_case_types)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: marker :: Copy,
    :: std :: clone :: Clone,
    :: std :: default :: Default,
    :: std :: fmt :: Debug,
)]
#[repr(transparent)]
pub struct DXGI_DEBUG_RLO_FLAGS(pub i32);
impl DXGI_DEBUG_RLO_FLAGS {
    #![allow(non_upper_case_globals)]
    pub const DXGI_DEBUG_RLO_SUMMARY: Self = Self(1i32);
    pub const DXGI_DEBUG_RLO_DETAIL: Self = Self(2i32);
    pub const DXGI_DEBUG_RLO_IGNORE_INTERNAL: Self = Self(4i32);
    pub const DXGI_DEBUG_RLO_ALL: Self = Self(7i32);
}
impl ::std::convert::From<i32> for DXGI_DEBUG_RLO_FLAGS {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::Abi for DXGI_DEBUG_RLO_FLAGS {
    type Abi = Self;
}
#[repr(C)]
#[allow(non_snake_case)]
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
pub struct DXGI_DECODE_SWAP_CHAIN_DESC {
    pub flags: u32,
}
impl DXGI_DECODE_SWAP_CHAIN_DESC {}
impl ::std::default::Default for DXGI_DECODE_SWAP_CHAIN_DESC {
    fn default() -> Self {
        Self { flags: 0 }
    }
}
impl ::std::fmt::Debug for DXGI_DECODE_SWAP_CHAIN_DESC {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("DXGI_DECODE_SWAP_CHAIN_DESC")
            .field("flags", &format_args!("{:?}", self.flags))
            .finish()
    }
}
impl ::std::cmp::PartialEq for DXGI_DECODE_SWAP_CHAIN_DESC {
    fn eq(&self, other: &Self) -> bool {
        self.flags == other.flags
    }
}
impl ::std::cmp::Eq for DXGI_DECODE_SWAP_CHAIN_DESC {}
unsafe impl ::windows::Abi for DXGI_DECODE_SWAP_CHAIN_DESC {
    type Abi = Self;
}
#[repr(C)]
#[allow(non_snake_case)]
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
pub struct DXGI_DISPLAY_COLOR_SPACE {
    pub primary_coordinates: [f32; 16],
    pub white_points: [f32; 32],
}
impl DXGI_DISPLAY_COLOR_SPACE {}
impl ::std::default::Default for DXGI_DISPLAY_COLOR_SPACE {
    fn default() -> Self {
        Self {
            primary_coordinates: [0.0; 16],
            white_points: [0.0; 32],
        }
    }
}
impl ::std::fmt::Debug for DXGI_DISPLAY_COLOR_SPACE {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("DXGI_DISPLAY_COLOR_SPACE")
            .field(
                "primary_coordinates",
                &format_args!("{:?}", self.primary_coordinates),
            )
            .field("white_points", &format_args!("{:?}", self.white_points))
            .finish()
    }
}
impl ::std::cmp::PartialEq for DXGI_DISPLAY_COLOR_SPACE {
    fn eq(&self, other: &Self) -> bool {
        self.primary_coordinates == other.primary_coordinates
            && self.white_points == other.white_points
    }
}
impl ::std::cmp::Eq for DXGI_DISPLAY_COLOR_SPACE {}
unsafe impl ::windows::Abi for DXGI_DISPLAY_COLOR_SPACE {
    type Abi = Self;
}
pub const DXGI_ENUM_MODES_DISABLED_STEREO: u32 = 8u32;
pub const DXGI_ENUM_MODES_INTERLACED: u32 = 1u32;
pub const DXGI_ENUM_MODES_SCALING: u32 = 2u32;
pub const DXGI_ENUM_MODES_STEREO: u32 = 4u32;
pub const DXGI_ERROR_ACCESS_DENIED: i32 = -2005270485i32;
pub const DXGI_ERROR_ACCESS_LOST: i32 = -2005270490i32;
pub const DXGI_ERROR_ALREADY_EXISTS: i32 = -2005270474i32;
pub const DXGI_ERROR_CACHE_CORRUPT: i32 = -2005270477i32;
pub const DXGI_ERROR_CACHE_FULL: i32 = -2005270476i32;
pub const DXGI_ERROR_CACHE_HASH_COLLISION: i32 = -2005270475i32;
pub const DXGI_ERROR_CANNOT_PROTECT_CONTENT: i32 = -2005270486i32;
pub const DXGI_ERROR_DEVICE_HUNG: i32 = -2005270522i32;
pub const DXGI_ERROR_DEVICE_REMOVED: i32 = -2005270523i32;
pub const DXGI_ERROR_DEVICE_RESET: i32 = -2005270521i32;
pub const DXGI_ERROR_DRIVER_INTERNAL_ERROR: i32 = -2005270496i32;
pub const DXGI_ERROR_DYNAMIC_CODE_POLICY_VIOLATION: i32 = -2005270479i32;
pub const DXGI_ERROR_FRAME_STATISTICS_DISJOINT: i32 = -2005270517i32;
pub const DXGI_ERROR_GRAPHICS_VIDPN_SOURCE_IN_USE: i32 = -2005270516i32;
pub const DXGI_ERROR_HW_PROTECTION_OUTOFMEMORY: i32 = -2005270480i32;
pub const DXGI_ERROR_INVALID_CALL: i32 = -2005270527i32;
pub const DXGI_ERROR_MODE_CHANGE_IN_PROGRESS: i32 = -2005270491i32;
pub const DXGI_ERROR_MORE_DATA: i32 = -2005270525i32;
pub const DXGI_ERROR_NAME_ALREADY_EXISTS: i32 = -2005270484i32;
pub const DXGI_ERROR_NONEXCLUSIVE: i32 = -2005270495i32;
pub const DXGI_ERROR_NON_COMPOSITED_UI: i32 = -2005270478i32;
pub const DXGI_ERROR_NOT_CURRENT: i32 = -2005270482i32;
pub const DXGI_ERROR_NOT_CURRENTLY_AVAILABLE: i32 = -2005270494i32;
pub const DXGI_ERROR_NOT_FOUND: i32 = -2005270526i32;
pub const DXGI_ERROR_REMOTE_CLIENT_DISCONNECTED: i32 = -2005270493i32;
pub const DXGI_ERROR_REMOTE_OUTOFMEMORY: i32 = -2005270492i32;
pub const DXGI_ERROR_RESTRICT_TO_OUTPUT_STALE: i32 = -2005270487i32;
pub const DXGI_ERROR_SDK_COMPONENT_MISSING: i32 = -2005270483i32;
pub const DXGI_ERROR_SESSION_DISCONNECTED: i32 = -2005270488i32;
pub const DXGI_ERROR_UNSUPPORTED: i32 = -2005270524i32;
pub const DXGI_ERROR_WAIT_TIMEOUT: i32 = -2005270489i32;
pub const DXGI_ERROR_WAS_STILL_DRAWING: i32 = -2005270518i32;
#[allow(non_camel_case_types)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: marker :: Copy,
    :: std :: clone :: Clone,
    :: std :: default :: Default,
    :: std :: fmt :: Debug,
)]
#[repr(transparent)]
pub struct DXGI_FEATURE(pub i32);
impl DXGI_FEATURE {
    #![allow(non_upper_case_globals)]
    pub const DXGI_FEATURE_PRESENT_ALLOW_TEARING: Self = Self(0i32);
}
impl ::std::convert::From<i32> for DXGI_FEATURE {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::Abi for DXGI_FEATURE {
    type Abi = Self;
}
pub const DXGI_FORMAT_DEFINED: u32 = 1u32;
#[allow(non_camel_case_types)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: marker :: Copy,
    :: std :: clone :: Clone,
    :: std :: default :: Default,
    :: std :: fmt :: Debug,
)]
#[repr(transparent)]
pub struct DXGI_FRAME_PRESENTATION_MODE(pub i32);
impl DXGI_FRAME_PRESENTATION_MODE {
    #![allow(non_upper_case_globals)]
    pub const DXGI_FRAME_PRESENTATION_MODE_COMPOSED: Self = Self(0i32);
    pub const DXGI_FRAME_PRESENTATION_MODE_OVERLAY: Self = Self(1i32);
    pub const DXGI_FRAME_PRESENTATION_MODE_NONE: Self = Self(2i32);
    pub const DXGI_FRAME_PRESENTATION_MODE_COMPOSITION_FAILURE: Self = Self(3i32);
}
impl ::std::convert::From<i32> for DXGI_FRAME_PRESENTATION_MODE {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::Abi for DXGI_FRAME_PRESENTATION_MODE {
    type Abi = Self;
}
#[repr(C)]
#[allow(non_snake_case)]
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
pub struct DXGI_FRAME_STATISTICS {
    pub present_count: u32,
    pub present_refresh_count: u32,
    pub sync_refresh_count: u32,
    pub sync_qpc_time: i64,
    pub sync_gpu_time: i64,
}
impl DXGI_FRAME_STATISTICS {}
impl ::std::default::Default for DXGI_FRAME_STATISTICS {
    fn default() -> Self {
        Self {
            present_count: 0,
            present_refresh_count: 0,
            sync_refresh_count: 0,
            sync_qpc_time: 0,
            sync_gpu_time: 0,
        }
    }
}
impl ::std::fmt::Debug for DXGI_FRAME_STATISTICS {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("DXGI_FRAME_STATISTICS")
            .field("present_count", &format_args!("{:?}", self.present_count))
            .field(
                "present_refresh_count",
                &format_args!("{:?}", self.present_refresh_count),
            )
            .field(
                "sync_refresh_count",
                &format_args!("{:?}", self.sync_refresh_count),
            )
            .field("sync_qpc_time", &format_args!("{:?}", self.sync_qpc_time))
            .field("sync_gpu_time", &format_args!("{:?}", self.sync_gpu_time))
            .finish()
    }
}
impl ::std::cmp::PartialEq for DXGI_FRAME_STATISTICS {
    fn eq(&self, other: &Self) -> bool {
        self.present_count == other.present_count
            && self.present_refresh_count == other.present_refresh_count
            && self.sync_refresh_count == other.sync_refresh_count
            && self.sync_qpc_time == other.sync_qpc_time
            && self.sync_gpu_time == other.sync_gpu_time
    }
}
impl ::std::cmp::Eq for DXGI_FRAME_STATISTICS {}
unsafe impl ::windows::Abi for DXGI_FRAME_STATISTICS {
    type Abi = Self;
}
#[repr(C)]
#[allow(non_snake_case)]
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
pub struct DXGI_FRAME_STATISTICS_MEDIA {
    pub present_count: u32,
    pub present_refresh_count: u32,
    pub sync_refresh_count: u32,
    pub sync_qpc_time: i64,
    pub sync_gpu_time: i64,
    pub composition_mode: DXGI_FRAME_PRESENTATION_MODE,
    pub approved_present_duration: u32,
}
impl DXGI_FRAME_STATISTICS_MEDIA {}
impl ::std::default::Default for DXGI_FRAME_STATISTICS_MEDIA {
    fn default() -> Self {
        Self {
            present_count: 0,
            present_refresh_count: 0,
            sync_refresh_count: 0,
            sync_qpc_time: 0,
            sync_gpu_time: 0,
            composition_mode: ::std::default::Default::default(),
            approved_present_duration: 0,
        }
    }
}
impl ::std::fmt::Debug for DXGI_FRAME_STATISTICS_MEDIA {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("DXGI_FRAME_STATISTICS_MEDIA")
            .field("present_count", &format_args!("{:?}", self.present_count))
            .field(
                "present_refresh_count",
                &format_args!("{:?}", self.present_refresh_count),
            )
            .field(
                "sync_refresh_count",
                &format_args!("{:?}", self.sync_refresh_count),
            )
            .field("sync_qpc_time", &format_args!("{:?}", self.sync_qpc_time))
            .field("sync_gpu_time", &format_args!("{:?}", self.sync_gpu_time))
            .field(
                "composition_mode",
                &format_args!("{:?}", self.composition_mode),
            )
            .field(
                "approved_present_duration",
                &format_args!("{:?}", self.approved_present_duration),
            )
            .finish()
    }
}
impl ::std::cmp::PartialEq for DXGI_FRAME_STATISTICS_MEDIA {
    fn eq(&self, other: &Self) -> bool {
        self.present_count == other.present_count
            && self.present_refresh_count == other.present_refresh_count
            && self.sync_refresh_count == other.sync_refresh_count
            && self.sync_qpc_time == other.sync_qpc_time
            && self.sync_gpu_time == other.sync_gpu_time
            && self.composition_mode == other.composition_mode
            && self.approved_present_duration == other.approved_present_duration
    }
}
impl ::std::cmp::Eq for DXGI_FRAME_STATISTICS_MEDIA {}
unsafe impl ::windows::Abi for DXGI_FRAME_STATISTICS_MEDIA {
    type Abi = Self;
}
#[repr(C)]
#[allow(non_snake_case)]
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
pub struct DXGI_RGB {
    pub red: f32,
    pub green: f32,
    pub blue: f32,
}
impl DXGI_RGB {}
impl ::std::default::Default for DXGI_RGB {
    fn default() -> Self {
        Self {
            red: 0.0,
            green: 0.0,
            blue: 0.0,
        }
    }
}
impl ::std::fmt::Debug for DXGI_RGB {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("DXGI_RGB")
            .field("red", &format_args!("{:?}", self.red))
            .field("green", &format_args!("{:?}", self.green))
            .field("blue", &format_args!("{:?}", self.blue))
            .finish()
    }
}
impl ::std::cmp::PartialEq for DXGI_RGB {
    fn eq(&self, other: &Self) -> bool {
        self.red == other.red && self.green == other.green && self.blue == other.blue
    }
}
impl ::std::cmp::Eq for DXGI_RGB {}
unsafe impl ::windows::Abi for DXGI_RGB {
    type Abi = Self;
}
#[repr(C)]
#[allow(non_snake_case)]
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
pub struct DXGI_GAMMA_CONTROL {
    pub scale: DXGI_RGB,
    pub offset: DXGI_RGB,
    pub gamma_curve: [DXGI_RGB; 1025],
}
impl DXGI_GAMMA_CONTROL {}
impl ::std::default::Default for DXGI_GAMMA_CONTROL {
    fn default() -> Self {
        Self {
            scale: ::std::default::Default::default(),
            offset: ::std::default::Default::default(),
            gamma_curve: [::std::default::Default::default(); 1025],
        }
    }
}
impl ::std::fmt::Debug for DXGI_GAMMA_CONTROL {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("DXGI_GAMMA_CONTROL")
            .field("scale", &format_args!("{:?}", self.scale))
            .field("offset", &format_args!("{:?}", self.offset))
            .field("gamma_curve", &format_args!("{:?}", self.gamma_curve))
            .finish()
    }
}
impl ::std::cmp::PartialEq for DXGI_GAMMA_CONTROL {
    fn eq(&self, other: &Self) -> bool {
        self.scale == other.scale
            && self.offset == other.offset
            && self.gamma_curve == other.gamma_curve
    }
}
impl ::std::cmp::Eq for DXGI_GAMMA_CONTROL {}
unsafe impl ::windows::Abi for DXGI_GAMMA_CONTROL {
    type Abi = Self;
}
#[repr(C)]
#[allow(non_snake_case)]
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
pub struct DXGI_GAMMA_CONTROL_CAPABILITIES {
    pub scale_and_offset_supported: super::system_services::BOOL,
    pub max_converted_value: f32,
    pub min_converted_value: f32,
    pub num_gamma_control_points: u32,
    pub control_point_positions: [f32; 1025],
}
impl DXGI_GAMMA_CONTROL_CAPABILITIES {}
impl ::std::default::Default for DXGI_GAMMA_CONTROL_CAPABILITIES {
    fn default() -> Self {
        Self {
            scale_and_offset_supported: ::std::default::Default::default(),
            max_converted_value: 0.0,
            min_converted_value: 0.0,
            num_gamma_control_points: 0,
            control_point_positions: [0.0; 1025],
        }
    }
}
impl ::std::fmt::Debug for DXGI_GAMMA_CONTROL_CAPABILITIES {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("DXGI_GAMMA_CONTROL_CAPABILITIES")
            .field(
                "scale_and_offset_supported",
                &format_args!("{:?}", self.scale_and_offset_supported),
            )
            .field(
                "max_converted_value",
                &format_args!("{:?}", self.max_converted_value),
            )
            .field(
                "min_converted_value",
                &format_args!("{:?}", self.min_converted_value),
            )
            .field(
                "num_gamma_control_points",
                &format_args!("{:?}", self.num_gamma_control_points),
            )
            .field(
                "control_point_positions",
                &format_args!("{:?}", self.control_point_positions),
            )
            .finish()
    }
}
impl ::std::cmp::PartialEq for DXGI_GAMMA_CONTROL_CAPABILITIES {
    fn eq(&self, other: &Self) -> bool {
        self.scale_and_offset_supported == other.scale_and_offset_supported
            && self.max_converted_value == other.max_converted_value
            && self.min_converted_value == other.min_converted_value
            && self.num_gamma_control_points == other.num_gamma_control_points
            && self.control_point_positions == other.control_point_positions
    }
}
impl ::std::cmp::Eq for DXGI_GAMMA_CONTROL_CAPABILITIES {}
unsafe impl ::windows::Abi for DXGI_GAMMA_CONTROL_CAPABILITIES {
    type Abi = Self;
}
#[allow(non_camel_case_types)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: marker :: Copy,
    :: std :: clone :: Clone,
    :: std :: default :: Default,
    :: std :: fmt :: Debug,
)]
#[repr(transparent)]
pub struct DXGI_GPU_PREFERENCE(pub i32);
impl DXGI_GPU_PREFERENCE {
    #![allow(non_upper_case_globals)]
    pub const DXGI_GPU_PREFERENCE_UNSPECIFIED: Self = Self(0i32);
    pub const DXGI_GPU_PREFERENCE_MINIMUM_POWER: Self = Self(1i32);
    pub const DXGI_GPU_PREFERENCE_HIGH_PERFORMANCE: Self = Self(2i32);
}
impl ::std::convert::From<i32> for DXGI_GPU_PREFERENCE {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::Abi for DXGI_GPU_PREFERENCE {
    type Abi = Self;
}
#[allow(non_camel_case_types)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: marker :: Copy,
    :: std :: clone :: Clone,
    :: std :: default :: Default,
    :: std :: fmt :: Debug,
)]
#[repr(transparent)]
pub struct DXGI_HARDWARE_COMPOSITION_SUPPORT_FLAGS(pub i32);
impl DXGI_HARDWARE_COMPOSITION_SUPPORT_FLAGS {
    #![allow(non_upper_case_globals)]
    pub const DXGI_HARDWARE_COMPOSITION_SUPPORT_FLAG_FULLSCREEN: Self = Self(1i32);
    pub const DXGI_HARDWARE_COMPOSITION_SUPPORT_FLAG_WINDOWED: Self = Self(2i32);
    pub const DXGI_HARDWARE_COMPOSITION_SUPPORT_FLAG_CURSOR_STRETCHED: Self = Self(4i32);
}
impl ::std::convert::From<i32> for DXGI_HARDWARE_COMPOSITION_SUPPORT_FLAGS {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::Abi for DXGI_HARDWARE_COMPOSITION_SUPPORT_FLAGS {
    type Abi = Self;
}
#[repr(C)]
#[allow(non_snake_case)]
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
pub struct DXGI_HDR_METADATA_HDR10 {
    pub red_primary: [u16; 2],
    pub green_primary: [u16; 2],
    pub blue_primary: [u16; 2],
    pub white_point: [u16; 2],
    pub max_mastering_luminance: u32,
    pub min_mastering_luminance: u32,
    pub max_content_light_level: u16,
    pub max_frame_average_light_level: u16,
}
impl DXGI_HDR_METADATA_HDR10 {}
impl ::std::default::Default for DXGI_HDR_METADATA_HDR10 {
    fn default() -> Self {
        Self {
            red_primary: [0; 2],
            green_primary: [0; 2],
            blue_primary: [0; 2],
            white_point: [0; 2],
            max_mastering_luminance: 0,
            min_mastering_luminance: 0,
            max_content_light_level: 0,
            max_frame_average_light_level: 0,
        }
    }
}
impl ::std::fmt::Debug for DXGI_HDR_METADATA_HDR10 {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("DXGI_HDR_METADATA_HDR10")
            .field("red_primary", &format_args!("{:?}", self.red_primary))
            .field("green_primary", &format_args!("{:?}", self.green_primary))
            .field("blue_primary", &format_args!("{:?}", self.blue_primary))
            .field("white_point", &format_args!("{:?}", self.white_point))
            .field(
                "max_mastering_luminance",
                &format_args!("{:?}", self.max_mastering_luminance),
            )
            .field(
                "min_mastering_luminance",
                &format_args!("{:?}", self.min_mastering_luminance),
            )
            .field(
                "max_content_light_level",
                &format_args!("{:?}", self.max_content_light_level),
            )
            .field(
                "max_frame_average_light_level",
                &format_args!("{:?}", self.max_frame_average_light_level),
            )
            .finish()
    }
}
impl ::std::cmp::PartialEq for DXGI_HDR_METADATA_HDR10 {
    fn eq(&self, other: &Self) -> bool {
        self.red_primary == other.red_primary
            && self.green_primary == other.green_primary
            && self.blue_primary == other.blue_primary
            && self.white_point == other.white_point
            && self.max_mastering_luminance == other.max_mastering_luminance
            && self.min_mastering_luminance == other.min_mastering_luminance
            && self.max_content_light_level == other.max_content_light_level
            && self.max_frame_average_light_level == other.max_frame_average_light_level
    }
}
impl ::std::cmp::Eq for DXGI_HDR_METADATA_HDR10 {}
unsafe impl ::windows::Abi for DXGI_HDR_METADATA_HDR10 {
    type Abi = Self;
}
#[repr(C)]
#[allow(non_snake_case)]
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
pub struct DXGI_HDR_METADATA_HDR10PLUS {
    pub data: [u8; 72],
}
impl DXGI_HDR_METADATA_HDR10PLUS {}
impl ::std::default::Default for DXGI_HDR_METADATA_HDR10PLUS {
    fn default() -> Self {
        Self { data: [0; 72] }
    }
}
impl ::std::fmt::Debug for DXGI_HDR_METADATA_HDR10PLUS {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("DXGI_HDR_METADATA_HDR10PLUS")
            .field("data", &format_args!("{:?}", self.data))
            .finish()
    }
}
impl ::std::cmp::PartialEq for DXGI_HDR_METADATA_HDR10PLUS {
    fn eq(&self, other: &Self) -> bool {
        self.data == other.data
    }
}
impl ::std::cmp::Eq for DXGI_HDR_METADATA_HDR10PLUS {}
unsafe impl ::windows::Abi for DXGI_HDR_METADATA_HDR10PLUS {
    type Abi = Self;
}
#[allow(non_camel_case_types)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: marker :: Copy,
    :: std :: clone :: Clone,
    :: std :: default :: Default,
    :: std :: fmt :: Debug,
)]
#[repr(transparent)]
pub struct DXGI_HDR_METADATA_TYPE(pub i32);
impl DXGI_HDR_METADATA_TYPE {
    #![allow(non_upper_case_globals)]
    pub const DXGI_HDR_METADATA_TYPE_NONE: Self = Self(0i32);
    pub const DXGI_HDR_METADATA_TYPE_HDR10: Self = Self(1i32);
    pub const DXGI_HDR_METADATA_TYPE_HDR10PLUS: Self = Self(2i32);
}
impl ::std::convert::From<i32> for DXGI_HDR_METADATA_TYPE {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::Abi for DXGI_HDR_METADATA_TYPE {
    type Abi = Self;
}
pub const DXGI_INFO_QUEUE_DEFAULT_MESSAGE_COUNT_LIMIT: u32 = 1024u32;
#[allow(non_camel_case_types)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: marker :: Copy,
    :: std :: clone :: Clone,
    :: std :: default :: Default,
    :: std :: fmt :: Debug,
)]
#[repr(transparent)]
pub struct DXGI_INFO_QUEUE_MESSAGE_CATEGORY(pub i32);
impl DXGI_INFO_QUEUE_MESSAGE_CATEGORY {
    #![allow(non_upper_case_globals)]
    pub const DXGI_INFO_QUEUE_MESSAGE_CATEGORY_UNKNOWN: Self = Self(0i32);
    pub const DXGI_INFO_QUEUE_MESSAGE_CATEGORY_MISCELLANEOUS: Self = Self(1i32);
    pub const DXGI_INFO_QUEUE_MESSAGE_CATEGORY_INITIALIZATION: Self = Self(2i32);
    pub const DXGI_INFO_QUEUE_MESSAGE_CATEGORY_CLEANUP: Self = Self(3i32);
    pub const DXGI_INFO_QUEUE_MESSAGE_CATEGORY_COMPILATION: Self = Self(4i32);
    pub const DXGI_INFO_QUEUE_MESSAGE_CATEGORY_STATE_CREATION: Self = Self(5i32);
    pub const DXGI_INFO_QUEUE_MESSAGE_CATEGORY_STATE_SETTING: Self = Self(6i32);
    pub const DXGI_INFO_QUEUE_MESSAGE_CATEGORY_STATE_GETTING: Self = Self(7i32);
    pub const DXGI_INFO_QUEUE_MESSAGE_CATEGORY_RESOURCE_MANIPULATION: Self = Self(8i32);
    pub const DXGI_INFO_QUEUE_MESSAGE_CATEGORY_EXECUTION: Self = Self(9i32);
    pub const DXGI_INFO_QUEUE_MESSAGE_CATEGORY_SHADER: Self = Self(10i32);
}
impl ::std::convert::From<i32> for DXGI_INFO_QUEUE_MESSAGE_CATEGORY {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::Abi for DXGI_INFO_QUEUE_MESSAGE_CATEGORY {
    type Abi = Self;
}
#[allow(non_camel_case_types)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: marker :: Copy,
    :: std :: clone :: Clone,
    :: std :: default :: Default,
    :: std :: fmt :: Debug,
)]
#[repr(transparent)]
pub struct DXGI_INFO_QUEUE_MESSAGE_SEVERITY(pub i32);
impl DXGI_INFO_QUEUE_MESSAGE_SEVERITY {
    #![allow(non_upper_case_globals)]
    pub const DXGI_INFO_QUEUE_MESSAGE_SEVERITY_CORRUPTION: Self = Self(0i32);
    pub const DXGI_INFO_QUEUE_MESSAGE_SEVERITY_ERROR: Self = Self(1i32);
    pub const DXGI_INFO_QUEUE_MESSAGE_SEVERITY_WARNING: Self = Self(2i32);
    pub const DXGI_INFO_QUEUE_MESSAGE_SEVERITY_INFO: Self = Self(3i32);
    pub const DXGI_INFO_QUEUE_MESSAGE_SEVERITY_MESSAGE: Self = Self(4i32);
}
impl ::std::convert::From<i32> for DXGI_INFO_QUEUE_MESSAGE_SEVERITY {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::Abi for DXGI_INFO_QUEUE_MESSAGE_SEVERITY {
    type Abi = Self;
}
#[repr(C)]
#[allow(non_snake_case)]
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
pub struct DXGI_INFO_QUEUE_FILTER_DESC {
    pub num_categories: u32,
    pub p_category_list: *mut DXGI_INFO_QUEUE_MESSAGE_CATEGORY,
    pub num_severities: u32,
    pub p_severity_list: *mut DXGI_INFO_QUEUE_MESSAGE_SEVERITY,
    pub num_ids: u32,
    pub p_id_list: *mut i32,
}
impl DXGI_INFO_QUEUE_FILTER_DESC {}
impl ::std::default::Default for DXGI_INFO_QUEUE_FILTER_DESC {
    fn default() -> Self {
        Self {
            num_categories: 0,
            p_category_list: ::std::ptr::null_mut(),
            num_severities: 0,
            p_severity_list: ::std::ptr::null_mut(),
            num_ids: 0,
            p_id_list: ::std::ptr::null_mut(),
        }
    }
}
impl ::std::fmt::Debug for DXGI_INFO_QUEUE_FILTER_DESC {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("DXGI_INFO_QUEUE_FILTER_DESC")
            .field("num_categories", &format_args!("{:?}", self.num_categories))
            .field(
                "p_category_list",
                &format_args!("{:?}", self.p_category_list),
            )
            .field("num_severities", &format_args!("{:?}", self.num_severities))
            .field(
                "p_severity_list",
                &format_args!("{:?}", self.p_severity_list),
            )
            .field("num_ids", &format_args!("{:?}", self.num_ids))
            .field("p_id_list", &format_args!("{:?}", self.p_id_list))
            .finish()
    }
}
impl ::std::cmp::PartialEq for DXGI_INFO_QUEUE_FILTER_DESC {
    fn eq(&self, other: &Self) -> bool {
        self.num_categories == other.num_categories
            && self.p_category_list == other.p_category_list
            && self.num_severities == other.num_severities
            && self.p_severity_list == other.p_severity_list
            && self.num_ids == other.num_ids
            && self.p_id_list == other.p_id_list
    }
}
impl ::std::cmp::Eq for DXGI_INFO_QUEUE_FILTER_DESC {}
unsafe impl ::windows::Abi for DXGI_INFO_QUEUE_FILTER_DESC {
    type Abi = Self;
}
#[repr(C)]
#[allow(non_snake_case)]
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
pub struct DXGI_INFO_QUEUE_FILTER {
    pub allow_list: DXGI_INFO_QUEUE_FILTER_DESC,
    pub deny_list: DXGI_INFO_QUEUE_FILTER_DESC,
}
impl DXGI_INFO_QUEUE_FILTER {}
impl ::std::default::Default for DXGI_INFO_QUEUE_FILTER {
    fn default() -> Self {
        Self {
            allow_list: ::std::default::Default::default(),
            deny_list: ::std::default::Default::default(),
        }
    }
}
impl ::std::fmt::Debug for DXGI_INFO_QUEUE_FILTER {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("DXGI_INFO_QUEUE_FILTER")
            .field("allow_list", &format_args!("{:?}", self.allow_list))
            .field("deny_list", &format_args!("{:?}", self.deny_list))
            .finish()
    }
}
impl ::std::cmp::PartialEq for DXGI_INFO_QUEUE_FILTER {
    fn eq(&self, other: &Self) -> bool {
        self.allow_list == other.allow_list && self.deny_list == other.deny_list
    }
}
impl ::std::cmp::Eq for DXGI_INFO_QUEUE_FILTER {}
unsafe impl ::windows::Abi for DXGI_INFO_QUEUE_FILTER {
    type Abi = Self;
}
#[repr(C)]
#[allow(non_snake_case)]
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
pub struct DXGI_INFO_QUEUE_MESSAGE {
    pub producer: ::windows::Guid,
    pub category: DXGI_INFO_QUEUE_MESSAGE_CATEGORY,
    pub severity: DXGI_INFO_QUEUE_MESSAGE_SEVERITY,
    pub id: i32,
    pub p_description: *mut u8,
    pub description_byte_length: usize,
}
impl DXGI_INFO_QUEUE_MESSAGE {}
impl ::std::default::Default for DXGI_INFO_QUEUE_MESSAGE {
    fn default() -> Self {
        Self {
            producer: ::std::default::Default::default(),
            category: ::std::default::Default::default(),
            severity: ::std::default::Default::default(),
            id: 0,
            p_description: ::std::ptr::null_mut(),
            description_byte_length: 0,
        }
    }
}
impl ::std::fmt::Debug for DXGI_INFO_QUEUE_MESSAGE {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("DXGI_INFO_QUEUE_MESSAGE")
            .field("producer", &format_args!("{:?}", self.producer))
            .field("category", &format_args!("{:?}", self.category))
            .field("severity", &format_args!("{:?}", self.severity))
            .field("id", &format_args!("{:?}", self.id))
            .field("p_description", &format_args!("{:?}", self.p_description))
            .field(
                "description_byte_length",
                &format_args!("{:?}", self.description_byte_length),
            )
            .finish()
    }
}
impl ::std::cmp::PartialEq for DXGI_INFO_QUEUE_MESSAGE {
    fn eq(&self, other: &Self) -> bool {
        self.producer == other.producer
            && self.category == other.category
            && self.severity == other.severity
            && self.id == other.id
            && self.p_description == other.p_description
            && self.description_byte_length == other.description_byte_length
    }
}
impl ::std::cmp::Eq for DXGI_INFO_QUEUE_MESSAGE {}
unsafe impl ::windows::Abi for DXGI_INFO_QUEUE_MESSAGE {
    type Abi = Self;
}
pub const DXGI_INFO_QUEUE_MESSAGE_ID_STRING_FROM_APPLICATION: u32 = 0u32;
#[repr(C)]
#[allow(non_snake_case)]
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
pub struct DXGI_JPEG_AC_HUFFMAN_TABLE {
    pub code_counts: [u8; 16],
    pub code_values: [u8; 162],
}
impl DXGI_JPEG_AC_HUFFMAN_TABLE {}
impl ::std::default::Default for DXGI_JPEG_AC_HUFFMAN_TABLE {
    fn default() -> Self {
        Self {
            code_counts: [0; 16],
            code_values: [0; 162],
        }
    }
}
impl ::std::fmt::Debug for DXGI_JPEG_AC_HUFFMAN_TABLE {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("DXGI_JPEG_AC_HUFFMAN_TABLE")
            .field("code_counts", &format_args!("{:?}", self.code_counts))
            .field("code_values", &format_args!("{:?}", self.code_values))
            .finish()
    }
}
impl ::std::cmp::PartialEq for DXGI_JPEG_AC_HUFFMAN_TABLE {
    fn eq(&self, other: &Self) -> bool {
        self.code_counts == other.code_counts && self.code_values == other.code_values
    }
}
impl ::std::cmp::Eq for DXGI_JPEG_AC_HUFFMAN_TABLE {}
unsafe impl ::windows::Abi for DXGI_JPEG_AC_HUFFMAN_TABLE {
    type Abi = Self;
}
#[repr(C)]
#[allow(non_snake_case)]
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
pub struct DXGI_JPEG_DC_HUFFMAN_TABLE {
    pub code_counts: [u8; 12],
    pub code_values: [u8; 12],
}
impl DXGI_JPEG_DC_HUFFMAN_TABLE {}
impl ::std::default::Default for DXGI_JPEG_DC_HUFFMAN_TABLE {
    fn default() -> Self {
        Self {
            code_counts: [0; 12],
            code_values: [0; 12],
        }
    }
}
impl ::std::fmt::Debug for DXGI_JPEG_DC_HUFFMAN_TABLE {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("DXGI_JPEG_DC_HUFFMAN_TABLE")
            .field("code_counts", &format_args!("{:?}", self.code_counts))
            .field("code_values", &format_args!("{:?}", self.code_values))
            .finish()
    }
}
impl ::std::cmp::PartialEq for DXGI_JPEG_DC_HUFFMAN_TABLE {
    fn eq(&self, other: &Self) -> bool {
        self.code_counts == other.code_counts && self.code_values == other.code_values
    }
}
impl ::std::cmp::Eq for DXGI_JPEG_DC_HUFFMAN_TABLE {}
unsafe impl ::windows::Abi for DXGI_JPEG_DC_HUFFMAN_TABLE {
    type Abi = Self;
}
#[repr(C)]
#[allow(non_snake_case)]
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
pub struct DXGI_JPEG_QUANTIZATION_TABLE {
    pub elements: [u8; 64],
}
impl DXGI_JPEG_QUANTIZATION_TABLE {}
impl ::std::default::Default for DXGI_JPEG_QUANTIZATION_TABLE {
    fn default() -> Self {
        Self { elements: [0; 64] }
    }
}
impl ::std::fmt::Debug for DXGI_JPEG_QUANTIZATION_TABLE {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("DXGI_JPEG_QUANTIZATION_TABLE")
            .field("elements", &format_args!("{:?}", self.elements))
            .finish()
    }
}
impl ::std::cmp::PartialEq for DXGI_JPEG_QUANTIZATION_TABLE {
    fn eq(&self, other: &Self) -> bool {
        self.elements == other.elements
    }
}
impl ::std::cmp::Eq for DXGI_JPEG_QUANTIZATION_TABLE {}
unsafe impl ::windows::Abi for DXGI_JPEG_QUANTIZATION_TABLE {
    type Abi = Self;
}
#[repr(C)]
#[allow(non_snake_case)]
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
pub struct DXGI_MAPPED_RECT {
    pub pitch: i32,
    pub p_bits: *mut u8,
}
impl DXGI_MAPPED_RECT {}
impl ::std::default::Default for DXGI_MAPPED_RECT {
    fn default() -> Self {
        Self {
            pitch: 0,
            p_bits: ::std::ptr::null_mut(),
        }
    }
}
impl ::std::fmt::Debug for DXGI_MAPPED_RECT {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("DXGI_MAPPED_RECT")
            .field("pitch", &format_args!("{:?}", self.pitch))
            .field("p_bits", &format_args!("{:?}", self.p_bits))
            .finish()
    }
}
impl ::std::cmp::PartialEq for DXGI_MAPPED_RECT {
    fn eq(&self, other: &Self) -> bool {
        self.pitch == other.pitch && self.p_bits == other.p_bits
    }
}
impl ::std::cmp::Eq for DXGI_MAPPED_RECT {}
unsafe impl ::windows::Abi for DXGI_MAPPED_RECT {
    type Abi = Self;
}
pub const DXGI_MAP_DISCARD: u32 = 4u32;
pub const DXGI_MAP_READ: u32 = 1u32;
pub const DXGI_MAP_WRITE: u32 = 2u32;
#[repr(C)]
#[allow(non_snake_case)]
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
pub struct DXGI_MATRIX_3X2_F {
    pub _11: f32,
    pub _12: f32,
    pub _21: f32,
    pub _22: f32,
    pub _31: f32,
    pub _32: f32,
}
impl DXGI_MATRIX_3X2_F {}
impl ::std::default::Default for DXGI_MATRIX_3X2_F {
    fn default() -> Self {
        Self {
            _11: 0.0,
            _12: 0.0,
            _21: 0.0,
            _22: 0.0,
            _31: 0.0,
            _32: 0.0,
        }
    }
}
impl ::std::fmt::Debug for DXGI_MATRIX_3X2_F {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("DXGI_MATRIX_3X2_F")
            .field("_11", &format_args!("{:?}", self._11))
            .field("_12", &format_args!("{:?}", self._12))
            .field("_21", &format_args!("{:?}", self._21))
            .field("_22", &format_args!("{:?}", self._22))
            .field("_31", &format_args!("{:?}", self._31))
            .field("_32", &format_args!("{:?}", self._32))
            .finish()
    }
}
impl ::std::cmp::PartialEq for DXGI_MATRIX_3X2_F {
    fn eq(&self, other: &Self) -> bool {
        self._11 == other._11
            && self._12 == other._12
            && self._21 == other._21
            && self._22 == other._22
            && self._31 == other._31
            && self._32 == other._32
    }
}
impl ::std::cmp::Eq for DXGI_MATRIX_3X2_F {}
unsafe impl ::windows::Abi for DXGI_MATRIX_3X2_F {
    type Abi = Self;
}
pub const DXGI_MAX_SWAP_CHAIN_BUFFERS: u32 = 16u32;
#[allow(non_camel_case_types)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: marker :: Copy,
    :: std :: clone :: Clone,
    :: std :: default :: Default,
    :: std :: fmt :: Debug,
)]
#[repr(transparent)]
pub struct DXGI_MEMORY_SEGMENT_GROUP(pub i32);
impl DXGI_MEMORY_SEGMENT_GROUP {
    #![allow(non_upper_case_globals)]
    pub const DXGI_MEMORY_SEGMENT_GROUP_LOCAL: Self = Self(0i32);
    pub const DXGI_MEMORY_SEGMENT_GROUP_NON_LOCAL: Self = Self(1i32);
}
impl ::std::convert::From<i32> for DXGI_MEMORY_SEGMENT_GROUP {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::Abi for DXGI_MEMORY_SEGMENT_GROUP {
    type Abi = Self;
}
#[repr(C)]
#[allow(non_snake_case)]
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
pub struct DXGI_RATIONAL {
    pub numerator: u32,
    pub denominator: u32,
}
impl DXGI_RATIONAL {}
impl ::std::default::Default for DXGI_RATIONAL {
    fn default() -> Self {
        Self {
            numerator: 0,
            denominator: 0,
        }
    }
}
impl ::std::fmt::Debug for DXGI_RATIONAL {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("DXGI_RATIONAL")
            .field("numerator", &format_args!("{:?}", self.numerator))
            .field("denominator", &format_args!("{:?}", self.denominator))
            .finish()
    }
}
impl ::std::cmp::PartialEq for DXGI_RATIONAL {
    fn eq(&self, other: &Self) -> bool {
        self.numerator == other.numerator && self.denominator == other.denominator
    }
}
impl ::std::cmp::Eq for DXGI_RATIONAL {}
unsafe impl ::windows::Abi for DXGI_RATIONAL {
    type Abi = Self;
}
#[allow(non_camel_case_types)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: marker :: Copy,
    :: std :: clone :: Clone,
    :: std :: default :: Default,
    :: std :: fmt :: Debug,
)]
#[repr(transparent)]
pub struct DXGI_MODE_SCANLINE_ORDER(pub i32);
impl DXGI_MODE_SCANLINE_ORDER {
    #![allow(non_upper_case_globals)]
    pub const DXGI_MODE_SCANLINE_ORDER_UNSPECIFIED: Self = Self(0i32);
    pub const DXGI_MODE_SCANLINE_ORDER_PROGRESSIVE: Self = Self(1i32);
    pub const DXGI_MODE_SCANLINE_ORDER_UPPER_FIELD_FIRST: Self = Self(2i32);
    pub const DXGI_MODE_SCANLINE_ORDER_LOWER_FIELD_FIRST: Self = Self(3i32);
}
impl ::std::convert::From<i32> for DXGI_MODE_SCANLINE_ORDER {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::Abi for DXGI_MODE_SCANLINE_ORDER {
    type Abi = Self;
}
#[allow(non_camel_case_types)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: marker :: Copy,
    :: std :: clone :: Clone,
    :: std :: default :: Default,
    :: std :: fmt :: Debug,
)]
#[repr(transparent)]
pub struct DXGI_MODE_SCALING(pub i32);
impl DXGI_MODE_SCALING {
    #![allow(non_upper_case_globals)]
    pub const DXGI_MODE_SCALING_UNSPECIFIED: Self = Self(0i32);
    pub const DXGI_MODE_SCALING_CENTERED: Self = Self(1i32);
    pub const DXGI_MODE_SCALING_STRETCHED: Self = Self(2i32);
}
impl ::std::convert::From<i32> for DXGI_MODE_SCALING {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::Abi for DXGI_MODE_SCALING {
    type Abi = Self;
}
#[repr(C)]
#[allow(non_snake_case)]
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
pub struct DXGI_MODE_DESC {
    pub width: u32,
    pub height: u32,
    pub refresh_rate: DXGI_RATIONAL,
    pub format: DXGI_FORMAT,
    pub scanline_ordering: DXGI_MODE_SCANLINE_ORDER,
    pub scaling: DXGI_MODE_SCALING,
}
impl DXGI_MODE_DESC {}
impl ::std::default::Default for DXGI_MODE_DESC {
    fn default() -> Self {
        Self {
            width: 0,
            height: 0,
            refresh_rate: ::std::default::Default::default(),
            format: ::std::default::Default::default(),
            scanline_ordering: ::std::default::Default::default(),
            scaling: ::std::default::Default::default(),
        }
    }
}
impl ::std::fmt::Debug for DXGI_MODE_DESC {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("DXGI_MODE_DESC")
            .field("width", &format_args!("{:?}", self.width))
            .field("height", &format_args!("{:?}", self.height))
            .field("refresh_rate", &format_args!("{:?}", self.refresh_rate))
            .field("format", &format_args!("{:?}", self.format))
            .field(
                "scanline_ordering",
                &format_args!("{:?}", self.scanline_ordering),
            )
            .field("scaling", &format_args!("{:?}", self.scaling))
            .finish()
    }
}
impl ::std::cmp::PartialEq for DXGI_MODE_DESC {
    fn eq(&self, other: &Self) -> bool {
        self.width == other.width
            && self.height == other.height
            && self.refresh_rate == other.refresh_rate
            && self.format == other.format
            && self.scanline_ordering == other.scanline_ordering
            && self.scaling == other.scaling
    }
}
impl ::std::cmp::Eq for DXGI_MODE_DESC {}
unsafe impl ::windows::Abi for DXGI_MODE_DESC {
    type Abi = Self;
}
#[repr(C)]
#[allow(non_snake_case)]
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
pub struct DXGI_MODE_DESC1 {
    pub width: u32,
    pub height: u32,
    pub refresh_rate: DXGI_RATIONAL,
    pub format: DXGI_FORMAT,
    pub scanline_ordering: DXGI_MODE_SCANLINE_ORDER,
    pub scaling: DXGI_MODE_SCALING,
    pub stereo: super::system_services::BOOL,
}
impl DXGI_MODE_DESC1 {}
impl ::std::default::Default for DXGI_MODE_DESC1 {
    fn default() -> Self {
        Self {
            width: 0,
            height: 0,
            refresh_rate: ::std::default::Default::default(),
            format: ::std::default::Default::default(),
            scanline_ordering: ::std::default::Default::default(),
            scaling: ::std::default::Default::default(),
            stereo: ::std::default::Default::default(),
        }
    }
}
impl ::std::fmt::Debug for DXGI_MODE_DESC1 {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("DXGI_MODE_DESC1")
            .field("width", &format_args!("{:?}", self.width))
            .field("height", &format_args!("{:?}", self.height))
            .field("refresh_rate", &format_args!("{:?}", self.refresh_rate))
            .field("format", &format_args!("{:?}", self.format))
            .field(
                "scanline_ordering",
                &format_args!("{:?}", self.scanline_ordering),
            )
            .field("scaling", &format_args!("{:?}", self.scaling))
            .field("stereo", &format_args!("{:?}", self.stereo))
            .finish()
    }
}
impl ::std::cmp::PartialEq for DXGI_MODE_DESC1 {
    fn eq(&self, other: &Self) -> bool {
        self.width == other.width
            && self.height == other.height
            && self.refresh_rate == other.refresh_rate
            && self.format == other.format
            && self.scanline_ordering == other.scanline_ordering
            && self.scaling == other.scaling
            && self.stereo == other.stereo
    }
}
impl ::std::cmp::Eq for DXGI_MODE_DESC1 {}
unsafe impl ::windows::Abi for DXGI_MODE_DESC1 {
    type Abi = Self;
}
#[allow(non_camel_case_types)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: marker :: Copy,
    :: std :: clone :: Clone,
    :: std :: default :: Default,
    :: std :: fmt :: Debug,
)]
#[repr(transparent)]
pub struct DXGI_MODE_ROTATION(pub i32);
impl DXGI_MODE_ROTATION {
    #![allow(non_upper_case_globals)]
    pub const DXGI_MODE_ROTATION_UNSPECIFIED: Self = Self(0i32);
    pub const DXGI_MODE_ROTATION_IDENTITY: Self = Self(1i32);
    pub const DXGI_MODE_ROTATION_ROTATE90: Self = Self(2i32);
    pub const DXGI_MODE_ROTATION_ROTATE180: Self = Self(3i32);
    pub const DXGI_MODE_ROTATION_ROTATE270: Self = Self(4i32);
}
impl ::std::convert::From<i32> for DXGI_MODE_ROTATION {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::Abi for DXGI_MODE_ROTATION {
    type Abi = Self;
}
#[allow(non_camel_case_types)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: marker :: Copy,
    :: std :: clone :: Clone,
    :: std :: default :: Default,
    :: std :: fmt :: Debug,
)]
#[repr(transparent)]
pub struct DXGI_MULTIPLANE_OVERLAY_YCbCr_FLAGS(pub i32);
impl DXGI_MULTIPLANE_OVERLAY_YCbCr_FLAGS {
    #![allow(non_upper_case_globals)]
    pub const DXGI_MULTIPLANE_OVERLAY_YCbCr_FLAG_NOMINAL_RANGE: Self = Self(1i32);
    pub const DXGI_MULTIPLANE_OVERLAY_YCbCr_FLAG_BT709: Self = Self(2i32);
    pub const DXGI_MULTIPLANE_OVERLAY_YCbCr_FLAG_xvYCC: Self = Self(4i32);
}
impl ::std::convert::From<i32> for DXGI_MULTIPLANE_OVERLAY_YCbCr_FLAGS {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::Abi for DXGI_MULTIPLANE_OVERLAY_YCbCr_FLAGS {
    type Abi = Self;
}
pub const DXGI_MWA_NO_ALT_ENTER: u32 = 2u32;
pub const DXGI_MWA_NO_PRINT_SCREEN: u32 = 4u32;
pub const DXGI_MWA_NO_WINDOW_CHANGES: u32 = 1u32;
pub const DXGI_MWA_VALID: u32 = 7u32;
#[allow(non_camel_case_types)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: marker :: Copy,
    :: std :: clone :: Clone,
    :: std :: default :: Default,
    :: std :: fmt :: Debug,
)]
#[repr(transparent)]
pub struct DXGI_OFFER_RESOURCE_FLAGS(pub i32);
impl DXGI_OFFER_RESOURCE_FLAGS {
    #![allow(non_upper_case_globals)]
    pub const DXGI_OFFER_RESOURCE_FLAG_ALLOW_DECOMMIT: Self = Self(1i32);
}
impl ::std::convert::From<i32> for DXGI_OFFER_RESOURCE_FLAGS {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::Abi for DXGI_OFFER_RESOURCE_FLAGS {
    type Abi = Self;
}
#[allow(non_camel_case_types)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: marker :: Copy,
    :: std :: clone :: Clone,
    :: std :: default :: Default,
    :: std :: fmt :: Debug,
)]
#[repr(transparent)]
pub struct DXGI_OFFER_RESOURCE_PRIORITY(pub i32);
impl DXGI_OFFER_RESOURCE_PRIORITY {
    #![allow(non_upper_case_globals)]
    pub const DXGI_OFFER_RESOURCE_PRIORITY_LOW: Self = Self(1i32);
    pub const DXGI_OFFER_RESOURCE_PRIORITY_NORMAL: Self = Self(2i32);
    pub const DXGI_OFFER_RESOURCE_PRIORITY_HIGH: Self = Self(3i32);
}
impl ::std::convert::From<i32> for DXGI_OFFER_RESOURCE_PRIORITY {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::Abi for DXGI_OFFER_RESOURCE_PRIORITY {
    type Abi = Self;
}
#[repr(C)]
#[allow(non_snake_case)]
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
pub struct DXGI_OUTDUPL_DESC {
    pub mode_desc: DXGI_MODE_DESC,
    pub rotation: DXGI_MODE_ROTATION,
    pub desktop_image_in_system_memory: super::system_services::BOOL,
}
impl DXGI_OUTDUPL_DESC {}
impl ::std::default::Default for DXGI_OUTDUPL_DESC {
    fn default() -> Self {
        Self {
            mode_desc: ::std::default::Default::default(),
            rotation: ::std::default::Default::default(),
            desktop_image_in_system_memory: ::std::default::Default::default(),
        }
    }
}
impl ::std::fmt::Debug for DXGI_OUTDUPL_DESC {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("DXGI_OUTDUPL_DESC")
            .field("mode_desc", &format_args!("{:?}", self.mode_desc))
            .field("rotation", &format_args!("{:?}", self.rotation))
            .field(
                "desktop_image_in_system_memory",
                &format_args!("{:?}", self.desktop_image_in_system_memory),
            )
            .finish()
    }
}
impl ::std::cmp::PartialEq for DXGI_OUTDUPL_DESC {
    fn eq(&self, other: &Self) -> bool {
        self.mode_desc == other.mode_desc
            && self.rotation == other.rotation
            && self.desktop_image_in_system_memory == other.desktop_image_in_system_memory
    }
}
impl ::std::cmp::Eq for DXGI_OUTDUPL_DESC {}
unsafe impl ::windows::Abi for DXGI_OUTDUPL_DESC {
    type Abi = Self;
}
#[allow(non_camel_case_types)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: marker :: Copy,
    :: std :: clone :: Clone,
    :: std :: default :: Default,
    :: std :: fmt :: Debug,
)]
#[repr(transparent)]
pub struct DXGI_OUTDUPL_FLAG(pub i32);
impl DXGI_OUTDUPL_FLAG {
    #![allow(non_upper_case_globals)]
    pub const DXGI_OUTDUPL_COMPOSITED_UI_CAPTURE_ONLY: Self = Self(1i32);
}
impl ::std::convert::From<i32> for DXGI_OUTDUPL_FLAG {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::Abi for DXGI_OUTDUPL_FLAG {
    type Abi = Self;
}
#[repr(C)]
#[allow(non_snake_case)]
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
pub struct DXGI_OUTDUPL_POINTER_POSITION {
    pub position: super::display_devices::POINT,
    pub visible: super::system_services::BOOL,
}
impl DXGI_OUTDUPL_POINTER_POSITION {}
impl ::std::default::Default for DXGI_OUTDUPL_POINTER_POSITION {
    fn default() -> Self {
        Self {
            position: ::std::default::Default::default(),
            visible: ::std::default::Default::default(),
        }
    }
}
impl ::std::fmt::Debug for DXGI_OUTDUPL_POINTER_POSITION {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("DXGI_OUTDUPL_POINTER_POSITION")
            .field("position", &format_args!("{:?}", self.position))
            .field("visible", &format_args!("{:?}", self.visible))
            .finish()
    }
}
impl ::std::cmp::PartialEq for DXGI_OUTDUPL_POINTER_POSITION {
    fn eq(&self, other: &Self) -> bool {
        self.position == other.position && self.visible == other.visible
    }
}
impl ::std::cmp::Eq for DXGI_OUTDUPL_POINTER_POSITION {}
unsafe impl ::windows::Abi for DXGI_OUTDUPL_POINTER_POSITION {
    type Abi = Self;
}
#[repr(C)]
#[allow(non_snake_case)]
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
pub struct DXGI_OUTDUPL_FRAME_INFO {
    pub last_present_time: i64,
    pub last_mouse_update_time: i64,
    pub accumulated_frames: u32,
    pub rects_coalesced: super::system_services::BOOL,
    pub protected_content_masked_out: super::system_services::BOOL,
    pub pointer_position: DXGI_OUTDUPL_POINTER_POSITION,
    pub total_metadata_buffer_size: u32,
    pub pointer_shape_buffer_size: u32,
}
impl DXGI_OUTDUPL_FRAME_INFO {}
impl ::std::default::Default for DXGI_OUTDUPL_FRAME_INFO {
    fn default() -> Self {
        Self {
            last_present_time: 0,
            last_mouse_update_time: 0,
            accumulated_frames: 0,
            rects_coalesced: ::std::default::Default::default(),
            protected_content_masked_out: ::std::default::Default::default(),
            pointer_position: ::std::default::Default::default(),
            total_metadata_buffer_size: 0,
            pointer_shape_buffer_size: 0,
        }
    }
}
impl ::std::fmt::Debug for DXGI_OUTDUPL_FRAME_INFO {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("DXGI_OUTDUPL_FRAME_INFO")
            .field(
                "last_present_time",
                &format_args!("{:?}", self.last_present_time),
            )
            .field(
                "last_mouse_update_time",
                &format_args!("{:?}", self.last_mouse_update_time),
            )
            .field(
                "accumulated_frames",
                &format_args!("{:?}", self.accumulated_frames),
            )
            .field(
                "rects_coalesced",
                &format_args!("{:?}", self.rects_coalesced),
            )
            .field(
                "protected_content_masked_out",
                &format_args!("{:?}", self.protected_content_masked_out),
            )
            .field(
                "pointer_position",
                &format_args!("{:?}", self.pointer_position),
            )
            .field(
                "total_metadata_buffer_size",
                &format_args!("{:?}", self.total_metadata_buffer_size),
            )
            .field(
                "pointer_shape_buffer_size",
                &format_args!("{:?}", self.pointer_shape_buffer_size),
            )
            .finish()
    }
}
impl ::std::cmp::PartialEq for DXGI_OUTDUPL_FRAME_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.last_present_time == other.last_present_time
            && self.last_mouse_update_time == other.last_mouse_update_time
            && self.accumulated_frames == other.accumulated_frames
            && self.rects_coalesced == other.rects_coalesced
            && self.protected_content_masked_out == other.protected_content_masked_out
            && self.pointer_position == other.pointer_position
            && self.total_metadata_buffer_size == other.total_metadata_buffer_size
            && self.pointer_shape_buffer_size == other.pointer_shape_buffer_size
    }
}
impl ::std::cmp::Eq for DXGI_OUTDUPL_FRAME_INFO {}
unsafe impl ::windows::Abi for DXGI_OUTDUPL_FRAME_INFO {
    type Abi = Self;
}
#[repr(C)]
#[allow(non_snake_case)]
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
pub struct DXGI_OUTDUPL_MOVE_RECT {
    pub source_point: super::display_devices::POINT,
    pub destination_rect: super::display_devices::RECT,
}
impl DXGI_OUTDUPL_MOVE_RECT {}
impl ::std::default::Default for DXGI_OUTDUPL_MOVE_RECT {
    fn default() -> Self {
        Self {
            source_point: ::std::default::Default::default(),
            destination_rect: ::std::default::Default::default(),
        }
    }
}
impl ::std::fmt::Debug for DXGI_OUTDUPL_MOVE_RECT {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("DXGI_OUTDUPL_MOVE_RECT")
            .field("source_point", &format_args!("{:?}", self.source_point))
            .field(
                "destination_rect",
                &format_args!("{:?}", self.destination_rect),
            )
            .finish()
    }
}
impl ::std::cmp::PartialEq for DXGI_OUTDUPL_MOVE_RECT {
    fn eq(&self, other: &Self) -> bool {
        self.source_point == other.source_point && self.destination_rect == other.destination_rect
    }
}
impl ::std::cmp::Eq for DXGI_OUTDUPL_MOVE_RECT {}
unsafe impl ::windows::Abi for DXGI_OUTDUPL_MOVE_RECT {
    type Abi = Self;
}
#[repr(C)]
#[allow(non_snake_case)]
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
pub struct DXGI_OUTDUPL_POINTER_SHAPE_INFO {
    pub r#type: u32,
    pub width: u32,
    pub height: u32,
    pub pitch: u32,
    pub hot_spot: super::display_devices::POINT,
}
impl DXGI_OUTDUPL_POINTER_SHAPE_INFO {}
impl ::std::default::Default for DXGI_OUTDUPL_POINTER_SHAPE_INFO {
    fn default() -> Self {
        Self {
            r#type: 0,
            width: 0,
            height: 0,
            pitch: 0,
            hot_spot: ::std::default::Default::default(),
        }
    }
}
impl ::std::fmt::Debug for DXGI_OUTDUPL_POINTER_SHAPE_INFO {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("DXGI_OUTDUPL_POINTER_SHAPE_INFO")
            .field("r#type", &format_args!("{:?}", self.r#type))
            .field("width", &format_args!("{:?}", self.width))
            .field("height", &format_args!("{:?}", self.height))
            .field("pitch", &format_args!("{:?}", self.pitch))
            .field("hot_spot", &format_args!("{:?}", self.hot_spot))
            .finish()
    }
}
impl ::std::cmp::PartialEq for DXGI_OUTDUPL_POINTER_SHAPE_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.r#type == other.r#type
            && self.width == other.width
            && self.height == other.height
            && self.pitch == other.pitch
            && self.hot_spot == other.hot_spot
    }
}
impl ::std::cmp::Eq for DXGI_OUTDUPL_POINTER_SHAPE_INFO {}
unsafe impl ::windows::Abi for DXGI_OUTDUPL_POINTER_SHAPE_INFO {
    type Abi = Self;
}
#[allow(non_camel_case_types)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: marker :: Copy,
    :: std :: clone :: Clone,
    :: std :: default :: Default,
    :: std :: fmt :: Debug,
)]
#[repr(transparent)]
pub struct DXGI_OUTDUPL_POINTER_SHAPE_TYPE(pub i32);
impl DXGI_OUTDUPL_POINTER_SHAPE_TYPE {
    #![allow(non_upper_case_globals)]
    pub const DXGI_OUTDUPL_POINTER_SHAPE_TYPE_MONOCHROME: Self = Self(1i32);
    pub const DXGI_OUTDUPL_POINTER_SHAPE_TYPE_COLOR: Self = Self(2i32);
    pub const DXGI_OUTDUPL_POINTER_SHAPE_TYPE_MASKED_COLOR: Self = Self(4i32);
}
impl ::std::convert::From<i32> for DXGI_OUTDUPL_POINTER_SHAPE_TYPE {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::Abi for DXGI_OUTDUPL_POINTER_SHAPE_TYPE {
    type Abi = Self;
}
#[repr(C)]
#[allow(non_snake_case)]
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
pub struct DXGI_OUTPUT_DESC {
    pub device_name: [u16; 32],
    pub desktop_coordinates: super::display_devices::RECT,
    pub attached_to_desktop: super::system_services::BOOL,
    pub rotation: DXGI_MODE_ROTATION,
    pub monitor: super::gdi::HMONITOR,
}
impl DXGI_OUTPUT_DESC {}
impl ::std::default::Default for DXGI_OUTPUT_DESC {
    fn default() -> Self {
        Self {
            device_name: [0; 32],
            desktop_coordinates: ::std::default::Default::default(),
            attached_to_desktop: ::std::default::Default::default(),
            rotation: ::std::default::Default::default(),
            monitor: ::std::default::Default::default(),
        }
    }
}
impl ::std::fmt::Debug for DXGI_OUTPUT_DESC {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("DXGI_OUTPUT_DESC")
            .field("device_name", &format_args!("{:?}", self.device_name))
            .field(
                "desktop_coordinates",
                &format_args!("{:?}", self.desktop_coordinates),
            )
            .field(
                "attached_to_desktop",
                &format_args!("{:?}", self.attached_to_desktop),
            )
            .field("rotation", &format_args!("{:?}", self.rotation))
            .field("monitor", &format_args!("{:?}", self.monitor))
            .finish()
    }
}
impl ::std::cmp::PartialEq for DXGI_OUTPUT_DESC {
    fn eq(&self, other: &Self) -> bool {
        self.device_name == other.device_name
            && self.desktop_coordinates == other.desktop_coordinates
            && self.attached_to_desktop == other.attached_to_desktop
            && self.rotation == other.rotation
            && self.monitor == other.monitor
    }
}
impl ::std::cmp::Eq for DXGI_OUTPUT_DESC {}
unsafe impl ::windows::Abi for DXGI_OUTPUT_DESC {
    type Abi = Self;
}
#[repr(C)]
#[allow(non_snake_case)]
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
pub struct DXGI_OUTPUT_DESC1 {
    pub device_name: [u16; 32],
    pub desktop_coordinates: super::display_devices::RECT,
    pub attached_to_desktop: super::system_services::BOOL,
    pub rotation: DXGI_MODE_ROTATION,
    pub monitor: super::gdi::HMONITOR,
    pub bits_per_color: u32,
    pub color_space: DXGI_COLOR_SPACE_TYPE,
    pub red_primary: [f32; 2],
    pub green_primary: [f32; 2],
    pub blue_primary: [f32; 2],
    pub white_point: [f32; 2],
    pub min_luminance: f32,
    pub max_luminance: f32,
    pub max_full_frame_luminance: f32,
}
impl DXGI_OUTPUT_DESC1 {}
impl ::std::default::Default for DXGI_OUTPUT_DESC1 {
    fn default() -> Self {
        Self {
            device_name: [0; 32],
            desktop_coordinates: ::std::default::Default::default(),
            attached_to_desktop: ::std::default::Default::default(),
            rotation: ::std::default::Default::default(),
            monitor: ::std::default::Default::default(),
            bits_per_color: 0,
            color_space: ::std::default::Default::default(),
            red_primary: [0.0; 2],
            green_primary: [0.0; 2],
            blue_primary: [0.0; 2],
            white_point: [0.0; 2],
            min_luminance: 0.0,
            max_luminance: 0.0,
            max_full_frame_luminance: 0.0,
        }
    }
}
impl ::std::fmt::Debug for DXGI_OUTPUT_DESC1 {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("DXGI_OUTPUT_DESC1")
            .field("device_name", &format_args!("{:?}", self.device_name))
            .field(
                "desktop_coordinates",
                &format_args!("{:?}", self.desktop_coordinates),
            )
            .field(
                "attached_to_desktop",
                &format_args!("{:?}", self.attached_to_desktop),
            )
            .field("rotation", &format_args!("{:?}", self.rotation))
            .field("monitor", &format_args!("{:?}", self.monitor))
            .field("bits_per_color", &format_args!("{:?}", self.bits_per_color))
            .field("color_space", &format_args!("{:?}", self.color_space))
            .field("red_primary", &format_args!("{:?}", self.red_primary))
            .field("green_primary", &format_args!("{:?}", self.green_primary))
            .field("blue_primary", &format_args!("{:?}", self.blue_primary))
            .field("white_point", &format_args!("{:?}", self.white_point))
            .field("min_luminance", &format_args!("{:?}", self.min_luminance))
            .field("max_luminance", &format_args!("{:?}", self.max_luminance))
            .field(
                "max_full_frame_luminance",
                &format_args!("{:?}", self.max_full_frame_luminance),
            )
            .finish()
    }
}
impl ::std::cmp::PartialEq for DXGI_OUTPUT_DESC1 {
    fn eq(&self, other: &Self) -> bool {
        self.device_name == other.device_name
            && self.desktop_coordinates == other.desktop_coordinates
            && self.attached_to_desktop == other.attached_to_desktop
            && self.rotation == other.rotation
            && self.monitor == other.monitor
            && self.bits_per_color == other.bits_per_color
            && self.color_space == other.color_space
            && self.red_primary == other.red_primary
            && self.green_primary == other.green_primary
            && self.blue_primary == other.blue_primary
            && self.white_point == other.white_point
            && self.min_luminance == other.min_luminance
            && self.max_luminance == other.max_luminance
            && self.max_full_frame_luminance == other.max_full_frame_luminance
    }
}
impl ::std::cmp::Eq for DXGI_OUTPUT_DESC1 {}
unsafe impl ::windows::Abi for DXGI_OUTPUT_DESC1 {
    type Abi = Self;
}
#[allow(non_camel_case_types)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: marker :: Copy,
    :: std :: clone :: Clone,
    :: std :: default :: Default,
    :: std :: fmt :: Debug,
)]
#[repr(transparent)]
pub struct DXGI_OVERLAY_COLOR_SPACE_SUPPORT_FLAG(pub i32);
impl DXGI_OVERLAY_COLOR_SPACE_SUPPORT_FLAG {
    #![allow(non_upper_case_globals)]
    pub const DXGI_OVERLAY_COLOR_SPACE_SUPPORT_FLAG_PRESENT: Self = Self(1i32);
}
impl ::std::convert::From<i32> for DXGI_OVERLAY_COLOR_SPACE_SUPPORT_FLAG {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::Abi for DXGI_OVERLAY_COLOR_SPACE_SUPPORT_FLAG {
    type Abi = Self;
}
#[allow(non_camel_case_types)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: marker :: Copy,
    :: std :: clone :: Clone,
    :: std :: default :: Default,
    :: std :: fmt :: Debug,
)]
#[repr(transparent)]
pub struct DXGI_OVERLAY_SUPPORT_FLAG(pub i32);
impl DXGI_OVERLAY_SUPPORT_FLAG {
    #![allow(non_upper_case_globals)]
    pub const DXGI_OVERLAY_SUPPORT_FLAG_DIRECT: Self = Self(1i32);
    pub const DXGI_OVERLAY_SUPPORT_FLAG_SCALING: Self = Self(2i32);
}
impl ::std::convert::From<i32> for DXGI_OVERLAY_SUPPORT_FLAG {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::Abi for DXGI_OVERLAY_SUPPORT_FLAG {
    type Abi = Self;
}
pub const DXGI_PRESENT_ALLOW_TEARING: u32 = 512u32;
pub const DXGI_PRESENT_DO_NOT_SEQUENCE: u32 = 2u32;
pub const DXGI_PRESENT_DO_NOT_WAIT: u32 = 8u32;
#[repr(C)]
#[allow(non_snake_case)]
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
pub struct DXGI_PRESENT_PARAMETERS {
    pub dirty_rects_count: u32,
    pub p_dirty_rects: *mut super::display_devices::RECT,
    pub p_scroll_rect: *mut super::display_devices::RECT,
    pub p_scroll_offset: *mut super::display_devices::POINT,
}
impl DXGI_PRESENT_PARAMETERS {}
impl ::std::default::Default for DXGI_PRESENT_PARAMETERS {
    fn default() -> Self {
        Self {
            dirty_rects_count: 0,
            p_dirty_rects: ::std::ptr::null_mut(),
            p_scroll_rect: ::std::ptr::null_mut(),
            p_scroll_offset: ::std::ptr::null_mut(),
        }
    }
}
impl ::std::fmt::Debug for DXGI_PRESENT_PARAMETERS {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("DXGI_PRESENT_PARAMETERS")
            .field(
                "dirty_rects_count",
                &format_args!("{:?}", self.dirty_rects_count),
            )
            .field("p_dirty_rects", &format_args!("{:?}", self.p_dirty_rects))
            .field("p_scroll_rect", &format_args!("{:?}", self.p_scroll_rect))
            .field(
                "p_scroll_offset",
                &format_args!("{:?}", self.p_scroll_offset),
            )
            .finish()
    }
}
impl ::std::cmp::PartialEq for DXGI_PRESENT_PARAMETERS {
    fn eq(&self, other: &Self) -> bool {
        self.dirty_rects_count == other.dirty_rects_count
            && self.p_dirty_rects == other.p_dirty_rects
            && self.p_scroll_rect == other.p_scroll_rect
            && self.p_scroll_offset == other.p_scroll_offset
    }
}
impl ::std::cmp::Eq for DXGI_PRESENT_PARAMETERS {}
unsafe impl ::windows::Abi for DXGI_PRESENT_PARAMETERS {
    type Abi = Self;
}
pub const DXGI_PRESENT_RESTART: u32 = 4u32;
pub const DXGI_PRESENT_RESTRICT_TO_OUTPUT: u32 = 64u32;
pub const DXGI_PRESENT_STEREO_PREFER_RIGHT: u32 = 16u32;
pub const DXGI_PRESENT_STEREO_TEMPORARY_MONO: u32 = 32u32;
pub const DXGI_PRESENT_TEST: u32 = 1u32;
pub const DXGI_PRESENT_USE_DURATION: u32 = 256u32;
#[repr(C)]
#[allow(non_snake_case)]
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
pub struct DXGI_QUERY_VIDEO_MEMORY_INFO {
    pub budget: u64,
    pub current_usage: u64,
    pub available_for_reservation: u64,
    pub current_reservation: u64,
}
impl DXGI_QUERY_VIDEO_MEMORY_INFO {}
impl ::std::default::Default for DXGI_QUERY_VIDEO_MEMORY_INFO {
    fn default() -> Self {
        Self {
            budget: 0,
            current_usage: 0,
            available_for_reservation: 0,
            current_reservation: 0,
        }
    }
}
impl ::std::fmt::Debug for DXGI_QUERY_VIDEO_MEMORY_INFO {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("DXGI_QUERY_VIDEO_MEMORY_INFO")
            .field("budget", &format_args!("{:?}", self.budget))
            .field("current_usage", &format_args!("{:?}", self.current_usage))
            .field(
                "available_for_reservation",
                &format_args!("{:?}", self.available_for_reservation),
            )
            .field(
                "current_reservation",
                &format_args!("{:?}", self.current_reservation),
            )
            .finish()
    }
}
impl ::std::cmp::PartialEq for DXGI_QUERY_VIDEO_MEMORY_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.budget == other.budget
            && self.current_usage == other.current_usage
            && self.available_for_reservation == other.available_for_reservation
            && self.current_reservation == other.current_reservation
    }
}
impl ::std::cmp::Eq for DXGI_QUERY_VIDEO_MEMORY_INFO {}
unsafe impl ::windows::Abi for DXGI_QUERY_VIDEO_MEMORY_INFO {
    type Abi = Self;
}
#[allow(non_camel_case_types)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: marker :: Copy,
    :: std :: clone :: Clone,
    :: std :: default :: Default,
    :: std :: fmt :: Debug,
)]
#[repr(transparent)]
pub struct DXGI_RECLAIM_RESOURCE_RESULTS(pub i32);
impl DXGI_RECLAIM_RESOURCE_RESULTS {
    #![allow(non_upper_case_globals)]
    pub const DXGI_RECLAIM_RESOURCE_RESULT_OK: Self = Self(0i32);
    pub const DXGI_RECLAIM_RESOURCE_RESULT_DISCARDED: Self = Self(1i32);
    pub const DXGI_RECLAIM_RESOURCE_RESULT_NOT_COMMITTED: Self = Self(2i32);
}
impl ::std::convert::From<i32> for DXGI_RECLAIM_RESOURCE_RESULTS {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::Abi for DXGI_RECLAIM_RESOURCE_RESULTS {
    type Abi = Self;
}
#[allow(non_camel_case_types)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: marker :: Copy,
    :: std :: clone :: Clone,
    :: std :: default :: Default,
    :: std :: fmt :: Debug,
)]
#[repr(transparent)]
pub struct DXGI_RESIDENCY(pub i32);
impl DXGI_RESIDENCY {
    #![allow(non_upper_case_globals)]
    pub const DXGI_RESIDENCY_FULLY_RESIDENT: Self = Self(1i32);
    pub const DXGI_RESIDENCY_RESIDENT_IN_SHARED_MEMORY: Self = Self(2i32);
    pub const DXGI_RESIDENCY_EVICTED_TO_DISK: Self = Self(3i32);
}
impl ::std::convert::From<i32> for DXGI_RESIDENCY {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::Abi for DXGI_RESIDENCY {
    type Abi = Self;
}
pub const DXGI_RESOURCE_PRIORITY_HIGH: u32 = 2684354560u32;
pub const DXGI_RESOURCE_PRIORITY_LOW: u32 = 1342177280u32;
pub const DXGI_RESOURCE_PRIORITY_MAXIMUM: u32 = 3355443200u32;
pub const DXGI_RESOURCE_PRIORITY_MINIMUM: u32 = 671088640u32;
pub const DXGI_RESOURCE_PRIORITY_NORMAL: u32 = 2013265920u32;
#[repr(C)]
#[allow(non_snake_case)]
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
pub struct DXGI_RGBA {
    pub r: f32,
    pub g: f32,
    pub b: f32,
    pub a: f32,
}
impl DXGI_RGBA {}
impl ::std::default::Default for DXGI_RGBA {
    fn default() -> Self {
        Self {
            r: 0.0,
            g: 0.0,
            b: 0.0,
            a: 0.0,
        }
    }
}
impl ::std::fmt::Debug for DXGI_RGBA {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("DXGI_RGBA")
            .field("r", &format_args!("{:?}", self.r))
            .field("g", &format_args!("{:?}", self.g))
            .field("b", &format_args!("{:?}", self.b))
            .field("a", &format_args!("{:?}", self.a))
            .finish()
    }
}
impl ::std::cmp::PartialEq for DXGI_RGBA {
    fn eq(&self, other: &Self) -> bool {
        self.r == other.r && self.g == other.g && self.b == other.b && self.a == other.a
    }
}
impl ::std::cmp::Eq for DXGI_RGBA {}
unsafe impl ::windows::Abi for DXGI_RGBA {
    type Abi = Self;
}
#[allow(non_camel_case_types)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: marker :: Copy,
    :: std :: clone :: Clone,
    :: std :: default :: Default,
    :: std :: fmt :: Debug,
)]
#[repr(transparent)]
pub struct DXGI_SCALING(pub i32);
impl DXGI_SCALING {
    #![allow(non_upper_case_globals)]
    pub const DXGI_SCALING_STRETCH: Self = Self(0i32);
    pub const DXGI_SCALING_NONE: Self = Self(1i32);
    pub const DXGI_SCALING_ASPECT_RATIO_STRETCH: Self = Self(2i32);
}
impl ::std::convert::From<i32> for DXGI_SCALING {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::Abi for DXGI_SCALING {
    type Abi = Self;
}
#[repr(C)]
#[allow(non_snake_case)]
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
pub struct DXGI_SHARED_RESOURCE {
    pub handle: super::system_services::HANDLE,
}
impl DXGI_SHARED_RESOURCE {}
impl ::std::default::Default for DXGI_SHARED_RESOURCE {
    fn default() -> Self {
        Self {
            handle: ::std::default::Default::default(),
        }
    }
}
impl ::std::fmt::Debug for DXGI_SHARED_RESOURCE {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("DXGI_SHARED_RESOURCE")
            .field("handle", &format_args!("{:?}", self.handle))
            .finish()
    }
}
impl ::std::cmp::PartialEq for DXGI_SHARED_RESOURCE {
    fn eq(&self, other: &Self) -> bool {
        self.handle == other.handle
    }
}
impl ::std::cmp::Eq for DXGI_SHARED_RESOURCE {}
unsafe impl ::windows::Abi for DXGI_SHARED_RESOURCE {
    type Abi = Self;
}
pub const DXGI_SHARED_RESOURCE_READ: i32 = -2147483648i32;
pub const DXGI_SHARED_RESOURCE_WRITE: u32 = 1u32;
pub const DXGI_STANDARD_MULTISAMPLE_QUALITY_PATTERN: u32 = 4294967295u32;
#[repr(C)]
#[allow(non_snake_case)]
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
pub struct DXGI_SURFACE_DESC {
    pub width: u32,
    pub height: u32,
    pub format: DXGI_FORMAT,
    pub sample_desc: DXGI_SAMPLE_DESC,
}
impl DXGI_SURFACE_DESC {}
impl ::std::default::Default for DXGI_SURFACE_DESC {
    fn default() -> Self {
        Self {
            width: 0,
            height: 0,
            format: ::std::default::Default::default(),
            sample_desc: ::std::default::Default::default(),
        }
    }
}
impl ::std::fmt::Debug for DXGI_SURFACE_DESC {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("DXGI_SURFACE_DESC")
            .field("width", &format_args!("{:?}", self.width))
            .field("height", &format_args!("{:?}", self.height))
            .field("format", &format_args!("{:?}", self.format))
            .field("sample_desc", &format_args!("{:?}", self.sample_desc))
            .finish()
    }
}
impl ::std::cmp::PartialEq for DXGI_SURFACE_DESC {
    fn eq(&self, other: &Self) -> bool {
        self.width == other.width
            && self.height == other.height
            && self.format == other.format
            && self.sample_desc == other.sample_desc
    }
}
impl ::std::cmp::Eq for DXGI_SURFACE_DESC {}
unsafe impl ::windows::Abi for DXGI_SURFACE_DESC {
    type Abi = Self;
}
#[allow(non_camel_case_types)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: marker :: Copy,
    :: std :: clone :: Clone,
    :: std :: default :: Default,
    :: std :: fmt :: Debug,
)]
#[repr(transparent)]
pub struct DXGI_SWAP_CHAIN_COLOR_SPACE_SUPPORT_FLAG(pub i32);
impl DXGI_SWAP_CHAIN_COLOR_SPACE_SUPPORT_FLAG {
    #![allow(non_upper_case_globals)]
    pub const DXGI_SWAP_CHAIN_COLOR_SPACE_SUPPORT_FLAG_PRESENT: Self = Self(1i32);
    pub const DXGI_SWAP_CHAIN_COLOR_SPACE_SUPPORT_FLAG_OVERLAY_PRESENT: Self = Self(2i32);
}
impl ::std::convert::From<i32> for DXGI_SWAP_CHAIN_COLOR_SPACE_SUPPORT_FLAG {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::Abi for DXGI_SWAP_CHAIN_COLOR_SPACE_SUPPORT_FLAG {
    type Abi = Self;
}
#[allow(non_camel_case_types)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: marker :: Copy,
    :: std :: clone :: Clone,
    :: std :: default :: Default,
    :: std :: fmt :: Debug,
)]
#[repr(transparent)]
pub struct DXGI_SWAP_EFFECT(pub i32);
impl DXGI_SWAP_EFFECT {
    #![allow(non_upper_case_globals)]
    pub const DXGI_SWAP_EFFECT_DISCARD: Self = Self(0i32);
    pub const DXGI_SWAP_EFFECT_SEQUENTIAL: Self = Self(1i32);
    pub const DXGI_SWAP_EFFECT_FLIP_SEQUENTIAL: Self = Self(3i32);
    pub const DXGI_SWAP_EFFECT_FLIP_DISCARD: Self = Self(4i32);
}
impl ::std::convert::From<i32> for DXGI_SWAP_EFFECT {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::Abi for DXGI_SWAP_EFFECT {
    type Abi = Self;
}
#[repr(C)]
#[allow(non_snake_case)]
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
pub struct DXGI_SWAP_CHAIN_DESC {
    pub buffer_desc: DXGI_MODE_DESC,
    pub sample_desc: DXGI_SAMPLE_DESC,
    pub buffer_usage: u32,
    pub buffer_count: u32,
    pub output_window: super::windows_and_messaging::HWND,
    pub windowed: super::system_services::BOOL,
    pub swap_effect: DXGI_SWAP_EFFECT,
    pub flags: u32,
}
impl DXGI_SWAP_CHAIN_DESC {}
impl ::std::default::Default for DXGI_SWAP_CHAIN_DESC {
    fn default() -> Self {
        Self {
            buffer_desc: ::std::default::Default::default(),
            sample_desc: ::std::default::Default::default(),
            buffer_usage: 0,
            buffer_count: 0,
            output_window: ::std::default::Default::default(),
            windowed: ::std::default::Default::default(),
            swap_effect: ::std::default::Default::default(),
            flags: 0,
        }
    }
}
impl ::std::fmt::Debug for DXGI_SWAP_CHAIN_DESC {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("DXGI_SWAP_CHAIN_DESC")
            .field("buffer_desc", &format_args!("{:?}", self.buffer_desc))
            .field("sample_desc", &format_args!("{:?}", self.sample_desc))
            .field("buffer_usage", &format_args!("{:?}", self.buffer_usage))
            .field("buffer_count", &format_args!("{:?}", self.buffer_count))
            .field("output_window", &format_args!("{:?}", self.output_window))
            .field("windowed", &format_args!("{:?}", self.windowed))
            .field("swap_effect", &format_args!("{:?}", self.swap_effect))
            .field("flags", &format_args!("{:?}", self.flags))
            .finish()
    }
}
impl ::std::cmp::PartialEq for DXGI_SWAP_CHAIN_DESC {
    fn eq(&self, other: &Self) -> bool {
        self.buffer_desc == other.buffer_desc
            && self.sample_desc == other.sample_desc
            && self.buffer_usage == other.buffer_usage
            && self.buffer_count == other.buffer_count
            && self.output_window == other.output_window
            && self.windowed == other.windowed
            && self.swap_effect == other.swap_effect
            && self.flags == other.flags
    }
}
impl ::std::cmp::Eq for DXGI_SWAP_CHAIN_DESC {}
unsafe impl ::windows::Abi for DXGI_SWAP_CHAIN_DESC {
    type Abi = Self;
}
#[repr(C)]
#[allow(non_snake_case)]
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
pub struct DXGI_SWAP_CHAIN_DESC1 {
    pub width: u32,
    pub height: u32,
    pub format: DXGI_FORMAT,
    pub stereo: super::system_services::BOOL,
    pub sample_desc: DXGI_SAMPLE_DESC,
    pub buffer_usage: u32,
    pub buffer_count: u32,
    pub scaling: DXGI_SCALING,
    pub swap_effect: DXGI_SWAP_EFFECT,
    pub alpha_mode: DXGI_ALPHA_MODE,
    pub flags: u32,
}
impl DXGI_SWAP_CHAIN_DESC1 {}
impl ::std::default::Default for DXGI_SWAP_CHAIN_DESC1 {
    fn default() -> Self {
        Self {
            width: 0,
            height: 0,
            format: ::std::default::Default::default(),
            stereo: ::std::default::Default::default(),
            sample_desc: ::std::default::Default::default(),
            buffer_usage: 0,
            buffer_count: 0,
            scaling: ::std::default::Default::default(),
            swap_effect: ::std::default::Default::default(),
            alpha_mode: ::std::default::Default::default(),
            flags: 0,
        }
    }
}
impl ::std::fmt::Debug for DXGI_SWAP_CHAIN_DESC1 {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("DXGI_SWAP_CHAIN_DESC1")
            .field("width", &format_args!("{:?}", self.width))
            .field("height", &format_args!("{:?}", self.height))
            .field("format", &format_args!("{:?}", self.format))
            .field("stereo", &format_args!("{:?}", self.stereo))
            .field("sample_desc", &format_args!("{:?}", self.sample_desc))
            .field("buffer_usage", &format_args!("{:?}", self.buffer_usage))
            .field("buffer_count", &format_args!("{:?}", self.buffer_count))
            .field("scaling", &format_args!("{:?}", self.scaling))
            .field("swap_effect", &format_args!("{:?}", self.swap_effect))
            .field("alpha_mode", &format_args!("{:?}", self.alpha_mode))
            .field("flags", &format_args!("{:?}", self.flags))
            .finish()
    }
}
impl ::std::cmp::PartialEq for DXGI_SWAP_CHAIN_DESC1 {
    fn eq(&self, other: &Self) -> bool {
        self.width == other.width
            && self.height == other.height
            && self.format == other.format
            && self.stereo == other.stereo
            && self.sample_desc == other.sample_desc
            && self.buffer_usage == other.buffer_usage
            && self.buffer_count == other.buffer_count
            && self.scaling == other.scaling
            && self.swap_effect == other.swap_effect
            && self.alpha_mode == other.alpha_mode
            && self.flags == other.flags
    }
}
impl ::std::cmp::Eq for DXGI_SWAP_CHAIN_DESC1 {}
unsafe impl ::windows::Abi for DXGI_SWAP_CHAIN_DESC1 {
    type Abi = Self;
}
#[allow(non_camel_case_types)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: marker :: Copy,
    :: std :: clone :: Clone,
    :: std :: default :: Default,
    :: std :: fmt :: Debug,
)]
#[repr(transparent)]
pub struct DXGI_SWAP_CHAIN_FLAG(pub i32);
impl DXGI_SWAP_CHAIN_FLAG {
    #![allow(non_upper_case_globals)]
    pub const DXGI_SWAP_CHAIN_FLAG_NONPREROTATED: Self = Self(1i32);
    pub const DXGI_SWAP_CHAIN_FLAG_ALLOW_MODE_SWITCH: Self = Self(2i32);
    pub const DXGI_SWAP_CHAIN_FLAG_GDI_COMPATIBLE: Self = Self(4i32);
    pub const DXGI_SWAP_CHAIN_FLAG_RESTRICTED_CONTENT: Self = Self(8i32);
    pub const DXGI_SWAP_CHAIN_FLAG_RESTRICT_SHARED_RESOURCE_DRIVER: Self = Self(16i32);
    pub const DXGI_SWAP_CHAIN_FLAG_DISPLAY_ONLY: Self = Self(32i32);
    pub const DXGI_SWAP_CHAIN_FLAG_FRAME_LATENCY_WAITABLE_OBJECT: Self = Self(64i32);
    pub const DXGI_SWAP_CHAIN_FLAG_FOREGROUND_LAYER: Self = Self(128i32);
    pub const DXGI_SWAP_CHAIN_FLAG_FULLSCREEN_VIDEO: Self = Self(256i32);
    pub const DXGI_SWAP_CHAIN_FLAG_YUV_VIDEO: Self = Self(512i32);
    pub const DXGI_SWAP_CHAIN_FLAG_HW_PROTECTED: Self = Self(1024i32);
    pub const DXGI_SWAP_CHAIN_FLAG_ALLOW_TEARING: Self = Self(2048i32);
    pub const DXGI_SWAP_CHAIN_FLAG_RESTRICTED_TO_ALL_HOLOGRAPHIC_DISPLAYS: Self = Self(4096i32);
}
impl ::std::convert::From<i32> for DXGI_SWAP_CHAIN_FLAG {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::Abi for DXGI_SWAP_CHAIN_FLAG {
    type Abi = Self;
}
#[repr(C)]
#[allow(non_snake_case)]
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
pub struct DXGI_SWAP_CHAIN_FULLSCREEN_DESC {
    pub refresh_rate: DXGI_RATIONAL,
    pub scanline_ordering: DXGI_MODE_SCANLINE_ORDER,
    pub scaling: DXGI_MODE_SCALING,
    pub windowed: super::system_services::BOOL,
}
impl DXGI_SWAP_CHAIN_FULLSCREEN_DESC {}
impl ::std::default::Default for DXGI_SWAP_CHAIN_FULLSCREEN_DESC {
    fn default() -> Self {
        Self {
            refresh_rate: ::std::default::Default::default(),
            scanline_ordering: ::std::default::Default::default(),
            scaling: ::std::default::Default::default(),
            windowed: ::std::default::Default::default(),
        }
    }
}
impl ::std::fmt::Debug for DXGI_SWAP_CHAIN_FULLSCREEN_DESC {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("DXGI_SWAP_CHAIN_FULLSCREEN_DESC")
            .field("refresh_rate", &format_args!("{:?}", self.refresh_rate))
            .field(
                "scanline_ordering",
                &format_args!("{:?}", self.scanline_ordering),
            )
            .field("scaling", &format_args!("{:?}", self.scaling))
            .field("windowed", &format_args!("{:?}", self.windowed))
            .finish()
    }
}
impl ::std::cmp::PartialEq for DXGI_SWAP_CHAIN_FULLSCREEN_DESC {
    fn eq(&self, other: &Self) -> bool {
        self.refresh_rate == other.refresh_rate
            && self.scanline_ordering == other.scanline_ordering
            && self.scaling == other.scaling
            && self.windowed == other.windowed
    }
}
impl ::std::cmp::Eq for DXGI_SWAP_CHAIN_FULLSCREEN_DESC {}
unsafe impl ::windows::Abi for DXGI_SWAP_CHAIN_FULLSCREEN_DESC {
    type Abi = Self;
}
pub const DXGI_USAGE_BACK_BUFFER: u32 = 64u32;
pub const DXGI_USAGE_DISCARD_ON_PRESENT: u32 = 512u32;
pub const DXGI_USAGE_READ_ONLY: u32 = 256u32;
pub const DXGI_USAGE_RENDER_TARGET_OUTPUT: u32 = 32u32;
pub const DXGI_USAGE_SHADER_INPUT: u32 = 16u32;
pub const DXGI_USAGE_SHARED: u32 = 128u32;
pub const DXGI_USAGE_UNORDERED_ACCESS: u32 = 1024u32;
#[repr(transparent)]
#[allow(non_camel_case_types)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct IDXGIObject(::windows::IUnknown);
impl IDXGIObject {}
unsafe impl ::windows::Interface for IDXGIObject {
    type Vtable = IDXGIObject_abi;
    const IID: ::windows::Guid = ::windows::Guid::from_values(
        2931961784,
        30451,
        17977,
        [155, 224, 40, 235, 67, 166, 122, 46],
    );
}
#[allow(non_snake_case)]
impl IDXGIObject {
    pub unsafe fn SetPrivateData(
        &self,
        name: *const ::windows::Guid,
        data_size: u32,
        p_data: *const ::std::ffi::c_void,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).3)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(name),
            ::std::mem::transmute(data_size),
            ::std::mem::transmute(p_data),
        )
    }
    pub unsafe fn SetPrivateDataInterface<
        'a,
        T1__: ::windows::IntoParam<'a, ::windows::IUnknown>,
    >(
        &self,
        name: *const ::windows::Guid,
        p_unknown: T1__,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).4)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(name),
            p_unknown.into_param().abi(),
        )
    }
    pub unsafe fn GetPrivateData(
        &self,
        name: *const ::windows::Guid,
        p_data_size: *mut u32,
        p_data: *mut ::std::ffi::c_void,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).5)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(name),
            ::std::mem::transmute(p_data_size),
            ::std::mem::transmute(p_data),
        )
    }
    pub unsafe fn GetParent(
        &self,
        riid: *const ::windows::Guid,
        pp_parent: *mut *mut ::std::ffi::c_void,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).6)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(riid),
            ::std::mem::transmute(pp_parent),
        )
    }
}
impl ::std::convert::From<IDXGIObject> for ::windows::IUnknown {
    fn from(value: IDXGIObject) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IDXGIObject> for ::windows::IUnknown {
    fn from(value: &IDXGIObject) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::IntoParam<'a, ::windows::IUnknown> for IDXGIObject {
    fn into_param(self) -> ::windows::Param<'a, ::windows::IUnknown> {
        ::windows::Param::Owned(::std::convert::Into::<::windows::IUnknown>::into(self))
    }
}
impl<'a> ::windows::IntoParam<'a, ::windows::IUnknown> for &'a IDXGIObject {
    fn into_param(self) -> ::windows::Param<'a, ::windows::IUnknown> {
        ::windows::Param::Owned(::std::convert::Into::<::windows::IUnknown>::into(
            ::std::clone::Clone::clone(self),
        ))
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDXGIObject_abi(
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        iid: &::windows::Guid,
        interface: *mut ::windows::RawPtr,
    ) -> ::windows::ErrorCode,
    pub unsafe extern "system" fn(this: ::windows::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::RawPtr) -> u32,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        name: *const ::windows::Guid,
        data_size: u32,
        p_data: *const ::std::ffi::c_void,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        name: *const ::windows::Guid,
        p_unknown: ::windows::RawPtr,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        name: *const ::windows::Guid,
        p_data_size: *mut u32,
        p_data: *mut ::std::ffi::c_void,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        riid: *const ::windows::Guid,
        pp_parent: *mut *mut ::std::ffi::c_void,
    ) -> ::windows::ErrorCode,
);
#[repr(transparent)]
#[allow(non_camel_case_types)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct IDXGIAdapter(::windows::IUnknown);
impl IDXGIAdapter {}
unsafe impl ::windows::Interface for IDXGIAdapter {
    type Vtable = IDXGIAdapter_abi;
    const IID: ::windows::Guid = ::windows::Guid::from_values(
        605153249,
        4780,
        19663,
        [189, 20, 151, 152, 232, 83, 77, 192],
    );
}
#[allow(non_snake_case)]
impl IDXGIAdapter {
    pub unsafe fn SetPrivateData(
        &self,
        name: *const ::windows::Guid,
        data_size: u32,
        p_data: *const ::std::ffi::c_void,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).3)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(name),
            ::std::mem::transmute(data_size),
            ::std::mem::transmute(p_data),
        )
    }
    pub unsafe fn SetPrivateDataInterface<
        'a,
        T1__: ::windows::IntoParam<'a, ::windows::IUnknown>,
    >(
        &self,
        name: *const ::windows::Guid,
        p_unknown: T1__,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).4)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(name),
            p_unknown.into_param().abi(),
        )
    }
    pub unsafe fn GetPrivateData(
        &self,
        name: *const ::windows::Guid,
        p_data_size: *mut u32,
        p_data: *mut ::std::ffi::c_void,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).5)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(name),
            ::std::mem::transmute(p_data_size),
            ::std::mem::transmute(p_data),
        )
    }
    pub unsafe fn GetParent(
        &self,
        riid: *const ::windows::Guid,
        pp_parent: *mut *mut ::std::ffi::c_void,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).6)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(riid),
            ::std::mem::transmute(pp_parent),
        )
    }
    pub unsafe fn EnumOutputs(
        &self,
        output: u32,
        pp_output: *mut ::std::option::Option<IDXGIOutput>,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).7)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(output),
            ::std::mem::transmute(pp_output),
        )
    }
    pub unsafe fn GetDesc(&self, p_desc: *mut DXGI_ADAPTER_DESC) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).8)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(p_desc),
        )
    }
    pub unsafe fn CheckInterfaceSupport(
        &self,
        interface_name: *const ::windows::Guid,
        p_umd_version: *mut i64,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).9)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(interface_name),
            ::std::mem::transmute(p_umd_version),
        )
    }
}
impl ::std::convert::From<IDXGIAdapter> for ::windows::IUnknown {
    fn from(value: IDXGIAdapter) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IDXGIAdapter> for ::windows::IUnknown {
    fn from(value: &IDXGIAdapter) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::IntoParam<'a, ::windows::IUnknown> for IDXGIAdapter {
    fn into_param(self) -> ::windows::Param<'a, ::windows::IUnknown> {
        ::windows::Param::Owned(::std::convert::Into::<::windows::IUnknown>::into(self))
    }
}
impl<'a> ::windows::IntoParam<'a, ::windows::IUnknown> for &'a IDXGIAdapter {
    fn into_param(self) -> ::windows::Param<'a, ::windows::IUnknown> {
        ::windows::Param::Owned(::std::convert::Into::<::windows::IUnknown>::into(
            ::std::clone::Clone::clone(self),
        ))
    }
}
impl ::std::convert::From<IDXGIAdapter> for IDXGIObject {
    fn from(value: IDXGIAdapter) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IDXGIAdapter> for IDXGIObject {
    fn from(value: &IDXGIAdapter) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::IntoParam<'a, IDXGIObject> for IDXGIAdapter {
    fn into_param(self) -> ::windows::Param<'a, IDXGIObject> {
        ::windows::Param::Owned(::std::convert::Into::<IDXGIObject>::into(self))
    }
}
impl<'a> ::windows::IntoParam<'a, IDXGIObject> for &'a IDXGIAdapter {
    fn into_param(self) -> ::windows::Param<'a, IDXGIObject> {
        ::windows::Param::Owned(::std::convert::Into::<IDXGIObject>::into(
            ::std::clone::Clone::clone(self),
        ))
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDXGIAdapter_abi(
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        iid: &::windows::Guid,
        interface: *mut ::windows::RawPtr,
    ) -> ::windows::ErrorCode,
    pub unsafe extern "system" fn(this: ::windows::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::RawPtr) -> u32,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        name: *const ::windows::Guid,
        data_size: u32,
        p_data: *const ::std::ffi::c_void,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        name: *const ::windows::Guid,
        p_unknown: ::windows::RawPtr,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        name: *const ::windows::Guid,
        p_data_size: *mut u32,
        p_data: *mut ::std::ffi::c_void,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        riid: *const ::windows::Guid,
        pp_parent: *mut *mut ::std::ffi::c_void,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        output: u32,
        pp_output: *mut ::windows::RawPtr,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        p_desc: *mut DXGI_ADAPTER_DESC,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        interface_name: *const ::windows::Guid,
        p_umd_version: *mut i64,
    ) -> ::windows::ErrorCode,
);
#[repr(transparent)]
#[allow(non_camel_case_types)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct IDXGIAdapter1(::windows::IUnknown);
impl IDXGIAdapter1 {}
unsafe impl ::windows::Interface for IDXGIAdapter1 {
    type Vtable = IDXGIAdapter1_abi;
    const IID: ::windows::Guid =
        ::windows::Guid::from_values(688099169, 14393, 17958, [145, 253, 8, 104, 121, 1, 26, 5]);
}
#[allow(non_snake_case)]
impl IDXGIAdapter1 {
    pub unsafe fn SetPrivateData(
        &self,
        name: *const ::windows::Guid,
        data_size: u32,
        p_data: *const ::std::ffi::c_void,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).3)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(name),
            ::std::mem::transmute(data_size),
            ::std::mem::transmute(p_data),
        )
    }
    pub unsafe fn SetPrivateDataInterface<
        'a,
        T1__: ::windows::IntoParam<'a, ::windows::IUnknown>,
    >(
        &self,
        name: *const ::windows::Guid,
        p_unknown: T1__,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).4)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(name),
            p_unknown.into_param().abi(),
        )
    }
    pub unsafe fn GetPrivateData(
        &self,
        name: *const ::windows::Guid,
        p_data_size: *mut u32,
        p_data: *mut ::std::ffi::c_void,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).5)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(name),
            ::std::mem::transmute(p_data_size),
            ::std::mem::transmute(p_data),
        )
    }
    pub unsafe fn GetParent(
        &self,
        riid: *const ::windows::Guid,
        pp_parent: *mut *mut ::std::ffi::c_void,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).6)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(riid),
            ::std::mem::transmute(pp_parent),
        )
    }
    pub unsafe fn EnumOutputs(
        &self,
        output: u32,
        pp_output: *mut ::std::option::Option<IDXGIOutput>,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).7)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(output),
            ::std::mem::transmute(pp_output),
        )
    }
    pub unsafe fn GetDesc(&self, p_desc: *mut DXGI_ADAPTER_DESC) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).8)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(p_desc),
        )
    }
    pub unsafe fn CheckInterfaceSupport(
        &self,
        interface_name: *const ::windows::Guid,
        p_umd_version: *mut i64,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).9)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(interface_name),
            ::std::mem::transmute(p_umd_version),
        )
    }
    pub unsafe fn GetDesc1(&self, p_desc: *mut DXGI_ADAPTER_DESC1) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).10)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(p_desc),
        )
    }
}
impl ::std::convert::From<IDXGIAdapter1> for ::windows::IUnknown {
    fn from(value: IDXGIAdapter1) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IDXGIAdapter1> for ::windows::IUnknown {
    fn from(value: &IDXGIAdapter1) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::IntoParam<'a, ::windows::IUnknown> for IDXGIAdapter1 {
    fn into_param(self) -> ::windows::Param<'a, ::windows::IUnknown> {
        ::windows::Param::Owned(::std::convert::Into::<::windows::IUnknown>::into(self))
    }
}
impl<'a> ::windows::IntoParam<'a, ::windows::IUnknown> for &'a IDXGIAdapter1 {
    fn into_param(self) -> ::windows::Param<'a, ::windows::IUnknown> {
        ::windows::Param::Owned(::std::convert::Into::<::windows::IUnknown>::into(
            ::std::clone::Clone::clone(self),
        ))
    }
}
impl ::std::convert::From<IDXGIAdapter1> for IDXGIAdapter {
    fn from(value: IDXGIAdapter1) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IDXGIAdapter1> for IDXGIAdapter {
    fn from(value: &IDXGIAdapter1) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::IntoParam<'a, IDXGIAdapter> for IDXGIAdapter1 {
    fn into_param(self) -> ::windows::Param<'a, IDXGIAdapter> {
        ::windows::Param::Owned(::std::convert::Into::<IDXGIAdapter>::into(self))
    }
}
impl<'a> ::windows::IntoParam<'a, IDXGIAdapter> for &'a IDXGIAdapter1 {
    fn into_param(self) -> ::windows::Param<'a, IDXGIAdapter> {
        ::windows::Param::Owned(::std::convert::Into::<IDXGIAdapter>::into(
            ::std::clone::Clone::clone(self),
        ))
    }
}
impl ::std::convert::From<IDXGIAdapter1> for IDXGIObject {
    fn from(value: IDXGIAdapter1) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IDXGIAdapter1> for IDXGIObject {
    fn from(value: &IDXGIAdapter1) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::IntoParam<'a, IDXGIObject> for IDXGIAdapter1 {
    fn into_param(self) -> ::windows::Param<'a, IDXGIObject> {
        ::windows::Param::Owned(::std::convert::Into::<IDXGIObject>::into(self))
    }
}
impl<'a> ::windows::IntoParam<'a, IDXGIObject> for &'a IDXGIAdapter1 {
    fn into_param(self) -> ::windows::Param<'a, IDXGIObject> {
        ::windows::Param::Owned(::std::convert::Into::<IDXGIObject>::into(
            ::std::clone::Clone::clone(self),
        ))
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDXGIAdapter1_abi(
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        iid: &::windows::Guid,
        interface: *mut ::windows::RawPtr,
    ) -> ::windows::ErrorCode,
    pub unsafe extern "system" fn(this: ::windows::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::RawPtr) -> u32,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        name: *const ::windows::Guid,
        data_size: u32,
        p_data: *const ::std::ffi::c_void,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        name: *const ::windows::Guid,
        p_unknown: ::windows::RawPtr,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        name: *const ::windows::Guid,
        p_data_size: *mut u32,
        p_data: *mut ::std::ffi::c_void,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        riid: *const ::windows::Guid,
        pp_parent: *mut *mut ::std::ffi::c_void,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        output: u32,
        pp_output: *mut ::windows::RawPtr,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        p_desc: *mut DXGI_ADAPTER_DESC,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        interface_name: *const ::windows::Guid,
        p_umd_version: *mut i64,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        p_desc: *mut DXGI_ADAPTER_DESC1,
    ) -> ::windows::ErrorCode,
);
#[repr(transparent)]
#[allow(non_camel_case_types)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct IDXGIAdapter2(::windows::IUnknown);
impl IDXGIAdapter2 {}
unsafe impl ::windows::Interface for IDXGIAdapter2 {
    type Vtable = IDXGIAdapter2_abi;
    const IID: ::windows::Guid = ::windows::Guid::from_values(
        178368010,
        64014,
        19332,
        [134, 68, 224, 95, 248, 229, 172, 181],
    );
}
#[allow(non_snake_case)]
impl IDXGIAdapter2 {
    pub unsafe fn SetPrivateData(
        &self,
        name: *const ::windows::Guid,
        data_size: u32,
        p_data: *const ::std::ffi::c_void,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).3)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(name),
            ::std::mem::transmute(data_size),
            ::std::mem::transmute(p_data),
        )
    }
    pub unsafe fn SetPrivateDataInterface<
        'a,
        T1__: ::windows::IntoParam<'a, ::windows::IUnknown>,
    >(
        &self,
        name: *const ::windows::Guid,
        p_unknown: T1__,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).4)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(name),
            p_unknown.into_param().abi(),
        )
    }
    pub unsafe fn GetPrivateData(
        &self,
        name: *const ::windows::Guid,
        p_data_size: *mut u32,
        p_data: *mut ::std::ffi::c_void,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).5)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(name),
            ::std::mem::transmute(p_data_size),
            ::std::mem::transmute(p_data),
        )
    }
    pub unsafe fn GetParent(
        &self,
        riid: *const ::windows::Guid,
        pp_parent: *mut *mut ::std::ffi::c_void,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).6)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(riid),
            ::std::mem::transmute(pp_parent),
        )
    }
    pub unsafe fn EnumOutputs(
        &self,
        output: u32,
        pp_output: *mut ::std::option::Option<IDXGIOutput>,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).7)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(output),
            ::std::mem::transmute(pp_output),
        )
    }
    pub unsafe fn GetDesc(&self, p_desc: *mut DXGI_ADAPTER_DESC) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).8)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(p_desc),
        )
    }
    pub unsafe fn CheckInterfaceSupport(
        &self,
        interface_name: *const ::windows::Guid,
        p_umd_version: *mut i64,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).9)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(interface_name),
            ::std::mem::transmute(p_umd_version),
        )
    }
    pub unsafe fn GetDesc1(&self, p_desc: *mut DXGI_ADAPTER_DESC1) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).10)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(p_desc),
        )
    }
    pub unsafe fn GetDesc2(&self, p_desc: *mut DXGI_ADAPTER_DESC2) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).11)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(p_desc),
        )
    }
}
impl ::std::convert::From<IDXGIAdapter2> for ::windows::IUnknown {
    fn from(value: IDXGIAdapter2) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IDXGIAdapter2> for ::windows::IUnknown {
    fn from(value: &IDXGIAdapter2) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::IntoParam<'a, ::windows::IUnknown> for IDXGIAdapter2 {
    fn into_param(self) -> ::windows::Param<'a, ::windows::IUnknown> {
        ::windows::Param::Owned(::std::convert::Into::<::windows::IUnknown>::into(self))
    }
}
impl<'a> ::windows::IntoParam<'a, ::windows::IUnknown> for &'a IDXGIAdapter2 {
    fn into_param(self) -> ::windows::Param<'a, ::windows::IUnknown> {
        ::windows::Param::Owned(::std::convert::Into::<::windows::IUnknown>::into(
            ::std::clone::Clone::clone(self),
        ))
    }
}
impl ::std::convert::From<IDXGIAdapter2> for IDXGIAdapter1 {
    fn from(value: IDXGIAdapter2) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IDXGIAdapter2> for IDXGIAdapter1 {
    fn from(value: &IDXGIAdapter2) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::IntoParam<'a, IDXGIAdapter1> for IDXGIAdapter2 {
    fn into_param(self) -> ::windows::Param<'a, IDXGIAdapter1> {
        ::windows::Param::Owned(::std::convert::Into::<IDXGIAdapter1>::into(self))
    }
}
impl<'a> ::windows::IntoParam<'a, IDXGIAdapter1> for &'a IDXGIAdapter2 {
    fn into_param(self) -> ::windows::Param<'a, IDXGIAdapter1> {
        ::windows::Param::Owned(::std::convert::Into::<IDXGIAdapter1>::into(
            ::std::clone::Clone::clone(self),
        ))
    }
}
impl ::std::convert::From<IDXGIAdapter2> for IDXGIAdapter {
    fn from(value: IDXGIAdapter2) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IDXGIAdapter2> for IDXGIAdapter {
    fn from(value: &IDXGIAdapter2) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::IntoParam<'a, IDXGIAdapter> for IDXGIAdapter2 {
    fn into_param(self) -> ::windows::Param<'a, IDXGIAdapter> {
        ::windows::Param::Owned(::std::convert::Into::<IDXGIAdapter>::into(self))
    }
}
impl<'a> ::windows::IntoParam<'a, IDXGIAdapter> for &'a IDXGIAdapter2 {
    fn into_param(self) -> ::windows::Param<'a, IDXGIAdapter> {
        ::windows::Param::Owned(::std::convert::Into::<IDXGIAdapter>::into(
            ::std::clone::Clone::clone(self),
        ))
    }
}
impl ::std::convert::From<IDXGIAdapter2> for IDXGIObject {
    fn from(value: IDXGIAdapter2) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IDXGIAdapter2> for IDXGIObject {
    fn from(value: &IDXGIAdapter2) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::IntoParam<'a, IDXGIObject> for IDXGIAdapter2 {
    fn into_param(self) -> ::windows::Param<'a, IDXGIObject> {
        ::windows::Param::Owned(::std::convert::Into::<IDXGIObject>::into(self))
    }
}
impl<'a> ::windows::IntoParam<'a, IDXGIObject> for &'a IDXGIAdapter2 {
    fn into_param(self) -> ::windows::Param<'a, IDXGIObject> {
        ::windows::Param::Owned(::std::convert::Into::<IDXGIObject>::into(
            ::std::clone::Clone::clone(self),
        ))
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDXGIAdapter2_abi(
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        iid: &::windows::Guid,
        interface: *mut ::windows::RawPtr,
    ) -> ::windows::ErrorCode,
    pub unsafe extern "system" fn(this: ::windows::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::RawPtr) -> u32,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        name: *const ::windows::Guid,
        data_size: u32,
        p_data: *const ::std::ffi::c_void,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        name: *const ::windows::Guid,
        p_unknown: ::windows::RawPtr,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        name: *const ::windows::Guid,
        p_data_size: *mut u32,
        p_data: *mut ::std::ffi::c_void,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        riid: *const ::windows::Guid,
        pp_parent: *mut *mut ::std::ffi::c_void,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        output: u32,
        pp_output: *mut ::windows::RawPtr,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        p_desc: *mut DXGI_ADAPTER_DESC,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        interface_name: *const ::windows::Guid,
        p_umd_version: *mut i64,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        p_desc: *mut DXGI_ADAPTER_DESC1,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        p_desc: *mut DXGI_ADAPTER_DESC2,
    ) -> ::windows::ErrorCode,
);
#[repr(transparent)]
#[allow(non_camel_case_types)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct IDXGIAdapter3(::windows::IUnknown);
impl IDXGIAdapter3 {}
unsafe impl ::windows::Interface for IDXGIAdapter3 {
    type Vtable = IDXGIAdapter3_abi;
    const IID: ::windows::Guid = ::windows::Guid::from_values(
        1683580836,
        5010,
        17168,
        [167, 152, 128, 83, 206, 62, 147, 253],
    );
}
#[allow(non_snake_case)]
impl IDXGIAdapter3 {
    pub unsafe fn SetPrivateData(
        &self,
        name: *const ::windows::Guid,
        data_size: u32,
        p_data: *const ::std::ffi::c_void,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).3)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(name),
            ::std::mem::transmute(data_size),
            ::std::mem::transmute(p_data),
        )
    }
    pub unsafe fn SetPrivateDataInterface<
        'a,
        T1__: ::windows::IntoParam<'a, ::windows::IUnknown>,
    >(
        &self,
        name: *const ::windows::Guid,
        p_unknown: T1__,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).4)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(name),
            p_unknown.into_param().abi(),
        )
    }
    pub unsafe fn GetPrivateData(
        &self,
        name: *const ::windows::Guid,
        p_data_size: *mut u32,
        p_data: *mut ::std::ffi::c_void,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).5)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(name),
            ::std::mem::transmute(p_data_size),
            ::std::mem::transmute(p_data),
        )
    }
    pub unsafe fn GetParent(
        &self,
        riid: *const ::windows::Guid,
        pp_parent: *mut *mut ::std::ffi::c_void,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).6)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(riid),
            ::std::mem::transmute(pp_parent),
        )
    }
    pub unsafe fn EnumOutputs(
        &self,
        output: u32,
        pp_output: *mut ::std::option::Option<IDXGIOutput>,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).7)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(output),
            ::std::mem::transmute(pp_output),
        )
    }
    pub unsafe fn GetDesc(&self, p_desc: *mut DXGI_ADAPTER_DESC) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).8)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(p_desc),
        )
    }
    pub unsafe fn CheckInterfaceSupport(
        &self,
        interface_name: *const ::windows::Guid,
        p_umd_version: *mut i64,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).9)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(interface_name),
            ::std::mem::transmute(p_umd_version),
        )
    }
    pub unsafe fn GetDesc1(&self, p_desc: *mut DXGI_ADAPTER_DESC1) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).10)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(p_desc),
        )
    }
    pub unsafe fn GetDesc2(&self, p_desc: *mut DXGI_ADAPTER_DESC2) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).11)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(p_desc),
        )
    }
    pub unsafe fn RegisterHardwareContentProtectionTeardownStatusEvent<
        'a,
        T0__: ::windows::IntoParam<'a, super::system_services::HANDLE>,
    >(
        &self,
        h_event: T0__,
        pdw_cookie: *mut u32,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).12)(
            ::windows::Abi::abi(self),
            h_event.into_param().abi(),
            ::std::mem::transmute(pdw_cookie),
        )
    }
    pub unsafe fn UnregisterHardwareContentProtectionTeardownStatus(&self, dw_cookie: u32) {
        (::windows::Interface::vtable(self).13)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(dw_cookie),
        )
    }
    pub unsafe fn QueryVideoMemoryInfo(
        &self,
        node_index: u32,
        memory_segment_group: DXGI_MEMORY_SEGMENT_GROUP,
        p_video_memory_info: *mut DXGI_QUERY_VIDEO_MEMORY_INFO,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).14)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(node_index),
            ::std::mem::transmute(memory_segment_group),
            ::std::mem::transmute(p_video_memory_info),
        )
    }
    pub unsafe fn SetVideoMemoryReservation(
        &self,
        node_index: u32,
        memory_segment_group: DXGI_MEMORY_SEGMENT_GROUP,
        reservation: u64,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).15)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(node_index),
            ::std::mem::transmute(memory_segment_group),
            ::std::mem::transmute(reservation),
        )
    }
    pub unsafe fn RegisterVideoMemoryBudgetChangeNotificationEvent<
        'a,
        T0__: ::windows::IntoParam<'a, super::system_services::HANDLE>,
    >(
        &self,
        h_event: T0__,
        pdw_cookie: *mut u32,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).16)(
            ::windows::Abi::abi(self),
            h_event.into_param().abi(),
            ::std::mem::transmute(pdw_cookie),
        )
    }
    pub unsafe fn UnregisterVideoMemoryBudgetChangeNotification(&self, dw_cookie: u32) {
        (::windows::Interface::vtable(self).17)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(dw_cookie),
        )
    }
}
impl ::std::convert::From<IDXGIAdapter3> for ::windows::IUnknown {
    fn from(value: IDXGIAdapter3) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IDXGIAdapter3> for ::windows::IUnknown {
    fn from(value: &IDXGIAdapter3) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::IntoParam<'a, ::windows::IUnknown> for IDXGIAdapter3 {
    fn into_param(self) -> ::windows::Param<'a, ::windows::IUnknown> {
        ::windows::Param::Owned(::std::convert::Into::<::windows::IUnknown>::into(self))
    }
}
impl<'a> ::windows::IntoParam<'a, ::windows::IUnknown> for &'a IDXGIAdapter3 {
    fn into_param(self) -> ::windows::Param<'a, ::windows::IUnknown> {
        ::windows::Param::Owned(::std::convert::Into::<::windows::IUnknown>::into(
            ::std::clone::Clone::clone(self),
        ))
    }
}
impl ::std::convert::From<IDXGIAdapter3> for IDXGIAdapter2 {
    fn from(value: IDXGIAdapter3) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IDXGIAdapter3> for IDXGIAdapter2 {
    fn from(value: &IDXGIAdapter3) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::IntoParam<'a, IDXGIAdapter2> for IDXGIAdapter3 {
    fn into_param(self) -> ::windows::Param<'a, IDXGIAdapter2> {
        ::windows::Param::Owned(::std::convert::Into::<IDXGIAdapter2>::into(self))
    }
}
impl<'a> ::windows::IntoParam<'a, IDXGIAdapter2> for &'a IDXGIAdapter3 {
    fn into_param(self) -> ::windows::Param<'a, IDXGIAdapter2> {
        ::windows::Param::Owned(::std::convert::Into::<IDXGIAdapter2>::into(
            ::std::clone::Clone::clone(self),
        ))
    }
}
impl ::std::convert::From<IDXGIAdapter3> for IDXGIAdapter1 {
    fn from(value: IDXGIAdapter3) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IDXGIAdapter3> for IDXGIAdapter1 {
    fn from(value: &IDXGIAdapter3) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::IntoParam<'a, IDXGIAdapter1> for IDXGIAdapter3 {
    fn into_param(self) -> ::windows::Param<'a, IDXGIAdapter1> {
        ::windows::Param::Owned(::std::convert::Into::<IDXGIAdapter1>::into(self))
    }
}
impl<'a> ::windows::IntoParam<'a, IDXGIAdapter1> for &'a IDXGIAdapter3 {
    fn into_param(self) -> ::windows::Param<'a, IDXGIAdapter1> {
        ::windows::Param::Owned(::std::convert::Into::<IDXGIAdapter1>::into(
            ::std::clone::Clone::clone(self),
        ))
    }
}
impl ::std::convert::From<IDXGIAdapter3> for IDXGIAdapter {
    fn from(value: IDXGIAdapter3) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IDXGIAdapter3> for IDXGIAdapter {
    fn from(value: &IDXGIAdapter3) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::IntoParam<'a, IDXGIAdapter> for IDXGIAdapter3 {
    fn into_param(self) -> ::windows::Param<'a, IDXGIAdapter> {
        ::windows::Param::Owned(::std::convert::Into::<IDXGIAdapter>::into(self))
    }
}
impl<'a> ::windows::IntoParam<'a, IDXGIAdapter> for &'a IDXGIAdapter3 {
    fn into_param(self) -> ::windows::Param<'a, IDXGIAdapter> {
        ::windows::Param::Owned(::std::convert::Into::<IDXGIAdapter>::into(
            ::std::clone::Clone::clone(self),
        ))
    }
}
impl ::std::convert::From<IDXGIAdapter3> for IDXGIObject {
    fn from(value: IDXGIAdapter3) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IDXGIAdapter3> for IDXGIObject {
    fn from(value: &IDXGIAdapter3) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::IntoParam<'a, IDXGIObject> for IDXGIAdapter3 {
    fn into_param(self) -> ::windows::Param<'a, IDXGIObject> {
        ::windows::Param::Owned(::std::convert::Into::<IDXGIObject>::into(self))
    }
}
impl<'a> ::windows::IntoParam<'a, IDXGIObject> for &'a IDXGIAdapter3 {
    fn into_param(self) -> ::windows::Param<'a, IDXGIObject> {
        ::windows::Param::Owned(::std::convert::Into::<IDXGIObject>::into(
            ::std::clone::Clone::clone(self),
        ))
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDXGIAdapter3_abi(
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        iid: &::windows::Guid,
        interface: *mut ::windows::RawPtr,
    ) -> ::windows::ErrorCode,
    pub unsafe extern "system" fn(this: ::windows::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::RawPtr) -> u32,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        name: *const ::windows::Guid,
        data_size: u32,
        p_data: *const ::std::ffi::c_void,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        name: *const ::windows::Guid,
        p_unknown: ::windows::RawPtr,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        name: *const ::windows::Guid,
        p_data_size: *mut u32,
        p_data: *mut ::std::ffi::c_void,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        riid: *const ::windows::Guid,
        pp_parent: *mut *mut ::std::ffi::c_void,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        output: u32,
        pp_output: *mut ::windows::RawPtr,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        p_desc: *mut DXGI_ADAPTER_DESC,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        interface_name: *const ::windows::Guid,
        p_umd_version: *mut i64,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        p_desc: *mut DXGI_ADAPTER_DESC1,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        p_desc: *mut DXGI_ADAPTER_DESC2,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        h_event: super::system_services::HANDLE,
        pdw_cookie: *mut u32,
    ) -> ::windows::ErrorCode,
    pub unsafe extern "system" fn(this: ::windows::RawPtr, dw_cookie: u32),
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        node_index: u32,
        memory_segment_group: DXGI_MEMORY_SEGMENT_GROUP,
        p_video_memory_info: *mut DXGI_QUERY_VIDEO_MEMORY_INFO,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        node_index: u32,
        memory_segment_group: DXGI_MEMORY_SEGMENT_GROUP,
        reservation: u64,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        h_event: super::system_services::HANDLE,
        pdw_cookie: *mut u32,
    ) -> ::windows::ErrorCode,
    pub unsafe extern "system" fn(this: ::windows::RawPtr, dw_cookie: u32),
);
#[repr(transparent)]
#[allow(non_camel_case_types)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct IDXGIAdapter4(::windows::IUnknown);
impl IDXGIAdapter4 {}
unsafe impl ::windows::Interface for IDXGIAdapter4 {
    type Vtable = IDXGIAdapter4_abi;
    const IID: ::windows::Guid = ::windows::Guid::from_values(
        1015912913,
        20415,
        16769,
        [168, 44, 175, 102, 191, 123, 210, 78],
    );
}
#[allow(non_snake_case)]
impl IDXGIAdapter4 {
    pub unsafe fn SetPrivateData(
        &self,
        name: *const ::windows::Guid,
        data_size: u32,
        p_data: *const ::std::ffi::c_void,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).3)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(name),
            ::std::mem::transmute(data_size),
            ::std::mem::transmute(p_data),
        )
    }
    pub unsafe fn SetPrivateDataInterface<
        'a,
        T1__: ::windows::IntoParam<'a, ::windows::IUnknown>,
    >(
        &self,
        name: *const ::windows::Guid,
        p_unknown: T1__,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).4)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(name),
            p_unknown.into_param().abi(),
        )
    }
    pub unsafe fn GetPrivateData(
        &self,
        name: *const ::windows::Guid,
        p_data_size: *mut u32,
        p_data: *mut ::std::ffi::c_void,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).5)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(name),
            ::std::mem::transmute(p_data_size),
            ::std::mem::transmute(p_data),
        )
    }
    pub unsafe fn GetParent(
        &self,
        riid: *const ::windows::Guid,
        pp_parent: *mut *mut ::std::ffi::c_void,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).6)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(riid),
            ::std::mem::transmute(pp_parent),
        )
    }
    pub unsafe fn EnumOutputs(
        &self,
        output: u32,
        pp_output: *mut ::std::option::Option<IDXGIOutput>,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).7)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(output),
            ::std::mem::transmute(pp_output),
        )
    }
    pub unsafe fn GetDesc(&self, p_desc: *mut DXGI_ADAPTER_DESC) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).8)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(p_desc),
        )
    }
    pub unsafe fn CheckInterfaceSupport(
        &self,
        interface_name: *const ::windows::Guid,
        p_umd_version: *mut i64,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).9)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(interface_name),
            ::std::mem::transmute(p_umd_version),
        )
    }
    pub unsafe fn GetDesc1(&self, p_desc: *mut DXGI_ADAPTER_DESC1) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).10)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(p_desc),
        )
    }
    pub unsafe fn GetDesc2(&self, p_desc: *mut DXGI_ADAPTER_DESC2) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).11)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(p_desc),
        )
    }
    pub unsafe fn RegisterHardwareContentProtectionTeardownStatusEvent<
        'a,
        T0__: ::windows::IntoParam<'a, super::system_services::HANDLE>,
    >(
        &self,
        h_event: T0__,
        pdw_cookie: *mut u32,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).12)(
            ::windows::Abi::abi(self),
            h_event.into_param().abi(),
            ::std::mem::transmute(pdw_cookie),
        )
    }
    pub unsafe fn UnregisterHardwareContentProtectionTeardownStatus(&self, dw_cookie: u32) {
        (::windows::Interface::vtable(self).13)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(dw_cookie),
        )
    }
    pub unsafe fn QueryVideoMemoryInfo(
        &self,
        node_index: u32,
        memory_segment_group: DXGI_MEMORY_SEGMENT_GROUP,
        p_video_memory_info: *mut DXGI_QUERY_VIDEO_MEMORY_INFO,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).14)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(node_index),
            ::std::mem::transmute(memory_segment_group),
            ::std::mem::transmute(p_video_memory_info),
        )
    }
    pub unsafe fn SetVideoMemoryReservation(
        &self,
        node_index: u32,
        memory_segment_group: DXGI_MEMORY_SEGMENT_GROUP,
        reservation: u64,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).15)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(node_index),
            ::std::mem::transmute(memory_segment_group),
            ::std::mem::transmute(reservation),
        )
    }
    pub unsafe fn RegisterVideoMemoryBudgetChangeNotificationEvent<
        'a,
        T0__: ::windows::IntoParam<'a, super::system_services::HANDLE>,
    >(
        &self,
        h_event: T0__,
        pdw_cookie: *mut u32,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).16)(
            ::windows::Abi::abi(self),
            h_event.into_param().abi(),
            ::std::mem::transmute(pdw_cookie),
        )
    }
    pub unsafe fn UnregisterVideoMemoryBudgetChangeNotification(&self, dw_cookie: u32) {
        (::windows::Interface::vtable(self).17)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(dw_cookie),
        )
    }
    pub unsafe fn GetDesc3(&self, p_desc: *mut DXGI_ADAPTER_DESC3) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).18)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(p_desc),
        )
    }
}
impl ::std::convert::From<IDXGIAdapter4> for ::windows::IUnknown {
    fn from(value: IDXGIAdapter4) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IDXGIAdapter4> for ::windows::IUnknown {
    fn from(value: &IDXGIAdapter4) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::IntoParam<'a, ::windows::IUnknown> for IDXGIAdapter4 {
    fn into_param(self) -> ::windows::Param<'a, ::windows::IUnknown> {
        ::windows::Param::Owned(::std::convert::Into::<::windows::IUnknown>::into(self))
    }
}
impl<'a> ::windows::IntoParam<'a, ::windows::IUnknown> for &'a IDXGIAdapter4 {
    fn into_param(self) -> ::windows::Param<'a, ::windows::IUnknown> {
        ::windows::Param::Owned(::std::convert::Into::<::windows::IUnknown>::into(
            ::std::clone::Clone::clone(self),
        ))
    }
}
impl ::std::convert::From<IDXGIAdapter4> for IDXGIAdapter3 {
    fn from(value: IDXGIAdapter4) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IDXGIAdapter4> for IDXGIAdapter3 {
    fn from(value: &IDXGIAdapter4) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::IntoParam<'a, IDXGIAdapter3> for IDXGIAdapter4 {
    fn into_param(self) -> ::windows::Param<'a, IDXGIAdapter3> {
        ::windows::Param::Owned(::std::convert::Into::<IDXGIAdapter3>::into(self))
    }
}
impl<'a> ::windows::IntoParam<'a, IDXGIAdapter3> for &'a IDXGIAdapter4 {
    fn into_param(self) -> ::windows::Param<'a, IDXGIAdapter3> {
        ::windows::Param::Owned(::std::convert::Into::<IDXGIAdapter3>::into(
            ::std::clone::Clone::clone(self),
        ))
    }
}
impl ::std::convert::From<IDXGIAdapter4> for IDXGIAdapter2 {
    fn from(value: IDXGIAdapter4) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IDXGIAdapter4> for IDXGIAdapter2 {
    fn from(value: &IDXGIAdapter4) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::IntoParam<'a, IDXGIAdapter2> for IDXGIAdapter4 {
    fn into_param(self) -> ::windows::Param<'a, IDXGIAdapter2> {
        ::windows::Param::Owned(::std::convert::Into::<IDXGIAdapter2>::into(self))
    }
}
impl<'a> ::windows::IntoParam<'a, IDXGIAdapter2> for &'a IDXGIAdapter4 {
    fn into_param(self) -> ::windows::Param<'a, IDXGIAdapter2> {
        ::windows::Param::Owned(::std::convert::Into::<IDXGIAdapter2>::into(
            ::std::clone::Clone::clone(self),
        ))
    }
}
impl ::std::convert::From<IDXGIAdapter4> for IDXGIAdapter1 {
    fn from(value: IDXGIAdapter4) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IDXGIAdapter4> for IDXGIAdapter1 {
    fn from(value: &IDXGIAdapter4) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::IntoParam<'a, IDXGIAdapter1> for IDXGIAdapter4 {
    fn into_param(self) -> ::windows::Param<'a, IDXGIAdapter1> {
        ::windows::Param::Owned(::std::convert::Into::<IDXGIAdapter1>::into(self))
    }
}
impl<'a> ::windows::IntoParam<'a, IDXGIAdapter1> for &'a IDXGIAdapter4 {
    fn into_param(self) -> ::windows::Param<'a, IDXGIAdapter1> {
        ::windows::Param::Owned(::std::convert::Into::<IDXGIAdapter1>::into(
            ::std::clone::Clone::clone(self),
        ))
    }
}
impl ::std::convert::From<IDXGIAdapter4> for IDXGIAdapter {
    fn from(value: IDXGIAdapter4) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IDXGIAdapter4> for IDXGIAdapter {
    fn from(value: &IDXGIAdapter4) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::IntoParam<'a, IDXGIAdapter> for IDXGIAdapter4 {
    fn into_param(self) -> ::windows::Param<'a, IDXGIAdapter> {
        ::windows::Param::Owned(::std::convert::Into::<IDXGIAdapter>::into(self))
    }
}
impl<'a> ::windows::IntoParam<'a, IDXGIAdapter> for &'a IDXGIAdapter4 {
    fn into_param(self) -> ::windows::Param<'a, IDXGIAdapter> {
        ::windows::Param::Owned(::std::convert::Into::<IDXGIAdapter>::into(
            ::std::clone::Clone::clone(self),
        ))
    }
}
impl ::std::convert::From<IDXGIAdapter4> for IDXGIObject {
    fn from(value: IDXGIAdapter4) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IDXGIAdapter4> for IDXGIObject {
    fn from(value: &IDXGIAdapter4) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::IntoParam<'a, IDXGIObject> for IDXGIAdapter4 {
    fn into_param(self) -> ::windows::Param<'a, IDXGIObject> {
        ::windows::Param::Owned(::std::convert::Into::<IDXGIObject>::into(self))
    }
}
impl<'a> ::windows::IntoParam<'a, IDXGIObject> for &'a IDXGIAdapter4 {
    fn into_param(self) -> ::windows::Param<'a, IDXGIObject> {
        ::windows::Param::Owned(::std::convert::Into::<IDXGIObject>::into(
            ::std::clone::Clone::clone(self),
        ))
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDXGIAdapter4_abi(
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        iid: &::windows::Guid,
        interface: *mut ::windows::RawPtr,
    ) -> ::windows::ErrorCode,
    pub unsafe extern "system" fn(this: ::windows::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::RawPtr) -> u32,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        name: *const ::windows::Guid,
        data_size: u32,
        p_data: *const ::std::ffi::c_void,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        name: *const ::windows::Guid,
        p_unknown: ::windows::RawPtr,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        name: *const ::windows::Guid,
        p_data_size: *mut u32,
        p_data: *mut ::std::ffi::c_void,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        riid: *const ::windows::Guid,
        pp_parent: *mut *mut ::std::ffi::c_void,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        output: u32,
        pp_output: *mut ::windows::RawPtr,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        p_desc: *mut DXGI_ADAPTER_DESC,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        interface_name: *const ::windows::Guid,
        p_umd_version: *mut i64,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        p_desc: *mut DXGI_ADAPTER_DESC1,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        p_desc: *mut DXGI_ADAPTER_DESC2,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        h_event: super::system_services::HANDLE,
        pdw_cookie: *mut u32,
    ) -> ::windows::ErrorCode,
    pub unsafe extern "system" fn(this: ::windows::RawPtr, dw_cookie: u32),
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        node_index: u32,
        memory_segment_group: DXGI_MEMORY_SEGMENT_GROUP,
        p_video_memory_info: *mut DXGI_QUERY_VIDEO_MEMORY_INFO,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        node_index: u32,
        memory_segment_group: DXGI_MEMORY_SEGMENT_GROUP,
        reservation: u64,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        h_event: super::system_services::HANDLE,
        pdw_cookie: *mut u32,
    ) -> ::windows::ErrorCode,
    pub unsafe extern "system" fn(this: ::windows::RawPtr, dw_cookie: u32),
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        p_desc: *mut DXGI_ADAPTER_DESC3,
    ) -> ::windows::ErrorCode,
);
#[repr(transparent)]
#[allow(non_camel_case_types)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct IDXGIDebug(::windows::IUnknown);
impl IDXGIDebug {}
unsafe impl ::windows::Interface for IDXGIDebug {
    type Vtable = IDXGIDebug_abi;
    const IID: ::windows::Guid =
        ::windows::Guid::from_values(295597138, 56990, 16638, [136, 6, 136, 249, 12, 18, 180, 65]);
}
#[allow(non_snake_case)]
impl IDXGIDebug {
    pub unsafe fn ReportLiveObjects<'a, T0__: ::windows::IntoParam<'a, ::windows::Guid>>(
        &self,
        apiid: T0__,
        flags: DXGI_DEBUG_RLO_FLAGS,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).3)(
            ::windows::Abi::abi(self),
            apiid.into_param().abi(),
            ::std::mem::transmute(flags),
        )
    }
}
impl ::std::convert::From<IDXGIDebug> for ::windows::IUnknown {
    fn from(value: IDXGIDebug) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IDXGIDebug> for ::windows::IUnknown {
    fn from(value: &IDXGIDebug) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::IntoParam<'a, ::windows::IUnknown> for IDXGIDebug {
    fn into_param(self) -> ::windows::Param<'a, ::windows::IUnknown> {
        ::windows::Param::Owned(::std::convert::Into::<::windows::IUnknown>::into(self))
    }
}
impl<'a> ::windows::IntoParam<'a, ::windows::IUnknown> for &'a IDXGIDebug {
    fn into_param(self) -> ::windows::Param<'a, ::windows::IUnknown> {
        ::windows::Param::Owned(::std::convert::Into::<::windows::IUnknown>::into(
            ::std::clone::Clone::clone(self),
        ))
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDXGIDebug_abi(
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        iid: &::windows::Guid,
        interface: *mut ::windows::RawPtr,
    ) -> ::windows::ErrorCode,
    pub unsafe extern "system" fn(this: ::windows::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::RawPtr) -> u32,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        apiid: ::windows::Guid,
        flags: DXGI_DEBUG_RLO_FLAGS,
    ) -> ::windows::ErrorCode,
);
#[repr(transparent)]
#[allow(non_camel_case_types)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct IDXGIDebug1(::windows::IUnknown);
impl IDXGIDebug1 {}
unsafe impl ::windows::Interface for IDXGIDebug1 {
    type Vtable = IDXGIDebug1_abi;
    const IID: ::windows::Guid = ::windows::Guid::from_values(
        3315621644,
        5874,
        19167,
        [159, 77, 168, 196, 213, 138, 197, 80],
    );
}
#[allow(non_snake_case)]
impl IDXGIDebug1 {
    pub unsafe fn ReportLiveObjects<'a, T0__: ::windows::IntoParam<'a, ::windows::Guid>>(
        &self,
        apiid: T0__,
        flags: DXGI_DEBUG_RLO_FLAGS,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).3)(
            ::windows::Abi::abi(self),
            apiid.into_param().abi(),
            ::std::mem::transmute(flags),
        )
    }
    pub unsafe fn EnableLeakTrackingForThread(&self) {
        (::windows::Interface::vtable(self).4)(::windows::Abi::abi(self))
    }
    pub unsafe fn DisableLeakTrackingForThread(&self) {
        (::windows::Interface::vtable(self).5)(::windows::Abi::abi(self))
    }
    pub unsafe fn IsLeakTrackingEnabledForThread(&self) -> super::system_services::BOOL {
        (::windows::Interface::vtable(self).6)(::windows::Abi::abi(self))
    }
}
impl ::std::convert::From<IDXGIDebug1> for ::windows::IUnknown {
    fn from(value: IDXGIDebug1) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IDXGIDebug1> for ::windows::IUnknown {
    fn from(value: &IDXGIDebug1) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::IntoParam<'a, ::windows::IUnknown> for IDXGIDebug1 {
    fn into_param(self) -> ::windows::Param<'a, ::windows::IUnknown> {
        ::windows::Param::Owned(::std::convert::Into::<::windows::IUnknown>::into(self))
    }
}
impl<'a> ::windows::IntoParam<'a, ::windows::IUnknown> for &'a IDXGIDebug1 {
    fn into_param(self) -> ::windows::Param<'a, ::windows::IUnknown> {
        ::windows::Param::Owned(::std::convert::Into::<::windows::IUnknown>::into(
            ::std::clone::Clone::clone(self),
        ))
    }
}
impl ::std::convert::From<IDXGIDebug1> for IDXGIDebug {
    fn from(value: IDXGIDebug1) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IDXGIDebug1> for IDXGIDebug {
    fn from(value: &IDXGIDebug1) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::IntoParam<'a, IDXGIDebug> for IDXGIDebug1 {
    fn into_param(self) -> ::windows::Param<'a, IDXGIDebug> {
        ::windows::Param::Owned(::std::convert::Into::<IDXGIDebug>::into(self))
    }
}
impl<'a> ::windows::IntoParam<'a, IDXGIDebug> for &'a IDXGIDebug1 {
    fn into_param(self) -> ::windows::Param<'a, IDXGIDebug> {
        ::windows::Param::Owned(::std::convert::Into::<IDXGIDebug>::into(
            ::std::clone::Clone::clone(self),
        ))
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDXGIDebug1_abi(
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        iid: &::windows::Guid,
        interface: *mut ::windows::RawPtr,
    ) -> ::windows::ErrorCode,
    pub unsafe extern "system" fn(this: ::windows::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::RawPtr) -> u32,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        apiid: ::windows::Guid,
        flags: DXGI_DEBUG_RLO_FLAGS,
    ) -> ::windows::ErrorCode,
    pub unsafe extern "system" fn(this: ::windows::RawPtr),
    pub unsafe extern "system" fn(this: ::windows::RawPtr),
    pub unsafe extern "system" fn(this: ::windows::RawPtr) -> super::system_services::BOOL,
);
#[repr(transparent)]
#[allow(non_camel_case_types)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct IDXGIDecodeSwapChain(::windows::IUnknown);
impl IDXGIDecodeSwapChain {}
unsafe impl ::windows::Interface for IDXGIDecodeSwapChain {
    type Vtable = IDXGIDecodeSwapChain_abi;
    const IID: ::windows::Guid = ::windows::Guid::from_values(
        640878187,
        17684,
        19578,
        [143, 216, 18, 234, 152, 5, 157, 24],
    );
}
#[allow(non_snake_case)]
impl IDXGIDecodeSwapChain {
    pub unsafe fn PresentBuffer(
        &self,
        buffer_to_present: u32,
        sync_interval: u32,
        flags: u32,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).3)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(buffer_to_present),
            ::std::mem::transmute(sync_interval),
            ::std::mem::transmute(flags),
        )
    }
    pub unsafe fn SetSourceRect(
        &self,
        p_rect: *const super::display_devices::RECT,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).4)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(p_rect),
        )
    }
    pub unsafe fn SetTargetRect(
        &self,
        p_rect: *const super::display_devices::RECT,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).5)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(p_rect),
        )
    }
    pub unsafe fn SetDestSize(&self, width: u32, height: u32) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).6)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(width),
            ::std::mem::transmute(height),
        )
    }
    pub unsafe fn GetSourceRect(
        &self,
        p_rect: *mut super::display_devices::RECT,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).7)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(p_rect),
        )
    }
    pub unsafe fn GetTargetRect(
        &self,
        p_rect: *mut super::display_devices::RECT,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).8)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(p_rect),
        )
    }
    pub unsafe fn GetDestSize(
        &self,
        p_width: *mut u32,
        p_height: *mut u32,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).9)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(p_width),
            ::std::mem::transmute(p_height),
        )
    }
    pub unsafe fn SetColorSpace(
        &self,
        color_space: DXGI_MULTIPLANE_OVERLAY_YCbCr_FLAGS,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).10)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(color_space),
        )
    }
    pub unsafe fn GetColorSpace(&self) -> DXGI_MULTIPLANE_OVERLAY_YCbCr_FLAGS {
        (::windows::Interface::vtable(self).11)(::windows::Abi::abi(self))
    }
}
impl ::std::convert::From<IDXGIDecodeSwapChain> for ::windows::IUnknown {
    fn from(value: IDXGIDecodeSwapChain) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IDXGIDecodeSwapChain> for ::windows::IUnknown {
    fn from(value: &IDXGIDecodeSwapChain) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::IntoParam<'a, ::windows::IUnknown> for IDXGIDecodeSwapChain {
    fn into_param(self) -> ::windows::Param<'a, ::windows::IUnknown> {
        ::windows::Param::Owned(::std::convert::Into::<::windows::IUnknown>::into(self))
    }
}
impl<'a> ::windows::IntoParam<'a, ::windows::IUnknown> for &'a IDXGIDecodeSwapChain {
    fn into_param(self) -> ::windows::Param<'a, ::windows::IUnknown> {
        ::windows::Param::Owned(::std::convert::Into::<::windows::IUnknown>::into(
            ::std::clone::Clone::clone(self),
        ))
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDXGIDecodeSwapChain_abi(
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        iid: &::windows::Guid,
        interface: *mut ::windows::RawPtr,
    ) -> ::windows::ErrorCode,
    pub unsafe extern "system" fn(this: ::windows::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::RawPtr) -> u32,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        buffer_to_present: u32,
        sync_interval: u32,
        flags: u32,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        p_rect: *const super::display_devices::RECT,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        p_rect: *const super::display_devices::RECT,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        width: u32,
        height: u32,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        p_rect: *mut super::display_devices::RECT,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        p_rect: *mut super::display_devices::RECT,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        p_width: *mut u32,
        p_height: *mut u32,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        color_space: DXGI_MULTIPLANE_OVERLAY_YCbCr_FLAGS,
    ) -> ::windows::ErrorCode,
    pub unsafe extern "system" fn(this: ::windows::RawPtr) -> DXGI_MULTIPLANE_OVERLAY_YCbCr_FLAGS,
);
#[repr(transparent)]
#[allow(non_camel_case_types)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct IDXGIDevice(::windows::IUnknown);
impl IDXGIDevice {}
unsafe impl ::windows::Interface for IDXGIDevice {
    type Vtable = IDXGIDevice_abi;
    const IID: ::windows::Guid = ::windows::Guid::from_values(
        1424783354,
        4983,
        17638,
        [140, 50, 136, 253, 95, 68, 200, 76],
    );
}
#[allow(non_snake_case)]
impl IDXGIDevice {
    pub unsafe fn SetPrivateData(
        &self,
        name: *const ::windows::Guid,
        data_size: u32,
        p_data: *const ::std::ffi::c_void,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).3)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(name),
            ::std::mem::transmute(data_size),
            ::std::mem::transmute(p_data),
        )
    }
    pub unsafe fn SetPrivateDataInterface<
        'a,
        T1__: ::windows::IntoParam<'a, ::windows::IUnknown>,
    >(
        &self,
        name: *const ::windows::Guid,
        p_unknown: T1__,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).4)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(name),
            p_unknown.into_param().abi(),
        )
    }
    pub unsafe fn GetPrivateData(
        &self,
        name: *const ::windows::Guid,
        p_data_size: *mut u32,
        p_data: *mut ::std::ffi::c_void,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).5)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(name),
            ::std::mem::transmute(p_data_size),
            ::std::mem::transmute(p_data),
        )
    }
    pub unsafe fn GetParent(
        &self,
        riid: *const ::windows::Guid,
        pp_parent: *mut *mut ::std::ffi::c_void,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).6)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(riid),
            ::std::mem::transmute(pp_parent),
        )
    }
    pub unsafe fn GetAdapter(
        &self,
        p_adapter: *mut ::std::option::Option<IDXGIAdapter>,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).7)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(p_adapter),
        )
    }
    pub unsafe fn CreateSurface(
        &self,
        p_desc: *const DXGI_SURFACE_DESC,
        num_surfaces: u32,
        usage: u32,
        p_shared_resource: *const DXGI_SHARED_RESOURCE,
        pp_surface: *mut ::std::option::Option<IDXGISurface>,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).8)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(p_desc),
            ::std::mem::transmute(num_surfaces),
            ::std::mem::transmute(usage),
            ::std::mem::transmute(p_shared_resource),
            ::std::mem::transmute(pp_surface),
        )
    }
    pub unsafe fn QueryResourceResidency(
        &self,
        pp_resources: *mut ::std::option::Option<::windows::IUnknown>,
        p_residency_status: *mut DXGI_RESIDENCY,
        num_resources: u32,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).9)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(pp_resources),
            ::std::mem::transmute(p_residency_status),
            ::std::mem::transmute(num_resources),
        )
    }
    pub unsafe fn SetGPUThreadPriority(&self, priority: i32) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).10)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(priority),
        )
    }
    pub unsafe fn GetGPUThreadPriority(&self, p_priority: *mut i32) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).11)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(p_priority),
        )
    }
}
impl ::std::convert::From<IDXGIDevice> for ::windows::IUnknown {
    fn from(value: IDXGIDevice) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IDXGIDevice> for ::windows::IUnknown {
    fn from(value: &IDXGIDevice) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::IntoParam<'a, ::windows::IUnknown> for IDXGIDevice {
    fn into_param(self) -> ::windows::Param<'a, ::windows::IUnknown> {
        ::windows::Param::Owned(::std::convert::Into::<::windows::IUnknown>::into(self))
    }
}
impl<'a> ::windows::IntoParam<'a, ::windows::IUnknown> for &'a IDXGIDevice {
    fn into_param(self) -> ::windows::Param<'a, ::windows::IUnknown> {
        ::windows::Param::Owned(::std::convert::Into::<::windows::IUnknown>::into(
            ::std::clone::Clone::clone(self),
        ))
    }
}
impl ::std::convert::From<IDXGIDevice> for IDXGIObject {
    fn from(value: IDXGIDevice) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IDXGIDevice> for IDXGIObject {
    fn from(value: &IDXGIDevice) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::IntoParam<'a, IDXGIObject> for IDXGIDevice {
    fn into_param(self) -> ::windows::Param<'a, IDXGIObject> {
        ::windows::Param::Owned(::std::convert::Into::<IDXGIObject>::into(self))
    }
}
impl<'a> ::windows::IntoParam<'a, IDXGIObject> for &'a IDXGIDevice {
    fn into_param(self) -> ::windows::Param<'a, IDXGIObject> {
        ::windows::Param::Owned(::std::convert::Into::<IDXGIObject>::into(
            ::std::clone::Clone::clone(self),
        ))
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDXGIDevice_abi(
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        iid: &::windows::Guid,
        interface: *mut ::windows::RawPtr,
    ) -> ::windows::ErrorCode,
    pub unsafe extern "system" fn(this: ::windows::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::RawPtr) -> u32,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        name: *const ::windows::Guid,
        data_size: u32,
        p_data: *const ::std::ffi::c_void,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        name: *const ::windows::Guid,
        p_unknown: ::windows::RawPtr,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        name: *const ::windows::Guid,
        p_data_size: *mut u32,
        p_data: *mut ::std::ffi::c_void,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        riid: *const ::windows::Guid,
        pp_parent: *mut *mut ::std::ffi::c_void,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        p_adapter: *mut ::windows::RawPtr,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        p_desc: *const DXGI_SURFACE_DESC,
        num_surfaces: u32,
        usage: u32,
        p_shared_resource: *const DXGI_SHARED_RESOURCE,
        pp_surface: *mut ::windows::RawPtr,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        pp_resources: *mut ::windows::RawPtr,
        p_residency_status: *mut DXGI_RESIDENCY,
        num_resources: u32,
    ) -> ::windows::ErrorCode,
    pub unsafe extern "system" fn(this: ::windows::RawPtr, priority: i32) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        p_priority: *mut i32,
    ) -> ::windows::ErrorCode,
);
#[repr(transparent)]
#[allow(non_camel_case_types)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct IDXGIDevice1(::windows::IUnknown);
impl IDXGIDevice1 {}
unsafe impl ::windows::Interface for IDXGIDevice1 {
    type Vtable = IDXGIDevice1_abi;
    const IID: ::windows::Guid =
        ::windows::Guid::from_values(2010879759, 25206, 18618, [186, 40, 7, 1, 67, 180, 57, 44]);
}
#[allow(non_snake_case)]
impl IDXGIDevice1 {
    pub unsafe fn SetPrivateData(
        &self,
        name: *const ::windows::Guid,
        data_size: u32,
        p_data: *const ::std::ffi::c_void,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).3)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(name),
            ::std::mem::transmute(data_size),
            ::std::mem::transmute(p_data),
        )
    }
    pub unsafe fn SetPrivateDataInterface<
        'a,
        T1__: ::windows::IntoParam<'a, ::windows::IUnknown>,
    >(
        &self,
        name: *const ::windows::Guid,
        p_unknown: T1__,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).4)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(name),
            p_unknown.into_param().abi(),
        )
    }
    pub unsafe fn GetPrivateData(
        &self,
        name: *const ::windows::Guid,
        p_data_size: *mut u32,
        p_data: *mut ::std::ffi::c_void,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).5)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(name),
            ::std::mem::transmute(p_data_size),
            ::std::mem::transmute(p_data),
        )
    }
    pub unsafe fn GetParent(
        &self,
        riid: *const ::windows::Guid,
        pp_parent: *mut *mut ::std::ffi::c_void,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).6)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(riid),
            ::std::mem::transmute(pp_parent),
        )
    }
    pub unsafe fn GetAdapter(
        &self,
        p_adapter: *mut ::std::option::Option<IDXGIAdapter>,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).7)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(p_adapter),
        )
    }
    pub unsafe fn CreateSurface(
        &self,
        p_desc: *const DXGI_SURFACE_DESC,
        num_surfaces: u32,
        usage: u32,
        p_shared_resource: *const DXGI_SHARED_RESOURCE,
        pp_surface: *mut ::std::option::Option<IDXGISurface>,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).8)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(p_desc),
            ::std::mem::transmute(num_surfaces),
            ::std::mem::transmute(usage),
            ::std::mem::transmute(p_shared_resource),
            ::std::mem::transmute(pp_surface),
        )
    }
    pub unsafe fn QueryResourceResidency(
        &self,
        pp_resources: *mut ::std::option::Option<::windows::IUnknown>,
        p_residency_status: *mut DXGI_RESIDENCY,
        num_resources: u32,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).9)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(pp_resources),
            ::std::mem::transmute(p_residency_status),
            ::std::mem::transmute(num_resources),
        )
    }
    pub unsafe fn SetGPUThreadPriority(&self, priority: i32) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).10)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(priority),
        )
    }
    pub unsafe fn GetGPUThreadPriority(&self, p_priority: *mut i32) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).11)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(p_priority),
        )
    }
    pub unsafe fn SetMaximumFrameLatency(&self, max_latency: u32) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).12)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(max_latency),
        )
    }
    pub unsafe fn GetMaximumFrameLatency(&self, p_max_latency: *mut u32) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).13)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(p_max_latency),
        )
    }
}
impl ::std::convert::From<IDXGIDevice1> for ::windows::IUnknown {
    fn from(value: IDXGIDevice1) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IDXGIDevice1> for ::windows::IUnknown {
    fn from(value: &IDXGIDevice1) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::IntoParam<'a, ::windows::IUnknown> for IDXGIDevice1 {
    fn into_param(self) -> ::windows::Param<'a, ::windows::IUnknown> {
        ::windows::Param::Owned(::std::convert::Into::<::windows::IUnknown>::into(self))
    }
}
impl<'a> ::windows::IntoParam<'a, ::windows::IUnknown> for &'a IDXGIDevice1 {
    fn into_param(self) -> ::windows::Param<'a, ::windows::IUnknown> {
        ::windows::Param::Owned(::std::convert::Into::<::windows::IUnknown>::into(
            ::std::clone::Clone::clone(self),
        ))
    }
}
impl ::std::convert::From<IDXGIDevice1> for IDXGIDevice {
    fn from(value: IDXGIDevice1) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IDXGIDevice1> for IDXGIDevice {
    fn from(value: &IDXGIDevice1) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::IntoParam<'a, IDXGIDevice> for IDXGIDevice1 {
    fn into_param(self) -> ::windows::Param<'a, IDXGIDevice> {
        ::windows::Param::Owned(::std::convert::Into::<IDXGIDevice>::into(self))
    }
}
impl<'a> ::windows::IntoParam<'a, IDXGIDevice> for &'a IDXGIDevice1 {
    fn into_param(self) -> ::windows::Param<'a, IDXGIDevice> {
        ::windows::Param::Owned(::std::convert::Into::<IDXGIDevice>::into(
            ::std::clone::Clone::clone(self),
        ))
    }
}
impl ::std::convert::From<IDXGIDevice1> for IDXGIObject {
    fn from(value: IDXGIDevice1) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IDXGIDevice1> for IDXGIObject {
    fn from(value: &IDXGIDevice1) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::IntoParam<'a, IDXGIObject> for IDXGIDevice1 {
    fn into_param(self) -> ::windows::Param<'a, IDXGIObject> {
        ::windows::Param::Owned(::std::convert::Into::<IDXGIObject>::into(self))
    }
}
impl<'a> ::windows::IntoParam<'a, IDXGIObject> for &'a IDXGIDevice1 {
    fn into_param(self) -> ::windows::Param<'a, IDXGIObject> {
        ::windows::Param::Owned(::std::convert::Into::<IDXGIObject>::into(
            ::std::clone::Clone::clone(self),
        ))
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDXGIDevice1_abi(
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        iid: &::windows::Guid,
        interface: *mut ::windows::RawPtr,
    ) -> ::windows::ErrorCode,
    pub unsafe extern "system" fn(this: ::windows::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::RawPtr) -> u32,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        name: *const ::windows::Guid,
        data_size: u32,
        p_data: *const ::std::ffi::c_void,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        name: *const ::windows::Guid,
        p_unknown: ::windows::RawPtr,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        name: *const ::windows::Guid,
        p_data_size: *mut u32,
        p_data: *mut ::std::ffi::c_void,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        riid: *const ::windows::Guid,
        pp_parent: *mut *mut ::std::ffi::c_void,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        p_adapter: *mut ::windows::RawPtr,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        p_desc: *const DXGI_SURFACE_DESC,
        num_surfaces: u32,
        usage: u32,
        p_shared_resource: *const DXGI_SHARED_RESOURCE,
        pp_surface: *mut ::windows::RawPtr,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        pp_resources: *mut ::windows::RawPtr,
        p_residency_status: *mut DXGI_RESIDENCY,
        num_resources: u32,
    ) -> ::windows::ErrorCode,
    pub unsafe extern "system" fn(this: ::windows::RawPtr, priority: i32) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        p_priority: *mut i32,
    ) -> ::windows::ErrorCode,
    pub unsafe extern "system" fn(this: ::windows::RawPtr, max_latency: u32) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        p_max_latency: *mut u32,
    ) -> ::windows::ErrorCode,
);
#[repr(transparent)]
#[allow(non_camel_case_types)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct IDXGIDevice2(::windows::IUnknown);
impl IDXGIDevice2 {}
unsafe impl ::windows::Interface for IDXGIDevice2 {
    type Vtable = IDXGIDevice2_abi;
    const IID: ::windows::Guid = ::windows::Guid::from_values(
        83920407,
        64509,
        16465,
        [167, 144, 20, 72, 132, 180, 246, 169],
    );
}
#[allow(non_snake_case)]
impl IDXGIDevice2 {
    pub unsafe fn SetPrivateData(
        &self,
        name: *const ::windows::Guid,
        data_size: u32,
        p_data: *const ::std::ffi::c_void,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).3)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(name),
            ::std::mem::transmute(data_size),
            ::std::mem::transmute(p_data),
        )
    }
    pub unsafe fn SetPrivateDataInterface<
        'a,
        T1__: ::windows::IntoParam<'a, ::windows::IUnknown>,
    >(
        &self,
        name: *const ::windows::Guid,
        p_unknown: T1__,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).4)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(name),
            p_unknown.into_param().abi(),
        )
    }
    pub unsafe fn GetPrivateData(
        &self,
        name: *const ::windows::Guid,
        p_data_size: *mut u32,
        p_data: *mut ::std::ffi::c_void,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).5)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(name),
            ::std::mem::transmute(p_data_size),
            ::std::mem::transmute(p_data),
        )
    }
    pub unsafe fn GetParent(
        &self,
        riid: *const ::windows::Guid,
        pp_parent: *mut *mut ::std::ffi::c_void,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).6)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(riid),
            ::std::mem::transmute(pp_parent),
        )
    }
    pub unsafe fn GetAdapter(
        &self,
        p_adapter: *mut ::std::option::Option<IDXGIAdapter>,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).7)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(p_adapter),
        )
    }
    pub unsafe fn CreateSurface(
        &self,
        p_desc: *const DXGI_SURFACE_DESC,
        num_surfaces: u32,
        usage: u32,
        p_shared_resource: *const DXGI_SHARED_RESOURCE,
        pp_surface: *mut ::std::option::Option<IDXGISurface>,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).8)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(p_desc),
            ::std::mem::transmute(num_surfaces),
            ::std::mem::transmute(usage),
            ::std::mem::transmute(p_shared_resource),
            ::std::mem::transmute(pp_surface),
        )
    }
    pub unsafe fn QueryResourceResidency(
        &self,
        pp_resources: *mut ::std::option::Option<::windows::IUnknown>,
        p_residency_status: *mut DXGI_RESIDENCY,
        num_resources: u32,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).9)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(pp_resources),
            ::std::mem::transmute(p_residency_status),
            ::std::mem::transmute(num_resources),
        )
    }
    pub unsafe fn SetGPUThreadPriority(&self, priority: i32) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).10)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(priority),
        )
    }
    pub unsafe fn GetGPUThreadPriority(&self, p_priority: *mut i32) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).11)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(p_priority),
        )
    }
    pub unsafe fn SetMaximumFrameLatency(&self, max_latency: u32) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).12)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(max_latency),
        )
    }
    pub unsafe fn GetMaximumFrameLatency(&self, p_max_latency: *mut u32) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).13)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(p_max_latency),
        )
    }
    pub unsafe fn OfferResources(
        &self,
        num_resources: u32,
        pp_resources: *mut ::std::option::Option<IDXGIResource>,
        priority: DXGI_OFFER_RESOURCE_PRIORITY,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).14)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(num_resources),
            ::std::mem::transmute(pp_resources),
            ::std::mem::transmute(priority),
        )
    }
    pub unsafe fn ReclaimResources(
        &self,
        num_resources: u32,
        pp_resources: *mut ::std::option::Option<IDXGIResource>,
        p_discarded: *mut super::system_services::BOOL,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).15)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(num_resources),
            ::std::mem::transmute(pp_resources),
            ::std::mem::transmute(p_discarded),
        )
    }
    pub unsafe fn EnqueueSetEvent<
        'a,
        T0__: ::windows::IntoParam<'a, super::system_services::HANDLE>,
    >(
        &self,
        h_event: T0__,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).16)(
            ::windows::Abi::abi(self),
            h_event.into_param().abi(),
        )
    }
}
impl ::std::convert::From<IDXGIDevice2> for ::windows::IUnknown {
    fn from(value: IDXGIDevice2) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IDXGIDevice2> for ::windows::IUnknown {
    fn from(value: &IDXGIDevice2) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::IntoParam<'a, ::windows::IUnknown> for IDXGIDevice2 {
    fn into_param(self) -> ::windows::Param<'a, ::windows::IUnknown> {
        ::windows::Param::Owned(::std::convert::Into::<::windows::IUnknown>::into(self))
    }
}
impl<'a> ::windows::IntoParam<'a, ::windows::IUnknown> for &'a IDXGIDevice2 {
    fn into_param(self) -> ::windows::Param<'a, ::windows::IUnknown> {
        ::windows::Param::Owned(::std::convert::Into::<::windows::IUnknown>::into(
            ::std::clone::Clone::clone(self),
        ))
    }
}
impl ::std::convert::From<IDXGIDevice2> for IDXGIDevice1 {
    fn from(value: IDXGIDevice2) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IDXGIDevice2> for IDXGIDevice1 {
    fn from(value: &IDXGIDevice2) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::IntoParam<'a, IDXGIDevice1> for IDXGIDevice2 {
    fn into_param(self) -> ::windows::Param<'a, IDXGIDevice1> {
        ::windows::Param::Owned(::std::convert::Into::<IDXGIDevice1>::into(self))
    }
}
impl<'a> ::windows::IntoParam<'a, IDXGIDevice1> for &'a IDXGIDevice2 {
    fn into_param(self) -> ::windows::Param<'a, IDXGIDevice1> {
        ::windows::Param::Owned(::std::convert::Into::<IDXGIDevice1>::into(
            ::std::clone::Clone::clone(self),
        ))
    }
}
impl ::std::convert::From<IDXGIDevice2> for IDXGIDevice {
    fn from(value: IDXGIDevice2) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IDXGIDevice2> for IDXGIDevice {
    fn from(value: &IDXGIDevice2) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::IntoParam<'a, IDXGIDevice> for IDXGIDevice2 {
    fn into_param(self) -> ::windows::Param<'a, IDXGIDevice> {
        ::windows::Param::Owned(::std::convert::Into::<IDXGIDevice>::into(self))
    }
}
impl<'a> ::windows::IntoParam<'a, IDXGIDevice> for &'a IDXGIDevice2 {
    fn into_param(self) -> ::windows::Param<'a, IDXGIDevice> {
        ::windows::Param::Owned(::std::convert::Into::<IDXGIDevice>::into(
            ::std::clone::Clone::clone(self),
        ))
    }
}
impl ::std::convert::From<IDXGIDevice2> for IDXGIObject {
    fn from(value: IDXGIDevice2) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IDXGIDevice2> for IDXGIObject {
    fn from(value: &IDXGIDevice2) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::IntoParam<'a, IDXGIObject> for IDXGIDevice2 {
    fn into_param(self) -> ::windows::Param<'a, IDXGIObject> {
        ::windows::Param::Owned(::std::convert::Into::<IDXGIObject>::into(self))
    }
}
impl<'a> ::windows::IntoParam<'a, IDXGIObject> for &'a IDXGIDevice2 {
    fn into_param(self) -> ::windows::Param<'a, IDXGIObject> {
        ::windows::Param::Owned(::std::convert::Into::<IDXGIObject>::into(
            ::std::clone::Clone::clone(self),
        ))
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDXGIDevice2_abi(
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        iid: &::windows::Guid,
        interface: *mut ::windows::RawPtr,
    ) -> ::windows::ErrorCode,
    pub unsafe extern "system" fn(this: ::windows::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::RawPtr) -> u32,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        name: *const ::windows::Guid,
        data_size: u32,
        p_data: *const ::std::ffi::c_void,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        name: *const ::windows::Guid,
        p_unknown: ::windows::RawPtr,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        name: *const ::windows::Guid,
        p_data_size: *mut u32,
        p_data: *mut ::std::ffi::c_void,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        riid: *const ::windows::Guid,
        pp_parent: *mut *mut ::std::ffi::c_void,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        p_adapter: *mut ::windows::RawPtr,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        p_desc: *const DXGI_SURFACE_DESC,
        num_surfaces: u32,
        usage: u32,
        p_shared_resource: *const DXGI_SHARED_RESOURCE,
        pp_surface: *mut ::windows::RawPtr,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        pp_resources: *mut ::windows::RawPtr,
        p_residency_status: *mut DXGI_RESIDENCY,
        num_resources: u32,
    ) -> ::windows::ErrorCode,
    pub unsafe extern "system" fn(this: ::windows::RawPtr, priority: i32) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        p_priority: *mut i32,
    ) -> ::windows::ErrorCode,
    pub unsafe extern "system" fn(this: ::windows::RawPtr, max_latency: u32) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        p_max_latency: *mut u32,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        num_resources: u32,
        pp_resources: *mut ::windows::RawPtr,
        priority: DXGI_OFFER_RESOURCE_PRIORITY,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        num_resources: u32,
        pp_resources: *mut ::windows::RawPtr,
        p_discarded: *mut super::system_services::BOOL,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        h_event: super::system_services::HANDLE,
    ) -> ::windows::ErrorCode,
);
#[repr(transparent)]
#[allow(non_camel_case_types)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct IDXGIDevice3(::windows::IUnknown);
impl IDXGIDevice3 {}
unsafe impl ::windows::Interface for IDXGIDevice3 {
    type Vtable = IDXGIDevice3_abi;
    const IID: ::windows::Guid = ::windows::Guid::from_values(
        1611106668,
        12868,
        19197,
        [191, 24, 166, 211, 190, 218, 80, 35],
    );
}
#[allow(non_snake_case)]
impl IDXGIDevice3 {
    pub unsafe fn SetPrivateData(
        &self,
        name: *const ::windows::Guid,
        data_size: u32,
        p_data: *const ::std::ffi::c_void,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).3)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(name),
            ::std::mem::transmute(data_size),
            ::std::mem::transmute(p_data),
        )
    }
    pub unsafe fn SetPrivateDataInterface<
        'a,
        T1__: ::windows::IntoParam<'a, ::windows::IUnknown>,
    >(
        &self,
        name: *const ::windows::Guid,
        p_unknown: T1__,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).4)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(name),
            p_unknown.into_param().abi(),
        )
    }
    pub unsafe fn GetPrivateData(
        &self,
        name: *const ::windows::Guid,
        p_data_size: *mut u32,
        p_data: *mut ::std::ffi::c_void,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).5)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(name),
            ::std::mem::transmute(p_data_size),
            ::std::mem::transmute(p_data),
        )
    }
    pub unsafe fn GetParent(
        &self,
        riid: *const ::windows::Guid,
        pp_parent: *mut *mut ::std::ffi::c_void,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).6)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(riid),
            ::std::mem::transmute(pp_parent),
        )
    }
    pub unsafe fn GetAdapter(
        &self,
        p_adapter: *mut ::std::option::Option<IDXGIAdapter>,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).7)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(p_adapter),
        )
    }
    pub unsafe fn CreateSurface(
        &self,
        p_desc: *const DXGI_SURFACE_DESC,
        num_surfaces: u32,
        usage: u32,
        p_shared_resource: *const DXGI_SHARED_RESOURCE,
        pp_surface: *mut ::std::option::Option<IDXGISurface>,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).8)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(p_desc),
            ::std::mem::transmute(num_surfaces),
            ::std::mem::transmute(usage),
            ::std::mem::transmute(p_shared_resource),
            ::std::mem::transmute(pp_surface),
        )
    }
    pub unsafe fn QueryResourceResidency(
        &self,
        pp_resources: *mut ::std::option::Option<::windows::IUnknown>,
        p_residency_status: *mut DXGI_RESIDENCY,
        num_resources: u32,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).9)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(pp_resources),
            ::std::mem::transmute(p_residency_status),
            ::std::mem::transmute(num_resources),
        )
    }
    pub unsafe fn SetGPUThreadPriority(&self, priority: i32) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).10)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(priority),
        )
    }
    pub unsafe fn GetGPUThreadPriority(&self, p_priority: *mut i32) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).11)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(p_priority),
        )
    }
    pub unsafe fn SetMaximumFrameLatency(&self, max_latency: u32) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).12)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(max_latency),
        )
    }
    pub unsafe fn GetMaximumFrameLatency(&self, p_max_latency: *mut u32) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).13)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(p_max_latency),
        )
    }
    pub unsafe fn OfferResources(
        &self,
        num_resources: u32,
        pp_resources: *mut ::std::option::Option<IDXGIResource>,
        priority: DXGI_OFFER_RESOURCE_PRIORITY,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).14)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(num_resources),
            ::std::mem::transmute(pp_resources),
            ::std::mem::transmute(priority),
        )
    }
    pub unsafe fn ReclaimResources(
        &self,
        num_resources: u32,
        pp_resources: *mut ::std::option::Option<IDXGIResource>,
        p_discarded: *mut super::system_services::BOOL,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).15)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(num_resources),
            ::std::mem::transmute(pp_resources),
            ::std::mem::transmute(p_discarded),
        )
    }
    pub unsafe fn EnqueueSetEvent<
        'a,
        T0__: ::windows::IntoParam<'a, super::system_services::HANDLE>,
    >(
        &self,
        h_event: T0__,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).16)(
            ::windows::Abi::abi(self),
            h_event.into_param().abi(),
        )
    }
    pub unsafe fn Trim(&self) {
        (::windows::Interface::vtable(self).17)(::windows::Abi::abi(self))
    }
}
impl ::std::convert::From<IDXGIDevice3> for ::windows::IUnknown {
    fn from(value: IDXGIDevice3) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IDXGIDevice3> for ::windows::IUnknown {
    fn from(value: &IDXGIDevice3) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::IntoParam<'a, ::windows::IUnknown> for IDXGIDevice3 {
    fn into_param(self) -> ::windows::Param<'a, ::windows::IUnknown> {
        ::windows::Param::Owned(::std::convert::Into::<::windows::IUnknown>::into(self))
    }
}
impl<'a> ::windows::IntoParam<'a, ::windows::IUnknown> for &'a IDXGIDevice3 {
    fn into_param(self) -> ::windows::Param<'a, ::windows::IUnknown> {
        ::windows::Param::Owned(::std::convert::Into::<::windows::IUnknown>::into(
            ::std::clone::Clone::clone(self),
        ))
    }
}
impl ::std::convert::From<IDXGIDevice3> for IDXGIDevice2 {
    fn from(value: IDXGIDevice3) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IDXGIDevice3> for IDXGIDevice2 {
    fn from(value: &IDXGIDevice3) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::IntoParam<'a, IDXGIDevice2> for IDXGIDevice3 {
    fn into_param(self) -> ::windows::Param<'a, IDXGIDevice2> {
        ::windows::Param::Owned(::std::convert::Into::<IDXGIDevice2>::into(self))
    }
}
impl<'a> ::windows::IntoParam<'a, IDXGIDevice2> for &'a IDXGIDevice3 {
    fn into_param(self) -> ::windows::Param<'a, IDXGIDevice2> {
        ::windows::Param::Owned(::std::convert::Into::<IDXGIDevice2>::into(
            ::std::clone::Clone::clone(self),
        ))
    }
}
impl ::std::convert::From<IDXGIDevice3> for IDXGIDevice1 {
    fn from(value: IDXGIDevice3) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IDXGIDevice3> for IDXGIDevice1 {
    fn from(value: &IDXGIDevice3) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::IntoParam<'a, IDXGIDevice1> for IDXGIDevice3 {
    fn into_param(self) -> ::windows::Param<'a, IDXGIDevice1> {
        ::windows::Param::Owned(::std::convert::Into::<IDXGIDevice1>::into(self))
    }
}
impl<'a> ::windows::IntoParam<'a, IDXGIDevice1> for &'a IDXGIDevice3 {
    fn into_param(self) -> ::windows::Param<'a, IDXGIDevice1> {
        ::windows::Param::Owned(::std::convert::Into::<IDXGIDevice1>::into(
            ::std::clone::Clone::clone(self),
        ))
    }
}
impl ::std::convert::From<IDXGIDevice3> for IDXGIDevice {
    fn from(value: IDXGIDevice3) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IDXGIDevice3> for IDXGIDevice {
    fn from(value: &IDXGIDevice3) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::IntoParam<'a, IDXGIDevice> for IDXGIDevice3 {
    fn into_param(self) -> ::windows::Param<'a, IDXGIDevice> {
        ::windows::Param::Owned(::std::convert::Into::<IDXGIDevice>::into(self))
    }
}
impl<'a> ::windows::IntoParam<'a, IDXGIDevice> for &'a IDXGIDevice3 {
    fn into_param(self) -> ::windows::Param<'a, IDXGIDevice> {
        ::windows::Param::Owned(::std::convert::Into::<IDXGIDevice>::into(
            ::std::clone::Clone::clone(self),
        ))
    }
}
impl ::std::convert::From<IDXGIDevice3> for IDXGIObject {
    fn from(value: IDXGIDevice3) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IDXGIDevice3> for IDXGIObject {
    fn from(value: &IDXGIDevice3) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::IntoParam<'a, IDXGIObject> for IDXGIDevice3 {
    fn into_param(self) -> ::windows::Param<'a, IDXGIObject> {
        ::windows::Param::Owned(::std::convert::Into::<IDXGIObject>::into(self))
    }
}
impl<'a> ::windows::IntoParam<'a, IDXGIObject> for &'a IDXGIDevice3 {
    fn into_param(self) -> ::windows::Param<'a, IDXGIObject> {
        ::windows::Param::Owned(::std::convert::Into::<IDXGIObject>::into(
            ::std::clone::Clone::clone(self),
        ))
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDXGIDevice3_abi(
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        iid: &::windows::Guid,
        interface: *mut ::windows::RawPtr,
    ) -> ::windows::ErrorCode,
    pub unsafe extern "system" fn(this: ::windows::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::RawPtr) -> u32,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        name: *const ::windows::Guid,
        data_size: u32,
        p_data: *const ::std::ffi::c_void,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        name: *const ::windows::Guid,
        p_unknown: ::windows::RawPtr,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        name: *const ::windows::Guid,
        p_data_size: *mut u32,
        p_data: *mut ::std::ffi::c_void,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        riid: *const ::windows::Guid,
        pp_parent: *mut *mut ::std::ffi::c_void,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        p_adapter: *mut ::windows::RawPtr,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        p_desc: *const DXGI_SURFACE_DESC,
        num_surfaces: u32,
        usage: u32,
        p_shared_resource: *const DXGI_SHARED_RESOURCE,
        pp_surface: *mut ::windows::RawPtr,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        pp_resources: *mut ::windows::RawPtr,
        p_residency_status: *mut DXGI_RESIDENCY,
        num_resources: u32,
    ) -> ::windows::ErrorCode,
    pub unsafe extern "system" fn(this: ::windows::RawPtr, priority: i32) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        p_priority: *mut i32,
    ) -> ::windows::ErrorCode,
    pub unsafe extern "system" fn(this: ::windows::RawPtr, max_latency: u32) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        p_max_latency: *mut u32,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        num_resources: u32,
        pp_resources: *mut ::windows::RawPtr,
        priority: DXGI_OFFER_RESOURCE_PRIORITY,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        num_resources: u32,
        pp_resources: *mut ::windows::RawPtr,
        p_discarded: *mut super::system_services::BOOL,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        h_event: super::system_services::HANDLE,
    ) -> ::windows::ErrorCode,
    pub unsafe extern "system" fn(this: ::windows::RawPtr),
);
#[repr(transparent)]
#[allow(non_camel_case_types)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct IDXGIDevice4(::windows::IUnknown);
impl IDXGIDevice4 {}
unsafe impl ::windows::Interface for IDXGIDevice4 {
    type Vtable = IDXGIDevice4_abi;
    const IID: ::windows::Guid = ::windows::Guid::from_values(
        2511665503,
        55514,
        19620,
        [158, 230, 59, 118, 213, 150, 138, 16],
    );
}
#[allow(non_snake_case)]
impl IDXGIDevice4 {
    pub unsafe fn SetPrivateData(
        &self,
        name: *const ::windows::Guid,
        data_size: u32,
        p_data: *const ::std::ffi::c_void,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).3)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(name),
            ::std::mem::transmute(data_size),
            ::std::mem::transmute(p_data),
        )
    }
    pub unsafe fn SetPrivateDataInterface<
        'a,
        T1__: ::windows::IntoParam<'a, ::windows::IUnknown>,
    >(
        &self,
        name: *const ::windows::Guid,
        p_unknown: T1__,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).4)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(name),
            p_unknown.into_param().abi(),
        )
    }
    pub unsafe fn GetPrivateData(
        &self,
        name: *const ::windows::Guid,
        p_data_size: *mut u32,
        p_data: *mut ::std::ffi::c_void,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).5)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(name),
            ::std::mem::transmute(p_data_size),
            ::std::mem::transmute(p_data),
        )
    }
    pub unsafe fn GetParent(
        &self,
        riid: *const ::windows::Guid,
        pp_parent: *mut *mut ::std::ffi::c_void,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).6)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(riid),
            ::std::mem::transmute(pp_parent),
        )
    }
    pub unsafe fn GetAdapter(
        &self,
        p_adapter: *mut ::std::option::Option<IDXGIAdapter>,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).7)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(p_adapter),
        )
    }
    pub unsafe fn CreateSurface(
        &self,
        p_desc: *const DXGI_SURFACE_DESC,
        num_surfaces: u32,
        usage: u32,
        p_shared_resource: *const DXGI_SHARED_RESOURCE,
        pp_surface: *mut ::std::option::Option<IDXGISurface>,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).8)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(p_desc),
            ::std::mem::transmute(num_surfaces),
            ::std::mem::transmute(usage),
            ::std::mem::transmute(p_shared_resource),
            ::std::mem::transmute(pp_surface),
        )
    }
    pub unsafe fn QueryResourceResidency(
        &self,
        pp_resources: *mut ::std::option::Option<::windows::IUnknown>,
        p_residency_status: *mut DXGI_RESIDENCY,
        num_resources: u32,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).9)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(pp_resources),
            ::std::mem::transmute(p_residency_status),
            ::std::mem::transmute(num_resources),
        )
    }
    pub unsafe fn SetGPUThreadPriority(&self, priority: i32) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).10)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(priority),
        )
    }
    pub unsafe fn GetGPUThreadPriority(&self, p_priority: *mut i32) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).11)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(p_priority),
        )
    }
    pub unsafe fn SetMaximumFrameLatency(&self, max_latency: u32) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).12)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(max_latency),
        )
    }
    pub unsafe fn GetMaximumFrameLatency(&self, p_max_latency: *mut u32) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).13)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(p_max_latency),
        )
    }
    pub unsafe fn OfferResources(
        &self,
        num_resources: u32,
        pp_resources: *mut ::std::option::Option<IDXGIResource>,
        priority: DXGI_OFFER_RESOURCE_PRIORITY,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).14)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(num_resources),
            ::std::mem::transmute(pp_resources),
            ::std::mem::transmute(priority),
        )
    }
    pub unsafe fn ReclaimResources(
        &self,
        num_resources: u32,
        pp_resources: *mut ::std::option::Option<IDXGIResource>,
        p_discarded: *mut super::system_services::BOOL,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).15)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(num_resources),
            ::std::mem::transmute(pp_resources),
            ::std::mem::transmute(p_discarded),
        )
    }
    pub unsafe fn EnqueueSetEvent<
        'a,
        T0__: ::windows::IntoParam<'a, super::system_services::HANDLE>,
    >(
        &self,
        h_event: T0__,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).16)(
            ::windows::Abi::abi(self),
            h_event.into_param().abi(),
        )
    }
    pub unsafe fn Trim(&self) {
        (::windows::Interface::vtable(self).17)(::windows::Abi::abi(self))
    }
    pub unsafe fn OfferResources1(
        &self,
        num_resources: u32,
        pp_resources: *mut ::std::option::Option<IDXGIResource>,
        priority: DXGI_OFFER_RESOURCE_PRIORITY,
        flags: u32,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).18)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(num_resources),
            ::std::mem::transmute(pp_resources),
            ::std::mem::transmute(priority),
            ::std::mem::transmute(flags),
        )
    }
    pub unsafe fn ReclaimResources1(
        &self,
        num_resources: u32,
        pp_resources: *mut ::std::option::Option<IDXGIResource>,
        p_results: *mut DXGI_RECLAIM_RESOURCE_RESULTS,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).19)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(num_resources),
            ::std::mem::transmute(pp_resources),
            ::std::mem::transmute(p_results),
        )
    }
}
impl ::std::convert::From<IDXGIDevice4> for ::windows::IUnknown {
    fn from(value: IDXGIDevice4) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IDXGIDevice4> for ::windows::IUnknown {
    fn from(value: &IDXGIDevice4) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::IntoParam<'a, ::windows::IUnknown> for IDXGIDevice4 {
    fn into_param(self) -> ::windows::Param<'a, ::windows::IUnknown> {
        ::windows::Param::Owned(::std::convert::Into::<::windows::IUnknown>::into(self))
    }
}
impl<'a> ::windows::IntoParam<'a, ::windows::IUnknown> for &'a IDXGIDevice4 {
    fn into_param(self) -> ::windows::Param<'a, ::windows::IUnknown> {
        ::windows::Param::Owned(::std::convert::Into::<::windows::IUnknown>::into(
            ::std::clone::Clone::clone(self),
        ))
    }
}
impl ::std::convert::From<IDXGIDevice4> for IDXGIDevice3 {
    fn from(value: IDXGIDevice4) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IDXGIDevice4> for IDXGIDevice3 {
    fn from(value: &IDXGIDevice4) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::IntoParam<'a, IDXGIDevice3> for IDXGIDevice4 {
    fn into_param(self) -> ::windows::Param<'a, IDXGIDevice3> {
        ::windows::Param::Owned(::std::convert::Into::<IDXGIDevice3>::into(self))
    }
}
impl<'a> ::windows::IntoParam<'a, IDXGIDevice3> for &'a IDXGIDevice4 {
    fn into_param(self) -> ::windows::Param<'a, IDXGIDevice3> {
        ::windows::Param::Owned(::std::convert::Into::<IDXGIDevice3>::into(
            ::std::clone::Clone::clone(self),
        ))
    }
}
impl ::std::convert::From<IDXGIDevice4> for IDXGIDevice2 {
    fn from(value: IDXGIDevice4) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IDXGIDevice4> for IDXGIDevice2 {
    fn from(value: &IDXGIDevice4) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::IntoParam<'a, IDXGIDevice2> for IDXGIDevice4 {
    fn into_param(self) -> ::windows::Param<'a, IDXGIDevice2> {
        ::windows::Param::Owned(::std::convert::Into::<IDXGIDevice2>::into(self))
    }
}
impl<'a> ::windows::IntoParam<'a, IDXGIDevice2> for &'a IDXGIDevice4 {
    fn into_param(self) -> ::windows::Param<'a, IDXGIDevice2> {
        ::windows::Param::Owned(::std::convert::Into::<IDXGIDevice2>::into(
            ::std::clone::Clone::clone(self),
        ))
    }
}
impl ::std::convert::From<IDXGIDevice4> for IDXGIDevice1 {
    fn from(value: IDXGIDevice4) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IDXGIDevice4> for IDXGIDevice1 {
    fn from(value: &IDXGIDevice4) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::IntoParam<'a, IDXGIDevice1> for IDXGIDevice4 {
    fn into_param(self) -> ::windows::Param<'a, IDXGIDevice1> {
        ::windows::Param::Owned(::std::convert::Into::<IDXGIDevice1>::into(self))
    }
}
impl<'a> ::windows::IntoParam<'a, IDXGIDevice1> for &'a IDXGIDevice4 {
    fn into_param(self) -> ::windows::Param<'a, IDXGIDevice1> {
        ::windows::Param::Owned(::std::convert::Into::<IDXGIDevice1>::into(
            ::std::clone::Clone::clone(self),
        ))
    }
}
impl ::std::convert::From<IDXGIDevice4> for IDXGIDevice {
    fn from(value: IDXGIDevice4) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IDXGIDevice4> for IDXGIDevice {
    fn from(value: &IDXGIDevice4) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::IntoParam<'a, IDXGIDevice> for IDXGIDevice4 {
    fn into_param(self) -> ::windows::Param<'a, IDXGIDevice> {
        ::windows::Param::Owned(::std::convert::Into::<IDXGIDevice>::into(self))
    }
}
impl<'a> ::windows::IntoParam<'a, IDXGIDevice> for &'a IDXGIDevice4 {
    fn into_param(self) -> ::windows::Param<'a, IDXGIDevice> {
        ::windows::Param::Owned(::std::convert::Into::<IDXGIDevice>::into(
            ::std::clone::Clone::clone(self),
        ))
    }
}
impl ::std::convert::From<IDXGIDevice4> for IDXGIObject {
    fn from(value: IDXGIDevice4) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IDXGIDevice4> for IDXGIObject {
    fn from(value: &IDXGIDevice4) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::IntoParam<'a, IDXGIObject> for IDXGIDevice4 {
    fn into_param(self) -> ::windows::Param<'a, IDXGIObject> {
        ::windows::Param::Owned(::std::convert::Into::<IDXGIObject>::into(self))
    }
}
impl<'a> ::windows::IntoParam<'a, IDXGIObject> for &'a IDXGIDevice4 {
    fn into_param(self) -> ::windows::Param<'a, IDXGIObject> {
        ::windows::Param::Owned(::std::convert::Into::<IDXGIObject>::into(
            ::std::clone::Clone::clone(self),
        ))
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDXGIDevice4_abi(
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        iid: &::windows::Guid,
        interface: *mut ::windows::RawPtr,
    ) -> ::windows::ErrorCode,
    pub unsafe extern "system" fn(this: ::windows::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::RawPtr) -> u32,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        name: *const ::windows::Guid,
        data_size: u32,
        p_data: *const ::std::ffi::c_void,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        name: *const ::windows::Guid,
        p_unknown: ::windows::RawPtr,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        name: *const ::windows::Guid,
        p_data_size: *mut u32,
        p_data: *mut ::std::ffi::c_void,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        riid: *const ::windows::Guid,
        pp_parent: *mut *mut ::std::ffi::c_void,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        p_adapter: *mut ::windows::RawPtr,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        p_desc: *const DXGI_SURFACE_DESC,
        num_surfaces: u32,
        usage: u32,
        p_shared_resource: *const DXGI_SHARED_RESOURCE,
        pp_surface: *mut ::windows::RawPtr,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        pp_resources: *mut ::windows::RawPtr,
        p_residency_status: *mut DXGI_RESIDENCY,
        num_resources: u32,
    ) -> ::windows::ErrorCode,
    pub unsafe extern "system" fn(this: ::windows::RawPtr, priority: i32) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        p_priority: *mut i32,
    ) -> ::windows::ErrorCode,
    pub unsafe extern "system" fn(this: ::windows::RawPtr, max_latency: u32) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        p_max_latency: *mut u32,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        num_resources: u32,
        pp_resources: *mut ::windows::RawPtr,
        priority: DXGI_OFFER_RESOURCE_PRIORITY,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        num_resources: u32,
        pp_resources: *mut ::windows::RawPtr,
        p_discarded: *mut super::system_services::BOOL,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        h_event: super::system_services::HANDLE,
    ) -> ::windows::ErrorCode,
    pub unsafe extern "system" fn(this: ::windows::RawPtr),
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        num_resources: u32,
        pp_resources: *mut ::windows::RawPtr,
        priority: DXGI_OFFER_RESOURCE_PRIORITY,
        flags: u32,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        num_resources: u32,
        pp_resources: *mut ::windows::RawPtr,
        p_results: *mut DXGI_RECLAIM_RESOURCE_RESULTS,
    ) -> ::windows::ErrorCode,
);
#[repr(transparent)]
#[allow(non_camel_case_types)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct IDXGIDeviceSubObject(::windows::IUnknown);
impl IDXGIDeviceSubObject {}
unsafe impl ::windows::Interface for IDXGIDeviceSubObject {
    type Vtable = IDXGIDeviceSubObject_abi;
    const IID: ::windows::Guid = ::windows::Guid::from_values(
        1027474297,
        63966,
        19800,
        [187, 108, 24, 214, 41, 146, 241, 166],
    );
}
#[allow(non_snake_case)]
impl IDXGIDeviceSubObject {
    pub unsafe fn SetPrivateData(
        &self,
        name: *const ::windows::Guid,
        data_size: u32,
        p_data: *const ::std::ffi::c_void,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).3)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(name),
            ::std::mem::transmute(data_size),
            ::std::mem::transmute(p_data),
        )
    }
    pub unsafe fn SetPrivateDataInterface<
        'a,
        T1__: ::windows::IntoParam<'a, ::windows::IUnknown>,
    >(
        &self,
        name: *const ::windows::Guid,
        p_unknown: T1__,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).4)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(name),
            p_unknown.into_param().abi(),
        )
    }
    pub unsafe fn GetPrivateData(
        &self,
        name: *const ::windows::Guid,
        p_data_size: *mut u32,
        p_data: *mut ::std::ffi::c_void,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).5)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(name),
            ::std::mem::transmute(p_data_size),
            ::std::mem::transmute(p_data),
        )
    }
    pub unsafe fn GetParent(
        &self,
        riid: *const ::windows::Guid,
        pp_parent: *mut *mut ::std::ffi::c_void,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).6)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(riid),
            ::std::mem::transmute(pp_parent),
        )
    }
    pub unsafe fn GetDevice(
        &self,
        riid: *const ::windows::Guid,
        pp_device: *mut *mut ::std::ffi::c_void,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).7)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(riid),
            ::std::mem::transmute(pp_device),
        )
    }
}
impl ::std::convert::From<IDXGIDeviceSubObject> for ::windows::IUnknown {
    fn from(value: IDXGIDeviceSubObject) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IDXGIDeviceSubObject> for ::windows::IUnknown {
    fn from(value: &IDXGIDeviceSubObject) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::IntoParam<'a, ::windows::IUnknown> for IDXGIDeviceSubObject {
    fn into_param(self) -> ::windows::Param<'a, ::windows::IUnknown> {
        ::windows::Param::Owned(::std::convert::Into::<::windows::IUnknown>::into(self))
    }
}
impl<'a> ::windows::IntoParam<'a, ::windows::IUnknown> for &'a IDXGIDeviceSubObject {
    fn into_param(self) -> ::windows::Param<'a, ::windows::IUnknown> {
        ::windows::Param::Owned(::std::convert::Into::<::windows::IUnknown>::into(
            ::std::clone::Clone::clone(self),
        ))
    }
}
impl ::std::convert::From<IDXGIDeviceSubObject> for IDXGIObject {
    fn from(value: IDXGIDeviceSubObject) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IDXGIDeviceSubObject> for IDXGIObject {
    fn from(value: &IDXGIDeviceSubObject) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::IntoParam<'a, IDXGIObject> for IDXGIDeviceSubObject {
    fn into_param(self) -> ::windows::Param<'a, IDXGIObject> {
        ::windows::Param::Owned(::std::convert::Into::<IDXGIObject>::into(self))
    }
}
impl<'a> ::windows::IntoParam<'a, IDXGIObject> for &'a IDXGIDeviceSubObject {
    fn into_param(self) -> ::windows::Param<'a, IDXGIObject> {
        ::windows::Param::Owned(::std::convert::Into::<IDXGIObject>::into(
            ::std::clone::Clone::clone(self),
        ))
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDXGIDeviceSubObject_abi(
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        iid: &::windows::Guid,
        interface: *mut ::windows::RawPtr,
    ) -> ::windows::ErrorCode,
    pub unsafe extern "system" fn(this: ::windows::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::RawPtr) -> u32,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        name: *const ::windows::Guid,
        data_size: u32,
        p_data: *const ::std::ffi::c_void,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        name: *const ::windows::Guid,
        p_unknown: ::windows::RawPtr,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        name: *const ::windows::Guid,
        p_data_size: *mut u32,
        p_data: *mut ::std::ffi::c_void,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        riid: *const ::windows::Guid,
        pp_parent: *mut *mut ::std::ffi::c_void,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        riid: *const ::windows::Guid,
        pp_device: *mut *mut ::std::ffi::c_void,
    ) -> ::windows::ErrorCode,
);
#[repr(transparent)]
#[allow(non_camel_case_types)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct IDXGIDisplayControl(::windows::IUnknown);
impl IDXGIDisplayControl {}
unsafe impl ::windows::Interface for IDXGIDisplayControl {
    type Vtable = IDXGIDisplayControl_abi;
    const IID: ::windows::Guid = ::windows::Guid::from_values(
        3936206618,
        51342,
        17542,
        [133, 74, 152, 170, 1, 56, 243, 12],
    );
}
#[allow(non_snake_case)]
impl IDXGIDisplayControl {
    pub unsafe fn IsStereoEnabled(&self) -> super::system_services::BOOL {
        (::windows::Interface::vtable(self).3)(::windows::Abi::abi(self))
    }
    pub unsafe fn SetStereoEnabled<
        'a,
        T0__: ::windows::IntoParam<'a, super::system_services::BOOL>,
    >(
        &self,
        enabled: T0__,
    ) {
        (::windows::Interface::vtable(self).4)(
            ::windows::Abi::abi(self),
            enabled.into_param().abi(),
        )
    }
}
impl ::std::convert::From<IDXGIDisplayControl> for ::windows::IUnknown {
    fn from(value: IDXGIDisplayControl) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IDXGIDisplayControl> for ::windows::IUnknown {
    fn from(value: &IDXGIDisplayControl) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::IntoParam<'a, ::windows::IUnknown> for IDXGIDisplayControl {
    fn into_param(self) -> ::windows::Param<'a, ::windows::IUnknown> {
        ::windows::Param::Owned(::std::convert::Into::<::windows::IUnknown>::into(self))
    }
}
impl<'a> ::windows::IntoParam<'a, ::windows::IUnknown> for &'a IDXGIDisplayControl {
    fn into_param(self) -> ::windows::Param<'a, ::windows::IUnknown> {
        ::windows::Param::Owned(::std::convert::Into::<::windows::IUnknown>::into(
            ::std::clone::Clone::clone(self),
        ))
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDXGIDisplayControl_abi(
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        iid: &::windows::Guid,
        interface: *mut ::windows::RawPtr,
    ) -> ::windows::ErrorCode,
    pub unsafe extern "system" fn(this: ::windows::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::RawPtr) -> super::system_services::BOOL,
    pub unsafe extern "system" fn(this: ::windows::RawPtr, enabled: super::system_services::BOOL),
);
#[repr(transparent)]
#[allow(non_camel_case_types)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct IDXGIFactory(::windows::IUnknown);
impl IDXGIFactory {}
unsafe impl ::windows::Interface for IDXGIFactory {
    type Vtable = IDXGIFactory_abi;
    const IID: ::windows::Guid = ::windows::Guid::from_values(
        2071029484,
        8647,
        17582,
        [178, 26, 201, 174, 50, 26, 227, 105],
    );
}
#[allow(non_snake_case)]
impl IDXGIFactory {
    pub unsafe fn SetPrivateData(
        &self,
        name: *const ::windows::Guid,
        data_size: u32,
        p_data: *const ::std::ffi::c_void,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).3)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(name),
            ::std::mem::transmute(data_size),
            ::std::mem::transmute(p_data),
        )
    }
    pub unsafe fn SetPrivateDataInterface<
        'a,
        T1__: ::windows::IntoParam<'a, ::windows::IUnknown>,
    >(
        &self,
        name: *const ::windows::Guid,
        p_unknown: T1__,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).4)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(name),
            p_unknown.into_param().abi(),
        )
    }
    pub unsafe fn GetPrivateData(
        &self,
        name: *const ::windows::Guid,
        p_data_size: *mut u32,
        p_data: *mut ::std::ffi::c_void,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).5)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(name),
            ::std::mem::transmute(p_data_size),
            ::std::mem::transmute(p_data),
        )
    }
    pub unsafe fn GetParent(
        &self,
        riid: *const ::windows::Guid,
        pp_parent: *mut *mut ::std::ffi::c_void,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).6)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(riid),
            ::std::mem::transmute(pp_parent),
        )
    }
    pub unsafe fn EnumAdapters(
        &self,
        adapter: u32,
        pp_adapter: *mut ::std::option::Option<IDXGIAdapter>,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).7)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(adapter),
            ::std::mem::transmute(pp_adapter),
        )
    }
    pub unsafe fn MakeWindowAssociation<
        'a,
        T0__: ::windows::IntoParam<'a, super::windows_and_messaging::HWND>,
    >(
        &self,
        window_handle: T0__,
        flags: u32,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).8)(
            ::windows::Abi::abi(self),
            window_handle.into_param().abi(),
            ::std::mem::transmute(flags),
        )
    }
    pub unsafe fn GetWindowAssociation(
        &self,
        p_window_handle: *mut super::windows_and_messaging::HWND,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).9)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(p_window_handle),
        )
    }
    pub unsafe fn CreateSwapChain<'a, T0__: ::windows::IntoParam<'a, ::windows::IUnknown>>(
        &self,
        p_device: T0__,
        p_desc: *mut DXGI_SWAP_CHAIN_DESC,
        pp_swap_chain: *mut ::std::option::Option<IDXGISwapChain>,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).10)(
            ::windows::Abi::abi(self),
            p_device.into_param().abi(),
            ::std::mem::transmute(p_desc),
            ::std::mem::transmute(pp_swap_chain),
        )
    }
    pub unsafe fn CreateSoftwareAdapter(
        &self,
        module: isize,
        pp_adapter: *mut ::std::option::Option<IDXGIAdapter>,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).11)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(module),
            ::std::mem::transmute(pp_adapter),
        )
    }
}
impl ::std::convert::From<IDXGIFactory> for ::windows::IUnknown {
    fn from(value: IDXGIFactory) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IDXGIFactory> for ::windows::IUnknown {
    fn from(value: &IDXGIFactory) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::IntoParam<'a, ::windows::IUnknown> for IDXGIFactory {
    fn into_param(self) -> ::windows::Param<'a, ::windows::IUnknown> {
        ::windows::Param::Owned(::std::convert::Into::<::windows::IUnknown>::into(self))
    }
}
impl<'a> ::windows::IntoParam<'a, ::windows::IUnknown> for &'a IDXGIFactory {
    fn into_param(self) -> ::windows::Param<'a, ::windows::IUnknown> {
        ::windows::Param::Owned(::std::convert::Into::<::windows::IUnknown>::into(
            ::std::clone::Clone::clone(self),
        ))
    }
}
impl ::std::convert::From<IDXGIFactory> for IDXGIObject {
    fn from(value: IDXGIFactory) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IDXGIFactory> for IDXGIObject {
    fn from(value: &IDXGIFactory) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::IntoParam<'a, IDXGIObject> for IDXGIFactory {
    fn into_param(self) -> ::windows::Param<'a, IDXGIObject> {
        ::windows::Param::Owned(::std::convert::Into::<IDXGIObject>::into(self))
    }
}
impl<'a> ::windows::IntoParam<'a, IDXGIObject> for &'a IDXGIFactory {
    fn into_param(self) -> ::windows::Param<'a, IDXGIObject> {
        ::windows::Param::Owned(::std::convert::Into::<IDXGIObject>::into(
            ::std::clone::Clone::clone(self),
        ))
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDXGIFactory_abi(
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        iid: &::windows::Guid,
        interface: *mut ::windows::RawPtr,
    ) -> ::windows::ErrorCode,
    pub unsafe extern "system" fn(this: ::windows::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::RawPtr) -> u32,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        name: *const ::windows::Guid,
        data_size: u32,
        p_data: *const ::std::ffi::c_void,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        name: *const ::windows::Guid,
        p_unknown: ::windows::RawPtr,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        name: *const ::windows::Guid,
        p_data_size: *mut u32,
        p_data: *mut ::std::ffi::c_void,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        riid: *const ::windows::Guid,
        pp_parent: *mut *mut ::std::ffi::c_void,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        adapter: u32,
        pp_adapter: *mut ::windows::RawPtr,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        window_handle: super::windows_and_messaging::HWND,
        flags: u32,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        p_window_handle: *mut super::windows_and_messaging::HWND,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        p_device: ::windows::RawPtr,
        p_desc: *mut DXGI_SWAP_CHAIN_DESC,
        pp_swap_chain: *mut ::windows::RawPtr,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        module: isize,
        pp_adapter: *mut ::windows::RawPtr,
    ) -> ::windows::ErrorCode,
);
#[repr(transparent)]
#[allow(non_camel_case_types)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct IDXGIFactory1(::windows::IUnknown);
impl IDXGIFactory1 {}
unsafe impl ::windows::Interface for IDXGIFactory1 {
    type Vtable = IDXGIFactory1_abi;
    const IID: ::windows::Guid = ::windows::Guid::from_values(
        1997188728,
        62063,
        19898,
        [168, 41, 37, 60, 131, 209, 179, 135],
    );
}
#[allow(non_snake_case)]
impl IDXGIFactory1 {
    pub unsafe fn SetPrivateData(
        &self,
        name: *const ::windows::Guid,
        data_size: u32,
        p_data: *const ::std::ffi::c_void,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).3)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(name),
            ::std::mem::transmute(data_size),
            ::std::mem::transmute(p_data),
        )
    }
    pub unsafe fn SetPrivateDataInterface<
        'a,
        T1__: ::windows::IntoParam<'a, ::windows::IUnknown>,
    >(
        &self,
        name: *const ::windows::Guid,
        p_unknown: T1__,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).4)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(name),
            p_unknown.into_param().abi(),
        )
    }
    pub unsafe fn GetPrivateData(
        &self,
        name: *const ::windows::Guid,
        p_data_size: *mut u32,
        p_data: *mut ::std::ffi::c_void,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).5)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(name),
            ::std::mem::transmute(p_data_size),
            ::std::mem::transmute(p_data),
        )
    }
    pub unsafe fn GetParent(
        &self,
        riid: *const ::windows::Guid,
        pp_parent: *mut *mut ::std::ffi::c_void,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).6)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(riid),
            ::std::mem::transmute(pp_parent),
        )
    }
    pub unsafe fn EnumAdapters(
        &self,
        adapter: u32,
        pp_adapter: *mut ::std::option::Option<IDXGIAdapter>,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).7)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(adapter),
            ::std::mem::transmute(pp_adapter),
        )
    }
    pub unsafe fn MakeWindowAssociation<
        'a,
        T0__: ::windows::IntoParam<'a, super::windows_and_messaging::HWND>,
    >(
        &self,
        window_handle: T0__,
        flags: u32,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).8)(
            ::windows::Abi::abi(self),
            window_handle.into_param().abi(),
            ::std::mem::transmute(flags),
        )
    }
    pub unsafe fn GetWindowAssociation(
        &self,
        p_window_handle: *mut super::windows_and_messaging::HWND,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).9)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(p_window_handle),
        )
    }
    pub unsafe fn CreateSwapChain<'a, T0__: ::windows::IntoParam<'a, ::windows::IUnknown>>(
        &self,
        p_device: T0__,
        p_desc: *mut DXGI_SWAP_CHAIN_DESC,
        pp_swap_chain: *mut ::std::option::Option<IDXGISwapChain>,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).10)(
            ::windows::Abi::abi(self),
            p_device.into_param().abi(),
            ::std::mem::transmute(p_desc),
            ::std::mem::transmute(pp_swap_chain),
        )
    }
    pub unsafe fn CreateSoftwareAdapter(
        &self,
        module: isize,
        pp_adapter: *mut ::std::option::Option<IDXGIAdapter>,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).11)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(module),
            ::std::mem::transmute(pp_adapter),
        )
    }
    pub unsafe fn EnumAdapters1(
        &self,
        adapter: u32,
        pp_adapter: *mut ::std::option::Option<IDXGIAdapter1>,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).12)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(adapter),
            ::std::mem::transmute(pp_adapter),
        )
    }
    pub unsafe fn IsCurrent(&self) -> super::system_services::BOOL {
        (::windows::Interface::vtable(self).13)(::windows::Abi::abi(self))
    }
}
impl ::std::convert::From<IDXGIFactory1> for ::windows::IUnknown {
    fn from(value: IDXGIFactory1) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IDXGIFactory1> for ::windows::IUnknown {
    fn from(value: &IDXGIFactory1) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::IntoParam<'a, ::windows::IUnknown> for IDXGIFactory1 {
    fn into_param(self) -> ::windows::Param<'a, ::windows::IUnknown> {
        ::windows::Param::Owned(::std::convert::Into::<::windows::IUnknown>::into(self))
    }
}
impl<'a> ::windows::IntoParam<'a, ::windows::IUnknown> for &'a IDXGIFactory1 {
    fn into_param(self) -> ::windows::Param<'a, ::windows::IUnknown> {
        ::windows::Param::Owned(::std::convert::Into::<::windows::IUnknown>::into(
            ::std::clone::Clone::clone(self),
        ))
    }
}
impl ::std::convert::From<IDXGIFactory1> for IDXGIFactory {
    fn from(value: IDXGIFactory1) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IDXGIFactory1> for IDXGIFactory {
    fn from(value: &IDXGIFactory1) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::IntoParam<'a, IDXGIFactory> for IDXGIFactory1 {
    fn into_param(self) -> ::windows::Param<'a, IDXGIFactory> {
        ::windows::Param::Owned(::std::convert::Into::<IDXGIFactory>::into(self))
    }
}
impl<'a> ::windows::IntoParam<'a, IDXGIFactory> for &'a IDXGIFactory1 {
    fn into_param(self) -> ::windows::Param<'a, IDXGIFactory> {
        ::windows::Param::Owned(::std::convert::Into::<IDXGIFactory>::into(
            ::std::clone::Clone::clone(self),
        ))
    }
}
impl ::std::convert::From<IDXGIFactory1> for IDXGIObject {
    fn from(value: IDXGIFactory1) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IDXGIFactory1> for IDXGIObject {
    fn from(value: &IDXGIFactory1) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::IntoParam<'a, IDXGIObject> for IDXGIFactory1 {
    fn into_param(self) -> ::windows::Param<'a, IDXGIObject> {
        ::windows::Param::Owned(::std::convert::Into::<IDXGIObject>::into(self))
    }
}
impl<'a> ::windows::IntoParam<'a, IDXGIObject> for &'a IDXGIFactory1 {
    fn into_param(self) -> ::windows::Param<'a, IDXGIObject> {
        ::windows::Param::Owned(::std::convert::Into::<IDXGIObject>::into(
            ::std::clone::Clone::clone(self),
        ))
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDXGIFactory1_abi(
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        iid: &::windows::Guid,
        interface: *mut ::windows::RawPtr,
    ) -> ::windows::ErrorCode,
    pub unsafe extern "system" fn(this: ::windows::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::RawPtr) -> u32,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        name: *const ::windows::Guid,
        data_size: u32,
        p_data: *const ::std::ffi::c_void,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        name: *const ::windows::Guid,
        p_unknown: ::windows::RawPtr,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        name: *const ::windows::Guid,
        p_data_size: *mut u32,
        p_data: *mut ::std::ffi::c_void,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        riid: *const ::windows::Guid,
        pp_parent: *mut *mut ::std::ffi::c_void,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        adapter: u32,
        pp_adapter: *mut ::windows::RawPtr,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        window_handle: super::windows_and_messaging::HWND,
        flags: u32,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        p_window_handle: *mut super::windows_and_messaging::HWND,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        p_device: ::windows::RawPtr,
        p_desc: *mut DXGI_SWAP_CHAIN_DESC,
        pp_swap_chain: *mut ::windows::RawPtr,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        module: isize,
        pp_adapter: *mut ::windows::RawPtr,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        adapter: u32,
        pp_adapter: *mut ::windows::RawPtr,
    ) -> ::windows::ErrorCode,
    pub unsafe extern "system" fn(this: ::windows::RawPtr) -> super::system_services::BOOL,
);
#[repr(transparent)]
#[allow(non_camel_case_types)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct IDXGIFactory2(::windows::IUnknown);
impl IDXGIFactory2 {}
unsafe impl ::windows::Interface for IDXGIFactory2 {
    type Vtable = IDXGIFactory2_abi;
    const IID: ::windows::Guid = ::windows::Guid::from_values(
        1355299356,
        57458,
        19528,
        [135, 176, 54, 48, 250, 54, 166, 208],
    );
}
#[allow(non_snake_case)]
impl IDXGIFactory2 {
    pub unsafe fn SetPrivateData(
        &self,
        name: *const ::windows::Guid,
        data_size: u32,
        p_data: *const ::std::ffi::c_void,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).3)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(name),
            ::std::mem::transmute(data_size),
            ::std::mem::transmute(p_data),
        )
    }
    pub unsafe fn SetPrivateDataInterface<
        'a,
        T1__: ::windows::IntoParam<'a, ::windows::IUnknown>,
    >(
        &self,
        name: *const ::windows::Guid,
        p_unknown: T1__,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).4)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(name),
            p_unknown.into_param().abi(),
        )
    }
    pub unsafe fn GetPrivateData(
        &self,
        name: *const ::windows::Guid,
        p_data_size: *mut u32,
        p_data: *mut ::std::ffi::c_void,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).5)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(name),
            ::std::mem::transmute(p_data_size),
            ::std::mem::transmute(p_data),
        )
    }
    pub unsafe fn GetParent(
        &self,
        riid: *const ::windows::Guid,
        pp_parent: *mut *mut ::std::ffi::c_void,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).6)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(riid),
            ::std::mem::transmute(pp_parent),
        )
    }
    pub unsafe fn EnumAdapters(
        &self,
        adapter: u32,
        pp_adapter: *mut ::std::option::Option<IDXGIAdapter>,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).7)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(adapter),
            ::std::mem::transmute(pp_adapter),
        )
    }
    pub unsafe fn MakeWindowAssociation<
        'a,
        T0__: ::windows::IntoParam<'a, super::windows_and_messaging::HWND>,
    >(
        &self,
        window_handle: T0__,
        flags: u32,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).8)(
            ::windows::Abi::abi(self),
            window_handle.into_param().abi(),
            ::std::mem::transmute(flags),
        )
    }
    pub unsafe fn GetWindowAssociation(
        &self,
        p_window_handle: *mut super::windows_and_messaging::HWND,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).9)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(p_window_handle),
        )
    }
    pub unsafe fn CreateSwapChain<'a, T0__: ::windows::IntoParam<'a, ::windows::IUnknown>>(
        &self,
        p_device: T0__,
        p_desc: *mut DXGI_SWAP_CHAIN_DESC,
        pp_swap_chain: *mut ::std::option::Option<IDXGISwapChain>,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).10)(
            ::windows::Abi::abi(self),
            p_device.into_param().abi(),
            ::std::mem::transmute(p_desc),
            ::std::mem::transmute(pp_swap_chain),
        )
    }
    pub unsafe fn CreateSoftwareAdapter(
        &self,
        module: isize,
        pp_adapter: *mut ::std::option::Option<IDXGIAdapter>,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).11)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(module),
            ::std::mem::transmute(pp_adapter),
        )
    }
    pub unsafe fn EnumAdapters1(
        &self,
        adapter: u32,
        pp_adapter: *mut ::std::option::Option<IDXGIAdapter1>,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).12)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(adapter),
            ::std::mem::transmute(pp_adapter),
        )
    }
    pub unsafe fn IsCurrent(&self) -> super::system_services::BOOL {
        (::windows::Interface::vtable(self).13)(::windows::Abi::abi(self))
    }
    pub unsafe fn IsWindowedStereoEnabled(&self) -> super::system_services::BOOL {
        (::windows::Interface::vtable(self).14)(::windows::Abi::abi(self))
    }
    pub unsafe fn CreateSwapChainForHwnd<
        'a,
        T0__: ::windows::IntoParam<'a, ::windows::IUnknown>,
        T1__: ::windows::IntoParam<'a, super::windows_and_messaging::HWND>,
        T4__: ::windows::IntoParam<'a, IDXGIOutput>,
    >(
        &self,
        p_device: T0__,
        h_wnd: T1__,
        p_desc: *const DXGI_SWAP_CHAIN_DESC1,
        p_fullscreen_desc: *const DXGI_SWAP_CHAIN_FULLSCREEN_DESC,
        p_restrict_to_output: T4__,
        pp_swap_chain: *mut ::std::option::Option<IDXGISwapChain1>,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).15)(
            ::windows::Abi::abi(self),
            p_device.into_param().abi(),
            h_wnd.into_param().abi(),
            ::std::mem::transmute(p_desc),
            ::std::mem::transmute(p_fullscreen_desc),
            p_restrict_to_output.into_param().abi(),
            ::std::mem::transmute(pp_swap_chain),
        )
    }
    pub unsafe fn CreateSwapChainForCoreWindow<
        'a,
        T0__: ::windows::IntoParam<'a, ::windows::IUnknown>,
        T1__: ::windows::IntoParam<'a, ::windows::IUnknown>,
        T3__: ::windows::IntoParam<'a, IDXGIOutput>,
    >(
        &self,
        p_device: T0__,
        p_window: T1__,
        p_desc: *const DXGI_SWAP_CHAIN_DESC1,
        p_restrict_to_output: T3__,
        pp_swap_chain: *mut ::std::option::Option<IDXGISwapChain1>,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).16)(
            ::windows::Abi::abi(self),
            p_device.into_param().abi(),
            p_window.into_param().abi(),
            ::std::mem::transmute(p_desc),
            p_restrict_to_output.into_param().abi(),
            ::std::mem::transmute(pp_swap_chain),
        )
    }
    pub unsafe fn GetSharedResourceAdapterLuid<
        'a,
        T0__: ::windows::IntoParam<'a, super::system_services::HANDLE>,
    >(
        &self,
        h_resource: T0__,
        p_luid: *mut super::kernel::LUID,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).17)(
            ::windows::Abi::abi(self),
            h_resource.into_param().abi(),
            ::std::mem::transmute(p_luid),
        )
    }
    pub unsafe fn RegisterStereoStatusWindow<
        'a,
        T0__: ::windows::IntoParam<'a, super::windows_and_messaging::HWND>,
    >(
        &self,
        window_handle: T0__,
        w_msg: u32,
        pdw_cookie: *mut u32,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).18)(
            ::windows::Abi::abi(self),
            window_handle.into_param().abi(),
            ::std::mem::transmute(w_msg),
            ::std::mem::transmute(pdw_cookie),
        )
    }
    pub unsafe fn RegisterStereoStatusEvent<
        'a,
        T0__: ::windows::IntoParam<'a, super::system_services::HANDLE>,
    >(
        &self,
        h_event: T0__,
        pdw_cookie: *mut u32,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).19)(
            ::windows::Abi::abi(self),
            h_event.into_param().abi(),
            ::std::mem::transmute(pdw_cookie),
        )
    }
    pub unsafe fn UnregisterStereoStatus(&self, dw_cookie: u32) {
        (::windows::Interface::vtable(self).20)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(dw_cookie),
        )
    }
    pub unsafe fn RegisterOcclusionStatusWindow<
        'a,
        T0__: ::windows::IntoParam<'a, super::windows_and_messaging::HWND>,
    >(
        &self,
        window_handle: T0__,
        w_msg: u32,
        pdw_cookie: *mut u32,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).21)(
            ::windows::Abi::abi(self),
            window_handle.into_param().abi(),
            ::std::mem::transmute(w_msg),
            ::std::mem::transmute(pdw_cookie),
        )
    }
    pub unsafe fn RegisterOcclusionStatusEvent<
        'a,
        T0__: ::windows::IntoParam<'a, super::system_services::HANDLE>,
    >(
        &self,
        h_event: T0__,
        pdw_cookie: *mut u32,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).22)(
            ::windows::Abi::abi(self),
            h_event.into_param().abi(),
            ::std::mem::transmute(pdw_cookie),
        )
    }
    pub unsafe fn UnregisterOcclusionStatus(&self, dw_cookie: u32) {
        (::windows::Interface::vtable(self).23)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(dw_cookie),
        )
    }
    pub unsafe fn CreateSwapChainForComposition<
        'a,
        T0__: ::windows::IntoParam<'a, ::windows::IUnknown>,
        T2__: ::windows::IntoParam<'a, IDXGIOutput>,
    >(
        &self,
        p_device: T0__,
        p_desc: *const DXGI_SWAP_CHAIN_DESC1,
        p_restrict_to_output: T2__,
        pp_swap_chain: *mut ::std::option::Option<IDXGISwapChain1>,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).24)(
            ::windows::Abi::abi(self),
            p_device.into_param().abi(),
            ::std::mem::transmute(p_desc),
            p_restrict_to_output.into_param().abi(),
            ::std::mem::transmute(pp_swap_chain),
        )
    }
}
impl ::std::convert::From<IDXGIFactory2> for ::windows::IUnknown {
    fn from(value: IDXGIFactory2) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IDXGIFactory2> for ::windows::IUnknown {
    fn from(value: &IDXGIFactory2) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::IntoParam<'a, ::windows::IUnknown> for IDXGIFactory2 {
    fn into_param(self) -> ::windows::Param<'a, ::windows::IUnknown> {
        ::windows::Param::Owned(::std::convert::Into::<::windows::IUnknown>::into(self))
    }
}
impl<'a> ::windows::IntoParam<'a, ::windows::IUnknown> for &'a IDXGIFactory2 {
    fn into_param(self) -> ::windows::Param<'a, ::windows::IUnknown> {
        ::windows::Param::Owned(::std::convert::Into::<::windows::IUnknown>::into(
            ::std::clone::Clone::clone(self),
        ))
    }
}
impl ::std::convert::From<IDXGIFactory2> for IDXGIFactory1 {
    fn from(value: IDXGIFactory2) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IDXGIFactory2> for IDXGIFactory1 {
    fn from(value: &IDXGIFactory2) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::IntoParam<'a, IDXGIFactory1> for IDXGIFactory2 {
    fn into_param(self) -> ::windows::Param<'a, IDXGIFactory1> {
        ::windows::Param::Owned(::std::convert::Into::<IDXGIFactory1>::into(self))
    }
}
impl<'a> ::windows::IntoParam<'a, IDXGIFactory1> for &'a IDXGIFactory2 {
    fn into_param(self) -> ::windows::Param<'a, IDXGIFactory1> {
        ::windows::Param::Owned(::std::convert::Into::<IDXGIFactory1>::into(
            ::std::clone::Clone::clone(self),
        ))
    }
}
impl ::std::convert::From<IDXGIFactory2> for IDXGIFactory {
    fn from(value: IDXGIFactory2) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IDXGIFactory2> for IDXGIFactory {
    fn from(value: &IDXGIFactory2) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::IntoParam<'a, IDXGIFactory> for IDXGIFactory2 {
    fn into_param(self) -> ::windows::Param<'a, IDXGIFactory> {
        ::windows::Param::Owned(::std::convert::Into::<IDXGIFactory>::into(self))
    }
}
impl<'a> ::windows::IntoParam<'a, IDXGIFactory> for &'a IDXGIFactory2 {
    fn into_param(self) -> ::windows::Param<'a, IDXGIFactory> {
        ::windows::Param::Owned(::std::convert::Into::<IDXGIFactory>::into(
            ::std::clone::Clone::clone(self),
        ))
    }
}
impl ::std::convert::From<IDXGIFactory2> for IDXGIObject {
    fn from(value: IDXGIFactory2) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IDXGIFactory2> for IDXGIObject {
    fn from(value: &IDXGIFactory2) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::IntoParam<'a, IDXGIObject> for IDXGIFactory2 {
    fn into_param(self) -> ::windows::Param<'a, IDXGIObject> {
        ::windows::Param::Owned(::std::convert::Into::<IDXGIObject>::into(self))
    }
}
impl<'a> ::windows::IntoParam<'a, IDXGIObject> for &'a IDXGIFactory2 {
    fn into_param(self) -> ::windows::Param<'a, IDXGIObject> {
        ::windows::Param::Owned(::std::convert::Into::<IDXGIObject>::into(
            ::std::clone::Clone::clone(self),
        ))
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDXGIFactory2_abi(
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        iid: &::windows::Guid,
        interface: *mut ::windows::RawPtr,
    ) -> ::windows::ErrorCode,
    pub unsafe extern "system" fn(this: ::windows::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::RawPtr) -> u32,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        name: *const ::windows::Guid,
        data_size: u32,
        p_data: *const ::std::ffi::c_void,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        name: *const ::windows::Guid,
        p_unknown: ::windows::RawPtr,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        name: *const ::windows::Guid,
        p_data_size: *mut u32,
        p_data: *mut ::std::ffi::c_void,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        riid: *const ::windows::Guid,
        pp_parent: *mut *mut ::std::ffi::c_void,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        adapter: u32,
        pp_adapter: *mut ::windows::RawPtr,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        window_handle: super::windows_and_messaging::HWND,
        flags: u32,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        p_window_handle: *mut super::windows_and_messaging::HWND,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        p_device: ::windows::RawPtr,
        p_desc: *mut DXGI_SWAP_CHAIN_DESC,
        pp_swap_chain: *mut ::windows::RawPtr,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        module: isize,
        pp_adapter: *mut ::windows::RawPtr,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        adapter: u32,
        pp_adapter: *mut ::windows::RawPtr,
    ) -> ::windows::ErrorCode,
    pub unsafe extern "system" fn(this: ::windows::RawPtr) -> super::system_services::BOOL,
    pub unsafe extern "system" fn(this: ::windows::RawPtr) -> super::system_services::BOOL,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        p_device: ::windows::RawPtr,
        h_wnd: super::windows_and_messaging::HWND,
        p_desc: *const DXGI_SWAP_CHAIN_DESC1,
        p_fullscreen_desc: *const DXGI_SWAP_CHAIN_FULLSCREEN_DESC,
        p_restrict_to_output: ::windows::RawPtr,
        pp_swap_chain: *mut ::windows::RawPtr,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        p_device: ::windows::RawPtr,
        p_window: ::windows::RawPtr,
        p_desc: *const DXGI_SWAP_CHAIN_DESC1,
        p_restrict_to_output: ::windows::RawPtr,
        pp_swap_chain: *mut ::windows::RawPtr,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        h_resource: super::system_services::HANDLE,
        p_luid: *mut super::kernel::LUID,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        window_handle: super::windows_and_messaging::HWND,
        w_msg: u32,
        pdw_cookie: *mut u32,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        h_event: super::system_services::HANDLE,
        pdw_cookie: *mut u32,
    ) -> ::windows::ErrorCode,
    pub unsafe extern "system" fn(this: ::windows::RawPtr, dw_cookie: u32),
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        window_handle: super::windows_and_messaging::HWND,
        w_msg: u32,
        pdw_cookie: *mut u32,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        h_event: super::system_services::HANDLE,
        pdw_cookie: *mut u32,
    ) -> ::windows::ErrorCode,
    pub unsafe extern "system" fn(this: ::windows::RawPtr, dw_cookie: u32),
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        p_device: ::windows::RawPtr,
        p_desc: *const DXGI_SWAP_CHAIN_DESC1,
        p_restrict_to_output: ::windows::RawPtr,
        pp_swap_chain: *mut ::windows::RawPtr,
    ) -> ::windows::ErrorCode,
);
#[repr(transparent)]
#[allow(non_camel_case_types)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct IDXGIFactory3(::windows::IUnknown);
impl IDXGIFactory3 {}
unsafe impl ::windows::Interface for IDXGIFactory3 {
    type Vtable = IDXGIFactory3_abi;
    const IID: ::windows::Guid = ::windows::Guid::from_values(
        625489955,
        52550,
        19581,
        [134, 202, 71, 170, 149, 184, 55, 189],
    );
}
#[allow(non_snake_case)]
impl IDXGIFactory3 {
    pub unsafe fn SetPrivateData(
        &self,
        name: *const ::windows::Guid,
        data_size: u32,
        p_data: *const ::std::ffi::c_void,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).3)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(name),
            ::std::mem::transmute(data_size),
            ::std::mem::transmute(p_data),
        )
    }
    pub unsafe fn SetPrivateDataInterface<
        'a,
        T1__: ::windows::IntoParam<'a, ::windows::IUnknown>,
    >(
        &self,
        name: *const ::windows::Guid,
        p_unknown: T1__,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).4)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(name),
            p_unknown.into_param().abi(),
        )
    }
    pub unsafe fn GetPrivateData(
        &self,
        name: *const ::windows::Guid,
        p_data_size: *mut u32,
        p_data: *mut ::std::ffi::c_void,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).5)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(name),
            ::std::mem::transmute(p_data_size),
            ::std::mem::transmute(p_data),
        )
    }
    pub unsafe fn GetParent(
        &self,
        riid: *const ::windows::Guid,
        pp_parent: *mut *mut ::std::ffi::c_void,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).6)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(riid),
            ::std::mem::transmute(pp_parent),
        )
    }
    pub unsafe fn EnumAdapters(
        &self,
        adapter: u32,
        pp_adapter: *mut ::std::option::Option<IDXGIAdapter>,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).7)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(adapter),
            ::std::mem::transmute(pp_adapter),
        )
    }
    pub unsafe fn MakeWindowAssociation<
        'a,
        T0__: ::windows::IntoParam<'a, super::windows_and_messaging::HWND>,
    >(
        &self,
        window_handle: T0__,
        flags: u32,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).8)(
            ::windows::Abi::abi(self),
            window_handle.into_param().abi(),
            ::std::mem::transmute(flags),
        )
    }
    pub unsafe fn GetWindowAssociation(
        &self,
        p_window_handle: *mut super::windows_and_messaging::HWND,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).9)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(p_window_handle),
        )
    }
    pub unsafe fn CreateSwapChain<'a, T0__: ::windows::IntoParam<'a, ::windows::IUnknown>>(
        &self,
        p_device: T0__,
        p_desc: *mut DXGI_SWAP_CHAIN_DESC,
        pp_swap_chain: *mut ::std::option::Option<IDXGISwapChain>,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).10)(
            ::windows::Abi::abi(self),
            p_device.into_param().abi(),
            ::std::mem::transmute(p_desc),
            ::std::mem::transmute(pp_swap_chain),
        )
    }
    pub unsafe fn CreateSoftwareAdapter(
        &self,
        module: isize,
        pp_adapter: *mut ::std::option::Option<IDXGIAdapter>,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).11)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(module),
            ::std::mem::transmute(pp_adapter),
        )
    }
    pub unsafe fn EnumAdapters1(
        &self,
        adapter: u32,
        pp_adapter: *mut ::std::option::Option<IDXGIAdapter1>,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).12)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(adapter),
            ::std::mem::transmute(pp_adapter),
        )
    }
    pub unsafe fn IsCurrent(&self) -> super::system_services::BOOL {
        (::windows::Interface::vtable(self).13)(::windows::Abi::abi(self))
    }
    pub unsafe fn IsWindowedStereoEnabled(&self) -> super::system_services::BOOL {
        (::windows::Interface::vtable(self).14)(::windows::Abi::abi(self))
    }
    pub unsafe fn CreateSwapChainForHwnd<
        'a,
        T0__: ::windows::IntoParam<'a, ::windows::IUnknown>,
        T1__: ::windows::IntoParam<'a, super::windows_and_messaging::HWND>,
        T4__: ::windows::IntoParam<'a, IDXGIOutput>,
    >(
        &self,
        p_device: T0__,
        h_wnd: T1__,
        p_desc: *const DXGI_SWAP_CHAIN_DESC1,
        p_fullscreen_desc: *const DXGI_SWAP_CHAIN_FULLSCREEN_DESC,
        p_restrict_to_output: T4__,
        pp_swap_chain: *mut ::std::option::Option<IDXGISwapChain1>,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).15)(
            ::windows::Abi::abi(self),
            p_device.into_param().abi(),
            h_wnd.into_param().abi(),
            ::std::mem::transmute(p_desc),
            ::std::mem::transmute(p_fullscreen_desc),
            p_restrict_to_output.into_param().abi(),
            ::std::mem::transmute(pp_swap_chain),
        )
    }
    pub unsafe fn CreateSwapChainForCoreWindow<
        'a,
        T0__: ::windows::IntoParam<'a, ::windows::IUnknown>,
        T1__: ::windows::IntoParam<'a, ::windows::IUnknown>,
        T3__: ::windows::IntoParam<'a, IDXGIOutput>,
    >(
        &self,
        p_device: T0__,
        p_window: T1__,
        p_desc: *const DXGI_SWAP_CHAIN_DESC1,
        p_restrict_to_output: T3__,
        pp_swap_chain: *mut ::std::option::Option<IDXGISwapChain1>,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).16)(
            ::windows::Abi::abi(self),
            p_device.into_param().abi(),
            p_window.into_param().abi(),
            ::std::mem::transmute(p_desc),
            p_restrict_to_output.into_param().abi(),
            ::std::mem::transmute(pp_swap_chain),
        )
    }
    pub unsafe fn GetSharedResourceAdapterLuid<
        'a,
        T0__: ::windows::IntoParam<'a, super::system_services::HANDLE>,
    >(
        &self,
        h_resource: T0__,
        p_luid: *mut super::kernel::LUID,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).17)(
            ::windows::Abi::abi(self),
            h_resource.into_param().abi(),
            ::std::mem::transmute(p_luid),
        )
    }
    pub unsafe fn RegisterStereoStatusWindow<
        'a,
        T0__: ::windows::IntoParam<'a, super::windows_and_messaging::HWND>,
    >(
        &self,
        window_handle: T0__,
        w_msg: u32,
        pdw_cookie: *mut u32,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).18)(
            ::windows::Abi::abi(self),
            window_handle.into_param().abi(),
            ::std::mem::transmute(w_msg),
            ::std::mem::transmute(pdw_cookie),
        )
    }
    pub unsafe fn RegisterStereoStatusEvent<
        'a,
        T0__: ::windows::IntoParam<'a, super::system_services::HANDLE>,
    >(
        &self,
        h_event: T0__,
        pdw_cookie: *mut u32,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).19)(
            ::windows::Abi::abi(self),
            h_event.into_param().abi(),
            ::std::mem::transmute(pdw_cookie),
        )
    }
    pub unsafe fn UnregisterStereoStatus(&self, dw_cookie: u32) {
        (::windows::Interface::vtable(self).20)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(dw_cookie),
        )
    }
    pub unsafe fn RegisterOcclusionStatusWindow<
        'a,
        T0__: ::windows::IntoParam<'a, super::windows_and_messaging::HWND>,
    >(
        &self,
        window_handle: T0__,
        w_msg: u32,
        pdw_cookie: *mut u32,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).21)(
            ::windows::Abi::abi(self),
            window_handle.into_param().abi(),
            ::std::mem::transmute(w_msg),
            ::std::mem::transmute(pdw_cookie),
        )
    }
    pub unsafe fn RegisterOcclusionStatusEvent<
        'a,
        T0__: ::windows::IntoParam<'a, super::system_services::HANDLE>,
    >(
        &self,
        h_event: T0__,
        pdw_cookie: *mut u32,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).22)(
            ::windows::Abi::abi(self),
            h_event.into_param().abi(),
            ::std::mem::transmute(pdw_cookie),
        )
    }
    pub unsafe fn UnregisterOcclusionStatus(&self, dw_cookie: u32) {
        (::windows::Interface::vtable(self).23)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(dw_cookie),
        )
    }
    pub unsafe fn CreateSwapChainForComposition<
        'a,
        T0__: ::windows::IntoParam<'a, ::windows::IUnknown>,
        T2__: ::windows::IntoParam<'a, IDXGIOutput>,
    >(
        &self,
        p_device: T0__,
        p_desc: *const DXGI_SWAP_CHAIN_DESC1,
        p_restrict_to_output: T2__,
        pp_swap_chain: *mut ::std::option::Option<IDXGISwapChain1>,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).24)(
            ::windows::Abi::abi(self),
            p_device.into_param().abi(),
            ::std::mem::transmute(p_desc),
            p_restrict_to_output.into_param().abi(),
            ::std::mem::transmute(pp_swap_chain),
        )
    }
    pub unsafe fn GetCreationFlags(&self) -> u32 {
        (::windows::Interface::vtable(self).25)(::windows::Abi::abi(self))
    }
}
impl ::std::convert::From<IDXGIFactory3> for ::windows::IUnknown {
    fn from(value: IDXGIFactory3) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IDXGIFactory3> for ::windows::IUnknown {
    fn from(value: &IDXGIFactory3) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::IntoParam<'a, ::windows::IUnknown> for IDXGIFactory3 {
    fn into_param(self) -> ::windows::Param<'a, ::windows::IUnknown> {
        ::windows::Param::Owned(::std::convert::Into::<::windows::IUnknown>::into(self))
    }
}
impl<'a> ::windows::IntoParam<'a, ::windows::IUnknown> for &'a IDXGIFactory3 {
    fn into_param(self) -> ::windows::Param<'a, ::windows::IUnknown> {
        ::windows::Param::Owned(::std::convert::Into::<::windows::IUnknown>::into(
            ::std::clone::Clone::clone(self),
        ))
    }
}
impl ::std::convert::From<IDXGIFactory3> for IDXGIFactory2 {
    fn from(value: IDXGIFactory3) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IDXGIFactory3> for IDXGIFactory2 {
    fn from(value: &IDXGIFactory3) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::IntoParam<'a, IDXGIFactory2> for IDXGIFactory3 {
    fn into_param(self) -> ::windows::Param<'a, IDXGIFactory2> {
        ::windows::Param::Owned(::std::convert::Into::<IDXGIFactory2>::into(self))
    }
}
impl<'a> ::windows::IntoParam<'a, IDXGIFactory2> for &'a IDXGIFactory3 {
    fn into_param(self) -> ::windows::Param<'a, IDXGIFactory2> {
        ::windows::Param::Owned(::std::convert::Into::<IDXGIFactory2>::into(
            ::std::clone::Clone::clone(self),
        ))
    }
}
impl ::std::convert::From<IDXGIFactory3> for IDXGIFactory1 {
    fn from(value: IDXGIFactory3) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IDXGIFactory3> for IDXGIFactory1 {
    fn from(value: &IDXGIFactory3) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::IntoParam<'a, IDXGIFactory1> for IDXGIFactory3 {
    fn into_param(self) -> ::windows::Param<'a, IDXGIFactory1> {
        ::windows::Param::Owned(::std::convert::Into::<IDXGIFactory1>::into(self))
    }
}
impl<'a> ::windows::IntoParam<'a, IDXGIFactory1> for &'a IDXGIFactory3 {
    fn into_param(self) -> ::windows::Param<'a, IDXGIFactory1> {
        ::windows::Param::Owned(::std::convert::Into::<IDXGIFactory1>::into(
            ::std::clone::Clone::clone(self),
        ))
    }
}
impl ::std::convert::From<IDXGIFactory3> for IDXGIFactory {
    fn from(value: IDXGIFactory3) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IDXGIFactory3> for IDXGIFactory {
    fn from(value: &IDXGIFactory3) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::IntoParam<'a, IDXGIFactory> for IDXGIFactory3 {
    fn into_param(self) -> ::windows::Param<'a, IDXGIFactory> {
        ::windows::Param::Owned(::std::convert::Into::<IDXGIFactory>::into(self))
    }
}
impl<'a> ::windows::IntoParam<'a, IDXGIFactory> for &'a IDXGIFactory3 {
    fn into_param(self) -> ::windows::Param<'a, IDXGIFactory> {
        ::windows::Param::Owned(::std::convert::Into::<IDXGIFactory>::into(
            ::std::clone::Clone::clone(self),
        ))
    }
}
impl ::std::convert::From<IDXGIFactory3> for IDXGIObject {
    fn from(value: IDXGIFactory3) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IDXGIFactory3> for IDXGIObject {
    fn from(value: &IDXGIFactory3) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::IntoParam<'a, IDXGIObject> for IDXGIFactory3 {
    fn into_param(self) -> ::windows::Param<'a, IDXGIObject> {
        ::windows::Param::Owned(::std::convert::Into::<IDXGIObject>::into(self))
    }
}
impl<'a> ::windows::IntoParam<'a, IDXGIObject> for &'a IDXGIFactory3 {
    fn into_param(self) -> ::windows::Param<'a, IDXGIObject> {
        ::windows::Param::Owned(::std::convert::Into::<IDXGIObject>::into(
            ::std::clone::Clone::clone(self),
        ))
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDXGIFactory3_abi(
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        iid: &::windows::Guid,
        interface: *mut ::windows::RawPtr,
    ) -> ::windows::ErrorCode,
    pub unsafe extern "system" fn(this: ::windows::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::RawPtr) -> u32,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        name: *const ::windows::Guid,
        data_size: u32,
        p_data: *const ::std::ffi::c_void,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        name: *const ::windows::Guid,
        p_unknown: ::windows::RawPtr,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        name: *const ::windows::Guid,
        p_data_size: *mut u32,
        p_data: *mut ::std::ffi::c_void,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        riid: *const ::windows::Guid,
        pp_parent: *mut *mut ::std::ffi::c_void,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        adapter: u32,
        pp_adapter: *mut ::windows::RawPtr,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        window_handle: super::windows_and_messaging::HWND,
        flags: u32,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        p_window_handle: *mut super::windows_and_messaging::HWND,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        p_device: ::windows::RawPtr,
        p_desc: *mut DXGI_SWAP_CHAIN_DESC,
        pp_swap_chain: *mut ::windows::RawPtr,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        module: isize,
        pp_adapter: *mut ::windows::RawPtr,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        adapter: u32,
        pp_adapter: *mut ::windows::RawPtr,
    ) -> ::windows::ErrorCode,
    pub unsafe extern "system" fn(this: ::windows::RawPtr) -> super::system_services::BOOL,
    pub unsafe extern "system" fn(this: ::windows::RawPtr) -> super::system_services::BOOL,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        p_device: ::windows::RawPtr,
        h_wnd: super::windows_and_messaging::HWND,
        p_desc: *const DXGI_SWAP_CHAIN_DESC1,
        p_fullscreen_desc: *const DXGI_SWAP_CHAIN_FULLSCREEN_DESC,
        p_restrict_to_output: ::windows::RawPtr,
        pp_swap_chain: *mut ::windows::RawPtr,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        p_device: ::windows::RawPtr,
        p_window: ::windows::RawPtr,
        p_desc: *const DXGI_SWAP_CHAIN_DESC1,
        p_restrict_to_output: ::windows::RawPtr,
        pp_swap_chain: *mut ::windows::RawPtr,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        h_resource: super::system_services::HANDLE,
        p_luid: *mut super::kernel::LUID,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        window_handle: super::windows_and_messaging::HWND,
        w_msg: u32,
        pdw_cookie: *mut u32,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        h_event: super::system_services::HANDLE,
        pdw_cookie: *mut u32,
    ) -> ::windows::ErrorCode,
    pub unsafe extern "system" fn(this: ::windows::RawPtr, dw_cookie: u32),
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        window_handle: super::windows_and_messaging::HWND,
        w_msg: u32,
        pdw_cookie: *mut u32,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        h_event: super::system_services::HANDLE,
        pdw_cookie: *mut u32,
    ) -> ::windows::ErrorCode,
    pub unsafe extern "system" fn(this: ::windows::RawPtr, dw_cookie: u32),
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        p_device: ::windows::RawPtr,
        p_desc: *const DXGI_SWAP_CHAIN_DESC1,
        p_restrict_to_output: ::windows::RawPtr,
        pp_swap_chain: *mut ::windows::RawPtr,
    ) -> ::windows::ErrorCode,
    pub unsafe extern "system" fn(this: ::windows::RawPtr) -> u32,
);
#[repr(transparent)]
#[allow(non_camel_case_types)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct IDXGIFactory4(::windows::IUnknown);
impl IDXGIFactory4 {}
unsafe impl ::windows::Interface for IDXGIFactory4 {
    type Vtable = IDXGIFactory4_abi;
    const IID: ::windows::Guid = ::windows::Guid::from_values(
        466020866,
        61238,
        17999,
        [191, 12, 33, 202, 57, 229, 22, 138],
    );
}
#[allow(non_snake_case)]
impl IDXGIFactory4 {
    pub unsafe fn SetPrivateData(
        &self,
        name: *const ::windows::Guid,
        data_size: u32,
        p_data: *const ::std::ffi::c_void,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).3)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(name),
            ::std::mem::transmute(data_size),
            ::std::mem::transmute(p_data),
        )
    }
    pub unsafe fn SetPrivateDataInterface<
        'a,
        T1__: ::windows::IntoParam<'a, ::windows::IUnknown>,
    >(
        &self,
        name: *const ::windows::Guid,
        p_unknown: T1__,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).4)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(name),
            p_unknown.into_param().abi(),
        )
    }
    pub unsafe fn GetPrivateData(
        &self,
        name: *const ::windows::Guid,
        p_data_size: *mut u32,
        p_data: *mut ::std::ffi::c_void,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).5)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(name),
            ::std::mem::transmute(p_data_size),
            ::std::mem::transmute(p_data),
        )
    }
    pub unsafe fn GetParent(
        &self,
        riid: *const ::windows::Guid,
        pp_parent: *mut *mut ::std::ffi::c_void,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).6)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(riid),
            ::std::mem::transmute(pp_parent),
        )
    }
    pub unsafe fn EnumAdapters(
        &self,
        adapter: u32,
        pp_adapter: *mut ::std::option::Option<IDXGIAdapter>,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).7)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(adapter),
            ::std::mem::transmute(pp_adapter),
        )
    }
    pub unsafe fn MakeWindowAssociation<
        'a,
        T0__: ::windows::IntoParam<'a, super::windows_and_messaging::HWND>,
    >(
        &self,
        window_handle: T0__,
        flags: u32,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).8)(
            ::windows::Abi::abi(self),
            window_handle.into_param().abi(),
            ::std::mem::transmute(flags),
        )
    }
    pub unsafe fn GetWindowAssociation(
        &self,
        p_window_handle: *mut super::windows_and_messaging::HWND,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).9)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(p_window_handle),
        )
    }
    pub unsafe fn CreateSwapChain<'a, T0__: ::windows::IntoParam<'a, ::windows::IUnknown>>(
        &self,
        p_device: T0__,
        p_desc: *mut DXGI_SWAP_CHAIN_DESC,
        pp_swap_chain: *mut ::std::option::Option<IDXGISwapChain>,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).10)(
            ::windows::Abi::abi(self),
            p_device.into_param().abi(),
            ::std::mem::transmute(p_desc),
            ::std::mem::transmute(pp_swap_chain),
        )
    }
    pub unsafe fn CreateSoftwareAdapter(
        &self,
        module: isize,
        pp_adapter: *mut ::std::option::Option<IDXGIAdapter>,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).11)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(module),
            ::std::mem::transmute(pp_adapter),
        )
    }
    pub unsafe fn EnumAdapters1(
        &self,
        adapter: u32,
        pp_adapter: *mut ::std::option::Option<IDXGIAdapter1>,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).12)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(adapter),
            ::std::mem::transmute(pp_adapter),
        )
    }
    pub unsafe fn IsCurrent(&self) -> super::system_services::BOOL {
        (::windows::Interface::vtable(self).13)(::windows::Abi::abi(self))
    }
    pub unsafe fn IsWindowedStereoEnabled(&self) -> super::system_services::BOOL {
        (::windows::Interface::vtable(self).14)(::windows::Abi::abi(self))
    }
    pub unsafe fn CreateSwapChainForHwnd<
        'a,
        T0__: ::windows::IntoParam<'a, ::windows::IUnknown>,
        T1__: ::windows::IntoParam<'a, super::windows_and_messaging::HWND>,
        T4__: ::windows::IntoParam<'a, IDXGIOutput>,
    >(
        &self,
        p_device: T0__,
        h_wnd: T1__,
        p_desc: *const DXGI_SWAP_CHAIN_DESC1,
        p_fullscreen_desc: *const DXGI_SWAP_CHAIN_FULLSCREEN_DESC,
        p_restrict_to_output: T4__,
        pp_swap_chain: *mut ::std::option::Option<IDXGISwapChain1>,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).15)(
            ::windows::Abi::abi(self),
            p_device.into_param().abi(),
            h_wnd.into_param().abi(),
            ::std::mem::transmute(p_desc),
            ::std::mem::transmute(p_fullscreen_desc),
            p_restrict_to_output.into_param().abi(),
            ::std::mem::transmute(pp_swap_chain),
        )
    }
    pub unsafe fn CreateSwapChainForCoreWindow<
        'a,
        T0__: ::windows::IntoParam<'a, ::windows::IUnknown>,
        T1__: ::windows::IntoParam<'a, ::windows::IUnknown>,
        T3__: ::windows::IntoParam<'a, IDXGIOutput>,
    >(
        &self,
        p_device: T0__,
        p_window: T1__,
        p_desc: *const DXGI_SWAP_CHAIN_DESC1,
        p_restrict_to_output: T3__,
        pp_swap_chain: *mut ::std::option::Option<IDXGISwapChain1>,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).16)(
            ::windows::Abi::abi(self),
            p_device.into_param().abi(),
            p_window.into_param().abi(),
            ::std::mem::transmute(p_desc),
            p_restrict_to_output.into_param().abi(),
            ::std::mem::transmute(pp_swap_chain),
        )
    }
    pub unsafe fn GetSharedResourceAdapterLuid<
        'a,
        T0__: ::windows::IntoParam<'a, super::system_services::HANDLE>,
    >(
        &self,
        h_resource: T0__,
        p_luid: *mut super::kernel::LUID,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).17)(
            ::windows::Abi::abi(self),
            h_resource.into_param().abi(),
            ::std::mem::transmute(p_luid),
        )
    }
    pub unsafe fn RegisterStereoStatusWindow<
        'a,
        T0__: ::windows::IntoParam<'a, super::windows_and_messaging::HWND>,
    >(
        &self,
        window_handle: T0__,
        w_msg: u32,
        pdw_cookie: *mut u32,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).18)(
            ::windows::Abi::abi(self),
            window_handle.into_param().abi(),
            ::std::mem::transmute(w_msg),
            ::std::mem::transmute(pdw_cookie),
        )
    }
    pub unsafe fn RegisterStereoStatusEvent<
        'a,
        T0__: ::windows::IntoParam<'a, super::system_services::HANDLE>,
    >(
        &self,
        h_event: T0__,
        pdw_cookie: *mut u32,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).19)(
            ::windows::Abi::abi(self),
            h_event.into_param().abi(),
            ::std::mem::transmute(pdw_cookie),
        )
    }
    pub unsafe fn UnregisterStereoStatus(&self, dw_cookie: u32) {
        (::windows::Interface::vtable(self).20)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(dw_cookie),
        )
    }
    pub unsafe fn RegisterOcclusionStatusWindow<
        'a,
        T0__: ::windows::IntoParam<'a, super::windows_and_messaging::HWND>,
    >(
        &self,
        window_handle: T0__,
        w_msg: u32,
        pdw_cookie: *mut u32,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).21)(
            ::windows::Abi::abi(self),
            window_handle.into_param().abi(),
            ::std::mem::transmute(w_msg),
            ::std::mem::transmute(pdw_cookie),
        )
    }
    pub unsafe fn RegisterOcclusionStatusEvent<
        'a,
        T0__: ::windows::IntoParam<'a, super::system_services::HANDLE>,
    >(
        &self,
        h_event: T0__,
        pdw_cookie: *mut u32,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).22)(
            ::windows::Abi::abi(self),
            h_event.into_param().abi(),
            ::std::mem::transmute(pdw_cookie),
        )
    }
    pub unsafe fn UnregisterOcclusionStatus(&self, dw_cookie: u32) {
        (::windows::Interface::vtable(self).23)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(dw_cookie),
        )
    }
    pub unsafe fn CreateSwapChainForComposition<
        'a,
        T0__: ::windows::IntoParam<'a, ::windows::IUnknown>,
        T2__: ::windows::IntoParam<'a, IDXGIOutput>,
    >(
        &self,
        p_device: T0__,
        p_desc: *const DXGI_SWAP_CHAIN_DESC1,
        p_restrict_to_output: T2__,
        pp_swap_chain: *mut ::std::option::Option<IDXGISwapChain1>,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).24)(
            ::windows::Abi::abi(self),
            p_device.into_param().abi(),
            ::std::mem::transmute(p_desc),
            p_restrict_to_output.into_param().abi(),
            ::std::mem::transmute(pp_swap_chain),
        )
    }
    pub unsafe fn GetCreationFlags(&self) -> u32 {
        (::windows::Interface::vtable(self).25)(::windows::Abi::abi(self))
    }
    pub unsafe fn EnumAdapterByLuid<'a, T0__: ::windows::IntoParam<'a, super::kernel::LUID>>(
        &self,
        adapter_luid: T0__,
        riid: *const ::windows::Guid,
        ppv_adapter: *mut *mut ::std::ffi::c_void,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).26)(
            ::windows::Abi::abi(self),
            adapter_luid.into_param().abi(),
            ::std::mem::transmute(riid),
            ::std::mem::transmute(ppv_adapter),
        )
    }
    pub unsafe fn EnumWarpAdapter(
        &self,
        riid: *const ::windows::Guid,
        ppv_adapter: *mut *mut ::std::ffi::c_void,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).27)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(riid),
            ::std::mem::transmute(ppv_adapter),
        )
    }
}
impl ::std::convert::From<IDXGIFactory4> for ::windows::IUnknown {
    fn from(value: IDXGIFactory4) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IDXGIFactory4> for ::windows::IUnknown {
    fn from(value: &IDXGIFactory4) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::IntoParam<'a, ::windows::IUnknown> for IDXGIFactory4 {
    fn into_param(self) -> ::windows::Param<'a, ::windows::IUnknown> {
        ::windows::Param::Owned(::std::convert::Into::<::windows::IUnknown>::into(self))
    }
}
impl<'a> ::windows::IntoParam<'a, ::windows::IUnknown> for &'a IDXGIFactory4 {
    fn into_param(self) -> ::windows::Param<'a, ::windows::IUnknown> {
        ::windows::Param::Owned(::std::convert::Into::<::windows::IUnknown>::into(
            ::std::clone::Clone::clone(self),
        ))
    }
}
impl ::std::convert::From<IDXGIFactory4> for IDXGIFactory3 {
    fn from(value: IDXGIFactory4) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IDXGIFactory4> for IDXGIFactory3 {
    fn from(value: &IDXGIFactory4) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::IntoParam<'a, IDXGIFactory3> for IDXGIFactory4 {
    fn into_param(self) -> ::windows::Param<'a, IDXGIFactory3> {
        ::windows::Param::Owned(::std::convert::Into::<IDXGIFactory3>::into(self))
    }
}
impl<'a> ::windows::IntoParam<'a, IDXGIFactory3> for &'a IDXGIFactory4 {
    fn into_param(self) -> ::windows::Param<'a, IDXGIFactory3> {
        ::windows::Param::Owned(::std::convert::Into::<IDXGIFactory3>::into(
            ::std::clone::Clone::clone(self),
        ))
    }
}
impl ::std::convert::From<IDXGIFactory4> for IDXGIFactory2 {
    fn from(value: IDXGIFactory4) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IDXGIFactory4> for IDXGIFactory2 {
    fn from(value: &IDXGIFactory4) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::IntoParam<'a, IDXGIFactory2> for IDXGIFactory4 {
    fn into_param(self) -> ::windows::Param<'a, IDXGIFactory2> {
        ::windows::Param::Owned(::std::convert::Into::<IDXGIFactory2>::into(self))
    }
}
impl<'a> ::windows::IntoParam<'a, IDXGIFactory2> for &'a IDXGIFactory4 {
    fn into_param(self) -> ::windows::Param<'a, IDXGIFactory2> {
        ::windows::Param::Owned(::std::convert::Into::<IDXGIFactory2>::into(
            ::std::clone::Clone::clone(self),
        ))
    }
}
impl ::std::convert::From<IDXGIFactory4> for IDXGIFactory1 {
    fn from(value: IDXGIFactory4) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IDXGIFactory4> for IDXGIFactory1 {
    fn from(value: &IDXGIFactory4) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::IntoParam<'a, IDXGIFactory1> for IDXGIFactory4 {
    fn into_param(self) -> ::windows::Param<'a, IDXGIFactory1> {
        ::windows::Param::Owned(::std::convert::Into::<IDXGIFactory1>::into(self))
    }
}
impl<'a> ::windows::IntoParam<'a, IDXGIFactory1> for &'a IDXGIFactory4 {
    fn into_param(self) -> ::windows::Param<'a, IDXGIFactory1> {
        ::windows::Param::Owned(::std::convert::Into::<IDXGIFactory1>::into(
            ::std::clone::Clone::clone(self),
        ))
    }
}
impl ::std::convert::From<IDXGIFactory4> for IDXGIFactory {
    fn from(value: IDXGIFactory4) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IDXGIFactory4> for IDXGIFactory {
    fn from(value: &IDXGIFactory4) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::IntoParam<'a, IDXGIFactory> for IDXGIFactory4 {
    fn into_param(self) -> ::windows::Param<'a, IDXGIFactory> {
        ::windows::Param::Owned(::std::convert::Into::<IDXGIFactory>::into(self))
    }
}
impl<'a> ::windows::IntoParam<'a, IDXGIFactory> for &'a IDXGIFactory4 {
    fn into_param(self) -> ::windows::Param<'a, IDXGIFactory> {
        ::windows::Param::Owned(::std::convert::Into::<IDXGIFactory>::into(
            ::std::clone::Clone::clone(self),
        ))
    }
}
impl ::std::convert::From<IDXGIFactory4> for IDXGIObject {
    fn from(value: IDXGIFactory4) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IDXGIFactory4> for IDXGIObject {
    fn from(value: &IDXGIFactory4) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::IntoParam<'a, IDXGIObject> for IDXGIFactory4 {
    fn into_param(self) -> ::windows::Param<'a, IDXGIObject> {
        ::windows::Param::Owned(::std::convert::Into::<IDXGIObject>::into(self))
    }
}
impl<'a> ::windows::IntoParam<'a, IDXGIObject> for &'a IDXGIFactory4 {
    fn into_param(self) -> ::windows::Param<'a, IDXGIObject> {
        ::windows::Param::Owned(::std::convert::Into::<IDXGIObject>::into(
            ::std::clone::Clone::clone(self),
        ))
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDXGIFactory4_abi(
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        iid: &::windows::Guid,
        interface: *mut ::windows::RawPtr,
    ) -> ::windows::ErrorCode,
    pub unsafe extern "system" fn(this: ::windows::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::RawPtr) -> u32,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        name: *const ::windows::Guid,
        data_size: u32,
        p_data: *const ::std::ffi::c_void,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        name: *const ::windows::Guid,
        p_unknown: ::windows::RawPtr,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        name: *const ::windows::Guid,
        p_data_size: *mut u32,
        p_data: *mut ::std::ffi::c_void,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        riid: *const ::windows::Guid,
        pp_parent: *mut *mut ::std::ffi::c_void,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        adapter: u32,
        pp_adapter: *mut ::windows::RawPtr,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        window_handle: super::windows_and_messaging::HWND,
        flags: u32,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        p_window_handle: *mut super::windows_and_messaging::HWND,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        p_device: ::windows::RawPtr,
        p_desc: *mut DXGI_SWAP_CHAIN_DESC,
        pp_swap_chain: *mut ::windows::RawPtr,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        module: isize,
        pp_adapter: *mut ::windows::RawPtr,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        adapter: u32,
        pp_adapter: *mut ::windows::RawPtr,
    ) -> ::windows::ErrorCode,
    pub unsafe extern "system" fn(this: ::windows::RawPtr) -> super::system_services::BOOL,
    pub unsafe extern "system" fn(this: ::windows::RawPtr) -> super::system_services::BOOL,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        p_device: ::windows::RawPtr,
        h_wnd: super::windows_and_messaging::HWND,
        p_desc: *const DXGI_SWAP_CHAIN_DESC1,
        p_fullscreen_desc: *const DXGI_SWAP_CHAIN_FULLSCREEN_DESC,
        p_restrict_to_output: ::windows::RawPtr,
        pp_swap_chain: *mut ::windows::RawPtr,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        p_device: ::windows::RawPtr,
        p_window: ::windows::RawPtr,
        p_desc: *const DXGI_SWAP_CHAIN_DESC1,
        p_restrict_to_output: ::windows::RawPtr,
        pp_swap_chain: *mut ::windows::RawPtr,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        h_resource: super::system_services::HANDLE,
        p_luid: *mut super::kernel::LUID,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        window_handle: super::windows_and_messaging::HWND,
        w_msg: u32,
        pdw_cookie: *mut u32,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        h_event: super::system_services::HANDLE,
        pdw_cookie: *mut u32,
    ) -> ::windows::ErrorCode,
    pub unsafe extern "system" fn(this: ::windows::RawPtr, dw_cookie: u32),
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        window_handle: super::windows_and_messaging::HWND,
        w_msg: u32,
        pdw_cookie: *mut u32,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        h_event: super::system_services::HANDLE,
        pdw_cookie: *mut u32,
    ) -> ::windows::ErrorCode,
    pub unsafe extern "system" fn(this: ::windows::RawPtr, dw_cookie: u32),
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        p_device: ::windows::RawPtr,
        p_desc: *const DXGI_SWAP_CHAIN_DESC1,
        p_restrict_to_output: ::windows::RawPtr,
        pp_swap_chain: *mut ::windows::RawPtr,
    ) -> ::windows::ErrorCode,
    pub unsafe extern "system" fn(this: ::windows::RawPtr) -> u32,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        adapter_luid: super::kernel::LUID,
        riid: *const ::windows::Guid,
        ppv_adapter: *mut *mut ::std::ffi::c_void,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        riid: *const ::windows::Guid,
        ppv_adapter: *mut *mut ::std::ffi::c_void,
    ) -> ::windows::ErrorCode,
);
#[repr(transparent)]
#[allow(non_camel_case_types)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct IDXGIFactory5(::windows::IUnknown);
impl IDXGIFactory5 {}
unsafe impl ::windows::Interface for IDXGIFactory5 {
    type Vtable = IDXGIFactory5_abi;
    const IID: ::windows::Guid = ::windows::Guid::from_values(
        1983046133,
        61029,
        19914,
        [135, 253, 132, 205, 117, 248, 131, 141],
    );
}
#[allow(non_snake_case)]
impl IDXGIFactory5 {
    pub unsafe fn SetPrivateData(
        &self,
        name: *const ::windows::Guid,
        data_size: u32,
        p_data: *const ::std::ffi::c_void,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).3)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(name),
            ::std::mem::transmute(data_size),
            ::std::mem::transmute(p_data),
        )
    }
    pub unsafe fn SetPrivateDataInterface<
        'a,
        T1__: ::windows::IntoParam<'a, ::windows::IUnknown>,
    >(
        &self,
        name: *const ::windows::Guid,
        p_unknown: T1__,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).4)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(name),
            p_unknown.into_param().abi(),
        )
    }
    pub unsafe fn GetPrivateData(
        &self,
        name: *const ::windows::Guid,
        p_data_size: *mut u32,
        p_data: *mut ::std::ffi::c_void,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).5)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(name),
            ::std::mem::transmute(p_data_size),
            ::std::mem::transmute(p_data),
        )
    }
    pub unsafe fn GetParent(
        &self,
        riid: *const ::windows::Guid,
        pp_parent: *mut *mut ::std::ffi::c_void,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).6)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(riid),
            ::std::mem::transmute(pp_parent),
        )
    }
    pub unsafe fn EnumAdapters(
        &self,
        adapter: u32,
        pp_adapter: *mut ::std::option::Option<IDXGIAdapter>,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).7)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(adapter),
            ::std::mem::transmute(pp_adapter),
        )
    }
    pub unsafe fn MakeWindowAssociation<
        'a,
        T0__: ::windows::IntoParam<'a, super::windows_and_messaging::HWND>,
    >(
        &self,
        window_handle: T0__,
        flags: u32,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).8)(
            ::windows::Abi::abi(self),
            window_handle.into_param().abi(),
            ::std::mem::transmute(flags),
        )
    }
    pub unsafe fn GetWindowAssociation(
        &self,
        p_window_handle: *mut super::windows_and_messaging::HWND,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).9)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(p_window_handle),
        )
    }
    pub unsafe fn CreateSwapChain<'a, T0__: ::windows::IntoParam<'a, ::windows::IUnknown>>(
        &self,
        p_device: T0__,
        p_desc: *mut DXGI_SWAP_CHAIN_DESC,
        pp_swap_chain: *mut ::std::option::Option<IDXGISwapChain>,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).10)(
            ::windows::Abi::abi(self),
            p_device.into_param().abi(),
            ::std::mem::transmute(p_desc),
            ::std::mem::transmute(pp_swap_chain),
        )
    }
    pub unsafe fn CreateSoftwareAdapter(
        &self,
        module: isize,
        pp_adapter: *mut ::std::option::Option<IDXGIAdapter>,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).11)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(module),
            ::std::mem::transmute(pp_adapter),
        )
    }
    pub unsafe fn EnumAdapters1(
        &self,
        adapter: u32,
        pp_adapter: *mut ::std::option::Option<IDXGIAdapter1>,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).12)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(adapter),
            ::std::mem::transmute(pp_adapter),
        )
    }
    pub unsafe fn IsCurrent(&self) -> super::system_services::BOOL {
        (::windows::Interface::vtable(self).13)(::windows::Abi::abi(self))
    }
    pub unsafe fn IsWindowedStereoEnabled(&self) -> super::system_services::BOOL {
        (::windows::Interface::vtable(self).14)(::windows::Abi::abi(self))
    }
    pub unsafe fn CreateSwapChainForHwnd<
        'a,
        T0__: ::windows::IntoParam<'a, ::windows::IUnknown>,
        T1__: ::windows::IntoParam<'a, super::windows_and_messaging::HWND>,
        T4__: ::windows::IntoParam<'a, IDXGIOutput>,
    >(
        &self,
        p_device: T0__,
        h_wnd: T1__,
        p_desc: *const DXGI_SWAP_CHAIN_DESC1,
        p_fullscreen_desc: *const DXGI_SWAP_CHAIN_FULLSCREEN_DESC,
        p_restrict_to_output: T4__,
        pp_swap_chain: *mut ::std::option::Option<IDXGISwapChain1>,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).15)(
            ::windows::Abi::abi(self),
            p_device.into_param().abi(),
            h_wnd.into_param().abi(),
            ::std::mem::transmute(p_desc),
            ::std::mem::transmute(p_fullscreen_desc),
            p_restrict_to_output.into_param().abi(),
            ::std::mem::transmute(pp_swap_chain),
        )
    }
    pub unsafe fn CreateSwapChainForCoreWindow<
        'a,
        T0__: ::windows::IntoParam<'a, ::windows::IUnknown>,
        T1__: ::windows::IntoParam<'a, ::windows::IUnknown>,
        T3__: ::windows::IntoParam<'a, IDXGIOutput>,
    >(
        &self,
        p_device: T0__,
        p_window: T1__,
        p_desc: *const DXGI_SWAP_CHAIN_DESC1,
        p_restrict_to_output: T3__,
        pp_swap_chain: *mut ::std::option::Option<IDXGISwapChain1>,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).16)(
            ::windows::Abi::abi(self),
            p_device.into_param().abi(),
            p_window.into_param().abi(),
            ::std::mem::transmute(p_desc),
            p_restrict_to_output.into_param().abi(),
            ::std::mem::transmute(pp_swap_chain),
        )
    }
    pub unsafe fn GetSharedResourceAdapterLuid<
        'a,
        T0__: ::windows::IntoParam<'a, super::system_services::HANDLE>,
    >(
        &self,
        h_resource: T0__,
        p_luid: *mut super::kernel::LUID,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).17)(
            ::windows::Abi::abi(self),
            h_resource.into_param().abi(),
            ::std::mem::transmute(p_luid),
        )
    }
    pub unsafe fn RegisterStereoStatusWindow<
        'a,
        T0__: ::windows::IntoParam<'a, super::windows_and_messaging::HWND>,
    >(
        &self,
        window_handle: T0__,
        w_msg: u32,
        pdw_cookie: *mut u32,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).18)(
            ::windows::Abi::abi(self),
            window_handle.into_param().abi(),
            ::std::mem::transmute(w_msg),
            ::std::mem::transmute(pdw_cookie),
        )
    }
    pub unsafe fn RegisterStereoStatusEvent<
        'a,
        T0__: ::windows::IntoParam<'a, super::system_services::HANDLE>,
    >(
        &self,
        h_event: T0__,
        pdw_cookie: *mut u32,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).19)(
            ::windows::Abi::abi(self),
            h_event.into_param().abi(),
            ::std::mem::transmute(pdw_cookie),
        )
    }
    pub unsafe fn UnregisterStereoStatus(&self, dw_cookie: u32) {
        (::windows::Interface::vtable(self).20)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(dw_cookie),
        )
    }
    pub unsafe fn RegisterOcclusionStatusWindow<
        'a,
        T0__: ::windows::IntoParam<'a, super::windows_and_messaging::HWND>,
    >(
        &self,
        window_handle: T0__,
        w_msg: u32,
        pdw_cookie: *mut u32,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).21)(
            ::windows::Abi::abi(self),
            window_handle.into_param().abi(),
            ::std::mem::transmute(w_msg),
            ::std::mem::transmute(pdw_cookie),
        )
    }
    pub unsafe fn RegisterOcclusionStatusEvent<
        'a,
        T0__: ::windows::IntoParam<'a, super::system_services::HANDLE>,
    >(
        &self,
        h_event: T0__,
        pdw_cookie: *mut u32,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).22)(
            ::windows::Abi::abi(self),
            h_event.into_param().abi(),
            ::std::mem::transmute(pdw_cookie),
        )
    }
    pub unsafe fn UnregisterOcclusionStatus(&self, dw_cookie: u32) {
        (::windows::Interface::vtable(self).23)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(dw_cookie),
        )
    }
    pub unsafe fn CreateSwapChainForComposition<
        'a,
        T0__: ::windows::IntoParam<'a, ::windows::IUnknown>,
        T2__: ::windows::IntoParam<'a, IDXGIOutput>,
    >(
        &self,
        p_device: T0__,
        p_desc: *const DXGI_SWAP_CHAIN_DESC1,
        p_restrict_to_output: T2__,
        pp_swap_chain: *mut ::std::option::Option<IDXGISwapChain1>,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).24)(
            ::windows::Abi::abi(self),
            p_device.into_param().abi(),
            ::std::mem::transmute(p_desc),
            p_restrict_to_output.into_param().abi(),
            ::std::mem::transmute(pp_swap_chain),
        )
    }
    pub unsafe fn GetCreationFlags(&self) -> u32 {
        (::windows::Interface::vtable(self).25)(::windows::Abi::abi(self))
    }
    pub unsafe fn EnumAdapterByLuid<'a, T0__: ::windows::IntoParam<'a, super::kernel::LUID>>(
        &self,
        adapter_luid: T0__,
        riid: *const ::windows::Guid,
        ppv_adapter: *mut *mut ::std::ffi::c_void,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).26)(
            ::windows::Abi::abi(self),
            adapter_luid.into_param().abi(),
            ::std::mem::transmute(riid),
            ::std::mem::transmute(ppv_adapter),
        )
    }
    pub unsafe fn EnumWarpAdapter(
        &self,
        riid: *const ::windows::Guid,
        ppv_adapter: *mut *mut ::std::ffi::c_void,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).27)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(riid),
            ::std::mem::transmute(ppv_adapter),
        )
    }
    pub unsafe fn CheckFeatureSupport(
        &self,
        feature: DXGI_FEATURE,
        p_feature_support_data: *mut ::std::ffi::c_void,
        feature_support_data_size: u32,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).28)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(feature),
            ::std::mem::transmute(p_feature_support_data),
            ::std::mem::transmute(feature_support_data_size),
        )
    }
}
impl ::std::convert::From<IDXGIFactory5> for ::windows::IUnknown {
    fn from(value: IDXGIFactory5) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IDXGIFactory5> for ::windows::IUnknown {
    fn from(value: &IDXGIFactory5) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::IntoParam<'a, ::windows::IUnknown> for IDXGIFactory5 {
    fn into_param(self) -> ::windows::Param<'a, ::windows::IUnknown> {
        ::windows::Param::Owned(::std::convert::Into::<::windows::IUnknown>::into(self))
    }
}
impl<'a> ::windows::IntoParam<'a, ::windows::IUnknown> for &'a IDXGIFactory5 {
    fn into_param(self) -> ::windows::Param<'a, ::windows::IUnknown> {
        ::windows::Param::Owned(::std::convert::Into::<::windows::IUnknown>::into(
            ::std::clone::Clone::clone(self),
        ))
    }
}
impl ::std::convert::From<IDXGIFactory5> for IDXGIFactory4 {
    fn from(value: IDXGIFactory5) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IDXGIFactory5> for IDXGIFactory4 {
    fn from(value: &IDXGIFactory5) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::IntoParam<'a, IDXGIFactory4> for IDXGIFactory5 {
    fn into_param(self) -> ::windows::Param<'a, IDXGIFactory4> {
        ::windows::Param::Owned(::std::convert::Into::<IDXGIFactory4>::into(self))
    }
}
impl<'a> ::windows::IntoParam<'a, IDXGIFactory4> for &'a IDXGIFactory5 {
    fn into_param(self) -> ::windows::Param<'a, IDXGIFactory4> {
        ::windows::Param::Owned(::std::convert::Into::<IDXGIFactory4>::into(
            ::std::clone::Clone::clone(self),
        ))
    }
}
impl ::std::convert::From<IDXGIFactory5> for IDXGIFactory3 {
    fn from(value: IDXGIFactory5) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IDXGIFactory5> for IDXGIFactory3 {
    fn from(value: &IDXGIFactory5) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::IntoParam<'a, IDXGIFactory3> for IDXGIFactory5 {
    fn into_param(self) -> ::windows::Param<'a, IDXGIFactory3> {
        ::windows::Param::Owned(::std::convert::Into::<IDXGIFactory3>::into(self))
    }
}
impl<'a> ::windows::IntoParam<'a, IDXGIFactory3> for &'a IDXGIFactory5 {
    fn into_param(self) -> ::windows::Param<'a, IDXGIFactory3> {
        ::windows::Param::Owned(::std::convert::Into::<IDXGIFactory3>::into(
            ::std::clone::Clone::clone(self),
        ))
    }
}
impl ::std::convert::From<IDXGIFactory5> for IDXGIFactory2 {
    fn from(value: IDXGIFactory5) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IDXGIFactory5> for IDXGIFactory2 {
    fn from(value: &IDXGIFactory5) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::IntoParam<'a, IDXGIFactory2> for IDXGIFactory5 {
    fn into_param(self) -> ::windows::Param<'a, IDXGIFactory2> {
        ::windows::Param::Owned(::std::convert::Into::<IDXGIFactory2>::into(self))
    }
}
impl<'a> ::windows::IntoParam<'a, IDXGIFactory2> for &'a IDXGIFactory5 {
    fn into_param(self) -> ::windows::Param<'a, IDXGIFactory2> {
        ::windows::Param::Owned(::std::convert::Into::<IDXGIFactory2>::into(
            ::std::clone::Clone::clone(self),
        ))
    }
}
impl ::std::convert::From<IDXGIFactory5> for IDXGIFactory1 {
    fn from(value: IDXGIFactory5) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IDXGIFactory5> for IDXGIFactory1 {
    fn from(value: &IDXGIFactory5) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::IntoParam<'a, IDXGIFactory1> for IDXGIFactory5 {
    fn into_param(self) -> ::windows::Param<'a, IDXGIFactory1> {
        ::windows::Param::Owned(::std::convert::Into::<IDXGIFactory1>::into(self))
    }
}
impl<'a> ::windows::IntoParam<'a, IDXGIFactory1> for &'a IDXGIFactory5 {
    fn into_param(self) -> ::windows::Param<'a, IDXGIFactory1> {
        ::windows::Param::Owned(::std::convert::Into::<IDXGIFactory1>::into(
            ::std::clone::Clone::clone(self),
        ))
    }
}
impl ::std::convert::From<IDXGIFactory5> for IDXGIFactory {
    fn from(value: IDXGIFactory5) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IDXGIFactory5> for IDXGIFactory {
    fn from(value: &IDXGIFactory5) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::IntoParam<'a, IDXGIFactory> for IDXGIFactory5 {
    fn into_param(self) -> ::windows::Param<'a, IDXGIFactory> {
        ::windows::Param::Owned(::std::convert::Into::<IDXGIFactory>::into(self))
    }
}
impl<'a> ::windows::IntoParam<'a, IDXGIFactory> for &'a IDXGIFactory5 {
    fn into_param(self) -> ::windows::Param<'a, IDXGIFactory> {
        ::windows::Param::Owned(::std::convert::Into::<IDXGIFactory>::into(
            ::std::clone::Clone::clone(self),
        ))
    }
}
impl ::std::convert::From<IDXGIFactory5> for IDXGIObject {
    fn from(value: IDXGIFactory5) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IDXGIFactory5> for IDXGIObject {
    fn from(value: &IDXGIFactory5) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::IntoParam<'a, IDXGIObject> for IDXGIFactory5 {
    fn into_param(self) -> ::windows::Param<'a, IDXGIObject> {
        ::windows::Param::Owned(::std::convert::Into::<IDXGIObject>::into(self))
    }
}
impl<'a> ::windows::IntoParam<'a, IDXGIObject> for &'a IDXGIFactory5 {
    fn into_param(self) -> ::windows::Param<'a, IDXGIObject> {
        ::windows::Param::Owned(::std::convert::Into::<IDXGIObject>::into(
            ::std::clone::Clone::clone(self),
        ))
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDXGIFactory5_abi(
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        iid: &::windows::Guid,
        interface: *mut ::windows::RawPtr,
    ) -> ::windows::ErrorCode,
    pub unsafe extern "system" fn(this: ::windows::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::RawPtr) -> u32,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        name: *const ::windows::Guid,
        data_size: u32,
        p_data: *const ::std::ffi::c_void,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        name: *const ::windows::Guid,
        p_unknown: ::windows::RawPtr,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        name: *const ::windows::Guid,
        p_data_size: *mut u32,
        p_data: *mut ::std::ffi::c_void,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        riid: *const ::windows::Guid,
        pp_parent: *mut *mut ::std::ffi::c_void,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        adapter: u32,
        pp_adapter: *mut ::windows::RawPtr,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        window_handle: super::windows_and_messaging::HWND,
        flags: u32,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        p_window_handle: *mut super::windows_and_messaging::HWND,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        p_device: ::windows::RawPtr,
        p_desc: *mut DXGI_SWAP_CHAIN_DESC,
        pp_swap_chain: *mut ::windows::RawPtr,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        module: isize,
        pp_adapter: *mut ::windows::RawPtr,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        adapter: u32,
        pp_adapter: *mut ::windows::RawPtr,
    ) -> ::windows::ErrorCode,
    pub unsafe extern "system" fn(this: ::windows::RawPtr) -> super::system_services::BOOL,
    pub unsafe extern "system" fn(this: ::windows::RawPtr) -> super::system_services::BOOL,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        p_device: ::windows::RawPtr,
        h_wnd: super::windows_and_messaging::HWND,
        p_desc: *const DXGI_SWAP_CHAIN_DESC1,
        p_fullscreen_desc: *const DXGI_SWAP_CHAIN_FULLSCREEN_DESC,
        p_restrict_to_output: ::windows::RawPtr,
        pp_swap_chain: *mut ::windows::RawPtr,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        p_device: ::windows::RawPtr,
        p_window: ::windows::RawPtr,
        p_desc: *const DXGI_SWAP_CHAIN_DESC1,
        p_restrict_to_output: ::windows::RawPtr,
        pp_swap_chain: *mut ::windows::RawPtr,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        h_resource: super::system_services::HANDLE,
        p_luid: *mut super::kernel::LUID,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        window_handle: super::windows_and_messaging::HWND,
        w_msg: u32,
        pdw_cookie: *mut u32,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        h_event: super::system_services::HANDLE,
        pdw_cookie: *mut u32,
    ) -> ::windows::ErrorCode,
    pub unsafe extern "system" fn(this: ::windows::RawPtr, dw_cookie: u32),
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        window_handle: super::windows_and_messaging::HWND,
        w_msg: u32,
        pdw_cookie: *mut u32,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        h_event: super::system_services::HANDLE,
        pdw_cookie: *mut u32,
    ) -> ::windows::ErrorCode,
    pub unsafe extern "system" fn(this: ::windows::RawPtr, dw_cookie: u32),
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        p_device: ::windows::RawPtr,
        p_desc: *const DXGI_SWAP_CHAIN_DESC1,
        p_restrict_to_output: ::windows::RawPtr,
        pp_swap_chain: *mut ::windows::RawPtr,
    ) -> ::windows::ErrorCode,
    pub unsafe extern "system" fn(this: ::windows::RawPtr) -> u32,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        adapter_luid: super::kernel::LUID,
        riid: *const ::windows::Guid,
        ppv_adapter: *mut *mut ::std::ffi::c_void,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        riid: *const ::windows::Guid,
        ppv_adapter: *mut *mut ::std::ffi::c_void,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        feature: DXGI_FEATURE,
        p_feature_support_data: *mut ::std::ffi::c_void,
        feature_support_data_size: u32,
    ) -> ::windows::ErrorCode,
);
#[repr(transparent)]
#[allow(non_camel_case_types)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct IDXGIFactory6(::windows::IUnknown);
impl IDXGIFactory6 {}
unsafe impl ::windows::Interface for IDXGIFactory6 {
    type Vtable = IDXGIFactory6_abi;
    const IID: ::windows::Guid = ::windows::Guid::from_values(
        3249957199,
        65289,
        17577,
        [176, 60, 119, 144, 10, 10, 29, 23],
    );
}
#[allow(non_snake_case)]
impl IDXGIFactory6 {
    pub unsafe fn SetPrivateData(
        &self,
        name: *const ::windows::Guid,
        data_size: u32,
        p_data: *const ::std::ffi::c_void,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).3)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(name),
            ::std::mem::transmute(data_size),
            ::std::mem::transmute(p_data),
        )
    }
    pub unsafe fn SetPrivateDataInterface<
        'a,
        T1__: ::windows::IntoParam<'a, ::windows::IUnknown>,
    >(
        &self,
        name: *const ::windows::Guid,
        p_unknown: T1__,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).4)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(name),
            p_unknown.into_param().abi(),
        )
    }
    pub unsafe fn GetPrivateData(
        &self,
        name: *const ::windows::Guid,
        p_data_size: *mut u32,
        p_data: *mut ::std::ffi::c_void,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).5)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(name),
            ::std::mem::transmute(p_data_size),
            ::std::mem::transmute(p_data),
        )
    }
    pub unsafe fn GetParent(
        &self,
        riid: *const ::windows::Guid,
        pp_parent: *mut *mut ::std::ffi::c_void,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).6)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(riid),
            ::std::mem::transmute(pp_parent),
        )
    }
    pub unsafe fn EnumAdapters(
        &self,
        adapter: u32,
        pp_adapter: *mut ::std::option::Option<IDXGIAdapter>,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).7)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(adapter),
            ::std::mem::transmute(pp_adapter),
        )
    }
    pub unsafe fn MakeWindowAssociation<
        'a,
        T0__: ::windows::IntoParam<'a, super::windows_and_messaging::HWND>,
    >(
        &self,
        window_handle: T0__,
        flags: u32,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).8)(
            ::windows::Abi::abi(self),
            window_handle.into_param().abi(),
            ::std::mem::transmute(flags),
        )
    }
    pub unsafe fn GetWindowAssociation(
        &self,
        p_window_handle: *mut super::windows_and_messaging::HWND,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).9)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(p_window_handle),
        )
    }
    pub unsafe fn CreateSwapChain<'a, T0__: ::windows::IntoParam<'a, ::windows::IUnknown>>(
        &self,
        p_device: T0__,
        p_desc: *mut DXGI_SWAP_CHAIN_DESC,
        pp_swap_chain: *mut ::std::option::Option<IDXGISwapChain>,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).10)(
            ::windows::Abi::abi(self),
            p_device.into_param().abi(),
            ::std::mem::transmute(p_desc),
            ::std::mem::transmute(pp_swap_chain),
        )
    }
    pub unsafe fn CreateSoftwareAdapter(
        &self,
        module: isize,
        pp_adapter: *mut ::std::option::Option<IDXGIAdapter>,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).11)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(module),
            ::std::mem::transmute(pp_adapter),
        )
    }
    pub unsafe fn EnumAdapters1(
        &self,
        adapter: u32,
        pp_adapter: *mut ::std::option::Option<IDXGIAdapter1>,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).12)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(adapter),
            ::std::mem::transmute(pp_adapter),
        )
    }
    pub unsafe fn IsCurrent(&self) -> super::system_services::BOOL {
        (::windows::Interface::vtable(self).13)(::windows::Abi::abi(self))
    }
    pub unsafe fn IsWindowedStereoEnabled(&self) -> super::system_services::BOOL {
        (::windows::Interface::vtable(self).14)(::windows::Abi::abi(self))
    }
    pub unsafe fn CreateSwapChainForHwnd<
        'a,
        T0__: ::windows::IntoParam<'a, ::windows::IUnknown>,
        T1__: ::windows::IntoParam<'a, super::windows_and_messaging::HWND>,
        T4__: ::windows::IntoParam<'a, IDXGIOutput>,
    >(
        &self,
        p_device: T0__,
        h_wnd: T1__,
        p_desc: *const DXGI_SWAP_CHAIN_DESC1,
        p_fullscreen_desc: *const DXGI_SWAP_CHAIN_FULLSCREEN_DESC,
        p_restrict_to_output: T4__,
        pp_swap_chain: *mut ::std::option::Option<IDXGISwapChain1>,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).15)(
            ::windows::Abi::abi(self),
            p_device.into_param().abi(),
            h_wnd.into_param().abi(),
            ::std::mem::transmute(p_desc),
            ::std::mem::transmute(p_fullscreen_desc),
            p_restrict_to_output.into_param().abi(),
            ::std::mem::transmute(pp_swap_chain),
        )
    }
    pub unsafe fn CreateSwapChainForCoreWindow<
        'a,
        T0__: ::windows::IntoParam<'a, ::windows::IUnknown>,
        T1__: ::windows::IntoParam<'a, ::windows::IUnknown>,
        T3__: ::windows::IntoParam<'a, IDXGIOutput>,
    >(
        &self,
        p_device: T0__,
        p_window: T1__,
        p_desc: *const DXGI_SWAP_CHAIN_DESC1,
        p_restrict_to_output: T3__,
        pp_swap_chain: *mut ::std::option::Option<IDXGISwapChain1>,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).16)(
            ::windows::Abi::abi(self),
            p_device.into_param().abi(),
            p_window.into_param().abi(),
            ::std::mem::transmute(p_desc),
            p_restrict_to_output.into_param().abi(),
            ::std::mem::transmute(pp_swap_chain),
        )
    }
    pub unsafe fn GetSharedResourceAdapterLuid<
        'a,
        T0__: ::windows::IntoParam<'a, super::system_services::HANDLE>,
    >(
        &self,
        h_resource: T0__,
        p_luid: *mut super::kernel::LUID,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).17)(
            ::windows::Abi::abi(self),
            h_resource.into_param().abi(),
            ::std::mem::transmute(p_luid),
        )
    }
    pub unsafe fn RegisterStereoStatusWindow<
        'a,
        T0__: ::windows::IntoParam<'a, super::windows_and_messaging::HWND>,
    >(
        &self,
        window_handle: T0__,
        w_msg: u32,
        pdw_cookie: *mut u32,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).18)(
            ::windows::Abi::abi(self),
            window_handle.into_param().abi(),
            ::std::mem::transmute(w_msg),
            ::std::mem::transmute(pdw_cookie),
        )
    }
    pub unsafe fn RegisterStereoStatusEvent<
        'a,
        T0__: ::windows::IntoParam<'a, super::system_services::HANDLE>,
    >(
        &self,
        h_event: T0__,
        pdw_cookie: *mut u32,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).19)(
            ::windows::Abi::abi(self),
            h_event.into_param().abi(),
            ::std::mem::transmute(pdw_cookie),
        )
    }
    pub unsafe fn UnregisterStereoStatus(&self, dw_cookie: u32) {
        (::windows::Interface::vtable(self).20)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(dw_cookie),
        )
    }
    pub unsafe fn RegisterOcclusionStatusWindow<
        'a,
        T0__: ::windows::IntoParam<'a, super::windows_and_messaging::HWND>,
    >(
        &self,
        window_handle: T0__,
        w_msg: u32,
        pdw_cookie: *mut u32,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).21)(
            ::windows::Abi::abi(self),
            window_handle.into_param().abi(),
            ::std::mem::transmute(w_msg),
            ::std::mem::transmute(pdw_cookie),
        )
    }
    pub unsafe fn RegisterOcclusionStatusEvent<
        'a,
        T0__: ::windows::IntoParam<'a, super::system_services::HANDLE>,
    >(
        &self,
        h_event: T0__,
        pdw_cookie: *mut u32,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).22)(
            ::windows::Abi::abi(self),
            h_event.into_param().abi(),
            ::std::mem::transmute(pdw_cookie),
        )
    }
    pub unsafe fn UnregisterOcclusionStatus(&self, dw_cookie: u32) {
        (::windows::Interface::vtable(self).23)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(dw_cookie),
        )
    }
    pub unsafe fn CreateSwapChainForComposition<
        'a,
        T0__: ::windows::IntoParam<'a, ::windows::IUnknown>,
        T2__: ::windows::IntoParam<'a, IDXGIOutput>,
    >(
        &self,
        p_device: T0__,
        p_desc: *const DXGI_SWAP_CHAIN_DESC1,
        p_restrict_to_output: T2__,
        pp_swap_chain: *mut ::std::option::Option<IDXGISwapChain1>,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).24)(
            ::windows::Abi::abi(self),
            p_device.into_param().abi(),
            ::std::mem::transmute(p_desc),
            p_restrict_to_output.into_param().abi(),
            ::std::mem::transmute(pp_swap_chain),
        )
    }
    pub unsafe fn GetCreationFlags(&self) -> u32 {
        (::windows::Interface::vtable(self).25)(::windows::Abi::abi(self))
    }
    pub unsafe fn EnumAdapterByLuid<'a, T0__: ::windows::IntoParam<'a, super::kernel::LUID>>(
        &self,
        adapter_luid: T0__,
        riid: *const ::windows::Guid,
        ppv_adapter: *mut *mut ::std::ffi::c_void,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).26)(
            ::windows::Abi::abi(self),
            adapter_luid.into_param().abi(),
            ::std::mem::transmute(riid),
            ::std::mem::transmute(ppv_adapter),
        )
    }
    pub unsafe fn EnumWarpAdapter(
        &self,
        riid: *const ::windows::Guid,
        ppv_adapter: *mut *mut ::std::ffi::c_void,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).27)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(riid),
            ::std::mem::transmute(ppv_adapter),
        )
    }
    pub unsafe fn CheckFeatureSupport(
        &self,
        feature: DXGI_FEATURE,
        p_feature_support_data: *mut ::std::ffi::c_void,
        feature_support_data_size: u32,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).28)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(feature),
            ::std::mem::transmute(p_feature_support_data),
            ::std::mem::transmute(feature_support_data_size),
        )
    }
    pub unsafe fn EnumAdapterByGpuPreference(
        &self,
        adapter: u32,
        gpu_preference: DXGI_GPU_PREFERENCE,
        riid: *const ::windows::Guid,
        ppv_adapter: *mut *mut ::std::ffi::c_void,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).29)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(adapter),
            ::std::mem::transmute(gpu_preference),
            ::std::mem::transmute(riid),
            ::std::mem::transmute(ppv_adapter),
        )
    }
}
impl ::std::convert::From<IDXGIFactory6> for ::windows::IUnknown {
    fn from(value: IDXGIFactory6) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IDXGIFactory6> for ::windows::IUnknown {
    fn from(value: &IDXGIFactory6) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::IntoParam<'a, ::windows::IUnknown> for IDXGIFactory6 {
    fn into_param(self) -> ::windows::Param<'a, ::windows::IUnknown> {
        ::windows::Param::Owned(::std::convert::Into::<::windows::IUnknown>::into(self))
    }
}
impl<'a> ::windows::IntoParam<'a, ::windows::IUnknown> for &'a IDXGIFactory6 {
    fn into_param(self) -> ::windows::Param<'a, ::windows::IUnknown> {
        ::windows::Param::Owned(::std::convert::Into::<::windows::IUnknown>::into(
            ::std::clone::Clone::clone(self),
        ))
    }
}
impl ::std::convert::From<IDXGIFactory6> for IDXGIFactory5 {
    fn from(value: IDXGIFactory6) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IDXGIFactory6> for IDXGIFactory5 {
    fn from(value: &IDXGIFactory6) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::IntoParam<'a, IDXGIFactory5> for IDXGIFactory6 {
    fn into_param(self) -> ::windows::Param<'a, IDXGIFactory5> {
        ::windows::Param::Owned(::std::convert::Into::<IDXGIFactory5>::into(self))
    }
}
impl<'a> ::windows::IntoParam<'a, IDXGIFactory5> for &'a IDXGIFactory6 {
    fn into_param(self) -> ::windows::Param<'a, IDXGIFactory5> {
        ::windows::Param::Owned(::std::convert::Into::<IDXGIFactory5>::into(
            ::std::clone::Clone::clone(self),
        ))
    }
}
impl ::std::convert::From<IDXGIFactory6> for IDXGIFactory4 {
    fn from(value: IDXGIFactory6) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IDXGIFactory6> for IDXGIFactory4 {
    fn from(value: &IDXGIFactory6) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::IntoParam<'a, IDXGIFactory4> for IDXGIFactory6 {
    fn into_param(self) -> ::windows::Param<'a, IDXGIFactory4> {
        ::windows::Param::Owned(::std::convert::Into::<IDXGIFactory4>::into(self))
    }
}
impl<'a> ::windows::IntoParam<'a, IDXGIFactory4> for &'a IDXGIFactory6 {
    fn into_param(self) -> ::windows::Param<'a, IDXGIFactory4> {
        ::windows::Param::Owned(::std::convert::Into::<IDXGIFactory4>::into(
            ::std::clone::Clone::clone(self),
        ))
    }
}
impl ::std::convert::From<IDXGIFactory6> for IDXGIFactory3 {
    fn from(value: IDXGIFactory6) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IDXGIFactory6> for IDXGIFactory3 {
    fn from(value: &IDXGIFactory6) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::IntoParam<'a, IDXGIFactory3> for IDXGIFactory6 {
    fn into_param(self) -> ::windows::Param<'a, IDXGIFactory3> {
        ::windows::Param::Owned(::std::convert::Into::<IDXGIFactory3>::into(self))
    }
}
impl<'a> ::windows::IntoParam<'a, IDXGIFactory3> for &'a IDXGIFactory6 {
    fn into_param(self) -> ::windows::Param<'a, IDXGIFactory3> {
        ::windows::Param::Owned(::std::convert::Into::<IDXGIFactory3>::into(
            ::std::clone::Clone::clone(self),
        ))
    }
}
impl ::std::convert::From<IDXGIFactory6> for IDXGIFactory2 {
    fn from(value: IDXGIFactory6) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IDXGIFactory6> for IDXGIFactory2 {
    fn from(value: &IDXGIFactory6) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::IntoParam<'a, IDXGIFactory2> for IDXGIFactory6 {
    fn into_param(self) -> ::windows::Param<'a, IDXGIFactory2> {
        ::windows::Param::Owned(::std::convert::Into::<IDXGIFactory2>::into(self))
    }
}
impl<'a> ::windows::IntoParam<'a, IDXGIFactory2> for &'a IDXGIFactory6 {
    fn into_param(self) -> ::windows::Param<'a, IDXGIFactory2> {
        ::windows::Param::Owned(::std::convert::Into::<IDXGIFactory2>::into(
            ::std::clone::Clone::clone(self),
        ))
    }
}
impl ::std::convert::From<IDXGIFactory6> for IDXGIFactory1 {
    fn from(value: IDXGIFactory6) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IDXGIFactory6> for IDXGIFactory1 {
    fn from(value: &IDXGIFactory6) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::IntoParam<'a, IDXGIFactory1> for IDXGIFactory6 {
    fn into_param(self) -> ::windows::Param<'a, IDXGIFactory1> {
        ::windows::Param::Owned(::std::convert::Into::<IDXGIFactory1>::into(self))
    }
}
impl<'a> ::windows::IntoParam<'a, IDXGIFactory1> for &'a IDXGIFactory6 {
    fn into_param(self) -> ::windows::Param<'a, IDXGIFactory1> {
        ::windows::Param::Owned(::std::convert::Into::<IDXGIFactory1>::into(
            ::std::clone::Clone::clone(self),
        ))
    }
}
impl ::std::convert::From<IDXGIFactory6> for IDXGIFactory {
    fn from(value: IDXGIFactory6) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IDXGIFactory6> for IDXGIFactory {
    fn from(value: &IDXGIFactory6) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::IntoParam<'a, IDXGIFactory> for IDXGIFactory6 {
    fn into_param(self) -> ::windows::Param<'a, IDXGIFactory> {
        ::windows::Param::Owned(::std::convert::Into::<IDXGIFactory>::into(self))
    }
}
impl<'a> ::windows::IntoParam<'a, IDXGIFactory> for &'a IDXGIFactory6 {
    fn into_param(self) -> ::windows::Param<'a, IDXGIFactory> {
        ::windows::Param::Owned(::std::convert::Into::<IDXGIFactory>::into(
            ::std::clone::Clone::clone(self),
        ))
    }
}
impl ::std::convert::From<IDXGIFactory6> for IDXGIObject {
    fn from(value: IDXGIFactory6) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IDXGIFactory6> for IDXGIObject {
    fn from(value: &IDXGIFactory6) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::IntoParam<'a, IDXGIObject> for IDXGIFactory6 {
    fn into_param(self) -> ::windows::Param<'a, IDXGIObject> {
        ::windows::Param::Owned(::std::convert::Into::<IDXGIObject>::into(self))
    }
}
impl<'a> ::windows::IntoParam<'a, IDXGIObject> for &'a IDXGIFactory6 {
    fn into_param(self) -> ::windows::Param<'a, IDXGIObject> {
        ::windows::Param::Owned(::std::convert::Into::<IDXGIObject>::into(
            ::std::clone::Clone::clone(self),
        ))
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDXGIFactory6_abi(
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        iid: &::windows::Guid,
        interface: *mut ::windows::RawPtr,
    ) -> ::windows::ErrorCode,
    pub unsafe extern "system" fn(this: ::windows::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::RawPtr) -> u32,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        name: *const ::windows::Guid,
        data_size: u32,
        p_data: *const ::std::ffi::c_void,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        name: *const ::windows::Guid,
        p_unknown: ::windows::RawPtr,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        name: *const ::windows::Guid,
        p_data_size: *mut u32,
        p_data: *mut ::std::ffi::c_void,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        riid: *const ::windows::Guid,
        pp_parent: *mut *mut ::std::ffi::c_void,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        adapter: u32,
        pp_adapter: *mut ::windows::RawPtr,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        window_handle: super::windows_and_messaging::HWND,
        flags: u32,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        p_window_handle: *mut super::windows_and_messaging::HWND,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        p_device: ::windows::RawPtr,
        p_desc: *mut DXGI_SWAP_CHAIN_DESC,
        pp_swap_chain: *mut ::windows::RawPtr,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        module: isize,
        pp_adapter: *mut ::windows::RawPtr,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        adapter: u32,
        pp_adapter: *mut ::windows::RawPtr,
    ) -> ::windows::ErrorCode,
    pub unsafe extern "system" fn(this: ::windows::RawPtr) -> super::system_services::BOOL,
    pub unsafe extern "system" fn(this: ::windows::RawPtr) -> super::system_services::BOOL,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        p_device: ::windows::RawPtr,
        h_wnd: super::windows_and_messaging::HWND,
        p_desc: *const DXGI_SWAP_CHAIN_DESC1,
        p_fullscreen_desc: *const DXGI_SWAP_CHAIN_FULLSCREEN_DESC,
        p_restrict_to_output: ::windows::RawPtr,
        pp_swap_chain: *mut ::windows::RawPtr,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        p_device: ::windows::RawPtr,
        p_window: ::windows::RawPtr,
        p_desc: *const DXGI_SWAP_CHAIN_DESC1,
        p_restrict_to_output: ::windows::RawPtr,
        pp_swap_chain: *mut ::windows::RawPtr,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        h_resource: super::system_services::HANDLE,
        p_luid: *mut super::kernel::LUID,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        window_handle: super::windows_and_messaging::HWND,
        w_msg: u32,
        pdw_cookie: *mut u32,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        h_event: super::system_services::HANDLE,
        pdw_cookie: *mut u32,
    ) -> ::windows::ErrorCode,
    pub unsafe extern "system" fn(this: ::windows::RawPtr, dw_cookie: u32),
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        window_handle: super::windows_and_messaging::HWND,
        w_msg: u32,
        pdw_cookie: *mut u32,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        h_event: super::system_services::HANDLE,
        pdw_cookie: *mut u32,
    ) -> ::windows::ErrorCode,
    pub unsafe extern "system" fn(this: ::windows::RawPtr, dw_cookie: u32),
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        p_device: ::windows::RawPtr,
        p_desc: *const DXGI_SWAP_CHAIN_DESC1,
        p_restrict_to_output: ::windows::RawPtr,
        pp_swap_chain: *mut ::windows::RawPtr,
    ) -> ::windows::ErrorCode,
    pub unsafe extern "system" fn(this: ::windows::RawPtr) -> u32,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        adapter_luid: super::kernel::LUID,
        riid: *const ::windows::Guid,
        ppv_adapter: *mut *mut ::std::ffi::c_void,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        riid: *const ::windows::Guid,
        ppv_adapter: *mut *mut ::std::ffi::c_void,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        feature: DXGI_FEATURE,
        p_feature_support_data: *mut ::std::ffi::c_void,
        feature_support_data_size: u32,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        adapter: u32,
        gpu_preference: DXGI_GPU_PREFERENCE,
        riid: *const ::windows::Guid,
        ppv_adapter: *mut *mut ::std::ffi::c_void,
    ) -> ::windows::ErrorCode,
);
#[repr(transparent)]
#[allow(non_camel_case_types)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct IDXGIFactory7(::windows::IUnknown);
impl IDXGIFactory7 {}
unsafe impl ::windows::Interface for IDXGIFactory7 {
    type Vtable = IDXGIFactory7_abi;
    const IID: ::windows::Guid = ::windows::Guid::from_values(
        2761322221,
        30427,
        17626,
        [132, 193, 238, 154, 122, 251, 32, 168],
    );
}
#[allow(non_snake_case)]
impl IDXGIFactory7 {
    pub unsafe fn SetPrivateData(
        &self,
        name: *const ::windows::Guid,
        data_size: u32,
        p_data: *const ::std::ffi::c_void,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).3)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(name),
            ::std::mem::transmute(data_size),
            ::std::mem::transmute(p_data),
        )
    }
    pub unsafe fn SetPrivateDataInterface<
        'a,
        T1__: ::windows::IntoParam<'a, ::windows::IUnknown>,
    >(
        &self,
        name: *const ::windows::Guid,
        p_unknown: T1__,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).4)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(name),
            p_unknown.into_param().abi(),
        )
    }
    pub unsafe fn GetPrivateData(
        &self,
        name: *const ::windows::Guid,
        p_data_size: *mut u32,
        p_data: *mut ::std::ffi::c_void,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).5)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(name),
            ::std::mem::transmute(p_data_size),
            ::std::mem::transmute(p_data),
        )
    }
    pub unsafe fn GetParent(
        &self,
        riid: *const ::windows::Guid,
        pp_parent: *mut *mut ::std::ffi::c_void,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).6)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(riid),
            ::std::mem::transmute(pp_parent),
        )
    }
    pub unsafe fn EnumAdapters(
        &self,
        adapter: u32,
        pp_adapter: *mut ::std::option::Option<IDXGIAdapter>,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).7)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(adapter),
            ::std::mem::transmute(pp_adapter),
        )
    }
    pub unsafe fn MakeWindowAssociation<
        'a,
        T0__: ::windows::IntoParam<'a, super::windows_and_messaging::HWND>,
    >(
        &self,
        window_handle: T0__,
        flags: u32,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).8)(
            ::windows::Abi::abi(self),
            window_handle.into_param().abi(),
            ::std::mem::transmute(flags),
        )
    }
    pub unsafe fn GetWindowAssociation(
        &self,
        p_window_handle: *mut super::windows_and_messaging::HWND,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).9)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(p_window_handle),
        )
    }
    pub unsafe fn CreateSwapChain<'a, T0__: ::windows::IntoParam<'a, ::windows::IUnknown>>(
        &self,
        p_device: T0__,
        p_desc: *mut DXGI_SWAP_CHAIN_DESC,
        pp_swap_chain: *mut ::std::option::Option<IDXGISwapChain>,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).10)(
            ::windows::Abi::abi(self),
            p_device.into_param().abi(),
            ::std::mem::transmute(p_desc),
            ::std::mem::transmute(pp_swap_chain),
        )
    }
    pub unsafe fn CreateSoftwareAdapter(
        &self,
        module: isize,
        pp_adapter: *mut ::std::option::Option<IDXGIAdapter>,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).11)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(module),
            ::std::mem::transmute(pp_adapter),
        )
    }
    pub unsafe fn EnumAdapters1(
        &self,
        adapter: u32,
        pp_adapter: *mut ::std::option::Option<IDXGIAdapter1>,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).12)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(adapter),
            ::std::mem::transmute(pp_adapter),
        )
    }
    pub unsafe fn IsCurrent(&self) -> super::system_services::BOOL {
        (::windows::Interface::vtable(self).13)(::windows::Abi::abi(self))
    }
    pub unsafe fn IsWindowedStereoEnabled(&self) -> super::system_services::BOOL {
        (::windows::Interface::vtable(self).14)(::windows::Abi::abi(self))
    }
    pub unsafe fn CreateSwapChainForHwnd<
        'a,
        T0__: ::windows::IntoParam<'a, ::windows::IUnknown>,
        T1__: ::windows::IntoParam<'a, super::windows_and_messaging::HWND>,
        T4__: ::windows::IntoParam<'a, IDXGIOutput>,
    >(
        &self,
        p_device: T0__,
        h_wnd: T1__,
        p_desc: *const DXGI_SWAP_CHAIN_DESC1,
        p_fullscreen_desc: *const DXGI_SWAP_CHAIN_FULLSCREEN_DESC,
        p_restrict_to_output: T4__,
        pp_swap_chain: *mut ::std::option::Option<IDXGISwapChain1>,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).15)(
            ::windows::Abi::abi(self),
            p_device.into_param().abi(),
            h_wnd.into_param().abi(),
            ::std::mem::transmute(p_desc),
            ::std::mem::transmute(p_fullscreen_desc),
            p_restrict_to_output.into_param().abi(),
            ::std::mem::transmute(pp_swap_chain),
        )
    }
    pub unsafe fn CreateSwapChainForCoreWindow<
        'a,
        T0__: ::windows::IntoParam<'a, ::windows::IUnknown>,
        T1__: ::windows::IntoParam<'a, ::windows::IUnknown>,
        T3__: ::windows::IntoParam<'a, IDXGIOutput>,
    >(
        &self,
        p_device: T0__,
        p_window: T1__,
        p_desc: *const DXGI_SWAP_CHAIN_DESC1,
        p_restrict_to_output: T3__,
        pp_swap_chain: *mut ::std::option::Option<IDXGISwapChain1>,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).16)(
            ::windows::Abi::abi(self),
            p_device.into_param().abi(),
            p_window.into_param().abi(),
            ::std::mem::transmute(p_desc),
            p_restrict_to_output.into_param().abi(),
            ::std::mem::transmute(pp_swap_chain),
        )
    }
    pub unsafe fn GetSharedResourceAdapterLuid<
        'a,
        T0__: ::windows::IntoParam<'a, super::system_services::HANDLE>,
    >(
        &self,
        h_resource: T0__,
        p_luid: *mut super::kernel::LUID,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).17)(
            ::windows::Abi::abi(self),
            h_resource.into_param().abi(),
            ::std::mem::transmute(p_luid),
        )
    }
    pub unsafe fn RegisterStereoStatusWindow<
        'a,
        T0__: ::windows::IntoParam<'a, super::windows_and_messaging::HWND>,
    >(
        &self,
        window_handle: T0__,
        w_msg: u32,
        pdw_cookie: *mut u32,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).18)(
            ::windows::Abi::abi(self),
            window_handle.into_param().abi(),
            ::std::mem::transmute(w_msg),
            ::std::mem::transmute(pdw_cookie),
        )
    }
    pub unsafe fn RegisterStereoStatusEvent<
        'a,
        T0__: ::windows::IntoParam<'a, super::system_services::HANDLE>,
    >(
        &self,
        h_event: T0__,
        pdw_cookie: *mut u32,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).19)(
            ::windows::Abi::abi(self),
            h_event.into_param().abi(),
            ::std::mem::transmute(pdw_cookie),
        )
    }
    pub unsafe fn UnregisterStereoStatus(&self, dw_cookie: u32) {
        (::windows::Interface::vtable(self).20)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(dw_cookie),
        )
    }
    pub unsafe fn RegisterOcclusionStatusWindow<
        'a,
        T0__: ::windows::IntoParam<'a, super::windows_and_messaging::HWND>,
    >(
        &self,
        window_handle: T0__,
        w_msg: u32,
        pdw_cookie: *mut u32,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).21)(
            ::windows::Abi::abi(self),
            window_handle.into_param().abi(),
            ::std::mem::transmute(w_msg),
            ::std::mem::transmute(pdw_cookie),
        )
    }
    pub unsafe fn RegisterOcclusionStatusEvent<
        'a,
        T0__: ::windows::IntoParam<'a, super::system_services::HANDLE>,
    >(
        &self,
        h_event: T0__,
        pdw_cookie: *mut u32,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).22)(
            ::windows::Abi::abi(self),
            h_event.into_param().abi(),
            ::std::mem::transmute(pdw_cookie),
        )
    }
    pub unsafe fn UnregisterOcclusionStatus(&self, dw_cookie: u32) {
        (::windows::Interface::vtable(self).23)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(dw_cookie),
        )
    }
    pub unsafe fn CreateSwapChainForComposition<
        'a,
        T0__: ::windows::IntoParam<'a, ::windows::IUnknown>,
        T2__: ::windows::IntoParam<'a, IDXGIOutput>,
    >(
        &self,
        p_device: T0__,
        p_desc: *const DXGI_SWAP_CHAIN_DESC1,
        p_restrict_to_output: T2__,
        pp_swap_chain: *mut ::std::option::Option<IDXGISwapChain1>,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).24)(
            ::windows::Abi::abi(self),
            p_device.into_param().abi(),
            ::std::mem::transmute(p_desc),
            p_restrict_to_output.into_param().abi(),
            ::std::mem::transmute(pp_swap_chain),
        )
    }
    pub unsafe fn GetCreationFlags(&self) -> u32 {
        (::windows::Interface::vtable(self).25)(::windows::Abi::abi(self))
    }
    pub unsafe fn EnumAdapterByLuid<'a, T0__: ::windows::IntoParam<'a, super::kernel::LUID>>(
        &self,
        adapter_luid: T0__,
        riid: *const ::windows::Guid,
        ppv_adapter: *mut *mut ::std::ffi::c_void,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).26)(
            ::windows::Abi::abi(self),
            adapter_luid.into_param().abi(),
            ::std::mem::transmute(riid),
            ::std::mem::transmute(ppv_adapter),
        )
    }
    pub unsafe fn EnumWarpAdapter(
        &self,
        riid: *const ::windows::Guid,
        ppv_adapter: *mut *mut ::std::ffi::c_void,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).27)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(riid),
            ::std::mem::transmute(ppv_adapter),
        )
    }
    pub unsafe fn CheckFeatureSupport(
        &self,
        feature: DXGI_FEATURE,
        p_feature_support_data: *mut ::std::ffi::c_void,
        feature_support_data_size: u32,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).28)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(feature),
            ::std::mem::transmute(p_feature_support_data),
            ::std::mem::transmute(feature_support_data_size),
        )
    }
    pub unsafe fn EnumAdapterByGpuPreference(
        &self,
        adapter: u32,
        gpu_preference: DXGI_GPU_PREFERENCE,
        riid: *const ::windows::Guid,
        ppv_adapter: *mut *mut ::std::ffi::c_void,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).29)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(adapter),
            ::std::mem::transmute(gpu_preference),
            ::std::mem::transmute(riid),
            ::std::mem::transmute(ppv_adapter),
        )
    }
    pub unsafe fn RegisterAdaptersChangedEvent<
        'a,
        T0__: ::windows::IntoParam<'a, super::system_services::HANDLE>,
    >(
        &self,
        h_event: T0__,
        pdw_cookie: *mut u32,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).30)(
            ::windows::Abi::abi(self),
            h_event.into_param().abi(),
            ::std::mem::transmute(pdw_cookie),
        )
    }
    pub unsafe fn UnregisterAdaptersChangedEvent(&self, dw_cookie: u32) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).31)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(dw_cookie),
        )
    }
}
impl ::std::convert::From<IDXGIFactory7> for ::windows::IUnknown {
    fn from(value: IDXGIFactory7) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IDXGIFactory7> for ::windows::IUnknown {
    fn from(value: &IDXGIFactory7) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::IntoParam<'a, ::windows::IUnknown> for IDXGIFactory7 {
    fn into_param(self) -> ::windows::Param<'a, ::windows::IUnknown> {
        ::windows::Param::Owned(::std::convert::Into::<::windows::IUnknown>::into(self))
    }
}
impl<'a> ::windows::IntoParam<'a, ::windows::IUnknown> for &'a IDXGIFactory7 {
    fn into_param(self) -> ::windows::Param<'a, ::windows::IUnknown> {
        ::windows::Param::Owned(::std::convert::Into::<::windows::IUnknown>::into(
            ::std::clone::Clone::clone(self),
        ))
    }
}
impl ::std::convert::From<IDXGIFactory7> for IDXGIFactory6 {
    fn from(value: IDXGIFactory7) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IDXGIFactory7> for IDXGIFactory6 {
    fn from(value: &IDXGIFactory7) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::IntoParam<'a, IDXGIFactory6> for IDXGIFactory7 {
    fn into_param(self) -> ::windows::Param<'a, IDXGIFactory6> {
        ::windows::Param::Owned(::std::convert::Into::<IDXGIFactory6>::into(self))
    }
}
impl<'a> ::windows::IntoParam<'a, IDXGIFactory6> for &'a IDXGIFactory7 {
    fn into_param(self) -> ::windows::Param<'a, IDXGIFactory6> {
        ::windows::Param::Owned(::std::convert::Into::<IDXGIFactory6>::into(
            ::std::clone::Clone::clone(self),
        ))
    }
}
impl ::std::convert::From<IDXGIFactory7> for IDXGIFactory5 {
    fn from(value: IDXGIFactory7) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IDXGIFactory7> for IDXGIFactory5 {
    fn from(value: &IDXGIFactory7) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::IntoParam<'a, IDXGIFactory5> for IDXGIFactory7 {
    fn into_param(self) -> ::windows::Param<'a, IDXGIFactory5> {
        ::windows::Param::Owned(::std::convert::Into::<IDXGIFactory5>::into(self))
    }
}
impl<'a> ::windows::IntoParam<'a, IDXGIFactory5> for &'a IDXGIFactory7 {
    fn into_param(self) -> ::windows::Param<'a, IDXGIFactory5> {
        ::windows::Param::Owned(::std::convert::Into::<IDXGIFactory5>::into(
            ::std::clone::Clone::clone(self),
        ))
    }
}
impl ::std::convert::From<IDXGIFactory7> for IDXGIFactory4 {
    fn from(value: IDXGIFactory7) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IDXGIFactory7> for IDXGIFactory4 {
    fn from(value: &IDXGIFactory7) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::IntoParam<'a, IDXGIFactory4> for IDXGIFactory7 {
    fn into_param(self) -> ::windows::Param<'a, IDXGIFactory4> {
        ::windows::Param::Owned(::std::convert::Into::<IDXGIFactory4>::into(self))
    }
}
impl<'a> ::windows::IntoParam<'a, IDXGIFactory4> for &'a IDXGIFactory7 {
    fn into_param(self) -> ::windows::Param<'a, IDXGIFactory4> {
        ::windows::Param::Owned(::std::convert::Into::<IDXGIFactory4>::into(
            ::std::clone::Clone::clone(self),
        ))
    }
}
impl ::std::convert::From<IDXGIFactory7> for IDXGIFactory3 {
    fn from(value: IDXGIFactory7) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IDXGIFactory7> for IDXGIFactory3 {
    fn from(value: &IDXGIFactory7) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::IntoParam<'a, IDXGIFactory3> for IDXGIFactory7 {
    fn into_param(self) -> ::windows::Param<'a, IDXGIFactory3> {
        ::windows::Param::Owned(::std::convert::Into::<IDXGIFactory3>::into(self))
    }
}
impl<'a> ::windows::IntoParam<'a, IDXGIFactory3> for &'a IDXGIFactory7 {
    fn into_param(self) -> ::windows::Param<'a, IDXGIFactory3> {
        ::windows::Param::Owned(::std::convert::Into::<IDXGIFactory3>::into(
            ::std::clone::Clone::clone(self),
        ))
    }
}
impl ::std::convert::From<IDXGIFactory7> for IDXGIFactory2 {
    fn from(value: IDXGIFactory7) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IDXGIFactory7> for IDXGIFactory2 {
    fn from(value: &IDXGIFactory7) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::IntoParam<'a, IDXGIFactory2> for IDXGIFactory7 {
    fn into_param(self) -> ::windows::Param<'a, IDXGIFactory2> {
        ::windows::Param::Owned(::std::convert::Into::<IDXGIFactory2>::into(self))
    }
}
impl<'a> ::windows::IntoParam<'a, IDXGIFactory2> for &'a IDXGIFactory7 {
    fn into_param(self) -> ::windows::Param<'a, IDXGIFactory2> {
        ::windows::Param::Owned(::std::convert::Into::<IDXGIFactory2>::into(
            ::std::clone::Clone::clone(self),
        ))
    }
}
impl ::std::convert::From<IDXGIFactory7> for IDXGIFactory1 {
    fn from(value: IDXGIFactory7) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IDXGIFactory7> for IDXGIFactory1 {
    fn from(value: &IDXGIFactory7) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::IntoParam<'a, IDXGIFactory1> for IDXGIFactory7 {
    fn into_param(self) -> ::windows::Param<'a, IDXGIFactory1> {
        ::windows::Param::Owned(::std::convert::Into::<IDXGIFactory1>::into(self))
    }
}
impl<'a> ::windows::IntoParam<'a, IDXGIFactory1> for &'a IDXGIFactory7 {
    fn into_param(self) -> ::windows::Param<'a, IDXGIFactory1> {
        ::windows::Param::Owned(::std::convert::Into::<IDXGIFactory1>::into(
            ::std::clone::Clone::clone(self),
        ))
    }
}
impl ::std::convert::From<IDXGIFactory7> for IDXGIFactory {
    fn from(value: IDXGIFactory7) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IDXGIFactory7> for IDXGIFactory {
    fn from(value: &IDXGIFactory7) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::IntoParam<'a, IDXGIFactory> for IDXGIFactory7 {
    fn into_param(self) -> ::windows::Param<'a, IDXGIFactory> {
        ::windows::Param::Owned(::std::convert::Into::<IDXGIFactory>::into(self))
    }
}
impl<'a> ::windows::IntoParam<'a, IDXGIFactory> for &'a IDXGIFactory7 {
    fn into_param(self) -> ::windows::Param<'a, IDXGIFactory> {
        ::windows::Param::Owned(::std::convert::Into::<IDXGIFactory>::into(
            ::std::clone::Clone::clone(self),
        ))
    }
}
impl ::std::convert::From<IDXGIFactory7> for IDXGIObject {
    fn from(value: IDXGIFactory7) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IDXGIFactory7> for IDXGIObject {
    fn from(value: &IDXGIFactory7) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::IntoParam<'a, IDXGIObject> for IDXGIFactory7 {
    fn into_param(self) -> ::windows::Param<'a, IDXGIObject> {
        ::windows::Param::Owned(::std::convert::Into::<IDXGIObject>::into(self))
    }
}
impl<'a> ::windows::IntoParam<'a, IDXGIObject> for &'a IDXGIFactory7 {
    fn into_param(self) -> ::windows::Param<'a, IDXGIObject> {
        ::windows::Param::Owned(::std::convert::Into::<IDXGIObject>::into(
            ::std::clone::Clone::clone(self),
        ))
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDXGIFactory7_abi(
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        iid: &::windows::Guid,
        interface: *mut ::windows::RawPtr,
    ) -> ::windows::ErrorCode,
    pub unsafe extern "system" fn(this: ::windows::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::RawPtr) -> u32,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        name: *const ::windows::Guid,
        data_size: u32,
        p_data: *const ::std::ffi::c_void,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        name: *const ::windows::Guid,
        p_unknown: ::windows::RawPtr,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        name: *const ::windows::Guid,
        p_data_size: *mut u32,
        p_data: *mut ::std::ffi::c_void,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        riid: *const ::windows::Guid,
        pp_parent: *mut *mut ::std::ffi::c_void,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        adapter: u32,
        pp_adapter: *mut ::windows::RawPtr,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        window_handle: super::windows_and_messaging::HWND,
        flags: u32,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        p_window_handle: *mut super::windows_and_messaging::HWND,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        p_device: ::windows::RawPtr,
        p_desc: *mut DXGI_SWAP_CHAIN_DESC,
        pp_swap_chain: *mut ::windows::RawPtr,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        module: isize,
        pp_adapter: *mut ::windows::RawPtr,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        adapter: u32,
        pp_adapter: *mut ::windows::RawPtr,
    ) -> ::windows::ErrorCode,
    pub unsafe extern "system" fn(this: ::windows::RawPtr) -> super::system_services::BOOL,
    pub unsafe extern "system" fn(this: ::windows::RawPtr) -> super::system_services::BOOL,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        p_device: ::windows::RawPtr,
        h_wnd: super::windows_and_messaging::HWND,
        p_desc: *const DXGI_SWAP_CHAIN_DESC1,
        p_fullscreen_desc: *const DXGI_SWAP_CHAIN_FULLSCREEN_DESC,
        p_restrict_to_output: ::windows::RawPtr,
        pp_swap_chain: *mut ::windows::RawPtr,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        p_device: ::windows::RawPtr,
        p_window: ::windows::RawPtr,
        p_desc: *const DXGI_SWAP_CHAIN_DESC1,
        p_restrict_to_output: ::windows::RawPtr,
        pp_swap_chain: *mut ::windows::RawPtr,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        h_resource: super::system_services::HANDLE,
        p_luid: *mut super::kernel::LUID,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        window_handle: super::windows_and_messaging::HWND,
        w_msg: u32,
        pdw_cookie: *mut u32,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        h_event: super::system_services::HANDLE,
        pdw_cookie: *mut u32,
    ) -> ::windows::ErrorCode,
    pub unsafe extern "system" fn(this: ::windows::RawPtr, dw_cookie: u32),
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        window_handle: super::windows_and_messaging::HWND,
        w_msg: u32,
        pdw_cookie: *mut u32,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        h_event: super::system_services::HANDLE,
        pdw_cookie: *mut u32,
    ) -> ::windows::ErrorCode,
    pub unsafe extern "system" fn(this: ::windows::RawPtr, dw_cookie: u32),
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        p_device: ::windows::RawPtr,
        p_desc: *const DXGI_SWAP_CHAIN_DESC1,
        p_restrict_to_output: ::windows::RawPtr,
        pp_swap_chain: *mut ::windows::RawPtr,
    ) -> ::windows::ErrorCode,
    pub unsafe extern "system" fn(this: ::windows::RawPtr) -> u32,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        adapter_luid: super::kernel::LUID,
        riid: *const ::windows::Guid,
        ppv_adapter: *mut *mut ::std::ffi::c_void,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        riid: *const ::windows::Guid,
        ppv_adapter: *mut *mut ::std::ffi::c_void,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        feature: DXGI_FEATURE,
        p_feature_support_data: *mut ::std::ffi::c_void,
        feature_support_data_size: u32,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        adapter: u32,
        gpu_preference: DXGI_GPU_PREFERENCE,
        riid: *const ::windows::Guid,
        ppv_adapter: *mut *mut ::std::ffi::c_void,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        h_event: super::system_services::HANDLE,
        pdw_cookie: *mut u32,
    ) -> ::windows::ErrorCode,
    pub unsafe extern "system" fn(this: ::windows::RawPtr, dw_cookie: u32) -> ::windows::ErrorCode,
);
#[repr(transparent)]
#[allow(non_camel_case_types)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct IDXGIFactoryMedia(::windows::IUnknown);
impl IDXGIFactoryMedia {}
unsafe impl ::windows::Interface for IDXGIFactoryMedia {
    type Vtable = IDXGIFactoryMedia_abi;
    const IID: ::windows::Guid = ::windows::Guid::from_values(
        1105711602,
        42385,
        20347,
        [162, 229, 250, 156, 132, 62, 28, 18],
    );
}
#[allow(non_snake_case)]
impl IDXGIFactoryMedia {
    pub unsafe fn CreateSwapChainForCompositionSurfaceHandle<
        'a,
        T0__: ::windows::IntoParam<'a, ::windows::IUnknown>,
        T1__: ::windows::IntoParam<'a, super::system_services::HANDLE>,
        T3__: ::windows::IntoParam<'a, IDXGIOutput>,
    >(
        &self,
        p_device: T0__,
        h_surface: T1__,
        p_desc: *const DXGI_SWAP_CHAIN_DESC1,
        p_restrict_to_output: T3__,
        pp_swap_chain: *mut ::std::option::Option<IDXGISwapChain1>,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).3)(
            ::windows::Abi::abi(self),
            p_device.into_param().abi(),
            h_surface.into_param().abi(),
            ::std::mem::transmute(p_desc),
            p_restrict_to_output.into_param().abi(),
            ::std::mem::transmute(pp_swap_chain),
        )
    }
    pub unsafe fn CreateDecodeSwapChainForCompositionSurfaceHandle<
        'a,
        T0__: ::windows::IntoParam<'a, ::windows::IUnknown>,
        T1__: ::windows::IntoParam<'a, super::system_services::HANDLE>,
        T3__: ::windows::IntoParam<'a, IDXGIResource>,
        T4__: ::windows::IntoParam<'a, IDXGIOutput>,
    >(
        &self,
        p_device: T0__,
        h_surface: T1__,
        p_desc: *mut DXGI_DECODE_SWAP_CHAIN_DESC,
        p_yuv_decode_buffers: T3__,
        p_restrict_to_output: T4__,
        pp_swap_chain: *mut ::std::option::Option<IDXGIDecodeSwapChain>,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).4)(
            ::windows::Abi::abi(self),
            p_device.into_param().abi(),
            h_surface.into_param().abi(),
            ::std::mem::transmute(p_desc),
            p_yuv_decode_buffers.into_param().abi(),
            p_restrict_to_output.into_param().abi(),
            ::std::mem::transmute(pp_swap_chain),
        )
    }
}
impl ::std::convert::From<IDXGIFactoryMedia> for ::windows::IUnknown {
    fn from(value: IDXGIFactoryMedia) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IDXGIFactoryMedia> for ::windows::IUnknown {
    fn from(value: &IDXGIFactoryMedia) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::IntoParam<'a, ::windows::IUnknown> for IDXGIFactoryMedia {
    fn into_param(self) -> ::windows::Param<'a, ::windows::IUnknown> {
        ::windows::Param::Owned(::std::convert::Into::<::windows::IUnknown>::into(self))
    }
}
impl<'a> ::windows::IntoParam<'a, ::windows::IUnknown> for &'a IDXGIFactoryMedia {
    fn into_param(self) -> ::windows::Param<'a, ::windows::IUnknown> {
        ::windows::Param::Owned(::std::convert::Into::<::windows::IUnknown>::into(
            ::std::clone::Clone::clone(self),
        ))
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDXGIFactoryMedia_abi(
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        iid: &::windows::Guid,
        interface: *mut ::windows::RawPtr,
    ) -> ::windows::ErrorCode,
    pub unsafe extern "system" fn(this: ::windows::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::RawPtr) -> u32,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        p_device: ::windows::RawPtr,
        h_surface: super::system_services::HANDLE,
        p_desc: *const DXGI_SWAP_CHAIN_DESC1,
        p_restrict_to_output: ::windows::RawPtr,
        pp_swap_chain: *mut ::windows::RawPtr,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        p_device: ::windows::RawPtr,
        h_surface: super::system_services::HANDLE,
        p_desc: *mut DXGI_DECODE_SWAP_CHAIN_DESC,
        p_yuv_decode_buffers: ::windows::RawPtr,
        p_restrict_to_output: ::windows::RawPtr,
        pp_swap_chain: *mut ::windows::RawPtr,
    ) -> ::windows::ErrorCode,
);
#[repr(transparent)]
#[allow(non_camel_case_types)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct IDXGIInfoQueue(::windows::IUnknown);
impl IDXGIInfoQueue {}
unsafe impl ::windows::Interface for IDXGIInfoQueue {
    type Vtable = IDXGIInfoQueue_abi;
    const IID: ::windows::Guid = ::windows::Guid::from_values(
        3597943239,
        26410,
        18287,
        [158, 130, 205, 85, 180, 73, 73, 206],
    );
}
#[allow(non_snake_case)]
impl IDXGIInfoQueue {
    pub unsafe fn SetMessageCountLimit<'a, T0__: ::windows::IntoParam<'a, ::windows::Guid>>(
        &self,
        producer: T0__,
        message_count_limit: u64,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).3)(
            ::windows::Abi::abi(self),
            producer.into_param().abi(),
            ::std::mem::transmute(message_count_limit),
        )
    }
    pub unsafe fn ClearStoredMessages<'a, T0__: ::windows::IntoParam<'a, ::windows::Guid>>(
        &self,
        producer: T0__,
    ) {
        (::windows::Interface::vtable(self).4)(
            ::windows::Abi::abi(self),
            producer.into_param().abi(),
        )
    }
    pub unsafe fn GetMessageA<'a, T0__: ::windows::IntoParam<'a, ::windows::Guid>>(
        &self,
        producer: T0__,
        message_index: u64,
        p_message: *mut DXGI_INFO_QUEUE_MESSAGE,
        p_message_byte_length: *mut usize,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).5)(
            ::windows::Abi::abi(self),
            producer.into_param().abi(),
            ::std::mem::transmute(message_index),
            ::std::mem::transmute(p_message),
            ::std::mem::transmute(p_message_byte_length),
        )
    }
    pub unsafe fn GetNumStoredMessagesAllowedByRetrievalFilters<
        'a,
        T0__: ::windows::IntoParam<'a, ::windows::Guid>,
    >(
        &self,
        producer: T0__,
    ) -> u64 {
        (::windows::Interface::vtable(self).6)(
            ::windows::Abi::abi(self),
            producer.into_param().abi(),
        )
    }
    pub unsafe fn GetNumStoredMessages<'a, T0__: ::windows::IntoParam<'a, ::windows::Guid>>(
        &self,
        producer: T0__,
    ) -> u64 {
        (::windows::Interface::vtable(self).7)(
            ::windows::Abi::abi(self),
            producer.into_param().abi(),
        )
    }
    pub unsafe fn GetNumMessagesDiscardedByMessageCountLimit<
        'a,
        T0__: ::windows::IntoParam<'a, ::windows::Guid>,
    >(
        &self,
        producer: T0__,
    ) -> u64 {
        (::windows::Interface::vtable(self).8)(
            ::windows::Abi::abi(self),
            producer.into_param().abi(),
        )
    }
    pub unsafe fn GetMessageCountLimit<'a, T0__: ::windows::IntoParam<'a, ::windows::Guid>>(
        &self,
        producer: T0__,
    ) -> u64 {
        (::windows::Interface::vtable(self).9)(
            ::windows::Abi::abi(self),
            producer.into_param().abi(),
        )
    }
    pub unsafe fn GetNumMessagesAllowedByStorageFilter<
        'a,
        T0__: ::windows::IntoParam<'a, ::windows::Guid>,
    >(
        &self,
        producer: T0__,
    ) -> u64 {
        (::windows::Interface::vtable(self).10)(
            ::windows::Abi::abi(self),
            producer.into_param().abi(),
        )
    }
    pub unsafe fn GetNumMessagesDeniedByStorageFilter<
        'a,
        T0__: ::windows::IntoParam<'a, ::windows::Guid>,
    >(
        &self,
        producer: T0__,
    ) -> u64 {
        (::windows::Interface::vtable(self).11)(
            ::windows::Abi::abi(self),
            producer.into_param().abi(),
        )
    }
    pub unsafe fn AddStorageFilterEntries<'a, T0__: ::windows::IntoParam<'a, ::windows::Guid>>(
        &self,
        producer: T0__,
        p_filter: *mut DXGI_INFO_QUEUE_FILTER,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).12)(
            ::windows::Abi::abi(self),
            producer.into_param().abi(),
            ::std::mem::transmute(p_filter),
        )
    }
    pub unsafe fn GetStorageFilter<'a, T0__: ::windows::IntoParam<'a, ::windows::Guid>>(
        &self,
        producer: T0__,
        p_filter: *mut DXGI_INFO_QUEUE_FILTER,
        p_filter_byte_length: *mut usize,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).13)(
            ::windows::Abi::abi(self),
            producer.into_param().abi(),
            ::std::mem::transmute(p_filter),
            ::std::mem::transmute(p_filter_byte_length),
        )
    }
    pub unsafe fn ClearStorageFilter<'a, T0__: ::windows::IntoParam<'a, ::windows::Guid>>(
        &self,
        producer: T0__,
    ) {
        (::windows::Interface::vtable(self).14)(
            ::windows::Abi::abi(self),
            producer.into_param().abi(),
        )
    }
    pub unsafe fn PushEmptyStorageFilter<'a, T0__: ::windows::IntoParam<'a, ::windows::Guid>>(
        &self,
        producer: T0__,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).15)(
            ::windows::Abi::abi(self),
            producer.into_param().abi(),
        )
    }
    pub unsafe fn PushDenyAllStorageFilter<'a, T0__: ::windows::IntoParam<'a, ::windows::Guid>>(
        &self,
        producer: T0__,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).16)(
            ::windows::Abi::abi(self),
            producer.into_param().abi(),
        )
    }
    pub unsafe fn PushCopyOfStorageFilter<'a, T0__: ::windows::IntoParam<'a, ::windows::Guid>>(
        &self,
        producer: T0__,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).17)(
            ::windows::Abi::abi(self),
            producer.into_param().abi(),
        )
    }
    pub unsafe fn PushStorageFilter<'a, T0__: ::windows::IntoParam<'a, ::windows::Guid>>(
        &self,
        producer: T0__,
        p_filter: *mut DXGI_INFO_QUEUE_FILTER,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).18)(
            ::windows::Abi::abi(self),
            producer.into_param().abi(),
            ::std::mem::transmute(p_filter),
        )
    }
    pub unsafe fn PopStorageFilter<'a, T0__: ::windows::IntoParam<'a, ::windows::Guid>>(
        &self,
        producer: T0__,
    ) {
        (::windows::Interface::vtable(self).19)(
            ::windows::Abi::abi(self),
            producer.into_param().abi(),
        )
    }
    pub unsafe fn GetStorageFilterStackSize<'a, T0__: ::windows::IntoParam<'a, ::windows::Guid>>(
        &self,
        producer: T0__,
    ) -> u32 {
        (::windows::Interface::vtable(self).20)(
            ::windows::Abi::abi(self),
            producer.into_param().abi(),
        )
    }
    pub unsafe fn AddRetrievalFilterEntries<'a, T0__: ::windows::IntoParam<'a, ::windows::Guid>>(
        &self,
        producer: T0__,
        p_filter: *mut DXGI_INFO_QUEUE_FILTER,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).21)(
            ::windows::Abi::abi(self),
            producer.into_param().abi(),
            ::std::mem::transmute(p_filter),
        )
    }
    pub unsafe fn GetRetrievalFilter<'a, T0__: ::windows::IntoParam<'a, ::windows::Guid>>(
        &self,
        producer: T0__,
        p_filter: *mut DXGI_INFO_QUEUE_FILTER,
        p_filter_byte_length: *mut usize,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).22)(
            ::windows::Abi::abi(self),
            producer.into_param().abi(),
            ::std::mem::transmute(p_filter),
            ::std::mem::transmute(p_filter_byte_length),
        )
    }
    pub unsafe fn ClearRetrievalFilter<'a, T0__: ::windows::IntoParam<'a, ::windows::Guid>>(
        &self,
        producer: T0__,
    ) {
        (::windows::Interface::vtable(self).23)(
            ::windows::Abi::abi(self),
            producer.into_param().abi(),
        )
    }
    pub unsafe fn PushEmptyRetrievalFilter<'a, T0__: ::windows::IntoParam<'a, ::windows::Guid>>(
        &self,
        producer: T0__,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).24)(
            ::windows::Abi::abi(self),
            producer.into_param().abi(),
        )
    }
    pub unsafe fn PushDenyAllRetrievalFilter<
        'a,
        T0__: ::windows::IntoParam<'a, ::windows::Guid>,
    >(
        &self,
        producer: T0__,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).25)(
            ::windows::Abi::abi(self),
            producer.into_param().abi(),
        )
    }
    pub unsafe fn PushCopyOfRetrievalFilter<'a, T0__: ::windows::IntoParam<'a, ::windows::Guid>>(
        &self,
        producer: T0__,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).26)(
            ::windows::Abi::abi(self),
            producer.into_param().abi(),
        )
    }
    pub unsafe fn PushRetrievalFilter<'a, T0__: ::windows::IntoParam<'a, ::windows::Guid>>(
        &self,
        producer: T0__,
        p_filter: *mut DXGI_INFO_QUEUE_FILTER,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).27)(
            ::windows::Abi::abi(self),
            producer.into_param().abi(),
            ::std::mem::transmute(p_filter),
        )
    }
    pub unsafe fn PopRetrievalFilter<'a, T0__: ::windows::IntoParam<'a, ::windows::Guid>>(
        &self,
        producer: T0__,
    ) {
        (::windows::Interface::vtable(self).28)(
            ::windows::Abi::abi(self),
            producer.into_param().abi(),
        )
    }
    pub unsafe fn GetRetrievalFilterStackSize<
        'a,
        T0__: ::windows::IntoParam<'a, ::windows::Guid>,
    >(
        &self,
        producer: T0__,
    ) -> u32 {
        (::windows::Interface::vtable(self).29)(
            ::windows::Abi::abi(self),
            producer.into_param().abi(),
        )
    }
    pub unsafe fn AddMessage<
        'a,
        T0__: ::windows::IntoParam<'a, ::windows::Guid>,
        T4__: ::windows::IntoParam<'a, super::system_services::PSTR>,
    >(
        &self,
        producer: T0__,
        category: DXGI_INFO_QUEUE_MESSAGE_CATEGORY,
        severity: DXGI_INFO_QUEUE_MESSAGE_SEVERITY,
        id: i32,
        p_description: T4__,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).30)(
            ::windows::Abi::abi(self),
            producer.into_param().abi(),
            ::std::mem::transmute(category),
            ::std::mem::transmute(severity),
            ::std::mem::transmute(id),
            p_description.into_param().abi(),
        )
    }
    pub unsafe fn AddApplicationMessage<
        'a,
        T1__: ::windows::IntoParam<'a, super::system_services::PSTR>,
    >(
        &self,
        severity: DXGI_INFO_QUEUE_MESSAGE_SEVERITY,
        p_description: T1__,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).31)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(severity),
            p_description.into_param().abi(),
        )
    }
    pub unsafe fn SetBreakOnCategory<
        'a,
        T0__: ::windows::IntoParam<'a, ::windows::Guid>,
        T2__: ::windows::IntoParam<'a, super::system_services::BOOL>,
    >(
        &self,
        producer: T0__,
        category: DXGI_INFO_QUEUE_MESSAGE_CATEGORY,
        b_enable: T2__,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).32)(
            ::windows::Abi::abi(self),
            producer.into_param().abi(),
            ::std::mem::transmute(category),
            b_enable.into_param().abi(),
        )
    }
    pub unsafe fn SetBreakOnSeverity<
        'a,
        T0__: ::windows::IntoParam<'a, ::windows::Guid>,
        T2__: ::windows::IntoParam<'a, super::system_services::BOOL>,
    >(
        &self,
        producer: T0__,
        severity: DXGI_INFO_QUEUE_MESSAGE_SEVERITY,
        b_enable: T2__,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).33)(
            ::windows::Abi::abi(self),
            producer.into_param().abi(),
            ::std::mem::transmute(severity),
            b_enable.into_param().abi(),
        )
    }
    pub unsafe fn SetBreakOnID<
        'a,
        T0__: ::windows::IntoParam<'a, ::windows::Guid>,
        T2__: ::windows::IntoParam<'a, super::system_services::BOOL>,
    >(
        &self,
        producer: T0__,
        id: i32,
        b_enable: T2__,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).34)(
            ::windows::Abi::abi(self),
            producer.into_param().abi(),
            ::std::mem::transmute(id),
            b_enable.into_param().abi(),
        )
    }
    pub unsafe fn GetBreakOnCategory<'a, T0__: ::windows::IntoParam<'a, ::windows::Guid>>(
        &self,
        producer: T0__,
        category: DXGI_INFO_QUEUE_MESSAGE_CATEGORY,
    ) -> super::system_services::BOOL {
        (::windows::Interface::vtable(self).35)(
            ::windows::Abi::abi(self),
            producer.into_param().abi(),
            ::std::mem::transmute(category),
        )
    }
    pub unsafe fn GetBreakOnSeverity<'a, T0__: ::windows::IntoParam<'a, ::windows::Guid>>(
        &self,
        producer: T0__,
        severity: DXGI_INFO_QUEUE_MESSAGE_SEVERITY,
    ) -> super::system_services::BOOL {
        (::windows::Interface::vtable(self).36)(
            ::windows::Abi::abi(self),
            producer.into_param().abi(),
            ::std::mem::transmute(severity),
        )
    }
    pub unsafe fn GetBreakOnID<'a, T0__: ::windows::IntoParam<'a, ::windows::Guid>>(
        &self,
        producer: T0__,
        id: i32,
    ) -> super::system_services::BOOL {
        (::windows::Interface::vtable(self).37)(
            ::windows::Abi::abi(self),
            producer.into_param().abi(),
            ::std::mem::transmute(id),
        )
    }
    pub unsafe fn SetMuteDebugOutput<
        'a,
        T0__: ::windows::IntoParam<'a, ::windows::Guid>,
        T1__: ::windows::IntoParam<'a, super::system_services::BOOL>,
    >(
        &self,
        producer: T0__,
        b_mute: T1__,
    ) {
        (::windows::Interface::vtable(self).38)(
            ::windows::Abi::abi(self),
            producer.into_param().abi(),
            b_mute.into_param().abi(),
        )
    }
    pub unsafe fn GetMuteDebugOutput<'a, T0__: ::windows::IntoParam<'a, ::windows::Guid>>(
        &self,
        producer: T0__,
    ) -> super::system_services::BOOL {
        (::windows::Interface::vtable(self).39)(
            ::windows::Abi::abi(self),
            producer.into_param().abi(),
        )
    }
}
impl ::std::convert::From<IDXGIInfoQueue> for ::windows::IUnknown {
    fn from(value: IDXGIInfoQueue) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IDXGIInfoQueue> for ::windows::IUnknown {
    fn from(value: &IDXGIInfoQueue) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::IntoParam<'a, ::windows::IUnknown> for IDXGIInfoQueue {
    fn into_param(self) -> ::windows::Param<'a, ::windows::IUnknown> {
        ::windows::Param::Owned(::std::convert::Into::<::windows::IUnknown>::into(self))
    }
}
impl<'a> ::windows::IntoParam<'a, ::windows::IUnknown> for &'a IDXGIInfoQueue {
    fn into_param(self) -> ::windows::Param<'a, ::windows::IUnknown> {
        ::windows::Param::Owned(::std::convert::Into::<::windows::IUnknown>::into(
            ::std::clone::Clone::clone(self),
        ))
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDXGIInfoQueue_abi(
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        iid: &::windows::Guid,
        interface: *mut ::windows::RawPtr,
    ) -> ::windows::ErrorCode,
    pub unsafe extern "system" fn(this: ::windows::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::RawPtr) -> u32,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        producer: ::windows::Guid,
        message_count_limit: u64,
    ) -> ::windows::ErrorCode,
    pub unsafe extern "system" fn(this: ::windows::RawPtr, producer: ::windows::Guid),
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        producer: ::windows::Guid,
        message_index: u64,
        p_message: *mut DXGI_INFO_QUEUE_MESSAGE,
        p_message_byte_length: *mut usize,
    ) -> ::windows::ErrorCode,
    pub unsafe extern "system" fn(this: ::windows::RawPtr, producer: ::windows::Guid) -> u64,
    pub unsafe extern "system" fn(this: ::windows::RawPtr, producer: ::windows::Guid) -> u64,
    pub unsafe extern "system" fn(this: ::windows::RawPtr, producer: ::windows::Guid) -> u64,
    pub unsafe extern "system" fn(this: ::windows::RawPtr, producer: ::windows::Guid) -> u64,
    pub unsafe extern "system" fn(this: ::windows::RawPtr, producer: ::windows::Guid) -> u64,
    pub unsafe extern "system" fn(this: ::windows::RawPtr, producer: ::windows::Guid) -> u64,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        producer: ::windows::Guid,
        p_filter: *mut DXGI_INFO_QUEUE_FILTER,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        producer: ::windows::Guid,
        p_filter: *mut DXGI_INFO_QUEUE_FILTER,
        p_filter_byte_length: *mut usize,
    ) -> ::windows::ErrorCode,
    pub unsafe extern "system" fn(this: ::windows::RawPtr, producer: ::windows::Guid),
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        producer: ::windows::Guid,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        producer: ::windows::Guid,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        producer: ::windows::Guid,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        producer: ::windows::Guid,
        p_filter: *mut DXGI_INFO_QUEUE_FILTER,
    ) -> ::windows::ErrorCode,
    pub unsafe extern "system" fn(this: ::windows::RawPtr, producer: ::windows::Guid),
    pub unsafe extern "system" fn(this: ::windows::RawPtr, producer: ::windows::Guid) -> u32,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        producer: ::windows::Guid,
        p_filter: *mut DXGI_INFO_QUEUE_FILTER,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        producer: ::windows::Guid,
        p_filter: *mut DXGI_INFO_QUEUE_FILTER,
        p_filter_byte_length: *mut usize,
    ) -> ::windows::ErrorCode,
    pub unsafe extern "system" fn(this: ::windows::RawPtr, producer: ::windows::Guid),
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        producer: ::windows::Guid,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        producer: ::windows::Guid,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        producer: ::windows::Guid,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        producer: ::windows::Guid,
        p_filter: *mut DXGI_INFO_QUEUE_FILTER,
    ) -> ::windows::ErrorCode,
    pub unsafe extern "system" fn(this: ::windows::RawPtr, producer: ::windows::Guid),
    pub unsafe extern "system" fn(this: ::windows::RawPtr, producer: ::windows::Guid) -> u32,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        producer: ::windows::Guid,
        category: DXGI_INFO_QUEUE_MESSAGE_CATEGORY,
        severity: DXGI_INFO_QUEUE_MESSAGE_SEVERITY,
        id: i32,
        p_description: super::system_services::PSTR,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        severity: DXGI_INFO_QUEUE_MESSAGE_SEVERITY,
        p_description: super::system_services::PSTR,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        producer: ::windows::Guid,
        category: DXGI_INFO_QUEUE_MESSAGE_CATEGORY,
        b_enable: super::system_services::BOOL,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        producer: ::windows::Guid,
        severity: DXGI_INFO_QUEUE_MESSAGE_SEVERITY,
        b_enable: super::system_services::BOOL,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        producer: ::windows::Guid,
        id: i32,
        b_enable: super::system_services::BOOL,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        producer: ::windows::Guid,
        category: DXGI_INFO_QUEUE_MESSAGE_CATEGORY,
    ) -> super::system_services::BOOL,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        producer: ::windows::Guid,
        severity: DXGI_INFO_QUEUE_MESSAGE_SEVERITY,
    ) -> super::system_services::BOOL,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        producer: ::windows::Guid,
        id: i32,
    ) -> super::system_services::BOOL,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        producer: ::windows::Guid,
        b_mute: super::system_services::BOOL,
    ),
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        producer: ::windows::Guid,
    ) -> super::system_services::BOOL,
);
#[repr(transparent)]
#[allow(non_camel_case_types)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct IDXGIKeyedMutex(::windows::IUnknown);
impl IDXGIKeyedMutex {}
unsafe impl ::windows::Interface for IDXGIKeyedMutex {
    type Vtable = IDXGIKeyedMutex_abi;
    const IID: ::windows::Guid = ::windows::Guid::from_values(
        2643333769,
        55219,
        18015,
        [129, 38, 37, 14, 52, 154, 248, 93],
    );
}
#[allow(non_snake_case)]
impl IDXGIKeyedMutex {
    pub unsafe fn SetPrivateData(
        &self,
        name: *const ::windows::Guid,
        data_size: u32,
        p_data: *const ::std::ffi::c_void,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).3)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(name),
            ::std::mem::transmute(data_size),
            ::std::mem::transmute(p_data),
        )
    }
    pub unsafe fn SetPrivateDataInterface<
        'a,
        T1__: ::windows::IntoParam<'a, ::windows::IUnknown>,
    >(
        &self,
        name: *const ::windows::Guid,
        p_unknown: T1__,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).4)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(name),
            p_unknown.into_param().abi(),
        )
    }
    pub unsafe fn GetPrivateData(
        &self,
        name: *const ::windows::Guid,
        p_data_size: *mut u32,
        p_data: *mut ::std::ffi::c_void,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).5)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(name),
            ::std::mem::transmute(p_data_size),
            ::std::mem::transmute(p_data),
        )
    }
    pub unsafe fn GetParent(
        &self,
        riid: *const ::windows::Guid,
        pp_parent: *mut *mut ::std::ffi::c_void,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).6)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(riid),
            ::std::mem::transmute(pp_parent),
        )
    }
    pub unsafe fn GetDevice(
        &self,
        riid: *const ::windows::Guid,
        pp_device: *mut *mut ::std::ffi::c_void,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).7)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(riid),
            ::std::mem::transmute(pp_device),
        )
    }
    pub unsafe fn AcquireSync(&self, key: u64, dw_milliseconds: u32) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).8)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(key),
            ::std::mem::transmute(dw_milliseconds),
        )
    }
    pub unsafe fn ReleaseSync(&self, key: u64) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).9)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(key),
        )
    }
}
impl ::std::convert::From<IDXGIKeyedMutex> for ::windows::IUnknown {
    fn from(value: IDXGIKeyedMutex) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IDXGIKeyedMutex> for ::windows::IUnknown {
    fn from(value: &IDXGIKeyedMutex) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::IntoParam<'a, ::windows::IUnknown> for IDXGIKeyedMutex {
    fn into_param(self) -> ::windows::Param<'a, ::windows::IUnknown> {
        ::windows::Param::Owned(::std::convert::Into::<::windows::IUnknown>::into(self))
    }
}
impl<'a> ::windows::IntoParam<'a, ::windows::IUnknown> for &'a IDXGIKeyedMutex {
    fn into_param(self) -> ::windows::Param<'a, ::windows::IUnknown> {
        ::windows::Param::Owned(::std::convert::Into::<::windows::IUnknown>::into(
            ::std::clone::Clone::clone(self),
        ))
    }
}
impl ::std::convert::From<IDXGIKeyedMutex> for IDXGIDeviceSubObject {
    fn from(value: IDXGIKeyedMutex) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IDXGIKeyedMutex> for IDXGIDeviceSubObject {
    fn from(value: &IDXGIKeyedMutex) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::IntoParam<'a, IDXGIDeviceSubObject> for IDXGIKeyedMutex {
    fn into_param(self) -> ::windows::Param<'a, IDXGIDeviceSubObject> {
        ::windows::Param::Owned(::std::convert::Into::<IDXGIDeviceSubObject>::into(self))
    }
}
impl<'a> ::windows::IntoParam<'a, IDXGIDeviceSubObject> for &'a IDXGIKeyedMutex {
    fn into_param(self) -> ::windows::Param<'a, IDXGIDeviceSubObject> {
        ::windows::Param::Owned(::std::convert::Into::<IDXGIDeviceSubObject>::into(
            ::std::clone::Clone::clone(self),
        ))
    }
}
impl ::std::convert::From<IDXGIKeyedMutex> for IDXGIObject {
    fn from(value: IDXGIKeyedMutex) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IDXGIKeyedMutex> for IDXGIObject {
    fn from(value: &IDXGIKeyedMutex) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::IntoParam<'a, IDXGIObject> for IDXGIKeyedMutex {
    fn into_param(self) -> ::windows::Param<'a, IDXGIObject> {
        ::windows::Param::Owned(::std::convert::Into::<IDXGIObject>::into(self))
    }
}
impl<'a> ::windows::IntoParam<'a, IDXGIObject> for &'a IDXGIKeyedMutex {
    fn into_param(self) -> ::windows::Param<'a, IDXGIObject> {
        ::windows::Param::Owned(::std::convert::Into::<IDXGIObject>::into(
            ::std::clone::Clone::clone(self),
        ))
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDXGIKeyedMutex_abi(
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        iid: &::windows::Guid,
        interface: *mut ::windows::RawPtr,
    ) -> ::windows::ErrorCode,
    pub unsafe extern "system" fn(this: ::windows::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::RawPtr) -> u32,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        name: *const ::windows::Guid,
        data_size: u32,
        p_data: *const ::std::ffi::c_void,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        name: *const ::windows::Guid,
        p_unknown: ::windows::RawPtr,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        name: *const ::windows::Guid,
        p_data_size: *mut u32,
        p_data: *mut ::std::ffi::c_void,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        riid: *const ::windows::Guid,
        pp_parent: *mut *mut ::std::ffi::c_void,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        riid: *const ::windows::Guid,
        pp_device: *mut *mut ::std::ffi::c_void,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        key: u64,
        dw_milliseconds: u32,
    ) -> ::windows::ErrorCode,
    pub unsafe extern "system" fn(this: ::windows::RawPtr, key: u64) -> ::windows::ErrorCode,
);
#[repr(transparent)]
#[allow(non_camel_case_types)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct IDXGIOutput(::windows::IUnknown);
impl IDXGIOutput {}
unsafe impl ::windows::Interface for IDXGIOutput {
    type Vtable = IDXGIOutput_abi;
    const IID: ::windows::Guid = ::windows::Guid::from_values(
        2919427803,
        50997,
        18064,
        [141, 82, 90, 141, 194, 2, 19, 170],
    );
}
#[allow(non_snake_case)]
impl IDXGIOutput {
    pub unsafe fn SetPrivateData(
        &self,
        name: *const ::windows::Guid,
        data_size: u32,
        p_data: *const ::std::ffi::c_void,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).3)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(name),
            ::std::mem::transmute(data_size),
            ::std::mem::transmute(p_data),
        )
    }
    pub unsafe fn SetPrivateDataInterface<
        'a,
        T1__: ::windows::IntoParam<'a, ::windows::IUnknown>,
    >(
        &self,
        name: *const ::windows::Guid,
        p_unknown: T1__,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).4)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(name),
            p_unknown.into_param().abi(),
        )
    }
    pub unsafe fn GetPrivateData(
        &self,
        name: *const ::windows::Guid,
        p_data_size: *mut u32,
        p_data: *mut ::std::ffi::c_void,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).5)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(name),
            ::std::mem::transmute(p_data_size),
            ::std::mem::transmute(p_data),
        )
    }
    pub unsafe fn GetParent(
        &self,
        riid: *const ::windows::Guid,
        pp_parent: *mut *mut ::std::ffi::c_void,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).6)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(riid),
            ::std::mem::transmute(pp_parent),
        )
    }
    pub unsafe fn GetDesc(&self, p_desc: *mut DXGI_OUTPUT_DESC) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).7)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(p_desc),
        )
    }
    pub unsafe fn GetDisplayModeList(
        &self,
        enum_format: DXGI_FORMAT,
        flags: u32,
        p_num_modes: *mut u32,
        p_desc: *mut DXGI_MODE_DESC,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).8)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(enum_format),
            ::std::mem::transmute(flags),
            ::std::mem::transmute(p_num_modes),
            ::std::mem::transmute(p_desc),
        )
    }
    pub unsafe fn FindClosestMatchingMode<
        'a,
        T2__: ::windows::IntoParam<'a, ::windows::IUnknown>,
    >(
        &self,
        p_mode_to_match: *const DXGI_MODE_DESC,
        p_closest_match: *mut DXGI_MODE_DESC,
        p_concerned_device: T2__,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).9)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(p_mode_to_match),
            ::std::mem::transmute(p_closest_match),
            p_concerned_device.into_param().abi(),
        )
    }
    pub unsafe fn WaitForVBlank(&self) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).10)(::windows::Abi::abi(self))
    }
    pub unsafe fn TakeOwnership<
        'a,
        T0__: ::windows::IntoParam<'a, ::windows::IUnknown>,
        T1__: ::windows::IntoParam<'a, super::system_services::BOOL>,
    >(
        &self,
        p_device: T0__,
        exclusive: T1__,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).11)(
            ::windows::Abi::abi(self),
            p_device.into_param().abi(),
            exclusive.into_param().abi(),
        )
    }
    pub unsafe fn ReleaseOwnership(&self) {
        (::windows::Interface::vtable(self).12)(::windows::Abi::abi(self))
    }
    pub unsafe fn GetGammaControlCapabilities(
        &self,
        p_gamma_caps: *mut DXGI_GAMMA_CONTROL_CAPABILITIES,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).13)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(p_gamma_caps),
        )
    }
    pub unsafe fn SetGammaControl(
        &self,
        p_array: *const DXGI_GAMMA_CONTROL,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).14)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(p_array),
        )
    }
    pub unsafe fn GetGammaControl(&self, p_array: *mut DXGI_GAMMA_CONTROL) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).15)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(p_array),
        )
    }
    pub unsafe fn SetDisplaySurface<'a, T0__: ::windows::IntoParam<'a, IDXGISurface>>(
        &self,
        p_scanout_surface: T0__,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).16)(
            ::windows::Abi::abi(self),
            p_scanout_surface.into_param().abi(),
        )
    }
    pub unsafe fn GetDisplaySurfaceData<'a, T0__: ::windows::IntoParam<'a, IDXGISurface>>(
        &self,
        p_destination: T0__,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).17)(
            ::windows::Abi::abi(self),
            p_destination.into_param().abi(),
        )
    }
    pub unsafe fn GetFrameStatistics(
        &self,
        p_stats: *mut DXGI_FRAME_STATISTICS,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).18)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(p_stats),
        )
    }
}
impl ::std::convert::From<IDXGIOutput> for ::windows::IUnknown {
    fn from(value: IDXGIOutput) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IDXGIOutput> for ::windows::IUnknown {
    fn from(value: &IDXGIOutput) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::IntoParam<'a, ::windows::IUnknown> for IDXGIOutput {
    fn into_param(self) -> ::windows::Param<'a, ::windows::IUnknown> {
        ::windows::Param::Owned(::std::convert::Into::<::windows::IUnknown>::into(self))
    }
}
impl<'a> ::windows::IntoParam<'a, ::windows::IUnknown> for &'a IDXGIOutput {
    fn into_param(self) -> ::windows::Param<'a, ::windows::IUnknown> {
        ::windows::Param::Owned(::std::convert::Into::<::windows::IUnknown>::into(
            ::std::clone::Clone::clone(self),
        ))
    }
}
impl ::std::convert::From<IDXGIOutput> for IDXGIObject {
    fn from(value: IDXGIOutput) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IDXGIOutput> for IDXGIObject {
    fn from(value: &IDXGIOutput) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::IntoParam<'a, IDXGIObject> for IDXGIOutput {
    fn into_param(self) -> ::windows::Param<'a, IDXGIObject> {
        ::windows::Param::Owned(::std::convert::Into::<IDXGIObject>::into(self))
    }
}
impl<'a> ::windows::IntoParam<'a, IDXGIObject> for &'a IDXGIOutput {
    fn into_param(self) -> ::windows::Param<'a, IDXGIObject> {
        ::windows::Param::Owned(::std::convert::Into::<IDXGIObject>::into(
            ::std::clone::Clone::clone(self),
        ))
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDXGIOutput_abi(
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        iid: &::windows::Guid,
        interface: *mut ::windows::RawPtr,
    ) -> ::windows::ErrorCode,
    pub unsafe extern "system" fn(this: ::windows::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::RawPtr) -> u32,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        name: *const ::windows::Guid,
        data_size: u32,
        p_data: *const ::std::ffi::c_void,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        name: *const ::windows::Guid,
        p_unknown: ::windows::RawPtr,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        name: *const ::windows::Guid,
        p_data_size: *mut u32,
        p_data: *mut ::std::ffi::c_void,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        riid: *const ::windows::Guid,
        pp_parent: *mut *mut ::std::ffi::c_void,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        p_desc: *mut DXGI_OUTPUT_DESC,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        enum_format: DXGI_FORMAT,
        flags: u32,
        p_num_modes: *mut u32,
        p_desc: *mut DXGI_MODE_DESC,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        p_mode_to_match: *const DXGI_MODE_DESC,
        p_closest_match: *mut DXGI_MODE_DESC,
        p_concerned_device: ::windows::RawPtr,
    ) -> ::windows::ErrorCode,
    pub unsafe extern "system" fn(this: ::windows::RawPtr) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        p_device: ::windows::RawPtr,
        exclusive: super::system_services::BOOL,
    ) -> ::windows::ErrorCode,
    pub unsafe extern "system" fn(this: ::windows::RawPtr),
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        p_gamma_caps: *mut DXGI_GAMMA_CONTROL_CAPABILITIES,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        p_array: *const DXGI_GAMMA_CONTROL,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        p_array: *mut DXGI_GAMMA_CONTROL,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        p_scanout_surface: ::windows::RawPtr,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        p_destination: ::windows::RawPtr,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        p_stats: *mut DXGI_FRAME_STATISTICS,
    ) -> ::windows::ErrorCode,
);
#[repr(transparent)]
#[allow(non_camel_case_types)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct IDXGIOutput1(::windows::IUnknown);
impl IDXGIOutput1 {}
unsafe impl ::windows::Interface for IDXGIOutput1 {
    type Vtable = IDXGIOutput1_abi;
    const IID: ::windows::Guid = ::windows::Guid::from_values(
        13491880,
        37787,
        19331,
        [163, 64, 166, 133, 34, 102, 102, 204],
    );
}
#[allow(non_snake_case)]
impl IDXGIOutput1 {
    pub unsafe fn SetPrivateData(
        &self,
        name: *const ::windows::Guid,
        data_size: u32,
        p_data: *const ::std::ffi::c_void,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).3)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(name),
            ::std::mem::transmute(data_size),
            ::std::mem::transmute(p_data),
        )
    }
    pub unsafe fn SetPrivateDataInterface<
        'a,
        T1__: ::windows::IntoParam<'a, ::windows::IUnknown>,
    >(
        &self,
        name: *const ::windows::Guid,
        p_unknown: T1__,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).4)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(name),
            p_unknown.into_param().abi(),
        )
    }
    pub unsafe fn GetPrivateData(
        &self,
        name: *const ::windows::Guid,
        p_data_size: *mut u32,
        p_data: *mut ::std::ffi::c_void,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).5)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(name),
            ::std::mem::transmute(p_data_size),
            ::std::mem::transmute(p_data),
        )
    }
    pub unsafe fn GetParent(
        &self,
        riid: *const ::windows::Guid,
        pp_parent: *mut *mut ::std::ffi::c_void,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).6)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(riid),
            ::std::mem::transmute(pp_parent),
        )
    }
    pub unsafe fn GetDesc(&self, p_desc: *mut DXGI_OUTPUT_DESC) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).7)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(p_desc),
        )
    }
    pub unsafe fn GetDisplayModeList(
        &self,
        enum_format: DXGI_FORMAT,
        flags: u32,
        p_num_modes: *mut u32,
        p_desc: *mut DXGI_MODE_DESC,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).8)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(enum_format),
            ::std::mem::transmute(flags),
            ::std::mem::transmute(p_num_modes),
            ::std::mem::transmute(p_desc),
        )
    }
    pub unsafe fn FindClosestMatchingMode<
        'a,
        T2__: ::windows::IntoParam<'a, ::windows::IUnknown>,
    >(
        &self,
        p_mode_to_match: *const DXGI_MODE_DESC,
        p_closest_match: *mut DXGI_MODE_DESC,
        p_concerned_device: T2__,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).9)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(p_mode_to_match),
            ::std::mem::transmute(p_closest_match),
            p_concerned_device.into_param().abi(),
        )
    }
    pub unsafe fn WaitForVBlank(&self) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).10)(::windows::Abi::abi(self))
    }
    pub unsafe fn TakeOwnership<
        'a,
        T0__: ::windows::IntoParam<'a, ::windows::IUnknown>,
        T1__: ::windows::IntoParam<'a, super::system_services::BOOL>,
    >(
        &self,
        p_device: T0__,
        exclusive: T1__,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).11)(
            ::windows::Abi::abi(self),
            p_device.into_param().abi(),
            exclusive.into_param().abi(),
        )
    }
    pub unsafe fn ReleaseOwnership(&self) {
        (::windows::Interface::vtable(self).12)(::windows::Abi::abi(self))
    }
    pub unsafe fn GetGammaControlCapabilities(
        &self,
        p_gamma_caps: *mut DXGI_GAMMA_CONTROL_CAPABILITIES,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).13)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(p_gamma_caps),
        )
    }
    pub unsafe fn SetGammaControl(
        &self,
        p_array: *const DXGI_GAMMA_CONTROL,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).14)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(p_array),
        )
    }
    pub unsafe fn GetGammaControl(&self, p_array: *mut DXGI_GAMMA_CONTROL) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).15)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(p_array),
        )
    }
    pub unsafe fn SetDisplaySurface<'a, T0__: ::windows::IntoParam<'a, IDXGISurface>>(
        &self,
        p_scanout_surface: T0__,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).16)(
            ::windows::Abi::abi(self),
            p_scanout_surface.into_param().abi(),
        )
    }
    pub unsafe fn GetDisplaySurfaceData<'a, T0__: ::windows::IntoParam<'a, IDXGISurface>>(
        &self,
        p_destination: T0__,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).17)(
            ::windows::Abi::abi(self),
            p_destination.into_param().abi(),
        )
    }
    pub unsafe fn GetFrameStatistics(
        &self,
        p_stats: *mut DXGI_FRAME_STATISTICS,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).18)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(p_stats),
        )
    }
    pub unsafe fn GetDisplayModeList1(
        &self,
        enum_format: DXGI_FORMAT,
        flags: u32,
        p_num_modes: *mut u32,
        p_desc: *mut DXGI_MODE_DESC1,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).19)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(enum_format),
            ::std::mem::transmute(flags),
            ::std::mem::transmute(p_num_modes),
            ::std::mem::transmute(p_desc),
        )
    }
    pub unsafe fn FindClosestMatchingMode1<
        'a,
        T2__: ::windows::IntoParam<'a, ::windows::IUnknown>,
    >(
        &self,
        p_mode_to_match: *const DXGI_MODE_DESC1,
        p_closest_match: *mut DXGI_MODE_DESC1,
        p_concerned_device: T2__,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).20)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(p_mode_to_match),
            ::std::mem::transmute(p_closest_match),
            p_concerned_device.into_param().abi(),
        )
    }
    pub unsafe fn GetDisplaySurfaceData1<'a, T0__: ::windows::IntoParam<'a, IDXGIResource>>(
        &self,
        p_destination: T0__,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).21)(
            ::windows::Abi::abi(self),
            p_destination.into_param().abi(),
        )
    }
    pub unsafe fn DuplicateOutput<'a, T0__: ::windows::IntoParam<'a, ::windows::IUnknown>>(
        &self,
        p_device: T0__,
        pp_output_duplication: *mut ::std::option::Option<IDXGIOutputDuplication>,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).22)(
            ::windows::Abi::abi(self),
            p_device.into_param().abi(),
            ::std::mem::transmute(pp_output_duplication),
        )
    }
}
impl ::std::convert::From<IDXGIOutput1> for ::windows::IUnknown {
    fn from(value: IDXGIOutput1) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IDXGIOutput1> for ::windows::IUnknown {
    fn from(value: &IDXGIOutput1) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::IntoParam<'a, ::windows::IUnknown> for IDXGIOutput1 {
    fn into_param(self) -> ::windows::Param<'a, ::windows::IUnknown> {
        ::windows::Param::Owned(::std::convert::Into::<::windows::IUnknown>::into(self))
    }
}
impl<'a> ::windows::IntoParam<'a, ::windows::IUnknown> for &'a IDXGIOutput1 {
    fn into_param(self) -> ::windows::Param<'a, ::windows::IUnknown> {
        ::windows::Param::Owned(::std::convert::Into::<::windows::IUnknown>::into(
            ::std::clone::Clone::clone(self),
        ))
    }
}
impl ::std::convert::From<IDXGIOutput1> for IDXGIOutput {
    fn from(value: IDXGIOutput1) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IDXGIOutput1> for IDXGIOutput {
    fn from(value: &IDXGIOutput1) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::IntoParam<'a, IDXGIOutput> for IDXGIOutput1 {
    fn into_param(self) -> ::windows::Param<'a, IDXGIOutput> {
        ::windows::Param::Owned(::std::convert::Into::<IDXGIOutput>::into(self))
    }
}
impl<'a> ::windows::IntoParam<'a, IDXGIOutput> for &'a IDXGIOutput1 {
    fn into_param(self) -> ::windows::Param<'a, IDXGIOutput> {
        ::windows::Param::Owned(::std::convert::Into::<IDXGIOutput>::into(
            ::std::clone::Clone::clone(self),
        ))
    }
}
impl ::std::convert::From<IDXGIOutput1> for IDXGIObject {
    fn from(value: IDXGIOutput1) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IDXGIOutput1> for IDXGIObject {
    fn from(value: &IDXGIOutput1) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::IntoParam<'a, IDXGIObject> for IDXGIOutput1 {
    fn into_param(self) -> ::windows::Param<'a, IDXGIObject> {
        ::windows::Param::Owned(::std::convert::Into::<IDXGIObject>::into(self))
    }
}
impl<'a> ::windows::IntoParam<'a, IDXGIObject> for &'a IDXGIOutput1 {
    fn into_param(self) -> ::windows::Param<'a, IDXGIObject> {
        ::windows::Param::Owned(::std::convert::Into::<IDXGIObject>::into(
            ::std::clone::Clone::clone(self),
        ))
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDXGIOutput1_abi(
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        iid: &::windows::Guid,
        interface: *mut ::windows::RawPtr,
    ) -> ::windows::ErrorCode,
    pub unsafe extern "system" fn(this: ::windows::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::RawPtr) -> u32,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        name: *const ::windows::Guid,
        data_size: u32,
        p_data: *const ::std::ffi::c_void,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        name: *const ::windows::Guid,
        p_unknown: ::windows::RawPtr,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        name: *const ::windows::Guid,
        p_data_size: *mut u32,
        p_data: *mut ::std::ffi::c_void,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        riid: *const ::windows::Guid,
        pp_parent: *mut *mut ::std::ffi::c_void,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        p_desc: *mut DXGI_OUTPUT_DESC,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        enum_format: DXGI_FORMAT,
        flags: u32,
        p_num_modes: *mut u32,
        p_desc: *mut DXGI_MODE_DESC,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        p_mode_to_match: *const DXGI_MODE_DESC,
        p_closest_match: *mut DXGI_MODE_DESC,
        p_concerned_device: ::windows::RawPtr,
    ) -> ::windows::ErrorCode,
    pub unsafe extern "system" fn(this: ::windows::RawPtr) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        p_device: ::windows::RawPtr,
        exclusive: super::system_services::BOOL,
    ) -> ::windows::ErrorCode,
    pub unsafe extern "system" fn(this: ::windows::RawPtr),
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        p_gamma_caps: *mut DXGI_GAMMA_CONTROL_CAPABILITIES,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        p_array: *const DXGI_GAMMA_CONTROL,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        p_array: *mut DXGI_GAMMA_CONTROL,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        p_scanout_surface: ::windows::RawPtr,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        p_destination: ::windows::RawPtr,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        p_stats: *mut DXGI_FRAME_STATISTICS,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        enum_format: DXGI_FORMAT,
        flags: u32,
        p_num_modes: *mut u32,
        p_desc: *mut DXGI_MODE_DESC1,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        p_mode_to_match: *const DXGI_MODE_DESC1,
        p_closest_match: *mut DXGI_MODE_DESC1,
        p_concerned_device: ::windows::RawPtr,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        p_destination: ::windows::RawPtr,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        p_device: ::windows::RawPtr,
        pp_output_duplication: *mut ::windows::RawPtr,
    ) -> ::windows::ErrorCode,
);
#[repr(transparent)]
#[allow(non_camel_case_types)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct IDXGIOutput2(::windows::IUnknown);
impl IDXGIOutput2 {}
unsafe impl ::windows::Interface for IDXGIOutput2 {
    type Vtable = IDXGIOutput2_abi;
    const IID: ::windows::Guid = ::windows::Guid::from_values(
        1499347409,
        10020,
        18019,
        [153, 177, 218, 150, 157, 226, 131, 100],
    );
}
#[allow(non_snake_case)]
impl IDXGIOutput2 {
    pub unsafe fn SetPrivateData(
        &self,
        name: *const ::windows::Guid,
        data_size: u32,
        p_data: *const ::std::ffi::c_void,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).3)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(name),
            ::std::mem::transmute(data_size),
            ::std::mem::transmute(p_data),
        )
    }
    pub unsafe fn SetPrivateDataInterface<
        'a,
        T1__: ::windows::IntoParam<'a, ::windows::IUnknown>,
    >(
        &self,
        name: *const ::windows::Guid,
        p_unknown: T1__,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).4)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(name),
            p_unknown.into_param().abi(),
        )
    }
    pub unsafe fn GetPrivateData(
        &self,
        name: *const ::windows::Guid,
        p_data_size: *mut u32,
        p_data: *mut ::std::ffi::c_void,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).5)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(name),
            ::std::mem::transmute(p_data_size),
            ::std::mem::transmute(p_data),
        )
    }
    pub unsafe fn GetParent(
        &self,
        riid: *const ::windows::Guid,
        pp_parent: *mut *mut ::std::ffi::c_void,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).6)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(riid),
            ::std::mem::transmute(pp_parent),
        )
    }
    pub unsafe fn GetDesc(&self, p_desc: *mut DXGI_OUTPUT_DESC) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).7)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(p_desc),
        )
    }
    pub unsafe fn GetDisplayModeList(
        &self,
        enum_format: DXGI_FORMAT,
        flags: u32,
        p_num_modes: *mut u32,
        p_desc: *mut DXGI_MODE_DESC,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).8)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(enum_format),
            ::std::mem::transmute(flags),
            ::std::mem::transmute(p_num_modes),
            ::std::mem::transmute(p_desc),
        )
    }
    pub unsafe fn FindClosestMatchingMode<
        'a,
        T2__: ::windows::IntoParam<'a, ::windows::IUnknown>,
    >(
        &self,
        p_mode_to_match: *const DXGI_MODE_DESC,
        p_closest_match: *mut DXGI_MODE_DESC,
        p_concerned_device: T2__,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).9)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(p_mode_to_match),
            ::std::mem::transmute(p_closest_match),
            p_concerned_device.into_param().abi(),
        )
    }
    pub unsafe fn WaitForVBlank(&self) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).10)(::windows::Abi::abi(self))
    }
    pub unsafe fn TakeOwnership<
        'a,
        T0__: ::windows::IntoParam<'a, ::windows::IUnknown>,
        T1__: ::windows::IntoParam<'a, super::system_services::BOOL>,
    >(
        &self,
        p_device: T0__,
        exclusive: T1__,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).11)(
            ::windows::Abi::abi(self),
            p_device.into_param().abi(),
            exclusive.into_param().abi(),
        )
    }
    pub unsafe fn ReleaseOwnership(&self) {
        (::windows::Interface::vtable(self).12)(::windows::Abi::abi(self))
    }
    pub unsafe fn GetGammaControlCapabilities(
        &self,
        p_gamma_caps: *mut DXGI_GAMMA_CONTROL_CAPABILITIES,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).13)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(p_gamma_caps),
        )
    }
    pub unsafe fn SetGammaControl(
        &self,
        p_array: *const DXGI_GAMMA_CONTROL,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).14)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(p_array),
        )
    }
    pub unsafe fn GetGammaControl(&self, p_array: *mut DXGI_GAMMA_CONTROL) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).15)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(p_array),
        )
    }
    pub unsafe fn SetDisplaySurface<'a, T0__: ::windows::IntoParam<'a, IDXGISurface>>(
        &self,
        p_scanout_surface: T0__,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).16)(
            ::windows::Abi::abi(self),
            p_scanout_surface.into_param().abi(),
        )
    }
    pub unsafe fn GetDisplaySurfaceData<'a, T0__: ::windows::IntoParam<'a, IDXGISurface>>(
        &self,
        p_destination: T0__,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).17)(
            ::windows::Abi::abi(self),
            p_destination.into_param().abi(),
        )
    }
    pub unsafe fn GetFrameStatistics(
        &self,
        p_stats: *mut DXGI_FRAME_STATISTICS,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).18)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(p_stats),
        )
    }
    pub unsafe fn GetDisplayModeList1(
        &self,
        enum_format: DXGI_FORMAT,
        flags: u32,
        p_num_modes: *mut u32,
        p_desc: *mut DXGI_MODE_DESC1,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).19)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(enum_format),
            ::std::mem::transmute(flags),
            ::std::mem::transmute(p_num_modes),
            ::std::mem::transmute(p_desc),
        )
    }
    pub unsafe fn FindClosestMatchingMode1<
        'a,
        T2__: ::windows::IntoParam<'a, ::windows::IUnknown>,
    >(
        &self,
        p_mode_to_match: *const DXGI_MODE_DESC1,
        p_closest_match: *mut DXGI_MODE_DESC1,
        p_concerned_device: T2__,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).20)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(p_mode_to_match),
            ::std::mem::transmute(p_closest_match),
            p_concerned_device.into_param().abi(),
        )
    }
    pub unsafe fn GetDisplaySurfaceData1<'a, T0__: ::windows::IntoParam<'a, IDXGIResource>>(
        &self,
        p_destination: T0__,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).21)(
            ::windows::Abi::abi(self),
            p_destination.into_param().abi(),
        )
    }
    pub unsafe fn DuplicateOutput<'a, T0__: ::windows::IntoParam<'a, ::windows::IUnknown>>(
        &self,
        p_device: T0__,
        pp_output_duplication: *mut ::std::option::Option<IDXGIOutputDuplication>,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).22)(
            ::windows::Abi::abi(self),
            p_device.into_param().abi(),
            ::std::mem::transmute(pp_output_duplication),
        )
    }
    pub unsafe fn SupportsOverlays(&self) -> super::system_services::BOOL {
        (::windows::Interface::vtable(self).23)(::windows::Abi::abi(self))
    }
}
impl ::std::convert::From<IDXGIOutput2> for ::windows::IUnknown {
    fn from(value: IDXGIOutput2) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IDXGIOutput2> for ::windows::IUnknown {
    fn from(value: &IDXGIOutput2) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::IntoParam<'a, ::windows::IUnknown> for IDXGIOutput2 {
    fn into_param(self) -> ::windows::Param<'a, ::windows::IUnknown> {
        ::windows::Param::Owned(::std::convert::Into::<::windows::IUnknown>::into(self))
    }
}
impl<'a> ::windows::IntoParam<'a, ::windows::IUnknown> for &'a IDXGIOutput2 {
    fn into_param(self) -> ::windows::Param<'a, ::windows::IUnknown> {
        ::windows::Param::Owned(::std::convert::Into::<::windows::IUnknown>::into(
            ::std::clone::Clone::clone(self),
        ))
    }
}
impl ::std::convert::From<IDXGIOutput2> for IDXGIOutput1 {
    fn from(value: IDXGIOutput2) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IDXGIOutput2> for IDXGIOutput1 {
    fn from(value: &IDXGIOutput2) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::IntoParam<'a, IDXGIOutput1> for IDXGIOutput2 {
    fn into_param(self) -> ::windows::Param<'a, IDXGIOutput1> {
        ::windows::Param::Owned(::std::convert::Into::<IDXGIOutput1>::into(self))
    }
}
impl<'a> ::windows::IntoParam<'a, IDXGIOutput1> for &'a IDXGIOutput2 {
    fn into_param(self) -> ::windows::Param<'a, IDXGIOutput1> {
        ::windows::Param::Owned(::std::convert::Into::<IDXGIOutput1>::into(
            ::std::clone::Clone::clone(self),
        ))
    }
}
impl ::std::convert::From<IDXGIOutput2> for IDXGIOutput {
    fn from(value: IDXGIOutput2) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IDXGIOutput2> for IDXGIOutput {
    fn from(value: &IDXGIOutput2) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::IntoParam<'a, IDXGIOutput> for IDXGIOutput2 {
    fn into_param(self) -> ::windows::Param<'a, IDXGIOutput> {
        ::windows::Param::Owned(::std::convert::Into::<IDXGIOutput>::into(self))
    }
}
impl<'a> ::windows::IntoParam<'a, IDXGIOutput> for &'a IDXGIOutput2 {
    fn into_param(self) -> ::windows::Param<'a, IDXGIOutput> {
        ::windows::Param::Owned(::std::convert::Into::<IDXGIOutput>::into(
            ::std::clone::Clone::clone(self),
        ))
    }
}
impl ::std::convert::From<IDXGIOutput2> for IDXGIObject {
    fn from(value: IDXGIOutput2) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IDXGIOutput2> for IDXGIObject {
    fn from(value: &IDXGIOutput2) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::IntoParam<'a, IDXGIObject> for IDXGIOutput2 {
    fn into_param(self) -> ::windows::Param<'a, IDXGIObject> {
        ::windows::Param::Owned(::std::convert::Into::<IDXGIObject>::into(self))
    }
}
impl<'a> ::windows::IntoParam<'a, IDXGIObject> for &'a IDXGIOutput2 {
    fn into_param(self) -> ::windows::Param<'a, IDXGIObject> {
        ::windows::Param::Owned(::std::convert::Into::<IDXGIObject>::into(
            ::std::clone::Clone::clone(self),
        ))
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDXGIOutput2_abi(
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        iid: &::windows::Guid,
        interface: *mut ::windows::RawPtr,
    ) -> ::windows::ErrorCode,
    pub unsafe extern "system" fn(this: ::windows::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::RawPtr) -> u32,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        name: *const ::windows::Guid,
        data_size: u32,
        p_data: *const ::std::ffi::c_void,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        name: *const ::windows::Guid,
        p_unknown: ::windows::RawPtr,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        name: *const ::windows::Guid,
        p_data_size: *mut u32,
        p_data: *mut ::std::ffi::c_void,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        riid: *const ::windows::Guid,
        pp_parent: *mut *mut ::std::ffi::c_void,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        p_desc: *mut DXGI_OUTPUT_DESC,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        enum_format: DXGI_FORMAT,
        flags: u32,
        p_num_modes: *mut u32,
        p_desc: *mut DXGI_MODE_DESC,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        p_mode_to_match: *const DXGI_MODE_DESC,
        p_closest_match: *mut DXGI_MODE_DESC,
        p_concerned_device: ::windows::RawPtr,
    ) -> ::windows::ErrorCode,
    pub unsafe extern "system" fn(this: ::windows::RawPtr) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        p_device: ::windows::RawPtr,
        exclusive: super::system_services::BOOL,
    ) -> ::windows::ErrorCode,
    pub unsafe extern "system" fn(this: ::windows::RawPtr),
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        p_gamma_caps: *mut DXGI_GAMMA_CONTROL_CAPABILITIES,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        p_array: *const DXGI_GAMMA_CONTROL,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        p_array: *mut DXGI_GAMMA_CONTROL,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        p_scanout_surface: ::windows::RawPtr,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        p_destination: ::windows::RawPtr,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        p_stats: *mut DXGI_FRAME_STATISTICS,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        enum_format: DXGI_FORMAT,
        flags: u32,
        p_num_modes: *mut u32,
        p_desc: *mut DXGI_MODE_DESC1,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        p_mode_to_match: *const DXGI_MODE_DESC1,
        p_closest_match: *mut DXGI_MODE_DESC1,
        p_concerned_device: ::windows::RawPtr,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        p_destination: ::windows::RawPtr,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        p_device: ::windows::RawPtr,
        pp_output_duplication: *mut ::windows::RawPtr,
    ) -> ::windows::ErrorCode,
    pub unsafe extern "system" fn(this: ::windows::RawPtr) -> super::system_services::BOOL,
);
#[repr(transparent)]
#[allow(non_camel_case_types)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct IDXGIOutput3(::windows::IUnknown);
impl IDXGIOutput3 {}
unsafe impl ::windows::Interface for IDXGIOutput3 {
    type Vtable = IDXGIOutput3_abi;
    const IID: ::windows::Guid = ::windows::Guid::from_values(
        2322313985,
        32382,
        16884,
        [168, 224, 91, 50, 247, 249, 155, 24],
    );
}
#[allow(non_snake_case)]
impl IDXGIOutput3 {
    pub unsafe fn SetPrivateData(
        &self,
        name: *const ::windows::Guid,
        data_size: u32,
        p_data: *const ::std::ffi::c_void,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).3)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(name),
            ::std::mem::transmute(data_size),
            ::std::mem::transmute(p_data),
        )
    }
    pub unsafe fn SetPrivateDataInterface<
        'a,
        T1__: ::windows::IntoParam<'a, ::windows::IUnknown>,
    >(
        &self,
        name: *const ::windows::Guid,
        p_unknown: T1__,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).4)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(name),
            p_unknown.into_param().abi(),
        )
    }
    pub unsafe fn GetPrivateData(
        &self,
        name: *const ::windows::Guid,
        p_data_size: *mut u32,
        p_data: *mut ::std::ffi::c_void,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).5)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(name),
            ::std::mem::transmute(p_data_size),
            ::std::mem::transmute(p_data),
        )
    }
    pub unsafe fn GetParent(
        &self,
        riid: *const ::windows::Guid,
        pp_parent: *mut *mut ::std::ffi::c_void,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).6)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(riid),
            ::std::mem::transmute(pp_parent),
        )
    }
    pub unsafe fn GetDesc(&self, p_desc: *mut DXGI_OUTPUT_DESC) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).7)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(p_desc),
        )
    }
    pub unsafe fn GetDisplayModeList(
        &self,
        enum_format: DXGI_FORMAT,
        flags: u32,
        p_num_modes: *mut u32,
        p_desc: *mut DXGI_MODE_DESC,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).8)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(enum_format),
            ::std::mem::transmute(flags),
            ::std::mem::transmute(p_num_modes),
            ::std::mem::transmute(p_desc),
        )
    }
    pub unsafe fn FindClosestMatchingMode<
        'a,
        T2__: ::windows::IntoParam<'a, ::windows::IUnknown>,
    >(
        &self,
        p_mode_to_match: *const DXGI_MODE_DESC,
        p_closest_match: *mut DXGI_MODE_DESC,
        p_concerned_device: T2__,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).9)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(p_mode_to_match),
            ::std::mem::transmute(p_closest_match),
            p_concerned_device.into_param().abi(),
        )
    }
    pub unsafe fn WaitForVBlank(&self) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).10)(::windows::Abi::abi(self))
    }
    pub unsafe fn TakeOwnership<
        'a,
        T0__: ::windows::IntoParam<'a, ::windows::IUnknown>,
        T1__: ::windows::IntoParam<'a, super::system_services::BOOL>,
    >(
        &self,
        p_device: T0__,
        exclusive: T1__,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).11)(
            ::windows::Abi::abi(self),
            p_device.into_param().abi(),
            exclusive.into_param().abi(),
        )
    }
    pub unsafe fn ReleaseOwnership(&self) {
        (::windows::Interface::vtable(self).12)(::windows::Abi::abi(self))
    }
    pub unsafe fn GetGammaControlCapabilities(
        &self,
        p_gamma_caps: *mut DXGI_GAMMA_CONTROL_CAPABILITIES,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).13)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(p_gamma_caps),
        )
    }
    pub unsafe fn SetGammaControl(
        &self,
        p_array: *const DXGI_GAMMA_CONTROL,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).14)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(p_array),
        )
    }
    pub unsafe fn GetGammaControl(&self, p_array: *mut DXGI_GAMMA_CONTROL) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).15)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(p_array),
        )
    }
    pub unsafe fn SetDisplaySurface<'a, T0__: ::windows::IntoParam<'a, IDXGISurface>>(
        &self,
        p_scanout_surface: T0__,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).16)(
            ::windows::Abi::abi(self),
            p_scanout_surface.into_param().abi(),
        )
    }
    pub unsafe fn GetDisplaySurfaceData<'a, T0__: ::windows::IntoParam<'a, IDXGISurface>>(
        &self,
        p_destination: T0__,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).17)(
            ::windows::Abi::abi(self),
            p_destination.into_param().abi(),
        )
    }
    pub unsafe fn GetFrameStatistics(
        &self,
        p_stats: *mut DXGI_FRAME_STATISTICS,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).18)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(p_stats),
        )
    }
    pub unsafe fn GetDisplayModeList1(
        &self,
        enum_format: DXGI_FORMAT,
        flags: u32,
        p_num_modes: *mut u32,
        p_desc: *mut DXGI_MODE_DESC1,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).19)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(enum_format),
            ::std::mem::transmute(flags),
            ::std::mem::transmute(p_num_modes),
            ::std::mem::transmute(p_desc),
        )
    }
    pub unsafe fn FindClosestMatchingMode1<
        'a,
        T2__: ::windows::IntoParam<'a, ::windows::IUnknown>,
    >(
        &self,
        p_mode_to_match: *const DXGI_MODE_DESC1,
        p_closest_match: *mut DXGI_MODE_DESC1,
        p_concerned_device: T2__,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).20)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(p_mode_to_match),
            ::std::mem::transmute(p_closest_match),
            p_concerned_device.into_param().abi(),
        )
    }
    pub unsafe fn GetDisplaySurfaceData1<'a, T0__: ::windows::IntoParam<'a, IDXGIResource>>(
        &self,
        p_destination: T0__,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).21)(
            ::windows::Abi::abi(self),
            p_destination.into_param().abi(),
        )
    }
    pub unsafe fn DuplicateOutput<'a, T0__: ::windows::IntoParam<'a, ::windows::IUnknown>>(
        &self,
        p_device: T0__,
        pp_output_duplication: *mut ::std::option::Option<IDXGIOutputDuplication>,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).22)(
            ::windows::Abi::abi(self),
            p_device.into_param().abi(),
            ::std::mem::transmute(pp_output_duplication),
        )
    }
    pub unsafe fn SupportsOverlays(&self) -> super::system_services::BOOL {
        (::windows::Interface::vtable(self).23)(::windows::Abi::abi(self))
    }
    pub unsafe fn CheckOverlaySupport<'a, T1__: ::windows::IntoParam<'a, ::windows::IUnknown>>(
        &self,
        enum_format: DXGI_FORMAT,
        p_concerned_device: T1__,
        p_flags: *mut u32,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).24)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(enum_format),
            p_concerned_device.into_param().abi(),
            ::std::mem::transmute(p_flags),
        )
    }
}
impl ::std::convert::From<IDXGIOutput3> for ::windows::IUnknown {
    fn from(value: IDXGIOutput3) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IDXGIOutput3> for ::windows::IUnknown {
    fn from(value: &IDXGIOutput3) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::IntoParam<'a, ::windows::IUnknown> for IDXGIOutput3 {
    fn into_param(self) -> ::windows::Param<'a, ::windows::IUnknown> {
        ::windows::Param::Owned(::std::convert::Into::<::windows::IUnknown>::into(self))
    }
}
impl<'a> ::windows::IntoParam<'a, ::windows::IUnknown> for &'a IDXGIOutput3 {
    fn into_param(self) -> ::windows::Param<'a, ::windows::IUnknown> {
        ::windows::Param::Owned(::std::convert::Into::<::windows::IUnknown>::into(
            ::std::clone::Clone::clone(self),
        ))
    }
}
impl ::std::convert::From<IDXGIOutput3> for IDXGIOutput2 {
    fn from(value: IDXGIOutput3) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IDXGIOutput3> for IDXGIOutput2 {
    fn from(value: &IDXGIOutput3) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::IntoParam<'a, IDXGIOutput2> for IDXGIOutput3 {
    fn into_param(self) -> ::windows::Param<'a, IDXGIOutput2> {
        ::windows::Param::Owned(::std::convert::Into::<IDXGIOutput2>::into(self))
    }
}
impl<'a> ::windows::IntoParam<'a, IDXGIOutput2> for &'a IDXGIOutput3 {
    fn into_param(self) -> ::windows::Param<'a, IDXGIOutput2> {
        ::windows::Param::Owned(::std::convert::Into::<IDXGIOutput2>::into(
            ::std::clone::Clone::clone(self),
        ))
    }
}
impl ::std::convert::From<IDXGIOutput3> for IDXGIOutput1 {
    fn from(value: IDXGIOutput3) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IDXGIOutput3> for IDXGIOutput1 {
    fn from(value: &IDXGIOutput3) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::IntoParam<'a, IDXGIOutput1> for IDXGIOutput3 {
    fn into_param(self) -> ::windows::Param<'a, IDXGIOutput1> {
        ::windows::Param::Owned(::std::convert::Into::<IDXGIOutput1>::into(self))
    }
}
impl<'a> ::windows::IntoParam<'a, IDXGIOutput1> for &'a IDXGIOutput3 {
    fn into_param(self) -> ::windows::Param<'a, IDXGIOutput1> {
        ::windows::Param::Owned(::std::convert::Into::<IDXGIOutput1>::into(
            ::std::clone::Clone::clone(self),
        ))
    }
}
impl ::std::convert::From<IDXGIOutput3> for IDXGIOutput {
    fn from(value: IDXGIOutput3) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IDXGIOutput3> for IDXGIOutput {
    fn from(value: &IDXGIOutput3) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::IntoParam<'a, IDXGIOutput> for IDXGIOutput3 {
    fn into_param(self) -> ::windows::Param<'a, IDXGIOutput> {
        ::windows::Param::Owned(::std::convert::Into::<IDXGIOutput>::into(self))
    }
}
impl<'a> ::windows::IntoParam<'a, IDXGIOutput> for &'a IDXGIOutput3 {
    fn into_param(self) -> ::windows::Param<'a, IDXGIOutput> {
        ::windows::Param::Owned(::std::convert::Into::<IDXGIOutput>::into(
            ::std::clone::Clone::clone(self),
        ))
    }
}
impl ::std::convert::From<IDXGIOutput3> for IDXGIObject {
    fn from(value: IDXGIOutput3) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IDXGIOutput3> for IDXGIObject {
    fn from(value: &IDXGIOutput3) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::IntoParam<'a, IDXGIObject> for IDXGIOutput3 {
    fn into_param(self) -> ::windows::Param<'a, IDXGIObject> {
        ::windows::Param::Owned(::std::convert::Into::<IDXGIObject>::into(self))
    }
}
impl<'a> ::windows::IntoParam<'a, IDXGIObject> for &'a IDXGIOutput3 {
    fn into_param(self) -> ::windows::Param<'a, IDXGIObject> {
        ::windows::Param::Owned(::std::convert::Into::<IDXGIObject>::into(
            ::std::clone::Clone::clone(self),
        ))
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDXGIOutput3_abi(
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        iid: &::windows::Guid,
        interface: *mut ::windows::RawPtr,
    ) -> ::windows::ErrorCode,
    pub unsafe extern "system" fn(this: ::windows::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::RawPtr) -> u32,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        name: *const ::windows::Guid,
        data_size: u32,
        p_data: *const ::std::ffi::c_void,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        name: *const ::windows::Guid,
        p_unknown: ::windows::RawPtr,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        name: *const ::windows::Guid,
        p_data_size: *mut u32,
        p_data: *mut ::std::ffi::c_void,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        riid: *const ::windows::Guid,
        pp_parent: *mut *mut ::std::ffi::c_void,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        p_desc: *mut DXGI_OUTPUT_DESC,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        enum_format: DXGI_FORMAT,
        flags: u32,
        p_num_modes: *mut u32,
        p_desc: *mut DXGI_MODE_DESC,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        p_mode_to_match: *const DXGI_MODE_DESC,
        p_closest_match: *mut DXGI_MODE_DESC,
        p_concerned_device: ::windows::RawPtr,
    ) -> ::windows::ErrorCode,
    pub unsafe extern "system" fn(this: ::windows::RawPtr) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        p_device: ::windows::RawPtr,
        exclusive: super::system_services::BOOL,
    ) -> ::windows::ErrorCode,
    pub unsafe extern "system" fn(this: ::windows::RawPtr),
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        p_gamma_caps: *mut DXGI_GAMMA_CONTROL_CAPABILITIES,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        p_array: *const DXGI_GAMMA_CONTROL,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        p_array: *mut DXGI_GAMMA_CONTROL,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        p_scanout_surface: ::windows::RawPtr,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        p_destination: ::windows::RawPtr,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        p_stats: *mut DXGI_FRAME_STATISTICS,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        enum_format: DXGI_FORMAT,
        flags: u32,
        p_num_modes: *mut u32,
        p_desc: *mut DXGI_MODE_DESC1,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        p_mode_to_match: *const DXGI_MODE_DESC1,
        p_closest_match: *mut DXGI_MODE_DESC1,
        p_concerned_device: ::windows::RawPtr,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        p_destination: ::windows::RawPtr,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        p_device: ::windows::RawPtr,
        pp_output_duplication: *mut ::windows::RawPtr,
    ) -> ::windows::ErrorCode,
    pub unsafe extern "system" fn(this: ::windows::RawPtr) -> super::system_services::BOOL,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        enum_format: DXGI_FORMAT,
        p_concerned_device: ::windows::RawPtr,
        p_flags: *mut u32,
    ) -> ::windows::ErrorCode,
);
#[repr(transparent)]
#[allow(non_camel_case_types)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct IDXGIOutput4(::windows::IUnknown);
impl IDXGIOutput4 {}
unsafe impl ::windows::Interface for IDXGIOutput4 {
    type Vtable = IDXGIOutput4_abi;
    const IID: ::windows::Guid =
        ::windows::Guid::from_values(3699231285, 8598, 16717, [159, 83, 97, 120, 132, 3, 42, 96]);
}
#[allow(non_snake_case)]
impl IDXGIOutput4 {
    pub unsafe fn SetPrivateData(
        &self,
        name: *const ::windows::Guid,
        data_size: u32,
        p_data: *const ::std::ffi::c_void,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).3)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(name),
            ::std::mem::transmute(data_size),
            ::std::mem::transmute(p_data),
        )
    }
    pub unsafe fn SetPrivateDataInterface<
        'a,
        T1__: ::windows::IntoParam<'a, ::windows::IUnknown>,
    >(
        &self,
        name: *const ::windows::Guid,
        p_unknown: T1__,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).4)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(name),
            p_unknown.into_param().abi(),
        )
    }
    pub unsafe fn GetPrivateData(
        &self,
        name: *const ::windows::Guid,
        p_data_size: *mut u32,
        p_data: *mut ::std::ffi::c_void,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).5)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(name),
            ::std::mem::transmute(p_data_size),
            ::std::mem::transmute(p_data),
        )
    }
    pub unsafe fn GetParent(
        &self,
        riid: *const ::windows::Guid,
        pp_parent: *mut *mut ::std::ffi::c_void,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).6)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(riid),
            ::std::mem::transmute(pp_parent),
        )
    }
    pub unsafe fn GetDesc(&self, p_desc: *mut DXGI_OUTPUT_DESC) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).7)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(p_desc),
        )
    }
    pub unsafe fn GetDisplayModeList(
        &self,
        enum_format: DXGI_FORMAT,
        flags: u32,
        p_num_modes: *mut u32,
        p_desc: *mut DXGI_MODE_DESC,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).8)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(enum_format),
            ::std::mem::transmute(flags),
            ::std::mem::transmute(p_num_modes),
            ::std::mem::transmute(p_desc),
        )
    }
    pub unsafe fn FindClosestMatchingMode<
        'a,
        T2__: ::windows::IntoParam<'a, ::windows::IUnknown>,
    >(
        &self,
        p_mode_to_match: *const DXGI_MODE_DESC,
        p_closest_match: *mut DXGI_MODE_DESC,
        p_concerned_device: T2__,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).9)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(p_mode_to_match),
            ::std::mem::transmute(p_closest_match),
            p_concerned_device.into_param().abi(),
        )
    }
    pub unsafe fn WaitForVBlank(&self) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).10)(::windows::Abi::abi(self))
    }
    pub unsafe fn TakeOwnership<
        'a,
        T0__: ::windows::IntoParam<'a, ::windows::IUnknown>,
        T1__: ::windows::IntoParam<'a, super::system_services::BOOL>,
    >(
        &self,
        p_device: T0__,
        exclusive: T1__,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).11)(
            ::windows::Abi::abi(self),
            p_device.into_param().abi(),
            exclusive.into_param().abi(),
        )
    }
    pub unsafe fn ReleaseOwnership(&self) {
        (::windows::Interface::vtable(self).12)(::windows::Abi::abi(self))
    }
    pub unsafe fn GetGammaControlCapabilities(
        &self,
        p_gamma_caps: *mut DXGI_GAMMA_CONTROL_CAPABILITIES,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).13)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(p_gamma_caps),
        )
    }
    pub unsafe fn SetGammaControl(
        &self,
        p_array: *const DXGI_GAMMA_CONTROL,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).14)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(p_array),
        )
    }
    pub unsafe fn GetGammaControl(&self, p_array: *mut DXGI_GAMMA_CONTROL) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).15)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(p_array),
        )
    }
    pub unsafe fn SetDisplaySurface<'a, T0__: ::windows::IntoParam<'a, IDXGISurface>>(
        &self,
        p_scanout_surface: T0__,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).16)(
            ::windows::Abi::abi(self),
            p_scanout_surface.into_param().abi(),
        )
    }
    pub unsafe fn GetDisplaySurfaceData<'a, T0__: ::windows::IntoParam<'a, IDXGISurface>>(
        &self,
        p_destination: T0__,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).17)(
            ::windows::Abi::abi(self),
            p_destination.into_param().abi(),
        )
    }
    pub unsafe fn GetFrameStatistics(
        &self,
        p_stats: *mut DXGI_FRAME_STATISTICS,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).18)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(p_stats),
        )
    }
    pub unsafe fn GetDisplayModeList1(
        &self,
        enum_format: DXGI_FORMAT,
        flags: u32,
        p_num_modes: *mut u32,
        p_desc: *mut DXGI_MODE_DESC1,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).19)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(enum_format),
            ::std::mem::transmute(flags),
            ::std::mem::transmute(p_num_modes),
            ::std::mem::transmute(p_desc),
        )
    }
    pub unsafe fn FindClosestMatchingMode1<
        'a,
        T2__: ::windows::IntoParam<'a, ::windows::IUnknown>,
    >(
        &self,
        p_mode_to_match: *const DXGI_MODE_DESC1,
        p_closest_match: *mut DXGI_MODE_DESC1,
        p_concerned_device: T2__,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).20)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(p_mode_to_match),
            ::std::mem::transmute(p_closest_match),
            p_concerned_device.into_param().abi(),
        )
    }
    pub unsafe fn GetDisplaySurfaceData1<'a, T0__: ::windows::IntoParam<'a, IDXGIResource>>(
        &self,
        p_destination: T0__,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).21)(
            ::windows::Abi::abi(self),
            p_destination.into_param().abi(),
        )
    }
    pub unsafe fn DuplicateOutput<'a, T0__: ::windows::IntoParam<'a, ::windows::IUnknown>>(
        &self,
        p_device: T0__,
        pp_output_duplication: *mut ::std::option::Option<IDXGIOutputDuplication>,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).22)(
            ::windows::Abi::abi(self),
            p_device.into_param().abi(),
            ::std::mem::transmute(pp_output_duplication),
        )
    }
    pub unsafe fn SupportsOverlays(&self) -> super::system_services::BOOL {
        (::windows::Interface::vtable(self).23)(::windows::Abi::abi(self))
    }
    pub unsafe fn CheckOverlaySupport<'a, T1__: ::windows::IntoParam<'a, ::windows::IUnknown>>(
        &self,
        enum_format: DXGI_FORMAT,
        p_concerned_device: T1__,
        p_flags: *mut u32,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).24)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(enum_format),
            p_concerned_device.into_param().abi(),
            ::std::mem::transmute(p_flags),
        )
    }
    pub unsafe fn CheckOverlayColorSpaceSupport<
        'a,
        T2__: ::windows::IntoParam<'a, ::windows::IUnknown>,
    >(
        &self,
        format: DXGI_FORMAT,
        color_space: DXGI_COLOR_SPACE_TYPE,
        p_concerned_device: T2__,
        p_flags: *mut u32,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).25)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(format),
            ::std::mem::transmute(color_space),
            p_concerned_device.into_param().abi(),
            ::std::mem::transmute(p_flags),
        )
    }
}
impl ::std::convert::From<IDXGIOutput4> for ::windows::IUnknown {
    fn from(value: IDXGIOutput4) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IDXGIOutput4> for ::windows::IUnknown {
    fn from(value: &IDXGIOutput4) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::IntoParam<'a, ::windows::IUnknown> for IDXGIOutput4 {
    fn into_param(self) -> ::windows::Param<'a, ::windows::IUnknown> {
        ::windows::Param::Owned(::std::convert::Into::<::windows::IUnknown>::into(self))
    }
}
impl<'a> ::windows::IntoParam<'a, ::windows::IUnknown> for &'a IDXGIOutput4 {
    fn into_param(self) -> ::windows::Param<'a, ::windows::IUnknown> {
        ::windows::Param::Owned(::std::convert::Into::<::windows::IUnknown>::into(
            ::std::clone::Clone::clone(self),
        ))
    }
}
impl ::std::convert::From<IDXGIOutput4> for IDXGIOutput3 {
    fn from(value: IDXGIOutput4) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IDXGIOutput4> for IDXGIOutput3 {
    fn from(value: &IDXGIOutput4) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::IntoParam<'a, IDXGIOutput3> for IDXGIOutput4 {
    fn into_param(self) -> ::windows::Param<'a, IDXGIOutput3> {
        ::windows::Param::Owned(::std::convert::Into::<IDXGIOutput3>::into(self))
    }
}
impl<'a> ::windows::IntoParam<'a, IDXGIOutput3> for &'a IDXGIOutput4 {
    fn into_param(self) -> ::windows::Param<'a, IDXGIOutput3> {
        ::windows::Param::Owned(::std::convert::Into::<IDXGIOutput3>::into(
            ::std::clone::Clone::clone(self),
        ))
    }
}
impl ::std::convert::From<IDXGIOutput4> for IDXGIOutput2 {
    fn from(value: IDXGIOutput4) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IDXGIOutput4> for IDXGIOutput2 {
    fn from(value: &IDXGIOutput4) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::IntoParam<'a, IDXGIOutput2> for IDXGIOutput4 {
    fn into_param(self) -> ::windows::Param<'a, IDXGIOutput2> {
        ::windows::Param::Owned(::std::convert::Into::<IDXGIOutput2>::into(self))
    }
}
impl<'a> ::windows::IntoParam<'a, IDXGIOutput2> for &'a IDXGIOutput4 {
    fn into_param(self) -> ::windows::Param<'a, IDXGIOutput2> {
        ::windows::Param::Owned(::std::convert::Into::<IDXGIOutput2>::into(
            ::std::clone::Clone::clone(self),
        ))
    }
}
impl ::std::convert::From<IDXGIOutput4> for IDXGIOutput1 {
    fn from(value: IDXGIOutput4) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IDXGIOutput4> for IDXGIOutput1 {
    fn from(value: &IDXGIOutput4) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::IntoParam<'a, IDXGIOutput1> for IDXGIOutput4 {
    fn into_param(self) -> ::windows::Param<'a, IDXGIOutput1> {
        ::windows::Param::Owned(::std::convert::Into::<IDXGIOutput1>::into(self))
    }
}
impl<'a> ::windows::IntoParam<'a, IDXGIOutput1> for &'a IDXGIOutput4 {
    fn into_param(self) -> ::windows::Param<'a, IDXGIOutput1> {
        ::windows::Param::Owned(::std::convert::Into::<IDXGIOutput1>::into(
            ::std::clone::Clone::clone(self),
        ))
    }
}
impl ::std::convert::From<IDXGIOutput4> for IDXGIOutput {
    fn from(value: IDXGIOutput4) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IDXGIOutput4> for IDXGIOutput {
    fn from(value: &IDXGIOutput4) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::IntoParam<'a, IDXGIOutput> for IDXGIOutput4 {
    fn into_param(self) -> ::windows::Param<'a, IDXGIOutput> {
        ::windows::Param::Owned(::std::convert::Into::<IDXGIOutput>::into(self))
    }
}
impl<'a> ::windows::IntoParam<'a, IDXGIOutput> for &'a IDXGIOutput4 {
    fn into_param(self) -> ::windows::Param<'a, IDXGIOutput> {
        ::windows::Param::Owned(::std::convert::Into::<IDXGIOutput>::into(
            ::std::clone::Clone::clone(self),
        ))
    }
}
impl ::std::convert::From<IDXGIOutput4> for IDXGIObject {
    fn from(value: IDXGIOutput4) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IDXGIOutput4> for IDXGIObject {
    fn from(value: &IDXGIOutput4) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::IntoParam<'a, IDXGIObject> for IDXGIOutput4 {
    fn into_param(self) -> ::windows::Param<'a, IDXGIObject> {
        ::windows::Param::Owned(::std::convert::Into::<IDXGIObject>::into(self))
    }
}
impl<'a> ::windows::IntoParam<'a, IDXGIObject> for &'a IDXGIOutput4 {
    fn into_param(self) -> ::windows::Param<'a, IDXGIObject> {
        ::windows::Param::Owned(::std::convert::Into::<IDXGIObject>::into(
            ::std::clone::Clone::clone(self),
        ))
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDXGIOutput4_abi(
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        iid: &::windows::Guid,
        interface: *mut ::windows::RawPtr,
    ) -> ::windows::ErrorCode,
    pub unsafe extern "system" fn(this: ::windows::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::RawPtr) -> u32,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        name: *const ::windows::Guid,
        data_size: u32,
        p_data: *const ::std::ffi::c_void,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        name: *const ::windows::Guid,
        p_unknown: ::windows::RawPtr,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        name: *const ::windows::Guid,
        p_data_size: *mut u32,
        p_data: *mut ::std::ffi::c_void,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        riid: *const ::windows::Guid,
        pp_parent: *mut *mut ::std::ffi::c_void,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        p_desc: *mut DXGI_OUTPUT_DESC,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        enum_format: DXGI_FORMAT,
        flags: u32,
        p_num_modes: *mut u32,
        p_desc: *mut DXGI_MODE_DESC,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        p_mode_to_match: *const DXGI_MODE_DESC,
        p_closest_match: *mut DXGI_MODE_DESC,
        p_concerned_device: ::windows::RawPtr,
    ) -> ::windows::ErrorCode,
    pub unsafe extern "system" fn(this: ::windows::RawPtr) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        p_device: ::windows::RawPtr,
        exclusive: super::system_services::BOOL,
    ) -> ::windows::ErrorCode,
    pub unsafe extern "system" fn(this: ::windows::RawPtr),
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        p_gamma_caps: *mut DXGI_GAMMA_CONTROL_CAPABILITIES,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        p_array: *const DXGI_GAMMA_CONTROL,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        p_array: *mut DXGI_GAMMA_CONTROL,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        p_scanout_surface: ::windows::RawPtr,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        p_destination: ::windows::RawPtr,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        p_stats: *mut DXGI_FRAME_STATISTICS,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        enum_format: DXGI_FORMAT,
        flags: u32,
        p_num_modes: *mut u32,
        p_desc: *mut DXGI_MODE_DESC1,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        p_mode_to_match: *const DXGI_MODE_DESC1,
        p_closest_match: *mut DXGI_MODE_DESC1,
        p_concerned_device: ::windows::RawPtr,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        p_destination: ::windows::RawPtr,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        p_device: ::windows::RawPtr,
        pp_output_duplication: *mut ::windows::RawPtr,
    ) -> ::windows::ErrorCode,
    pub unsafe extern "system" fn(this: ::windows::RawPtr) -> super::system_services::BOOL,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        enum_format: DXGI_FORMAT,
        p_concerned_device: ::windows::RawPtr,
        p_flags: *mut u32,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        format: DXGI_FORMAT,
        color_space: DXGI_COLOR_SPACE_TYPE,
        p_concerned_device: ::windows::RawPtr,
        p_flags: *mut u32,
    ) -> ::windows::ErrorCode,
);
#[repr(transparent)]
#[allow(non_camel_case_types)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct IDXGIOutput5(::windows::IUnknown);
impl IDXGIOutput5 {}
unsafe impl ::windows::Interface for IDXGIOutput5 {
    type Vtable = IDXGIOutput5_abi;
    const IID: ::windows::Guid = ::windows::Guid::from_values(
        2157999140,
        43858,
        17131,
        [131, 60, 12, 66, 253, 40, 45, 152],
    );
}
#[allow(non_snake_case)]
impl IDXGIOutput5 {
    pub unsafe fn SetPrivateData(
        &self,
        name: *const ::windows::Guid,
        data_size: u32,
        p_data: *const ::std::ffi::c_void,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).3)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(name),
            ::std::mem::transmute(data_size),
            ::std::mem::transmute(p_data),
        )
    }
    pub unsafe fn SetPrivateDataInterface<
        'a,
        T1__: ::windows::IntoParam<'a, ::windows::IUnknown>,
    >(
        &self,
        name: *const ::windows::Guid,
        p_unknown: T1__,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).4)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(name),
            p_unknown.into_param().abi(),
        )
    }
    pub unsafe fn GetPrivateData(
        &self,
        name: *const ::windows::Guid,
        p_data_size: *mut u32,
        p_data: *mut ::std::ffi::c_void,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).5)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(name),
            ::std::mem::transmute(p_data_size),
            ::std::mem::transmute(p_data),
        )
    }
    pub unsafe fn GetParent(
        &self,
        riid: *const ::windows::Guid,
        pp_parent: *mut *mut ::std::ffi::c_void,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).6)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(riid),
            ::std::mem::transmute(pp_parent),
        )
    }
    pub unsafe fn GetDesc(&self, p_desc: *mut DXGI_OUTPUT_DESC) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).7)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(p_desc),
        )
    }
    pub unsafe fn GetDisplayModeList(
        &self,
        enum_format: DXGI_FORMAT,
        flags: u32,
        p_num_modes: *mut u32,
        p_desc: *mut DXGI_MODE_DESC,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).8)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(enum_format),
            ::std::mem::transmute(flags),
            ::std::mem::transmute(p_num_modes),
            ::std::mem::transmute(p_desc),
        )
    }
    pub unsafe fn FindClosestMatchingMode<
        'a,
        T2__: ::windows::IntoParam<'a, ::windows::IUnknown>,
    >(
        &self,
        p_mode_to_match: *const DXGI_MODE_DESC,
        p_closest_match: *mut DXGI_MODE_DESC,
        p_concerned_device: T2__,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).9)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(p_mode_to_match),
            ::std::mem::transmute(p_closest_match),
            p_concerned_device.into_param().abi(),
        )
    }
    pub unsafe fn WaitForVBlank(&self) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).10)(::windows::Abi::abi(self))
    }
    pub unsafe fn TakeOwnership<
        'a,
        T0__: ::windows::IntoParam<'a, ::windows::IUnknown>,
        T1__: ::windows::IntoParam<'a, super::system_services::BOOL>,
    >(
        &self,
        p_device: T0__,
        exclusive: T1__,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).11)(
            ::windows::Abi::abi(self),
            p_device.into_param().abi(),
            exclusive.into_param().abi(),
        )
    }
    pub unsafe fn ReleaseOwnership(&self) {
        (::windows::Interface::vtable(self).12)(::windows::Abi::abi(self))
    }
    pub unsafe fn GetGammaControlCapabilities(
        &self,
        p_gamma_caps: *mut DXGI_GAMMA_CONTROL_CAPABILITIES,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).13)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(p_gamma_caps),
        )
    }
    pub unsafe fn SetGammaControl(
        &self,
        p_array: *const DXGI_GAMMA_CONTROL,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).14)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(p_array),
        )
    }
    pub unsafe fn GetGammaControl(&self, p_array: *mut DXGI_GAMMA_CONTROL) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).15)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(p_array),
        )
    }
    pub unsafe fn SetDisplaySurface<'a, T0__: ::windows::IntoParam<'a, IDXGISurface>>(
        &self,
        p_scanout_surface: T0__,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).16)(
            ::windows::Abi::abi(self),
            p_scanout_surface.into_param().abi(),
        )
    }
    pub unsafe fn GetDisplaySurfaceData<'a, T0__: ::windows::IntoParam<'a, IDXGISurface>>(
        &self,
        p_destination: T0__,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).17)(
            ::windows::Abi::abi(self),
            p_destination.into_param().abi(),
        )
    }
    pub unsafe fn GetFrameStatistics(
        &self,
        p_stats: *mut DXGI_FRAME_STATISTICS,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).18)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(p_stats),
        )
    }
    pub unsafe fn GetDisplayModeList1(
        &self,
        enum_format: DXGI_FORMAT,
        flags: u32,
        p_num_modes: *mut u32,
        p_desc: *mut DXGI_MODE_DESC1,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).19)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(enum_format),
            ::std::mem::transmute(flags),
            ::std::mem::transmute(p_num_modes),
            ::std::mem::transmute(p_desc),
        )
    }
    pub unsafe fn FindClosestMatchingMode1<
        'a,
        T2__: ::windows::IntoParam<'a, ::windows::IUnknown>,
    >(
        &self,
        p_mode_to_match: *const DXGI_MODE_DESC1,
        p_closest_match: *mut DXGI_MODE_DESC1,
        p_concerned_device: T2__,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).20)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(p_mode_to_match),
            ::std::mem::transmute(p_closest_match),
            p_concerned_device.into_param().abi(),
        )
    }
    pub unsafe fn GetDisplaySurfaceData1<'a, T0__: ::windows::IntoParam<'a, IDXGIResource>>(
        &self,
        p_destination: T0__,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).21)(
            ::windows::Abi::abi(self),
            p_destination.into_param().abi(),
        )
    }
    pub unsafe fn DuplicateOutput<'a, T0__: ::windows::IntoParam<'a, ::windows::IUnknown>>(
        &self,
        p_device: T0__,
        pp_output_duplication: *mut ::std::option::Option<IDXGIOutputDuplication>,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).22)(
            ::windows::Abi::abi(self),
            p_device.into_param().abi(),
            ::std::mem::transmute(pp_output_duplication),
        )
    }
    pub unsafe fn SupportsOverlays(&self) -> super::system_services::BOOL {
        (::windows::Interface::vtable(self).23)(::windows::Abi::abi(self))
    }
    pub unsafe fn CheckOverlaySupport<'a, T1__: ::windows::IntoParam<'a, ::windows::IUnknown>>(
        &self,
        enum_format: DXGI_FORMAT,
        p_concerned_device: T1__,
        p_flags: *mut u32,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).24)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(enum_format),
            p_concerned_device.into_param().abi(),
            ::std::mem::transmute(p_flags),
        )
    }
    pub unsafe fn CheckOverlayColorSpaceSupport<
        'a,
        T2__: ::windows::IntoParam<'a, ::windows::IUnknown>,
    >(
        &self,
        format: DXGI_FORMAT,
        color_space: DXGI_COLOR_SPACE_TYPE,
        p_concerned_device: T2__,
        p_flags: *mut u32,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).25)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(format),
            ::std::mem::transmute(color_space),
            p_concerned_device.into_param().abi(),
            ::std::mem::transmute(p_flags),
        )
    }
    pub unsafe fn DuplicateOutput1<'a, T0__: ::windows::IntoParam<'a, ::windows::IUnknown>>(
        &self,
        p_device: T0__,
        flags: u32,
        supported_formats_count: u32,
        p_supported_formats: *const DXGI_FORMAT,
        pp_output_duplication: *mut ::std::option::Option<IDXGIOutputDuplication>,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).26)(
            ::windows::Abi::abi(self),
            p_device.into_param().abi(),
            ::std::mem::transmute(flags),
            ::std::mem::transmute(supported_formats_count),
            ::std::mem::transmute(p_supported_formats),
            ::std::mem::transmute(pp_output_duplication),
        )
    }
}
impl ::std::convert::From<IDXGIOutput5> for ::windows::IUnknown {
    fn from(value: IDXGIOutput5) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IDXGIOutput5> for ::windows::IUnknown {
    fn from(value: &IDXGIOutput5) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::IntoParam<'a, ::windows::IUnknown> for IDXGIOutput5 {
    fn into_param(self) -> ::windows::Param<'a, ::windows::IUnknown> {
        ::windows::Param::Owned(::std::convert::Into::<::windows::IUnknown>::into(self))
    }
}
impl<'a> ::windows::IntoParam<'a, ::windows::IUnknown> for &'a IDXGIOutput5 {
    fn into_param(self) -> ::windows::Param<'a, ::windows::IUnknown> {
        ::windows::Param::Owned(::std::convert::Into::<::windows::IUnknown>::into(
            ::std::clone::Clone::clone(self),
        ))
    }
}
impl ::std::convert::From<IDXGIOutput5> for IDXGIOutput4 {
    fn from(value: IDXGIOutput5) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IDXGIOutput5> for IDXGIOutput4 {
    fn from(value: &IDXGIOutput5) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::IntoParam<'a, IDXGIOutput4> for IDXGIOutput5 {
    fn into_param(self) -> ::windows::Param<'a, IDXGIOutput4> {
        ::windows::Param::Owned(::std::convert::Into::<IDXGIOutput4>::into(self))
    }
}
impl<'a> ::windows::IntoParam<'a, IDXGIOutput4> for &'a IDXGIOutput5 {
    fn into_param(self) -> ::windows::Param<'a, IDXGIOutput4> {
        ::windows::Param::Owned(::std::convert::Into::<IDXGIOutput4>::into(
            ::std::clone::Clone::clone(self),
        ))
    }
}
impl ::std::convert::From<IDXGIOutput5> for IDXGIOutput3 {
    fn from(value: IDXGIOutput5) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IDXGIOutput5> for IDXGIOutput3 {
    fn from(value: &IDXGIOutput5) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::IntoParam<'a, IDXGIOutput3> for IDXGIOutput5 {
    fn into_param(self) -> ::windows::Param<'a, IDXGIOutput3> {
        ::windows::Param::Owned(::std::convert::Into::<IDXGIOutput3>::into(self))
    }
}
impl<'a> ::windows::IntoParam<'a, IDXGIOutput3> for &'a IDXGIOutput5 {
    fn into_param(self) -> ::windows::Param<'a, IDXGIOutput3> {
        ::windows::Param::Owned(::std::convert::Into::<IDXGIOutput3>::into(
            ::std::clone::Clone::clone(self),
        ))
    }
}
impl ::std::convert::From<IDXGIOutput5> for IDXGIOutput2 {
    fn from(value: IDXGIOutput5) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IDXGIOutput5> for IDXGIOutput2 {
    fn from(value: &IDXGIOutput5) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::IntoParam<'a, IDXGIOutput2> for IDXGIOutput5 {
    fn into_param(self) -> ::windows::Param<'a, IDXGIOutput2> {
        ::windows::Param::Owned(::std::convert::Into::<IDXGIOutput2>::into(self))
    }
}
impl<'a> ::windows::IntoParam<'a, IDXGIOutput2> for &'a IDXGIOutput5 {
    fn into_param(self) -> ::windows::Param<'a, IDXGIOutput2> {
        ::windows::Param::Owned(::std::convert::Into::<IDXGIOutput2>::into(
            ::std::clone::Clone::clone(self),
        ))
    }
}
impl ::std::convert::From<IDXGIOutput5> for IDXGIOutput1 {
    fn from(value: IDXGIOutput5) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IDXGIOutput5> for IDXGIOutput1 {
    fn from(value: &IDXGIOutput5) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::IntoParam<'a, IDXGIOutput1> for IDXGIOutput5 {
    fn into_param(self) -> ::windows::Param<'a, IDXGIOutput1> {
        ::windows::Param::Owned(::std::convert::Into::<IDXGIOutput1>::into(self))
    }
}
impl<'a> ::windows::IntoParam<'a, IDXGIOutput1> for &'a IDXGIOutput5 {
    fn into_param(self) -> ::windows::Param<'a, IDXGIOutput1> {
        ::windows::Param::Owned(::std::convert::Into::<IDXGIOutput1>::into(
            ::std::clone::Clone::clone(self),
        ))
    }
}
impl ::std::convert::From<IDXGIOutput5> for IDXGIOutput {
    fn from(value: IDXGIOutput5) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IDXGIOutput5> for IDXGIOutput {
    fn from(value: &IDXGIOutput5) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::IntoParam<'a, IDXGIOutput> for IDXGIOutput5 {
    fn into_param(self) -> ::windows::Param<'a, IDXGIOutput> {
        ::windows::Param::Owned(::std::convert::Into::<IDXGIOutput>::into(self))
    }
}
impl<'a> ::windows::IntoParam<'a, IDXGIOutput> for &'a IDXGIOutput5 {
    fn into_param(self) -> ::windows::Param<'a, IDXGIOutput> {
        ::windows::Param::Owned(::std::convert::Into::<IDXGIOutput>::into(
            ::std::clone::Clone::clone(self),
        ))
    }
}
impl ::std::convert::From<IDXGIOutput5> for IDXGIObject {
    fn from(value: IDXGIOutput5) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IDXGIOutput5> for IDXGIObject {
    fn from(value: &IDXGIOutput5) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::IntoParam<'a, IDXGIObject> for IDXGIOutput5 {
    fn into_param(self) -> ::windows::Param<'a, IDXGIObject> {
        ::windows::Param::Owned(::std::convert::Into::<IDXGIObject>::into(self))
    }
}
impl<'a> ::windows::IntoParam<'a, IDXGIObject> for &'a IDXGIOutput5 {
    fn into_param(self) -> ::windows::Param<'a, IDXGIObject> {
        ::windows::Param::Owned(::std::convert::Into::<IDXGIObject>::into(
            ::std::clone::Clone::clone(self),
        ))
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDXGIOutput5_abi(
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        iid: &::windows::Guid,
        interface: *mut ::windows::RawPtr,
    ) -> ::windows::ErrorCode,
    pub unsafe extern "system" fn(this: ::windows::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::RawPtr) -> u32,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        name: *const ::windows::Guid,
        data_size: u32,
        p_data: *const ::std::ffi::c_void,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        name: *const ::windows::Guid,
        p_unknown: ::windows::RawPtr,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        name: *const ::windows::Guid,
        p_data_size: *mut u32,
        p_data: *mut ::std::ffi::c_void,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        riid: *const ::windows::Guid,
        pp_parent: *mut *mut ::std::ffi::c_void,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        p_desc: *mut DXGI_OUTPUT_DESC,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        enum_format: DXGI_FORMAT,
        flags: u32,
        p_num_modes: *mut u32,
        p_desc: *mut DXGI_MODE_DESC,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        p_mode_to_match: *const DXGI_MODE_DESC,
        p_closest_match: *mut DXGI_MODE_DESC,
        p_concerned_device: ::windows::RawPtr,
    ) -> ::windows::ErrorCode,
    pub unsafe extern "system" fn(this: ::windows::RawPtr) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        p_device: ::windows::RawPtr,
        exclusive: super::system_services::BOOL,
    ) -> ::windows::ErrorCode,
    pub unsafe extern "system" fn(this: ::windows::RawPtr),
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        p_gamma_caps: *mut DXGI_GAMMA_CONTROL_CAPABILITIES,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        p_array: *const DXGI_GAMMA_CONTROL,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        p_array: *mut DXGI_GAMMA_CONTROL,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        p_scanout_surface: ::windows::RawPtr,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        p_destination: ::windows::RawPtr,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        p_stats: *mut DXGI_FRAME_STATISTICS,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        enum_format: DXGI_FORMAT,
        flags: u32,
        p_num_modes: *mut u32,
        p_desc: *mut DXGI_MODE_DESC1,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        p_mode_to_match: *const DXGI_MODE_DESC1,
        p_closest_match: *mut DXGI_MODE_DESC1,
        p_concerned_device: ::windows::RawPtr,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        p_destination: ::windows::RawPtr,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        p_device: ::windows::RawPtr,
        pp_output_duplication: *mut ::windows::RawPtr,
    ) -> ::windows::ErrorCode,
    pub unsafe extern "system" fn(this: ::windows::RawPtr) -> super::system_services::BOOL,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        enum_format: DXGI_FORMAT,
        p_concerned_device: ::windows::RawPtr,
        p_flags: *mut u32,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        format: DXGI_FORMAT,
        color_space: DXGI_COLOR_SPACE_TYPE,
        p_concerned_device: ::windows::RawPtr,
        p_flags: *mut u32,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        p_device: ::windows::RawPtr,
        flags: u32,
        supported_formats_count: u32,
        p_supported_formats: *const DXGI_FORMAT,
        pp_output_duplication: *mut ::windows::RawPtr,
    ) -> ::windows::ErrorCode,
);
#[repr(transparent)]
#[allow(non_camel_case_types)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct IDXGIOutput6(::windows::IUnknown);
impl IDXGIOutput6 {}
unsafe impl ::windows::Interface for IDXGIOutput6 {
    type Vtable = IDXGIOutput6_abi;
    const IID: ::windows::Guid = ::windows::Guid::from_values(
        109266664,
        43756,
        19332,
        [173, 215, 19, 127, 81, 63, 119, 161],
    );
}
#[allow(non_snake_case)]
impl IDXGIOutput6 {
    pub unsafe fn SetPrivateData(
        &self,
        name: *const ::windows::Guid,
        data_size: u32,
        p_data: *const ::std::ffi::c_void,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).3)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(name),
            ::std::mem::transmute(data_size),
            ::std::mem::transmute(p_data),
        )
    }
    pub unsafe fn SetPrivateDataInterface<
        'a,
        T1__: ::windows::IntoParam<'a, ::windows::IUnknown>,
    >(
        &self,
        name: *const ::windows::Guid,
        p_unknown: T1__,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).4)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(name),
            p_unknown.into_param().abi(),
        )
    }
    pub unsafe fn GetPrivateData(
        &self,
        name: *const ::windows::Guid,
        p_data_size: *mut u32,
        p_data: *mut ::std::ffi::c_void,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).5)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(name),
            ::std::mem::transmute(p_data_size),
            ::std::mem::transmute(p_data),
        )
    }
    pub unsafe fn GetParent(
        &self,
        riid: *const ::windows::Guid,
        pp_parent: *mut *mut ::std::ffi::c_void,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).6)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(riid),
            ::std::mem::transmute(pp_parent),
        )
    }
    pub unsafe fn GetDesc(&self, p_desc: *mut DXGI_OUTPUT_DESC) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).7)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(p_desc),
        )
    }
    pub unsafe fn GetDisplayModeList(
        &self,
        enum_format: DXGI_FORMAT,
        flags: u32,
        p_num_modes: *mut u32,
        p_desc: *mut DXGI_MODE_DESC,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).8)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(enum_format),
            ::std::mem::transmute(flags),
            ::std::mem::transmute(p_num_modes),
            ::std::mem::transmute(p_desc),
        )
    }
    pub unsafe fn FindClosestMatchingMode<
        'a,
        T2__: ::windows::IntoParam<'a, ::windows::IUnknown>,
    >(
        &self,
        p_mode_to_match: *const DXGI_MODE_DESC,
        p_closest_match: *mut DXGI_MODE_DESC,
        p_concerned_device: T2__,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).9)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(p_mode_to_match),
            ::std::mem::transmute(p_closest_match),
            p_concerned_device.into_param().abi(),
        )
    }
    pub unsafe fn WaitForVBlank(&self) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).10)(::windows::Abi::abi(self))
    }
    pub unsafe fn TakeOwnership<
        'a,
        T0__: ::windows::IntoParam<'a, ::windows::IUnknown>,
        T1__: ::windows::IntoParam<'a, super::system_services::BOOL>,
    >(
        &self,
        p_device: T0__,
        exclusive: T1__,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).11)(
            ::windows::Abi::abi(self),
            p_device.into_param().abi(),
            exclusive.into_param().abi(),
        )
    }
    pub unsafe fn ReleaseOwnership(&self) {
        (::windows::Interface::vtable(self).12)(::windows::Abi::abi(self))
    }
    pub unsafe fn GetGammaControlCapabilities(
        &self,
        p_gamma_caps: *mut DXGI_GAMMA_CONTROL_CAPABILITIES,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).13)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(p_gamma_caps),
        )
    }
    pub unsafe fn SetGammaControl(
        &self,
        p_array: *const DXGI_GAMMA_CONTROL,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).14)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(p_array),
        )
    }
    pub unsafe fn GetGammaControl(&self, p_array: *mut DXGI_GAMMA_CONTROL) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).15)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(p_array),
        )
    }
    pub unsafe fn SetDisplaySurface<'a, T0__: ::windows::IntoParam<'a, IDXGISurface>>(
        &self,
        p_scanout_surface: T0__,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).16)(
            ::windows::Abi::abi(self),
            p_scanout_surface.into_param().abi(),
        )
    }
    pub unsafe fn GetDisplaySurfaceData<'a, T0__: ::windows::IntoParam<'a, IDXGISurface>>(
        &self,
        p_destination: T0__,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).17)(
            ::windows::Abi::abi(self),
            p_destination.into_param().abi(),
        )
    }
    pub unsafe fn GetFrameStatistics(
        &self,
        p_stats: *mut DXGI_FRAME_STATISTICS,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).18)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(p_stats),
        )
    }
    pub unsafe fn GetDisplayModeList1(
        &self,
        enum_format: DXGI_FORMAT,
        flags: u32,
        p_num_modes: *mut u32,
        p_desc: *mut DXGI_MODE_DESC1,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).19)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(enum_format),
            ::std::mem::transmute(flags),
            ::std::mem::transmute(p_num_modes),
            ::std::mem::transmute(p_desc),
        )
    }
    pub unsafe fn FindClosestMatchingMode1<
        'a,
        T2__: ::windows::IntoParam<'a, ::windows::IUnknown>,
    >(
        &self,
        p_mode_to_match: *const DXGI_MODE_DESC1,
        p_closest_match: *mut DXGI_MODE_DESC1,
        p_concerned_device: T2__,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).20)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(p_mode_to_match),
            ::std::mem::transmute(p_closest_match),
            p_concerned_device.into_param().abi(),
        )
    }
    pub unsafe fn GetDisplaySurfaceData1<'a, T0__: ::windows::IntoParam<'a, IDXGIResource>>(
        &self,
        p_destination: T0__,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).21)(
            ::windows::Abi::abi(self),
            p_destination.into_param().abi(),
        )
    }
    pub unsafe fn DuplicateOutput<'a, T0__: ::windows::IntoParam<'a, ::windows::IUnknown>>(
        &self,
        p_device: T0__,
        pp_output_duplication: *mut ::std::option::Option<IDXGIOutputDuplication>,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).22)(
            ::windows::Abi::abi(self),
            p_device.into_param().abi(),
            ::std::mem::transmute(pp_output_duplication),
        )
    }
    pub unsafe fn SupportsOverlays(&self) -> super::system_services::BOOL {
        (::windows::Interface::vtable(self).23)(::windows::Abi::abi(self))
    }
    pub unsafe fn CheckOverlaySupport<'a, T1__: ::windows::IntoParam<'a, ::windows::IUnknown>>(
        &self,
        enum_format: DXGI_FORMAT,
        p_concerned_device: T1__,
        p_flags: *mut u32,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).24)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(enum_format),
            p_concerned_device.into_param().abi(),
            ::std::mem::transmute(p_flags),
        )
    }
    pub unsafe fn CheckOverlayColorSpaceSupport<
        'a,
        T2__: ::windows::IntoParam<'a, ::windows::IUnknown>,
    >(
        &self,
        format: DXGI_FORMAT,
        color_space: DXGI_COLOR_SPACE_TYPE,
        p_concerned_device: T2__,
        p_flags: *mut u32,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).25)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(format),
            ::std::mem::transmute(color_space),
            p_concerned_device.into_param().abi(),
            ::std::mem::transmute(p_flags),
        )
    }
    pub unsafe fn DuplicateOutput1<'a, T0__: ::windows::IntoParam<'a, ::windows::IUnknown>>(
        &self,
        p_device: T0__,
        flags: u32,
        supported_formats_count: u32,
        p_supported_formats: *const DXGI_FORMAT,
        pp_output_duplication: *mut ::std::option::Option<IDXGIOutputDuplication>,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).26)(
            ::windows::Abi::abi(self),
            p_device.into_param().abi(),
            ::std::mem::transmute(flags),
            ::std::mem::transmute(supported_formats_count),
            ::std::mem::transmute(p_supported_formats),
            ::std::mem::transmute(pp_output_duplication),
        )
    }
    pub unsafe fn GetDesc1(&self, p_desc: *mut DXGI_OUTPUT_DESC1) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).27)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(p_desc),
        )
    }
    pub unsafe fn CheckHardwareCompositionSupport(
        &self,
        p_flags: *mut u32,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).28)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(p_flags),
        )
    }
}
impl ::std::convert::From<IDXGIOutput6> for ::windows::IUnknown {
    fn from(value: IDXGIOutput6) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IDXGIOutput6> for ::windows::IUnknown {
    fn from(value: &IDXGIOutput6) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::IntoParam<'a, ::windows::IUnknown> for IDXGIOutput6 {
    fn into_param(self) -> ::windows::Param<'a, ::windows::IUnknown> {
        ::windows::Param::Owned(::std::convert::Into::<::windows::IUnknown>::into(self))
    }
}
impl<'a> ::windows::IntoParam<'a, ::windows::IUnknown> for &'a IDXGIOutput6 {
    fn into_param(self) -> ::windows::Param<'a, ::windows::IUnknown> {
        ::windows::Param::Owned(::std::convert::Into::<::windows::IUnknown>::into(
            ::std::clone::Clone::clone(self),
        ))
    }
}
impl ::std::convert::From<IDXGIOutput6> for IDXGIOutput5 {
    fn from(value: IDXGIOutput6) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IDXGIOutput6> for IDXGIOutput5 {
    fn from(value: &IDXGIOutput6) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::IntoParam<'a, IDXGIOutput5> for IDXGIOutput6 {
    fn into_param(self) -> ::windows::Param<'a, IDXGIOutput5> {
        ::windows::Param::Owned(::std::convert::Into::<IDXGIOutput5>::into(self))
    }
}
impl<'a> ::windows::IntoParam<'a, IDXGIOutput5> for &'a IDXGIOutput6 {
    fn into_param(self) -> ::windows::Param<'a, IDXGIOutput5> {
        ::windows::Param::Owned(::std::convert::Into::<IDXGIOutput5>::into(
            ::std::clone::Clone::clone(self),
        ))
    }
}
impl ::std::convert::From<IDXGIOutput6> for IDXGIOutput4 {
    fn from(value: IDXGIOutput6) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IDXGIOutput6> for IDXGIOutput4 {
    fn from(value: &IDXGIOutput6) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::IntoParam<'a, IDXGIOutput4> for IDXGIOutput6 {
    fn into_param(self) -> ::windows::Param<'a, IDXGIOutput4> {
        ::windows::Param::Owned(::std::convert::Into::<IDXGIOutput4>::into(self))
    }
}
impl<'a> ::windows::IntoParam<'a, IDXGIOutput4> for &'a IDXGIOutput6 {
    fn into_param(self) -> ::windows::Param<'a, IDXGIOutput4> {
        ::windows::Param::Owned(::std::convert::Into::<IDXGIOutput4>::into(
            ::std::clone::Clone::clone(self),
        ))
    }
}
impl ::std::convert::From<IDXGIOutput6> for IDXGIOutput3 {
    fn from(value: IDXGIOutput6) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IDXGIOutput6> for IDXGIOutput3 {
    fn from(value: &IDXGIOutput6) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::IntoParam<'a, IDXGIOutput3> for IDXGIOutput6 {
    fn into_param(self) -> ::windows::Param<'a, IDXGIOutput3> {
        ::windows::Param::Owned(::std::convert::Into::<IDXGIOutput3>::into(self))
    }
}
impl<'a> ::windows::IntoParam<'a, IDXGIOutput3> for &'a IDXGIOutput6 {
    fn into_param(self) -> ::windows::Param<'a, IDXGIOutput3> {
        ::windows::Param::Owned(::std::convert::Into::<IDXGIOutput3>::into(
            ::std::clone::Clone::clone(self),
        ))
    }
}
impl ::std::convert::From<IDXGIOutput6> for IDXGIOutput2 {
    fn from(value: IDXGIOutput6) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IDXGIOutput6> for IDXGIOutput2 {
    fn from(value: &IDXGIOutput6) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::IntoParam<'a, IDXGIOutput2> for IDXGIOutput6 {
    fn into_param(self) -> ::windows::Param<'a, IDXGIOutput2> {
        ::windows::Param::Owned(::std::convert::Into::<IDXGIOutput2>::into(self))
    }
}
impl<'a> ::windows::IntoParam<'a, IDXGIOutput2> for &'a IDXGIOutput6 {
    fn into_param(self) -> ::windows::Param<'a, IDXGIOutput2> {
        ::windows::Param::Owned(::std::convert::Into::<IDXGIOutput2>::into(
            ::std::clone::Clone::clone(self),
        ))
    }
}
impl ::std::convert::From<IDXGIOutput6> for IDXGIOutput1 {
    fn from(value: IDXGIOutput6) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IDXGIOutput6> for IDXGIOutput1 {
    fn from(value: &IDXGIOutput6) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::IntoParam<'a, IDXGIOutput1> for IDXGIOutput6 {
    fn into_param(self) -> ::windows::Param<'a, IDXGIOutput1> {
        ::windows::Param::Owned(::std::convert::Into::<IDXGIOutput1>::into(self))
    }
}
impl<'a> ::windows::IntoParam<'a, IDXGIOutput1> for &'a IDXGIOutput6 {
    fn into_param(self) -> ::windows::Param<'a, IDXGIOutput1> {
        ::windows::Param::Owned(::std::convert::Into::<IDXGIOutput1>::into(
            ::std::clone::Clone::clone(self),
        ))
    }
}
impl ::std::convert::From<IDXGIOutput6> for IDXGIOutput {
    fn from(value: IDXGIOutput6) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IDXGIOutput6> for IDXGIOutput {
    fn from(value: &IDXGIOutput6) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::IntoParam<'a, IDXGIOutput> for IDXGIOutput6 {
    fn into_param(self) -> ::windows::Param<'a, IDXGIOutput> {
        ::windows::Param::Owned(::std::convert::Into::<IDXGIOutput>::into(self))
    }
}
impl<'a> ::windows::IntoParam<'a, IDXGIOutput> for &'a IDXGIOutput6 {
    fn into_param(self) -> ::windows::Param<'a, IDXGIOutput> {
        ::windows::Param::Owned(::std::convert::Into::<IDXGIOutput>::into(
            ::std::clone::Clone::clone(self),
        ))
    }
}
impl ::std::convert::From<IDXGIOutput6> for IDXGIObject {
    fn from(value: IDXGIOutput6) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IDXGIOutput6> for IDXGIObject {
    fn from(value: &IDXGIOutput6) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::IntoParam<'a, IDXGIObject> for IDXGIOutput6 {
    fn into_param(self) -> ::windows::Param<'a, IDXGIObject> {
        ::windows::Param::Owned(::std::convert::Into::<IDXGIObject>::into(self))
    }
}
impl<'a> ::windows::IntoParam<'a, IDXGIObject> for &'a IDXGIOutput6 {
    fn into_param(self) -> ::windows::Param<'a, IDXGIObject> {
        ::windows::Param::Owned(::std::convert::Into::<IDXGIObject>::into(
            ::std::clone::Clone::clone(self),
        ))
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDXGIOutput6_abi(
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        iid: &::windows::Guid,
        interface: *mut ::windows::RawPtr,
    ) -> ::windows::ErrorCode,
    pub unsafe extern "system" fn(this: ::windows::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::RawPtr) -> u32,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        name: *const ::windows::Guid,
        data_size: u32,
        p_data: *const ::std::ffi::c_void,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        name: *const ::windows::Guid,
        p_unknown: ::windows::RawPtr,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        name: *const ::windows::Guid,
        p_data_size: *mut u32,
        p_data: *mut ::std::ffi::c_void,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        riid: *const ::windows::Guid,
        pp_parent: *mut *mut ::std::ffi::c_void,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        p_desc: *mut DXGI_OUTPUT_DESC,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        enum_format: DXGI_FORMAT,
        flags: u32,
        p_num_modes: *mut u32,
        p_desc: *mut DXGI_MODE_DESC,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        p_mode_to_match: *const DXGI_MODE_DESC,
        p_closest_match: *mut DXGI_MODE_DESC,
        p_concerned_device: ::windows::RawPtr,
    ) -> ::windows::ErrorCode,
    pub unsafe extern "system" fn(this: ::windows::RawPtr) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        p_device: ::windows::RawPtr,
        exclusive: super::system_services::BOOL,
    ) -> ::windows::ErrorCode,
    pub unsafe extern "system" fn(this: ::windows::RawPtr),
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        p_gamma_caps: *mut DXGI_GAMMA_CONTROL_CAPABILITIES,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        p_array: *const DXGI_GAMMA_CONTROL,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        p_array: *mut DXGI_GAMMA_CONTROL,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        p_scanout_surface: ::windows::RawPtr,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        p_destination: ::windows::RawPtr,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        p_stats: *mut DXGI_FRAME_STATISTICS,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        enum_format: DXGI_FORMAT,
        flags: u32,
        p_num_modes: *mut u32,
        p_desc: *mut DXGI_MODE_DESC1,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        p_mode_to_match: *const DXGI_MODE_DESC1,
        p_closest_match: *mut DXGI_MODE_DESC1,
        p_concerned_device: ::windows::RawPtr,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        p_destination: ::windows::RawPtr,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        p_device: ::windows::RawPtr,
        pp_output_duplication: *mut ::windows::RawPtr,
    ) -> ::windows::ErrorCode,
    pub unsafe extern "system" fn(this: ::windows::RawPtr) -> super::system_services::BOOL,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        enum_format: DXGI_FORMAT,
        p_concerned_device: ::windows::RawPtr,
        p_flags: *mut u32,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        format: DXGI_FORMAT,
        color_space: DXGI_COLOR_SPACE_TYPE,
        p_concerned_device: ::windows::RawPtr,
        p_flags: *mut u32,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        p_device: ::windows::RawPtr,
        flags: u32,
        supported_formats_count: u32,
        p_supported_formats: *const DXGI_FORMAT,
        pp_output_duplication: *mut ::windows::RawPtr,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        p_desc: *mut DXGI_OUTPUT_DESC1,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        p_flags: *mut u32,
    ) -> ::windows::ErrorCode,
);
#[repr(transparent)]
#[allow(non_camel_case_types)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct IDXGIOutputDuplication(::windows::IUnknown);
impl IDXGIOutputDuplication {}
unsafe impl ::windows::Interface for IDXGIOutputDuplication {
    type Vtable = IDXGIOutputDuplication_abi;
    const IID: ::windows::Guid = ::windows::Guid::from_values(
        421329603,
        41793,
        18189,
        [178, 110, 168, 100, 244, 40, 49, 156],
    );
}
#[allow(non_snake_case)]
impl IDXGIOutputDuplication {
    pub unsafe fn SetPrivateData(
        &self,
        name: *const ::windows::Guid,
        data_size: u32,
        p_data: *const ::std::ffi::c_void,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).3)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(name),
            ::std::mem::transmute(data_size),
            ::std::mem::transmute(p_data),
        )
    }
    pub unsafe fn SetPrivateDataInterface<
        'a,
        T1__: ::windows::IntoParam<'a, ::windows::IUnknown>,
    >(
        &self,
        name: *const ::windows::Guid,
        p_unknown: T1__,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).4)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(name),
            p_unknown.into_param().abi(),
        )
    }
    pub unsafe fn GetPrivateData(
        &self,
        name: *const ::windows::Guid,
        p_data_size: *mut u32,
        p_data: *mut ::std::ffi::c_void,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).5)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(name),
            ::std::mem::transmute(p_data_size),
            ::std::mem::transmute(p_data),
        )
    }
    pub unsafe fn GetParent(
        &self,
        riid: *const ::windows::Guid,
        pp_parent: *mut *mut ::std::ffi::c_void,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).6)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(riid),
            ::std::mem::transmute(pp_parent),
        )
    }
    pub unsafe fn GetDesc(&self, p_desc: *mut DXGI_OUTDUPL_DESC) {
        (::windows::Interface::vtable(self).7)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(p_desc),
        )
    }
    pub unsafe fn AcquireNextFrame(
        &self,
        timeout_in_milliseconds: u32,
        p_frame_info: *mut DXGI_OUTDUPL_FRAME_INFO,
        pp_desktop_resource: *mut ::std::option::Option<IDXGIResource>,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).8)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(timeout_in_milliseconds),
            ::std::mem::transmute(p_frame_info),
            ::std::mem::transmute(pp_desktop_resource),
        )
    }
    pub unsafe fn GetFrameDirtyRects(
        &self,
        dirty_rects_buffer_size: u32,
        p_dirty_rects_buffer: *mut super::display_devices::RECT,
        p_dirty_rects_buffer_size_required: *mut u32,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).9)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(dirty_rects_buffer_size),
            ::std::mem::transmute(p_dirty_rects_buffer),
            ::std::mem::transmute(p_dirty_rects_buffer_size_required),
        )
    }
    pub unsafe fn GetFrameMoveRects(
        &self,
        move_rects_buffer_size: u32,
        p_move_rect_buffer: *mut DXGI_OUTDUPL_MOVE_RECT,
        p_move_rects_buffer_size_required: *mut u32,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).10)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(move_rects_buffer_size),
            ::std::mem::transmute(p_move_rect_buffer),
            ::std::mem::transmute(p_move_rects_buffer_size_required),
        )
    }
    pub unsafe fn GetFramePointerShape(
        &self,
        pointer_shape_buffer_size: u32,
        p_pointer_shape_buffer: *mut ::std::ffi::c_void,
        p_pointer_shape_buffer_size_required: *mut u32,
        p_pointer_shape_info: *mut DXGI_OUTDUPL_POINTER_SHAPE_INFO,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).11)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(pointer_shape_buffer_size),
            ::std::mem::transmute(p_pointer_shape_buffer),
            ::std::mem::transmute(p_pointer_shape_buffer_size_required),
            ::std::mem::transmute(p_pointer_shape_info),
        )
    }
    pub unsafe fn MapDesktopSurface(
        &self,
        p_locked_rect: *mut DXGI_MAPPED_RECT,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).12)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(p_locked_rect),
        )
    }
    pub unsafe fn UnMapDesktopSurface(&self) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).13)(::windows::Abi::abi(self))
    }
    pub unsafe fn ReleaseFrame(&self) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).14)(::windows::Abi::abi(self))
    }
}
impl ::std::convert::From<IDXGIOutputDuplication> for ::windows::IUnknown {
    fn from(value: IDXGIOutputDuplication) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IDXGIOutputDuplication> for ::windows::IUnknown {
    fn from(value: &IDXGIOutputDuplication) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::IntoParam<'a, ::windows::IUnknown> for IDXGIOutputDuplication {
    fn into_param(self) -> ::windows::Param<'a, ::windows::IUnknown> {
        ::windows::Param::Owned(::std::convert::Into::<::windows::IUnknown>::into(self))
    }
}
impl<'a> ::windows::IntoParam<'a, ::windows::IUnknown> for &'a IDXGIOutputDuplication {
    fn into_param(self) -> ::windows::Param<'a, ::windows::IUnknown> {
        ::windows::Param::Owned(::std::convert::Into::<::windows::IUnknown>::into(
            ::std::clone::Clone::clone(self),
        ))
    }
}
impl ::std::convert::From<IDXGIOutputDuplication> for IDXGIObject {
    fn from(value: IDXGIOutputDuplication) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IDXGIOutputDuplication> for IDXGIObject {
    fn from(value: &IDXGIOutputDuplication) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::IntoParam<'a, IDXGIObject> for IDXGIOutputDuplication {
    fn into_param(self) -> ::windows::Param<'a, IDXGIObject> {
        ::windows::Param::Owned(::std::convert::Into::<IDXGIObject>::into(self))
    }
}
impl<'a> ::windows::IntoParam<'a, IDXGIObject> for &'a IDXGIOutputDuplication {
    fn into_param(self) -> ::windows::Param<'a, IDXGIObject> {
        ::windows::Param::Owned(::std::convert::Into::<IDXGIObject>::into(
            ::std::clone::Clone::clone(self),
        ))
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDXGIOutputDuplication_abi(
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        iid: &::windows::Guid,
        interface: *mut ::windows::RawPtr,
    ) -> ::windows::ErrorCode,
    pub unsafe extern "system" fn(this: ::windows::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::RawPtr) -> u32,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        name: *const ::windows::Guid,
        data_size: u32,
        p_data: *const ::std::ffi::c_void,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        name: *const ::windows::Guid,
        p_unknown: ::windows::RawPtr,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        name: *const ::windows::Guid,
        p_data_size: *mut u32,
        p_data: *mut ::std::ffi::c_void,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        riid: *const ::windows::Guid,
        pp_parent: *mut *mut ::std::ffi::c_void,
    ) -> ::windows::ErrorCode,
    pub unsafe extern "system" fn(this: ::windows::RawPtr, p_desc: *mut DXGI_OUTDUPL_DESC),
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        timeout_in_milliseconds: u32,
        p_frame_info: *mut DXGI_OUTDUPL_FRAME_INFO,
        pp_desktop_resource: *mut ::windows::RawPtr,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        dirty_rects_buffer_size: u32,
        p_dirty_rects_buffer: *mut super::display_devices::RECT,
        p_dirty_rects_buffer_size_required: *mut u32,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        move_rects_buffer_size: u32,
        p_move_rect_buffer: *mut DXGI_OUTDUPL_MOVE_RECT,
        p_move_rects_buffer_size_required: *mut u32,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        pointer_shape_buffer_size: u32,
        p_pointer_shape_buffer: *mut ::std::ffi::c_void,
        p_pointer_shape_buffer_size_required: *mut u32,
        p_pointer_shape_info: *mut DXGI_OUTDUPL_POINTER_SHAPE_INFO,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        p_locked_rect: *mut DXGI_MAPPED_RECT,
    ) -> ::windows::ErrorCode,
    pub unsafe extern "system" fn(this: ::windows::RawPtr) -> ::windows::ErrorCode,
    pub unsafe extern "system" fn(this: ::windows::RawPtr) -> ::windows::ErrorCode,
);
#[repr(transparent)]
#[allow(non_camel_case_types)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct IDXGIResource(::windows::IUnknown);
impl IDXGIResource {}
unsafe impl ::windows::Interface for IDXGIResource {
    type Vtable = IDXGIResource_abi;
    const IID: ::windows::Guid = ::windows::Guid::from_values(
        56572596,
        18478,
        20048,
        [180, 31, 138, 127, 139, 216, 150, 11],
    );
}
#[allow(non_snake_case)]
impl IDXGIResource {
    pub unsafe fn SetPrivateData(
        &self,
        name: *const ::windows::Guid,
        data_size: u32,
        p_data: *const ::std::ffi::c_void,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).3)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(name),
            ::std::mem::transmute(data_size),
            ::std::mem::transmute(p_data),
        )
    }
    pub unsafe fn SetPrivateDataInterface<
        'a,
        T1__: ::windows::IntoParam<'a, ::windows::IUnknown>,
    >(
        &self,
        name: *const ::windows::Guid,
        p_unknown: T1__,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).4)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(name),
            p_unknown.into_param().abi(),
        )
    }
    pub unsafe fn GetPrivateData(
        &self,
        name: *const ::windows::Guid,
        p_data_size: *mut u32,
        p_data: *mut ::std::ffi::c_void,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).5)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(name),
            ::std::mem::transmute(p_data_size),
            ::std::mem::transmute(p_data),
        )
    }
    pub unsafe fn GetParent(
        &self,
        riid: *const ::windows::Guid,
        pp_parent: *mut *mut ::std::ffi::c_void,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).6)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(riid),
            ::std::mem::transmute(pp_parent),
        )
    }
    pub unsafe fn GetDevice(
        &self,
        riid: *const ::windows::Guid,
        pp_device: *mut *mut ::std::ffi::c_void,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).7)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(riid),
            ::std::mem::transmute(pp_device),
        )
    }
    pub unsafe fn GetSharedHandle(
        &self,
        p_shared_handle: *mut super::system_services::HANDLE,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).8)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(p_shared_handle),
        )
    }
    pub unsafe fn GetUsage(&self, p_usage: *mut u32) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).9)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(p_usage),
        )
    }
    pub unsafe fn SetEvictionPriority(&self, eviction_priority: u32) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).10)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(eviction_priority),
        )
    }
    pub unsafe fn GetEvictionPriority(
        &self,
        p_eviction_priority: *mut u32,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).11)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(p_eviction_priority),
        )
    }
}
impl ::std::convert::From<IDXGIResource> for ::windows::IUnknown {
    fn from(value: IDXGIResource) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IDXGIResource> for ::windows::IUnknown {
    fn from(value: &IDXGIResource) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::IntoParam<'a, ::windows::IUnknown> for IDXGIResource {
    fn into_param(self) -> ::windows::Param<'a, ::windows::IUnknown> {
        ::windows::Param::Owned(::std::convert::Into::<::windows::IUnknown>::into(self))
    }
}
impl<'a> ::windows::IntoParam<'a, ::windows::IUnknown> for &'a IDXGIResource {
    fn into_param(self) -> ::windows::Param<'a, ::windows::IUnknown> {
        ::windows::Param::Owned(::std::convert::Into::<::windows::IUnknown>::into(
            ::std::clone::Clone::clone(self),
        ))
    }
}
impl ::std::convert::From<IDXGIResource> for IDXGIDeviceSubObject {
    fn from(value: IDXGIResource) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IDXGIResource> for IDXGIDeviceSubObject {
    fn from(value: &IDXGIResource) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::IntoParam<'a, IDXGIDeviceSubObject> for IDXGIResource {
    fn into_param(self) -> ::windows::Param<'a, IDXGIDeviceSubObject> {
        ::windows::Param::Owned(::std::convert::Into::<IDXGIDeviceSubObject>::into(self))
    }
}
impl<'a> ::windows::IntoParam<'a, IDXGIDeviceSubObject> for &'a IDXGIResource {
    fn into_param(self) -> ::windows::Param<'a, IDXGIDeviceSubObject> {
        ::windows::Param::Owned(::std::convert::Into::<IDXGIDeviceSubObject>::into(
            ::std::clone::Clone::clone(self),
        ))
    }
}
impl ::std::convert::From<IDXGIResource> for IDXGIObject {
    fn from(value: IDXGIResource) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IDXGIResource> for IDXGIObject {
    fn from(value: &IDXGIResource) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::IntoParam<'a, IDXGIObject> for IDXGIResource {
    fn into_param(self) -> ::windows::Param<'a, IDXGIObject> {
        ::windows::Param::Owned(::std::convert::Into::<IDXGIObject>::into(self))
    }
}
impl<'a> ::windows::IntoParam<'a, IDXGIObject> for &'a IDXGIResource {
    fn into_param(self) -> ::windows::Param<'a, IDXGIObject> {
        ::windows::Param::Owned(::std::convert::Into::<IDXGIObject>::into(
            ::std::clone::Clone::clone(self),
        ))
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDXGIResource_abi(
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        iid: &::windows::Guid,
        interface: *mut ::windows::RawPtr,
    ) -> ::windows::ErrorCode,
    pub unsafe extern "system" fn(this: ::windows::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::RawPtr) -> u32,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        name: *const ::windows::Guid,
        data_size: u32,
        p_data: *const ::std::ffi::c_void,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        name: *const ::windows::Guid,
        p_unknown: ::windows::RawPtr,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        name: *const ::windows::Guid,
        p_data_size: *mut u32,
        p_data: *mut ::std::ffi::c_void,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        riid: *const ::windows::Guid,
        pp_parent: *mut *mut ::std::ffi::c_void,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        riid: *const ::windows::Guid,
        pp_device: *mut *mut ::std::ffi::c_void,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        p_shared_handle: *mut super::system_services::HANDLE,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        p_usage: *mut u32,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        eviction_priority: u32,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        p_eviction_priority: *mut u32,
    ) -> ::windows::ErrorCode,
);
#[repr(transparent)]
#[allow(non_camel_case_types)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct IDXGIResource1(::windows::IUnknown);
impl IDXGIResource1 {}
unsafe impl ::windows::Interface for IDXGIResource1 {
    type Vtable = IDXGIResource1_abi;
    const IID: ::windows::Guid = ::windows::Guid::from_values(
        815141753,
        17929,
        19009,
        [153, 142, 84, 254, 86, 126, 224, 193],
    );
}
#[allow(non_snake_case)]
impl IDXGIResource1 {
    pub unsafe fn SetPrivateData(
        &self,
        name: *const ::windows::Guid,
        data_size: u32,
        p_data: *const ::std::ffi::c_void,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).3)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(name),
            ::std::mem::transmute(data_size),
            ::std::mem::transmute(p_data),
        )
    }
    pub unsafe fn SetPrivateDataInterface<
        'a,
        T1__: ::windows::IntoParam<'a, ::windows::IUnknown>,
    >(
        &self,
        name: *const ::windows::Guid,
        p_unknown: T1__,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).4)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(name),
            p_unknown.into_param().abi(),
        )
    }
    pub unsafe fn GetPrivateData(
        &self,
        name: *const ::windows::Guid,
        p_data_size: *mut u32,
        p_data: *mut ::std::ffi::c_void,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).5)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(name),
            ::std::mem::transmute(p_data_size),
            ::std::mem::transmute(p_data),
        )
    }
    pub unsafe fn GetParent(
        &self,
        riid: *const ::windows::Guid,
        pp_parent: *mut *mut ::std::ffi::c_void,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).6)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(riid),
            ::std::mem::transmute(pp_parent),
        )
    }
    pub unsafe fn GetDevice(
        &self,
        riid: *const ::windows::Guid,
        pp_device: *mut *mut ::std::ffi::c_void,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).7)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(riid),
            ::std::mem::transmute(pp_device),
        )
    }
    pub unsafe fn GetSharedHandle(
        &self,
        p_shared_handle: *mut super::system_services::HANDLE,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).8)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(p_shared_handle),
        )
    }
    pub unsafe fn GetUsage(&self, p_usage: *mut u32) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).9)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(p_usage),
        )
    }
    pub unsafe fn SetEvictionPriority(&self, eviction_priority: u32) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).10)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(eviction_priority),
        )
    }
    pub unsafe fn GetEvictionPriority(
        &self,
        p_eviction_priority: *mut u32,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).11)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(p_eviction_priority),
        )
    }
    pub unsafe fn CreateSubresourceSurface(
        &self,
        index: u32,
        pp_surface: *mut ::std::option::Option<IDXGISurface2>,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).12)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(index),
            ::std::mem::transmute(pp_surface),
        )
    }
    pub unsafe fn CreateSharedHandle<
        'a,
        T2__: ::windows::IntoParam<'a, super::system_services::PWSTR>,
    >(
        &self,
        p_attributes: *const super::system_services::SECURITY_ATTRIBUTES,
        dw_access: u32,
        lp_name: T2__,
        p_handle: *mut super::system_services::HANDLE,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).13)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(p_attributes),
            ::std::mem::transmute(dw_access),
            lp_name.into_param().abi(),
            ::std::mem::transmute(p_handle),
        )
    }
}
impl ::std::convert::From<IDXGIResource1> for ::windows::IUnknown {
    fn from(value: IDXGIResource1) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IDXGIResource1> for ::windows::IUnknown {
    fn from(value: &IDXGIResource1) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::IntoParam<'a, ::windows::IUnknown> for IDXGIResource1 {
    fn into_param(self) -> ::windows::Param<'a, ::windows::IUnknown> {
        ::windows::Param::Owned(::std::convert::Into::<::windows::IUnknown>::into(self))
    }
}
impl<'a> ::windows::IntoParam<'a, ::windows::IUnknown> for &'a IDXGIResource1 {
    fn into_param(self) -> ::windows::Param<'a, ::windows::IUnknown> {
        ::windows::Param::Owned(::std::convert::Into::<::windows::IUnknown>::into(
            ::std::clone::Clone::clone(self),
        ))
    }
}
impl ::std::convert::From<IDXGIResource1> for IDXGIResource {
    fn from(value: IDXGIResource1) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IDXGIResource1> for IDXGIResource {
    fn from(value: &IDXGIResource1) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::IntoParam<'a, IDXGIResource> for IDXGIResource1 {
    fn into_param(self) -> ::windows::Param<'a, IDXGIResource> {
        ::windows::Param::Owned(::std::convert::Into::<IDXGIResource>::into(self))
    }
}
impl<'a> ::windows::IntoParam<'a, IDXGIResource> for &'a IDXGIResource1 {
    fn into_param(self) -> ::windows::Param<'a, IDXGIResource> {
        ::windows::Param::Owned(::std::convert::Into::<IDXGIResource>::into(
            ::std::clone::Clone::clone(self),
        ))
    }
}
impl ::std::convert::From<IDXGIResource1> for IDXGIDeviceSubObject {
    fn from(value: IDXGIResource1) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IDXGIResource1> for IDXGIDeviceSubObject {
    fn from(value: &IDXGIResource1) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::IntoParam<'a, IDXGIDeviceSubObject> for IDXGIResource1 {
    fn into_param(self) -> ::windows::Param<'a, IDXGIDeviceSubObject> {
        ::windows::Param::Owned(::std::convert::Into::<IDXGIDeviceSubObject>::into(self))
    }
}
impl<'a> ::windows::IntoParam<'a, IDXGIDeviceSubObject> for &'a IDXGIResource1 {
    fn into_param(self) -> ::windows::Param<'a, IDXGIDeviceSubObject> {
        ::windows::Param::Owned(::std::convert::Into::<IDXGIDeviceSubObject>::into(
            ::std::clone::Clone::clone(self),
        ))
    }
}
impl ::std::convert::From<IDXGIResource1> for IDXGIObject {
    fn from(value: IDXGIResource1) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IDXGIResource1> for IDXGIObject {
    fn from(value: &IDXGIResource1) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::IntoParam<'a, IDXGIObject> for IDXGIResource1 {
    fn into_param(self) -> ::windows::Param<'a, IDXGIObject> {
        ::windows::Param::Owned(::std::convert::Into::<IDXGIObject>::into(self))
    }
}
impl<'a> ::windows::IntoParam<'a, IDXGIObject> for &'a IDXGIResource1 {
    fn into_param(self) -> ::windows::Param<'a, IDXGIObject> {
        ::windows::Param::Owned(::std::convert::Into::<IDXGIObject>::into(
            ::std::clone::Clone::clone(self),
        ))
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDXGIResource1_abi(
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        iid: &::windows::Guid,
        interface: *mut ::windows::RawPtr,
    ) -> ::windows::ErrorCode,
    pub unsafe extern "system" fn(this: ::windows::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::RawPtr) -> u32,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        name: *const ::windows::Guid,
        data_size: u32,
        p_data: *const ::std::ffi::c_void,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        name: *const ::windows::Guid,
        p_unknown: ::windows::RawPtr,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        name: *const ::windows::Guid,
        p_data_size: *mut u32,
        p_data: *mut ::std::ffi::c_void,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        riid: *const ::windows::Guid,
        pp_parent: *mut *mut ::std::ffi::c_void,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        riid: *const ::windows::Guid,
        pp_device: *mut *mut ::std::ffi::c_void,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        p_shared_handle: *mut super::system_services::HANDLE,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        p_usage: *mut u32,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        eviction_priority: u32,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        p_eviction_priority: *mut u32,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        index: u32,
        pp_surface: *mut ::windows::RawPtr,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        p_attributes: *const super::system_services::SECURITY_ATTRIBUTES,
        dw_access: u32,
        lp_name: super::system_services::PWSTR,
        p_handle: *mut super::system_services::HANDLE,
    ) -> ::windows::ErrorCode,
);
#[repr(transparent)]
#[allow(non_camel_case_types)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct IDXGISurface(::windows::IUnknown);
impl IDXGISurface {}
unsafe impl ::windows::Interface for IDXGISurface {
    type Vtable = IDXGISurface_abi;
    const IID: ::windows::Guid = ::windows::Guid::from_values(
        3405559148,
        27331,
        18569,
        [191, 71, 158, 35, 187, 210, 96, 236],
    );
}
#[allow(non_snake_case)]
impl IDXGISurface {
    pub unsafe fn SetPrivateData(
        &self,
        name: *const ::windows::Guid,
        data_size: u32,
        p_data: *const ::std::ffi::c_void,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).3)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(name),
            ::std::mem::transmute(data_size),
            ::std::mem::transmute(p_data),
        )
    }
    pub unsafe fn SetPrivateDataInterface<
        'a,
        T1__: ::windows::IntoParam<'a, ::windows::IUnknown>,
    >(
        &self,
        name: *const ::windows::Guid,
        p_unknown: T1__,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).4)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(name),
            p_unknown.into_param().abi(),
        )
    }
    pub unsafe fn GetPrivateData(
        &self,
        name: *const ::windows::Guid,
        p_data_size: *mut u32,
        p_data: *mut ::std::ffi::c_void,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).5)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(name),
            ::std::mem::transmute(p_data_size),
            ::std::mem::transmute(p_data),
        )
    }
    pub unsafe fn GetParent(
        &self,
        riid: *const ::windows::Guid,
        pp_parent: *mut *mut ::std::ffi::c_void,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).6)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(riid),
            ::std::mem::transmute(pp_parent),
        )
    }
    pub unsafe fn GetDevice(
        &self,
        riid: *const ::windows::Guid,
        pp_device: *mut *mut ::std::ffi::c_void,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).7)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(riid),
            ::std::mem::transmute(pp_device),
        )
    }
    pub unsafe fn GetDesc(&self, p_desc: *mut DXGI_SURFACE_DESC) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).8)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(p_desc),
        )
    }
    pub unsafe fn Map(
        &self,
        p_locked_rect: *mut DXGI_MAPPED_RECT,
        map_flags: u32,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).9)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(p_locked_rect),
            ::std::mem::transmute(map_flags),
        )
    }
    pub unsafe fn Unmap(&self) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).10)(::windows::Abi::abi(self))
    }
}
impl ::std::convert::From<IDXGISurface> for ::windows::IUnknown {
    fn from(value: IDXGISurface) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IDXGISurface> for ::windows::IUnknown {
    fn from(value: &IDXGISurface) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::IntoParam<'a, ::windows::IUnknown> for IDXGISurface {
    fn into_param(self) -> ::windows::Param<'a, ::windows::IUnknown> {
        ::windows::Param::Owned(::std::convert::Into::<::windows::IUnknown>::into(self))
    }
}
impl<'a> ::windows::IntoParam<'a, ::windows::IUnknown> for &'a IDXGISurface {
    fn into_param(self) -> ::windows::Param<'a, ::windows::IUnknown> {
        ::windows::Param::Owned(::std::convert::Into::<::windows::IUnknown>::into(
            ::std::clone::Clone::clone(self),
        ))
    }
}
impl ::std::convert::From<IDXGISurface> for IDXGIDeviceSubObject {
    fn from(value: IDXGISurface) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IDXGISurface> for IDXGIDeviceSubObject {
    fn from(value: &IDXGISurface) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::IntoParam<'a, IDXGIDeviceSubObject> for IDXGISurface {
    fn into_param(self) -> ::windows::Param<'a, IDXGIDeviceSubObject> {
        ::windows::Param::Owned(::std::convert::Into::<IDXGIDeviceSubObject>::into(self))
    }
}
impl<'a> ::windows::IntoParam<'a, IDXGIDeviceSubObject> for &'a IDXGISurface {
    fn into_param(self) -> ::windows::Param<'a, IDXGIDeviceSubObject> {
        ::windows::Param::Owned(::std::convert::Into::<IDXGIDeviceSubObject>::into(
            ::std::clone::Clone::clone(self),
        ))
    }
}
impl ::std::convert::From<IDXGISurface> for IDXGIObject {
    fn from(value: IDXGISurface) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IDXGISurface> for IDXGIObject {
    fn from(value: &IDXGISurface) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::IntoParam<'a, IDXGIObject> for IDXGISurface {
    fn into_param(self) -> ::windows::Param<'a, IDXGIObject> {
        ::windows::Param::Owned(::std::convert::Into::<IDXGIObject>::into(self))
    }
}
impl<'a> ::windows::IntoParam<'a, IDXGIObject> for &'a IDXGISurface {
    fn into_param(self) -> ::windows::Param<'a, IDXGIObject> {
        ::windows::Param::Owned(::std::convert::Into::<IDXGIObject>::into(
            ::std::clone::Clone::clone(self),
        ))
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDXGISurface_abi(
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        iid: &::windows::Guid,
        interface: *mut ::windows::RawPtr,
    ) -> ::windows::ErrorCode,
    pub unsafe extern "system" fn(this: ::windows::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::RawPtr) -> u32,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        name: *const ::windows::Guid,
        data_size: u32,
        p_data: *const ::std::ffi::c_void,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        name: *const ::windows::Guid,
        p_unknown: ::windows::RawPtr,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        name: *const ::windows::Guid,
        p_data_size: *mut u32,
        p_data: *mut ::std::ffi::c_void,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        riid: *const ::windows::Guid,
        pp_parent: *mut *mut ::std::ffi::c_void,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        riid: *const ::windows::Guid,
        pp_device: *mut *mut ::std::ffi::c_void,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        p_desc: *mut DXGI_SURFACE_DESC,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        p_locked_rect: *mut DXGI_MAPPED_RECT,
        map_flags: u32,
    ) -> ::windows::ErrorCode,
    pub unsafe extern "system" fn(this: ::windows::RawPtr) -> ::windows::ErrorCode,
);
#[repr(transparent)]
#[allow(non_camel_case_types)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct IDXGISurface1(::windows::IUnknown);
impl IDXGISurface1 {}
unsafe impl ::windows::Interface for IDXGISurface1 {
    type Vtable = IDXGISurface1_abi;
    const IID: ::windows::Guid = ::windows::Guid::from_values(
        1256599698,
        25383,
        19483,
        [128, 174, 191, 225, 46, 163, 43, 134],
    );
}
#[allow(non_snake_case)]
impl IDXGISurface1 {
    pub unsafe fn SetPrivateData(
        &self,
        name: *const ::windows::Guid,
        data_size: u32,
        p_data: *const ::std::ffi::c_void,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).3)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(name),
            ::std::mem::transmute(data_size),
            ::std::mem::transmute(p_data),
        )
    }
    pub unsafe fn SetPrivateDataInterface<
        'a,
        T1__: ::windows::IntoParam<'a, ::windows::IUnknown>,
    >(
        &self,
        name: *const ::windows::Guid,
        p_unknown: T1__,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).4)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(name),
            p_unknown.into_param().abi(),
        )
    }
    pub unsafe fn GetPrivateData(
        &self,
        name: *const ::windows::Guid,
        p_data_size: *mut u32,
        p_data: *mut ::std::ffi::c_void,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).5)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(name),
            ::std::mem::transmute(p_data_size),
            ::std::mem::transmute(p_data),
        )
    }
    pub unsafe fn GetParent(
        &self,
        riid: *const ::windows::Guid,
        pp_parent: *mut *mut ::std::ffi::c_void,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).6)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(riid),
            ::std::mem::transmute(pp_parent),
        )
    }
    pub unsafe fn GetDevice(
        &self,
        riid: *const ::windows::Guid,
        pp_device: *mut *mut ::std::ffi::c_void,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).7)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(riid),
            ::std::mem::transmute(pp_device),
        )
    }
    pub unsafe fn GetDesc(&self, p_desc: *mut DXGI_SURFACE_DESC) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).8)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(p_desc),
        )
    }
    pub unsafe fn Map(
        &self,
        p_locked_rect: *mut DXGI_MAPPED_RECT,
        map_flags: u32,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).9)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(p_locked_rect),
            ::std::mem::transmute(map_flags),
        )
    }
    pub unsafe fn Unmap(&self) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).10)(::windows::Abi::abi(self))
    }
    pub unsafe fn ReleaseDC(
        &self,
        p_dirty_rect: *mut super::display_devices::RECT,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).12)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(p_dirty_rect),
        )
    }
}
impl ::std::convert::From<IDXGISurface1> for ::windows::IUnknown {
    fn from(value: IDXGISurface1) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IDXGISurface1> for ::windows::IUnknown {
    fn from(value: &IDXGISurface1) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::IntoParam<'a, ::windows::IUnknown> for IDXGISurface1 {
    fn into_param(self) -> ::windows::Param<'a, ::windows::IUnknown> {
        ::windows::Param::Owned(::std::convert::Into::<::windows::IUnknown>::into(self))
    }
}
impl<'a> ::windows::IntoParam<'a, ::windows::IUnknown> for &'a IDXGISurface1 {
    fn into_param(self) -> ::windows::Param<'a, ::windows::IUnknown> {
        ::windows::Param::Owned(::std::convert::Into::<::windows::IUnknown>::into(
            ::std::clone::Clone::clone(self),
        ))
    }
}
impl ::std::convert::From<IDXGISurface1> for IDXGISurface {
    fn from(value: IDXGISurface1) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IDXGISurface1> for IDXGISurface {
    fn from(value: &IDXGISurface1) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::IntoParam<'a, IDXGISurface> for IDXGISurface1 {
    fn into_param(self) -> ::windows::Param<'a, IDXGISurface> {
        ::windows::Param::Owned(::std::convert::Into::<IDXGISurface>::into(self))
    }
}
impl<'a> ::windows::IntoParam<'a, IDXGISurface> for &'a IDXGISurface1 {
    fn into_param(self) -> ::windows::Param<'a, IDXGISurface> {
        ::windows::Param::Owned(::std::convert::Into::<IDXGISurface>::into(
            ::std::clone::Clone::clone(self),
        ))
    }
}
impl ::std::convert::From<IDXGISurface1> for IDXGIDeviceSubObject {
    fn from(value: IDXGISurface1) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IDXGISurface1> for IDXGIDeviceSubObject {
    fn from(value: &IDXGISurface1) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::IntoParam<'a, IDXGIDeviceSubObject> for IDXGISurface1 {
    fn into_param(self) -> ::windows::Param<'a, IDXGIDeviceSubObject> {
        ::windows::Param::Owned(::std::convert::Into::<IDXGIDeviceSubObject>::into(self))
    }
}
impl<'a> ::windows::IntoParam<'a, IDXGIDeviceSubObject> for &'a IDXGISurface1 {
    fn into_param(self) -> ::windows::Param<'a, IDXGIDeviceSubObject> {
        ::windows::Param::Owned(::std::convert::Into::<IDXGIDeviceSubObject>::into(
            ::std::clone::Clone::clone(self),
        ))
    }
}
impl ::std::convert::From<IDXGISurface1> for IDXGIObject {
    fn from(value: IDXGISurface1) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IDXGISurface1> for IDXGIObject {
    fn from(value: &IDXGISurface1) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::IntoParam<'a, IDXGIObject> for IDXGISurface1 {
    fn into_param(self) -> ::windows::Param<'a, IDXGIObject> {
        ::windows::Param::Owned(::std::convert::Into::<IDXGIObject>::into(self))
    }
}
impl<'a> ::windows::IntoParam<'a, IDXGIObject> for &'a IDXGISurface1 {
    fn into_param(self) -> ::windows::Param<'a, IDXGIObject> {
        ::windows::Param::Owned(::std::convert::Into::<IDXGIObject>::into(
            ::std::clone::Clone::clone(self),
        ))
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDXGISurface1_abi(
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        iid: &::windows::Guid,
        interface: *mut ::windows::RawPtr,
    ) -> ::windows::ErrorCode,
    pub unsafe extern "system" fn(this: ::windows::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::RawPtr) -> u32,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        name: *const ::windows::Guid,
        data_size: u32,
        p_data: *const ::std::ffi::c_void,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        name: *const ::windows::Guid,
        p_unknown: ::windows::RawPtr,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        name: *const ::windows::Guid,
        p_data_size: *mut u32,
        p_data: *mut ::std::ffi::c_void,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        riid: *const ::windows::Guid,
        pp_parent: *mut *mut ::std::ffi::c_void,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        riid: *const ::windows::Guid,
        pp_device: *mut *mut ::std::ffi::c_void,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        p_desc: *mut DXGI_SURFACE_DESC,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        p_locked_rect: *mut DXGI_MAPPED_RECT,
        map_flags: u32,
    ) -> ::windows::ErrorCode,
    pub unsafe extern "system" fn(this: ::windows::RawPtr) -> ::windows::ErrorCode,
    pub unsafe extern "system" fn(),
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        p_dirty_rect: *mut super::display_devices::RECT,
    ) -> ::windows::ErrorCode,
);
#[repr(transparent)]
#[allow(non_camel_case_types)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct IDXGISurface2(::windows::IUnknown);
impl IDXGISurface2 {}
unsafe impl ::windows::Interface for IDXGISurface2 {
    type Vtable = IDXGISurface2_abi;
    const IID: ::windows::Guid = ::windows::Guid::from_values(
        2879690461,
        46615,
        19640,
        [168, 102, 188, 68, 215, 235, 31, 162],
    );
}
#[allow(non_snake_case)]
impl IDXGISurface2 {
    pub unsafe fn SetPrivateData(
        &self,
        name: *const ::windows::Guid,
        data_size: u32,
        p_data: *const ::std::ffi::c_void,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).3)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(name),
            ::std::mem::transmute(data_size),
            ::std::mem::transmute(p_data),
        )
    }
    pub unsafe fn SetPrivateDataInterface<
        'a,
        T1__: ::windows::IntoParam<'a, ::windows::IUnknown>,
    >(
        &self,
        name: *const ::windows::Guid,
        p_unknown: T1__,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).4)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(name),
            p_unknown.into_param().abi(),
        )
    }
    pub unsafe fn GetPrivateData(
        &self,
        name: *const ::windows::Guid,
        p_data_size: *mut u32,
        p_data: *mut ::std::ffi::c_void,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).5)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(name),
            ::std::mem::transmute(p_data_size),
            ::std::mem::transmute(p_data),
        )
    }
    pub unsafe fn GetParent(
        &self,
        riid: *const ::windows::Guid,
        pp_parent: *mut *mut ::std::ffi::c_void,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).6)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(riid),
            ::std::mem::transmute(pp_parent),
        )
    }
    pub unsafe fn GetDevice(
        &self,
        riid: *const ::windows::Guid,
        pp_device: *mut *mut ::std::ffi::c_void,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).7)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(riid),
            ::std::mem::transmute(pp_device),
        )
    }
    pub unsafe fn GetDesc(&self, p_desc: *mut DXGI_SURFACE_DESC) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).8)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(p_desc),
        )
    }
    pub unsafe fn Map(
        &self,
        p_locked_rect: *mut DXGI_MAPPED_RECT,
        map_flags: u32,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).9)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(p_locked_rect),
            ::std::mem::transmute(map_flags),
        )
    }
    pub unsafe fn Unmap(&self) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).10)(::windows::Abi::abi(self))
    }
    pub unsafe fn ReleaseDC(
        &self,
        p_dirty_rect: *mut super::display_devices::RECT,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).12)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(p_dirty_rect),
        )
    }
    pub unsafe fn GetResource(
        &self,
        riid: *const ::windows::Guid,
        pp_parent_resource: *mut *mut ::std::ffi::c_void,
        p_subresource_index: *mut u32,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).13)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(riid),
            ::std::mem::transmute(pp_parent_resource),
            ::std::mem::transmute(p_subresource_index),
        )
    }
}
impl ::std::convert::From<IDXGISurface2> for ::windows::IUnknown {
    fn from(value: IDXGISurface2) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IDXGISurface2> for ::windows::IUnknown {
    fn from(value: &IDXGISurface2) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::IntoParam<'a, ::windows::IUnknown> for IDXGISurface2 {
    fn into_param(self) -> ::windows::Param<'a, ::windows::IUnknown> {
        ::windows::Param::Owned(::std::convert::Into::<::windows::IUnknown>::into(self))
    }
}
impl<'a> ::windows::IntoParam<'a, ::windows::IUnknown> for &'a IDXGISurface2 {
    fn into_param(self) -> ::windows::Param<'a, ::windows::IUnknown> {
        ::windows::Param::Owned(::std::convert::Into::<::windows::IUnknown>::into(
            ::std::clone::Clone::clone(self),
        ))
    }
}
impl ::std::convert::From<IDXGISurface2> for IDXGISurface1 {
    fn from(value: IDXGISurface2) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IDXGISurface2> for IDXGISurface1 {
    fn from(value: &IDXGISurface2) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::IntoParam<'a, IDXGISurface1> for IDXGISurface2 {
    fn into_param(self) -> ::windows::Param<'a, IDXGISurface1> {
        ::windows::Param::Owned(::std::convert::Into::<IDXGISurface1>::into(self))
    }
}
impl<'a> ::windows::IntoParam<'a, IDXGISurface1> for &'a IDXGISurface2 {
    fn into_param(self) -> ::windows::Param<'a, IDXGISurface1> {
        ::windows::Param::Owned(::std::convert::Into::<IDXGISurface1>::into(
            ::std::clone::Clone::clone(self),
        ))
    }
}
impl ::std::convert::From<IDXGISurface2> for IDXGISurface {
    fn from(value: IDXGISurface2) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IDXGISurface2> for IDXGISurface {
    fn from(value: &IDXGISurface2) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::IntoParam<'a, IDXGISurface> for IDXGISurface2 {
    fn into_param(self) -> ::windows::Param<'a, IDXGISurface> {
        ::windows::Param::Owned(::std::convert::Into::<IDXGISurface>::into(self))
    }
}
impl<'a> ::windows::IntoParam<'a, IDXGISurface> for &'a IDXGISurface2 {
    fn into_param(self) -> ::windows::Param<'a, IDXGISurface> {
        ::windows::Param::Owned(::std::convert::Into::<IDXGISurface>::into(
            ::std::clone::Clone::clone(self),
        ))
    }
}
impl ::std::convert::From<IDXGISurface2> for IDXGIDeviceSubObject {
    fn from(value: IDXGISurface2) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IDXGISurface2> for IDXGIDeviceSubObject {
    fn from(value: &IDXGISurface2) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::IntoParam<'a, IDXGIDeviceSubObject> for IDXGISurface2 {
    fn into_param(self) -> ::windows::Param<'a, IDXGIDeviceSubObject> {
        ::windows::Param::Owned(::std::convert::Into::<IDXGIDeviceSubObject>::into(self))
    }
}
impl<'a> ::windows::IntoParam<'a, IDXGIDeviceSubObject> for &'a IDXGISurface2 {
    fn into_param(self) -> ::windows::Param<'a, IDXGIDeviceSubObject> {
        ::windows::Param::Owned(::std::convert::Into::<IDXGIDeviceSubObject>::into(
            ::std::clone::Clone::clone(self),
        ))
    }
}
impl ::std::convert::From<IDXGISurface2> for IDXGIObject {
    fn from(value: IDXGISurface2) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IDXGISurface2> for IDXGIObject {
    fn from(value: &IDXGISurface2) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::IntoParam<'a, IDXGIObject> for IDXGISurface2 {
    fn into_param(self) -> ::windows::Param<'a, IDXGIObject> {
        ::windows::Param::Owned(::std::convert::Into::<IDXGIObject>::into(self))
    }
}
impl<'a> ::windows::IntoParam<'a, IDXGIObject> for &'a IDXGISurface2 {
    fn into_param(self) -> ::windows::Param<'a, IDXGIObject> {
        ::windows::Param::Owned(::std::convert::Into::<IDXGIObject>::into(
            ::std::clone::Clone::clone(self),
        ))
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDXGISurface2_abi(
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        iid: &::windows::Guid,
        interface: *mut ::windows::RawPtr,
    ) -> ::windows::ErrorCode,
    pub unsafe extern "system" fn(this: ::windows::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::RawPtr) -> u32,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        name: *const ::windows::Guid,
        data_size: u32,
        p_data: *const ::std::ffi::c_void,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        name: *const ::windows::Guid,
        p_unknown: ::windows::RawPtr,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        name: *const ::windows::Guid,
        p_data_size: *mut u32,
        p_data: *mut ::std::ffi::c_void,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        riid: *const ::windows::Guid,
        pp_parent: *mut *mut ::std::ffi::c_void,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        riid: *const ::windows::Guid,
        pp_device: *mut *mut ::std::ffi::c_void,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        p_desc: *mut DXGI_SURFACE_DESC,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        p_locked_rect: *mut DXGI_MAPPED_RECT,
        map_flags: u32,
    ) -> ::windows::ErrorCode,
    pub unsafe extern "system" fn(this: ::windows::RawPtr) -> ::windows::ErrorCode,
    pub unsafe extern "system" fn(),
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        p_dirty_rect: *mut super::display_devices::RECT,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        riid: *const ::windows::Guid,
        pp_parent_resource: *mut *mut ::std::ffi::c_void,
        p_subresource_index: *mut u32,
    ) -> ::windows::ErrorCode,
);
#[repr(transparent)]
#[allow(non_camel_case_types)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct IDXGISwapChain(::windows::IUnknown);
impl IDXGISwapChain {}
unsafe impl ::windows::Interface for IDXGISwapChain {
    type Vtable = IDXGISwapChain_abi;
    const IID: ::windows::Guid = ::windows::Guid::from_values(
        822949536,
        53991,
        19466,
        [170, 4, 106, 157, 35, 184, 136, 106],
    );
}
#[allow(non_snake_case)]
impl IDXGISwapChain {
    pub unsafe fn SetPrivateData(
        &self,
        name: *const ::windows::Guid,
        data_size: u32,
        p_data: *const ::std::ffi::c_void,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).3)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(name),
            ::std::mem::transmute(data_size),
            ::std::mem::transmute(p_data),
        )
    }
    pub unsafe fn SetPrivateDataInterface<
        'a,
        T1__: ::windows::IntoParam<'a, ::windows::IUnknown>,
    >(
        &self,
        name: *const ::windows::Guid,
        p_unknown: T1__,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).4)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(name),
            p_unknown.into_param().abi(),
        )
    }
    pub unsafe fn GetPrivateData(
        &self,
        name: *const ::windows::Guid,
        p_data_size: *mut u32,
        p_data: *mut ::std::ffi::c_void,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).5)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(name),
            ::std::mem::transmute(p_data_size),
            ::std::mem::transmute(p_data),
        )
    }
    pub unsafe fn GetParent(
        &self,
        riid: *const ::windows::Guid,
        pp_parent: *mut *mut ::std::ffi::c_void,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).6)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(riid),
            ::std::mem::transmute(pp_parent),
        )
    }
    pub unsafe fn GetDevice(
        &self,
        riid: *const ::windows::Guid,
        pp_device: *mut *mut ::std::ffi::c_void,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).7)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(riid),
            ::std::mem::transmute(pp_device),
        )
    }
    pub unsafe fn Present(&self, sync_interval: u32, flags: u32) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).8)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(sync_interval),
            ::std::mem::transmute(flags),
        )
    }
    pub unsafe fn GetBuffer(
        &self,
        buffer: u32,
        riid: *const ::windows::Guid,
        pp_surface: *mut *mut ::std::ffi::c_void,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).9)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(buffer),
            ::std::mem::transmute(riid),
            ::std::mem::transmute(pp_surface),
        )
    }
    pub unsafe fn SetFullscreenState<
        'a,
        T0__: ::windows::IntoParam<'a, super::system_services::BOOL>,
        T1__: ::windows::IntoParam<'a, IDXGIOutput>,
    >(
        &self,
        fullscreen: T0__,
        p_target: T1__,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).10)(
            ::windows::Abi::abi(self),
            fullscreen.into_param().abi(),
            p_target.into_param().abi(),
        )
    }
    pub unsafe fn GetFullscreenState(
        &self,
        p_fullscreen: *mut super::system_services::BOOL,
        pp_target: *mut ::std::option::Option<IDXGIOutput>,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).11)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(p_fullscreen),
            ::std::mem::transmute(pp_target),
        )
    }
    pub unsafe fn GetDesc(&self, p_desc: *mut DXGI_SWAP_CHAIN_DESC) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).12)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(p_desc),
        )
    }
    pub unsafe fn ResizeBuffers(
        &self,
        buffer_count: u32,
        width: u32,
        height: u32,
        new_format: DXGI_FORMAT,
        swap_chain_flags: u32,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).13)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(buffer_count),
            ::std::mem::transmute(width),
            ::std::mem::transmute(height),
            ::std::mem::transmute(new_format),
            ::std::mem::transmute(swap_chain_flags),
        )
    }
    pub unsafe fn ResizeTarget(
        &self,
        p_new_target_parameters: *const DXGI_MODE_DESC,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).14)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(p_new_target_parameters),
        )
    }
    pub unsafe fn GetContainingOutput(
        &self,
        pp_output: *mut ::std::option::Option<IDXGIOutput>,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).15)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(pp_output),
        )
    }
    pub unsafe fn GetFrameStatistics(
        &self,
        p_stats: *mut DXGI_FRAME_STATISTICS,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).16)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(p_stats),
        )
    }
    pub unsafe fn GetLastPresentCount(
        &self,
        p_last_present_count: *mut u32,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).17)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(p_last_present_count),
        )
    }
}
impl ::std::convert::From<IDXGISwapChain> for ::windows::IUnknown {
    fn from(value: IDXGISwapChain) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IDXGISwapChain> for ::windows::IUnknown {
    fn from(value: &IDXGISwapChain) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::IntoParam<'a, ::windows::IUnknown> for IDXGISwapChain {
    fn into_param(self) -> ::windows::Param<'a, ::windows::IUnknown> {
        ::windows::Param::Owned(::std::convert::Into::<::windows::IUnknown>::into(self))
    }
}
impl<'a> ::windows::IntoParam<'a, ::windows::IUnknown> for &'a IDXGISwapChain {
    fn into_param(self) -> ::windows::Param<'a, ::windows::IUnknown> {
        ::windows::Param::Owned(::std::convert::Into::<::windows::IUnknown>::into(
            ::std::clone::Clone::clone(self),
        ))
    }
}
impl ::std::convert::From<IDXGISwapChain> for IDXGIDeviceSubObject {
    fn from(value: IDXGISwapChain) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IDXGISwapChain> for IDXGIDeviceSubObject {
    fn from(value: &IDXGISwapChain) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::IntoParam<'a, IDXGIDeviceSubObject> for IDXGISwapChain {
    fn into_param(self) -> ::windows::Param<'a, IDXGIDeviceSubObject> {
        ::windows::Param::Owned(::std::convert::Into::<IDXGIDeviceSubObject>::into(self))
    }
}
impl<'a> ::windows::IntoParam<'a, IDXGIDeviceSubObject> for &'a IDXGISwapChain {
    fn into_param(self) -> ::windows::Param<'a, IDXGIDeviceSubObject> {
        ::windows::Param::Owned(::std::convert::Into::<IDXGIDeviceSubObject>::into(
            ::std::clone::Clone::clone(self),
        ))
    }
}
impl ::std::convert::From<IDXGISwapChain> for IDXGIObject {
    fn from(value: IDXGISwapChain) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IDXGISwapChain> for IDXGIObject {
    fn from(value: &IDXGISwapChain) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::IntoParam<'a, IDXGIObject> for IDXGISwapChain {
    fn into_param(self) -> ::windows::Param<'a, IDXGIObject> {
        ::windows::Param::Owned(::std::convert::Into::<IDXGIObject>::into(self))
    }
}
impl<'a> ::windows::IntoParam<'a, IDXGIObject> for &'a IDXGISwapChain {
    fn into_param(self) -> ::windows::Param<'a, IDXGIObject> {
        ::windows::Param::Owned(::std::convert::Into::<IDXGIObject>::into(
            ::std::clone::Clone::clone(self),
        ))
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDXGISwapChain_abi(
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        iid: &::windows::Guid,
        interface: *mut ::windows::RawPtr,
    ) -> ::windows::ErrorCode,
    pub unsafe extern "system" fn(this: ::windows::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::RawPtr) -> u32,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        name: *const ::windows::Guid,
        data_size: u32,
        p_data: *const ::std::ffi::c_void,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        name: *const ::windows::Guid,
        p_unknown: ::windows::RawPtr,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        name: *const ::windows::Guid,
        p_data_size: *mut u32,
        p_data: *mut ::std::ffi::c_void,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        riid: *const ::windows::Guid,
        pp_parent: *mut *mut ::std::ffi::c_void,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        riid: *const ::windows::Guid,
        pp_device: *mut *mut ::std::ffi::c_void,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        sync_interval: u32,
        flags: u32,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        buffer: u32,
        riid: *const ::windows::Guid,
        pp_surface: *mut *mut ::std::ffi::c_void,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        fullscreen: super::system_services::BOOL,
        p_target: ::windows::RawPtr,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        p_fullscreen: *mut super::system_services::BOOL,
        pp_target: *mut ::windows::RawPtr,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        p_desc: *mut DXGI_SWAP_CHAIN_DESC,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        buffer_count: u32,
        width: u32,
        height: u32,
        new_format: DXGI_FORMAT,
        swap_chain_flags: u32,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        p_new_target_parameters: *const DXGI_MODE_DESC,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        pp_output: *mut ::windows::RawPtr,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        p_stats: *mut DXGI_FRAME_STATISTICS,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        p_last_present_count: *mut u32,
    ) -> ::windows::ErrorCode,
);
#[repr(transparent)]
#[allow(non_camel_case_types)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct IDXGISwapChain1(::windows::IUnknown);
impl IDXGISwapChain1 {}
unsafe impl ::windows::Interface for IDXGISwapChain1 {
    type Vtable = IDXGISwapChain1_abi;
    const IID: ::windows::Guid = ::windows::Guid::from_values(
        2030716407,
        3394,
        18550,
        [152, 58, 10, 85, 207, 230, 244, 170],
    );
}
#[allow(non_snake_case)]
impl IDXGISwapChain1 {
    pub unsafe fn SetPrivateData(
        &self,
        name: *const ::windows::Guid,
        data_size: u32,
        p_data: *const ::std::ffi::c_void,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).3)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(name),
            ::std::mem::transmute(data_size),
            ::std::mem::transmute(p_data),
        )
    }
    pub unsafe fn SetPrivateDataInterface<
        'a,
        T1__: ::windows::IntoParam<'a, ::windows::IUnknown>,
    >(
        &self,
        name: *const ::windows::Guid,
        p_unknown: T1__,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).4)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(name),
            p_unknown.into_param().abi(),
        )
    }
    pub unsafe fn GetPrivateData(
        &self,
        name: *const ::windows::Guid,
        p_data_size: *mut u32,
        p_data: *mut ::std::ffi::c_void,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).5)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(name),
            ::std::mem::transmute(p_data_size),
            ::std::mem::transmute(p_data),
        )
    }
    pub unsafe fn GetParent(
        &self,
        riid: *const ::windows::Guid,
        pp_parent: *mut *mut ::std::ffi::c_void,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).6)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(riid),
            ::std::mem::transmute(pp_parent),
        )
    }
    pub unsafe fn GetDevice(
        &self,
        riid: *const ::windows::Guid,
        pp_device: *mut *mut ::std::ffi::c_void,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).7)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(riid),
            ::std::mem::transmute(pp_device),
        )
    }
    pub unsafe fn Present(&self, sync_interval: u32, flags: u32) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).8)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(sync_interval),
            ::std::mem::transmute(flags),
        )
    }
    pub unsafe fn GetBuffer(
        &self,
        buffer: u32,
        riid: *const ::windows::Guid,
        pp_surface: *mut *mut ::std::ffi::c_void,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).9)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(buffer),
            ::std::mem::transmute(riid),
            ::std::mem::transmute(pp_surface),
        )
    }
    pub unsafe fn SetFullscreenState<
        'a,
        T0__: ::windows::IntoParam<'a, super::system_services::BOOL>,
        T1__: ::windows::IntoParam<'a, IDXGIOutput>,
    >(
        &self,
        fullscreen: T0__,
        p_target: T1__,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).10)(
            ::windows::Abi::abi(self),
            fullscreen.into_param().abi(),
            p_target.into_param().abi(),
        )
    }
    pub unsafe fn GetFullscreenState(
        &self,
        p_fullscreen: *mut super::system_services::BOOL,
        pp_target: *mut ::std::option::Option<IDXGIOutput>,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).11)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(p_fullscreen),
            ::std::mem::transmute(pp_target),
        )
    }
    pub unsafe fn GetDesc(&self, p_desc: *mut DXGI_SWAP_CHAIN_DESC) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).12)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(p_desc),
        )
    }
    pub unsafe fn ResizeBuffers(
        &self,
        buffer_count: u32,
        width: u32,
        height: u32,
        new_format: DXGI_FORMAT,
        swap_chain_flags: u32,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).13)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(buffer_count),
            ::std::mem::transmute(width),
            ::std::mem::transmute(height),
            ::std::mem::transmute(new_format),
            ::std::mem::transmute(swap_chain_flags),
        )
    }
    pub unsafe fn ResizeTarget(
        &self,
        p_new_target_parameters: *const DXGI_MODE_DESC,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).14)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(p_new_target_parameters),
        )
    }
    pub unsafe fn GetContainingOutput(
        &self,
        pp_output: *mut ::std::option::Option<IDXGIOutput>,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).15)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(pp_output),
        )
    }
    pub unsafe fn GetFrameStatistics(
        &self,
        p_stats: *mut DXGI_FRAME_STATISTICS,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).16)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(p_stats),
        )
    }
    pub unsafe fn GetLastPresentCount(
        &self,
        p_last_present_count: *mut u32,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).17)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(p_last_present_count),
        )
    }
    pub unsafe fn GetDesc1(&self, p_desc: *mut DXGI_SWAP_CHAIN_DESC1) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).18)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(p_desc),
        )
    }
    pub unsafe fn GetFullscreenDesc(
        &self,
        p_desc: *mut DXGI_SWAP_CHAIN_FULLSCREEN_DESC,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).19)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(p_desc),
        )
    }
    pub unsafe fn GetHwnd(
        &self,
        p_hwnd: *mut super::windows_and_messaging::HWND,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).20)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(p_hwnd),
        )
    }
    pub unsafe fn GetCoreWindow(
        &self,
        refiid: *const ::windows::Guid,
        pp_unk: *mut *mut ::std::ffi::c_void,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).21)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(refiid),
            ::std::mem::transmute(pp_unk),
        )
    }
    pub unsafe fn Present1(
        &self,
        sync_interval: u32,
        present_flags: u32,
        p_present_parameters: *const DXGI_PRESENT_PARAMETERS,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).22)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(sync_interval),
            ::std::mem::transmute(present_flags),
            ::std::mem::transmute(p_present_parameters),
        )
    }
    pub unsafe fn IsTemporaryMonoSupported(&self) -> super::system_services::BOOL {
        (::windows::Interface::vtable(self).23)(::windows::Abi::abi(self))
    }
    pub unsafe fn GetRestrictToOutput(
        &self,
        pp_restrict_to_output: *mut ::std::option::Option<IDXGIOutput>,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).24)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(pp_restrict_to_output),
        )
    }
    pub unsafe fn SetBackgroundColor(&self, p_color: *const DXGI_RGBA) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).25)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(p_color),
        )
    }
    pub unsafe fn GetBackgroundColor(&self, p_color: *mut DXGI_RGBA) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).26)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(p_color),
        )
    }
    pub unsafe fn SetRotation(&self, rotation: DXGI_MODE_ROTATION) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).27)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(rotation),
        )
    }
    pub unsafe fn GetRotation(&self, p_rotation: *mut DXGI_MODE_ROTATION) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).28)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(p_rotation),
        )
    }
}
impl ::std::convert::From<IDXGISwapChain1> for ::windows::IUnknown {
    fn from(value: IDXGISwapChain1) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IDXGISwapChain1> for ::windows::IUnknown {
    fn from(value: &IDXGISwapChain1) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::IntoParam<'a, ::windows::IUnknown> for IDXGISwapChain1 {
    fn into_param(self) -> ::windows::Param<'a, ::windows::IUnknown> {
        ::windows::Param::Owned(::std::convert::Into::<::windows::IUnknown>::into(self))
    }
}
impl<'a> ::windows::IntoParam<'a, ::windows::IUnknown> for &'a IDXGISwapChain1 {
    fn into_param(self) -> ::windows::Param<'a, ::windows::IUnknown> {
        ::windows::Param::Owned(::std::convert::Into::<::windows::IUnknown>::into(
            ::std::clone::Clone::clone(self),
        ))
    }
}
impl ::std::convert::From<IDXGISwapChain1> for IDXGISwapChain {
    fn from(value: IDXGISwapChain1) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IDXGISwapChain1> for IDXGISwapChain {
    fn from(value: &IDXGISwapChain1) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::IntoParam<'a, IDXGISwapChain> for IDXGISwapChain1 {
    fn into_param(self) -> ::windows::Param<'a, IDXGISwapChain> {
        ::windows::Param::Owned(::std::convert::Into::<IDXGISwapChain>::into(self))
    }
}
impl<'a> ::windows::IntoParam<'a, IDXGISwapChain> for &'a IDXGISwapChain1 {
    fn into_param(self) -> ::windows::Param<'a, IDXGISwapChain> {
        ::windows::Param::Owned(::std::convert::Into::<IDXGISwapChain>::into(
            ::std::clone::Clone::clone(self),
        ))
    }
}
impl ::std::convert::From<IDXGISwapChain1> for IDXGIDeviceSubObject {
    fn from(value: IDXGISwapChain1) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IDXGISwapChain1> for IDXGIDeviceSubObject {
    fn from(value: &IDXGISwapChain1) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::IntoParam<'a, IDXGIDeviceSubObject> for IDXGISwapChain1 {
    fn into_param(self) -> ::windows::Param<'a, IDXGIDeviceSubObject> {
        ::windows::Param::Owned(::std::convert::Into::<IDXGIDeviceSubObject>::into(self))
    }
}
impl<'a> ::windows::IntoParam<'a, IDXGIDeviceSubObject> for &'a IDXGISwapChain1 {
    fn into_param(self) -> ::windows::Param<'a, IDXGIDeviceSubObject> {
        ::windows::Param::Owned(::std::convert::Into::<IDXGIDeviceSubObject>::into(
            ::std::clone::Clone::clone(self),
        ))
    }
}
impl ::std::convert::From<IDXGISwapChain1> for IDXGIObject {
    fn from(value: IDXGISwapChain1) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IDXGISwapChain1> for IDXGIObject {
    fn from(value: &IDXGISwapChain1) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::IntoParam<'a, IDXGIObject> for IDXGISwapChain1 {
    fn into_param(self) -> ::windows::Param<'a, IDXGIObject> {
        ::windows::Param::Owned(::std::convert::Into::<IDXGIObject>::into(self))
    }
}
impl<'a> ::windows::IntoParam<'a, IDXGIObject> for &'a IDXGISwapChain1 {
    fn into_param(self) -> ::windows::Param<'a, IDXGIObject> {
        ::windows::Param::Owned(::std::convert::Into::<IDXGIObject>::into(
            ::std::clone::Clone::clone(self),
        ))
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDXGISwapChain1_abi(
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        iid: &::windows::Guid,
        interface: *mut ::windows::RawPtr,
    ) -> ::windows::ErrorCode,
    pub unsafe extern "system" fn(this: ::windows::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::RawPtr) -> u32,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        name: *const ::windows::Guid,
        data_size: u32,
        p_data: *const ::std::ffi::c_void,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        name: *const ::windows::Guid,
        p_unknown: ::windows::RawPtr,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        name: *const ::windows::Guid,
        p_data_size: *mut u32,
        p_data: *mut ::std::ffi::c_void,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        riid: *const ::windows::Guid,
        pp_parent: *mut *mut ::std::ffi::c_void,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        riid: *const ::windows::Guid,
        pp_device: *mut *mut ::std::ffi::c_void,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        sync_interval: u32,
        flags: u32,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        buffer: u32,
        riid: *const ::windows::Guid,
        pp_surface: *mut *mut ::std::ffi::c_void,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        fullscreen: super::system_services::BOOL,
        p_target: ::windows::RawPtr,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        p_fullscreen: *mut super::system_services::BOOL,
        pp_target: *mut ::windows::RawPtr,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        p_desc: *mut DXGI_SWAP_CHAIN_DESC,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        buffer_count: u32,
        width: u32,
        height: u32,
        new_format: DXGI_FORMAT,
        swap_chain_flags: u32,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        p_new_target_parameters: *const DXGI_MODE_DESC,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        pp_output: *mut ::windows::RawPtr,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        p_stats: *mut DXGI_FRAME_STATISTICS,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        p_last_present_count: *mut u32,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        p_desc: *mut DXGI_SWAP_CHAIN_DESC1,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        p_desc: *mut DXGI_SWAP_CHAIN_FULLSCREEN_DESC,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        p_hwnd: *mut super::windows_and_messaging::HWND,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        refiid: *const ::windows::Guid,
        pp_unk: *mut *mut ::std::ffi::c_void,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        sync_interval: u32,
        present_flags: u32,
        p_present_parameters: *const DXGI_PRESENT_PARAMETERS,
    ) -> ::windows::ErrorCode,
    pub unsafe extern "system" fn(this: ::windows::RawPtr) -> super::system_services::BOOL,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        pp_restrict_to_output: *mut ::windows::RawPtr,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        p_color: *const DXGI_RGBA,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        p_color: *mut DXGI_RGBA,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        rotation: DXGI_MODE_ROTATION,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        p_rotation: *mut DXGI_MODE_ROTATION,
    ) -> ::windows::ErrorCode,
);
#[repr(transparent)]
#[allow(non_camel_case_types)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct IDXGISwapChain2(::windows::IUnknown);
impl IDXGISwapChain2 {}
unsafe impl ::windows::Interface for IDXGISwapChain2 {
    type Vtable = IDXGISwapChain2_abi;
    const IID: ::windows::Guid = ::windows::Guid::from_values(
        2831035076,
        6559,
        18758,
        [179, 49, 121, 89, 159, 185, 141, 231],
    );
}
#[allow(non_snake_case)]
impl IDXGISwapChain2 {
    pub unsafe fn SetPrivateData(
        &self,
        name: *const ::windows::Guid,
        data_size: u32,
        p_data: *const ::std::ffi::c_void,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).3)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(name),
            ::std::mem::transmute(data_size),
            ::std::mem::transmute(p_data),
        )
    }
    pub unsafe fn SetPrivateDataInterface<
        'a,
        T1__: ::windows::IntoParam<'a, ::windows::IUnknown>,
    >(
        &self,
        name: *const ::windows::Guid,
        p_unknown: T1__,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).4)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(name),
            p_unknown.into_param().abi(),
        )
    }
    pub unsafe fn GetPrivateData(
        &self,
        name: *const ::windows::Guid,
        p_data_size: *mut u32,
        p_data: *mut ::std::ffi::c_void,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).5)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(name),
            ::std::mem::transmute(p_data_size),
            ::std::mem::transmute(p_data),
        )
    }
    pub unsafe fn GetParent(
        &self,
        riid: *const ::windows::Guid,
        pp_parent: *mut *mut ::std::ffi::c_void,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).6)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(riid),
            ::std::mem::transmute(pp_parent),
        )
    }
    pub unsafe fn GetDevice(
        &self,
        riid: *const ::windows::Guid,
        pp_device: *mut *mut ::std::ffi::c_void,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).7)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(riid),
            ::std::mem::transmute(pp_device),
        )
    }
    pub unsafe fn Present(&self, sync_interval: u32, flags: u32) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).8)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(sync_interval),
            ::std::mem::transmute(flags),
        )
    }
    pub unsafe fn GetBuffer(
        &self,
        buffer: u32,
        riid: *const ::windows::Guid,
        pp_surface: *mut *mut ::std::ffi::c_void,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).9)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(buffer),
            ::std::mem::transmute(riid),
            ::std::mem::transmute(pp_surface),
        )
    }
    pub unsafe fn SetFullscreenState<
        'a,
        T0__: ::windows::IntoParam<'a, super::system_services::BOOL>,
        T1__: ::windows::IntoParam<'a, IDXGIOutput>,
    >(
        &self,
        fullscreen: T0__,
        p_target: T1__,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).10)(
            ::windows::Abi::abi(self),
            fullscreen.into_param().abi(),
            p_target.into_param().abi(),
        )
    }
    pub unsafe fn GetFullscreenState(
        &self,
        p_fullscreen: *mut super::system_services::BOOL,
        pp_target: *mut ::std::option::Option<IDXGIOutput>,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).11)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(p_fullscreen),
            ::std::mem::transmute(pp_target),
        )
    }
    pub unsafe fn GetDesc(&self, p_desc: *mut DXGI_SWAP_CHAIN_DESC) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).12)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(p_desc),
        )
    }
    pub unsafe fn ResizeBuffers(
        &self,
        buffer_count: u32,
        width: u32,
        height: u32,
        new_format: DXGI_FORMAT,
        swap_chain_flags: u32,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).13)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(buffer_count),
            ::std::mem::transmute(width),
            ::std::mem::transmute(height),
            ::std::mem::transmute(new_format),
            ::std::mem::transmute(swap_chain_flags),
        )
    }
    pub unsafe fn ResizeTarget(
        &self,
        p_new_target_parameters: *const DXGI_MODE_DESC,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).14)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(p_new_target_parameters),
        )
    }
    pub unsafe fn GetContainingOutput(
        &self,
        pp_output: *mut ::std::option::Option<IDXGIOutput>,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).15)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(pp_output),
        )
    }
    pub unsafe fn GetFrameStatistics(
        &self,
        p_stats: *mut DXGI_FRAME_STATISTICS,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).16)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(p_stats),
        )
    }
    pub unsafe fn GetLastPresentCount(
        &self,
        p_last_present_count: *mut u32,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).17)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(p_last_present_count),
        )
    }
    pub unsafe fn GetDesc1(&self, p_desc: *mut DXGI_SWAP_CHAIN_DESC1) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).18)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(p_desc),
        )
    }
    pub unsafe fn GetFullscreenDesc(
        &self,
        p_desc: *mut DXGI_SWAP_CHAIN_FULLSCREEN_DESC,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).19)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(p_desc),
        )
    }
    pub unsafe fn GetHwnd(
        &self,
        p_hwnd: *mut super::windows_and_messaging::HWND,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).20)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(p_hwnd),
        )
    }
    pub unsafe fn GetCoreWindow(
        &self,
        refiid: *const ::windows::Guid,
        pp_unk: *mut *mut ::std::ffi::c_void,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).21)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(refiid),
            ::std::mem::transmute(pp_unk),
        )
    }
    pub unsafe fn Present1(
        &self,
        sync_interval: u32,
        present_flags: u32,
        p_present_parameters: *const DXGI_PRESENT_PARAMETERS,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).22)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(sync_interval),
            ::std::mem::transmute(present_flags),
            ::std::mem::transmute(p_present_parameters),
        )
    }
    pub unsafe fn IsTemporaryMonoSupported(&self) -> super::system_services::BOOL {
        (::windows::Interface::vtable(self).23)(::windows::Abi::abi(self))
    }
    pub unsafe fn GetRestrictToOutput(
        &self,
        pp_restrict_to_output: *mut ::std::option::Option<IDXGIOutput>,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).24)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(pp_restrict_to_output),
        )
    }
    pub unsafe fn SetBackgroundColor(&self, p_color: *const DXGI_RGBA) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).25)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(p_color),
        )
    }
    pub unsafe fn GetBackgroundColor(&self, p_color: *mut DXGI_RGBA) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).26)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(p_color),
        )
    }
    pub unsafe fn SetRotation(&self, rotation: DXGI_MODE_ROTATION) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).27)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(rotation),
        )
    }
    pub unsafe fn GetRotation(&self, p_rotation: *mut DXGI_MODE_ROTATION) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).28)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(p_rotation),
        )
    }
    pub unsafe fn SetSourceSize(&self, width: u32, height: u32) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).29)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(width),
            ::std::mem::transmute(height),
        )
    }
    pub unsafe fn GetSourceSize(
        &self,
        p_width: *mut u32,
        p_height: *mut u32,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).30)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(p_width),
            ::std::mem::transmute(p_height),
        )
    }
    pub unsafe fn SetMaximumFrameLatency(&self, max_latency: u32) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).31)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(max_latency),
        )
    }
    pub unsafe fn GetMaximumFrameLatency(&self, p_max_latency: *mut u32) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).32)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(p_max_latency),
        )
    }
    pub unsafe fn GetFrameLatencyWaitableObject(&self) -> super::system_services::HANDLE {
        (::windows::Interface::vtable(self).33)(::windows::Abi::abi(self))
    }
    pub unsafe fn SetMatrixTransform(
        &self,
        p_matrix: *const DXGI_MATRIX_3X2_F,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).34)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(p_matrix),
        )
    }
    pub unsafe fn GetMatrixTransform(
        &self,
        p_matrix: *mut DXGI_MATRIX_3X2_F,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).35)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(p_matrix),
        )
    }
}
impl ::std::convert::From<IDXGISwapChain2> for ::windows::IUnknown {
    fn from(value: IDXGISwapChain2) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IDXGISwapChain2> for ::windows::IUnknown {
    fn from(value: &IDXGISwapChain2) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::IntoParam<'a, ::windows::IUnknown> for IDXGISwapChain2 {
    fn into_param(self) -> ::windows::Param<'a, ::windows::IUnknown> {
        ::windows::Param::Owned(::std::convert::Into::<::windows::IUnknown>::into(self))
    }
}
impl<'a> ::windows::IntoParam<'a, ::windows::IUnknown> for &'a IDXGISwapChain2 {
    fn into_param(self) -> ::windows::Param<'a, ::windows::IUnknown> {
        ::windows::Param::Owned(::std::convert::Into::<::windows::IUnknown>::into(
            ::std::clone::Clone::clone(self),
        ))
    }
}
impl ::std::convert::From<IDXGISwapChain2> for IDXGISwapChain1 {
    fn from(value: IDXGISwapChain2) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IDXGISwapChain2> for IDXGISwapChain1 {
    fn from(value: &IDXGISwapChain2) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::IntoParam<'a, IDXGISwapChain1> for IDXGISwapChain2 {
    fn into_param(self) -> ::windows::Param<'a, IDXGISwapChain1> {
        ::windows::Param::Owned(::std::convert::Into::<IDXGISwapChain1>::into(self))
    }
}
impl<'a> ::windows::IntoParam<'a, IDXGISwapChain1> for &'a IDXGISwapChain2 {
    fn into_param(self) -> ::windows::Param<'a, IDXGISwapChain1> {
        ::windows::Param::Owned(::std::convert::Into::<IDXGISwapChain1>::into(
            ::std::clone::Clone::clone(self),
        ))
    }
}
impl ::std::convert::From<IDXGISwapChain2> for IDXGISwapChain {
    fn from(value: IDXGISwapChain2) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IDXGISwapChain2> for IDXGISwapChain {
    fn from(value: &IDXGISwapChain2) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::IntoParam<'a, IDXGISwapChain> for IDXGISwapChain2 {
    fn into_param(self) -> ::windows::Param<'a, IDXGISwapChain> {
        ::windows::Param::Owned(::std::convert::Into::<IDXGISwapChain>::into(self))
    }
}
impl<'a> ::windows::IntoParam<'a, IDXGISwapChain> for &'a IDXGISwapChain2 {
    fn into_param(self) -> ::windows::Param<'a, IDXGISwapChain> {
        ::windows::Param::Owned(::std::convert::Into::<IDXGISwapChain>::into(
            ::std::clone::Clone::clone(self),
        ))
    }
}
impl ::std::convert::From<IDXGISwapChain2> for IDXGIDeviceSubObject {
    fn from(value: IDXGISwapChain2) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IDXGISwapChain2> for IDXGIDeviceSubObject {
    fn from(value: &IDXGISwapChain2) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::IntoParam<'a, IDXGIDeviceSubObject> for IDXGISwapChain2 {
    fn into_param(self) -> ::windows::Param<'a, IDXGIDeviceSubObject> {
        ::windows::Param::Owned(::std::convert::Into::<IDXGIDeviceSubObject>::into(self))
    }
}
impl<'a> ::windows::IntoParam<'a, IDXGIDeviceSubObject> for &'a IDXGISwapChain2 {
    fn into_param(self) -> ::windows::Param<'a, IDXGIDeviceSubObject> {
        ::windows::Param::Owned(::std::convert::Into::<IDXGIDeviceSubObject>::into(
            ::std::clone::Clone::clone(self),
        ))
    }
}
impl ::std::convert::From<IDXGISwapChain2> for IDXGIObject {
    fn from(value: IDXGISwapChain2) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IDXGISwapChain2> for IDXGIObject {
    fn from(value: &IDXGISwapChain2) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::IntoParam<'a, IDXGIObject> for IDXGISwapChain2 {
    fn into_param(self) -> ::windows::Param<'a, IDXGIObject> {
        ::windows::Param::Owned(::std::convert::Into::<IDXGIObject>::into(self))
    }
}
impl<'a> ::windows::IntoParam<'a, IDXGIObject> for &'a IDXGISwapChain2 {
    fn into_param(self) -> ::windows::Param<'a, IDXGIObject> {
        ::windows::Param::Owned(::std::convert::Into::<IDXGIObject>::into(
            ::std::clone::Clone::clone(self),
        ))
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDXGISwapChain2_abi(
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        iid: &::windows::Guid,
        interface: *mut ::windows::RawPtr,
    ) -> ::windows::ErrorCode,
    pub unsafe extern "system" fn(this: ::windows::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::RawPtr) -> u32,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        name: *const ::windows::Guid,
        data_size: u32,
        p_data: *const ::std::ffi::c_void,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        name: *const ::windows::Guid,
        p_unknown: ::windows::RawPtr,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        name: *const ::windows::Guid,
        p_data_size: *mut u32,
        p_data: *mut ::std::ffi::c_void,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        riid: *const ::windows::Guid,
        pp_parent: *mut *mut ::std::ffi::c_void,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        riid: *const ::windows::Guid,
        pp_device: *mut *mut ::std::ffi::c_void,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        sync_interval: u32,
        flags: u32,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        buffer: u32,
        riid: *const ::windows::Guid,
        pp_surface: *mut *mut ::std::ffi::c_void,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        fullscreen: super::system_services::BOOL,
        p_target: ::windows::RawPtr,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        p_fullscreen: *mut super::system_services::BOOL,
        pp_target: *mut ::windows::RawPtr,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        p_desc: *mut DXGI_SWAP_CHAIN_DESC,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        buffer_count: u32,
        width: u32,
        height: u32,
        new_format: DXGI_FORMAT,
        swap_chain_flags: u32,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        p_new_target_parameters: *const DXGI_MODE_DESC,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        pp_output: *mut ::windows::RawPtr,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        p_stats: *mut DXGI_FRAME_STATISTICS,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        p_last_present_count: *mut u32,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        p_desc: *mut DXGI_SWAP_CHAIN_DESC1,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        p_desc: *mut DXGI_SWAP_CHAIN_FULLSCREEN_DESC,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        p_hwnd: *mut super::windows_and_messaging::HWND,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        refiid: *const ::windows::Guid,
        pp_unk: *mut *mut ::std::ffi::c_void,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        sync_interval: u32,
        present_flags: u32,
        p_present_parameters: *const DXGI_PRESENT_PARAMETERS,
    ) -> ::windows::ErrorCode,
    pub unsafe extern "system" fn(this: ::windows::RawPtr) -> super::system_services::BOOL,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        pp_restrict_to_output: *mut ::windows::RawPtr,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        p_color: *const DXGI_RGBA,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        p_color: *mut DXGI_RGBA,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        rotation: DXGI_MODE_ROTATION,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        p_rotation: *mut DXGI_MODE_ROTATION,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        width: u32,
        height: u32,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        p_width: *mut u32,
        p_height: *mut u32,
    ) -> ::windows::ErrorCode,
    pub unsafe extern "system" fn(this: ::windows::RawPtr, max_latency: u32) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        p_max_latency: *mut u32,
    ) -> ::windows::ErrorCode,
    pub unsafe extern "system" fn(this: ::windows::RawPtr) -> super::system_services::HANDLE,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        p_matrix: *const DXGI_MATRIX_3X2_F,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        p_matrix: *mut DXGI_MATRIX_3X2_F,
    ) -> ::windows::ErrorCode,
);
#[repr(transparent)]
#[allow(non_camel_case_types)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct IDXGISwapChain3(::windows::IUnknown);
impl IDXGISwapChain3 {}
unsafe impl ::windows::Interface for IDXGISwapChain3 {
    type Vtable = IDXGISwapChain3_abi;
    const IID: ::windows::Guid = ::windows::Guid::from_values(
        2497289179,
        61944,
        19120,
        [178, 54, 125, 160, 23, 14, 218, 177],
    );
}
#[allow(non_snake_case)]
impl IDXGISwapChain3 {
    pub unsafe fn SetPrivateData(
        &self,
        name: *const ::windows::Guid,
        data_size: u32,
        p_data: *const ::std::ffi::c_void,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).3)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(name),
            ::std::mem::transmute(data_size),
            ::std::mem::transmute(p_data),
        )
    }
    pub unsafe fn SetPrivateDataInterface<
        'a,
        T1__: ::windows::IntoParam<'a, ::windows::IUnknown>,
    >(
        &self,
        name: *const ::windows::Guid,
        p_unknown: T1__,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).4)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(name),
            p_unknown.into_param().abi(),
        )
    }
    pub unsafe fn GetPrivateData(
        &self,
        name: *const ::windows::Guid,
        p_data_size: *mut u32,
        p_data: *mut ::std::ffi::c_void,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).5)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(name),
            ::std::mem::transmute(p_data_size),
            ::std::mem::transmute(p_data),
        )
    }
    pub unsafe fn GetParent(
        &self,
        riid: *const ::windows::Guid,
        pp_parent: *mut *mut ::std::ffi::c_void,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).6)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(riid),
            ::std::mem::transmute(pp_parent),
        )
    }
    pub unsafe fn GetDevice(
        &self,
        riid: *const ::windows::Guid,
        pp_device: *mut *mut ::std::ffi::c_void,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).7)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(riid),
            ::std::mem::transmute(pp_device),
        )
    }
    pub unsafe fn Present(&self, sync_interval: u32, flags: u32) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).8)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(sync_interval),
            ::std::mem::transmute(flags),
        )
    }
    pub unsafe fn GetBuffer(
        &self,
        buffer: u32,
        riid: *const ::windows::Guid,
        pp_surface: *mut *mut ::std::ffi::c_void,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).9)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(buffer),
            ::std::mem::transmute(riid),
            ::std::mem::transmute(pp_surface),
        )
    }
    pub unsafe fn SetFullscreenState<
        'a,
        T0__: ::windows::IntoParam<'a, super::system_services::BOOL>,
        T1__: ::windows::IntoParam<'a, IDXGIOutput>,
    >(
        &self,
        fullscreen: T0__,
        p_target: T1__,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).10)(
            ::windows::Abi::abi(self),
            fullscreen.into_param().abi(),
            p_target.into_param().abi(),
        )
    }
    pub unsafe fn GetFullscreenState(
        &self,
        p_fullscreen: *mut super::system_services::BOOL,
        pp_target: *mut ::std::option::Option<IDXGIOutput>,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).11)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(p_fullscreen),
            ::std::mem::transmute(pp_target),
        )
    }
    pub unsafe fn GetDesc(&self, p_desc: *mut DXGI_SWAP_CHAIN_DESC) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).12)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(p_desc),
        )
    }
    pub unsafe fn ResizeBuffers(
        &self,
        buffer_count: u32,
        width: u32,
        height: u32,
        new_format: DXGI_FORMAT,
        swap_chain_flags: u32,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).13)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(buffer_count),
            ::std::mem::transmute(width),
            ::std::mem::transmute(height),
            ::std::mem::transmute(new_format),
            ::std::mem::transmute(swap_chain_flags),
        )
    }
    pub unsafe fn ResizeTarget(
        &self,
        p_new_target_parameters: *const DXGI_MODE_DESC,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).14)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(p_new_target_parameters),
        )
    }
    pub unsafe fn GetContainingOutput(
        &self,
        pp_output: *mut ::std::option::Option<IDXGIOutput>,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).15)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(pp_output),
        )
    }
    pub unsafe fn GetFrameStatistics(
        &self,
        p_stats: *mut DXGI_FRAME_STATISTICS,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).16)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(p_stats),
        )
    }
    pub unsafe fn GetLastPresentCount(
        &self,
        p_last_present_count: *mut u32,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).17)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(p_last_present_count),
        )
    }
    pub unsafe fn GetDesc1(&self, p_desc: *mut DXGI_SWAP_CHAIN_DESC1) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).18)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(p_desc),
        )
    }
    pub unsafe fn GetFullscreenDesc(
        &self,
        p_desc: *mut DXGI_SWAP_CHAIN_FULLSCREEN_DESC,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).19)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(p_desc),
        )
    }
    pub unsafe fn GetHwnd(
        &self,
        p_hwnd: *mut super::windows_and_messaging::HWND,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).20)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(p_hwnd),
        )
    }
    pub unsafe fn GetCoreWindow(
        &self,
        refiid: *const ::windows::Guid,
        pp_unk: *mut *mut ::std::ffi::c_void,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).21)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(refiid),
            ::std::mem::transmute(pp_unk),
        )
    }
    pub unsafe fn Present1(
        &self,
        sync_interval: u32,
        present_flags: u32,
        p_present_parameters: *const DXGI_PRESENT_PARAMETERS,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).22)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(sync_interval),
            ::std::mem::transmute(present_flags),
            ::std::mem::transmute(p_present_parameters),
        )
    }
    pub unsafe fn IsTemporaryMonoSupported(&self) -> super::system_services::BOOL {
        (::windows::Interface::vtable(self).23)(::windows::Abi::abi(self))
    }
    pub unsafe fn GetRestrictToOutput(
        &self,
        pp_restrict_to_output: *mut ::std::option::Option<IDXGIOutput>,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).24)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(pp_restrict_to_output),
        )
    }
    pub unsafe fn SetBackgroundColor(&self, p_color: *const DXGI_RGBA) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).25)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(p_color),
        )
    }
    pub unsafe fn GetBackgroundColor(&self, p_color: *mut DXGI_RGBA) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).26)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(p_color),
        )
    }
    pub unsafe fn SetRotation(&self, rotation: DXGI_MODE_ROTATION) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).27)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(rotation),
        )
    }
    pub unsafe fn GetRotation(&self, p_rotation: *mut DXGI_MODE_ROTATION) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).28)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(p_rotation),
        )
    }
    pub unsafe fn SetSourceSize(&self, width: u32, height: u32) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).29)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(width),
            ::std::mem::transmute(height),
        )
    }
    pub unsafe fn GetSourceSize(
        &self,
        p_width: *mut u32,
        p_height: *mut u32,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).30)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(p_width),
            ::std::mem::transmute(p_height),
        )
    }
    pub unsafe fn SetMaximumFrameLatency(&self, max_latency: u32) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).31)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(max_latency),
        )
    }
    pub unsafe fn GetMaximumFrameLatency(&self, p_max_latency: *mut u32) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).32)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(p_max_latency),
        )
    }
    pub unsafe fn GetFrameLatencyWaitableObject(&self) -> super::system_services::HANDLE {
        (::windows::Interface::vtable(self).33)(::windows::Abi::abi(self))
    }
    pub unsafe fn SetMatrixTransform(
        &self,
        p_matrix: *const DXGI_MATRIX_3X2_F,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).34)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(p_matrix),
        )
    }
    pub unsafe fn GetMatrixTransform(
        &self,
        p_matrix: *mut DXGI_MATRIX_3X2_F,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).35)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(p_matrix),
        )
    }
    pub unsafe fn GetCurrentBackBufferIndex(&self) -> u32 {
        (::windows::Interface::vtable(self).36)(::windows::Abi::abi(self))
    }
    pub unsafe fn CheckColorSpaceSupport(
        &self,
        color_space: DXGI_COLOR_SPACE_TYPE,
        p_color_space_support: *mut u32,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).37)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(color_space),
            ::std::mem::transmute(p_color_space_support),
        )
    }
    pub unsafe fn SetColorSpace1(
        &self,
        color_space: DXGI_COLOR_SPACE_TYPE,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).38)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(color_space),
        )
    }
    pub unsafe fn ResizeBuffers1(
        &self,
        buffer_count: u32,
        width: u32,
        height: u32,
        format: DXGI_FORMAT,
        swap_chain_flags: u32,
        p_creation_node_mask: *const u32,
        pp_present_queue: *mut ::std::option::Option<::windows::IUnknown>,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).39)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(buffer_count),
            ::std::mem::transmute(width),
            ::std::mem::transmute(height),
            ::std::mem::transmute(format),
            ::std::mem::transmute(swap_chain_flags),
            ::std::mem::transmute(p_creation_node_mask),
            ::std::mem::transmute(pp_present_queue),
        )
    }
}
impl ::std::convert::From<IDXGISwapChain3> for ::windows::IUnknown {
    fn from(value: IDXGISwapChain3) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IDXGISwapChain3> for ::windows::IUnknown {
    fn from(value: &IDXGISwapChain3) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::IntoParam<'a, ::windows::IUnknown> for IDXGISwapChain3 {
    fn into_param(self) -> ::windows::Param<'a, ::windows::IUnknown> {
        ::windows::Param::Owned(::std::convert::Into::<::windows::IUnknown>::into(self))
    }
}
impl<'a> ::windows::IntoParam<'a, ::windows::IUnknown> for &'a IDXGISwapChain3 {
    fn into_param(self) -> ::windows::Param<'a, ::windows::IUnknown> {
        ::windows::Param::Owned(::std::convert::Into::<::windows::IUnknown>::into(
            ::std::clone::Clone::clone(self),
        ))
    }
}
impl ::std::convert::From<IDXGISwapChain3> for IDXGISwapChain2 {
    fn from(value: IDXGISwapChain3) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IDXGISwapChain3> for IDXGISwapChain2 {
    fn from(value: &IDXGISwapChain3) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::IntoParam<'a, IDXGISwapChain2> for IDXGISwapChain3 {
    fn into_param(self) -> ::windows::Param<'a, IDXGISwapChain2> {
        ::windows::Param::Owned(::std::convert::Into::<IDXGISwapChain2>::into(self))
    }
}
impl<'a> ::windows::IntoParam<'a, IDXGISwapChain2> for &'a IDXGISwapChain3 {
    fn into_param(self) -> ::windows::Param<'a, IDXGISwapChain2> {
        ::windows::Param::Owned(::std::convert::Into::<IDXGISwapChain2>::into(
            ::std::clone::Clone::clone(self),
        ))
    }
}
impl ::std::convert::From<IDXGISwapChain3> for IDXGISwapChain1 {
    fn from(value: IDXGISwapChain3) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IDXGISwapChain3> for IDXGISwapChain1 {
    fn from(value: &IDXGISwapChain3) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::IntoParam<'a, IDXGISwapChain1> for IDXGISwapChain3 {
    fn into_param(self) -> ::windows::Param<'a, IDXGISwapChain1> {
        ::windows::Param::Owned(::std::convert::Into::<IDXGISwapChain1>::into(self))
    }
}
impl<'a> ::windows::IntoParam<'a, IDXGISwapChain1> for &'a IDXGISwapChain3 {
    fn into_param(self) -> ::windows::Param<'a, IDXGISwapChain1> {
        ::windows::Param::Owned(::std::convert::Into::<IDXGISwapChain1>::into(
            ::std::clone::Clone::clone(self),
        ))
    }
}
impl ::std::convert::From<IDXGISwapChain3> for IDXGISwapChain {
    fn from(value: IDXGISwapChain3) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IDXGISwapChain3> for IDXGISwapChain {
    fn from(value: &IDXGISwapChain3) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::IntoParam<'a, IDXGISwapChain> for IDXGISwapChain3 {
    fn into_param(self) -> ::windows::Param<'a, IDXGISwapChain> {
        ::windows::Param::Owned(::std::convert::Into::<IDXGISwapChain>::into(self))
    }
}
impl<'a> ::windows::IntoParam<'a, IDXGISwapChain> for &'a IDXGISwapChain3 {
    fn into_param(self) -> ::windows::Param<'a, IDXGISwapChain> {
        ::windows::Param::Owned(::std::convert::Into::<IDXGISwapChain>::into(
            ::std::clone::Clone::clone(self),
        ))
    }
}
impl ::std::convert::From<IDXGISwapChain3> for IDXGIDeviceSubObject {
    fn from(value: IDXGISwapChain3) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IDXGISwapChain3> for IDXGIDeviceSubObject {
    fn from(value: &IDXGISwapChain3) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::IntoParam<'a, IDXGIDeviceSubObject> for IDXGISwapChain3 {
    fn into_param(self) -> ::windows::Param<'a, IDXGIDeviceSubObject> {
        ::windows::Param::Owned(::std::convert::Into::<IDXGIDeviceSubObject>::into(self))
    }
}
impl<'a> ::windows::IntoParam<'a, IDXGIDeviceSubObject> for &'a IDXGISwapChain3 {
    fn into_param(self) -> ::windows::Param<'a, IDXGIDeviceSubObject> {
        ::windows::Param::Owned(::std::convert::Into::<IDXGIDeviceSubObject>::into(
            ::std::clone::Clone::clone(self),
        ))
    }
}
impl ::std::convert::From<IDXGISwapChain3> for IDXGIObject {
    fn from(value: IDXGISwapChain3) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IDXGISwapChain3> for IDXGIObject {
    fn from(value: &IDXGISwapChain3) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::IntoParam<'a, IDXGIObject> for IDXGISwapChain3 {
    fn into_param(self) -> ::windows::Param<'a, IDXGIObject> {
        ::windows::Param::Owned(::std::convert::Into::<IDXGIObject>::into(self))
    }
}
impl<'a> ::windows::IntoParam<'a, IDXGIObject> for &'a IDXGISwapChain3 {
    fn into_param(self) -> ::windows::Param<'a, IDXGIObject> {
        ::windows::Param::Owned(::std::convert::Into::<IDXGIObject>::into(
            ::std::clone::Clone::clone(self),
        ))
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDXGISwapChain3_abi(
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        iid: &::windows::Guid,
        interface: *mut ::windows::RawPtr,
    ) -> ::windows::ErrorCode,
    pub unsafe extern "system" fn(this: ::windows::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::RawPtr) -> u32,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        name: *const ::windows::Guid,
        data_size: u32,
        p_data: *const ::std::ffi::c_void,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        name: *const ::windows::Guid,
        p_unknown: ::windows::RawPtr,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        name: *const ::windows::Guid,
        p_data_size: *mut u32,
        p_data: *mut ::std::ffi::c_void,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        riid: *const ::windows::Guid,
        pp_parent: *mut *mut ::std::ffi::c_void,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        riid: *const ::windows::Guid,
        pp_device: *mut *mut ::std::ffi::c_void,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        sync_interval: u32,
        flags: u32,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        buffer: u32,
        riid: *const ::windows::Guid,
        pp_surface: *mut *mut ::std::ffi::c_void,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        fullscreen: super::system_services::BOOL,
        p_target: ::windows::RawPtr,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        p_fullscreen: *mut super::system_services::BOOL,
        pp_target: *mut ::windows::RawPtr,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        p_desc: *mut DXGI_SWAP_CHAIN_DESC,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        buffer_count: u32,
        width: u32,
        height: u32,
        new_format: DXGI_FORMAT,
        swap_chain_flags: u32,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        p_new_target_parameters: *const DXGI_MODE_DESC,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        pp_output: *mut ::windows::RawPtr,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        p_stats: *mut DXGI_FRAME_STATISTICS,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        p_last_present_count: *mut u32,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        p_desc: *mut DXGI_SWAP_CHAIN_DESC1,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        p_desc: *mut DXGI_SWAP_CHAIN_FULLSCREEN_DESC,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        p_hwnd: *mut super::windows_and_messaging::HWND,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        refiid: *const ::windows::Guid,
        pp_unk: *mut *mut ::std::ffi::c_void,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        sync_interval: u32,
        present_flags: u32,
        p_present_parameters: *const DXGI_PRESENT_PARAMETERS,
    ) -> ::windows::ErrorCode,
    pub unsafe extern "system" fn(this: ::windows::RawPtr) -> super::system_services::BOOL,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        pp_restrict_to_output: *mut ::windows::RawPtr,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        p_color: *const DXGI_RGBA,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        p_color: *mut DXGI_RGBA,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        rotation: DXGI_MODE_ROTATION,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        p_rotation: *mut DXGI_MODE_ROTATION,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        width: u32,
        height: u32,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        p_width: *mut u32,
        p_height: *mut u32,
    ) -> ::windows::ErrorCode,
    pub unsafe extern "system" fn(this: ::windows::RawPtr, max_latency: u32) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        p_max_latency: *mut u32,
    ) -> ::windows::ErrorCode,
    pub unsafe extern "system" fn(this: ::windows::RawPtr) -> super::system_services::HANDLE,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        p_matrix: *const DXGI_MATRIX_3X2_F,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        p_matrix: *mut DXGI_MATRIX_3X2_F,
    ) -> ::windows::ErrorCode,
    pub unsafe extern "system" fn(this: ::windows::RawPtr) -> u32,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        color_space: DXGI_COLOR_SPACE_TYPE,
        p_color_space_support: *mut u32,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        color_space: DXGI_COLOR_SPACE_TYPE,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        buffer_count: u32,
        width: u32,
        height: u32,
        format: DXGI_FORMAT,
        swap_chain_flags: u32,
        p_creation_node_mask: *const u32,
        pp_present_queue: *mut ::windows::RawPtr,
    ) -> ::windows::ErrorCode,
);
#[repr(transparent)]
#[allow(non_camel_case_types)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct IDXGISwapChain4(::windows::IUnknown);
impl IDXGISwapChain4 {}
unsafe impl ::windows::Interface for IDXGISwapChain4 {
    type Vtable = IDXGISwapChain4_abi;
    const IID: ::windows::Guid = ::windows::Guid::from_values(
        1029201242,
        48458,
        18590,
        [177, 244, 61, 188, 182, 69, 47, 251],
    );
}
#[allow(non_snake_case)]
impl IDXGISwapChain4 {
    pub unsafe fn SetPrivateData(
        &self,
        name: *const ::windows::Guid,
        data_size: u32,
        p_data: *const ::std::ffi::c_void,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).3)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(name),
            ::std::mem::transmute(data_size),
            ::std::mem::transmute(p_data),
        )
    }
    pub unsafe fn SetPrivateDataInterface<
        'a,
        T1__: ::windows::IntoParam<'a, ::windows::IUnknown>,
    >(
        &self,
        name: *const ::windows::Guid,
        p_unknown: T1__,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).4)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(name),
            p_unknown.into_param().abi(),
        )
    }
    pub unsafe fn GetPrivateData(
        &self,
        name: *const ::windows::Guid,
        p_data_size: *mut u32,
        p_data: *mut ::std::ffi::c_void,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).5)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(name),
            ::std::mem::transmute(p_data_size),
            ::std::mem::transmute(p_data),
        )
    }
    pub unsafe fn GetParent(
        &self,
        riid: *const ::windows::Guid,
        pp_parent: *mut *mut ::std::ffi::c_void,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).6)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(riid),
            ::std::mem::transmute(pp_parent),
        )
    }
    pub unsafe fn GetDevice(
        &self,
        riid: *const ::windows::Guid,
        pp_device: *mut *mut ::std::ffi::c_void,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).7)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(riid),
            ::std::mem::transmute(pp_device),
        )
    }
    pub unsafe fn Present(&self, sync_interval: u32, flags: u32) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).8)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(sync_interval),
            ::std::mem::transmute(flags),
        )
    }
    pub unsafe fn GetBuffer(
        &self,
        buffer: u32,
        riid: *const ::windows::Guid,
        pp_surface: *mut *mut ::std::ffi::c_void,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).9)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(buffer),
            ::std::mem::transmute(riid),
            ::std::mem::transmute(pp_surface),
        )
    }
    pub unsafe fn SetFullscreenState<
        'a,
        T0__: ::windows::IntoParam<'a, super::system_services::BOOL>,
        T1__: ::windows::IntoParam<'a, IDXGIOutput>,
    >(
        &self,
        fullscreen: T0__,
        p_target: T1__,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).10)(
            ::windows::Abi::abi(self),
            fullscreen.into_param().abi(),
            p_target.into_param().abi(),
        )
    }
    pub unsafe fn GetFullscreenState(
        &self,
        p_fullscreen: *mut super::system_services::BOOL,
        pp_target: *mut ::std::option::Option<IDXGIOutput>,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).11)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(p_fullscreen),
            ::std::mem::transmute(pp_target),
        )
    }
    pub unsafe fn GetDesc(&self, p_desc: *mut DXGI_SWAP_CHAIN_DESC) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).12)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(p_desc),
        )
    }
    pub unsafe fn ResizeBuffers(
        &self,
        buffer_count: u32,
        width: u32,
        height: u32,
        new_format: DXGI_FORMAT,
        swap_chain_flags: u32,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).13)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(buffer_count),
            ::std::mem::transmute(width),
            ::std::mem::transmute(height),
            ::std::mem::transmute(new_format),
            ::std::mem::transmute(swap_chain_flags),
        )
    }
    pub unsafe fn ResizeTarget(
        &self,
        p_new_target_parameters: *const DXGI_MODE_DESC,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).14)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(p_new_target_parameters),
        )
    }
    pub unsafe fn GetContainingOutput(
        &self,
        pp_output: *mut ::std::option::Option<IDXGIOutput>,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).15)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(pp_output),
        )
    }
    pub unsafe fn GetFrameStatistics(
        &self,
        p_stats: *mut DXGI_FRAME_STATISTICS,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).16)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(p_stats),
        )
    }
    pub unsafe fn GetLastPresentCount(
        &self,
        p_last_present_count: *mut u32,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).17)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(p_last_present_count),
        )
    }
    pub unsafe fn GetDesc1(&self, p_desc: *mut DXGI_SWAP_CHAIN_DESC1) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).18)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(p_desc),
        )
    }
    pub unsafe fn GetFullscreenDesc(
        &self,
        p_desc: *mut DXGI_SWAP_CHAIN_FULLSCREEN_DESC,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).19)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(p_desc),
        )
    }
    pub unsafe fn GetHwnd(
        &self,
        p_hwnd: *mut super::windows_and_messaging::HWND,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).20)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(p_hwnd),
        )
    }
    pub unsafe fn GetCoreWindow(
        &self,
        refiid: *const ::windows::Guid,
        pp_unk: *mut *mut ::std::ffi::c_void,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).21)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(refiid),
            ::std::mem::transmute(pp_unk),
        )
    }
    pub unsafe fn Present1(
        &self,
        sync_interval: u32,
        present_flags: u32,
        p_present_parameters: *const DXGI_PRESENT_PARAMETERS,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).22)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(sync_interval),
            ::std::mem::transmute(present_flags),
            ::std::mem::transmute(p_present_parameters),
        )
    }
    pub unsafe fn IsTemporaryMonoSupported(&self) -> super::system_services::BOOL {
        (::windows::Interface::vtable(self).23)(::windows::Abi::abi(self))
    }
    pub unsafe fn GetRestrictToOutput(
        &self,
        pp_restrict_to_output: *mut ::std::option::Option<IDXGIOutput>,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).24)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(pp_restrict_to_output),
        )
    }
    pub unsafe fn SetBackgroundColor(&self, p_color: *const DXGI_RGBA) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).25)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(p_color),
        )
    }
    pub unsafe fn GetBackgroundColor(&self, p_color: *mut DXGI_RGBA) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).26)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(p_color),
        )
    }
    pub unsafe fn SetRotation(&self, rotation: DXGI_MODE_ROTATION) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).27)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(rotation),
        )
    }
    pub unsafe fn GetRotation(&self, p_rotation: *mut DXGI_MODE_ROTATION) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).28)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(p_rotation),
        )
    }
    pub unsafe fn SetSourceSize(&self, width: u32, height: u32) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).29)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(width),
            ::std::mem::transmute(height),
        )
    }
    pub unsafe fn GetSourceSize(
        &self,
        p_width: *mut u32,
        p_height: *mut u32,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).30)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(p_width),
            ::std::mem::transmute(p_height),
        )
    }
    pub unsafe fn SetMaximumFrameLatency(&self, max_latency: u32) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).31)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(max_latency),
        )
    }
    pub unsafe fn GetMaximumFrameLatency(&self, p_max_latency: *mut u32) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).32)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(p_max_latency),
        )
    }
    pub unsafe fn GetFrameLatencyWaitableObject(&self) -> super::system_services::HANDLE {
        (::windows::Interface::vtable(self).33)(::windows::Abi::abi(self))
    }
    pub unsafe fn SetMatrixTransform(
        &self,
        p_matrix: *const DXGI_MATRIX_3X2_F,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).34)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(p_matrix),
        )
    }
    pub unsafe fn GetMatrixTransform(
        &self,
        p_matrix: *mut DXGI_MATRIX_3X2_F,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).35)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(p_matrix),
        )
    }
    pub unsafe fn GetCurrentBackBufferIndex(&self) -> u32 {
        (::windows::Interface::vtable(self).36)(::windows::Abi::abi(self))
    }
    pub unsafe fn CheckColorSpaceSupport(
        &self,
        color_space: DXGI_COLOR_SPACE_TYPE,
        p_color_space_support: *mut u32,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).37)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(color_space),
            ::std::mem::transmute(p_color_space_support),
        )
    }
    pub unsafe fn SetColorSpace1(
        &self,
        color_space: DXGI_COLOR_SPACE_TYPE,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).38)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(color_space),
        )
    }
    pub unsafe fn ResizeBuffers1(
        &self,
        buffer_count: u32,
        width: u32,
        height: u32,
        format: DXGI_FORMAT,
        swap_chain_flags: u32,
        p_creation_node_mask: *const u32,
        pp_present_queue: *mut ::std::option::Option<::windows::IUnknown>,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).39)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(buffer_count),
            ::std::mem::transmute(width),
            ::std::mem::transmute(height),
            ::std::mem::transmute(format),
            ::std::mem::transmute(swap_chain_flags),
            ::std::mem::transmute(p_creation_node_mask),
            ::std::mem::transmute(pp_present_queue),
        )
    }
    pub unsafe fn SetHDRMetaData(
        &self,
        r#type: DXGI_HDR_METADATA_TYPE,
        size: u32,
        p_meta_data: *mut ::std::ffi::c_void,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).40)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(r#type),
            ::std::mem::transmute(size),
            ::std::mem::transmute(p_meta_data),
        )
    }
}
impl ::std::convert::From<IDXGISwapChain4> for ::windows::IUnknown {
    fn from(value: IDXGISwapChain4) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IDXGISwapChain4> for ::windows::IUnknown {
    fn from(value: &IDXGISwapChain4) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::IntoParam<'a, ::windows::IUnknown> for IDXGISwapChain4 {
    fn into_param(self) -> ::windows::Param<'a, ::windows::IUnknown> {
        ::windows::Param::Owned(::std::convert::Into::<::windows::IUnknown>::into(self))
    }
}
impl<'a> ::windows::IntoParam<'a, ::windows::IUnknown> for &'a IDXGISwapChain4 {
    fn into_param(self) -> ::windows::Param<'a, ::windows::IUnknown> {
        ::windows::Param::Owned(::std::convert::Into::<::windows::IUnknown>::into(
            ::std::clone::Clone::clone(self),
        ))
    }
}
impl ::std::convert::From<IDXGISwapChain4> for IDXGISwapChain3 {
    fn from(value: IDXGISwapChain4) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IDXGISwapChain4> for IDXGISwapChain3 {
    fn from(value: &IDXGISwapChain4) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::IntoParam<'a, IDXGISwapChain3> for IDXGISwapChain4 {
    fn into_param(self) -> ::windows::Param<'a, IDXGISwapChain3> {
        ::windows::Param::Owned(::std::convert::Into::<IDXGISwapChain3>::into(self))
    }
}
impl<'a> ::windows::IntoParam<'a, IDXGISwapChain3> for &'a IDXGISwapChain4 {
    fn into_param(self) -> ::windows::Param<'a, IDXGISwapChain3> {
        ::windows::Param::Owned(::std::convert::Into::<IDXGISwapChain3>::into(
            ::std::clone::Clone::clone(self),
        ))
    }
}
impl ::std::convert::From<IDXGISwapChain4> for IDXGISwapChain2 {
    fn from(value: IDXGISwapChain4) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IDXGISwapChain4> for IDXGISwapChain2 {
    fn from(value: &IDXGISwapChain4) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::IntoParam<'a, IDXGISwapChain2> for IDXGISwapChain4 {
    fn into_param(self) -> ::windows::Param<'a, IDXGISwapChain2> {
        ::windows::Param::Owned(::std::convert::Into::<IDXGISwapChain2>::into(self))
    }
}
impl<'a> ::windows::IntoParam<'a, IDXGISwapChain2> for &'a IDXGISwapChain4 {
    fn into_param(self) -> ::windows::Param<'a, IDXGISwapChain2> {
        ::windows::Param::Owned(::std::convert::Into::<IDXGISwapChain2>::into(
            ::std::clone::Clone::clone(self),
        ))
    }
}
impl ::std::convert::From<IDXGISwapChain4> for IDXGISwapChain1 {
    fn from(value: IDXGISwapChain4) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IDXGISwapChain4> for IDXGISwapChain1 {
    fn from(value: &IDXGISwapChain4) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::IntoParam<'a, IDXGISwapChain1> for IDXGISwapChain4 {
    fn into_param(self) -> ::windows::Param<'a, IDXGISwapChain1> {
        ::windows::Param::Owned(::std::convert::Into::<IDXGISwapChain1>::into(self))
    }
}
impl<'a> ::windows::IntoParam<'a, IDXGISwapChain1> for &'a IDXGISwapChain4 {
    fn into_param(self) -> ::windows::Param<'a, IDXGISwapChain1> {
        ::windows::Param::Owned(::std::convert::Into::<IDXGISwapChain1>::into(
            ::std::clone::Clone::clone(self),
        ))
    }
}
impl ::std::convert::From<IDXGISwapChain4> for IDXGISwapChain {
    fn from(value: IDXGISwapChain4) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IDXGISwapChain4> for IDXGISwapChain {
    fn from(value: &IDXGISwapChain4) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::IntoParam<'a, IDXGISwapChain> for IDXGISwapChain4 {
    fn into_param(self) -> ::windows::Param<'a, IDXGISwapChain> {
        ::windows::Param::Owned(::std::convert::Into::<IDXGISwapChain>::into(self))
    }
}
impl<'a> ::windows::IntoParam<'a, IDXGISwapChain> for &'a IDXGISwapChain4 {
    fn into_param(self) -> ::windows::Param<'a, IDXGISwapChain> {
        ::windows::Param::Owned(::std::convert::Into::<IDXGISwapChain>::into(
            ::std::clone::Clone::clone(self),
        ))
    }
}
impl ::std::convert::From<IDXGISwapChain4> for IDXGIDeviceSubObject {
    fn from(value: IDXGISwapChain4) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IDXGISwapChain4> for IDXGIDeviceSubObject {
    fn from(value: &IDXGISwapChain4) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::IntoParam<'a, IDXGIDeviceSubObject> for IDXGISwapChain4 {
    fn into_param(self) -> ::windows::Param<'a, IDXGIDeviceSubObject> {
        ::windows::Param::Owned(::std::convert::Into::<IDXGIDeviceSubObject>::into(self))
    }
}
impl<'a> ::windows::IntoParam<'a, IDXGIDeviceSubObject> for &'a IDXGISwapChain4 {
    fn into_param(self) -> ::windows::Param<'a, IDXGIDeviceSubObject> {
        ::windows::Param::Owned(::std::convert::Into::<IDXGIDeviceSubObject>::into(
            ::std::clone::Clone::clone(self),
        ))
    }
}
impl ::std::convert::From<IDXGISwapChain4> for IDXGIObject {
    fn from(value: IDXGISwapChain4) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IDXGISwapChain4> for IDXGIObject {
    fn from(value: &IDXGISwapChain4) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::IntoParam<'a, IDXGIObject> for IDXGISwapChain4 {
    fn into_param(self) -> ::windows::Param<'a, IDXGIObject> {
        ::windows::Param::Owned(::std::convert::Into::<IDXGIObject>::into(self))
    }
}
impl<'a> ::windows::IntoParam<'a, IDXGIObject> for &'a IDXGISwapChain4 {
    fn into_param(self) -> ::windows::Param<'a, IDXGIObject> {
        ::windows::Param::Owned(::std::convert::Into::<IDXGIObject>::into(
            ::std::clone::Clone::clone(self),
        ))
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDXGISwapChain4_abi(
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        iid: &::windows::Guid,
        interface: *mut ::windows::RawPtr,
    ) -> ::windows::ErrorCode,
    pub unsafe extern "system" fn(this: ::windows::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::RawPtr) -> u32,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        name: *const ::windows::Guid,
        data_size: u32,
        p_data: *const ::std::ffi::c_void,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        name: *const ::windows::Guid,
        p_unknown: ::windows::RawPtr,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        name: *const ::windows::Guid,
        p_data_size: *mut u32,
        p_data: *mut ::std::ffi::c_void,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        riid: *const ::windows::Guid,
        pp_parent: *mut *mut ::std::ffi::c_void,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        riid: *const ::windows::Guid,
        pp_device: *mut *mut ::std::ffi::c_void,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        sync_interval: u32,
        flags: u32,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        buffer: u32,
        riid: *const ::windows::Guid,
        pp_surface: *mut *mut ::std::ffi::c_void,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        fullscreen: super::system_services::BOOL,
        p_target: ::windows::RawPtr,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        p_fullscreen: *mut super::system_services::BOOL,
        pp_target: *mut ::windows::RawPtr,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        p_desc: *mut DXGI_SWAP_CHAIN_DESC,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        buffer_count: u32,
        width: u32,
        height: u32,
        new_format: DXGI_FORMAT,
        swap_chain_flags: u32,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        p_new_target_parameters: *const DXGI_MODE_DESC,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        pp_output: *mut ::windows::RawPtr,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        p_stats: *mut DXGI_FRAME_STATISTICS,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        p_last_present_count: *mut u32,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        p_desc: *mut DXGI_SWAP_CHAIN_DESC1,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        p_desc: *mut DXGI_SWAP_CHAIN_FULLSCREEN_DESC,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        p_hwnd: *mut super::windows_and_messaging::HWND,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        refiid: *const ::windows::Guid,
        pp_unk: *mut *mut ::std::ffi::c_void,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        sync_interval: u32,
        present_flags: u32,
        p_present_parameters: *const DXGI_PRESENT_PARAMETERS,
    ) -> ::windows::ErrorCode,
    pub unsafe extern "system" fn(this: ::windows::RawPtr) -> super::system_services::BOOL,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        pp_restrict_to_output: *mut ::windows::RawPtr,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        p_color: *const DXGI_RGBA,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        p_color: *mut DXGI_RGBA,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        rotation: DXGI_MODE_ROTATION,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        p_rotation: *mut DXGI_MODE_ROTATION,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        width: u32,
        height: u32,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        p_width: *mut u32,
        p_height: *mut u32,
    ) -> ::windows::ErrorCode,
    pub unsafe extern "system" fn(this: ::windows::RawPtr, max_latency: u32) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        p_max_latency: *mut u32,
    ) -> ::windows::ErrorCode,
    pub unsafe extern "system" fn(this: ::windows::RawPtr) -> super::system_services::HANDLE,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        p_matrix: *const DXGI_MATRIX_3X2_F,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        p_matrix: *mut DXGI_MATRIX_3X2_F,
    ) -> ::windows::ErrorCode,
    pub unsafe extern "system" fn(this: ::windows::RawPtr) -> u32,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        color_space: DXGI_COLOR_SPACE_TYPE,
        p_color_space_support: *mut u32,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        color_space: DXGI_COLOR_SPACE_TYPE,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        buffer_count: u32,
        width: u32,
        height: u32,
        format: DXGI_FORMAT,
        swap_chain_flags: u32,
        p_creation_node_mask: *const u32,
        pp_present_queue: *mut ::windows::RawPtr,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        r#type: DXGI_HDR_METADATA_TYPE,
        size: u32,
        p_meta_data: *mut ::std::ffi::c_void,
    ) -> ::windows::ErrorCode,
);
#[repr(transparent)]
#[allow(non_camel_case_types)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct IDXGISwapChainMedia(::windows::IUnknown);
impl IDXGISwapChainMedia {}
unsafe impl ::windows::Interface for IDXGISwapChainMedia {
    type Vtable = IDXGISwapChainMedia_abi;
    const IID: ::windows::Guid = ::windows::Guid::from_values(
        3717576971,
        61535,
        20330,
        [189, 101, 37, 191, 178, 100, 189, 132],
    );
}
#[allow(non_snake_case)]
impl IDXGISwapChainMedia {
    pub unsafe fn GetFrameStatisticsMedia(
        &self,
        p_stats: *mut DXGI_FRAME_STATISTICS_MEDIA,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).3)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(p_stats),
        )
    }
    pub unsafe fn SetPresentDuration(&self, duration: u32) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).4)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(duration),
        )
    }
    pub unsafe fn CheckPresentDurationSupport(
        &self,
        desired_present_duration: u32,
        p_closest_smaller_present_duration: *mut u32,
        p_closest_larger_present_duration: *mut u32,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).5)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(desired_present_duration),
            ::std::mem::transmute(p_closest_smaller_present_duration),
            ::std::mem::transmute(p_closest_larger_present_duration),
        )
    }
}
impl ::std::convert::From<IDXGISwapChainMedia> for ::windows::IUnknown {
    fn from(value: IDXGISwapChainMedia) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IDXGISwapChainMedia> for ::windows::IUnknown {
    fn from(value: &IDXGISwapChainMedia) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::IntoParam<'a, ::windows::IUnknown> for IDXGISwapChainMedia {
    fn into_param(self) -> ::windows::Param<'a, ::windows::IUnknown> {
        ::windows::Param::Owned(::std::convert::Into::<::windows::IUnknown>::into(self))
    }
}
impl<'a> ::windows::IntoParam<'a, ::windows::IUnknown> for &'a IDXGISwapChainMedia {
    fn into_param(self) -> ::windows::Param<'a, ::windows::IUnknown> {
        ::windows::Param::Owned(::std::convert::Into::<::windows::IUnknown>::into(
            ::std::clone::Clone::clone(self),
        ))
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDXGISwapChainMedia_abi(
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        iid: &::windows::Guid,
        interface: *mut ::windows::RawPtr,
    ) -> ::windows::ErrorCode,
    pub unsafe extern "system" fn(this: ::windows::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::RawPtr) -> u32,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        p_stats: *mut DXGI_FRAME_STATISTICS_MEDIA,
    ) -> ::windows::ErrorCode,
    pub unsafe extern "system" fn(this: ::windows::RawPtr, duration: u32) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        desired_present_duration: u32,
        p_closest_smaller_present_duration: *mut u32,
        p_closest_larger_present_duration: *mut u32,
    ) -> ::windows::ErrorCode,
);
