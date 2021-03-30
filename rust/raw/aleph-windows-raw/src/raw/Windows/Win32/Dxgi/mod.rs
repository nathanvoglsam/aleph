#![allow(
    unused_variables,
    non_upper_case_globals,
    non_snake_case,
    unused_unsafe,
    non_camel_case_types,
    dead_code,
    clippy::all
)]
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
impl ::std::ops::BitOrAssign for DXGI_FORMAT {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0.bitor_assign(rhs.0)
    }
}
impl ::std::ops::BitAndAssign for DXGI_FORMAT {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0.bitand_assign(rhs.0)
    }
}
#[repr(C)]
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
pub struct DXGI_SAMPLE_DESC {
    pub Count: u32,
    pub Quality: u32,
}
impl DXGI_SAMPLE_DESC {}
impl ::std::default::Default for DXGI_SAMPLE_DESC {
    fn default() -> Self {
        Self {
            Count: 0,
            Quality: 0,
        }
    }
}
impl ::std::fmt::Debug for DXGI_SAMPLE_DESC {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("DXGI_SAMPLE_DESC")
            .field("Count", &format_args!("{:?}", self.Count))
            .field("Quality", &format_args!("{:?}", self.Quality))
            .finish()
    }
}
impl ::std::cmp::PartialEq for DXGI_SAMPLE_DESC {
    fn eq(&self, other: &Self) -> bool {
        self.Count == other.Count && self.Quality == other.Quality
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
    ppfactory: *mut *mut ::std::ffi::c_void,
) -> ::windows::ErrorCode {
    #[link(name = "dxgi")]
    extern "system" {
        pub fn CreateDXGIFactory(
            riid: *const ::windows::Guid,
            ppfactory: *mut *mut ::std::ffi::c_void,
        ) -> ::windows::ErrorCode;
    }
    CreateDXGIFactory(
        ::std::mem::transmute(riid),
        ::std::mem::transmute(ppfactory),
    )
}
pub unsafe fn CreateDXGIFactory1(
    riid: *const ::windows::Guid,
    ppfactory: *mut *mut ::std::ffi::c_void,
) -> ::windows::ErrorCode {
    #[link(name = "dxgi")]
    extern "system" {
        pub fn CreateDXGIFactory1(
            riid: *const ::windows::Guid,
            ppfactory: *mut *mut ::std::ffi::c_void,
        ) -> ::windows::ErrorCode;
    }
    CreateDXGIFactory1(
        ::std::mem::transmute(riid),
        ::std::mem::transmute(ppfactory),
    )
}
pub unsafe fn CreateDXGIFactory2(
    flags: u32,
    riid: *const ::windows::Guid,
    ppfactory: *mut *mut ::std::ffi::c_void,
) -> ::windows::ErrorCode {
    #[link(name = "dxgi")]
    extern "system" {
        pub fn CreateDXGIFactory2(
            flags: u32,
            riid: *const ::windows::Guid,
            ppfactory: *mut *mut ::std::ffi::c_void,
        ) -> ::windows::ErrorCode;
    }
    CreateDXGIFactory2(
        ::std::mem::transmute(flags),
        ::std::mem::transmute(riid),
        ::std::mem::transmute(ppfactory),
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
    pdebug: *mut *mut ::std::ffi::c_void,
) -> ::windows::ErrorCode {
    #[link(name = "dxgi")]
    extern "system" {
        pub fn DXGIGetDebugInterface1(
            flags: u32,
            riid: *const ::windows::Guid,
            pdebug: *mut *mut ::std::ffi::c_void,
        ) -> ::windows::ErrorCode;
    }
    DXGIGetDebugInterface1(
        ::std::mem::transmute(flags),
        ::std::mem::transmute(riid),
        ::std::mem::transmute(pdebug),
    )
}
#[repr(C)]
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
pub struct DXGI_ADAPTER_DESC {
    pub Description: [u16; 128],
    pub VendorId: u32,
    pub DeviceId: u32,
    pub SubSysId: u32,
    pub Revision: u32,
    pub DedicatedVideoMemory: usize,
    pub DedicatedSystemMemory: usize,
    pub SharedSystemMemory: usize,
    pub AdapterLuid: super::Kernel::LUID,
}
impl DXGI_ADAPTER_DESC {}
impl ::std::default::Default for DXGI_ADAPTER_DESC {
    fn default() -> Self {
        Self {
            Description: [0; 128],
            VendorId: 0,
            DeviceId: 0,
            SubSysId: 0,
            Revision: 0,
            DedicatedVideoMemory: 0,
            DedicatedSystemMemory: 0,
            SharedSystemMemory: 0,
            AdapterLuid: ::std::default::Default::default(),
        }
    }
}
impl ::std::fmt::Debug for DXGI_ADAPTER_DESC {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("DXGI_ADAPTER_DESC")
            .field("Description", &format_args!("{:?}", self.Description))
            .field("VendorId", &format_args!("{:?}", self.VendorId))
            .field("DeviceId", &format_args!("{:?}", self.DeviceId))
            .field("SubSysId", &format_args!("{:?}", self.SubSysId))
            .field("Revision", &format_args!("{:?}", self.Revision))
            .field(
                "DedicatedVideoMemory",
                &format_args!("{:?}", self.DedicatedVideoMemory),
            )
            .field(
                "DedicatedSystemMemory",
                &format_args!("{:?}", self.DedicatedSystemMemory),
            )
            .field(
                "SharedSystemMemory",
                &format_args!("{:?}", self.SharedSystemMemory),
            )
            .field("AdapterLuid", &format_args!("{:?}", self.AdapterLuid))
            .finish()
    }
}
impl ::std::cmp::PartialEq for DXGI_ADAPTER_DESC {
    fn eq(&self, other: &Self) -> bool {
        self.Description == other.Description
            && self.VendorId == other.VendorId
            && self.DeviceId == other.DeviceId
            && self.SubSysId == other.SubSysId
            && self.Revision == other.Revision
            && self.DedicatedVideoMemory == other.DedicatedVideoMemory
            && self.DedicatedSystemMemory == other.DedicatedSystemMemory
            && self.SharedSystemMemory == other.SharedSystemMemory
            && self.AdapterLuid == other.AdapterLuid
    }
}
impl ::std::cmp::Eq for DXGI_ADAPTER_DESC {}
unsafe impl ::windows::Abi for DXGI_ADAPTER_DESC {
    type Abi = Self;
}
#[repr(C)]
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
pub struct DXGI_ADAPTER_DESC1 {
    pub Description: [u16; 128],
    pub VendorId: u32,
    pub DeviceId: u32,
    pub SubSysId: u32,
    pub Revision: u32,
    pub DedicatedVideoMemory: usize,
    pub DedicatedSystemMemory: usize,
    pub SharedSystemMemory: usize,
    pub AdapterLuid: super::Kernel::LUID,
    pub Flags: u32,
}
impl DXGI_ADAPTER_DESC1 {}
impl ::std::default::Default for DXGI_ADAPTER_DESC1 {
    fn default() -> Self {
        Self {
            Description: [0; 128],
            VendorId: 0,
            DeviceId: 0,
            SubSysId: 0,
            Revision: 0,
            DedicatedVideoMemory: 0,
            DedicatedSystemMemory: 0,
            SharedSystemMemory: 0,
            AdapterLuid: ::std::default::Default::default(),
            Flags: 0,
        }
    }
}
impl ::std::fmt::Debug for DXGI_ADAPTER_DESC1 {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("DXGI_ADAPTER_DESC1")
            .field("Description", &format_args!("{:?}", self.Description))
            .field("VendorId", &format_args!("{:?}", self.VendorId))
            .field("DeviceId", &format_args!("{:?}", self.DeviceId))
            .field("SubSysId", &format_args!("{:?}", self.SubSysId))
            .field("Revision", &format_args!("{:?}", self.Revision))
            .field(
                "DedicatedVideoMemory",
                &format_args!("{:?}", self.DedicatedVideoMemory),
            )
            .field(
                "DedicatedSystemMemory",
                &format_args!("{:?}", self.DedicatedSystemMemory),
            )
            .field(
                "SharedSystemMemory",
                &format_args!("{:?}", self.SharedSystemMemory),
            )
            .field("AdapterLuid", &format_args!("{:?}", self.AdapterLuid))
            .field("Flags", &format_args!("{:?}", self.Flags))
            .finish()
    }
}
impl ::std::cmp::PartialEq for DXGI_ADAPTER_DESC1 {
    fn eq(&self, other: &Self) -> bool {
        self.Description == other.Description
            && self.VendorId == other.VendorId
            && self.DeviceId == other.DeviceId
            && self.SubSysId == other.SubSysId
            && self.Revision == other.Revision
            && self.DedicatedVideoMemory == other.DedicatedVideoMemory
            && self.DedicatedSystemMemory == other.DedicatedSystemMemory
            && self.SharedSystemMemory == other.SharedSystemMemory
            && self.AdapterLuid == other.AdapterLuid
            && self.Flags == other.Flags
    }
}
impl ::std::cmp::Eq for DXGI_ADAPTER_DESC1 {}
unsafe impl ::windows::Abi for DXGI_ADAPTER_DESC1 {
    type Abi = Self;
}
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
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
pub struct DXGI_ADAPTER_DESC2 {
    pub Description: [u16; 128],
    pub VendorId: u32,
    pub DeviceId: u32,
    pub SubSysId: u32,
    pub Revision: u32,
    pub DedicatedVideoMemory: usize,
    pub DedicatedSystemMemory: usize,
    pub SharedSystemMemory: usize,
    pub AdapterLuid: super::Kernel::LUID,
    pub Flags: u32,
    pub GraphicsPreemptionGranularity: DXGI_GRAPHICS_PREEMPTION_GRANULARITY,
    pub ComputePreemptionGranularity: DXGI_COMPUTE_PREEMPTION_GRANULARITY,
}
impl DXGI_ADAPTER_DESC2 {}
impl ::std::default::Default for DXGI_ADAPTER_DESC2 {
    fn default() -> Self {
        Self {
            Description: [0; 128],
            VendorId: 0,
            DeviceId: 0,
            SubSysId: 0,
            Revision: 0,
            DedicatedVideoMemory: 0,
            DedicatedSystemMemory: 0,
            SharedSystemMemory: 0,
            AdapterLuid: ::std::default::Default::default(),
            Flags: 0,
            GraphicsPreemptionGranularity: ::std::default::Default::default(),
            ComputePreemptionGranularity: ::std::default::Default::default(),
        }
    }
}
impl ::std::fmt::Debug for DXGI_ADAPTER_DESC2 {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("DXGI_ADAPTER_DESC2")
            .field("Description", &format_args!("{:?}", self.Description))
            .field("VendorId", &format_args!("{:?}", self.VendorId))
            .field("DeviceId", &format_args!("{:?}", self.DeviceId))
            .field("SubSysId", &format_args!("{:?}", self.SubSysId))
            .field("Revision", &format_args!("{:?}", self.Revision))
            .field(
                "DedicatedVideoMemory",
                &format_args!("{:?}", self.DedicatedVideoMemory),
            )
            .field(
                "DedicatedSystemMemory",
                &format_args!("{:?}", self.DedicatedSystemMemory),
            )
            .field(
                "SharedSystemMemory",
                &format_args!("{:?}", self.SharedSystemMemory),
            )
            .field("AdapterLuid", &format_args!("{:?}", self.AdapterLuid))
            .field("Flags", &format_args!("{:?}", self.Flags))
            .field(
                "GraphicsPreemptionGranularity",
                &format_args!("{:?}", self.GraphicsPreemptionGranularity),
            )
            .field(
                "ComputePreemptionGranularity",
                &format_args!("{:?}", self.ComputePreemptionGranularity),
            )
            .finish()
    }
}
impl ::std::cmp::PartialEq for DXGI_ADAPTER_DESC2 {
    fn eq(&self, other: &Self) -> bool {
        self.Description == other.Description
            && self.VendorId == other.VendorId
            && self.DeviceId == other.DeviceId
            && self.SubSysId == other.SubSysId
            && self.Revision == other.Revision
            && self.DedicatedVideoMemory == other.DedicatedVideoMemory
            && self.DedicatedSystemMemory == other.DedicatedSystemMemory
            && self.SharedSystemMemory == other.SharedSystemMemory
            && self.AdapterLuid == other.AdapterLuid
            && self.Flags == other.Flags
            && self.GraphicsPreemptionGranularity == other.GraphicsPreemptionGranularity
            && self.ComputePreemptionGranularity == other.ComputePreemptionGranularity
    }
}
impl ::std::cmp::Eq for DXGI_ADAPTER_DESC2 {}
unsafe impl ::windows::Abi for DXGI_ADAPTER_DESC2 {
    type Abi = Self;
}
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
impl ::std::ops::BitOrAssign for DXGI_ADAPTER_FLAG3 {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0.bitor_assign(rhs.0)
    }
}
impl ::std::ops::BitAndAssign for DXGI_ADAPTER_FLAG3 {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0.bitand_assign(rhs.0)
    }
}
#[repr(C)]
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
pub struct DXGI_ADAPTER_DESC3 {
    pub Description: [u16; 128],
    pub VendorId: u32,
    pub DeviceId: u32,
    pub SubSysId: u32,
    pub Revision: u32,
    pub DedicatedVideoMemory: usize,
    pub DedicatedSystemMemory: usize,
    pub SharedSystemMemory: usize,
    pub AdapterLuid: super::Kernel::LUID,
    pub Flags: DXGI_ADAPTER_FLAG3,
    pub GraphicsPreemptionGranularity: DXGI_GRAPHICS_PREEMPTION_GRANULARITY,
    pub ComputePreemptionGranularity: DXGI_COMPUTE_PREEMPTION_GRANULARITY,
}
impl DXGI_ADAPTER_DESC3 {}
impl ::std::default::Default for DXGI_ADAPTER_DESC3 {
    fn default() -> Self {
        Self {
            Description: [0; 128],
            VendorId: 0,
            DeviceId: 0,
            SubSysId: 0,
            Revision: 0,
            DedicatedVideoMemory: 0,
            DedicatedSystemMemory: 0,
            SharedSystemMemory: 0,
            AdapterLuid: ::std::default::Default::default(),
            Flags: ::std::default::Default::default(),
            GraphicsPreemptionGranularity: ::std::default::Default::default(),
            ComputePreemptionGranularity: ::std::default::Default::default(),
        }
    }
}
impl ::std::fmt::Debug for DXGI_ADAPTER_DESC3 {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("DXGI_ADAPTER_DESC3")
            .field("Description", &format_args!("{:?}", self.Description))
            .field("VendorId", &format_args!("{:?}", self.VendorId))
            .field("DeviceId", &format_args!("{:?}", self.DeviceId))
            .field("SubSysId", &format_args!("{:?}", self.SubSysId))
            .field("Revision", &format_args!("{:?}", self.Revision))
            .field(
                "DedicatedVideoMemory",
                &format_args!("{:?}", self.DedicatedVideoMemory),
            )
            .field(
                "DedicatedSystemMemory",
                &format_args!("{:?}", self.DedicatedSystemMemory),
            )
            .field(
                "SharedSystemMemory",
                &format_args!("{:?}", self.SharedSystemMemory),
            )
            .field("AdapterLuid", &format_args!("{:?}", self.AdapterLuid))
            .field("Flags", &format_args!("{:?}", self.Flags))
            .field(
                "GraphicsPreemptionGranularity",
                &format_args!("{:?}", self.GraphicsPreemptionGranularity),
            )
            .field(
                "ComputePreemptionGranularity",
                &format_args!("{:?}", self.ComputePreemptionGranularity),
            )
            .finish()
    }
}
impl ::std::cmp::PartialEq for DXGI_ADAPTER_DESC3 {
    fn eq(&self, other: &Self) -> bool {
        self.Description == other.Description
            && self.VendorId == other.VendorId
            && self.DeviceId == other.DeviceId
            && self.SubSysId == other.SubSysId
            && self.Revision == other.Revision
            && self.DedicatedVideoMemory == other.DedicatedVideoMemory
            && self.DedicatedSystemMemory == other.DedicatedSystemMemory
            && self.SharedSystemMemory == other.SharedSystemMemory
            && self.AdapterLuid == other.AdapterLuid
            && self.Flags == other.Flags
            && self.GraphicsPreemptionGranularity == other.GraphicsPreemptionGranularity
            && self.ComputePreemptionGranularity == other.ComputePreemptionGranularity
    }
}
impl ::std::cmp::Eq for DXGI_ADAPTER_DESC3 {}
unsafe impl ::windows::Abi for DXGI_ADAPTER_DESC3 {
    type Abi = Self;
}
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
impl ::std::ops::BitOrAssign for DXGI_ADAPTER_FLAG {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0.bitor_assign(rhs.0)
    }
}
impl ::std::ops::BitAndAssign for DXGI_ADAPTER_FLAG {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0.bitand_assign(rhs.0)
    }
}
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
impl ::std::ops::BitOrAssign for DXGI_ALPHA_MODE {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0.bitor_assign(rhs.0)
    }
}
impl ::std::ops::BitAndAssign for DXGI_ALPHA_MODE {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0.bitand_assign(rhs.0)
    }
}
pub const DXGI_CENTER_MULTISAMPLE_QUALITY_PATTERN: u32 = 4294967294u32;
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
pub const DXGI_DEBUG_ALL: ::windows::Guid = ::windows::Guid::from_values(
    3834307203,
    55936,
    18699,
    [135, 230, 67, 233, 169, 207, 218, 8],
);
pub const DXGI_DEBUG_APP: ::windows::Guid =
    ::windows::Guid::from_values(114126337, 16921, 20157, [135, 9, 39, 237, 35, 54, 12, 98]);
pub const DXGI_DEBUG_BINARY_VERSION: u32 = 1u32;
pub const DXGI_DEBUG_DX: ::windows::Guid = ::windows::Guid::from_values(
    902682620,
    5042,
    16925,
    [165, 215, 126, 68, 81, 40, 125, 100],
);
pub const DXGI_DEBUG_DXGI: ::windows::Guid =
    ::windows::Guid::from_values(634247844, 45510, 18401, [172, 62, 152, 135, 91, 90, 46, 42]);
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
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
pub struct DXGI_DECODE_SWAP_CHAIN_DESC {
    pub Flags: u32,
}
impl DXGI_DECODE_SWAP_CHAIN_DESC {}
impl ::std::default::Default for DXGI_DECODE_SWAP_CHAIN_DESC {
    fn default() -> Self {
        Self { Flags: 0 }
    }
}
impl ::std::fmt::Debug for DXGI_DECODE_SWAP_CHAIN_DESC {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("DXGI_DECODE_SWAP_CHAIN_DESC")
            .field("Flags", &format_args!("{:?}", self.Flags))
            .finish()
    }
}
impl ::std::cmp::PartialEq for DXGI_DECODE_SWAP_CHAIN_DESC {
    fn eq(&self, other: &Self) -> bool {
        self.Flags == other.Flags
    }
}
impl ::std::cmp::Eq for DXGI_DECODE_SWAP_CHAIN_DESC {}
unsafe impl ::windows::Abi for DXGI_DECODE_SWAP_CHAIN_DESC {
    type Abi = Self;
}
#[repr(C)]
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
pub struct DXGI_DISPLAY_COLOR_SPACE {
    pub PrimaryCoordinates: [f32; 16],
    pub WhitePoints: [f32; 32],
}
impl DXGI_DISPLAY_COLOR_SPACE {}
impl ::std::default::Default for DXGI_DISPLAY_COLOR_SPACE {
    fn default() -> Self {
        Self {
            PrimaryCoordinates: [0.0; 16],
            WhitePoints: [0.0; 32],
        }
    }
}
impl ::std::fmt::Debug for DXGI_DISPLAY_COLOR_SPACE {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("DXGI_DISPLAY_COLOR_SPACE")
            .field(
                "PrimaryCoordinates",
                &format_args!("{:?}", self.PrimaryCoordinates),
            )
            .field("WhitePoints", &format_args!("{:?}", self.WhitePoints))
            .finish()
    }
}
impl ::std::cmp::PartialEq for DXGI_DISPLAY_COLOR_SPACE {
    fn eq(&self, other: &Self) -> bool {
        self.PrimaryCoordinates == other.PrimaryCoordinates && self.WhitePoints == other.WhitePoints
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
pub const DXGI_ERROR_ACCESS_DENIED: ::windows::ErrorCode =
    ::windows::ErrorCode(-2005270485i32 as _);
pub const DXGI_ERROR_ACCESS_LOST: ::windows::ErrorCode = ::windows::ErrorCode(-2005270490i32 as _);
pub const DXGI_ERROR_ALREADY_EXISTS: ::windows::ErrorCode =
    ::windows::ErrorCode(-2005270474i32 as _);
pub const DXGI_ERROR_CACHE_CORRUPT: ::windows::ErrorCode =
    ::windows::ErrorCode(-2005270477i32 as _);
pub const DXGI_ERROR_CACHE_FULL: ::windows::ErrorCode = ::windows::ErrorCode(-2005270476i32 as _);
pub const DXGI_ERROR_CACHE_HASH_COLLISION: ::windows::ErrorCode =
    ::windows::ErrorCode(-2005270475i32 as _);
pub const DXGI_ERROR_CANNOT_PROTECT_CONTENT: ::windows::ErrorCode =
    ::windows::ErrorCode(-2005270486i32 as _);
pub const DXGI_ERROR_DEVICE_HUNG: ::windows::ErrorCode = ::windows::ErrorCode(-2005270522i32 as _);
pub const DXGI_ERROR_DEVICE_REMOVED: ::windows::ErrorCode =
    ::windows::ErrorCode(-2005270523i32 as _);
pub const DXGI_ERROR_DEVICE_RESET: ::windows::ErrorCode = ::windows::ErrorCode(-2005270521i32 as _);
pub const DXGI_ERROR_DRIVER_INTERNAL_ERROR: ::windows::ErrorCode =
    ::windows::ErrorCode(-2005270496i32 as _);
pub const DXGI_ERROR_DYNAMIC_CODE_POLICY_VIOLATION: ::windows::ErrorCode =
    ::windows::ErrorCode(-2005270479i32 as _);
pub const DXGI_ERROR_FRAME_STATISTICS_DISJOINT: ::windows::ErrorCode =
    ::windows::ErrorCode(-2005270517i32 as _);
pub const DXGI_ERROR_GRAPHICS_VIDPN_SOURCE_IN_USE: ::windows::ErrorCode =
    ::windows::ErrorCode(-2005270516i32 as _);
pub const DXGI_ERROR_HW_PROTECTION_OUTOFMEMORY: ::windows::ErrorCode =
    ::windows::ErrorCode(-2005270480i32 as _);
pub const DXGI_ERROR_INVALID_CALL: ::windows::ErrorCode = ::windows::ErrorCode(-2005270527i32 as _);
pub const DXGI_ERROR_MODE_CHANGE_IN_PROGRESS: ::windows::ErrorCode =
    ::windows::ErrorCode(-2005270491i32 as _);
pub const DXGI_ERROR_MORE_DATA: ::windows::ErrorCode = ::windows::ErrorCode(-2005270525i32 as _);
pub const DXGI_ERROR_NAME_ALREADY_EXISTS: ::windows::ErrorCode =
    ::windows::ErrorCode(-2005270484i32 as _);
pub const DXGI_ERROR_NONEXCLUSIVE: ::windows::ErrorCode = ::windows::ErrorCode(-2005270495i32 as _);
pub const DXGI_ERROR_NON_COMPOSITED_UI: ::windows::ErrorCode =
    ::windows::ErrorCode(-2005270478i32 as _);
pub const DXGI_ERROR_NOT_CURRENT: ::windows::ErrorCode = ::windows::ErrorCode(-2005270482i32 as _);
pub const DXGI_ERROR_NOT_CURRENTLY_AVAILABLE: ::windows::ErrorCode =
    ::windows::ErrorCode(-2005270494i32 as _);
pub const DXGI_ERROR_NOT_FOUND: ::windows::ErrorCode = ::windows::ErrorCode(-2005270526i32 as _);
pub const DXGI_ERROR_REMOTE_CLIENT_DISCONNECTED: ::windows::ErrorCode =
    ::windows::ErrorCode(-2005270493i32 as _);
pub const DXGI_ERROR_REMOTE_OUTOFMEMORY: ::windows::ErrorCode =
    ::windows::ErrorCode(-2005270492i32 as _);
pub const DXGI_ERROR_RESTRICT_TO_OUTPUT_STALE: ::windows::ErrorCode =
    ::windows::ErrorCode(-2005270487i32 as _);
pub const DXGI_ERROR_SDK_COMPONENT_MISSING: ::windows::ErrorCode =
    ::windows::ErrorCode(-2005270483i32 as _);
pub const DXGI_ERROR_SESSION_DISCONNECTED: ::windows::ErrorCode =
    ::windows::ErrorCode(-2005270488i32 as _);
pub const DXGI_ERROR_UNSUPPORTED: ::windows::ErrorCode = ::windows::ErrorCode(-2005270524i32 as _);
pub const DXGI_ERROR_WAIT_TIMEOUT: ::windows::ErrorCode = ::windows::ErrorCode(-2005270489i32 as _);
pub const DXGI_ERROR_WAS_STILL_DRAWING: ::windows::ErrorCode =
    ::windows::ErrorCode(-2005270518i32 as _);
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
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
pub struct DXGI_FRAME_STATISTICS {
    pub PresentCount: u32,
    pub PresentRefreshCount: u32,
    pub SyncRefreshCount: u32,
    pub SyncQPCTime: i64,
    pub SyncGPUTime: i64,
}
impl DXGI_FRAME_STATISTICS {}
impl ::std::default::Default for DXGI_FRAME_STATISTICS {
    fn default() -> Self {
        Self {
            PresentCount: 0,
            PresentRefreshCount: 0,
            SyncRefreshCount: 0,
            SyncQPCTime: 0,
            SyncGPUTime: 0,
        }
    }
}
impl ::std::fmt::Debug for DXGI_FRAME_STATISTICS {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("DXGI_FRAME_STATISTICS")
            .field("PresentCount", &format_args!("{:?}", self.PresentCount))
            .field(
                "PresentRefreshCount",
                &format_args!("{:?}", self.PresentRefreshCount),
            )
            .field(
                "SyncRefreshCount",
                &format_args!("{:?}", self.SyncRefreshCount),
            )
            .field("SyncQPCTime", &format_args!("{:?}", self.SyncQPCTime))
            .field("SyncGPUTime", &format_args!("{:?}", self.SyncGPUTime))
            .finish()
    }
}
impl ::std::cmp::PartialEq for DXGI_FRAME_STATISTICS {
    fn eq(&self, other: &Self) -> bool {
        self.PresentCount == other.PresentCount
            && self.PresentRefreshCount == other.PresentRefreshCount
            && self.SyncRefreshCount == other.SyncRefreshCount
            && self.SyncQPCTime == other.SyncQPCTime
            && self.SyncGPUTime == other.SyncGPUTime
    }
}
impl ::std::cmp::Eq for DXGI_FRAME_STATISTICS {}
unsafe impl ::windows::Abi for DXGI_FRAME_STATISTICS {
    type Abi = Self;
}
#[repr(C)]
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
pub struct DXGI_FRAME_STATISTICS_MEDIA {
    pub PresentCount: u32,
    pub PresentRefreshCount: u32,
    pub SyncRefreshCount: u32,
    pub SyncQPCTime: i64,
    pub SyncGPUTime: i64,
    pub CompositionMode: DXGI_FRAME_PRESENTATION_MODE,
    pub ApprovedPresentDuration: u32,
}
impl DXGI_FRAME_STATISTICS_MEDIA {}
impl ::std::default::Default for DXGI_FRAME_STATISTICS_MEDIA {
    fn default() -> Self {
        Self {
            PresentCount: 0,
            PresentRefreshCount: 0,
            SyncRefreshCount: 0,
            SyncQPCTime: 0,
            SyncGPUTime: 0,
            CompositionMode: ::std::default::Default::default(),
            ApprovedPresentDuration: 0,
        }
    }
}
impl ::std::fmt::Debug for DXGI_FRAME_STATISTICS_MEDIA {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("DXGI_FRAME_STATISTICS_MEDIA")
            .field("PresentCount", &format_args!("{:?}", self.PresentCount))
            .field(
                "PresentRefreshCount",
                &format_args!("{:?}", self.PresentRefreshCount),
            )
            .field(
                "SyncRefreshCount",
                &format_args!("{:?}", self.SyncRefreshCount),
            )
            .field("SyncQPCTime", &format_args!("{:?}", self.SyncQPCTime))
            .field("SyncGPUTime", &format_args!("{:?}", self.SyncGPUTime))
            .field(
                "CompositionMode",
                &format_args!("{:?}", self.CompositionMode),
            )
            .field(
                "ApprovedPresentDuration",
                &format_args!("{:?}", self.ApprovedPresentDuration),
            )
            .finish()
    }
}
impl ::std::cmp::PartialEq for DXGI_FRAME_STATISTICS_MEDIA {
    fn eq(&self, other: &Self) -> bool {
        self.PresentCount == other.PresentCount
            && self.PresentRefreshCount == other.PresentRefreshCount
            && self.SyncRefreshCount == other.SyncRefreshCount
            && self.SyncQPCTime == other.SyncQPCTime
            && self.SyncGPUTime == other.SyncGPUTime
            && self.CompositionMode == other.CompositionMode
            && self.ApprovedPresentDuration == other.ApprovedPresentDuration
    }
}
impl ::std::cmp::Eq for DXGI_FRAME_STATISTICS_MEDIA {}
unsafe impl ::windows::Abi for DXGI_FRAME_STATISTICS_MEDIA {
    type Abi = Self;
}
#[repr(C)]
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
pub struct DXGI_RGB {
    pub Red: f32,
    pub Green: f32,
    pub Blue: f32,
}
impl DXGI_RGB {}
impl ::std::default::Default for DXGI_RGB {
    fn default() -> Self {
        Self {
            Red: 0.0,
            Green: 0.0,
            Blue: 0.0,
        }
    }
}
impl ::std::fmt::Debug for DXGI_RGB {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("DXGI_RGB")
            .field("Red", &format_args!("{:?}", self.Red))
            .field("Green", &format_args!("{:?}", self.Green))
            .field("Blue", &format_args!("{:?}", self.Blue))
            .finish()
    }
}
impl ::std::cmp::PartialEq for DXGI_RGB {
    fn eq(&self, other: &Self) -> bool {
        self.Red == other.Red && self.Green == other.Green && self.Blue == other.Blue
    }
}
impl ::std::cmp::Eq for DXGI_RGB {}
unsafe impl ::windows::Abi for DXGI_RGB {
    type Abi = Self;
}
#[repr(C)]
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
pub struct DXGI_GAMMA_CONTROL {
    pub Scale: DXGI_RGB,
    pub Offset: DXGI_RGB,
    pub GammaCurve: [DXGI_RGB; 1025],
}
impl DXGI_GAMMA_CONTROL {}
impl ::std::default::Default for DXGI_GAMMA_CONTROL {
    fn default() -> Self {
        Self {
            Scale: ::std::default::Default::default(),
            Offset: ::std::default::Default::default(),
            GammaCurve: [::std::default::Default::default(); 1025],
        }
    }
}
impl ::std::fmt::Debug for DXGI_GAMMA_CONTROL {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("DXGI_GAMMA_CONTROL")
            .field("Scale", &format_args!("{:?}", self.Scale))
            .field("Offset", &format_args!("{:?}", self.Offset))
            .field("GammaCurve", &format_args!("{:?}", self.GammaCurve))
            .finish()
    }
}
impl ::std::cmp::PartialEq for DXGI_GAMMA_CONTROL {
    fn eq(&self, other: &Self) -> bool {
        self.Scale == other.Scale
            && self.Offset == other.Offset
            && self.GammaCurve == other.GammaCurve
    }
}
impl ::std::cmp::Eq for DXGI_GAMMA_CONTROL {}
unsafe impl ::windows::Abi for DXGI_GAMMA_CONTROL {
    type Abi = Self;
}
#[repr(C)]
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
pub struct DXGI_GAMMA_CONTROL_CAPABILITIES {
    pub ScaleAndOffsetSupported: super::SystemServices::BOOL,
    pub MaxConvertedValue: f32,
    pub MinConvertedValue: f32,
    pub NumGammaControlPoints: u32,
    pub ControlPointPositions: [f32; 1025],
}
impl DXGI_GAMMA_CONTROL_CAPABILITIES {}
impl ::std::default::Default for DXGI_GAMMA_CONTROL_CAPABILITIES {
    fn default() -> Self {
        Self {
            ScaleAndOffsetSupported: ::std::default::Default::default(),
            MaxConvertedValue: 0.0,
            MinConvertedValue: 0.0,
            NumGammaControlPoints: 0,
            ControlPointPositions: [0.0; 1025],
        }
    }
}
impl ::std::fmt::Debug for DXGI_GAMMA_CONTROL_CAPABILITIES {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("DXGI_GAMMA_CONTROL_CAPABILITIES")
            .field(
                "ScaleAndOffsetSupported",
                &format_args!("{:?}", self.ScaleAndOffsetSupported),
            )
            .field(
                "MaxConvertedValue",
                &format_args!("{:?}", self.MaxConvertedValue),
            )
            .field(
                "MinConvertedValue",
                &format_args!("{:?}", self.MinConvertedValue),
            )
            .field(
                "NumGammaControlPoints",
                &format_args!("{:?}", self.NumGammaControlPoints),
            )
            .field(
                "ControlPointPositions",
                &format_args!("{:?}", self.ControlPointPositions),
            )
            .finish()
    }
}
impl ::std::cmp::PartialEq for DXGI_GAMMA_CONTROL_CAPABILITIES {
    fn eq(&self, other: &Self) -> bool {
        self.ScaleAndOffsetSupported == other.ScaleAndOffsetSupported
            && self.MaxConvertedValue == other.MaxConvertedValue
            && self.MinConvertedValue == other.MinConvertedValue
            && self.NumGammaControlPoints == other.NumGammaControlPoints
            && self.ControlPointPositions == other.ControlPointPositions
    }
}
impl ::std::cmp::Eq for DXGI_GAMMA_CONTROL_CAPABILITIES {}
unsafe impl ::windows::Abi for DXGI_GAMMA_CONTROL_CAPABILITIES {
    type Abi = Self;
}
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
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
pub struct DXGI_HDR_METADATA_HDR10 {
    pub RedPrimary: [u16; 2],
    pub GreenPrimary: [u16; 2],
    pub BluePrimary: [u16; 2],
    pub WhitePoint: [u16; 2],
    pub MaxMasteringLuminance: u32,
    pub MinMasteringLuminance: u32,
    pub MaxContentLightLevel: u16,
    pub MaxFrameAverageLightLevel: u16,
}
impl DXGI_HDR_METADATA_HDR10 {}
impl ::std::default::Default for DXGI_HDR_METADATA_HDR10 {
    fn default() -> Self {
        Self {
            RedPrimary: [0; 2],
            GreenPrimary: [0; 2],
            BluePrimary: [0; 2],
            WhitePoint: [0; 2],
            MaxMasteringLuminance: 0,
            MinMasteringLuminance: 0,
            MaxContentLightLevel: 0,
            MaxFrameAverageLightLevel: 0,
        }
    }
}
impl ::std::fmt::Debug for DXGI_HDR_METADATA_HDR10 {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("DXGI_HDR_METADATA_HDR10")
            .field("RedPrimary", &format_args!("{:?}", self.RedPrimary))
            .field("GreenPrimary", &format_args!("{:?}", self.GreenPrimary))
            .field("BluePrimary", &format_args!("{:?}", self.BluePrimary))
            .field("WhitePoint", &format_args!("{:?}", self.WhitePoint))
            .field(
                "MaxMasteringLuminance",
                &format_args!("{:?}", self.MaxMasteringLuminance),
            )
            .field(
                "MinMasteringLuminance",
                &format_args!("{:?}", self.MinMasteringLuminance),
            )
            .field(
                "MaxContentLightLevel",
                &format_args!("{:?}", self.MaxContentLightLevel),
            )
            .field(
                "MaxFrameAverageLightLevel",
                &format_args!("{:?}", self.MaxFrameAverageLightLevel),
            )
            .finish()
    }
}
impl ::std::cmp::PartialEq for DXGI_HDR_METADATA_HDR10 {
    fn eq(&self, other: &Self) -> bool {
        self.RedPrimary == other.RedPrimary
            && self.GreenPrimary == other.GreenPrimary
            && self.BluePrimary == other.BluePrimary
            && self.WhitePoint == other.WhitePoint
            && self.MaxMasteringLuminance == other.MaxMasteringLuminance
            && self.MinMasteringLuminance == other.MinMasteringLuminance
            && self.MaxContentLightLevel == other.MaxContentLightLevel
            && self.MaxFrameAverageLightLevel == other.MaxFrameAverageLightLevel
    }
}
impl ::std::cmp::Eq for DXGI_HDR_METADATA_HDR10 {}
unsafe impl ::windows::Abi for DXGI_HDR_METADATA_HDR10 {
    type Abi = Self;
}
#[repr(C)]
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
pub struct DXGI_HDR_METADATA_HDR10PLUS {
    pub Data: [u8; 72],
}
impl DXGI_HDR_METADATA_HDR10PLUS {}
impl ::std::default::Default for DXGI_HDR_METADATA_HDR10PLUS {
    fn default() -> Self {
        Self { Data: [0; 72] }
    }
}
impl ::std::fmt::Debug for DXGI_HDR_METADATA_HDR10PLUS {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("DXGI_HDR_METADATA_HDR10PLUS")
            .field("Data", &format_args!("{:?}", self.Data))
            .finish()
    }
}
impl ::std::cmp::PartialEq for DXGI_HDR_METADATA_HDR10PLUS {
    fn eq(&self, other: &Self) -> bool {
        self.Data == other.Data
    }
}
impl ::std::cmp::Eq for DXGI_HDR_METADATA_HDR10PLUS {}
unsafe impl ::windows::Abi for DXGI_HDR_METADATA_HDR10PLUS {
    type Abi = Self;
}
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
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
pub struct DXGI_INFO_QUEUE_FILTER_DESC {
    pub NumCategories: u32,
    pub pCategoryList: *mut DXGI_INFO_QUEUE_MESSAGE_CATEGORY,
    pub NumSeverities: u32,
    pub pSeverityList: *mut DXGI_INFO_QUEUE_MESSAGE_SEVERITY,
    pub NumIDs: u32,
    pub pIDList: *mut i32,
}
impl DXGI_INFO_QUEUE_FILTER_DESC {}
impl ::std::default::Default for DXGI_INFO_QUEUE_FILTER_DESC {
    fn default() -> Self {
        Self {
            NumCategories: 0,
            pCategoryList: ::std::ptr::null_mut(),
            NumSeverities: 0,
            pSeverityList: ::std::ptr::null_mut(),
            NumIDs: 0,
            pIDList: ::std::ptr::null_mut(),
        }
    }
}
impl ::std::fmt::Debug for DXGI_INFO_QUEUE_FILTER_DESC {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("DXGI_INFO_QUEUE_FILTER_DESC")
            .field("NumCategories", &format_args!("{:?}", self.NumCategories))
            .field("pCategoryList", &format_args!("{:?}", self.pCategoryList))
            .field("NumSeverities", &format_args!("{:?}", self.NumSeverities))
            .field("pSeverityList", &format_args!("{:?}", self.pSeverityList))
            .field("NumIDs", &format_args!("{:?}", self.NumIDs))
            .field("pIDList", &format_args!("{:?}", self.pIDList))
            .finish()
    }
}
impl ::std::cmp::PartialEq for DXGI_INFO_QUEUE_FILTER_DESC {
    fn eq(&self, other: &Self) -> bool {
        self.NumCategories == other.NumCategories
            && self.pCategoryList == other.pCategoryList
            && self.NumSeverities == other.NumSeverities
            && self.pSeverityList == other.pSeverityList
            && self.NumIDs == other.NumIDs
            && self.pIDList == other.pIDList
    }
}
impl ::std::cmp::Eq for DXGI_INFO_QUEUE_FILTER_DESC {}
unsafe impl ::windows::Abi for DXGI_INFO_QUEUE_FILTER_DESC {
    type Abi = Self;
}
#[repr(C)]
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
pub struct DXGI_INFO_QUEUE_FILTER {
    pub AllowList: DXGI_INFO_QUEUE_FILTER_DESC,
    pub DenyList: DXGI_INFO_QUEUE_FILTER_DESC,
}
impl DXGI_INFO_QUEUE_FILTER {}
impl ::std::default::Default for DXGI_INFO_QUEUE_FILTER {
    fn default() -> Self {
        Self {
            AllowList: ::std::default::Default::default(),
            DenyList: ::std::default::Default::default(),
        }
    }
}
impl ::std::fmt::Debug for DXGI_INFO_QUEUE_FILTER {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("DXGI_INFO_QUEUE_FILTER")
            .field("AllowList", &format_args!("{:?}", self.AllowList))
            .field("DenyList", &format_args!("{:?}", self.DenyList))
            .finish()
    }
}
impl ::std::cmp::PartialEq for DXGI_INFO_QUEUE_FILTER {
    fn eq(&self, other: &Self) -> bool {
        self.AllowList == other.AllowList && self.DenyList == other.DenyList
    }
}
impl ::std::cmp::Eq for DXGI_INFO_QUEUE_FILTER {}
unsafe impl ::windows::Abi for DXGI_INFO_QUEUE_FILTER {
    type Abi = Self;
}
#[repr(C)]
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
pub struct DXGI_INFO_QUEUE_MESSAGE {
    pub Producer: ::windows::Guid,
    pub Category: DXGI_INFO_QUEUE_MESSAGE_CATEGORY,
    pub Severity: DXGI_INFO_QUEUE_MESSAGE_SEVERITY,
    pub ID: i32,
    pub pDescription: *mut u8,
    pub DescriptionByteLength: usize,
}
impl DXGI_INFO_QUEUE_MESSAGE {}
impl ::std::default::Default for DXGI_INFO_QUEUE_MESSAGE {
    fn default() -> Self {
        Self {
            Producer: ::std::default::Default::default(),
            Category: ::std::default::Default::default(),
            Severity: ::std::default::Default::default(),
            ID: 0,
            pDescription: ::std::ptr::null_mut(),
            DescriptionByteLength: 0,
        }
    }
}
impl ::std::fmt::Debug for DXGI_INFO_QUEUE_MESSAGE {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("DXGI_INFO_QUEUE_MESSAGE")
            .field("Producer", &format_args!("{:?}", self.Producer))
            .field("Category", &format_args!("{:?}", self.Category))
            .field("Severity", &format_args!("{:?}", self.Severity))
            .field("ID", &format_args!("{:?}", self.ID))
            .field("pDescription", &format_args!("{:?}", self.pDescription))
            .field(
                "DescriptionByteLength",
                &format_args!("{:?}", self.DescriptionByteLength),
            )
            .finish()
    }
}
impl ::std::cmp::PartialEq for DXGI_INFO_QUEUE_MESSAGE {
    fn eq(&self, other: &Self) -> bool {
        self.Producer == other.Producer
            && self.Category == other.Category
            && self.Severity == other.Severity
            && self.ID == other.ID
            && self.pDescription == other.pDescription
            && self.DescriptionByteLength == other.DescriptionByteLength
    }
}
impl ::std::cmp::Eq for DXGI_INFO_QUEUE_MESSAGE {}
unsafe impl ::windows::Abi for DXGI_INFO_QUEUE_MESSAGE {
    type Abi = Self;
}
pub const DXGI_INFO_QUEUE_MESSAGE_ID_STRING_FROM_APPLICATION: u32 = 0u32;
#[repr(C)]
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
pub struct DXGI_JPEG_AC_HUFFMAN_TABLE {
    pub CodeCounts: [u8; 16],
    pub CodeValues: [u8; 162],
}
impl DXGI_JPEG_AC_HUFFMAN_TABLE {}
impl ::std::default::Default for DXGI_JPEG_AC_HUFFMAN_TABLE {
    fn default() -> Self {
        Self {
            CodeCounts: [0; 16],
            CodeValues: [0; 162],
        }
    }
}
impl ::std::fmt::Debug for DXGI_JPEG_AC_HUFFMAN_TABLE {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("DXGI_JPEG_AC_HUFFMAN_TABLE")
            .field("CodeCounts", &format_args!("{:?}", self.CodeCounts))
            .field("CodeValues", &format_args!("{:?}", self.CodeValues))
            .finish()
    }
}
impl ::std::cmp::PartialEq for DXGI_JPEG_AC_HUFFMAN_TABLE {
    fn eq(&self, other: &Self) -> bool {
        self.CodeCounts == other.CodeCounts && self.CodeValues == other.CodeValues
    }
}
impl ::std::cmp::Eq for DXGI_JPEG_AC_HUFFMAN_TABLE {}
unsafe impl ::windows::Abi for DXGI_JPEG_AC_HUFFMAN_TABLE {
    type Abi = Self;
}
#[repr(C)]
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
pub struct DXGI_JPEG_DC_HUFFMAN_TABLE {
    pub CodeCounts: [u8; 12],
    pub CodeValues: [u8; 12],
}
impl DXGI_JPEG_DC_HUFFMAN_TABLE {}
impl ::std::default::Default for DXGI_JPEG_DC_HUFFMAN_TABLE {
    fn default() -> Self {
        Self {
            CodeCounts: [0; 12],
            CodeValues: [0; 12],
        }
    }
}
impl ::std::fmt::Debug for DXGI_JPEG_DC_HUFFMAN_TABLE {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("DXGI_JPEG_DC_HUFFMAN_TABLE")
            .field("CodeCounts", &format_args!("{:?}", self.CodeCounts))
            .field("CodeValues", &format_args!("{:?}", self.CodeValues))
            .finish()
    }
}
impl ::std::cmp::PartialEq for DXGI_JPEG_DC_HUFFMAN_TABLE {
    fn eq(&self, other: &Self) -> bool {
        self.CodeCounts == other.CodeCounts && self.CodeValues == other.CodeValues
    }
}
impl ::std::cmp::Eq for DXGI_JPEG_DC_HUFFMAN_TABLE {}
unsafe impl ::windows::Abi for DXGI_JPEG_DC_HUFFMAN_TABLE {
    type Abi = Self;
}
#[repr(C)]
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
pub struct DXGI_JPEG_QUANTIZATION_TABLE {
    pub Elements: [u8; 64],
}
impl DXGI_JPEG_QUANTIZATION_TABLE {}
impl ::std::default::Default for DXGI_JPEG_QUANTIZATION_TABLE {
    fn default() -> Self {
        Self { Elements: [0; 64] }
    }
}
impl ::std::fmt::Debug for DXGI_JPEG_QUANTIZATION_TABLE {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("DXGI_JPEG_QUANTIZATION_TABLE")
            .field("Elements", &format_args!("{:?}", self.Elements))
            .finish()
    }
}
impl ::std::cmp::PartialEq for DXGI_JPEG_QUANTIZATION_TABLE {
    fn eq(&self, other: &Self) -> bool {
        self.Elements == other.Elements
    }
}
impl ::std::cmp::Eq for DXGI_JPEG_QUANTIZATION_TABLE {}
unsafe impl ::windows::Abi for DXGI_JPEG_QUANTIZATION_TABLE {
    type Abi = Self;
}
#[repr(C)]
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
pub struct DXGI_MAPPED_RECT {
    pub Pitch: i32,
    pub pBits: *mut u8,
}
impl DXGI_MAPPED_RECT {}
impl ::std::default::Default for DXGI_MAPPED_RECT {
    fn default() -> Self {
        Self {
            Pitch: 0,
            pBits: ::std::ptr::null_mut(),
        }
    }
}
impl ::std::fmt::Debug for DXGI_MAPPED_RECT {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("DXGI_MAPPED_RECT")
            .field("Pitch", &format_args!("{:?}", self.Pitch))
            .field("pBits", &format_args!("{:?}", self.pBits))
            .finish()
    }
}
impl ::std::cmp::PartialEq for DXGI_MAPPED_RECT {
    fn eq(&self, other: &Self) -> bool {
        self.Pitch == other.Pitch && self.pBits == other.pBits
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
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
pub struct DXGI_RATIONAL {
    pub Numerator: u32,
    pub Denominator: u32,
}
impl DXGI_RATIONAL {}
impl ::std::default::Default for DXGI_RATIONAL {
    fn default() -> Self {
        Self {
            Numerator: 0,
            Denominator: 0,
        }
    }
}
impl ::std::fmt::Debug for DXGI_RATIONAL {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("DXGI_RATIONAL")
            .field("Numerator", &format_args!("{:?}", self.Numerator))
            .field("Denominator", &format_args!("{:?}", self.Denominator))
            .finish()
    }
}
impl ::std::cmp::PartialEq for DXGI_RATIONAL {
    fn eq(&self, other: &Self) -> bool {
        self.Numerator == other.Numerator && self.Denominator == other.Denominator
    }
}
impl ::std::cmp::Eq for DXGI_RATIONAL {}
unsafe impl ::windows::Abi for DXGI_RATIONAL {
    type Abi = Self;
}
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
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
pub struct DXGI_MODE_DESC {
    pub Width: u32,
    pub Height: u32,
    pub RefreshRate: DXGI_RATIONAL,
    pub Format: DXGI_FORMAT,
    pub ScanlineOrdering: DXGI_MODE_SCANLINE_ORDER,
    pub Scaling: DXGI_MODE_SCALING,
}
impl DXGI_MODE_DESC {}
impl ::std::default::Default for DXGI_MODE_DESC {
    fn default() -> Self {
        Self {
            Width: 0,
            Height: 0,
            RefreshRate: ::std::default::Default::default(),
            Format: ::std::default::Default::default(),
            ScanlineOrdering: ::std::default::Default::default(),
            Scaling: ::std::default::Default::default(),
        }
    }
}
impl ::std::fmt::Debug for DXGI_MODE_DESC {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("DXGI_MODE_DESC")
            .field("Width", &format_args!("{:?}", self.Width))
            .field("Height", &format_args!("{:?}", self.Height))
            .field("RefreshRate", &format_args!("{:?}", self.RefreshRate))
            .field("Format", &format_args!("{:?}", self.Format))
            .field(
                "ScanlineOrdering",
                &format_args!("{:?}", self.ScanlineOrdering),
            )
            .field("Scaling", &format_args!("{:?}", self.Scaling))
            .finish()
    }
}
impl ::std::cmp::PartialEq for DXGI_MODE_DESC {
    fn eq(&self, other: &Self) -> bool {
        self.Width == other.Width
            && self.Height == other.Height
            && self.RefreshRate == other.RefreshRate
            && self.Format == other.Format
            && self.ScanlineOrdering == other.ScanlineOrdering
            && self.Scaling == other.Scaling
    }
}
impl ::std::cmp::Eq for DXGI_MODE_DESC {}
unsafe impl ::windows::Abi for DXGI_MODE_DESC {
    type Abi = Self;
}
#[repr(C)]
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
pub struct DXGI_MODE_DESC1 {
    pub Width: u32,
    pub Height: u32,
    pub RefreshRate: DXGI_RATIONAL,
    pub Format: DXGI_FORMAT,
    pub ScanlineOrdering: DXGI_MODE_SCANLINE_ORDER,
    pub Scaling: DXGI_MODE_SCALING,
    pub Stereo: super::SystemServices::BOOL,
}
impl DXGI_MODE_DESC1 {}
impl ::std::default::Default for DXGI_MODE_DESC1 {
    fn default() -> Self {
        Self {
            Width: 0,
            Height: 0,
            RefreshRate: ::std::default::Default::default(),
            Format: ::std::default::Default::default(),
            ScanlineOrdering: ::std::default::Default::default(),
            Scaling: ::std::default::Default::default(),
            Stereo: ::std::default::Default::default(),
        }
    }
}
impl ::std::fmt::Debug for DXGI_MODE_DESC1 {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("DXGI_MODE_DESC1")
            .field("Width", &format_args!("{:?}", self.Width))
            .field("Height", &format_args!("{:?}", self.Height))
            .field("RefreshRate", &format_args!("{:?}", self.RefreshRate))
            .field("Format", &format_args!("{:?}", self.Format))
            .field(
                "ScanlineOrdering",
                &format_args!("{:?}", self.ScanlineOrdering),
            )
            .field("Scaling", &format_args!("{:?}", self.Scaling))
            .field("Stereo", &format_args!("{:?}", self.Stereo))
            .finish()
    }
}
impl ::std::cmp::PartialEq for DXGI_MODE_DESC1 {
    fn eq(&self, other: &Self) -> bool {
        self.Width == other.Width
            && self.Height == other.Height
            && self.RefreshRate == other.RefreshRate
            && self.Format == other.Format
            && self.ScanlineOrdering == other.ScanlineOrdering
            && self.Scaling == other.Scaling
            && self.Stereo == other.Stereo
    }
}
impl ::std::cmp::Eq for DXGI_MODE_DESC1 {}
unsafe impl ::windows::Abi for DXGI_MODE_DESC1 {
    type Abi = Self;
}
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
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
pub struct DXGI_OUTDUPL_DESC {
    pub ModeDesc: DXGI_MODE_DESC,
    pub Rotation: DXGI_MODE_ROTATION,
    pub DesktopImageInSystemMemory: super::SystemServices::BOOL,
}
impl DXGI_OUTDUPL_DESC {}
impl ::std::default::Default for DXGI_OUTDUPL_DESC {
    fn default() -> Self {
        Self {
            ModeDesc: ::std::default::Default::default(),
            Rotation: ::std::default::Default::default(),
            DesktopImageInSystemMemory: ::std::default::Default::default(),
        }
    }
}
impl ::std::fmt::Debug for DXGI_OUTDUPL_DESC {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("DXGI_OUTDUPL_DESC")
            .field("ModeDesc", &format_args!("{:?}", self.ModeDesc))
            .field("Rotation", &format_args!("{:?}", self.Rotation))
            .field(
                "DesktopImageInSystemMemory",
                &format_args!("{:?}", self.DesktopImageInSystemMemory),
            )
            .finish()
    }
}
impl ::std::cmp::PartialEq for DXGI_OUTDUPL_DESC {
    fn eq(&self, other: &Self) -> bool {
        self.ModeDesc == other.ModeDesc
            && self.Rotation == other.Rotation
            && self.DesktopImageInSystemMemory == other.DesktopImageInSystemMemory
    }
}
impl ::std::cmp::Eq for DXGI_OUTDUPL_DESC {}
unsafe impl ::windows::Abi for DXGI_OUTDUPL_DESC {
    type Abi = Self;
}
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
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
pub struct DXGI_OUTDUPL_POINTER_POSITION {
    pub Position: super::DisplayDevices::POINT,
    pub Visible: super::SystemServices::BOOL,
}
impl DXGI_OUTDUPL_POINTER_POSITION {}
impl ::std::default::Default for DXGI_OUTDUPL_POINTER_POSITION {
    fn default() -> Self {
        Self {
            Position: ::std::default::Default::default(),
            Visible: ::std::default::Default::default(),
        }
    }
}
impl ::std::fmt::Debug for DXGI_OUTDUPL_POINTER_POSITION {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("DXGI_OUTDUPL_POINTER_POSITION")
            .field("Position", &format_args!("{:?}", self.Position))
            .field("Visible", &format_args!("{:?}", self.Visible))
            .finish()
    }
}
impl ::std::cmp::PartialEq for DXGI_OUTDUPL_POINTER_POSITION {
    fn eq(&self, other: &Self) -> bool {
        self.Position == other.Position && self.Visible == other.Visible
    }
}
impl ::std::cmp::Eq for DXGI_OUTDUPL_POINTER_POSITION {}
unsafe impl ::windows::Abi for DXGI_OUTDUPL_POINTER_POSITION {
    type Abi = Self;
}
#[repr(C)]
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
pub struct DXGI_OUTDUPL_FRAME_INFO {
    pub LastPresentTime: i64,
    pub LastMouseUpdateTime: i64,
    pub AccumulatedFrames: u32,
    pub RectsCoalesced: super::SystemServices::BOOL,
    pub ProtectedContentMaskedOut: super::SystemServices::BOOL,
    pub PointerPosition: DXGI_OUTDUPL_POINTER_POSITION,
    pub TotalMetadataBufferSize: u32,
    pub PointerShapeBufferSize: u32,
}
impl DXGI_OUTDUPL_FRAME_INFO {}
impl ::std::default::Default for DXGI_OUTDUPL_FRAME_INFO {
    fn default() -> Self {
        Self {
            LastPresentTime: 0,
            LastMouseUpdateTime: 0,
            AccumulatedFrames: 0,
            RectsCoalesced: ::std::default::Default::default(),
            ProtectedContentMaskedOut: ::std::default::Default::default(),
            PointerPosition: ::std::default::Default::default(),
            TotalMetadataBufferSize: 0,
            PointerShapeBufferSize: 0,
        }
    }
}
impl ::std::fmt::Debug for DXGI_OUTDUPL_FRAME_INFO {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("DXGI_OUTDUPL_FRAME_INFO")
            .field(
                "LastPresentTime",
                &format_args!("{:?}", self.LastPresentTime),
            )
            .field(
                "LastMouseUpdateTime",
                &format_args!("{:?}", self.LastMouseUpdateTime),
            )
            .field(
                "AccumulatedFrames",
                &format_args!("{:?}", self.AccumulatedFrames),
            )
            .field("RectsCoalesced", &format_args!("{:?}", self.RectsCoalesced))
            .field(
                "ProtectedContentMaskedOut",
                &format_args!("{:?}", self.ProtectedContentMaskedOut),
            )
            .field(
                "PointerPosition",
                &format_args!("{:?}", self.PointerPosition),
            )
            .field(
                "TotalMetadataBufferSize",
                &format_args!("{:?}", self.TotalMetadataBufferSize),
            )
            .field(
                "PointerShapeBufferSize",
                &format_args!("{:?}", self.PointerShapeBufferSize),
            )
            .finish()
    }
}
impl ::std::cmp::PartialEq for DXGI_OUTDUPL_FRAME_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.LastPresentTime == other.LastPresentTime
            && self.LastMouseUpdateTime == other.LastMouseUpdateTime
            && self.AccumulatedFrames == other.AccumulatedFrames
            && self.RectsCoalesced == other.RectsCoalesced
            && self.ProtectedContentMaskedOut == other.ProtectedContentMaskedOut
            && self.PointerPosition == other.PointerPosition
            && self.TotalMetadataBufferSize == other.TotalMetadataBufferSize
            && self.PointerShapeBufferSize == other.PointerShapeBufferSize
    }
}
impl ::std::cmp::Eq for DXGI_OUTDUPL_FRAME_INFO {}
unsafe impl ::windows::Abi for DXGI_OUTDUPL_FRAME_INFO {
    type Abi = Self;
}
#[repr(C)]
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
pub struct DXGI_OUTDUPL_MOVE_RECT {
    pub SourcePoint: super::DisplayDevices::POINT,
    pub DestinationRect: super::DisplayDevices::RECT,
}
impl DXGI_OUTDUPL_MOVE_RECT {}
impl ::std::default::Default for DXGI_OUTDUPL_MOVE_RECT {
    fn default() -> Self {
        Self {
            SourcePoint: ::std::default::Default::default(),
            DestinationRect: ::std::default::Default::default(),
        }
    }
}
impl ::std::fmt::Debug for DXGI_OUTDUPL_MOVE_RECT {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("DXGI_OUTDUPL_MOVE_RECT")
            .field("SourcePoint", &format_args!("{:?}", self.SourcePoint))
            .field(
                "DestinationRect",
                &format_args!("{:?}", self.DestinationRect),
            )
            .finish()
    }
}
impl ::std::cmp::PartialEq for DXGI_OUTDUPL_MOVE_RECT {
    fn eq(&self, other: &Self) -> bool {
        self.SourcePoint == other.SourcePoint && self.DestinationRect == other.DestinationRect
    }
}
impl ::std::cmp::Eq for DXGI_OUTDUPL_MOVE_RECT {}
unsafe impl ::windows::Abi for DXGI_OUTDUPL_MOVE_RECT {
    type Abi = Self;
}
#[repr(C)]
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
pub struct DXGI_OUTDUPL_POINTER_SHAPE_INFO {
    pub Type: u32,
    pub Width: u32,
    pub Height: u32,
    pub Pitch: u32,
    pub HotSpot: super::DisplayDevices::POINT,
}
impl DXGI_OUTDUPL_POINTER_SHAPE_INFO {}
impl ::std::default::Default for DXGI_OUTDUPL_POINTER_SHAPE_INFO {
    fn default() -> Self {
        Self {
            Type: 0,
            Width: 0,
            Height: 0,
            Pitch: 0,
            HotSpot: ::std::default::Default::default(),
        }
    }
}
impl ::std::fmt::Debug for DXGI_OUTDUPL_POINTER_SHAPE_INFO {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("DXGI_OUTDUPL_POINTER_SHAPE_INFO")
            .field("Type", &format_args!("{:?}", self.Type))
            .field("Width", &format_args!("{:?}", self.Width))
            .field("Height", &format_args!("{:?}", self.Height))
            .field("Pitch", &format_args!("{:?}", self.Pitch))
            .field("HotSpot", &format_args!("{:?}", self.HotSpot))
            .finish()
    }
}
impl ::std::cmp::PartialEq for DXGI_OUTDUPL_POINTER_SHAPE_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.Type == other.Type
            && self.Width == other.Width
            && self.Height == other.Height
            && self.Pitch == other.Pitch
            && self.HotSpot == other.HotSpot
    }
}
impl ::std::cmp::Eq for DXGI_OUTDUPL_POINTER_SHAPE_INFO {}
unsafe impl ::windows::Abi for DXGI_OUTDUPL_POINTER_SHAPE_INFO {
    type Abi = Self;
}
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
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
pub struct DXGI_OUTPUT_DESC {
    pub DeviceName: [u16; 32],
    pub DesktopCoordinates: super::DisplayDevices::RECT,
    pub AttachedToDesktop: super::SystemServices::BOOL,
    pub Rotation: DXGI_MODE_ROTATION,
    pub Monitor: super::Gdi::HMONITOR,
}
impl DXGI_OUTPUT_DESC {}
impl ::std::default::Default for DXGI_OUTPUT_DESC {
    fn default() -> Self {
        Self {
            DeviceName: [0; 32],
            DesktopCoordinates: ::std::default::Default::default(),
            AttachedToDesktop: ::std::default::Default::default(),
            Rotation: ::std::default::Default::default(),
            Monitor: ::std::default::Default::default(),
        }
    }
}
impl ::std::fmt::Debug for DXGI_OUTPUT_DESC {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("DXGI_OUTPUT_DESC")
            .field("DeviceName", &format_args!("{:?}", self.DeviceName))
            .field(
                "DesktopCoordinates",
                &format_args!("{:?}", self.DesktopCoordinates),
            )
            .field(
                "AttachedToDesktop",
                &format_args!("{:?}", self.AttachedToDesktop),
            )
            .field("Rotation", &format_args!("{:?}", self.Rotation))
            .field("Monitor", &format_args!("{:?}", self.Monitor))
            .finish()
    }
}
impl ::std::cmp::PartialEq for DXGI_OUTPUT_DESC {
    fn eq(&self, other: &Self) -> bool {
        self.DeviceName == other.DeviceName
            && self.DesktopCoordinates == other.DesktopCoordinates
            && self.AttachedToDesktop == other.AttachedToDesktop
            && self.Rotation == other.Rotation
            && self.Monitor == other.Monitor
    }
}
impl ::std::cmp::Eq for DXGI_OUTPUT_DESC {}
unsafe impl ::windows::Abi for DXGI_OUTPUT_DESC {
    type Abi = Self;
}
#[repr(C)]
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
pub struct DXGI_OUTPUT_DESC1 {
    pub DeviceName: [u16; 32],
    pub DesktopCoordinates: super::DisplayDevices::RECT,
    pub AttachedToDesktop: super::SystemServices::BOOL,
    pub Rotation: DXGI_MODE_ROTATION,
    pub Monitor: super::Gdi::HMONITOR,
    pub BitsPerColor: u32,
    pub ColorSpace: DXGI_COLOR_SPACE_TYPE,
    pub RedPrimary: [f32; 2],
    pub GreenPrimary: [f32; 2],
    pub BluePrimary: [f32; 2],
    pub WhitePoint: [f32; 2],
    pub MinLuminance: f32,
    pub MaxLuminance: f32,
    pub MaxFullFrameLuminance: f32,
}
impl DXGI_OUTPUT_DESC1 {}
impl ::std::default::Default for DXGI_OUTPUT_DESC1 {
    fn default() -> Self {
        Self {
            DeviceName: [0; 32],
            DesktopCoordinates: ::std::default::Default::default(),
            AttachedToDesktop: ::std::default::Default::default(),
            Rotation: ::std::default::Default::default(),
            Monitor: ::std::default::Default::default(),
            BitsPerColor: 0,
            ColorSpace: ::std::default::Default::default(),
            RedPrimary: [0.0; 2],
            GreenPrimary: [0.0; 2],
            BluePrimary: [0.0; 2],
            WhitePoint: [0.0; 2],
            MinLuminance: 0.0,
            MaxLuminance: 0.0,
            MaxFullFrameLuminance: 0.0,
        }
    }
}
impl ::std::fmt::Debug for DXGI_OUTPUT_DESC1 {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("DXGI_OUTPUT_DESC1")
            .field("DeviceName", &format_args!("{:?}", self.DeviceName))
            .field(
                "DesktopCoordinates",
                &format_args!("{:?}", self.DesktopCoordinates),
            )
            .field(
                "AttachedToDesktop",
                &format_args!("{:?}", self.AttachedToDesktop),
            )
            .field("Rotation", &format_args!("{:?}", self.Rotation))
            .field("Monitor", &format_args!("{:?}", self.Monitor))
            .field("BitsPerColor", &format_args!("{:?}", self.BitsPerColor))
            .field("ColorSpace", &format_args!("{:?}", self.ColorSpace))
            .field("RedPrimary", &format_args!("{:?}", self.RedPrimary))
            .field("GreenPrimary", &format_args!("{:?}", self.GreenPrimary))
            .field("BluePrimary", &format_args!("{:?}", self.BluePrimary))
            .field("WhitePoint", &format_args!("{:?}", self.WhitePoint))
            .field("MinLuminance", &format_args!("{:?}", self.MinLuminance))
            .field("MaxLuminance", &format_args!("{:?}", self.MaxLuminance))
            .field(
                "MaxFullFrameLuminance",
                &format_args!("{:?}", self.MaxFullFrameLuminance),
            )
            .finish()
    }
}
impl ::std::cmp::PartialEq for DXGI_OUTPUT_DESC1 {
    fn eq(&self, other: &Self) -> bool {
        self.DeviceName == other.DeviceName
            && self.DesktopCoordinates == other.DesktopCoordinates
            && self.AttachedToDesktop == other.AttachedToDesktop
            && self.Rotation == other.Rotation
            && self.Monitor == other.Monitor
            && self.BitsPerColor == other.BitsPerColor
            && self.ColorSpace == other.ColorSpace
            && self.RedPrimary == other.RedPrimary
            && self.GreenPrimary == other.GreenPrimary
            && self.BluePrimary == other.BluePrimary
            && self.WhitePoint == other.WhitePoint
            && self.MinLuminance == other.MinLuminance
            && self.MaxLuminance == other.MaxLuminance
            && self.MaxFullFrameLuminance == other.MaxFullFrameLuminance
    }
}
impl ::std::cmp::Eq for DXGI_OUTPUT_DESC1 {}
unsafe impl ::windows::Abi for DXGI_OUTPUT_DESC1 {
    type Abi = Self;
}
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
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
pub struct DXGI_PRESENT_PARAMETERS {
    pub DirtyRectsCount: u32,
    pub pDirtyRects: *mut super::DisplayDevices::RECT,
    pub pScrollRect: *mut super::DisplayDevices::RECT,
    pub pScrollOffset: *mut super::DisplayDevices::POINT,
}
impl DXGI_PRESENT_PARAMETERS {}
impl ::std::default::Default for DXGI_PRESENT_PARAMETERS {
    fn default() -> Self {
        Self {
            DirtyRectsCount: 0,
            pDirtyRects: ::std::ptr::null_mut(),
            pScrollRect: ::std::ptr::null_mut(),
            pScrollOffset: ::std::ptr::null_mut(),
        }
    }
}
impl ::std::fmt::Debug for DXGI_PRESENT_PARAMETERS {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("DXGI_PRESENT_PARAMETERS")
            .field(
                "DirtyRectsCount",
                &format_args!("{:?}", self.DirtyRectsCount),
            )
            .field("pDirtyRects", &format_args!("{:?}", self.pDirtyRects))
            .field("pScrollRect", &format_args!("{:?}", self.pScrollRect))
            .field("pScrollOffset", &format_args!("{:?}", self.pScrollOffset))
            .finish()
    }
}
impl ::std::cmp::PartialEq for DXGI_PRESENT_PARAMETERS {
    fn eq(&self, other: &Self) -> bool {
        self.DirtyRectsCount == other.DirtyRectsCount
            && self.pDirtyRects == other.pDirtyRects
            && self.pScrollRect == other.pScrollRect
            && self.pScrollOffset == other.pScrollOffset
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
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
pub struct DXGI_QUERY_VIDEO_MEMORY_INFO {
    pub Budget: u64,
    pub CurrentUsage: u64,
    pub AvailableForReservation: u64,
    pub CurrentReservation: u64,
}
impl DXGI_QUERY_VIDEO_MEMORY_INFO {}
impl ::std::default::Default for DXGI_QUERY_VIDEO_MEMORY_INFO {
    fn default() -> Self {
        Self {
            Budget: 0,
            CurrentUsage: 0,
            AvailableForReservation: 0,
            CurrentReservation: 0,
        }
    }
}
impl ::std::fmt::Debug for DXGI_QUERY_VIDEO_MEMORY_INFO {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("DXGI_QUERY_VIDEO_MEMORY_INFO")
            .field("Budget", &format_args!("{:?}", self.Budget))
            .field("CurrentUsage", &format_args!("{:?}", self.CurrentUsage))
            .field(
                "AvailableForReservation",
                &format_args!("{:?}", self.AvailableForReservation),
            )
            .field(
                "CurrentReservation",
                &format_args!("{:?}", self.CurrentReservation),
            )
            .finish()
    }
}
impl ::std::cmp::PartialEq for DXGI_QUERY_VIDEO_MEMORY_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.Budget == other.Budget
            && self.CurrentUsage == other.CurrentUsage
            && self.AvailableForReservation == other.AvailableForReservation
            && self.CurrentReservation == other.CurrentReservation
    }
}
impl ::std::cmp::Eq for DXGI_QUERY_VIDEO_MEMORY_INFO {}
unsafe impl ::windows::Abi for DXGI_QUERY_VIDEO_MEMORY_INFO {
    type Abi = Self;
}
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
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
pub struct DXGI_SHARED_RESOURCE {
    pub Handle: super::SystemServices::HANDLE,
}
impl DXGI_SHARED_RESOURCE {}
impl ::std::default::Default for DXGI_SHARED_RESOURCE {
    fn default() -> Self {
        Self {
            Handle: ::std::default::Default::default(),
        }
    }
}
impl ::std::fmt::Debug for DXGI_SHARED_RESOURCE {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("DXGI_SHARED_RESOURCE")
            .field("Handle", &format_args!("{:?}", self.Handle))
            .finish()
    }
}
impl ::std::cmp::PartialEq for DXGI_SHARED_RESOURCE {
    fn eq(&self, other: &Self) -> bool {
        self.Handle == other.Handle
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
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
pub struct DXGI_SURFACE_DESC {
    pub Width: u32,
    pub Height: u32,
    pub Format: DXGI_FORMAT,
    pub SampleDesc: DXGI_SAMPLE_DESC,
}
impl DXGI_SURFACE_DESC {}
impl ::std::default::Default for DXGI_SURFACE_DESC {
    fn default() -> Self {
        Self {
            Width: 0,
            Height: 0,
            Format: ::std::default::Default::default(),
            SampleDesc: ::std::default::Default::default(),
        }
    }
}
impl ::std::fmt::Debug for DXGI_SURFACE_DESC {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("DXGI_SURFACE_DESC")
            .field("Width", &format_args!("{:?}", self.Width))
            .field("Height", &format_args!("{:?}", self.Height))
            .field("Format", &format_args!("{:?}", self.Format))
            .field("SampleDesc", &format_args!("{:?}", self.SampleDesc))
            .finish()
    }
}
impl ::std::cmp::PartialEq for DXGI_SURFACE_DESC {
    fn eq(&self, other: &Self) -> bool {
        self.Width == other.Width
            && self.Height == other.Height
            && self.Format == other.Format
            && self.SampleDesc == other.SampleDesc
    }
}
impl ::std::cmp::Eq for DXGI_SURFACE_DESC {}
unsafe impl ::windows::Abi for DXGI_SURFACE_DESC {
    type Abi = Self;
}
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
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
pub struct DXGI_SWAP_CHAIN_DESC {
    pub BufferDesc: DXGI_MODE_DESC,
    pub SampleDesc: DXGI_SAMPLE_DESC,
    pub BufferUsage: u32,
    pub BufferCount: u32,
    pub OutputWindow: super::WindowsAndMessaging::HWND,
    pub Windowed: super::SystemServices::BOOL,
    pub SwapEffect: DXGI_SWAP_EFFECT,
    pub Flags: u32,
}
impl DXGI_SWAP_CHAIN_DESC {}
impl ::std::default::Default for DXGI_SWAP_CHAIN_DESC {
    fn default() -> Self {
        Self {
            BufferDesc: ::std::default::Default::default(),
            SampleDesc: ::std::default::Default::default(),
            BufferUsage: 0,
            BufferCount: 0,
            OutputWindow: ::std::default::Default::default(),
            Windowed: ::std::default::Default::default(),
            SwapEffect: ::std::default::Default::default(),
            Flags: 0,
        }
    }
}
impl ::std::fmt::Debug for DXGI_SWAP_CHAIN_DESC {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("DXGI_SWAP_CHAIN_DESC")
            .field("BufferDesc", &format_args!("{:?}", self.BufferDesc))
            .field("SampleDesc", &format_args!("{:?}", self.SampleDesc))
            .field("BufferUsage", &format_args!("{:?}", self.BufferUsage))
            .field("BufferCount", &format_args!("{:?}", self.BufferCount))
            .field("OutputWindow", &format_args!("{:?}", self.OutputWindow))
            .field("Windowed", &format_args!("{:?}", self.Windowed))
            .field("SwapEffect", &format_args!("{:?}", self.SwapEffect))
            .field("Flags", &format_args!("{:?}", self.Flags))
            .finish()
    }
}
impl ::std::cmp::PartialEq for DXGI_SWAP_CHAIN_DESC {
    fn eq(&self, other: &Self) -> bool {
        self.BufferDesc == other.BufferDesc
            && self.SampleDesc == other.SampleDesc
            && self.BufferUsage == other.BufferUsage
            && self.BufferCount == other.BufferCount
            && self.OutputWindow == other.OutputWindow
            && self.Windowed == other.Windowed
            && self.SwapEffect == other.SwapEffect
            && self.Flags == other.Flags
    }
}
impl ::std::cmp::Eq for DXGI_SWAP_CHAIN_DESC {}
unsafe impl ::windows::Abi for DXGI_SWAP_CHAIN_DESC {
    type Abi = Self;
}
#[repr(C)]
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
pub struct DXGI_SWAP_CHAIN_DESC1 {
    pub Width: u32,
    pub Height: u32,
    pub Format: DXGI_FORMAT,
    pub Stereo: super::SystemServices::BOOL,
    pub SampleDesc: DXGI_SAMPLE_DESC,
    pub BufferUsage: u32,
    pub BufferCount: u32,
    pub Scaling: DXGI_SCALING,
    pub SwapEffect: DXGI_SWAP_EFFECT,
    pub AlphaMode: DXGI_ALPHA_MODE,
    pub Flags: u32,
}
impl DXGI_SWAP_CHAIN_DESC1 {}
impl ::std::default::Default for DXGI_SWAP_CHAIN_DESC1 {
    fn default() -> Self {
        Self {
            Width: 0,
            Height: 0,
            Format: ::std::default::Default::default(),
            Stereo: ::std::default::Default::default(),
            SampleDesc: ::std::default::Default::default(),
            BufferUsage: 0,
            BufferCount: 0,
            Scaling: ::std::default::Default::default(),
            SwapEffect: ::std::default::Default::default(),
            AlphaMode: ::std::default::Default::default(),
            Flags: 0,
        }
    }
}
impl ::std::fmt::Debug for DXGI_SWAP_CHAIN_DESC1 {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("DXGI_SWAP_CHAIN_DESC1")
            .field("Width", &format_args!("{:?}", self.Width))
            .field("Height", &format_args!("{:?}", self.Height))
            .field("Format", &format_args!("{:?}", self.Format))
            .field("Stereo", &format_args!("{:?}", self.Stereo))
            .field("SampleDesc", &format_args!("{:?}", self.SampleDesc))
            .field("BufferUsage", &format_args!("{:?}", self.BufferUsage))
            .field("BufferCount", &format_args!("{:?}", self.BufferCount))
            .field("Scaling", &format_args!("{:?}", self.Scaling))
            .field("SwapEffect", &format_args!("{:?}", self.SwapEffect))
            .field("AlphaMode", &format_args!("{:?}", self.AlphaMode))
            .field("Flags", &format_args!("{:?}", self.Flags))
            .finish()
    }
}
impl ::std::cmp::PartialEq for DXGI_SWAP_CHAIN_DESC1 {
    fn eq(&self, other: &Self) -> bool {
        self.Width == other.Width
            && self.Height == other.Height
            && self.Format == other.Format
            && self.Stereo == other.Stereo
            && self.SampleDesc == other.SampleDesc
            && self.BufferUsage == other.BufferUsage
            && self.BufferCount == other.BufferCount
            && self.Scaling == other.Scaling
            && self.SwapEffect == other.SwapEffect
            && self.AlphaMode == other.AlphaMode
            && self.Flags == other.Flags
    }
}
impl ::std::cmp::Eq for DXGI_SWAP_CHAIN_DESC1 {}
unsafe impl ::windows::Abi for DXGI_SWAP_CHAIN_DESC1 {
    type Abi = Self;
}
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
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
pub struct DXGI_SWAP_CHAIN_FULLSCREEN_DESC {
    pub RefreshRate: DXGI_RATIONAL,
    pub ScanlineOrdering: DXGI_MODE_SCANLINE_ORDER,
    pub Scaling: DXGI_MODE_SCALING,
    pub Windowed: super::SystemServices::BOOL,
}
impl DXGI_SWAP_CHAIN_FULLSCREEN_DESC {}
impl ::std::default::Default for DXGI_SWAP_CHAIN_FULLSCREEN_DESC {
    fn default() -> Self {
        Self {
            RefreshRate: ::std::default::Default::default(),
            ScanlineOrdering: ::std::default::Default::default(),
            Scaling: ::std::default::Default::default(),
            Windowed: ::std::default::Default::default(),
        }
    }
}
impl ::std::fmt::Debug for DXGI_SWAP_CHAIN_FULLSCREEN_DESC {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("DXGI_SWAP_CHAIN_FULLSCREEN_DESC")
            .field("RefreshRate", &format_args!("{:?}", self.RefreshRate))
            .field(
                "ScanlineOrdering",
                &format_args!("{:?}", self.ScanlineOrdering),
            )
            .field("Scaling", &format_args!("{:?}", self.Scaling))
            .field("Windowed", &format_args!("{:?}", self.Windowed))
            .finish()
    }
}
impl ::std::cmp::PartialEq for DXGI_SWAP_CHAIN_FULLSCREEN_DESC {
    fn eq(&self, other: &Self) -> bool {
        self.RefreshRate == other.RefreshRate
            && self.ScanlineOrdering == other.ScanlineOrdering
            && self.Scaling == other.Scaling
            && self.Windowed == other.Windowed
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
impl IDXGIObject {
    pub unsafe fn SetPrivateData(
        &self,
        name: *const ::windows::Guid,
        datasize: u32,
        pdata: *const ::std::ffi::c_void,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).3)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(name),
            ::std::mem::transmute(datasize),
            ::std::mem::transmute(pdata),
        )
    }
    pub unsafe fn SetPrivateDataInterface<
        'a,
        T1__: ::windows::IntoParam<'a, ::windows::IUnknown>,
    >(
        &self,
        name: *const ::windows::Guid,
        punknown: T1__,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).4)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(name),
            punknown.into_param().abi(),
        )
    }
    pub unsafe fn GetPrivateData(
        &self,
        name: *const ::windows::Guid,
        pdatasize: *mut u32,
        pdata: *mut ::std::ffi::c_void,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).5)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(name),
            ::std::mem::transmute(pdatasize),
            ::std::mem::transmute(pdata),
        )
    }
    pub unsafe fn GetParent(
        &self,
        riid: *const ::windows::Guid,
        ppparent: *mut *mut ::std::ffi::c_void,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).6)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(riid),
            ::std::mem::transmute(ppparent),
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
        datasize: u32,
        pdata: *const ::std::ffi::c_void,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        name: *const ::windows::Guid,
        punknown: ::windows::RawPtr,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        name: *const ::windows::Guid,
        pdatasize: *mut u32,
        pdata: *mut ::std::ffi::c_void,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        riid: *const ::windows::Guid,
        ppparent: *mut *mut ::std::ffi::c_void,
    ) -> ::windows::ErrorCode,
);
#[repr(transparent)]
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
impl IDXGIAdapter {
    pub unsafe fn SetPrivateData(
        &self,
        name: *const ::windows::Guid,
        datasize: u32,
        pdata: *const ::std::ffi::c_void,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).3)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(name),
            ::std::mem::transmute(datasize),
            ::std::mem::transmute(pdata),
        )
    }
    pub unsafe fn SetPrivateDataInterface<
        'a,
        T1__: ::windows::IntoParam<'a, ::windows::IUnknown>,
    >(
        &self,
        name: *const ::windows::Guid,
        punknown: T1__,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).4)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(name),
            punknown.into_param().abi(),
        )
    }
    pub unsafe fn GetPrivateData(
        &self,
        name: *const ::windows::Guid,
        pdatasize: *mut u32,
        pdata: *mut ::std::ffi::c_void,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).5)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(name),
            ::std::mem::transmute(pdatasize),
            ::std::mem::transmute(pdata),
        )
    }
    pub unsafe fn GetParent(
        &self,
        riid: *const ::windows::Guid,
        ppparent: *mut *mut ::std::ffi::c_void,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).6)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(riid),
            ::std::mem::transmute(ppparent),
        )
    }
    pub unsafe fn EnumOutputs(
        &self,
        output: u32,
        ppoutput: *mut ::std::option::Option<IDXGIOutput>,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).7)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(output),
            ::std::mem::transmute(ppoutput),
        )
    }
    pub unsafe fn GetDesc(&self, pdesc: *mut DXGI_ADAPTER_DESC) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).8)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(pdesc),
        )
    }
    pub unsafe fn CheckInterfaceSupport(
        &self,
        interfacename: *const ::windows::Guid,
        pumdversion: *mut i64,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).9)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(interfacename),
            ::std::mem::transmute(pumdversion),
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
        datasize: u32,
        pdata: *const ::std::ffi::c_void,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        name: *const ::windows::Guid,
        punknown: ::windows::RawPtr,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        name: *const ::windows::Guid,
        pdatasize: *mut u32,
        pdata: *mut ::std::ffi::c_void,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        riid: *const ::windows::Guid,
        ppparent: *mut *mut ::std::ffi::c_void,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        output: u32,
        ppoutput: *mut ::windows::RawPtr,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        pdesc: *mut DXGI_ADAPTER_DESC,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        interfacename: *const ::windows::Guid,
        pumdversion: *mut i64,
    ) -> ::windows::ErrorCode,
);
#[repr(transparent)]
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
impl IDXGIAdapter1 {
    pub unsafe fn SetPrivateData(
        &self,
        name: *const ::windows::Guid,
        datasize: u32,
        pdata: *const ::std::ffi::c_void,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).3)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(name),
            ::std::mem::transmute(datasize),
            ::std::mem::transmute(pdata),
        )
    }
    pub unsafe fn SetPrivateDataInterface<
        'a,
        T1__: ::windows::IntoParam<'a, ::windows::IUnknown>,
    >(
        &self,
        name: *const ::windows::Guid,
        punknown: T1__,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).4)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(name),
            punknown.into_param().abi(),
        )
    }
    pub unsafe fn GetPrivateData(
        &self,
        name: *const ::windows::Guid,
        pdatasize: *mut u32,
        pdata: *mut ::std::ffi::c_void,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).5)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(name),
            ::std::mem::transmute(pdatasize),
            ::std::mem::transmute(pdata),
        )
    }
    pub unsafe fn GetParent(
        &self,
        riid: *const ::windows::Guid,
        ppparent: *mut *mut ::std::ffi::c_void,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).6)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(riid),
            ::std::mem::transmute(ppparent),
        )
    }
    pub unsafe fn EnumOutputs(
        &self,
        output: u32,
        ppoutput: *mut ::std::option::Option<IDXGIOutput>,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).7)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(output),
            ::std::mem::transmute(ppoutput),
        )
    }
    pub unsafe fn GetDesc(&self, pdesc: *mut DXGI_ADAPTER_DESC) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).8)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(pdesc),
        )
    }
    pub unsafe fn CheckInterfaceSupport(
        &self,
        interfacename: *const ::windows::Guid,
        pumdversion: *mut i64,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).9)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(interfacename),
            ::std::mem::transmute(pumdversion),
        )
    }
    pub unsafe fn GetDesc1(&self, pdesc: *mut DXGI_ADAPTER_DESC1) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).10)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(pdesc),
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
        datasize: u32,
        pdata: *const ::std::ffi::c_void,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        name: *const ::windows::Guid,
        punknown: ::windows::RawPtr,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        name: *const ::windows::Guid,
        pdatasize: *mut u32,
        pdata: *mut ::std::ffi::c_void,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        riid: *const ::windows::Guid,
        ppparent: *mut *mut ::std::ffi::c_void,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        output: u32,
        ppoutput: *mut ::windows::RawPtr,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        pdesc: *mut DXGI_ADAPTER_DESC,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        interfacename: *const ::windows::Guid,
        pumdversion: *mut i64,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        pdesc: *mut DXGI_ADAPTER_DESC1,
    ) -> ::windows::ErrorCode,
);
#[repr(transparent)]
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
impl IDXGIAdapter2 {
    pub unsafe fn SetPrivateData(
        &self,
        name: *const ::windows::Guid,
        datasize: u32,
        pdata: *const ::std::ffi::c_void,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).3)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(name),
            ::std::mem::transmute(datasize),
            ::std::mem::transmute(pdata),
        )
    }
    pub unsafe fn SetPrivateDataInterface<
        'a,
        T1__: ::windows::IntoParam<'a, ::windows::IUnknown>,
    >(
        &self,
        name: *const ::windows::Guid,
        punknown: T1__,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).4)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(name),
            punknown.into_param().abi(),
        )
    }
    pub unsafe fn GetPrivateData(
        &self,
        name: *const ::windows::Guid,
        pdatasize: *mut u32,
        pdata: *mut ::std::ffi::c_void,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).5)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(name),
            ::std::mem::transmute(pdatasize),
            ::std::mem::transmute(pdata),
        )
    }
    pub unsafe fn GetParent(
        &self,
        riid: *const ::windows::Guid,
        ppparent: *mut *mut ::std::ffi::c_void,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).6)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(riid),
            ::std::mem::transmute(ppparent),
        )
    }
    pub unsafe fn EnumOutputs(
        &self,
        output: u32,
        ppoutput: *mut ::std::option::Option<IDXGIOutput>,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).7)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(output),
            ::std::mem::transmute(ppoutput),
        )
    }
    pub unsafe fn GetDesc(&self, pdesc: *mut DXGI_ADAPTER_DESC) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).8)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(pdesc),
        )
    }
    pub unsafe fn CheckInterfaceSupport(
        &self,
        interfacename: *const ::windows::Guid,
        pumdversion: *mut i64,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).9)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(interfacename),
            ::std::mem::transmute(pumdversion),
        )
    }
    pub unsafe fn GetDesc1(&self, pdesc: *mut DXGI_ADAPTER_DESC1) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).10)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(pdesc),
        )
    }
    pub unsafe fn GetDesc2(&self, pdesc: *mut DXGI_ADAPTER_DESC2) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).11)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(pdesc),
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
        datasize: u32,
        pdata: *const ::std::ffi::c_void,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        name: *const ::windows::Guid,
        punknown: ::windows::RawPtr,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        name: *const ::windows::Guid,
        pdatasize: *mut u32,
        pdata: *mut ::std::ffi::c_void,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        riid: *const ::windows::Guid,
        ppparent: *mut *mut ::std::ffi::c_void,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        output: u32,
        ppoutput: *mut ::windows::RawPtr,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        pdesc: *mut DXGI_ADAPTER_DESC,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        interfacename: *const ::windows::Guid,
        pumdversion: *mut i64,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        pdesc: *mut DXGI_ADAPTER_DESC1,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        pdesc: *mut DXGI_ADAPTER_DESC2,
    ) -> ::windows::ErrorCode,
);
#[repr(transparent)]
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
impl IDXGIAdapter3 {
    pub unsafe fn SetPrivateData(
        &self,
        name: *const ::windows::Guid,
        datasize: u32,
        pdata: *const ::std::ffi::c_void,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).3)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(name),
            ::std::mem::transmute(datasize),
            ::std::mem::transmute(pdata),
        )
    }
    pub unsafe fn SetPrivateDataInterface<
        'a,
        T1__: ::windows::IntoParam<'a, ::windows::IUnknown>,
    >(
        &self,
        name: *const ::windows::Guid,
        punknown: T1__,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).4)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(name),
            punknown.into_param().abi(),
        )
    }
    pub unsafe fn GetPrivateData(
        &self,
        name: *const ::windows::Guid,
        pdatasize: *mut u32,
        pdata: *mut ::std::ffi::c_void,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).5)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(name),
            ::std::mem::transmute(pdatasize),
            ::std::mem::transmute(pdata),
        )
    }
    pub unsafe fn GetParent(
        &self,
        riid: *const ::windows::Guid,
        ppparent: *mut *mut ::std::ffi::c_void,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).6)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(riid),
            ::std::mem::transmute(ppparent),
        )
    }
    pub unsafe fn EnumOutputs(
        &self,
        output: u32,
        ppoutput: *mut ::std::option::Option<IDXGIOutput>,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).7)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(output),
            ::std::mem::transmute(ppoutput),
        )
    }
    pub unsafe fn GetDesc(&self, pdesc: *mut DXGI_ADAPTER_DESC) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).8)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(pdesc),
        )
    }
    pub unsafe fn CheckInterfaceSupport(
        &self,
        interfacename: *const ::windows::Guid,
        pumdversion: *mut i64,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).9)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(interfacename),
            ::std::mem::transmute(pumdversion),
        )
    }
    pub unsafe fn GetDesc1(&self, pdesc: *mut DXGI_ADAPTER_DESC1) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).10)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(pdesc),
        )
    }
    pub unsafe fn GetDesc2(&self, pdesc: *mut DXGI_ADAPTER_DESC2) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).11)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(pdesc),
        )
    }
    pub unsafe fn RegisterHardwareContentProtectionTeardownStatusEvent<
        'a,
        T0__: ::windows::IntoParam<'a, super::SystemServices::HANDLE>,
    >(
        &self,
        hevent: T0__,
        pdwcookie: *mut u32,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).12)(
            ::windows::Abi::abi(self),
            hevent.into_param().abi(),
            ::std::mem::transmute(pdwcookie),
        )
    }
    pub unsafe fn UnregisterHardwareContentProtectionTeardownStatus(&self, dwcookie: u32) {
        (::windows::Interface::vtable(self).13)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(dwcookie),
        )
    }
    pub unsafe fn QueryVideoMemoryInfo(
        &self,
        nodeindex: u32,
        memorysegmentgroup: DXGI_MEMORY_SEGMENT_GROUP,
        pvideomemoryinfo: *mut DXGI_QUERY_VIDEO_MEMORY_INFO,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).14)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(nodeindex),
            ::std::mem::transmute(memorysegmentgroup),
            ::std::mem::transmute(pvideomemoryinfo),
        )
    }
    pub unsafe fn SetVideoMemoryReservation(
        &self,
        nodeindex: u32,
        memorysegmentgroup: DXGI_MEMORY_SEGMENT_GROUP,
        reservation: u64,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).15)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(nodeindex),
            ::std::mem::transmute(memorysegmentgroup),
            ::std::mem::transmute(reservation),
        )
    }
    pub unsafe fn RegisterVideoMemoryBudgetChangeNotificationEvent<
        'a,
        T0__: ::windows::IntoParam<'a, super::SystemServices::HANDLE>,
    >(
        &self,
        hevent: T0__,
        pdwcookie: *mut u32,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).16)(
            ::windows::Abi::abi(self),
            hevent.into_param().abi(),
            ::std::mem::transmute(pdwcookie),
        )
    }
    pub unsafe fn UnregisterVideoMemoryBudgetChangeNotification(&self, dwcookie: u32) {
        (::windows::Interface::vtable(self).17)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(dwcookie),
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
        datasize: u32,
        pdata: *const ::std::ffi::c_void,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        name: *const ::windows::Guid,
        punknown: ::windows::RawPtr,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        name: *const ::windows::Guid,
        pdatasize: *mut u32,
        pdata: *mut ::std::ffi::c_void,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        riid: *const ::windows::Guid,
        ppparent: *mut *mut ::std::ffi::c_void,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        output: u32,
        ppoutput: *mut ::windows::RawPtr,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        pdesc: *mut DXGI_ADAPTER_DESC,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        interfacename: *const ::windows::Guid,
        pumdversion: *mut i64,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        pdesc: *mut DXGI_ADAPTER_DESC1,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        pdesc: *mut DXGI_ADAPTER_DESC2,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        hevent: super::SystemServices::HANDLE,
        pdwcookie: *mut u32,
    ) -> ::windows::ErrorCode,
    pub unsafe extern "system" fn(this: ::windows::RawPtr, dwcookie: u32),
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        nodeindex: u32,
        memorysegmentgroup: DXGI_MEMORY_SEGMENT_GROUP,
        pvideomemoryinfo: *mut DXGI_QUERY_VIDEO_MEMORY_INFO,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        nodeindex: u32,
        memorysegmentgroup: DXGI_MEMORY_SEGMENT_GROUP,
        reservation: u64,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        hevent: super::SystemServices::HANDLE,
        pdwcookie: *mut u32,
    ) -> ::windows::ErrorCode,
    pub unsafe extern "system" fn(this: ::windows::RawPtr, dwcookie: u32),
);
#[repr(transparent)]
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
impl IDXGIAdapter4 {
    pub unsafe fn SetPrivateData(
        &self,
        name: *const ::windows::Guid,
        datasize: u32,
        pdata: *const ::std::ffi::c_void,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).3)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(name),
            ::std::mem::transmute(datasize),
            ::std::mem::transmute(pdata),
        )
    }
    pub unsafe fn SetPrivateDataInterface<
        'a,
        T1__: ::windows::IntoParam<'a, ::windows::IUnknown>,
    >(
        &self,
        name: *const ::windows::Guid,
        punknown: T1__,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).4)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(name),
            punknown.into_param().abi(),
        )
    }
    pub unsafe fn GetPrivateData(
        &self,
        name: *const ::windows::Guid,
        pdatasize: *mut u32,
        pdata: *mut ::std::ffi::c_void,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).5)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(name),
            ::std::mem::transmute(pdatasize),
            ::std::mem::transmute(pdata),
        )
    }
    pub unsafe fn GetParent(
        &self,
        riid: *const ::windows::Guid,
        ppparent: *mut *mut ::std::ffi::c_void,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).6)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(riid),
            ::std::mem::transmute(ppparent),
        )
    }
    pub unsafe fn EnumOutputs(
        &self,
        output: u32,
        ppoutput: *mut ::std::option::Option<IDXGIOutput>,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).7)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(output),
            ::std::mem::transmute(ppoutput),
        )
    }
    pub unsafe fn GetDesc(&self, pdesc: *mut DXGI_ADAPTER_DESC) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).8)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(pdesc),
        )
    }
    pub unsafe fn CheckInterfaceSupport(
        &self,
        interfacename: *const ::windows::Guid,
        pumdversion: *mut i64,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).9)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(interfacename),
            ::std::mem::transmute(pumdversion),
        )
    }
    pub unsafe fn GetDesc1(&self, pdesc: *mut DXGI_ADAPTER_DESC1) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).10)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(pdesc),
        )
    }
    pub unsafe fn GetDesc2(&self, pdesc: *mut DXGI_ADAPTER_DESC2) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).11)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(pdesc),
        )
    }
    pub unsafe fn RegisterHardwareContentProtectionTeardownStatusEvent<
        'a,
        T0__: ::windows::IntoParam<'a, super::SystemServices::HANDLE>,
    >(
        &self,
        hevent: T0__,
        pdwcookie: *mut u32,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).12)(
            ::windows::Abi::abi(self),
            hevent.into_param().abi(),
            ::std::mem::transmute(pdwcookie),
        )
    }
    pub unsafe fn UnregisterHardwareContentProtectionTeardownStatus(&self, dwcookie: u32) {
        (::windows::Interface::vtable(self).13)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(dwcookie),
        )
    }
    pub unsafe fn QueryVideoMemoryInfo(
        &self,
        nodeindex: u32,
        memorysegmentgroup: DXGI_MEMORY_SEGMENT_GROUP,
        pvideomemoryinfo: *mut DXGI_QUERY_VIDEO_MEMORY_INFO,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).14)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(nodeindex),
            ::std::mem::transmute(memorysegmentgroup),
            ::std::mem::transmute(pvideomemoryinfo),
        )
    }
    pub unsafe fn SetVideoMemoryReservation(
        &self,
        nodeindex: u32,
        memorysegmentgroup: DXGI_MEMORY_SEGMENT_GROUP,
        reservation: u64,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).15)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(nodeindex),
            ::std::mem::transmute(memorysegmentgroup),
            ::std::mem::transmute(reservation),
        )
    }
    pub unsafe fn RegisterVideoMemoryBudgetChangeNotificationEvent<
        'a,
        T0__: ::windows::IntoParam<'a, super::SystemServices::HANDLE>,
    >(
        &self,
        hevent: T0__,
        pdwcookie: *mut u32,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).16)(
            ::windows::Abi::abi(self),
            hevent.into_param().abi(),
            ::std::mem::transmute(pdwcookie),
        )
    }
    pub unsafe fn UnregisterVideoMemoryBudgetChangeNotification(&self, dwcookie: u32) {
        (::windows::Interface::vtable(self).17)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(dwcookie),
        )
    }
    pub unsafe fn GetDesc3(&self, pdesc: *mut DXGI_ADAPTER_DESC3) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).18)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(pdesc),
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
        datasize: u32,
        pdata: *const ::std::ffi::c_void,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        name: *const ::windows::Guid,
        punknown: ::windows::RawPtr,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        name: *const ::windows::Guid,
        pdatasize: *mut u32,
        pdata: *mut ::std::ffi::c_void,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        riid: *const ::windows::Guid,
        ppparent: *mut *mut ::std::ffi::c_void,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        output: u32,
        ppoutput: *mut ::windows::RawPtr,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        pdesc: *mut DXGI_ADAPTER_DESC,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        interfacename: *const ::windows::Guid,
        pumdversion: *mut i64,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        pdesc: *mut DXGI_ADAPTER_DESC1,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        pdesc: *mut DXGI_ADAPTER_DESC2,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        hevent: super::SystemServices::HANDLE,
        pdwcookie: *mut u32,
    ) -> ::windows::ErrorCode,
    pub unsafe extern "system" fn(this: ::windows::RawPtr, dwcookie: u32),
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        nodeindex: u32,
        memorysegmentgroup: DXGI_MEMORY_SEGMENT_GROUP,
        pvideomemoryinfo: *mut DXGI_QUERY_VIDEO_MEMORY_INFO,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        nodeindex: u32,
        memorysegmentgroup: DXGI_MEMORY_SEGMENT_GROUP,
        reservation: u64,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        hevent: super::SystemServices::HANDLE,
        pdwcookie: *mut u32,
    ) -> ::windows::ErrorCode,
    pub unsafe extern "system" fn(this: ::windows::RawPtr, dwcookie: u32),
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        pdesc: *mut DXGI_ADAPTER_DESC3,
    ) -> ::windows::ErrorCode,
);
#[repr(transparent)]
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
    pub unsafe fn IsLeakTrackingEnabledForThread(&self) -> super::SystemServices::BOOL {
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
    pub unsafe extern "system" fn(this: ::windows::RawPtr) -> super::SystemServices::BOOL,
);
#[repr(transparent)]
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
impl IDXGIDecodeSwapChain {
    pub unsafe fn PresentBuffer(
        &self,
        buffertopresent: u32,
        syncinterval: u32,
        flags: u32,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).3)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(buffertopresent),
            ::std::mem::transmute(syncinterval),
            ::std::mem::transmute(flags),
        )
    }
    pub unsafe fn SetSourceRect(
        &self,
        prect: *const super::DisplayDevices::RECT,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).4)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(prect),
        )
    }
    pub unsafe fn SetTargetRect(
        &self,
        prect: *const super::DisplayDevices::RECT,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).5)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(prect),
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
        prect: *mut super::DisplayDevices::RECT,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).7)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(prect),
        )
    }
    pub unsafe fn GetTargetRect(
        &self,
        prect: *mut super::DisplayDevices::RECT,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).8)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(prect),
        )
    }
    pub unsafe fn GetDestSize(&self, pwidth: *mut u32, pheight: *mut u32) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).9)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(pwidth),
            ::std::mem::transmute(pheight),
        )
    }
    pub unsafe fn SetColorSpace(
        &self,
        colorspace: DXGI_MULTIPLANE_OVERLAY_YCbCr_FLAGS,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).10)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(colorspace),
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
        buffertopresent: u32,
        syncinterval: u32,
        flags: u32,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        prect: *const super::DisplayDevices::RECT,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        prect: *const super::DisplayDevices::RECT,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        width: u32,
        height: u32,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        prect: *mut super::DisplayDevices::RECT,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        prect: *mut super::DisplayDevices::RECT,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        pwidth: *mut u32,
        pheight: *mut u32,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        colorspace: DXGI_MULTIPLANE_OVERLAY_YCbCr_FLAGS,
    ) -> ::windows::ErrorCode,
    pub unsafe extern "system" fn(this: ::windows::RawPtr) -> DXGI_MULTIPLANE_OVERLAY_YCbCr_FLAGS,
);
#[repr(transparent)]
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
impl IDXGIDevice {
    pub unsafe fn SetPrivateData(
        &self,
        name: *const ::windows::Guid,
        datasize: u32,
        pdata: *const ::std::ffi::c_void,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).3)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(name),
            ::std::mem::transmute(datasize),
            ::std::mem::transmute(pdata),
        )
    }
    pub unsafe fn SetPrivateDataInterface<
        'a,
        T1__: ::windows::IntoParam<'a, ::windows::IUnknown>,
    >(
        &self,
        name: *const ::windows::Guid,
        punknown: T1__,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).4)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(name),
            punknown.into_param().abi(),
        )
    }
    pub unsafe fn GetPrivateData(
        &self,
        name: *const ::windows::Guid,
        pdatasize: *mut u32,
        pdata: *mut ::std::ffi::c_void,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).5)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(name),
            ::std::mem::transmute(pdatasize),
            ::std::mem::transmute(pdata),
        )
    }
    pub unsafe fn GetParent(
        &self,
        riid: *const ::windows::Guid,
        ppparent: *mut *mut ::std::ffi::c_void,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).6)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(riid),
            ::std::mem::transmute(ppparent),
        )
    }
    pub unsafe fn GetAdapter(
        &self,
        padapter: *mut ::std::option::Option<IDXGIAdapter>,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).7)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(padapter),
        )
    }
    pub unsafe fn CreateSurface(
        &self,
        pdesc: *const DXGI_SURFACE_DESC,
        numsurfaces: u32,
        usage: u32,
        psharedresource: *const DXGI_SHARED_RESOURCE,
        ppsurface: *mut ::std::option::Option<IDXGISurface>,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).8)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(pdesc),
            ::std::mem::transmute(numsurfaces),
            ::std::mem::transmute(usage),
            ::std::mem::transmute(psharedresource),
            ::std::mem::transmute(ppsurface),
        )
    }
    pub unsafe fn QueryResourceResidency(
        &self,
        ppresources: *mut ::std::option::Option<::windows::IUnknown>,
        presidencystatus: *mut DXGI_RESIDENCY,
        numresources: u32,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).9)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(ppresources),
            ::std::mem::transmute(presidencystatus),
            ::std::mem::transmute(numresources),
        )
    }
    pub unsafe fn SetGPUThreadPriority(&self, priority: i32) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).10)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(priority),
        )
    }
    pub unsafe fn GetGPUThreadPriority(&self, ppriority: *mut i32) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).11)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(ppriority),
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
        datasize: u32,
        pdata: *const ::std::ffi::c_void,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        name: *const ::windows::Guid,
        punknown: ::windows::RawPtr,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        name: *const ::windows::Guid,
        pdatasize: *mut u32,
        pdata: *mut ::std::ffi::c_void,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        riid: *const ::windows::Guid,
        ppparent: *mut *mut ::std::ffi::c_void,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        padapter: *mut ::windows::RawPtr,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        pdesc: *const DXGI_SURFACE_DESC,
        numsurfaces: u32,
        usage: u32,
        psharedresource: *const DXGI_SHARED_RESOURCE,
        ppsurface: *mut ::windows::RawPtr,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        ppresources: *mut ::windows::RawPtr,
        presidencystatus: *mut DXGI_RESIDENCY,
        numresources: u32,
    ) -> ::windows::ErrorCode,
    pub unsafe extern "system" fn(this: ::windows::RawPtr, priority: i32) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        ppriority: *mut i32,
    ) -> ::windows::ErrorCode,
);
#[repr(transparent)]
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
impl IDXGIDevice1 {
    pub unsafe fn SetPrivateData(
        &self,
        name: *const ::windows::Guid,
        datasize: u32,
        pdata: *const ::std::ffi::c_void,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).3)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(name),
            ::std::mem::transmute(datasize),
            ::std::mem::transmute(pdata),
        )
    }
    pub unsafe fn SetPrivateDataInterface<
        'a,
        T1__: ::windows::IntoParam<'a, ::windows::IUnknown>,
    >(
        &self,
        name: *const ::windows::Guid,
        punknown: T1__,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).4)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(name),
            punknown.into_param().abi(),
        )
    }
    pub unsafe fn GetPrivateData(
        &self,
        name: *const ::windows::Guid,
        pdatasize: *mut u32,
        pdata: *mut ::std::ffi::c_void,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).5)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(name),
            ::std::mem::transmute(pdatasize),
            ::std::mem::transmute(pdata),
        )
    }
    pub unsafe fn GetParent(
        &self,
        riid: *const ::windows::Guid,
        ppparent: *mut *mut ::std::ffi::c_void,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).6)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(riid),
            ::std::mem::transmute(ppparent),
        )
    }
    pub unsafe fn GetAdapter(
        &self,
        padapter: *mut ::std::option::Option<IDXGIAdapter>,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).7)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(padapter),
        )
    }
    pub unsafe fn CreateSurface(
        &self,
        pdesc: *const DXGI_SURFACE_DESC,
        numsurfaces: u32,
        usage: u32,
        psharedresource: *const DXGI_SHARED_RESOURCE,
        ppsurface: *mut ::std::option::Option<IDXGISurface>,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).8)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(pdesc),
            ::std::mem::transmute(numsurfaces),
            ::std::mem::transmute(usage),
            ::std::mem::transmute(psharedresource),
            ::std::mem::transmute(ppsurface),
        )
    }
    pub unsafe fn QueryResourceResidency(
        &self,
        ppresources: *mut ::std::option::Option<::windows::IUnknown>,
        presidencystatus: *mut DXGI_RESIDENCY,
        numresources: u32,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).9)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(ppresources),
            ::std::mem::transmute(presidencystatus),
            ::std::mem::transmute(numresources),
        )
    }
    pub unsafe fn SetGPUThreadPriority(&self, priority: i32) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).10)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(priority),
        )
    }
    pub unsafe fn GetGPUThreadPriority(&self, ppriority: *mut i32) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).11)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(ppriority),
        )
    }
    pub unsafe fn SetMaximumFrameLatency(&self, maxlatency: u32) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).12)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(maxlatency),
        )
    }
    pub unsafe fn GetMaximumFrameLatency(&self, pmaxlatency: *mut u32) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).13)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(pmaxlatency),
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
        datasize: u32,
        pdata: *const ::std::ffi::c_void,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        name: *const ::windows::Guid,
        punknown: ::windows::RawPtr,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        name: *const ::windows::Guid,
        pdatasize: *mut u32,
        pdata: *mut ::std::ffi::c_void,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        riid: *const ::windows::Guid,
        ppparent: *mut *mut ::std::ffi::c_void,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        padapter: *mut ::windows::RawPtr,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        pdesc: *const DXGI_SURFACE_DESC,
        numsurfaces: u32,
        usage: u32,
        psharedresource: *const DXGI_SHARED_RESOURCE,
        ppsurface: *mut ::windows::RawPtr,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        ppresources: *mut ::windows::RawPtr,
        presidencystatus: *mut DXGI_RESIDENCY,
        numresources: u32,
    ) -> ::windows::ErrorCode,
    pub unsafe extern "system" fn(this: ::windows::RawPtr, priority: i32) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        ppriority: *mut i32,
    ) -> ::windows::ErrorCode,
    pub unsafe extern "system" fn(this: ::windows::RawPtr, maxlatency: u32) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        pmaxlatency: *mut u32,
    ) -> ::windows::ErrorCode,
);
#[repr(transparent)]
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
impl IDXGIDevice2 {
    pub unsafe fn SetPrivateData(
        &self,
        name: *const ::windows::Guid,
        datasize: u32,
        pdata: *const ::std::ffi::c_void,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).3)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(name),
            ::std::mem::transmute(datasize),
            ::std::mem::transmute(pdata),
        )
    }
    pub unsafe fn SetPrivateDataInterface<
        'a,
        T1__: ::windows::IntoParam<'a, ::windows::IUnknown>,
    >(
        &self,
        name: *const ::windows::Guid,
        punknown: T1__,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).4)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(name),
            punknown.into_param().abi(),
        )
    }
    pub unsafe fn GetPrivateData(
        &self,
        name: *const ::windows::Guid,
        pdatasize: *mut u32,
        pdata: *mut ::std::ffi::c_void,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).5)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(name),
            ::std::mem::transmute(pdatasize),
            ::std::mem::transmute(pdata),
        )
    }
    pub unsafe fn GetParent(
        &self,
        riid: *const ::windows::Guid,
        ppparent: *mut *mut ::std::ffi::c_void,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).6)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(riid),
            ::std::mem::transmute(ppparent),
        )
    }
    pub unsafe fn GetAdapter(
        &self,
        padapter: *mut ::std::option::Option<IDXGIAdapter>,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).7)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(padapter),
        )
    }
    pub unsafe fn CreateSurface(
        &self,
        pdesc: *const DXGI_SURFACE_DESC,
        numsurfaces: u32,
        usage: u32,
        psharedresource: *const DXGI_SHARED_RESOURCE,
        ppsurface: *mut ::std::option::Option<IDXGISurface>,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).8)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(pdesc),
            ::std::mem::transmute(numsurfaces),
            ::std::mem::transmute(usage),
            ::std::mem::transmute(psharedresource),
            ::std::mem::transmute(ppsurface),
        )
    }
    pub unsafe fn QueryResourceResidency(
        &self,
        ppresources: *mut ::std::option::Option<::windows::IUnknown>,
        presidencystatus: *mut DXGI_RESIDENCY,
        numresources: u32,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).9)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(ppresources),
            ::std::mem::transmute(presidencystatus),
            ::std::mem::transmute(numresources),
        )
    }
    pub unsafe fn SetGPUThreadPriority(&self, priority: i32) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).10)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(priority),
        )
    }
    pub unsafe fn GetGPUThreadPriority(&self, ppriority: *mut i32) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).11)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(ppriority),
        )
    }
    pub unsafe fn SetMaximumFrameLatency(&self, maxlatency: u32) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).12)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(maxlatency),
        )
    }
    pub unsafe fn GetMaximumFrameLatency(&self, pmaxlatency: *mut u32) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).13)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(pmaxlatency),
        )
    }
    pub unsafe fn OfferResources(
        &self,
        numresources: u32,
        ppresources: *mut ::std::option::Option<IDXGIResource>,
        priority: DXGI_OFFER_RESOURCE_PRIORITY,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).14)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(numresources),
            ::std::mem::transmute(ppresources),
            ::std::mem::transmute(priority),
        )
    }
    pub unsafe fn ReclaimResources(
        &self,
        numresources: u32,
        ppresources: *mut ::std::option::Option<IDXGIResource>,
        pdiscarded: *mut super::SystemServices::BOOL,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).15)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(numresources),
            ::std::mem::transmute(ppresources),
            ::std::mem::transmute(pdiscarded),
        )
    }
    pub unsafe fn EnqueueSetEvent<
        'a,
        T0__: ::windows::IntoParam<'a, super::SystemServices::HANDLE>,
    >(
        &self,
        hevent: T0__,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).16)(
            ::windows::Abi::abi(self),
            hevent.into_param().abi(),
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
        datasize: u32,
        pdata: *const ::std::ffi::c_void,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        name: *const ::windows::Guid,
        punknown: ::windows::RawPtr,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        name: *const ::windows::Guid,
        pdatasize: *mut u32,
        pdata: *mut ::std::ffi::c_void,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        riid: *const ::windows::Guid,
        ppparent: *mut *mut ::std::ffi::c_void,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        padapter: *mut ::windows::RawPtr,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        pdesc: *const DXGI_SURFACE_DESC,
        numsurfaces: u32,
        usage: u32,
        psharedresource: *const DXGI_SHARED_RESOURCE,
        ppsurface: *mut ::windows::RawPtr,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        ppresources: *mut ::windows::RawPtr,
        presidencystatus: *mut DXGI_RESIDENCY,
        numresources: u32,
    ) -> ::windows::ErrorCode,
    pub unsafe extern "system" fn(this: ::windows::RawPtr, priority: i32) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        ppriority: *mut i32,
    ) -> ::windows::ErrorCode,
    pub unsafe extern "system" fn(this: ::windows::RawPtr, maxlatency: u32) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        pmaxlatency: *mut u32,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        numresources: u32,
        ppresources: *mut ::windows::RawPtr,
        priority: DXGI_OFFER_RESOURCE_PRIORITY,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        numresources: u32,
        ppresources: *mut ::windows::RawPtr,
        pdiscarded: *mut super::SystemServices::BOOL,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        hevent: super::SystemServices::HANDLE,
    ) -> ::windows::ErrorCode,
);
#[repr(transparent)]
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
impl IDXGIDevice3 {
    pub unsafe fn SetPrivateData(
        &self,
        name: *const ::windows::Guid,
        datasize: u32,
        pdata: *const ::std::ffi::c_void,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).3)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(name),
            ::std::mem::transmute(datasize),
            ::std::mem::transmute(pdata),
        )
    }
    pub unsafe fn SetPrivateDataInterface<
        'a,
        T1__: ::windows::IntoParam<'a, ::windows::IUnknown>,
    >(
        &self,
        name: *const ::windows::Guid,
        punknown: T1__,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).4)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(name),
            punknown.into_param().abi(),
        )
    }
    pub unsafe fn GetPrivateData(
        &self,
        name: *const ::windows::Guid,
        pdatasize: *mut u32,
        pdata: *mut ::std::ffi::c_void,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).5)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(name),
            ::std::mem::transmute(pdatasize),
            ::std::mem::transmute(pdata),
        )
    }
    pub unsafe fn GetParent(
        &self,
        riid: *const ::windows::Guid,
        ppparent: *mut *mut ::std::ffi::c_void,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).6)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(riid),
            ::std::mem::transmute(ppparent),
        )
    }
    pub unsafe fn GetAdapter(
        &self,
        padapter: *mut ::std::option::Option<IDXGIAdapter>,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).7)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(padapter),
        )
    }
    pub unsafe fn CreateSurface(
        &self,
        pdesc: *const DXGI_SURFACE_DESC,
        numsurfaces: u32,
        usage: u32,
        psharedresource: *const DXGI_SHARED_RESOURCE,
        ppsurface: *mut ::std::option::Option<IDXGISurface>,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).8)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(pdesc),
            ::std::mem::transmute(numsurfaces),
            ::std::mem::transmute(usage),
            ::std::mem::transmute(psharedresource),
            ::std::mem::transmute(ppsurface),
        )
    }
    pub unsafe fn QueryResourceResidency(
        &self,
        ppresources: *mut ::std::option::Option<::windows::IUnknown>,
        presidencystatus: *mut DXGI_RESIDENCY,
        numresources: u32,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).9)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(ppresources),
            ::std::mem::transmute(presidencystatus),
            ::std::mem::transmute(numresources),
        )
    }
    pub unsafe fn SetGPUThreadPriority(&self, priority: i32) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).10)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(priority),
        )
    }
    pub unsafe fn GetGPUThreadPriority(&self, ppriority: *mut i32) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).11)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(ppriority),
        )
    }
    pub unsafe fn SetMaximumFrameLatency(&self, maxlatency: u32) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).12)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(maxlatency),
        )
    }
    pub unsafe fn GetMaximumFrameLatency(&self, pmaxlatency: *mut u32) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).13)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(pmaxlatency),
        )
    }
    pub unsafe fn OfferResources(
        &self,
        numresources: u32,
        ppresources: *mut ::std::option::Option<IDXGIResource>,
        priority: DXGI_OFFER_RESOURCE_PRIORITY,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).14)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(numresources),
            ::std::mem::transmute(ppresources),
            ::std::mem::transmute(priority),
        )
    }
    pub unsafe fn ReclaimResources(
        &self,
        numresources: u32,
        ppresources: *mut ::std::option::Option<IDXGIResource>,
        pdiscarded: *mut super::SystemServices::BOOL,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).15)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(numresources),
            ::std::mem::transmute(ppresources),
            ::std::mem::transmute(pdiscarded),
        )
    }
    pub unsafe fn EnqueueSetEvent<
        'a,
        T0__: ::windows::IntoParam<'a, super::SystemServices::HANDLE>,
    >(
        &self,
        hevent: T0__,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).16)(
            ::windows::Abi::abi(self),
            hevent.into_param().abi(),
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
        datasize: u32,
        pdata: *const ::std::ffi::c_void,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        name: *const ::windows::Guid,
        punknown: ::windows::RawPtr,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        name: *const ::windows::Guid,
        pdatasize: *mut u32,
        pdata: *mut ::std::ffi::c_void,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        riid: *const ::windows::Guid,
        ppparent: *mut *mut ::std::ffi::c_void,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        padapter: *mut ::windows::RawPtr,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        pdesc: *const DXGI_SURFACE_DESC,
        numsurfaces: u32,
        usage: u32,
        psharedresource: *const DXGI_SHARED_RESOURCE,
        ppsurface: *mut ::windows::RawPtr,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        ppresources: *mut ::windows::RawPtr,
        presidencystatus: *mut DXGI_RESIDENCY,
        numresources: u32,
    ) -> ::windows::ErrorCode,
    pub unsafe extern "system" fn(this: ::windows::RawPtr, priority: i32) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        ppriority: *mut i32,
    ) -> ::windows::ErrorCode,
    pub unsafe extern "system" fn(this: ::windows::RawPtr, maxlatency: u32) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        pmaxlatency: *mut u32,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        numresources: u32,
        ppresources: *mut ::windows::RawPtr,
        priority: DXGI_OFFER_RESOURCE_PRIORITY,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        numresources: u32,
        ppresources: *mut ::windows::RawPtr,
        pdiscarded: *mut super::SystemServices::BOOL,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        hevent: super::SystemServices::HANDLE,
    ) -> ::windows::ErrorCode,
    pub unsafe extern "system" fn(this: ::windows::RawPtr),
);
#[repr(transparent)]
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
impl IDXGIDevice4 {
    pub unsafe fn SetPrivateData(
        &self,
        name: *const ::windows::Guid,
        datasize: u32,
        pdata: *const ::std::ffi::c_void,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).3)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(name),
            ::std::mem::transmute(datasize),
            ::std::mem::transmute(pdata),
        )
    }
    pub unsafe fn SetPrivateDataInterface<
        'a,
        T1__: ::windows::IntoParam<'a, ::windows::IUnknown>,
    >(
        &self,
        name: *const ::windows::Guid,
        punknown: T1__,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).4)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(name),
            punknown.into_param().abi(),
        )
    }
    pub unsafe fn GetPrivateData(
        &self,
        name: *const ::windows::Guid,
        pdatasize: *mut u32,
        pdata: *mut ::std::ffi::c_void,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).5)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(name),
            ::std::mem::transmute(pdatasize),
            ::std::mem::transmute(pdata),
        )
    }
    pub unsafe fn GetParent(
        &self,
        riid: *const ::windows::Guid,
        ppparent: *mut *mut ::std::ffi::c_void,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).6)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(riid),
            ::std::mem::transmute(ppparent),
        )
    }
    pub unsafe fn GetAdapter(
        &self,
        padapter: *mut ::std::option::Option<IDXGIAdapter>,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).7)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(padapter),
        )
    }
    pub unsafe fn CreateSurface(
        &self,
        pdesc: *const DXGI_SURFACE_DESC,
        numsurfaces: u32,
        usage: u32,
        psharedresource: *const DXGI_SHARED_RESOURCE,
        ppsurface: *mut ::std::option::Option<IDXGISurface>,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).8)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(pdesc),
            ::std::mem::transmute(numsurfaces),
            ::std::mem::transmute(usage),
            ::std::mem::transmute(psharedresource),
            ::std::mem::transmute(ppsurface),
        )
    }
    pub unsafe fn QueryResourceResidency(
        &self,
        ppresources: *mut ::std::option::Option<::windows::IUnknown>,
        presidencystatus: *mut DXGI_RESIDENCY,
        numresources: u32,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).9)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(ppresources),
            ::std::mem::transmute(presidencystatus),
            ::std::mem::transmute(numresources),
        )
    }
    pub unsafe fn SetGPUThreadPriority(&self, priority: i32) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).10)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(priority),
        )
    }
    pub unsafe fn GetGPUThreadPriority(&self, ppriority: *mut i32) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).11)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(ppriority),
        )
    }
    pub unsafe fn SetMaximumFrameLatency(&self, maxlatency: u32) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).12)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(maxlatency),
        )
    }
    pub unsafe fn GetMaximumFrameLatency(&self, pmaxlatency: *mut u32) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).13)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(pmaxlatency),
        )
    }
    pub unsafe fn OfferResources(
        &self,
        numresources: u32,
        ppresources: *mut ::std::option::Option<IDXGIResource>,
        priority: DXGI_OFFER_RESOURCE_PRIORITY,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).14)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(numresources),
            ::std::mem::transmute(ppresources),
            ::std::mem::transmute(priority),
        )
    }
    pub unsafe fn ReclaimResources(
        &self,
        numresources: u32,
        ppresources: *mut ::std::option::Option<IDXGIResource>,
        pdiscarded: *mut super::SystemServices::BOOL,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).15)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(numresources),
            ::std::mem::transmute(ppresources),
            ::std::mem::transmute(pdiscarded),
        )
    }
    pub unsafe fn EnqueueSetEvent<
        'a,
        T0__: ::windows::IntoParam<'a, super::SystemServices::HANDLE>,
    >(
        &self,
        hevent: T0__,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).16)(
            ::windows::Abi::abi(self),
            hevent.into_param().abi(),
        )
    }
    pub unsafe fn Trim(&self) {
        (::windows::Interface::vtable(self).17)(::windows::Abi::abi(self))
    }
    pub unsafe fn OfferResources1(
        &self,
        numresources: u32,
        ppresources: *mut ::std::option::Option<IDXGIResource>,
        priority: DXGI_OFFER_RESOURCE_PRIORITY,
        flags: u32,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).18)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(numresources),
            ::std::mem::transmute(ppresources),
            ::std::mem::transmute(priority),
            ::std::mem::transmute(flags),
        )
    }
    pub unsafe fn ReclaimResources1(
        &self,
        numresources: u32,
        ppresources: *mut ::std::option::Option<IDXGIResource>,
        presults: *mut DXGI_RECLAIM_RESOURCE_RESULTS,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).19)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(numresources),
            ::std::mem::transmute(ppresources),
            ::std::mem::transmute(presults),
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
        datasize: u32,
        pdata: *const ::std::ffi::c_void,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        name: *const ::windows::Guid,
        punknown: ::windows::RawPtr,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        name: *const ::windows::Guid,
        pdatasize: *mut u32,
        pdata: *mut ::std::ffi::c_void,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        riid: *const ::windows::Guid,
        ppparent: *mut *mut ::std::ffi::c_void,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        padapter: *mut ::windows::RawPtr,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        pdesc: *const DXGI_SURFACE_DESC,
        numsurfaces: u32,
        usage: u32,
        psharedresource: *const DXGI_SHARED_RESOURCE,
        ppsurface: *mut ::windows::RawPtr,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        ppresources: *mut ::windows::RawPtr,
        presidencystatus: *mut DXGI_RESIDENCY,
        numresources: u32,
    ) -> ::windows::ErrorCode,
    pub unsafe extern "system" fn(this: ::windows::RawPtr, priority: i32) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        ppriority: *mut i32,
    ) -> ::windows::ErrorCode,
    pub unsafe extern "system" fn(this: ::windows::RawPtr, maxlatency: u32) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        pmaxlatency: *mut u32,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        numresources: u32,
        ppresources: *mut ::windows::RawPtr,
        priority: DXGI_OFFER_RESOURCE_PRIORITY,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        numresources: u32,
        ppresources: *mut ::windows::RawPtr,
        pdiscarded: *mut super::SystemServices::BOOL,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        hevent: super::SystemServices::HANDLE,
    ) -> ::windows::ErrorCode,
    pub unsafe extern "system" fn(this: ::windows::RawPtr),
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        numresources: u32,
        ppresources: *mut ::windows::RawPtr,
        priority: DXGI_OFFER_RESOURCE_PRIORITY,
        flags: u32,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        numresources: u32,
        ppresources: *mut ::windows::RawPtr,
        presults: *mut DXGI_RECLAIM_RESOURCE_RESULTS,
    ) -> ::windows::ErrorCode,
);
#[repr(transparent)]
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
impl IDXGIDeviceSubObject {
    pub unsafe fn SetPrivateData(
        &self,
        name: *const ::windows::Guid,
        datasize: u32,
        pdata: *const ::std::ffi::c_void,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).3)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(name),
            ::std::mem::transmute(datasize),
            ::std::mem::transmute(pdata),
        )
    }
    pub unsafe fn SetPrivateDataInterface<
        'a,
        T1__: ::windows::IntoParam<'a, ::windows::IUnknown>,
    >(
        &self,
        name: *const ::windows::Guid,
        punknown: T1__,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).4)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(name),
            punknown.into_param().abi(),
        )
    }
    pub unsafe fn GetPrivateData(
        &self,
        name: *const ::windows::Guid,
        pdatasize: *mut u32,
        pdata: *mut ::std::ffi::c_void,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).5)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(name),
            ::std::mem::transmute(pdatasize),
            ::std::mem::transmute(pdata),
        )
    }
    pub unsafe fn GetParent(
        &self,
        riid: *const ::windows::Guid,
        ppparent: *mut *mut ::std::ffi::c_void,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).6)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(riid),
            ::std::mem::transmute(ppparent),
        )
    }
    pub unsafe fn GetDevice(
        &self,
        riid: *const ::windows::Guid,
        ppdevice: *mut *mut ::std::ffi::c_void,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).7)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(riid),
            ::std::mem::transmute(ppdevice),
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
        datasize: u32,
        pdata: *const ::std::ffi::c_void,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        name: *const ::windows::Guid,
        punknown: ::windows::RawPtr,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        name: *const ::windows::Guid,
        pdatasize: *mut u32,
        pdata: *mut ::std::ffi::c_void,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        riid: *const ::windows::Guid,
        ppparent: *mut *mut ::std::ffi::c_void,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        riid: *const ::windows::Guid,
        ppdevice: *mut *mut ::std::ffi::c_void,
    ) -> ::windows::ErrorCode,
);
#[repr(transparent)]
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
impl IDXGIDisplayControl {
    pub unsafe fn IsStereoEnabled(&self) -> super::SystemServices::BOOL {
        (::windows::Interface::vtable(self).3)(::windows::Abi::abi(self))
    }
    pub unsafe fn SetStereoEnabled<
        'a,
        T0__: ::windows::IntoParam<'a, super::SystemServices::BOOL>,
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
    pub unsafe extern "system" fn(this: ::windows::RawPtr) -> super::SystemServices::BOOL,
    pub unsafe extern "system" fn(this: ::windows::RawPtr, enabled: super::SystemServices::BOOL),
);
#[repr(transparent)]
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
impl IDXGIFactory {
    pub unsafe fn SetPrivateData(
        &self,
        name: *const ::windows::Guid,
        datasize: u32,
        pdata: *const ::std::ffi::c_void,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).3)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(name),
            ::std::mem::transmute(datasize),
            ::std::mem::transmute(pdata),
        )
    }
    pub unsafe fn SetPrivateDataInterface<
        'a,
        T1__: ::windows::IntoParam<'a, ::windows::IUnknown>,
    >(
        &self,
        name: *const ::windows::Guid,
        punknown: T1__,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).4)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(name),
            punknown.into_param().abi(),
        )
    }
    pub unsafe fn GetPrivateData(
        &self,
        name: *const ::windows::Guid,
        pdatasize: *mut u32,
        pdata: *mut ::std::ffi::c_void,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).5)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(name),
            ::std::mem::transmute(pdatasize),
            ::std::mem::transmute(pdata),
        )
    }
    pub unsafe fn GetParent(
        &self,
        riid: *const ::windows::Guid,
        ppparent: *mut *mut ::std::ffi::c_void,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).6)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(riid),
            ::std::mem::transmute(ppparent),
        )
    }
    pub unsafe fn EnumAdapters(
        &self,
        adapter: u32,
        ppadapter: *mut ::std::option::Option<IDXGIAdapter>,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).7)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(adapter),
            ::std::mem::transmute(ppadapter),
        )
    }
    pub unsafe fn MakeWindowAssociation<
        'a,
        T0__: ::windows::IntoParam<'a, super::WindowsAndMessaging::HWND>,
    >(
        &self,
        windowhandle: T0__,
        flags: u32,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).8)(
            ::windows::Abi::abi(self),
            windowhandle.into_param().abi(),
            ::std::mem::transmute(flags),
        )
    }
    pub unsafe fn GetWindowAssociation(
        &self,
        pwindowhandle: *mut super::WindowsAndMessaging::HWND,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).9)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(pwindowhandle),
        )
    }
    pub unsafe fn CreateSwapChain<'a, T0__: ::windows::IntoParam<'a, ::windows::IUnknown>>(
        &self,
        pdevice: T0__,
        pdesc: *mut DXGI_SWAP_CHAIN_DESC,
        ppswapchain: *mut ::std::option::Option<IDXGISwapChain>,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).10)(
            ::windows::Abi::abi(self),
            pdevice.into_param().abi(),
            ::std::mem::transmute(pdesc),
            ::std::mem::transmute(ppswapchain),
        )
    }
    pub unsafe fn CreateSoftwareAdapter(
        &self,
        module: isize,
        ppadapter: *mut ::std::option::Option<IDXGIAdapter>,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).11)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(module),
            ::std::mem::transmute(ppadapter),
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
        datasize: u32,
        pdata: *const ::std::ffi::c_void,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        name: *const ::windows::Guid,
        punknown: ::windows::RawPtr,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        name: *const ::windows::Guid,
        pdatasize: *mut u32,
        pdata: *mut ::std::ffi::c_void,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        riid: *const ::windows::Guid,
        ppparent: *mut *mut ::std::ffi::c_void,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        adapter: u32,
        ppadapter: *mut ::windows::RawPtr,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        windowhandle: super::WindowsAndMessaging::HWND,
        flags: u32,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        pwindowhandle: *mut super::WindowsAndMessaging::HWND,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        pdevice: ::windows::RawPtr,
        pdesc: *mut DXGI_SWAP_CHAIN_DESC,
        ppswapchain: *mut ::windows::RawPtr,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        module: isize,
        ppadapter: *mut ::windows::RawPtr,
    ) -> ::windows::ErrorCode,
);
#[repr(transparent)]
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
impl IDXGIFactory1 {
    pub unsafe fn SetPrivateData(
        &self,
        name: *const ::windows::Guid,
        datasize: u32,
        pdata: *const ::std::ffi::c_void,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).3)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(name),
            ::std::mem::transmute(datasize),
            ::std::mem::transmute(pdata),
        )
    }
    pub unsafe fn SetPrivateDataInterface<
        'a,
        T1__: ::windows::IntoParam<'a, ::windows::IUnknown>,
    >(
        &self,
        name: *const ::windows::Guid,
        punknown: T1__,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).4)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(name),
            punknown.into_param().abi(),
        )
    }
    pub unsafe fn GetPrivateData(
        &self,
        name: *const ::windows::Guid,
        pdatasize: *mut u32,
        pdata: *mut ::std::ffi::c_void,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).5)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(name),
            ::std::mem::transmute(pdatasize),
            ::std::mem::transmute(pdata),
        )
    }
    pub unsafe fn GetParent(
        &self,
        riid: *const ::windows::Guid,
        ppparent: *mut *mut ::std::ffi::c_void,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).6)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(riid),
            ::std::mem::transmute(ppparent),
        )
    }
    pub unsafe fn EnumAdapters(
        &self,
        adapter: u32,
        ppadapter: *mut ::std::option::Option<IDXGIAdapter>,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).7)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(adapter),
            ::std::mem::transmute(ppadapter),
        )
    }
    pub unsafe fn MakeWindowAssociation<
        'a,
        T0__: ::windows::IntoParam<'a, super::WindowsAndMessaging::HWND>,
    >(
        &self,
        windowhandle: T0__,
        flags: u32,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).8)(
            ::windows::Abi::abi(self),
            windowhandle.into_param().abi(),
            ::std::mem::transmute(flags),
        )
    }
    pub unsafe fn GetWindowAssociation(
        &self,
        pwindowhandle: *mut super::WindowsAndMessaging::HWND,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).9)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(pwindowhandle),
        )
    }
    pub unsafe fn CreateSwapChain<'a, T0__: ::windows::IntoParam<'a, ::windows::IUnknown>>(
        &self,
        pdevice: T0__,
        pdesc: *mut DXGI_SWAP_CHAIN_DESC,
        ppswapchain: *mut ::std::option::Option<IDXGISwapChain>,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).10)(
            ::windows::Abi::abi(self),
            pdevice.into_param().abi(),
            ::std::mem::transmute(pdesc),
            ::std::mem::transmute(ppswapchain),
        )
    }
    pub unsafe fn CreateSoftwareAdapter(
        &self,
        module: isize,
        ppadapter: *mut ::std::option::Option<IDXGIAdapter>,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).11)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(module),
            ::std::mem::transmute(ppadapter),
        )
    }
    pub unsafe fn EnumAdapters1(
        &self,
        adapter: u32,
        ppadapter: *mut ::std::option::Option<IDXGIAdapter1>,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).12)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(adapter),
            ::std::mem::transmute(ppadapter),
        )
    }
    pub unsafe fn IsCurrent(&self) -> super::SystemServices::BOOL {
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
        datasize: u32,
        pdata: *const ::std::ffi::c_void,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        name: *const ::windows::Guid,
        punknown: ::windows::RawPtr,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        name: *const ::windows::Guid,
        pdatasize: *mut u32,
        pdata: *mut ::std::ffi::c_void,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        riid: *const ::windows::Guid,
        ppparent: *mut *mut ::std::ffi::c_void,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        adapter: u32,
        ppadapter: *mut ::windows::RawPtr,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        windowhandle: super::WindowsAndMessaging::HWND,
        flags: u32,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        pwindowhandle: *mut super::WindowsAndMessaging::HWND,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        pdevice: ::windows::RawPtr,
        pdesc: *mut DXGI_SWAP_CHAIN_DESC,
        ppswapchain: *mut ::windows::RawPtr,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        module: isize,
        ppadapter: *mut ::windows::RawPtr,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        adapter: u32,
        ppadapter: *mut ::windows::RawPtr,
    ) -> ::windows::ErrorCode,
    pub unsafe extern "system" fn(this: ::windows::RawPtr) -> super::SystemServices::BOOL,
);
#[repr(transparent)]
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
impl IDXGIFactory2 {
    pub unsafe fn SetPrivateData(
        &self,
        name: *const ::windows::Guid,
        datasize: u32,
        pdata: *const ::std::ffi::c_void,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).3)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(name),
            ::std::mem::transmute(datasize),
            ::std::mem::transmute(pdata),
        )
    }
    pub unsafe fn SetPrivateDataInterface<
        'a,
        T1__: ::windows::IntoParam<'a, ::windows::IUnknown>,
    >(
        &self,
        name: *const ::windows::Guid,
        punknown: T1__,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).4)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(name),
            punknown.into_param().abi(),
        )
    }
    pub unsafe fn GetPrivateData(
        &self,
        name: *const ::windows::Guid,
        pdatasize: *mut u32,
        pdata: *mut ::std::ffi::c_void,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).5)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(name),
            ::std::mem::transmute(pdatasize),
            ::std::mem::transmute(pdata),
        )
    }
    pub unsafe fn GetParent(
        &self,
        riid: *const ::windows::Guid,
        ppparent: *mut *mut ::std::ffi::c_void,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).6)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(riid),
            ::std::mem::transmute(ppparent),
        )
    }
    pub unsafe fn EnumAdapters(
        &self,
        adapter: u32,
        ppadapter: *mut ::std::option::Option<IDXGIAdapter>,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).7)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(adapter),
            ::std::mem::transmute(ppadapter),
        )
    }
    pub unsafe fn MakeWindowAssociation<
        'a,
        T0__: ::windows::IntoParam<'a, super::WindowsAndMessaging::HWND>,
    >(
        &self,
        windowhandle: T0__,
        flags: u32,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).8)(
            ::windows::Abi::abi(self),
            windowhandle.into_param().abi(),
            ::std::mem::transmute(flags),
        )
    }
    pub unsafe fn GetWindowAssociation(
        &self,
        pwindowhandle: *mut super::WindowsAndMessaging::HWND,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).9)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(pwindowhandle),
        )
    }
    pub unsafe fn CreateSwapChain<'a, T0__: ::windows::IntoParam<'a, ::windows::IUnknown>>(
        &self,
        pdevice: T0__,
        pdesc: *mut DXGI_SWAP_CHAIN_DESC,
        ppswapchain: *mut ::std::option::Option<IDXGISwapChain>,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).10)(
            ::windows::Abi::abi(self),
            pdevice.into_param().abi(),
            ::std::mem::transmute(pdesc),
            ::std::mem::transmute(ppswapchain),
        )
    }
    pub unsafe fn CreateSoftwareAdapter(
        &self,
        module: isize,
        ppadapter: *mut ::std::option::Option<IDXGIAdapter>,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).11)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(module),
            ::std::mem::transmute(ppadapter),
        )
    }
    pub unsafe fn EnumAdapters1(
        &self,
        adapter: u32,
        ppadapter: *mut ::std::option::Option<IDXGIAdapter1>,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).12)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(adapter),
            ::std::mem::transmute(ppadapter),
        )
    }
    pub unsafe fn IsCurrent(&self) -> super::SystemServices::BOOL {
        (::windows::Interface::vtable(self).13)(::windows::Abi::abi(self))
    }
    pub unsafe fn IsWindowedStereoEnabled(&self) -> super::SystemServices::BOOL {
        (::windows::Interface::vtable(self).14)(::windows::Abi::abi(self))
    }
    pub unsafe fn CreateSwapChainForHwnd<
        'a,
        T0__: ::windows::IntoParam<'a, ::windows::IUnknown>,
        T1__: ::windows::IntoParam<'a, super::WindowsAndMessaging::HWND>,
        T4__: ::windows::IntoParam<'a, IDXGIOutput>,
    >(
        &self,
        pdevice: T0__,
        hwnd: T1__,
        pdesc: *const DXGI_SWAP_CHAIN_DESC1,
        pfullscreendesc: *const DXGI_SWAP_CHAIN_FULLSCREEN_DESC,
        prestricttooutput: T4__,
        ppswapchain: *mut ::std::option::Option<IDXGISwapChain1>,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).15)(
            ::windows::Abi::abi(self),
            pdevice.into_param().abi(),
            hwnd.into_param().abi(),
            ::std::mem::transmute(pdesc),
            ::std::mem::transmute(pfullscreendesc),
            prestricttooutput.into_param().abi(),
            ::std::mem::transmute(ppswapchain),
        )
    }
    pub unsafe fn CreateSwapChainForCoreWindow<
        'a,
        T0__: ::windows::IntoParam<'a, ::windows::IUnknown>,
        T1__: ::windows::IntoParam<'a, ::windows::IUnknown>,
        T3__: ::windows::IntoParam<'a, IDXGIOutput>,
    >(
        &self,
        pdevice: T0__,
        pwindow: T1__,
        pdesc: *const DXGI_SWAP_CHAIN_DESC1,
        prestricttooutput: T3__,
        ppswapchain: *mut ::std::option::Option<IDXGISwapChain1>,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).16)(
            ::windows::Abi::abi(self),
            pdevice.into_param().abi(),
            pwindow.into_param().abi(),
            ::std::mem::transmute(pdesc),
            prestricttooutput.into_param().abi(),
            ::std::mem::transmute(ppswapchain),
        )
    }
    pub unsafe fn GetSharedResourceAdapterLuid<
        'a,
        T0__: ::windows::IntoParam<'a, super::SystemServices::HANDLE>,
    >(
        &self,
        hresource: T0__,
        pluid: *mut super::Kernel::LUID,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).17)(
            ::windows::Abi::abi(self),
            hresource.into_param().abi(),
            ::std::mem::transmute(pluid),
        )
    }
    pub unsafe fn RegisterStereoStatusWindow<
        'a,
        T0__: ::windows::IntoParam<'a, super::WindowsAndMessaging::HWND>,
    >(
        &self,
        windowhandle: T0__,
        wmsg: u32,
        pdwcookie: *mut u32,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).18)(
            ::windows::Abi::abi(self),
            windowhandle.into_param().abi(),
            ::std::mem::transmute(wmsg),
            ::std::mem::transmute(pdwcookie),
        )
    }
    pub unsafe fn RegisterStereoStatusEvent<
        'a,
        T0__: ::windows::IntoParam<'a, super::SystemServices::HANDLE>,
    >(
        &self,
        hevent: T0__,
        pdwcookie: *mut u32,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).19)(
            ::windows::Abi::abi(self),
            hevent.into_param().abi(),
            ::std::mem::transmute(pdwcookie),
        )
    }
    pub unsafe fn UnregisterStereoStatus(&self, dwcookie: u32) {
        (::windows::Interface::vtable(self).20)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(dwcookie),
        )
    }
    pub unsafe fn RegisterOcclusionStatusWindow<
        'a,
        T0__: ::windows::IntoParam<'a, super::WindowsAndMessaging::HWND>,
    >(
        &self,
        windowhandle: T0__,
        wmsg: u32,
        pdwcookie: *mut u32,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).21)(
            ::windows::Abi::abi(self),
            windowhandle.into_param().abi(),
            ::std::mem::transmute(wmsg),
            ::std::mem::transmute(pdwcookie),
        )
    }
    pub unsafe fn RegisterOcclusionStatusEvent<
        'a,
        T0__: ::windows::IntoParam<'a, super::SystemServices::HANDLE>,
    >(
        &self,
        hevent: T0__,
        pdwcookie: *mut u32,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).22)(
            ::windows::Abi::abi(self),
            hevent.into_param().abi(),
            ::std::mem::transmute(pdwcookie),
        )
    }
    pub unsafe fn UnregisterOcclusionStatus(&self, dwcookie: u32) {
        (::windows::Interface::vtable(self).23)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(dwcookie),
        )
    }
    pub unsafe fn CreateSwapChainForComposition<
        'a,
        T0__: ::windows::IntoParam<'a, ::windows::IUnknown>,
        T2__: ::windows::IntoParam<'a, IDXGIOutput>,
    >(
        &self,
        pdevice: T0__,
        pdesc: *const DXGI_SWAP_CHAIN_DESC1,
        prestricttooutput: T2__,
        ppswapchain: *mut ::std::option::Option<IDXGISwapChain1>,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).24)(
            ::windows::Abi::abi(self),
            pdevice.into_param().abi(),
            ::std::mem::transmute(pdesc),
            prestricttooutput.into_param().abi(),
            ::std::mem::transmute(ppswapchain),
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
        datasize: u32,
        pdata: *const ::std::ffi::c_void,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        name: *const ::windows::Guid,
        punknown: ::windows::RawPtr,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        name: *const ::windows::Guid,
        pdatasize: *mut u32,
        pdata: *mut ::std::ffi::c_void,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        riid: *const ::windows::Guid,
        ppparent: *mut *mut ::std::ffi::c_void,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        adapter: u32,
        ppadapter: *mut ::windows::RawPtr,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        windowhandle: super::WindowsAndMessaging::HWND,
        flags: u32,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        pwindowhandle: *mut super::WindowsAndMessaging::HWND,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        pdevice: ::windows::RawPtr,
        pdesc: *mut DXGI_SWAP_CHAIN_DESC,
        ppswapchain: *mut ::windows::RawPtr,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        module: isize,
        ppadapter: *mut ::windows::RawPtr,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        adapter: u32,
        ppadapter: *mut ::windows::RawPtr,
    ) -> ::windows::ErrorCode,
    pub unsafe extern "system" fn(this: ::windows::RawPtr) -> super::SystemServices::BOOL,
    pub unsafe extern "system" fn(this: ::windows::RawPtr) -> super::SystemServices::BOOL,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        pdevice: ::windows::RawPtr,
        hwnd: super::WindowsAndMessaging::HWND,
        pdesc: *const DXGI_SWAP_CHAIN_DESC1,
        pfullscreendesc: *const DXGI_SWAP_CHAIN_FULLSCREEN_DESC,
        prestricttooutput: ::windows::RawPtr,
        ppswapchain: *mut ::windows::RawPtr,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        pdevice: ::windows::RawPtr,
        pwindow: ::windows::RawPtr,
        pdesc: *const DXGI_SWAP_CHAIN_DESC1,
        prestricttooutput: ::windows::RawPtr,
        ppswapchain: *mut ::windows::RawPtr,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        hresource: super::SystemServices::HANDLE,
        pluid: *mut super::Kernel::LUID,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        windowhandle: super::WindowsAndMessaging::HWND,
        wmsg: u32,
        pdwcookie: *mut u32,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        hevent: super::SystemServices::HANDLE,
        pdwcookie: *mut u32,
    ) -> ::windows::ErrorCode,
    pub unsafe extern "system" fn(this: ::windows::RawPtr, dwcookie: u32),
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        windowhandle: super::WindowsAndMessaging::HWND,
        wmsg: u32,
        pdwcookie: *mut u32,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        hevent: super::SystemServices::HANDLE,
        pdwcookie: *mut u32,
    ) -> ::windows::ErrorCode,
    pub unsafe extern "system" fn(this: ::windows::RawPtr, dwcookie: u32),
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        pdevice: ::windows::RawPtr,
        pdesc: *const DXGI_SWAP_CHAIN_DESC1,
        prestricttooutput: ::windows::RawPtr,
        ppswapchain: *mut ::windows::RawPtr,
    ) -> ::windows::ErrorCode,
);
#[repr(transparent)]
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
impl IDXGIFactory3 {
    pub unsafe fn SetPrivateData(
        &self,
        name: *const ::windows::Guid,
        datasize: u32,
        pdata: *const ::std::ffi::c_void,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).3)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(name),
            ::std::mem::transmute(datasize),
            ::std::mem::transmute(pdata),
        )
    }
    pub unsafe fn SetPrivateDataInterface<
        'a,
        T1__: ::windows::IntoParam<'a, ::windows::IUnknown>,
    >(
        &self,
        name: *const ::windows::Guid,
        punknown: T1__,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).4)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(name),
            punknown.into_param().abi(),
        )
    }
    pub unsafe fn GetPrivateData(
        &self,
        name: *const ::windows::Guid,
        pdatasize: *mut u32,
        pdata: *mut ::std::ffi::c_void,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).5)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(name),
            ::std::mem::transmute(pdatasize),
            ::std::mem::transmute(pdata),
        )
    }
    pub unsafe fn GetParent(
        &self,
        riid: *const ::windows::Guid,
        ppparent: *mut *mut ::std::ffi::c_void,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).6)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(riid),
            ::std::mem::transmute(ppparent),
        )
    }
    pub unsafe fn EnumAdapters(
        &self,
        adapter: u32,
        ppadapter: *mut ::std::option::Option<IDXGIAdapter>,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).7)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(adapter),
            ::std::mem::transmute(ppadapter),
        )
    }
    pub unsafe fn MakeWindowAssociation<
        'a,
        T0__: ::windows::IntoParam<'a, super::WindowsAndMessaging::HWND>,
    >(
        &self,
        windowhandle: T0__,
        flags: u32,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).8)(
            ::windows::Abi::abi(self),
            windowhandle.into_param().abi(),
            ::std::mem::transmute(flags),
        )
    }
    pub unsafe fn GetWindowAssociation(
        &self,
        pwindowhandle: *mut super::WindowsAndMessaging::HWND,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).9)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(pwindowhandle),
        )
    }
    pub unsafe fn CreateSwapChain<'a, T0__: ::windows::IntoParam<'a, ::windows::IUnknown>>(
        &self,
        pdevice: T0__,
        pdesc: *mut DXGI_SWAP_CHAIN_DESC,
        ppswapchain: *mut ::std::option::Option<IDXGISwapChain>,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).10)(
            ::windows::Abi::abi(self),
            pdevice.into_param().abi(),
            ::std::mem::transmute(pdesc),
            ::std::mem::transmute(ppswapchain),
        )
    }
    pub unsafe fn CreateSoftwareAdapter(
        &self,
        module: isize,
        ppadapter: *mut ::std::option::Option<IDXGIAdapter>,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).11)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(module),
            ::std::mem::transmute(ppadapter),
        )
    }
    pub unsafe fn EnumAdapters1(
        &self,
        adapter: u32,
        ppadapter: *mut ::std::option::Option<IDXGIAdapter1>,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).12)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(adapter),
            ::std::mem::transmute(ppadapter),
        )
    }
    pub unsafe fn IsCurrent(&self) -> super::SystemServices::BOOL {
        (::windows::Interface::vtable(self).13)(::windows::Abi::abi(self))
    }
    pub unsafe fn IsWindowedStereoEnabled(&self) -> super::SystemServices::BOOL {
        (::windows::Interface::vtable(self).14)(::windows::Abi::abi(self))
    }
    pub unsafe fn CreateSwapChainForHwnd<
        'a,
        T0__: ::windows::IntoParam<'a, ::windows::IUnknown>,
        T1__: ::windows::IntoParam<'a, super::WindowsAndMessaging::HWND>,
        T4__: ::windows::IntoParam<'a, IDXGIOutput>,
    >(
        &self,
        pdevice: T0__,
        hwnd: T1__,
        pdesc: *const DXGI_SWAP_CHAIN_DESC1,
        pfullscreendesc: *const DXGI_SWAP_CHAIN_FULLSCREEN_DESC,
        prestricttooutput: T4__,
        ppswapchain: *mut ::std::option::Option<IDXGISwapChain1>,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).15)(
            ::windows::Abi::abi(self),
            pdevice.into_param().abi(),
            hwnd.into_param().abi(),
            ::std::mem::transmute(pdesc),
            ::std::mem::transmute(pfullscreendesc),
            prestricttooutput.into_param().abi(),
            ::std::mem::transmute(ppswapchain),
        )
    }
    pub unsafe fn CreateSwapChainForCoreWindow<
        'a,
        T0__: ::windows::IntoParam<'a, ::windows::IUnknown>,
        T1__: ::windows::IntoParam<'a, ::windows::IUnknown>,
        T3__: ::windows::IntoParam<'a, IDXGIOutput>,
    >(
        &self,
        pdevice: T0__,
        pwindow: T1__,
        pdesc: *const DXGI_SWAP_CHAIN_DESC1,
        prestricttooutput: T3__,
        ppswapchain: *mut ::std::option::Option<IDXGISwapChain1>,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).16)(
            ::windows::Abi::abi(self),
            pdevice.into_param().abi(),
            pwindow.into_param().abi(),
            ::std::mem::transmute(pdesc),
            prestricttooutput.into_param().abi(),
            ::std::mem::transmute(ppswapchain),
        )
    }
    pub unsafe fn GetSharedResourceAdapterLuid<
        'a,
        T0__: ::windows::IntoParam<'a, super::SystemServices::HANDLE>,
    >(
        &self,
        hresource: T0__,
        pluid: *mut super::Kernel::LUID,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).17)(
            ::windows::Abi::abi(self),
            hresource.into_param().abi(),
            ::std::mem::transmute(pluid),
        )
    }
    pub unsafe fn RegisterStereoStatusWindow<
        'a,
        T0__: ::windows::IntoParam<'a, super::WindowsAndMessaging::HWND>,
    >(
        &self,
        windowhandle: T0__,
        wmsg: u32,
        pdwcookie: *mut u32,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).18)(
            ::windows::Abi::abi(self),
            windowhandle.into_param().abi(),
            ::std::mem::transmute(wmsg),
            ::std::mem::transmute(pdwcookie),
        )
    }
    pub unsafe fn RegisterStereoStatusEvent<
        'a,
        T0__: ::windows::IntoParam<'a, super::SystemServices::HANDLE>,
    >(
        &self,
        hevent: T0__,
        pdwcookie: *mut u32,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).19)(
            ::windows::Abi::abi(self),
            hevent.into_param().abi(),
            ::std::mem::transmute(pdwcookie),
        )
    }
    pub unsafe fn UnregisterStereoStatus(&self, dwcookie: u32) {
        (::windows::Interface::vtable(self).20)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(dwcookie),
        )
    }
    pub unsafe fn RegisterOcclusionStatusWindow<
        'a,
        T0__: ::windows::IntoParam<'a, super::WindowsAndMessaging::HWND>,
    >(
        &self,
        windowhandle: T0__,
        wmsg: u32,
        pdwcookie: *mut u32,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).21)(
            ::windows::Abi::abi(self),
            windowhandle.into_param().abi(),
            ::std::mem::transmute(wmsg),
            ::std::mem::transmute(pdwcookie),
        )
    }
    pub unsafe fn RegisterOcclusionStatusEvent<
        'a,
        T0__: ::windows::IntoParam<'a, super::SystemServices::HANDLE>,
    >(
        &self,
        hevent: T0__,
        pdwcookie: *mut u32,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).22)(
            ::windows::Abi::abi(self),
            hevent.into_param().abi(),
            ::std::mem::transmute(pdwcookie),
        )
    }
    pub unsafe fn UnregisterOcclusionStatus(&self, dwcookie: u32) {
        (::windows::Interface::vtable(self).23)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(dwcookie),
        )
    }
    pub unsafe fn CreateSwapChainForComposition<
        'a,
        T0__: ::windows::IntoParam<'a, ::windows::IUnknown>,
        T2__: ::windows::IntoParam<'a, IDXGIOutput>,
    >(
        &self,
        pdevice: T0__,
        pdesc: *const DXGI_SWAP_CHAIN_DESC1,
        prestricttooutput: T2__,
        ppswapchain: *mut ::std::option::Option<IDXGISwapChain1>,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).24)(
            ::windows::Abi::abi(self),
            pdevice.into_param().abi(),
            ::std::mem::transmute(pdesc),
            prestricttooutput.into_param().abi(),
            ::std::mem::transmute(ppswapchain),
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
        datasize: u32,
        pdata: *const ::std::ffi::c_void,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        name: *const ::windows::Guid,
        punknown: ::windows::RawPtr,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        name: *const ::windows::Guid,
        pdatasize: *mut u32,
        pdata: *mut ::std::ffi::c_void,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        riid: *const ::windows::Guid,
        ppparent: *mut *mut ::std::ffi::c_void,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        adapter: u32,
        ppadapter: *mut ::windows::RawPtr,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        windowhandle: super::WindowsAndMessaging::HWND,
        flags: u32,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        pwindowhandle: *mut super::WindowsAndMessaging::HWND,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        pdevice: ::windows::RawPtr,
        pdesc: *mut DXGI_SWAP_CHAIN_DESC,
        ppswapchain: *mut ::windows::RawPtr,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        module: isize,
        ppadapter: *mut ::windows::RawPtr,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        adapter: u32,
        ppadapter: *mut ::windows::RawPtr,
    ) -> ::windows::ErrorCode,
    pub unsafe extern "system" fn(this: ::windows::RawPtr) -> super::SystemServices::BOOL,
    pub unsafe extern "system" fn(this: ::windows::RawPtr) -> super::SystemServices::BOOL,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        pdevice: ::windows::RawPtr,
        hwnd: super::WindowsAndMessaging::HWND,
        pdesc: *const DXGI_SWAP_CHAIN_DESC1,
        pfullscreendesc: *const DXGI_SWAP_CHAIN_FULLSCREEN_DESC,
        prestricttooutput: ::windows::RawPtr,
        ppswapchain: *mut ::windows::RawPtr,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        pdevice: ::windows::RawPtr,
        pwindow: ::windows::RawPtr,
        pdesc: *const DXGI_SWAP_CHAIN_DESC1,
        prestricttooutput: ::windows::RawPtr,
        ppswapchain: *mut ::windows::RawPtr,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        hresource: super::SystemServices::HANDLE,
        pluid: *mut super::Kernel::LUID,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        windowhandle: super::WindowsAndMessaging::HWND,
        wmsg: u32,
        pdwcookie: *mut u32,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        hevent: super::SystemServices::HANDLE,
        pdwcookie: *mut u32,
    ) -> ::windows::ErrorCode,
    pub unsafe extern "system" fn(this: ::windows::RawPtr, dwcookie: u32),
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        windowhandle: super::WindowsAndMessaging::HWND,
        wmsg: u32,
        pdwcookie: *mut u32,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        hevent: super::SystemServices::HANDLE,
        pdwcookie: *mut u32,
    ) -> ::windows::ErrorCode,
    pub unsafe extern "system" fn(this: ::windows::RawPtr, dwcookie: u32),
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        pdevice: ::windows::RawPtr,
        pdesc: *const DXGI_SWAP_CHAIN_DESC1,
        prestricttooutput: ::windows::RawPtr,
        ppswapchain: *mut ::windows::RawPtr,
    ) -> ::windows::ErrorCode,
    pub unsafe extern "system" fn(this: ::windows::RawPtr) -> u32,
);
#[repr(transparent)]
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
impl IDXGIFactory4 {
    pub unsafe fn SetPrivateData(
        &self,
        name: *const ::windows::Guid,
        datasize: u32,
        pdata: *const ::std::ffi::c_void,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).3)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(name),
            ::std::mem::transmute(datasize),
            ::std::mem::transmute(pdata),
        )
    }
    pub unsafe fn SetPrivateDataInterface<
        'a,
        T1__: ::windows::IntoParam<'a, ::windows::IUnknown>,
    >(
        &self,
        name: *const ::windows::Guid,
        punknown: T1__,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).4)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(name),
            punknown.into_param().abi(),
        )
    }
    pub unsafe fn GetPrivateData(
        &self,
        name: *const ::windows::Guid,
        pdatasize: *mut u32,
        pdata: *mut ::std::ffi::c_void,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).5)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(name),
            ::std::mem::transmute(pdatasize),
            ::std::mem::transmute(pdata),
        )
    }
    pub unsafe fn GetParent(
        &self,
        riid: *const ::windows::Guid,
        ppparent: *mut *mut ::std::ffi::c_void,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).6)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(riid),
            ::std::mem::transmute(ppparent),
        )
    }
    pub unsafe fn EnumAdapters(
        &self,
        adapter: u32,
        ppadapter: *mut ::std::option::Option<IDXGIAdapter>,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).7)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(adapter),
            ::std::mem::transmute(ppadapter),
        )
    }
    pub unsafe fn MakeWindowAssociation<
        'a,
        T0__: ::windows::IntoParam<'a, super::WindowsAndMessaging::HWND>,
    >(
        &self,
        windowhandle: T0__,
        flags: u32,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).8)(
            ::windows::Abi::abi(self),
            windowhandle.into_param().abi(),
            ::std::mem::transmute(flags),
        )
    }
    pub unsafe fn GetWindowAssociation(
        &self,
        pwindowhandle: *mut super::WindowsAndMessaging::HWND,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).9)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(pwindowhandle),
        )
    }
    pub unsafe fn CreateSwapChain<'a, T0__: ::windows::IntoParam<'a, ::windows::IUnknown>>(
        &self,
        pdevice: T0__,
        pdesc: *mut DXGI_SWAP_CHAIN_DESC,
        ppswapchain: *mut ::std::option::Option<IDXGISwapChain>,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).10)(
            ::windows::Abi::abi(self),
            pdevice.into_param().abi(),
            ::std::mem::transmute(pdesc),
            ::std::mem::transmute(ppswapchain),
        )
    }
    pub unsafe fn CreateSoftwareAdapter(
        &self,
        module: isize,
        ppadapter: *mut ::std::option::Option<IDXGIAdapter>,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).11)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(module),
            ::std::mem::transmute(ppadapter),
        )
    }
    pub unsafe fn EnumAdapters1(
        &self,
        adapter: u32,
        ppadapter: *mut ::std::option::Option<IDXGIAdapter1>,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).12)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(adapter),
            ::std::mem::transmute(ppadapter),
        )
    }
    pub unsafe fn IsCurrent(&self) -> super::SystemServices::BOOL {
        (::windows::Interface::vtable(self).13)(::windows::Abi::abi(self))
    }
    pub unsafe fn IsWindowedStereoEnabled(&self) -> super::SystemServices::BOOL {
        (::windows::Interface::vtable(self).14)(::windows::Abi::abi(self))
    }
    pub unsafe fn CreateSwapChainForHwnd<
        'a,
        T0__: ::windows::IntoParam<'a, ::windows::IUnknown>,
        T1__: ::windows::IntoParam<'a, super::WindowsAndMessaging::HWND>,
        T4__: ::windows::IntoParam<'a, IDXGIOutput>,
    >(
        &self,
        pdevice: T0__,
        hwnd: T1__,
        pdesc: *const DXGI_SWAP_CHAIN_DESC1,
        pfullscreendesc: *const DXGI_SWAP_CHAIN_FULLSCREEN_DESC,
        prestricttooutput: T4__,
        ppswapchain: *mut ::std::option::Option<IDXGISwapChain1>,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).15)(
            ::windows::Abi::abi(self),
            pdevice.into_param().abi(),
            hwnd.into_param().abi(),
            ::std::mem::transmute(pdesc),
            ::std::mem::transmute(pfullscreendesc),
            prestricttooutput.into_param().abi(),
            ::std::mem::transmute(ppswapchain),
        )
    }
    pub unsafe fn CreateSwapChainForCoreWindow<
        'a,
        T0__: ::windows::IntoParam<'a, ::windows::IUnknown>,
        T1__: ::windows::IntoParam<'a, ::windows::IUnknown>,
        T3__: ::windows::IntoParam<'a, IDXGIOutput>,
    >(
        &self,
        pdevice: T0__,
        pwindow: T1__,
        pdesc: *const DXGI_SWAP_CHAIN_DESC1,
        prestricttooutput: T3__,
        ppswapchain: *mut ::std::option::Option<IDXGISwapChain1>,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).16)(
            ::windows::Abi::abi(self),
            pdevice.into_param().abi(),
            pwindow.into_param().abi(),
            ::std::mem::transmute(pdesc),
            prestricttooutput.into_param().abi(),
            ::std::mem::transmute(ppswapchain),
        )
    }
    pub unsafe fn GetSharedResourceAdapterLuid<
        'a,
        T0__: ::windows::IntoParam<'a, super::SystemServices::HANDLE>,
    >(
        &self,
        hresource: T0__,
        pluid: *mut super::Kernel::LUID,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).17)(
            ::windows::Abi::abi(self),
            hresource.into_param().abi(),
            ::std::mem::transmute(pluid),
        )
    }
    pub unsafe fn RegisterStereoStatusWindow<
        'a,
        T0__: ::windows::IntoParam<'a, super::WindowsAndMessaging::HWND>,
    >(
        &self,
        windowhandle: T0__,
        wmsg: u32,
        pdwcookie: *mut u32,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).18)(
            ::windows::Abi::abi(self),
            windowhandle.into_param().abi(),
            ::std::mem::transmute(wmsg),
            ::std::mem::transmute(pdwcookie),
        )
    }
    pub unsafe fn RegisterStereoStatusEvent<
        'a,
        T0__: ::windows::IntoParam<'a, super::SystemServices::HANDLE>,
    >(
        &self,
        hevent: T0__,
        pdwcookie: *mut u32,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).19)(
            ::windows::Abi::abi(self),
            hevent.into_param().abi(),
            ::std::mem::transmute(pdwcookie),
        )
    }
    pub unsafe fn UnregisterStereoStatus(&self, dwcookie: u32) {
        (::windows::Interface::vtable(self).20)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(dwcookie),
        )
    }
    pub unsafe fn RegisterOcclusionStatusWindow<
        'a,
        T0__: ::windows::IntoParam<'a, super::WindowsAndMessaging::HWND>,
    >(
        &self,
        windowhandle: T0__,
        wmsg: u32,
        pdwcookie: *mut u32,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).21)(
            ::windows::Abi::abi(self),
            windowhandle.into_param().abi(),
            ::std::mem::transmute(wmsg),
            ::std::mem::transmute(pdwcookie),
        )
    }
    pub unsafe fn RegisterOcclusionStatusEvent<
        'a,
        T0__: ::windows::IntoParam<'a, super::SystemServices::HANDLE>,
    >(
        &self,
        hevent: T0__,
        pdwcookie: *mut u32,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).22)(
            ::windows::Abi::abi(self),
            hevent.into_param().abi(),
            ::std::mem::transmute(pdwcookie),
        )
    }
    pub unsafe fn UnregisterOcclusionStatus(&self, dwcookie: u32) {
        (::windows::Interface::vtable(self).23)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(dwcookie),
        )
    }
    pub unsafe fn CreateSwapChainForComposition<
        'a,
        T0__: ::windows::IntoParam<'a, ::windows::IUnknown>,
        T2__: ::windows::IntoParam<'a, IDXGIOutput>,
    >(
        &self,
        pdevice: T0__,
        pdesc: *const DXGI_SWAP_CHAIN_DESC1,
        prestricttooutput: T2__,
        ppswapchain: *mut ::std::option::Option<IDXGISwapChain1>,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).24)(
            ::windows::Abi::abi(self),
            pdevice.into_param().abi(),
            ::std::mem::transmute(pdesc),
            prestricttooutput.into_param().abi(),
            ::std::mem::transmute(ppswapchain),
        )
    }
    pub unsafe fn GetCreationFlags(&self) -> u32 {
        (::windows::Interface::vtable(self).25)(::windows::Abi::abi(self))
    }
    pub unsafe fn EnumAdapterByLuid<'a, T0__: ::windows::IntoParam<'a, super::Kernel::LUID>>(
        &self,
        adapterluid: T0__,
        riid: *const ::windows::Guid,
        ppvadapter: *mut *mut ::std::ffi::c_void,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).26)(
            ::windows::Abi::abi(self),
            adapterluid.into_param().abi(),
            ::std::mem::transmute(riid),
            ::std::mem::transmute(ppvadapter),
        )
    }
    pub unsafe fn EnumWarpAdapter(
        &self,
        riid: *const ::windows::Guid,
        ppvadapter: *mut *mut ::std::ffi::c_void,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).27)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(riid),
            ::std::mem::transmute(ppvadapter),
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
        datasize: u32,
        pdata: *const ::std::ffi::c_void,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        name: *const ::windows::Guid,
        punknown: ::windows::RawPtr,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        name: *const ::windows::Guid,
        pdatasize: *mut u32,
        pdata: *mut ::std::ffi::c_void,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        riid: *const ::windows::Guid,
        ppparent: *mut *mut ::std::ffi::c_void,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        adapter: u32,
        ppadapter: *mut ::windows::RawPtr,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        windowhandle: super::WindowsAndMessaging::HWND,
        flags: u32,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        pwindowhandle: *mut super::WindowsAndMessaging::HWND,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        pdevice: ::windows::RawPtr,
        pdesc: *mut DXGI_SWAP_CHAIN_DESC,
        ppswapchain: *mut ::windows::RawPtr,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        module: isize,
        ppadapter: *mut ::windows::RawPtr,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        adapter: u32,
        ppadapter: *mut ::windows::RawPtr,
    ) -> ::windows::ErrorCode,
    pub unsafe extern "system" fn(this: ::windows::RawPtr) -> super::SystemServices::BOOL,
    pub unsafe extern "system" fn(this: ::windows::RawPtr) -> super::SystemServices::BOOL,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        pdevice: ::windows::RawPtr,
        hwnd: super::WindowsAndMessaging::HWND,
        pdesc: *const DXGI_SWAP_CHAIN_DESC1,
        pfullscreendesc: *const DXGI_SWAP_CHAIN_FULLSCREEN_DESC,
        prestricttooutput: ::windows::RawPtr,
        ppswapchain: *mut ::windows::RawPtr,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        pdevice: ::windows::RawPtr,
        pwindow: ::windows::RawPtr,
        pdesc: *const DXGI_SWAP_CHAIN_DESC1,
        prestricttooutput: ::windows::RawPtr,
        ppswapchain: *mut ::windows::RawPtr,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        hresource: super::SystemServices::HANDLE,
        pluid: *mut super::Kernel::LUID,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        windowhandle: super::WindowsAndMessaging::HWND,
        wmsg: u32,
        pdwcookie: *mut u32,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        hevent: super::SystemServices::HANDLE,
        pdwcookie: *mut u32,
    ) -> ::windows::ErrorCode,
    pub unsafe extern "system" fn(this: ::windows::RawPtr, dwcookie: u32),
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        windowhandle: super::WindowsAndMessaging::HWND,
        wmsg: u32,
        pdwcookie: *mut u32,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        hevent: super::SystemServices::HANDLE,
        pdwcookie: *mut u32,
    ) -> ::windows::ErrorCode,
    pub unsafe extern "system" fn(this: ::windows::RawPtr, dwcookie: u32),
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        pdevice: ::windows::RawPtr,
        pdesc: *const DXGI_SWAP_CHAIN_DESC1,
        prestricttooutput: ::windows::RawPtr,
        ppswapchain: *mut ::windows::RawPtr,
    ) -> ::windows::ErrorCode,
    pub unsafe extern "system" fn(this: ::windows::RawPtr) -> u32,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        adapterluid: super::Kernel::LUID,
        riid: *const ::windows::Guid,
        ppvadapter: *mut *mut ::std::ffi::c_void,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        riid: *const ::windows::Guid,
        ppvadapter: *mut *mut ::std::ffi::c_void,
    ) -> ::windows::ErrorCode,
);
#[repr(transparent)]
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
impl IDXGIFactory5 {
    pub unsafe fn SetPrivateData(
        &self,
        name: *const ::windows::Guid,
        datasize: u32,
        pdata: *const ::std::ffi::c_void,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).3)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(name),
            ::std::mem::transmute(datasize),
            ::std::mem::transmute(pdata),
        )
    }
    pub unsafe fn SetPrivateDataInterface<
        'a,
        T1__: ::windows::IntoParam<'a, ::windows::IUnknown>,
    >(
        &self,
        name: *const ::windows::Guid,
        punknown: T1__,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).4)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(name),
            punknown.into_param().abi(),
        )
    }
    pub unsafe fn GetPrivateData(
        &self,
        name: *const ::windows::Guid,
        pdatasize: *mut u32,
        pdata: *mut ::std::ffi::c_void,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).5)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(name),
            ::std::mem::transmute(pdatasize),
            ::std::mem::transmute(pdata),
        )
    }
    pub unsafe fn GetParent(
        &self,
        riid: *const ::windows::Guid,
        ppparent: *mut *mut ::std::ffi::c_void,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).6)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(riid),
            ::std::mem::transmute(ppparent),
        )
    }
    pub unsafe fn EnumAdapters(
        &self,
        adapter: u32,
        ppadapter: *mut ::std::option::Option<IDXGIAdapter>,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).7)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(adapter),
            ::std::mem::transmute(ppadapter),
        )
    }
    pub unsafe fn MakeWindowAssociation<
        'a,
        T0__: ::windows::IntoParam<'a, super::WindowsAndMessaging::HWND>,
    >(
        &self,
        windowhandle: T0__,
        flags: u32,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).8)(
            ::windows::Abi::abi(self),
            windowhandle.into_param().abi(),
            ::std::mem::transmute(flags),
        )
    }
    pub unsafe fn GetWindowAssociation(
        &self,
        pwindowhandle: *mut super::WindowsAndMessaging::HWND,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).9)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(pwindowhandle),
        )
    }
    pub unsafe fn CreateSwapChain<'a, T0__: ::windows::IntoParam<'a, ::windows::IUnknown>>(
        &self,
        pdevice: T0__,
        pdesc: *mut DXGI_SWAP_CHAIN_DESC,
        ppswapchain: *mut ::std::option::Option<IDXGISwapChain>,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).10)(
            ::windows::Abi::abi(self),
            pdevice.into_param().abi(),
            ::std::mem::transmute(pdesc),
            ::std::mem::transmute(ppswapchain),
        )
    }
    pub unsafe fn CreateSoftwareAdapter(
        &self,
        module: isize,
        ppadapter: *mut ::std::option::Option<IDXGIAdapter>,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).11)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(module),
            ::std::mem::transmute(ppadapter),
        )
    }
    pub unsafe fn EnumAdapters1(
        &self,
        adapter: u32,
        ppadapter: *mut ::std::option::Option<IDXGIAdapter1>,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).12)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(adapter),
            ::std::mem::transmute(ppadapter),
        )
    }
    pub unsafe fn IsCurrent(&self) -> super::SystemServices::BOOL {
        (::windows::Interface::vtable(self).13)(::windows::Abi::abi(self))
    }
    pub unsafe fn IsWindowedStereoEnabled(&self) -> super::SystemServices::BOOL {
        (::windows::Interface::vtable(self).14)(::windows::Abi::abi(self))
    }
    pub unsafe fn CreateSwapChainForHwnd<
        'a,
        T0__: ::windows::IntoParam<'a, ::windows::IUnknown>,
        T1__: ::windows::IntoParam<'a, super::WindowsAndMessaging::HWND>,
        T4__: ::windows::IntoParam<'a, IDXGIOutput>,
    >(
        &self,
        pdevice: T0__,
        hwnd: T1__,
        pdesc: *const DXGI_SWAP_CHAIN_DESC1,
        pfullscreendesc: *const DXGI_SWAP_CHAIN_FULLSCREEN_DESC,
        prestricttooutput: T4__,
        ppswapchain: *mut ::std::option::Option<IDXGISwapChain1>,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).15)(
            ::windows::Abi::abi(self),
            pdevice.into_param().abi(),
            hwnd.into_param().abi(),
            ::std::mem::transmute(pdesc),
            ::std::mem::transmute(pfullscreendesc),
            prestricttooutput.into_param().abi(),
            ::std::mem::transmute(ppswapchain),
        )
    }
    pub unsafe fn CreateSwapChainForCoreWindow<
        'a,
        T0__: ::windows::IntoParam<'a, ::windows::IUnknown>,
        T1__: ::windows::IntoParam<'a, ::windows::IUnknown>,
        T3__: ::windows::IntoParam<'a, IDXGIOutput>,
    >(
        &self,
        pdevice: T0__,
        pwindow: T1__,
        pdesc: *const DXGI_SWAP_CHAIN_DESC1,
        prestricttooutput: T3__,
        ppswapchain: *mut ::std::option::Option<IDXGISwapChain1>,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).16)(
            ::windows::Abi::abi(self),
            pdevice.into_param().abi(),
            pwindow.into_param().abi(),
            ::std::mem::transmute(pdesc),
            prestricttooutput.into_param().abi(),
            ::std::mem::transmute(ppswapchain),
        )
    }
    pub unsafe fn GetSharedResourceAdapterLuid<
        'a,
        T0__: ::windows::IntoParam<'a, super::SystemServices::HANDLE>,
    >(
        &self,
        hresource: T0__,
        pluid: *mut super::Kernel::LUID,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).17)(
            ::windows::Abi::abi(self),
            hresource.into_param().abi(),
            ::std::mem::transmute(pluid),
        )
    }
    pub unsafe fn RegisterStereoStatusWindow<
        'a,
        T0__: ::windows::IntoParam<'a, super::WindowsAndMessaging::HWND>,
    >(
        &self,
        windowhandle: T0__,
        wmsg: u32,
        pdwcookie: *mut u32,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).18)(
            ::windows::Abi::abi(self),
            windowhandle.into_param().abi(),
            ::std::mem::transmute(wmsg),
            ::std::mem::transmute(pdwcookie),
        )
    }
    pub unsafe fn RegisterStereoStatusEvent<
        'a,
        T0__: ::windows::IntoParam<'a, super::SystemServices::HANDLE>,
    >(
        &self,
        hevent: T0__,
        pdwcookie: *mut u32,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).19)(
            ::windows::Abi::abi(self),
            hevent.into_param().abi(),
            ::std::mem::transmute(pdwcookie),
        )
    }
    pub unsafe fn UnregisterStereoStatus(&self, dwcookie: u32) {
        (::windows::Interface::vtable(self).20)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(dwcookie),
        )
    }
    pub unsafe fn RegisterOcclusionStatusWindow<
        'a,
        T0__: ::windows::IntoParam<'a, super::WindowsAndMessaging::HWND>,
    >(
        &self,
        windowhandle: T0__,
        wmsg: u32,
        pdwcookie: *mut u32,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).21)(
            ::windows::Abi::abi(self),
            windowhandle.into_param().abi(),
            ::std::mem::transmute(wmsg),
            ::std::mem::transmute(pdwcookie),
        )
    }
    pub unsafe fn RegisterOcclusionStatusEvent<
        'a,
        T0__: ::windows::IntoParam<'a, super::SystemServices::HANDLE>,
    >(
        &self,
        hevent: T0__,
        pdwcookie: *mut u32,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).22)(
            ::windows::Abi::abi(self),
            hevent.into_param().abi(),
            ::std::mem::transmute(pdwcookie),
        )
    }
    pub unsafe fn UnregisterOcclusionStatus(&self, dwcookie: u32) {
        (::windows::Interface::vtable(self).23)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(dwcookie),
        )
    }
    pub unsafe fn CreateSwapChainForComposition<
        'a,
        T0__: ::windows::IntoParam<'a, ::windows::IUnknown>,
        T2__: ::windows::IntoParam<'a, IDXGIOutput>,
    >(
        &self,
        pdevice: T0__,
        pdesc: *const DXGI_SWAP_CHAIN_DESC1,
        prestricttooutput: T2__,
        ppswapchain: *mut ::std::option::Option<IDXGISwapChain1>,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).24)(
            ::windows::Abi::abi(self),
            pdevice.into_param().abi(),
            ::std::mem::transmute(pdesc),
            prestricttooutput.into_param().abi(),
            ::std::mem::transmute(ppswapchain),
        )
    }
    pub unsafe fn GetCreationFlags(&self) -> u32 {
        (::windows::Interface::vtable(self).25)(::windows::Abi::abi(self))
    }
    pub unsafe fn EnumAdapterByLuid<'a, T0__: ::windows::IntoParam<'a, super::Kernel::LUID>>(
        &self,
        adapterluid: T0__,
        riid: *const ::windows::Guid,
        ppvadapter: *mut *mut ::std::ffi::c_void,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).26)(
            ::windows::Abi::abi(self),
            adapterluid.into_param().abi(),
            ::std::mem::transmute(riid),
            ::std::mem::transmute(ppvadapter),
        )
    }
    pub unsafe fn EnumWarpAdapter(
        &self,
        riid: *const ::windows::Guid,
        ppvadapter: *mut *mut ::std::ffi::c_void,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).27)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(riid),
            ::std::mem::transmute(ppvadapter),
        )
    }
    pub unsafe fn CheckFeatureSupport(
        &self,
        feature: DXGI_FEATURE,
        pfeaturesupportdata: *mut ::std::ffi::c_void,
        featuresupportdatasize: u32,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).28)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(feature),
            ::std::mem::transmute(pfeaturesupportdata),
            ::std::mem::transmute(featuresupportdatasize),
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
        datasize: u32,
        pdata: *const ::std::ffi::c_void,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        name: *const ::windows::Guid,
        punknown: ::windows::RawPtr,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        name: *const ::windows::Guid,
        pdatasize: *mut u32,
        pdata: *mut ::std::ffi::c_void,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        riid: *const ::windows::Guid,
        ppparent: *mut *mut ::std::ffi::c_void,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        adapter: u32,
        ppadapter: *mut ::windows::RawPtr,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        windowhandle: super::WindowsAndMessaging::HWND,
        flags: u32,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        pwindowhandle: *mut super::WindowsAndMessaging::HWND,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        pdevice: ::windows::RawPtr,
        pdesc: *mut DXGI_SWAP_CHAIN_DESC,
        ppswapchain: *mut ::windows::RawPtr,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        module: isize,
        ppadapter: *mut ::windows::RawPtr,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        adapter: u32,
        ppadapter: *mut ::windows::RawPtr,
    ) -> ::windows::ErrorCode,
    pub unsafe extern "system" fn(this: ::windows::RawPtr) -> super::SystemServices::BOOL,
    pub unsafe extern "system" fn(this: ::windows::RawPtr) -> super::SystemServices::BOOL,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        pdevice: ::windows::RawPtr,
        hwnd: super::WindowsAndMessaging::HWND,
        pdesc: *const DXGI_SWAP_CHAIN_DESC1,
        pfullscreendesc: *const DXGI_SWAP_CHAIN_FULLSCREEN_DESC,
        prestricttooutput: ::windows::RawPtr,
        ppswapchain: *mut ::windows::RawPtr,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        pdevice: ::windows::RawPtr,
        pwindow: ::windows::RawPtr,
        pdesc: *const DXGI_SWAP_CHAIN_DESC1,
        prestricttooutput: ::windows::RawPtr,
        ppswapchain: *mut ::windows::RawPtr,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        hresource: super::SystemServices::HANDLE,
        pluid: *mut super::Kernel::LUID,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        windowhandle: super::WindowsAndMessaging::HWND,
        wmsg: u32,
        pdwcookie: *mut u32,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        hevent: super::SystemServices::HANDLE,
        pdwcookie: *mut u32,
    ) -> ::windows::ErrorCode,
    pub unsafe extern "system" fn(this: ::windows::RawPtr, dwcookie: u32),
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        windowhandle: super::WindowsAndMessaging::HWND,
        wmsg: u32,
        pdwcookie: *mut u32,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        hevent: super::SystemServices::HANDLE,
        pdwcookie: *mut u32,
    ) -> ::windows::ErrorCode,
    pub unsafe extern "system" fn(this: ::windows::RawPtr, dwcookie: u32),
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        pdevice: ::windows::RawPtr,
        pdesc: *const DXGI_SWAP_CHAIN_DESC1,
        prestricttooutput: ::windows::RawPtr,
        ppswapchain: *mut ::windows::RawPtr,
    ) -> ::windows::ErrorCode,
    pub unsafe extern "system" fn(this: ::windows::RawPtr) -> u32,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        adapterluid: super::Kernel::LUID,
        riid: *const ::windows::Guid,
        ppvadapter: *mut *mut ::std::ffi::c_void,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        riid: *const ::windows::Guid,
        ppvadapter: *mut *mut ::std::ffi::c_void,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        feature: DXGI_FEATURE,
        pfeaturesupportdata: *mut ::std::ffi::c_void,
        featuresupportdatasize: u32,
    ) -> ::windows::ErrorCode,
);
#[repr(transparent)]
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
impl IDXGIFactory6 {
    pub unsafe fn SetPrivateData(
        &self,
        name: *const ::windows::Guid,
        datasize: u32,
        pdata: *const ::std::ffi::c_void,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).3)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(name),
            ::std::mem::transmute(datasize),
            ::std::mem::transmute(pdata),
        )
    }
    pub unsafe fn SetPrivateDataInterface<
        'a,
        T1__: ::windows::IntoParam<'a, ::windows::IUnknown>,
    >(
        &self,
        name: *const ::windows::Guid,
        punknown: T1__,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).4)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(name),
            punknown.into_param().abi(),
        )
    }
    pub unsafe fn GetPrivateData(
        &self,
        name: *const ::windows::Guid,
        pdatasize: *mut u32,
        pdata: *mut ::std::ffi::c_void,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).5)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(name),
            ::std::mem::transmute(pdatasize),
            ::std::mem::transmute(pdata),
        )
    }
    pub unsafe fn GetParent(
        &self,
        riid: *const ::windows::Guid,
        ppparent: *mut *mut ::std::ffi::c_void,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).6)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(riid),
            ::std::mem::transmute(ppparent),
        )
    }
    pub unsafe fn EnumAdapters(
        &self,
        adapter: u32,
        ppadapter: *mut ::std::option::Option<IDXGIAdapter>,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).7)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(adapter),
            ::std::mem::transmute(ppadapter),
        )
    }
    pub unsafe fn MakeWindowAssociation<
        'a,
        T0__: ::windows::IntoParam<'a, super::WindowsAndMessaging::HWND>,
    >(
        &self,
        windowhandle: T0__,
        flags: u32,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).8)(
            ::windows::Abi::abi(self),
            windowhandle.into_param().abi(),
            ::std::mem::transmute(flags),
        )
    }
    pub unsafe fn GetWindowAssociation(
        &self,
        pwindowhandle: *mut super::WindowsAndMessaging::HWND,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).9)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(pwindowhandle),
        )
    }
    pub unsafe fn CreateSwapChain<'a, T0__: ::windows::IntoParam<'a, ::windows::IUnknown>>(
        &self,
        pdevice: T0__,
        pdesc: *mut DXGI_SWAP_CHAIN_DESC,
        ppswapchain: *mut ::std::option::Option<IDXGISwapChain>,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).10)(
            ::windows::Abi::abi(self),
            pdevice.into_param().abi(),
            ::std::mem::transmute(pdesc),
            ::std::mem::transmute(ppswapchain),
        )
    }
    pub unsafe fn CreateSoftwareAdapter(
        &self,
        module: isize,
        ppadapter: *mut ::std::option::Option<IDXGIAdapter>,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).11)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(module),
            ::std::mem::transmute(ppadapter),
        )
    }
    pub unsafe fn EnumAdapters1(
        &self,
        adapter: u32,
        ppadapter: *mut ::std::option::Option<IDXGIAdapter1>,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).12)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(adapter),
            ::std::mem::transmute(ppadapter),
        )
    }
    pub unsafe fn IsCurrent(&self) -> super::SystemServices::BOOL {
        (::windows::Interface::vtable(self).13)(::windows::Abi::abi(self))
    }
    pub unsafe fn IsWindowedStereoEnabled(&self) -> super::SystemServices::BOOL {
        (::windows::Interface::vtable(self).14)(::windows::Abi::abi(self))
    }
    pub unsafe fn CreateSwapChainForHwnd<
        'a,
        T0__: ::windows::IntoParam<'a, ::windows::IUnknown>,
        T1__: ::windows::IntoParam<'a, super::WindowsAndMessaging::HWND>,
        T4__: ::windows::IntoParam<'a, IDXGIOutput>,
    >(
        &self,
        pdevice: T0__,
        hwnd: T1__,
        pdesc: *const DXGI_SWAP_CHAIN_DESC1,
        pfullscreendesc: *const DXGI_SWAP_CHAIN_FULLSCREEN_DESC,
        prestricttooutput: T4__,
        ppswapchain: *mut ::std::option::Option<IDXGISwapChain1>,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).15)(
            ::windows::Abi::abi(self),
            pdevice.into_param().abi(),
            hwnd.into_param().abi(),
            ::std::mem::transmute(pdesc),
            ::std::mem::transmute(pfullscreendesc),
            prestricttooutput.into_param().abi(),
            ::std::mem::transmute(ppswapchain),
        )
    }
    pub unsafe fn CreateSwapChainForCoreWindow<
        'a,
        T0__: ::windows::IntoParam<'a, ::windows::IUnknown>,
        T1__: ::windows::IntoParam<'a, ::windows::IUnknown>,
        T3__: ::windows::IntoParam<'a, IDXGIOutput>,
    >(
        &self,
        pdevice: T0__,
        pwindow: T1__,
        pdesc: *const DXGI_SWAP_CHAIN_DESC1,
        prestricttooutput: T3__,
        ppswapchain: *mut ::std::option::Option<IDXGISwapChain1>,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).16)(
            ::windows::Abi::abi(self),
            pdevice.into_param().abi(),
            pwindow.into_param().abi(),
            ::std::mem::transmute(pdesc),
            prestricttooutput.into_param().abi(),
            ::std::mem::transmute(ppswapchain),
        )
    }
    pub unsafe fn GetSharedResourceAdapterLuid<
        'a,
        T0__: ::windows::IntoParam<'a, super::SystemServices::HANDLE>,
    >(
        &self,
        hresource: T0__,
        pluid: *mut super::Kernel::LUID,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).17)(
            ::windows::Abi::abi(self),
            hresource.into_param().abi(),
            ::std::mem::transmute(pluid),
        )
    }
    pub unsafe fn RegisterStereoStatusWindow<
        'a,
        T0__: ::windows::IntoParam<'a, super::WindowsAndMessaging::HWND>,
    >(
        &self,
        windowhandle: T0__,
        wmsg: u32,
        pdwcookie: *mut u32,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).18)(
            ::windows::Abi::abi(self),
            windowhandle.into_param().abi(),
            ::std::mem::transmute(wmsg),
            ::std::mem::transmute(pdwcookie),
        )
    }
    pub unsafe fn RegisterStereoStatusEvent<
        'a,
        T0__: ::windows::IntoParam<'a, super::SystemServices::HANDLE>,
    >(
        &self,
        hevent: T0__,
        pdwcookie: *mut u32,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).19)(
            ::windows::Abi::abi(self),
            hevent.into_param().abi(),
            ::std::mem::transmute(pdwcookie),
        )
    }
    pub unsafe fn UnregisterStereoStatus(&self, dwcookie: u32) {
        (::windows::Interface::vtable(self).20)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(dwcookie),
        )
    }
    pub unsafe fn RegisterOcclusionStatusWindow<
        'a,
        T0__: ::windows::IntoParam<'a, super::WindowsAndMessaging::HWND>,
    >(
        &self,
        windowhandle: T0__,
        wmsg: u32,
        pdwcookie: *mut u32,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).21)(
            ::windows::Abi::abi(self),
            windowhandle.into_param().abi(),
            ::std::mem::transmute(wmsg),
            ::std::mem::transmute(pdwcookie),
        )
    }
    pub unsafe fn RegisterOcclusionStatusEvent<
        'a,
        T0__: ::windows::IntoParam<'a, super::SystemServices::HANDLE>,
    >(
        &self,
        hevent: T0__,
        pdwcookie: *mut u32,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).22)(
            ::windows::Abi::abi(self),
            hevent.into_param().abi(),
            ::std::mem::transmute(pdwcookie),
        )
    }
    pub unsafe fn UnregisterOcclusionStatus(&self, dwcookie: u32) {
        (::windows::Interface::vtable(self).23)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(dwcookie),
        )
    }
    pub unsafe fn CreateSwapChainForComposition<
        'a,
        T0__: ::windows::IntoParam<'a, ::windows::IUnknown>,
        T2__: ::windows::IntoParam<'a, IDXGIOutput>,
    >(
        &self,
        pdevice: T0__,
        pdesc: *const DXGI_SWAP_CHAIN_DESC1,
        prestricttooutput: T2__,
        ppswapchain: *mut ::std::option::Option<IDXGISwapChain1>,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).24)(
            ::windows::Abi::abi(self),
            pdevice.into_param().abi(),
            ::std::mem::transmute(pdesc),
            prestricttooutput.into_param().abi(),
            ::std::mem::transmute(ppswapchain),
        )
    }
    pub unsafe fn GetCreationFlags(&self) -> u32 {
        (::windows::Interface::vtable(self).25)(::windows::Abi::abi(self))
    }
    pub unsafe fn EnumAdapterByLuid<'a, T0__: ::windows::IntoParam<'a, super::Kernel::LUID>>(
        &self,
        adapterluid: T0__,
        riid: *const ::windows::Guid,
        ppvadapter: *mut *mut ::std::ffi::c_void,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).26)(
            ::windows::Abi::abi(self),
            adapterluid.into_param().abi(),
            ::std::mem::transmute(riid),
            ::std::mem::transmute(ppvadapter),
        )
    }
    pub unsafe fn EnumWarpAdapter(
        &self,
        riid: *const ::windows::Guid,
        ppvadapter: *mut *mut ::std::ffi::c_void,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).27)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(riid),
            ::std::mem::transmute(ppvadapter),
        )
    }
    pub unsafe fn CheckFeatureSupport(
        &self,
        feature: DXGI_FEATURE,
        pfeaturesupportdata: *mut ::std::ffi::c_void,
        featuresupportdatasize: u32,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).28)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(feature),
            ::std::mem::transmute(pfeaturesupportdata),
            ::std::mem::transmute(featuresupportdatasize),
        )
    }
    pub unsafe fn EnumAdapterByGpuPreference(
        &self,
        adapter: u32,
        gpupreference: DXGI_GPU_PREFERENCE,
        riid: *const ::windows::Guid,
        ppvadapter: *mut *mut ::std::ffi::c_void,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).29)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(adapter),
            ::std::mem::transmute(gpupreference),
            ::std::mem::transmute(riid),
            ::std::mem::transmute(ppvadapter),
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
        datasize: u32,
        pdata: *const ::std::ffi::c_void,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        name: *const ::windows::Guid,
        punknown: ::windows::RawPtr,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        name: *const ::windows::Guid,
        pdatasize: *mut u32,
        pdata: *mut ::std::ffi::c_void,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        riid: *const ::windows::Guid,
        ppparent: *mut *mut ::std::ffi::c_void,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        adapter: u32,
        ppadapter: *mut ::windows::RawPtr,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        windowhandle: super::WindowsAndMessaging::HWND,
        flags: u32,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        pwindowhandle: *mut super::WindowsAndMessaging::HWND,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        pdevice: ::windows::RawPtr,
        pdesc: *mut DXGI_SWAP_CHAIN_DESC,
        ppswapchain: *mut ::windows::RawPtr,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        module: isize,
        ppadapter: *mut ::windows::RawPtr,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        adapter: u32,
        ppadapter: *mut ::windows::RawPtr,
    ) -> ::windows::ErrorCode,
    pub unsafe extern "system" fn(this: ::windows::RawPtr) -> super::SystemServices::BOOL,
    pub unsafe extern "system" fn(this: ::windows::RawPtr) -> super::SystemServices::BOOL,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        pdevice: ::windows::RawPtr,
        hwnd: super::WindowsAndMessaging::HWND,
        pdesc: *const DXGI_SWAP_CHAIN_DESC1,
        pfullscreendesc: *const DXGI_SWAP_CHAIN_FULLSCREEN_DESC,
        prestricttooutput: ::windows::RawPtr,
        ppswapchain: *mut ::windows::RawPtr,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        pdevice: ::windows::RawPtr,
        pwindow: ::windows::RawPtr,
        pdesc: *const DXGI_SWAP_CHAIN_DESC1,
        prestricttooutput: ::windows::RawPtr,
        ppswapchain: *mut ::windows::RawPtr,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        hresource: super::SystemServices::HANDLE,
        pluid: *mut super::Kernel::LUID,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        windowhandle: super::WindowsAndMessaging::HWND,
        wmsg: u32,
        pdwcookie: *mut u32,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        hevent: super::SystemServices::HANDLE,
        pdwcookie: *mut u32,
    ) -> ::windows::ErrorCode,
    pub unsafe extern "system" fn(this: ::windows::RawPtr, dwcookie: u32),
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        windowhandle: super::WindowsAndMessaging::HWND,
        wmsg: u32,
        pdwcookie: *mut u32,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        hevent: super::SystemServices::HANDLE,
        pdwcookie: *mut u32,
    ) -> ::windows::ErrorCode,
    pub unsafe extern "system" fn(this: ::windows::RawPtr, dwcookie: u32),
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        pdevice: ::windows::RawPtr,
        pdesc: *const DXGI_SWAP_CHAIN_DESC1,
        prestricttooutput: ::windows::RawPtr,
        ppswapchain: *mut ::windows::RawPtr,
    ) -> ::windows::ErrorCode,
    pub unsafe extern "system" fn(this: ::windows::RawPtr) -> u32,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        adapterluid: super::Kernel::LUID,
        riid: *const ::windows::Guid,
        ppvadapter: *mut *mut ::std::ffi::c_void,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        riid: *const ::windows::Guid,
        ppvadapter: *mut *mut ::std::ffi::c_void,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        feature: DXGI_FEATURE,
        pfeaturesupportdata: *mut ::std::ffi::c_void,
        featuresupportdatasize: u32,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        adapter: u32,
        gpupreference: DXGI_GPU_PREFERENCE,
        riid: *const ::windows::Guid,
        ppvadapter: *mut *mut ::std::ffi::c_void,
    ) -> ::windows::ErrorCode,
);
#[repr(transparent)]
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
impl IDXGIFactory7 {
    pub unsafe fn SetPrivateData(
        &self,
        name: *const ::windows::Guid,
        datasize: u32,
        pdata: *const ::std::ffi::c_void,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).3)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(name),
            ::std::mem::transmute(datasize),
            ::std::mem::transmute(pdata),
        )
    }
    pub unsafe fn SetPrivateDataInterface<
        'a,
        T1__: ::windows::IntoParam<'a, ::windows::IUnknown>,
    >(
        &self,
        name: *const ::windows::Guid,
        punknown: T1__,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).4)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(name),
            punknown.into_param().abi(),
        )
    }
    pub unsafe fn GetPrivateData(
        &self,
        name: *const ::windows::Guid,
        pdatasize: *mut u32,
        pdata: *mut ::std::ffi::c_void,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).5)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(name),
            ::std::mem::transmute(pdatasize),
            ::std::mem::transmute(pdata),
        )
    }
    pub unsafe fn GetParent(
        &self,
        riid: *const ::windows::Guid,
        ppparent: *mut *mut ::std::ffi::c_void,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).6)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(riid),
            ::std::mem::transmute(ppparent),
        )
    }
    pub unsafe fn EnumAdapters(
        &self,
        adapter: u32,
        ppadapter: *mut ::std::option::Option<IDXGIAdapter>,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).7)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(adapter),
            ::std::mem::transmute(ppadapter),
        )
    }
    pub unsafe fn MakeWindowAssociation<
        'a,
        T0__: ::windows::IntoParam<'a, super::WindowsAndMessaging::HWND>,
    >(
        &self,
        windowhandle: T0__,
        flags: u32,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).8)(
            ::windows::Abi::abi(self),
            windowhandle.into_param().abi(),
            ::std::mem::transmute(flags),
        )
    }
    pub unsafe fn GetWindowAssociation(
        &self,
        pwindowhandle: *mut super::WindowsAndMessaging::HWND,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).9)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(pwindowhandle),
        )
    }
    pub unsafe fn CreateSwapChain<'a, T0__: ::windows::IntoParam<'a, ::windows::IUnknown>>(
        &self,
        pdevice: T0__,
        pdesc: *mut DXGI_SWAP_CHAIN_DESC,
        ppswapchain: *mut ::std::option::Option<IDXGISwapChain>,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).10)(
            ::windows::Abi::abi(self),
            pdevice.into_param().abi(),
            ::std::mem::transmute(pdesc),
            ::std::mem::transmute(ppswapchain),
        )
    }
    pub unsafe fn CreateSoftwareAdapter(
        &self,
        module: isize,
        ppadapter: *mut ::std::option::Option<IDXGIAdapter>,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).11)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(module),
            ::std::mem::transmute(ppadapter),
        )
    }
    pub unsafe fn EnumAdapters1(
        &self,
        adapter: u32,
        ppadapter: *mut ::std::option::Option<IDXGIAdapter1>,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).12)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(adapter),
            ::std::mem::transmute(ppadapter),
        )
    }
    pub unsafe fn IsCurrent(&self) -> super::SystemServices::BOOL {
        (::windows::Interface::vtable(self).13)(::windows::Abi::abi(self))
    }
    pub unsafe fn IsWindowedStereoEnabled(&self) -> super::SystemServices::BOOL {
        (::windows::Interface::vtable(self).14)(::windows::Abi::abi(self))
    }
    pub unsafe fn CreateSwapChainForHwnd<
        'a,
        T0__: ::windows::IntoParam<'a, ::windows::IUnknown>,
        T1__: ::windows::IntoParam<'a, super::WindowsAndMessaging::HWND>,
        T4__: ::windows::IntoParam<'a, IDXGIOutput>,
    >(
        &self,
        pdevice: T0__,
        hwnd: T1__,
        pdesc: *const DXGI_SWAP_CHAIN_DESC1,
        pfullscreendesc: *const DXGI_SWAP_CHAIN_FULLSCREEN_DESC,
        prestricttooutput: T4__,
        ppswapchain: *mut ::std::option::Option<IDXGISwapChain1>,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).15)(
            ::windows::Abi::abi(self),
            pdevice.into_param().abi(),
            hwnd.into_param().abi(),
            ::std::mem::transmute(pdesc),
            ::std::mem::transmute(pfullscreendesc),
            prestricttooutput.into_param().abi(),
            ::std::mem::transmute(ppswapchain),
        )
    }
    pub unsafe fn CreateSwapChainForCoreWindow<
        'a,
        T0__: ::windows::IntoParam<'a, ::windows::IUnknown>,
        T1__: ::windows::IntoParam<'a, ::windows::IUnknown>,
        T3__: ::windows::IntoParam<'a, IDXGIOutput>,
    >(
        &self,
        pdevice: T0__,
        pwindow: T1__,
        pdesc: *const DXGI_SWAP_CHAIN_DESC1,
        prestricttooutput: T3__,
        ppswapchain: *mut ::std::option::Option<IDXGISwapChain1>,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).16)(
            ::windows::Abi::abi(self),
            pdevice.into_param().abi(),
            pwindow.into_param().abi(),
            ::std::mem::transmute(pdesc),
            prestricttooutput.into_param().abi(),
            ::std::mem::transmute(ppswapchain),
        )
    }
    pub unsafe fn GetSharedResourceAdapterLuid<
        'a,
        T0__: ::windows::IntoParam<'a, super::SystemServices::HANDLE>,
    >(
        &self,
        hresource: T0__,
        pluid: *mut super::Kernel::LUID,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).17)(
            ::windows::Abi::abi(self),
            hresource.into_param().abi(),
            ::std::mem::transmute(pluid),
        )
    }
    pub unsafe fn RegisterStereoStatusWindow<
        'a,
        T0__: ::windows::IntoParam<'a, super::WindowsAndMessaging::HWND>,
    >(
        &self,
        windowhandle: T0__,
        wmsg: u32,
        pdwcookie: *mut u32,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).18)(
            ::windows::Abi::abi(self),
            windowhandle.into_param().abi(),
            ::std::mem::transmute(wmsg),
            ::std::mem::transmute(pdwcookie),
        )
    }
    pub unsafe fn RegisterStereoStatusEvent<
        'a,
        T0__: ::windows::IntoParam<'a, super::SystemServices::HANDLE>,
    >(
        &self,
        hevent: T0__,
        pdwcookie: *mut u32,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).19)(
            ::windows::Abi::abi(self),
            hevent.into_param().abi(),
            ::std::mem::transmute(pdwcookie),
        )
    }
    pub unsafe fn UnregisterStereoStatus(&self, dwcookie: u32) {
        (::windows::Interface::vtable(self).20)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(dwcookie),
        )
    }
    pub unsafe fn RegisterOcclusionStatusWindow<
        'a,
        T0__: ::windows::IntoParam<'a, super::WindowsAndMessaging::HWND>,
    >(
        &self,
        windowhandle: T0__,
        wmsg: u32,
        pdwcookie: *mut u32,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).21)(
            ::windows::Abi::abi(self),
            windowhandle.into_param().abi(),
            ::std::mem::transmute(wmsg),
            ::std::mem::transmute(pdwcookie),
        )
    }
    pub unsafe fn RegisterOcclusionStatusEvent<
        'a,
        T0__: ::windows::IntoParam<'a, super::SystemServices::HANDLE>,
    >(
        &self,
        hevent: T0__,
        pdwcookie: *mut u32,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).22)(
            ::windows::Abi::abi(self),
            hevent.into_param().abi(),
            ::std::mem::transmute(pdwcookie),
        )
    }
    pub unsafe fn UnregisterOcclusionStatus(&self, dwcookie: u32) {
        (::windows::Interface::vtable(self).23)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(dwcookie),
        )
    }
    pub unsafe fn CreateSwapChainForComposition<
        'a,
        T0__: ::windows::IntoParam<'a, ::windows::IUnknown>,
        T2__: ::windows::IntoParam<'a, IDXGIOutput>,
    >(
        &self,
        pdevice: T0__,
        pdesc: *const DXGI_SWAP_CHAIN_DESC1,
        prestricttooutput: T2__,
        ppswapchain: *mut ::std::option::Option<IDXGISwapChain1>,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).24)(
            ::windows::Abi::abi(self),
            pdevice.into_param().abi(),
            ::std::mem::transmute(pdesc),
            prestricttooutput.into_param().abi(),
            ::std::mem::transmute(ppswapchain),
        )
    }
    pub unsafe fn GetCreationFlags(&self) -> u32 {
        (::windows::Interface::vtable(self).25)(::windows::Abi::abi(self))
    }
    pub unsafe fn EnumAdapterByLuid<'a, T0__: ::windows::IntoParam<'a, super::Kernel::LUID>>(
        &self,
        adapterluid: T0__,
        riid: *const ::windows::Guid,
        ppvadapter: *mut *mut ::std::ffi::c_void,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).26)(
            ::windows::Abi::abi(self),
            adapterluid.into_param().abi(),
            ::std::mem::transmute(riid),
            ::std::mem::transmute(ppvadapter),
        )
    }
    pub unsafe fn EnumWarpAdapter(
        &self,
        riid: *const ::windows::Guid,
        ppvadapter: *mut *mut ::std::ffi::c_void,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).27)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(riid),
            ::std::mem::transmute(ppvadapter),
        )
    }
    pub unsafe fn CheckFeatureSupport(
        &self,
        feature: DXGI_FEATURE,
        pfeaturesupportdata: *mut ::std::ffi::c_void,
        featuresupportdatasize: u32,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).28)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(feature),
            ::std::mem::transmute(pfeaturesupportdata),
            ::std::mem::transmute(featuresupportdatasize),
        )
    }
    pub unsafe fn EnumAdapterByGpuPreference(
        &self,
        adapter: u32,
        gpupreference: DXGI_GPU_PREFERENCE,
        riid: *const ::windows::Guid,
        ppvadapter: *mut *mut ::std::ffi::c_void,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).29)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(adapter),
            ::std::mem::transmute(gpupreference),
            ::std::mem::transmute(riid),
            ::std::mem::transmute(ppvadapter),
        )
    }
    pub unsafe fn RegisterAdaptersChangedEvent<
        'a,
        T0__: ::windows::IntoParam<'a, super::SystemServices::HANDLE>,
    >(
        &self,
        hevent: T0__,
        pdwcookie: *mut u32,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).30)(
            ::windows::Abi::abi(self),
            hevent.into_param().abi(),
            ::std::mem::transmute(pdwcookie),
        )
    }
    pub unsafe fn UnregisterAdaptersChangedEvent(&self, dwcookie: u32) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).31)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(dwcookie),
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
        datasize: u32,
        pdata: *const ::std::ffi::c_void,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        name: *const ::windows::Guid,
        punknown: ::windows::RawPtr,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        name: *const ::windows::Guid,
        pdatasize: *mut u32,
        pdata: *mut ::std::ffi::c_void,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        riid: *const ::windows::Guid,
        ppparent: *mut *mut ::std::ffi::c_void,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        adapter: u32,
        ppadapter: *mut ::windows::RawPtr,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        windowhandle: super::WindowsAndMessaging::HWND,
        flags: u32,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        pwindowhandle: *mut super::WindowsAndMessaging::HWND,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        pdevice: ::windows::RawPtr,
        pdesc: *mut DXGI_SWAP_CHAIN_DESC,
        ppswapchain: *mut ::windows::RawPtr,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        module: isize,
        ppadapter: *mut ::windows::RawPtr,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        adapter: u32,
        ppadapter: *mut ::windows::RawPtr,
    ) -> ::windows::ErrorCode,
    pub unsafe extern "system" fn(this: ::windows::RawPtr) -> super::SystemServices::BOOL,
    pub unsafe extern "system" fn(this: ::windows::RawPtr) -> super::SystemServices::BOOL,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        pdevice: ::windows::RawPtr,
        hwnd: super::WindowsAndMessaging::HWND,
        pdesc: *const DXGI_SWAP_CHAIN_DESC1,
        pfullscreendesc: *const DXGI_SWAP_CHAIN_FULLSCREEN_DESC,
        prestricttooutput: ::windows::RawPtr,
        ppswapchain: *mut ::windows::RawPtr,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        pdevice: ::windows::RawPtr,
        pwindow: ::windows::RawPtr,
        pdesc: *const DXGI_SWAP_CHAIN_DESC1,
        prestricttooutput: ::windows::RawPtr,
        ppswapchain: *mut ::windows::RawPtr,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        hresource: super::SystemServices::HANDLE,
        pluid: *mut super::Kernel::LUID,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        windowhandle: super::WindowsAndMessaging::HWND,
        wmsg: u32,
        pdwcookie: *mut u32,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        hevent: super::SystemServices::HANDLE,
        pdwcookie: *mut u32,
    ) -> ::windows::ErrorCode,
    pub unsafe extern "system" fn(this: ::windows::RawPtr, dwcookie: u32),
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        windowhandle: super::WindowsAndMessaging::HWND,
        wmsg: u32,
        pdwcookie: *mut u32,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        hevent: super::SystemServices::HANDLE,
        pdwcookie: *mut u32,
    ) -> ::windows::ErrorCode,
    pub unsafe extern "system" fn(this: ::windows::RawPtr, dwcookie: u32),
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        pdevice: ::windows::RawPtr,
        pdesc: *const DXGI_SWAP_CHAIN_DESC1,
        prestricttooutput: ::windows::RawPtr,
        ppswapchain: *mut ::windows::RawPtr,
    ) -> ::windows::ErrorCode,
    pub unsafe extern "system" fn(this: ::windows::RawPtr) -> u32,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        adapterluid: super::Kernel::LUID,
        riid: *const ::windows::Guid,
        ppvadapter: *mut *mut ::std::ffi::c_void,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        riid: *const ::windows::Guid,
        ppvadapter: *mut *mut ::std::ffi::c_void,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        feature: DXGI_FEATURE,
        pfeaturesupportdata: *mut ::std::ffi::c_void,
        featuresupportdatasize: u32,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        adapter: u32,
        gpupreference: DXGI_GPU_PREFERENCE,
        riid: *const ::windows::Guid,
        ppvadapter: *mut *mut ::std::ffi::c_void,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        hevent: super::SystemServices::HANDLE,
        pdwcookie: *mut u32,
    ) -> ::windows::ErrorCode,
    pub unsafe extern "system" fn(this: ::windows::RawPtr, dwcookie: u32) -> ::windows::ErrorCode,
);
#[repr(transparent)]
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
impl IDXGIFactoryMedia {
    pub unsafe fn CreateSwapChainForCompositionSurfaceHandle<
        'a,
        T0__: ::windows::IntoParam<'a, ::windows::IUnknown>,
        T1__: ::windows::IntoParam<'a, super::SystemServices::HANDLE>,
        T3__: ::windows::IntoParam<'a, IDXGIOutput>,
    >(
        &self,
        pdevice: T0__,
        hsurface: T1__,
        pdesc: *const DXGI_SWAP_CHAIN_DESC1,
        prestricttooutput: T3__,
        ppswapchain: *mut ::std::option::Option<IDXGISwapChain1>,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).3)(
            ::windows::Abi::abi(self),
            pdevice.into_param().abi(),
            hsurface.into_param().abi(),
            ::std::mem::transmute(pdesc),
            prestricttooutput.into_param().abi(),
            ::std::mem::transmute(ppswapchain),
        )
    }
    pub unsafe fn CreateDecodeSwapChainForCompositionSurfaceHandle<
        'a,
        T0__: ::windows::IntoParam<'a, ::windows::IUnknown>,
        T1__: ::windows::IntoParam<'a, super::SystemServices::HANDLE>,
        T3__: ::windows::IntoParam<'a, IDXGIResource>,
        T4__: ::windows::IntoParam<'a, IDXGIOutput>,
    >(
        &self,
        pdevice: T0__,
        hsurface: T1__,
        pdesc: *mut DXGI_DECODE_SWAP_CHAIN_DESC,
        pyuvdecodebuffers: T3__,
        prestricttooutput: T4__,
        ppswapchain: *mut ::std::option::Option<IDXGIDecodeSwapChain>,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).4)(
            ::windows::Abi::abi(self),
            pdevice.into_param().abi(),
            hsurface.into_param().abi(),
            ::std::mem::transmute(pdesc),
            pyuvdecodebuffers.into_param().abi(),
            prestricttooutput.into_param().abi(),
            ::std::mem::transmute(ppswapchain),
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
        pdevice: ::windows::RawPtr,
        hsurface: super::SystemServices::HANDLE,
        pdesc: *const DXGI_SWAP_CHAIN_DESC1,
        prestricttooutput: ::windows::RawPtr,
        ppswapchain: *mut ::windows::RawPtr,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        pdevice: ::windows::RawPtr,
        hsurface: super::SystemServices::HANDLE,
        pdesc: *mut DXGI_DECODE_SWAP_CHAIN_DESC,
        pyuvdecodebuffers: ::windows::RawPtr,
        prestricttooutput: ::windows::RawPtr,
        ppswapchain: *mut ::windows::RawPtr,
    ) -> ::windows::ErrorCode,
);
#[repr(transparent)]
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
impl IDXGIInfoQueue {
    pub unsafe fn SetMessageCountLimit<'a, T0__: ::windows::IntoParam<'a, ::windows::Guid>>(
        &self,
        producer: T0__,
        messagecountlimit: u64,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).3)(
            ::windows::Abi::abi(self),
            producer.into_param().abi(),
            ::std::mem::transmute(messagecountlimit),
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
    pub unsafe fn GetMessage<'a, T0__: ::windows::IntoParam<'a, ::windows::Guid>>(
        &self,
        producer: T0__,
        messageindex: u64,
        pmessage: *mut DXGI_INFO_QUEUE_MESSAGE,
        pmessagebytelength: *mut usize,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).5)(
            ::windows::Abi::abi(self),
            producer.into_param().abi(),
            ::std::mem::transmute(messageindex),
            ::std::mem::transmute(pmessage),
            ::std::mem::transmute(pmessagebytelength),
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
        pfilter: *mut DXGI_INFO_QUEUE_FILTER,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).12)(
            ::windows::Abi::abi(self),
            producer.into_param().abi(),
            ::std::mem::transmute(pfilter),
        )
    }
    pub unsafe fn GetStorageFilter<'a, T0__: ::windows::IntoParam<'a, ::windows::Guid>>(
        &self,
        producer: T0__,
        pfilter: *mut DXGI_INFO_QUEUE_FILTER,
        pfilterbytelength: *mut usize,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).13)(
            ::windows::Abi::abi(self),
            producer.into_param().abi(),
            ::std::mem::transmute(pfilter),
            ::std::mem::transmute(pfilterbytelength),
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
        pfilter: *mut DXGI_INFO_QUEUE_FILTER,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).18)(
            ::windows::Abi::abi(self),
            producer.into_param().abi(),
            ::std::mem::transmute(pfilter),
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
        pfilter: *mut DXGI_INFO_QUEUE_FILTER,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).21)(
            ::windows::Abi::abi(self),
            producer.into_param().abi(),
            ::std::mem::transmute(pfilter),
        )
    }
    pub unsafe fn GetRetrievalFilter<'a, T0__: ::windows::IntoParam<'a, ::windows::Guid>>(
        &self,
        producer: T0__,
        pfilter: *mut DXGI_INFO_QUEUE_FILTER,
        pfilterbytelength: *mut usize,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).22)(
            ::windows::Abi::abi(self),
            producer.into_param().abi(),
            ::std::mem::transmute(pfilter),
            ::std::mem::transmute(pfilterbytelength),
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
        pfilter: *mut DXGI_INFO_QUEUE_FILTER,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).27)(
            ::windows::Abi::abi(self),
            producer.into_param().abi(),
            ::std::mem::transmute(pfilter),
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
        T4__: ::windows::IntoParam<'a, super::SystemServices::PSTR>,
    >(
        &self,
        producer: T0__,
        category: DXGI_INFO_QUEUE_MESSAGE_CATEGORY,
        severity: DXGI_INFO_QUEUE_MESSAGE_SEVERITY,
        id: i32,
        pdescription: T4__,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).30)(
            ::windows::Abi::abi(self),
            producer.into_param().abi(),
            ::std::mem::transmute(category),
            ::std::mem::transmute(severity),
            ::std::mem::transmute(id),
            pdescription.into_param().abi(),
        )
    }
    pub unsafe fn AddApplicationMessage<
        'a,
        T1__: ::windows::IntoParam<'a, super::SystemServices::PSTR>,
    >(
        &self,
        severity: DXGI_INFO_QUEUE_MESSAGE_SEVERITY,
        pdescription: T1__,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).31)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(severity),
            pdescription.into_param().abi(),
        )
    }
    pub unsafe fn SetBreakOnCategory<
        'a,
        T0__: ::windows::IntoParam<'a, ::windows::Guid>,
        T2__: ::windows::IntoParam<'a, super::SystemServices::BOOL>,
    >(
        &self,
        producer: T0__,
        category: DXGI_INFO_QUEUE_MESSAGE_CATEGORY,
        benable: T2__,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).32)(
            ::windows::Abi::abi(self),
            producer.into_param().abi(),
            ::std::mem::transmute(category),
            benable.into_param().abi(),
        )
    }
    pub unsafe fn SetBreakOnSeverity<
        'a,
        T0__: ::windows::IntoParam<'a, ::windows::Guid>,
        T2__: ::windows::IntoParam<'a, super::SystemServices::BOOL>,
    >(
        &self,
        producer: T0__,
        severity: DXGI_INFO_QUEUE_MESSAGE_SEVERITY,
        benable: T2__,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).33)(
            ::windows::Abi::abi(self),
            producer.into_param().abi(),
            ::std::mem::transmute(severity),
            benable.into_param().abi(),
        )
    }
    pub unsafe fn SetBreakOnID<
        'a,
        T0__: ::windows::IntoParam<'a, ::windows::Guid>,
        T2__: ::windows::IntoParam<'a, super::SystemServices::BOOL>,
    >(
        &self,
        producer: T0__,
        id: i32,
        benable: T2__,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).34)(
            ::windows::Abi::abi(self),
            producer.into_param().abi(),
            ::std::mem::transmute(id),
            benable.into_param().abi(),
        )
    }
    pub unsafe fn GetBreakOnCategory<'a, T0__: ::windows::IntoParam<'a, ::windows::Guid>>(
        &self,
        producer: T0__,
        category: DXGI_INFO_QUEUE_MESSAGE_CATEGORY,
    ) -> super::SystemServices::BOOL {
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
    ) -> super::SystemServices::BOOL {
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
    ) -> super::SystemServices::BOOL {
        (::windows::Interface::vtable(self).37)(
            ::windows::Abi::abi(self),
            producer.into_param().abi(),
            ::std::mem::transmute(id),
        )
    }
    pub unsafe fn SetMuteDebugOutput<
        'a,
        T0__: ::windows::IntoParam<'a, ::windows::Guid>,
        T1__: ::windows::IntoParam<'a, super::SystemServices::BOOL>,
    >(
        &self,
        producer: T0__,
        bmute: T1__,
    ) {
        (::windows::Interface::vtable(self).38)(
            ::windows::Abi::abi(self),
            producer.into_param().abi(),
            bmute.into_param().abi(),
        )
    }
    pub unsafe fn GetMuteDebugOutput<'a, T0__: ::windows::IntoParam<'a, ::windows::Guid>>(
        &self,
        producer: T0__,
    ) -> super::SystemServices::BOOL {
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
        messagecountlimit: u64,
    ) -> ::windows::ErrorCode,
    pub unsafe extern "system" fn(this: ::windows::RawPtr, producer: ::windows::Guid),
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        producer: ::windows::Guid,
        messageindex: u64,
        pmessage: *mut DXGI_INFO_QUEUE_MESSAGE,
        pmessagebytelength: *mut usize,
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
        pfilter: *mut DXGI_INFO_QUEUE_FILTER,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        producer: ::windows::Guid,
        pfilter: *mut DXGI_INFO_QUEUE_FILTER,
        pfilterbytelength: *mut usize,
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
        pfilter: *mut DXGI_INFO_QUEUE_FILTER,
    ) -> ::windows::ErrorCode,
    pub unsafe extern "system" fn(this: ::windows::RawPtr, producer: ::windows::Guid),
    pub unsafe extern "system" fn(this: ::windows::RawPtr, producer: ::windows::Guid) -> u32,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        producer: ::windows::Guid,
        pfilter: *mut DXGI_INFO_QUEUE_FILTER,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        producer: ::windows::Guid,
        pfilter: *mut DXGI_INFO_QUEUE_FILTER,
        pfilterbytelength: *mut usize,
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
        pfilter: *mut DXGI_INFO_QUEUE_FILTER,
    ) -> ::windows::ErrorCode,
    pub unsafe extern "system" fn(this: ::windows::RawPtr, producer: ::windows::Guid),
    pub unsafe extern "system" fn(this: ::windows::RawPtr, producer: ::windows::Guid) -> u32,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        producer: ::windows::Guid,
        category: DXGI_INFO_QUEUE_MESSAGE_CATEGORY,
        severity: DXGI_INFO_QUEUE_MESSAGE_SEVERITY,
        id: i32,
        pdescription: super::SystemServices::PSTR,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        severity: DXGI_INFO_QUEUE_MESSAGE_SEVERITY,
        pdescription: super::SystemServices::PSTR,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        producer: ::windows::Guid,
        category: DXGI_INFO_QUEUE_MESSAGE_CATEGORY,
        benable: super::SystemServices::BOOL,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        producer: ::windows::Guid,
        severity: DXGI_INFO_QUEUE_MESSAGE_SEVERITY,
        benable: super::SystemServices::BOOL,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        producer: ::windows::Guid,
        id: i32,
        benable: super::SystemServices::BOOL,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        producer: ::windows::Guid,
        category: DXGI_INFO_QUEUE_MESSAGE_CATEGORY,
    ) -> super::SystemServices::BOOL,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        producer: ::windows::Guid,
        severity: DXGI_INFO_QUEUE_MESSAGE_SEVERITY,
    ) -> super::SystemServices::BOOL,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        producer: ::windows::Guid,
        id: i32,
    ) -> super::SystemServices::BOOL,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        producer: ::windows::Guid,
        bmute: super::SystemServices::BOOL,
    ),
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        producer: ::windows::Guid,
    ) -> super::SystemServices::BOOL,
);
#[repr(transparent)]
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
impl IDXGIKeyedMutex {
    pub unsafe fn SetPrivateData(
        &self,
        name: *const ::windows::Guid,
        datasize: u32,
        pdata: *const ::std::ffi::c_void,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).3)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(name),
            ::std::mem::transmute(datasize),
            ::std::mem::transmute(pdata),
        )
    }
    pub unsafe fn SetPrivateDataInterface<
        'a,
        T1__: ::windows::IntoParam<'a, ::windows::IUnknown>,
    >(
        &self,
        name: *const ::windows::Guid,
        punknown: T1__,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).4)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(name),
            punknown.into_param().abi(),
        )
    }
    pub unsafe fn GetPrivateData(
        &self,
        name: *const ::windows::Guid,
        pdatasize: *mut u32,
        pdata: *mut ::std::ffi::c_void,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).5)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(name),
            ::std::mem::transmute(pdatasize),
            ::std::mem::transmute(pdata),
        )
    }
    pub unsafe fn GetParent(
        &self,
        riid: *const ::windows::Guid,
        ppparent: *mut *mut ::std::ffi::c_void,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).6)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(riid),
            ::std::mem::transmute(ppparent),
        )
    }
    pub unsafe fn GetDevice(
        &self,
        riid: *const ::windows::Guid,
        ppdevice: *mut *mut ::std::ffi::c_void,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).7)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(riid),
            ::std::mem::transmute(ppdevice),
        )
    }
    pub unsafe fn AcquireSync(&self, key: u64, dwmilliseconds: u32) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).8)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(key),
            ::std::mem::transmute(dwmilliseconds),
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
        datasize: u32,
        pdata: *const ::std::ffi::c_void,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        name: *const ::windows::Guid,
        punknown: ::windows::RawPtr,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        name: *const ::windows::Guid,
        pdatasize: *mut u32,
        pdata: *mut ::std::ffi::c_void,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        riid: *const ::windows::Guid,
        ppparent: *mut *mut ::std::ffi::c_void,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        riid: *const ::windows::Guid,
        ppdevice: *mut *mut ::std::ffi::c_void,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        key: u64,
        dwmilliseconds: u32,
    ) -> ::windows::ErrorCode,
    pub unsafe extern "system" fn(this: ::windows::RawPtr, key: u64) -> ::windows::ErrorCode,
);
#[repr(transparent)]
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
impl IDXGIOutput {
    pub unsafe fn SetPrivateData(
        &self,
        name: *const ::windows::Guid,
        datasize: u32,
        pdata: *const ::std::ffi::c_void,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).3)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(name),
            ::std::mem::transmute(datasize),
            ::std::mem::transmute(pdata),
        )
    }
    pub unsafe fn SetPrivateDataInterface<
        'a,
        T1__: ::windows::IntoParam<'a, ::windows::IUnknown>,
    >(
        &self,
        name: *const ::windows::Guid,
        punknown: T1__,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).4)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(name),
            punknown.into_param().abi(),
        )
    }
    pub unsafe fn GetPrivateData(
        &self,
        name: *const ::windows::Guid,
        pdatasize: *mut u32,
        pdata: *mut ::std::ffi::c_void,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).5)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(name),
            ::std::mem::transmute(pdatasize),
            ::std::mem::transmute(pdata),
        )
    }
    pub unsafe fn GetParent(
        &self,
        riid: *const ::windows::Guid,
        ppparent: *mut *mut ::std::ffi::c_void,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).6)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(riid),
            ::std::mem::transmute(ppparent),
        )
    }
    pub unsafe fn GetDesc(&self, pdesc: *mut DXGI_OUTPUT_DESC) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).7)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(pdesc),
        )
    }
    pub unsafe fn GetDisplayModeList(
        &self,
        enumformat: DXGI_FORMAT,
        flags: u32,
        pnummodes: *mut u32,
        pdesc: *mut DXGI_MODE_DESC,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).8)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(enumformat),
            ::std::mem::transmute(flags),
            ::std::mem::transmute(pnummodes),
            ::std::mem::transmute(pdesc),
        )
    }
    pub unsafe fn FindClosestMatchingMode<
        'a,
        T2__: ::windows::IntoParam<'a, ::windows::IUnknown>,
    >(
        &self,
        pmodetomatch: *const DXGI_MODE_DESC,
        pclosestmatch: *mut DXGI_MODE_DESC,
        pconcerneddevice: T2__,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).9)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(pmodetomatch),
            ::std::mem::transmute(pclosestmatch),
            pconcerneddevice.into_param().abi(),
        )
    }
    pub unsafe fn WaitForVBlank(&self) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).10)(::windows::Abi::abi(self))
    }
    pub unsafe fn TakeOwnership<
        'a,
        T0__: ::windows::IntoParam<'a, ::windows::IUnknown>,
        T1__: ::windows::IntoParam<'a, super::SystemServices::BOOL>,
    >(
        &self,
        pdevice: T0__,
        exclusive: T1__,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).11)(
            ::windows::Abi::abi(self),
            pdevice.into_param().abi(),
            exclusive.into_param().abi(),
        )
    }
    pub unsafe fn ReleaseOwnership(&self) {
        (::windows::Interface::vtable(self).12)(::windows::Abi::abi(self))
    }
    pub unsafe fn GetGammaControlCapabilities(
        &self,
        pgammacaps: *mut DXGI_GAMMA_CONTROL_CAPABILITIES,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).13)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(pgammacaps),
        )
    }
    pub unsafe fn SetGammaControl(
        &self,
        parray: *const DXGI_GAMMA_CONTROL,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).14)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(parray),
        )
    }
    pub unsafe fn GetGammaControl(&self, parray: *mut DXGI_GAMMA_CONTROL) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).15)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(parray),
        )
    }
    pub unsafe fn SetDisplaySurface<'a, T0__: ::windows::IntoParam<'a, IDXGISurface>>(
        &self,
        pscanoutsurface: T0__,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).16)(
            ::windows::Abi::abi(self),
            pscanoutsurface.into_param().abi(),
        )
    }
    pub unsafe fn GetDisplaySurfaceData<'a, T0__: ::windows::IntoParam<'a, IDXGISurface>>(
        &self,
        pdestination: T0__,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).17)(
            ::windows::Abi::abi(self),
            pdestination.into_param().abi(),
        )
    }
    pub unsafe fn GetFrameStatistics(
        &self,
        pstats: *mut DXGI_FRAME_STATISTICS,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).18)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(pstats),
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
        datasize: u32,
        pdata: *const ::std::ffi::c_void,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        name: *const ::windows::Guid,
        punknown: ::windows::RawPtr,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        name: *const ::windows::Guid,
        pdatasize: *mut u32,
        pdata: *mut ::std::ffi::c_void,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        riid: *const ::windows::Guid,
        ppparent: *mut *mut ::std::ffi::c_void,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        pdesc: *mut DXGI_OUTPUT_DESC,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        enumformat: DXGI_FORMAT,
        flags: u32,
        pnummodes: *mut u32,
        pdesc: *mut DXGI_MODE_DESC,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        pmodetomatch: *const DXGI_MODE_DESC,
        pclosestmatch: *mut DXGI_MODE_DESC,
        pconcerneddevice: ::windows::RawPtr,
    ) -> ::windows::ErrorCode,
    pub unsafe extern "system" fn(this: ::windows::RawPtr) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        pdevice: ::windows::RawPtr,
        exclusive: super::SystemServices::BOOL,
    ) -> ::windows::ErrorCode,
    pub unsafe extern "system" fn(this: ::windows::RawPtr),
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        pgammacaps: *mut DXGI_GAMMA_CONTROL_CAPABILITIES,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        parray: *const DXGI_GAMMA_CONTROL,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        parray: *mut DXGI_GAMMA_CONTROL,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        pscanoutsurface: ::windows::RawPtr,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        pdestination: ::windows::RawPtr,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        pstats: *mut DXGI_FRAME_STATISTICS,
    ) -> ::windows::ErrorCode,
);
#[repr(transparent)]
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
impl IDXGIOutput1 {
    pub unsafe fn SetPrivateData(
        &self,
        name: *const ::windows::Guid,
        datasize: u32,
        pdata: *const ::std::ffi::c_void,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).3)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(name),
            ::std::mem::transmute(datasize),
            ::std::mem::transmute(pdata),
        )
    }
    pub unsafe fn SetPrivateDataInterface<
        'a,
        T1__: ::windows::IntoParam<'a, ::windows::IUnknown>,
    >(
        &self,
        name: *const ::windows::Guid,
        punknown: T1__,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).4)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(name),
            punknown.into_param().abi(),
        )
    }
    pub unsafe fn GetPrivateData(
        &self,
        name: *const ::windows::Guid,
        pdatasize: *mut u32,
        pdata: *mut ::std::ffi::c_void,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).5)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(name),
            ::std::mem::transmute(pdatasize),
            ::std::mem::transmute(pdata),
        )
    }
    pub unsafe fn GetParent(
        &self,
        riid: *const ::windows::Guid,
        ppparent: *mut *mut ::std::ffi::c_void,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).6)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(riid),
            ::std::mem::transmute(ppparent),
        )
    }
    pub unsafe fn GetDesc(&self, pdesc: *mut DXGI_OUTPUT_DESC) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).7)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(pdesc),
        )
    }
    pub unsafe fn GetDisplayModeList(
        &self,
        enumformat: DXGI_FORMAT,
        flags: u32,
        pnummodes: *mut u32,
        pdesc: *mut DXGI_MODE_DESC,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).8)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(enumformat),
            ::std::mem::transmute(flags),
            ::std::mem::transmute(pnummodes),
            ::std::mem::transmute(pdesc),
        )
    }
    pub unsafe fn FindClosestMatchingMode<
        'a,
        T2__: ::windows::IntoParam<'a, ::windows::IUnknown>,
    >(
        &self,
        pmodetomatch: *const DXGI_MODE_DESC,
        pclosestmatch: *mut DXGI_MODE_DESC,
        pconcerneddevice: T2__,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).9)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(pmodetomatch),
            ::std::mem::transmute(pclosestmatch),
            pconcerneddevice.into_param().abi(),
        )
    }
    pub unsafe fn WaitForVBlank(&self) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).10)(::windows::Abi::abi(self))
    }
    pub unsafe fn TakeOwnership<
        'a,
        T0__: ::windows::IntoParam<'a, ::windows::IUnknown>,
        T1__: ::windows::IntoParam<'a, super::SystemServices::BOOL>,
    >(
        &self,
        pdevice: T0__,
        exclusive: T1__,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).11)(
            ::windows::Abi::abi(self),
            pdevice.into_param().abi(),
            exclusive.into_param().abi(),
        )
    }
    pub unsafe fn ReleaseOwnership(&self) {
        (::windows::Interface::vtable(self).12)(::windows::Abi::abi(self))
    }
    pub unsafe fn GetGammaControlCapabilities(
        &self,
        pgammacaps: *mut DXGI_GAMMA_CONTROL_CAPABILITIES,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).13)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(pgammacaps),
        )
    }
    pub unsafe fn SetGammaControl(
        &self,
        parray: *const DXGI_GAMMA_CONTROL,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).14)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(parray),
        )
    }
    pub unsafe fn GetGammaControl(&self, parray: *mut DXGI_GAMMA_CONTROL) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).15)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(parray),
        )
    }
    pub unsafe fn SetDisplaySurface<'a, T0__: ::windows::IntoParam<'a, IDXGISurface>>(
        &self,
        pscanoutsurface: T0__,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).16)(
            ::windows::Abi::abi(self),
            pscanoutsurface.into_param().abi(),
        )
    }
    pub unsafe fn GetDisplaySurfaceData<'a, T0__: ::windows::IntoParam<'a, IDXGISurface>>(
        &self,
        pdestination: T0__,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).17)(
            ::windows::Abi::abi(self),
            pdestination.into_param().abi(),
        )
    }
    pub unsafe fn GetFrameStatistics(
        &self,
        pstats: *mut DXGI_FRAME_STATISTICS,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).18)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(pstats),
        )
    }
    pub unsafe fn GetDisplayModeList1(
        &self,
        enumformat: DXGI_FORMAT,
        flags: u32,
        pnummodes: *mut u32,
        pdesc: *mut DXGI_MODE_DESC1,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).19)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(enumformat),
            ::std::mem::transmute(flags),
            ::std::mem::transmute(pnummodes),
            ::std::mem::transmute(pdesc),
        )
    }
    pub unsafe fn FindClosestMatchingMode1<
        'a,
        T2__: ::windows::IntoParam<'a, ::windows::IUnknown>,
    >(
        &self,
        pmodetomatch: *const DXGI_MODE_DESC1,
        pclosestmatch: *mut DXGI_MODE_DESC1,
        pconcerneddevice: T2__,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).20)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(pmodetomatch),
            ::std::mem::transmute(pclosestmatch),
            pconcerneddevice.into_param().abi(),
        )
    }
    pub unsafe fn GetDisplaySurfaceData1<'a, T0__: ::windows::IntoParam<'a, IDXGIResource>>(
        &self,
        pdestination: T0__,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).21)(
            ::windows::Abi::abi(self),
            pdestination.into_param().abi(),
        )
    }
    pub unsafe fn DuplicateOutput<'a, T0__: ::windows::IntoParam<'a, ::windows::IUnknown>>(
        &self,
        pdevice: T0__,
        ppoutputduplication: *mut ::std::option::Option<IDXGIOutputDuplication>,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).22)(
            ::windows::Abi::abi(self),
            pdevice.into_param().abi(),
            ::std::mem::transmute(ppoutputduplication),
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
        datasize: u32,
        pdata: *const ::std::ffi::c_void,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        name: *const ::windows::Guid,
        punknown: ::windows::RawPtr,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        name: *const ::windows::Guid,
        pdatasize: *mut u32,
        pdata: *mut ::std::ffi::c_void,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        riid: *const ::windows::Guid,
        ppparent: *mut *mut ::std::ffi::c_void,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        pdesc: *mut DXGI_OUTPUT_DESC,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        enumformat: DXGI_FORMAT,
        flags: u32,
        pnummodes: *mut u32,
        pdesc: *mut DXGI_MODE_DESC,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        pmodetomatch: *const DXGI_MODE_DESC,
        pclosestmatch: *mut DXGI_MODE_DESC,
        pconcerneddevice: ::windows::RawPtr,
    ) -> ::windows::ErrorCode,
    pub unsafe extern "system" fn(this: ::windows::RawPtr) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        pdevice: ::windows::RawPtr,
        exclusive: super::SystemServices::BOOL,
    ) -> ::windows::ErrorCode,
    pub unsafe extern "system" fn(this: ::windows::RawPtr),
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        pgammacaps: *mut DXGI_GAMMA_CONTROL_CAPABILITIES,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        parray: *const DXGI_GAMMA_CONTROL,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        parray: *mut DXGI_GAMMA_CONTROL,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        pscanoutsurface: ::windows::RawPtr,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        pdestination: ::windows::RawPtr,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        pstats: *mut DXGI_FRAME_STATISTICS,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        enumformat: DXGI_FORMAT,
        flags: u32,
        pnummodes: *mut u32,
        pdesc: *mut DXGI_MODE_DESC1,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        pmodetomatch: *const DXGI_MODE_DESC1,
        pclosestmatch: *mut DXGI_MODE_DESC1,
        pconcerneddevice: ::windows::RawPtr,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        pdestination: ::windows::RawPtr,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        pdevice: ::windows::RawPtr,
        ppoutputduplication: *mut ::windows::RawPtr,
    ) -> ::windows::ErrorCode,
);
#[repr(transparent)]
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
impl IDXGIOutput2 {
    pub unsafe fn SetPrivateData(
        &self,
        name: *const ::windows::Guid,
        datasize: u32,
        pdata: *const ::std::ffi::c_void,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).3)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(name),
            ::std::mem::transmute(datasize),
            ::std::mem::transmute(pdata),
        )
    }
    pub unsafe fn SetPrivateDataInterface<
        'a,
        T1__: ::windows::IntoParam<'a, ::windows::IUnknown>,
    >(
        &self,
        name: *const ::windows::Guid,
        punknown: T1__,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).4)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(name),
            punknown.into_param().abi(),
        )
    }
    pub unsafe fn GetPrivateData(
        &self,
        name: *const ::windows::Guid,
        pdatasize: *mut u32,
        pdata: *mut ::std::ffi::c_void,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).5)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(name),
            ::std::mem::transmute(pdatasize),
            ::std::mem::transmute(pdata),
        )
    }
    pub unsafe fn GetParent(
        &self,
        riid: *const ::windows::Guid,
        ppparent: *mut *mut ::std::ffi::c_void,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).6)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(riid),
            ::std::mem::transmute(ppparent),
        )
    }
    pub unsafe fn GetDesc(&self, pdesc: *mut DXGI_OUTPUT_DESC) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).7)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(pdesc),
        )
    }
    pub unsafe fn GetDisplayModeList(
        &self,
        enumformat: DXGI_FORMAT,
        flags: u32,
        pnummodes: *mut u32,
        pdesc: *mut DXGI_MODE_DESC,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).8)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(enumformat),
            ::std::mem::transmute(flags),
            ::std::mem::transmute(pnummodes),
            ::std::mem::transmute(pdesc),
        )
    }
    pub unsafe fn FindClosestMatchingMode<
        'a,
        T2__: ::windows::IntoParam<'a, ::windows::IUnknown>,
    >(
        &self,
        pmodetomatch: *const DXGI_MODE_DESC,
        pclosestmatch: *mut DXGI_MODE_DESC,
        pconcerneddevice: T2__,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).9)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(pmodetomatch),
            ::std::mem::transmute(pclosestmatch),
            pconcerneddevice.into_param().abi(),
        )
    }
    pub unsafe fn WaitForVBlank(&self) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).10)(::windows::Abi::abi(self))
    }
    pub unsafe fn TakeOwnership<
        'a,
        T0__: ::windows::IntoParam<'a, ::windows::IUnknown>,
        T1__: ::windows::IntoParam<'a, super::SystemServices::BOOL>,
    >(
        &self,
        pdevice: T0__,
        exclusive: T1__,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).11)(
            ::windows::Abi::abi(self),
            pdevice.into_param().abi(),
            exclusive.into_param().abi(),
        )
    }
    pub unsafe fn ReleaseOwnership(&self) {
        (::windows::Interface::vtable(self).12)(::windows::Abi::abi(self))
    }
    pub unsafe fn GetGammaControlCapabilities(
        &self,
        pgammacaps: *mut DXGI_GAMMA_CONTROL_CAPABILITIES,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).13)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(pgammacaps),
        )
    }
    pub unsafe fn SetGammaControl(
        &self,
        parray: *const DXGI_GAMMA_CONTROL,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).14)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(parray),
        )
    }
    pub unsafe fn GetGammaControl(&self, parray: *mut DXGI_GAMMA_CONTROL) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).15)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(parray),
        )
    }
    pub unsafe fn SetDisplaySurface<'a, T0__: ::windows::IntoParam<'a, IDXGISurface>>(
        &self,
        pscanoutsurface: T0__,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).16)(
            ::windows::Abi::abi(self),
            pscanoutsurface.into_param().abi(),
        )
    }
    pub unsafe fn GetDisplaySurfaceData<'a, T0__: ::windows::IntoParam<'a, IDXGISurface>>(
        &self,
        pdestination: T0__,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).17)(
            ::windows::Abi::abi(self),
            pdestination.into_param().abi(),
        )
    }
    pub unsafe fn GetFrameStatistics(
        &self,
        pstats: *mut DXGI_FRAME_STATISTICS,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).18)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(pstats),
        )
    }
    pub unsafe fn GetDisplayModeList1(
        &self,
        enumformat: DXGI_FORMAT,
        flags: u32,
        pnummodes: *mut u32,
        pdesc: *mut DXGI_MODE_DESC1,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).19)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(enumformat),
            ::std::mem::transmute(flags),
            ::std::mem::transmute(pnummodes),
            ::std::mem::transmute(pdesc),
        )
    }
    pub unsafe fn FindClosestMatchingMode1<
        'a,
        T2__: ::windows::IntoParam<'a, ::windows::IUnknown>,
    >(
        &self,
        pmodetomatch: *const DXGI_MODE_DESC1,
        pclosestmatch: *mut DXGI_MODE_DESC1,
        pconcerneddevice: T2__,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).20)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(pmodetomatch),
            ::std::mem::transmute(pclosestmatch),
            pconcerneddevice.into_param().abi(),
        )
    }
    pub unsafe fn GetDisplaySurfaceData1<'a, T0__: ::windows::IntoParam<'a, IDXGIResource>>(
        &self,
        pdestination: T0__,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).21)(
            ::windows::Abi::abi(self),
            pdestination.into_param().abi(),
        )
    }
    pub unsafe fn DuplicateOutput<'a, T0__: ::windows::IntoParam<'a, ::windows::IUnknown>>(
        &self,
        pdevice: T0__,
        ppoutputduplication: *mut ::std::option::Option<IDXGIOutputDuplication>,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).22)(
            ::windows::Abi::abi(self),
            pdevice.into_param().abi(),
            ::std::mem::transmute(ppoutputduplication),
        )
    }
    pub unsafe fn SupportsOverlays(&self) -> super::SystemServices::BOOL {
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
        datasize: u32,
        pdata: *const ::std::ffi::c_void,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        name: *const ::windows::Guid,
        punknown: ::windows::RawPtr,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        name: *const ::windows::Guid,
        pdatasize: *mut u32,
        pdata: *mut ::std::ffi::c_void,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        riid: *const ::windows::Guid,
        ppparent: *mut *mut ::std::ffi::c_void,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        pdesc: *mut DXGI_OUTPUT_DESC,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        enumformat: DXGI_FORMAT,
        flags: u32,
        pnummodes: *mut u32,
        pdesc: *mut DXGI_MODE_DESC,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        pmodetomatch: *const DXGI_MODE_DESC,
        pclosestmatch: *mut DXGI_MODE_DESC,
        pconcerneddevice: ::windows::RawPtr,
    ) -> ::windows::ErrorCode,
    pub unsafe extern "system" fn(this: ::windows::RawPtr) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        pdevice: ::windows::RawPtr,
        exclusive: super::SystemServices::BOOL,
    ) -> ::windows::ErrorCode,
    pub unsafe extern "system" fn(this: ::windows::RawPtr),
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        pgammacaps: *mut DXGI_GAMMA_CONTROL_CAPABILITIES,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        parray: *const DXGI_GAMMA_CONTROL,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        parray: *mut DXGI_GAMMA_CONTROL,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        pscanoutsurface: ::windows::RawPtr,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        pdestination: ::windows::RawPtr,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        pstats: *mut DXGI_FRAME_STATISTICS,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        enumformat: DXGI_FORMAT,
        flags: u32,
        pnummodes: *mut u32,
        pdesc: *mut DXGI_MODE_DESC1,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        pmodetomatch: *const DXGI_MODE_DESC1,
        pclosestmatch: *mut DXGI_MODE_DESC1,
        pconcerneddevice: ::windows::RawPtr,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        pdestination: ::windows::RawPtr,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        pdevice: ::windows::RawPtr,
        ppoutputduplication: *mut ::windows::RawPtr,
    ) -> ::windows::ErrorCode,
    pub unsafe extern "system" fn(this: ::windows::RawPtr) -> super::SystemServices::BOOL,
);
#[repr(transparent)]
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
impl IDXGIOutput3 {
    pub unsafe fn SetPrivateData(
        &self,
        name: *const ::windows::Guid,
        datasize: u32,
        pdata: *const ::std::ffi::c_void,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).3)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(name),
            ::std::mem::transmute(datasize),
            ::std::mem::transmute(pdata),
        )
    }
    pub unsafe fn SetPrivateDataInterface<
        'a,
        T1__: ::windows::IntoParam<'a, ::windows::IUnknown>,
    >(
        &self,
        name: *const ::windows::Guid,
        punknown: T1__,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).4)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(name),
            punknown.into_param().abi(),
        )
    }
    pub unsafe fn GetPrivateData(
        &self,
        name: *const ::windows::Guid,
        pdatasize: *mut u32,
        pdata: *mut ::std::ffi::c_void,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).5)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(name),
            ::std::mem::transmute(pdatasize),
            ::std::mem::transmute(pdata),
        )
    }
    pub unsafe fn GetParent(
        &self,
        riid: *const ::windows::Guid,
        ppparent: *mut *mut ::std::ffi::c_void,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).6)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(riid),
            ::std::mem::transmute(ppparent),
        )
    }
    pub unsafe fn GetDesc(&self, pdesc: *mut DXGI_OUTPUT_DESC) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).7)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(pdesc),
        )
    }
    pub unsafe fn GetDisplayModeList(
        &self,
        enumformat: DXGI_FORMAT,
        flags: u32,
        pnummodes: *mut u32,
        pdesc: *mut DXGI_MODE_DESC,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).8)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(enumformat),
            ::std::mem::transmute(flags),
            ::std::mem::transmute(pnummodes),
            ::std::mem::transmute(pdesc),
        )
    }
    pub unsafe fn FindClosestMatchingMode<
        'a,
        T2__: ::windows::IntoParam<'a, ::windows::IUnknown>,
    >(
        &self,
        pmodetomatch: *const DXGI_MODE_DESC,
        pclosestmatch: *mut DXGI_MODE_DESC,
        pconcerneddevice: T2__,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).9)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(pmodetomatch),
            ::std::mem::transmute(pclosestmatch),
            pconcerneddevice.into_param().abi(),
        )
    }
    pub unsafe fn WaitForVBlank(&self) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).10)(::windows::Abi::abi(self))
    }
    pub unsafe fn TakeOwnership<
        'a,
        T0__: ::windows::IntoParam<'a, ::windows::IUnknown>,
        T1__: ::windows::IntoParam<'a, super::SystemServices::BOOL>,
    >(
        &self,
        pdevice: T0__,
        exclusive: T1__,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).11)(
            ::windows::Abi::abi(self),
            pdevice.into_param().abi(),
            exclusive.into_param().abi(),
        )
    }
    pub unsafe fn ReleaseOwnership(&self) {
        (::windows::Interface::vtable(self).12)(::windows::Abi::abi(self))
    }
    pub unsafe fn GetGammaControlCapabilities(
        &self,
        pgammacaps: *mut DXGI_GAMMA_CONTROL_CAPABILITIES,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).13)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(pgammacaps),
        )
    }
    pub unsafe fn SetGammaControl(
        &self,
        parray: *const DXGI_GAMMA_CONTROL,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).14)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(parray),
        )
    }
    pub unsafe fn GetGammaControl(&self, parray: *mut DXGI_GAMMA_CONTROL) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).15)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(parray),
        )
    }
    pub unsafe fn SetDisplaySurface<'a, T0__: ::windows::IntoParam<'a, IDXGISurface>>(
        &self,
        pscanoutsurface: T0__,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).16)(
            ::windows::Abi::abi(self),
            pscanoutsurface.into_param().abi(),
        )
    }
    pub unsafe fn GetDisplaySurfaceData<'a, T0__: ::windows::IntoParam<'a, IDXGISurface>>(
        &self,
        pdestination: T0__,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).17)(
            ::windows::Abi::abi(self),
            pdestination.into_param().abi(),
        )
    }
    pub unsafe fn GetFrameStatistics(
        &self,
        pstats: *mut DXGI_FRAME_STATISTICS,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).18)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(pstats),
        )
    }
    pub unsafe fn GetDisplayModeList1(
        &self,
        enumformat: DXGI_FORMAT,
        flags: u32,
        pnummodes: *mut u32,
        pdesc: *mut DXGI_MODE_DESC1,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).19)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(enumformat),
            ::std::mem::transmute(flags),
            ::std::mem::transmute(pnummodes),
            ::std::mem::transmute(pdesc),
        )
    }
    pub unsafe fn FindClosestMatchingMode1<
        'a,
        T2__: ::windows::IntoParam<'a, ::windows::IUnknown>,
    >(
        &self,
        pmodetomatch: *const DXGI_MODE_DESC1,
        pclosestmatch: *mut DXGI_MODE_DESC1,
        pconcerneddevice: T2__,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).20)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(pmodetomatch),
            ::std::mem::transmute(pclosestmatch),
            pconcerneddevice.into_param().abi(),
        )
    }
    pub unsafe fn GetDisplaySurfaceData1<'a, T0__: ::windows::IntoParam<'a, IDXGIResource>>(
        &self,
        pdestination: T0__,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).21)(
            ::windows::Abi::abi(self),
            pdestination.into_param().abi(),
        )
    }
    pub unsafe fn DuplicateOutput<'a, T0__: ::windows::IntoParam<'a, ::windows::IUnknown>>(
        &self,
        pdevice: T0__,
        ppoutputduplication: *mut ::std::option::Option<IDXGIOutputDuplication>,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).22)(
            ::windows::Abi::abi(self),
            pdevice.into_param().abi(),
            ::std::mem::transmute(ppoutputduplication),
        )
    }
    pub unsafe fn SupportsOverlays(&self) -> super::SystemServices::BOOL {
        (::windows::Interface::vtable(self).23)(::windows::Abi::abi(self))
    }
    pub unsafe fn CheckOverlaySupport<'a, T1__: ::windows::IntoParam<'a, ::windows::IUnknown>>(
        &self,
        enumformat: DXGI_FORMAT,
        pconcerneddevice: T1__,
        pflags: *mut u32,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).24)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(enumformat),
            pconcerneddevice.into_param().abi(),
            ::std::mem::transmute(pflags),
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
        datasize: u32,
        pdata: *const ::std::ffi::c_void,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        name: *const ::windows::Guid,
        punknown: ::windows::RawPtr,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        name: *const ::windows::Guid,
        pdatasize: *mut u32,
        pdata: *mut ::std::ffi::c_void,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        riid: *const ::windows::Guid,
        ppparent: *mut *mut ::std::ffi::c_void,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        pdesc: *mut DXGI_OUTPUT_DESC,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        enumformat: DXGI_FORMAT,
        flags: u32,
        pnummodes: *mut u32,
        pdesc: *mut DXGI_MODE_DESC,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        pmodetomatch: *const DXGI_MODE_DESC,
        pclosestmatch: *mut DXGI_MODE_DESC,
        pconcerneddevice: ::windows::RawPtr,
    ) -> ::windows::ErrorCode,
    pub unsafe extern "system" fn(this: ::windows::RawPtr) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        pdevice: ::windows::RawPtr,
        exclusive: super::SystemServices::BOOL,
    ) -> ::windows::ErrorCode,
    pub unsafe extern "system" fn(this: ::windows::RawPtr),
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        pgammacaps: *mut DXGI_GAMMA_CONTROL_CAPABILITIES,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        parray: *const DXGI_GAMMA_CONTROL,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        parray: *mut DXGI_GAMMA_CONTROL,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        pscanoutsurface: ::windows::RawPtr,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        pdestination: ::windows::RawPtr,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        pstats: *mut DXGI_FRAME_STATISTICS,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        enumformat: DXGI_FORMAT,
        flags: u32,
        pnummodes: *mut u32,
        pdesc: *mut DXGI_MODE_DESC1,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        pmodetomatch: *const DXGI_MODE_DESC1,
        pclosestmatch: *mut DXGI_MODE_DESC1,
        pconcerneddevice: ::windows::RawPtr,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        pdestination: ::windows::RawPtr,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        pdevice: ::windows::RawPtr,
        ppoutputduplication: *mut ::windows::RawPtr,
    ) -> ::windows::ErrorCode,
    pub unsafe extern "system" fn(this: ::windows::RawPtr) -> super::SystemServices::BOOL,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        enumformat: DXGI_FORMAT,
        pconcerneddevice: ::windows::RawPtr,
        pflags: *mut u32,
    ) -> ::windows::ErrorCode,
);
#[repr(transparent)]
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
impl IDXGIOutput4 {
    pub unsafe fn SetPrivateData(
        &self,
        name: *const ::windows::Guid,
        datasize: u32,
        pdata: *const ::std::ffi::c_void,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).3)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(name),
            ::std::mem::transmute(datasize),
            ::std::mem::transmute(pdata),
        )
    }
    pub unsafe fn SetPrivateDataInterface<
        'a,
        T1__: ::windows::IntoParam<'a, ::windows::IUnknown>,
    >(
        &self,
        name: *const ::windows::Guid,
        punknown: T1__,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).4)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(name),
            punknown.into_param().abi(),
        )
    }
    pub unsafe fn GetPrivateData(
        &self,
        name: *const ::windows::Guid,
        pdatasize: *mut u32,
        pdata: *mut ::std::ffi::c_void,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).5)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(name),
            ::std::mem::transmute(pdatasize),
            ::std::mem::transmute(pdata),
        )
    }
    pub unsafe fn GetParent(
        &self,
        riid: *const ::windows::Guid,
        ppparent: *mut *mut ::std::ffi::c_void,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).6)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(riid),
            ::std::mem::transmute(ppparent),
        )
    }
    pub unsafe fn GetDesc(&self, pdesc: *mut DXGI_OUTPUT_DESC) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).7)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(pdesc),
        )
    }
    pub unsafe fn GetDisplayModeList(
        &self,
        enumformat: DXGI_FORMAT,
        flags: u32,
        pnummodes: *mut u32,
        pdesc: *mut DXGI_MODE_DESC,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).8)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(enumformat),
            ::std::mem::transmute(flags),
            ::std::mem::transmute(pnummodes),
            ::std::mem::transmute(pdesc),
        )
    }
    pub unsafe fn FindClosestMatchingMode<
        'a,
        T2__: ::windows::IntoParam<'a, ::windows::IUnknown>,
    >(
        &self,
        pmodetomatch: *const DXGI_MODE_DESC,
        pclosestmatch: *mut DXGI_MODE_DESC,
        pconcerneddevice: T2__,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).9)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(pmodetomatch),
            ::std::mem::transmute(pclosestmatch),
            pconcerneddevice.into_param().abi(),
        )
    }
    pub unsafe fn WaitForVBlank(&self) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).10)(::windows::Abi::abi(self))
    }
    pub unsafe fn TakeOwnership<
        'a,
        T0__: ::windows::IntoParam<'a, ::windows::IUnknown>,
        T1__: ::windows::IntoParam<'a, super::SystemServices::BOOL>,
    >(
        &self,
        pdevice: T0__,
        exclusive: T1__,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).11)(
            ::windows::Abi::abi(self),
            pdevice.into_param().abi(),
            exclusive.into_param().abi(),
        )
    }
    pub unsafe fn ReleaseOwnership(&self) {
        (::windows::Interface::vtable(self).12)(::windows::Abi::abi(self))
    }
    pub unsafe fn GetGammaControlCapabilities(
        &self,
        pgammacaps: *mut DXGI_GAMMA_CONTROL_CAPABILITIES,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).13)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(pgammacaps),
        )
    }
    pub unsafe fn SetGammaControl(
        &self,
        parray: *const DXGI_GAMMA_CONTROL,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).14)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(parray),
        )
    }
    pub unsafe fn GetGammaControl(&self, parray: *mut DXGI_GAMMA_CONTROL) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).15)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(parray),
        )
    }
    pub unsafe fn SetDisplaySurface<'a, T0__: ::windows::IntoParam<'a, IDXGISurface>>(
        &self,
        pscanoutsurface: T0__,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).16)(
            ::windows::Abi::abi(self),
            pscanoutsurface.into_param().abi(),
        )
    }
    pub unsafe fn GetDisplaySurfaceData<'a, T0__: ::windows::IntoParam<'a, IDXGISurface>>(
        &self,
        pdestination: T0__,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).17)(
            ::windows::Abi::abi(self),
            pdestination.into_param().abi(),
        )
    }
    pub unsafe fn GetFrameStatistics(
        &self,
        pstats: *mut DXGI_FRAME_STATISTICS,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).18)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(pstats),
        )
    }
    pub unsafe fn GetDisplayModeList1(
        &self,
        enumformat: DXGI_FORMAT,
        flags: u32,
        pnummodes: *mut u32,
        pdesc: *mut DXGI_MODE_DESC1,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).19)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(enumformat),
            ::std::mem::transmute(flags),
            ::std::mem::transmute(pnummodes),
            ::std::mem::transmute(pdesc),
        )
    }
    pub unsafe fn FindClosestMatchingMode1<
        'a,
        T2__: ::windows::IntoParam<'a, ::windows::IUnknown>,
    >(
        &self,
        pmodetomatch: *const DXGI_MODE_DESC1,
        pclosestmatch: *mut DXGI_MODE_DESC1,
        pconcerneddevice: T2__,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).20)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(pmodetomatch),
            ::std::mem::transmute(pclosestmatch),
            pconcerneddevice.into_param().abi(),
        )
    }
    pub unsafe fn GetDisplaySurfaceData1<'a, T0__: ::windows::IntoParam<'a, IDXGIResource>>(
        &self,
        pdestination: T0__,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).21)(
            ::windows::Abi::abi(self),
            pdestination.into_param().abi(),
        )
    }
    pub unsafe fn DuplicateOutput<'a, T0__: ::windows::IntoParam<'a, ::windows::IUnknown>>(
        &self,
        pdevice: T0__,
        ppoutputduplication: *mut ::std::option::Option<IDXGIOutputDuplication>,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).22)(
            ::windows::Abi::abi(self),
            pdevice.into_param().abi(),
            ::std::mem::transmute(ppoutputduplication),
        )
    }
    pub unsafe fn SupportsOverlays(&self) -> super::SystemServices::BOOL {
        (::windows::Interface::vtable(self).23)(::windows::Abi::abi(self))
    }
    pub unsafe fn CheckOverlaySupport<'a, T1__: ::windows::IntoParam<'a, ::windows::IUnknown>>(
        &self,
        enumformat: DXGI_FORMAT,
        pconcerneddevice: T1__,
        pflags: *mut u32,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).24)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(enumformat),
            pconcerneddevice.into_param().abi(),
            ::std::mem::transmute(pflags),
        )
    }
    pub unsafe fn CheckOverlayColorSpaceSupport<
        'a,
        T2__: ::windows::IntoParam<'a, ::windows::IUnknown>,
    >(
        &self,
        format: DXGI_FORMAT,
        colorspace: DXGI_COLOR_SPACE_TYPE,
        pconcerneddevice: T2__,
        pflags: *mut u32,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).25)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(format),
            ::std::mem::transmute(colorspace),
            pconcerneddevice.into_param().abi(),
            ::std::mem::transmute(pflags),
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
        datasize: u32,
        pdata: *const ::std::ffi::c_void,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        name: *const ::windows::Guid,
        punknown: ::windows::RawPtr,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        name: *const ::windows::Guid,
        pdatasize: *mut u32,
        pdata: *mut ::std::ffi::c_void,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        riid: *const ::windows::Guid,
        ppparent: *mut *mut ::std::ffi::c_void,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        pdesc: *mut DXGI_OUTPUT_DESC,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        enumformat: DXGI_FORMAT,
        flags: u32,
        pnummodes: *mut u32,
        pdesc: *mut DXGI_MODE_DESC,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        pmodetomatch: *const DXGI_MODE_DESC,
        pclosestmatch: *mut DXGI_MODE_DESC,
        pconcerneddevice: ::windows::RawPtr,
    ) -> ::windows::ErrorCode,
    pub unsafe extern "system" fn(this: ::windows::RawPtr) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        pdevice: ::windows::RawPtr,
        exclusive: super::SystemServices::BOOL,
    ) -> ::windows::ErrorCode,
    pub unsafe extern "system" fn(this: ::windows::RawPtr),
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        pgammacaps: *mut DXGI_GAMMA_CONTROL_CAPABILITIES,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        parray: *const DXGI_GAMMA_CONTROL,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        parray: *mut DXGI_GAMMA_CONTROL,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        pscanoutsurface: ::windows::RawPtr,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        pdestination: ::windows::RawPtr,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        pstats: *mut DXGI_FRAME_STATISTICS,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        enumformat: DXGI_FORMAT,
        flags: u32,
        pnummodes: *mut u32,
        pdesc: *mut DXGI_MODE_DESC1,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        pmodetomatch: *const DXGI_MODE_DESC1,
        pclosestmatch: *mut DXGI_MODE_DESC1,
        pconcerneddevice: ::windows::RawPtr,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        pdestination: ::windows::RawPtr,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        pdevice: ::windows::RawPtr,
        ppoutputduplication: *mut ::windows::RawPtr,
    ) -> ::windows::ErrorCode,
    pub unsafe extern "system" fn(this: ::windows::RawPtr) -> super::SystemServices::BOOL,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        enumformat: DXGI_FORMAT,
        pconcerneddevice: ::windows::RawPtr,
        pflags: *mut u32,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        format: DXGI_FORMAT,
        colorspace: DXGI_COLOR_SPACE_TYPE,
        pconcerneddevice: ::windows::RawPtr,
        pflags: *mut u32,
    ) -> ::windows::ErrorCode,
);
#[repr(transparent)]
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
impl IDXGIOutput5 {
    pub unsafe fn SetPrivateData(
        &self,
        name: *const ::windows::Guid,
        datasize: u32,
        pdata: *const ::std::ffi::c_void,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).3)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(name),
            ::std::mem::transmute(datasize),
            ::std::mem::transmute(pdata),
        )
    }
    pub unsafe fn SetPrivateDataInterface<
        'a,
        T1__: ::windows::IntoParam<'a, ::windows::IUnknown>,
    >(
        &self,
        name: *const ::windows::Guid,
        punknown: T1__,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).4)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(name),
            punknown.into_param().abi(),
        )
    }
    pub unsafe fn GetPrivateData(
        &self,
        name: *const ::windows::Guid,
        pdatasize: *mut u32,
        pdata: *mut ::std::ffi::c_void,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).5)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(name),
            ::std::mem::transmute(pdatasize),
            ::std::mem::transmute(pdata),
        )
    }
    pub unsafe fn GetParent(
        &self,
        riid: *const ::windows::Guid,
        ppparent: *mut *mut ::std::ffi::c_void,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).6)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(riid),
            ::std::mem::transmute(ppparent),
        )
    }
    pub unsafe fn GetDesc(&self, pdesc: *mut DXGI_OUTPUT_DESC) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).7)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(pdesc),
        )
    }
    pub unsafe fn GetDisplayModeList(
        &self,
        enumformat: DXGI_FORMAT,
        flags: u32,
        pnummodes: *mut u32,
        pdesc: *mut DXGI_MODE_DESC,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).8)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(enumformat),
            ::std::mem::transmute(flags),
            ::std::mem::transmute(pnummodes),
            ::std::mem::transmute(pdesc),
        )
    }
    pub unsafe fn FindClosestMatchingMode<
        'a,
        T2__: ::windows::IntoParam<'a, ::windows::IUnknown>,
    >(
        &self,
        pmodetomatch: *const DXGI_MODE_DESC,
        pclosestmatch: *mut DXGI_MODE_DESC,
        pconcerneddevice: T2__,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).9)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(pmodetomatch),
            ::std::mem::transmute(pclosestmatch),
            pconcerneddevice.into_param().abi(),
        )
    }
    pub unsafe fn WaitForVBlank(&self) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).10)(::windows::Abi::abi(self))
    }
    pub unsafe fn TakeOwnership<
        'a,
        T0__: ::windows::IntoParam<'a, ::windows::IUnknown>,
        T1__: ::windows::IntoParam<'a, super::SystemServices::BOOL>,
    >(
        &self,
        pdevice: T0__,
        exclusive: T1__,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).11)(
            ::windows::Abi::abi(self),
            pdevice.into_param().abi(),
            exclusive.into_param().abi(),
        )
    }
    pub unsafe fn ReleaseOwnership(&self) {
        (::windows::Interface::vtable(self).12)(::windows::Abi::abi(self))
    }
    pub unsafe fn GetGammaControlCapabilities(
        &self,
        pgammacaps: *mut DXGI_GAMMA_CONTROL_CAPABILITIES,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).13)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(pgammacaps),
        )
    }
    pub unsafe fn SetGammaControl(
        &self,
        parray: *const DXGI_GAMMA_CONTROL,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).14)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(parray),
        )
    }
    pub unsafe fn GetGammaControl(&self, parray: *mut DXGI_GAMMA_CONTROL) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).15)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(parray),
        )
    }
    pub unsafe fn SetDisplaySurface<'a, T0__: ::windows::IntoParam<'a, IDXGISurface>>(
        &self,
        pscanoutsurface: T0__,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).16)(
            ::windows::Abi::abi(self),
            pscanoutsurface.into_param().abi(),
        )
    }
    pub unsafe fn GetDisplaySurfaceData<'a, T0__: ::windows::IntoParam<'a, IDXGISurface>>(
        &self,
        pdestination: T0__,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).17)(
            ::windows::Abi::abi(self),
            pdestination.into_param().abi(),
        )
    }
    pub unsafe fn GetFrameStatistics(
        &self,
        pstats: *mut DXGI_FRAME_STATISTICS,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).18)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(pstats),
        )
    }
    pub unsafe fn GetDisplayModeList1(
        &self,
        enumformat: DXGI_FORMAT,
        flags: u32,
        pnummodes: *mut u32,
        pdesc: *mut DXGI_MODE_DESC1,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).19)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(enumformat),
            ::std::mem::transmute(flags),
            ::std::mem::transmute(pnummodes),
            ::std::mem::transmute(pdesc),
        )
    }
    pub unsafe fn FindClosestMatchingMode1<
        'a,
        T2__: ::windows::IntoParam<'a, ::windows::IUnknown>,
    >(
        &self,
        pmodetomatch: *const DXGI_MODE_DESC1,
        pclosestmatch: *mut DXGI_MODE_DESC1,
        pconcerneddevice: T2__,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).20)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(pmodetomatch),
            ::std::mem::transmute(pclosestmatch),
            pconcerneddevice.into_param().abi(),
        )
    }
    pub unsafe fn GetDisplaySurfaceData1<'a, T0__: ::windows::IntoParam<'a, IDXGIResource>>(
        &self,
        pdestination: T0__,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).21)(
            ::windows::Abi::abi(self),
            pdestination.into_param().abi(),
        )
    }
    pub unsafe fn DuplicateOutput<'a, T0__: ::windows::IntoParam<'a, ::windows::IUnknown>>(
        &self,
        pdevice: T0__,
        ppoutputduplication: *mut ::std::option::Option<IDXGIOutputDuplication>,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).22)(
            ::windows::Abi::abi(self),
            pdevice.into_param().abi(),
            ::std::mem::transmute(ppoutputduplication),
        )
    }
    pub unsafe fn SupportsOverlays(&self) -> super::SystemServices::BOOL {
        (::windows::Interface::vtable(self).23)(::windows::Abi::abi(self))
    }
    pub unsafe fn CheckOverlaySupport<'a, T1__: ::windows::IntoParam<'a, ::windows::IUnknown>>(
        &self,
        enumformat: DXGI_FORMAT,
        pconcerneddevice: T1__,
        pflags: *mut u32,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).24)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(enumformat),
            pconcerneddevice.into_param().abi(),
            ::std::mem::transmute(pflags),
        )
    }
    pub unsafe fn CheckOverlayColorSpaceSupport<
        'a,
        T2__: ::windows::IntoParam<'a, ::windows::IUnknown>,
    >(
        &self,
        format: DXGI_FORMAT,
        colorspace: DXGI_COLOR_SPACE_TYPE,
        pconcerneddevice: T2__,
        pflags: *mut u32,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).25)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(format),
            ::std::mem::transmute(colorspace),
            pconcerneddevice.into_param().abi(),
            ::std::mem::transmute(pflags),
        )
    }
    pub unsafe fn DuplicateOutput1<'a, T0__: ::windows::IntoParam<'a, ::windows::IUnknown>>(
        &self,
        pdevice: T0__,
        flags: u32,
        supportedformatscount: u32,
        psupportedformats: *const DXGI_FORMAT,
        ppoutputduplication: *mut ::std::option::Option<IDXGIOutputDuplication>,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).26)(
            ::windows::Abi::abi(self),
            pdevice.into_param().abi(),
            ::std::mem::transmute(flags),
            ::std::mem::transmute(supportedformatscount),
            ::std::mem::transmute(psupportedformats),
            ::std::mem::transmute(ppoutputduplication),
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
        datasize: u32,
        pdata: *const ::std::ffi::c_void,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        name: *const ::windows::Guid,
        punknown: ::windows::RawPtr,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        name: *const ::windows::Guid,
        pdatasize: *mut u32,
        pdata: *mut ::std::ffi::c_void,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        riid: *const ::windows::Guid,
        ppparent: *mut *mut ::std::ffi::c_void,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        pdesc: *mut DXGI_OUTPUT_DESC,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        enumformat: DXGI_FORMAT,
        flags: u32,
        pnummodes: *mut u32,
        pdesc: *mut DXGI_MODE_DESC,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        pmodetomatch: *const DXGI_MODE_DESC,
        pclosestmatch: *mut DXGI_MODE_DESC,
        pconcerneddevice: ::windows::RawPtr,
    ) -> ::windows::ErrorCode,
    pub unsafe extern "system" fn(this: ::windows::RawPtr) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        pdevice: ::windows::RawPtr,
        exclusive: super::SystemServices::BOOL,
    ) -> ::windows::ErrorCode,
    pub unsafe extern "system" fn(this: ::windows::RawPtr),
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        pgammacaps: *mut DXGI_GAMMA_CONTROL_CAPABILITIES,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        parray: *const DXGI_GAMMA_CONTROL,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        parray: *mut DXGI_GAMMA_CONTROL,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        pscanoutsurface: ::windows::RawPtr,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        pdestination: ::windows::RawPtr,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        pstats: *mut DXGI_FRAME_STATISTICS,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        enumformat: DXGI_FORMAT,
        flags: u32,
        pnummodes: *mut u32,
        pdesc: *mut DXGI_MODE_DESC1,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        pmodetomatch: *const DXGI_MODE_DESC1,
        pclosestmatch: *mut DXGI_MODE_DESC1,
        pconcerneddevice: ::windows::RawPtr,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        pdestination: ::windows::RawPtr,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        pdevice: ::windows::RawPtr,
        ppoutputduplication: *mut ::windows::RawPtr,
    ) -> ::windows::ErrorCode,
    pub unsafe extern "system" fn(this: ::windows::RawPtr) -> super::SystemServices::BOOL,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        enumformat: DXGI_FORMAT,
        pconcerneddevice: ::windows::RawPtr,
        pflags: *mut u32,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        format: DXGI_FORMAT,
        colorspace: DXGI_COLOR_SPACE_TYPE,
        pconcerneddevice: ::windows::RawPtr,
        pflags: *mut u32,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        pdevice: ::windows::RawPtr,
        flags: u32,
        supportedformatscount: u32,
        psupportedformats: *const DXGI_FORMAT,
        ppoutputduplication: *mut ::windows::RawPtr,
    ) -> ::windows::ErrorCode,
);
#[repr(transparent)]
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
impl IDXGIOutput6 {
    pub unsafe fn SetPrivateData(
        &self,
        name: *const ::windows::Guid,
        datasize: u32,
        pdata: *const ::std::ffi::c_void,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).3)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(name),
            ::std::mem::transmute(datasize),
            ::std::mem::transmute(pdata),
        )
    }
    pub unsafe fn SetPrivateDataInterface<
        'a,
        T1__: ::windows::IntoParam<'a, ::windows::IUnknown>,
    >(
        &self,
        name: *const ::windows::Guid,
        punknown: T1__,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).4)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(name),
            punknown.into_param().abi(),
        )
    }
    pub unsafe fn GetPrivateData(
        &self,
        name: *const ::windows::Guid,
        pdatasize: *mut u32,
        pdata: *mut ::std::ffi::c_void,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).5)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(name),
            ::std::mem::transmute(pdatasize),
            ::std::mem::transmute(pdata),
        )
    }
    pub unsafe fn GetParent(
        &self,
        riid: *const ::windows::Guid,
        ppparent: *mut *mut ::std::ffi::c_void,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).6)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(riid),
            ::std::mem::transmute(ppparent),
        )
    }
    pub unsafe fn GetDesc(&self, pdesc: *mut DXGI_OUTPUT_DESC) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).7)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(pdesc),
        )
    }
    pub unsafe fn GetDisplayModeList(
        &self,
        enumformat: DXGI_FORMAT,
        flags: u32,
        pnummodes: *mut u32,
        pdesc: *mut DXGI_MODE_DESC,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).8)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(enumformat),
            ::std::mem::transmute(flags),
            ::std::mem::transmute(pnummodes),
            ::std::mem::transmute(pdesc),
        )
    }
    pub unsafe fn FindClosestMatchingMode<
        'a,
        T2__: ::windows::IntoParam<'a, ::windows::IUnknown>,
    >(
        &self,
        pmodetomatch: *const DXGI_MODE_DESC,
        pclosestmatch: *mut DXGI_MODE_DESC,
        pconcerneddevice: T2__,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).9)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(pmodetomatch),
            ::std::mem::transmute(pclosestmatch),
            pconcerneddevice.into_param().abi(),
        )
    }
    pub unsafe fn WaitForVBlank(&self) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).10)(::windows::Abi::abi(self))
    }
    pub unsafe fn TakeOwnership<
        'a,
        T0__: ::windows::IntoParam<'a, ::windows::IUnknown>,
        T1__: ::windows::IntoParam<'a, super::SystemServices::BOOL>,
    >(
        &self,
        pdevice: T0__,
        exclusive: T1__,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).11)(
            ::windows::Abi::abi(self),
            pdevice.into_param().abi(),
            exclusive.into_param().abi(),
        )
    }
    pub unsafe fn ReleaseOwnership(&self) {
        (::windows::Interface::vtable(self).12)(::windows::Abi::abi(self))
    }
    pub unsafe fn GetGammaControlCapabilities(
        &self,
        pgammacaps: *mut DXGI_GAMMA_CONTROL_CAPABILITIES,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).13)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(pgammacaps),
        )
    }
    pub unsafe fn SetGammaControl(
        &self,
        parray: *const DXGI_GAMMA_CONTROL,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).14)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(parray),
        )
    }
    pub unsafe fn GetGammaControl(&self, parray: *mut DXGI_GAMMA_CONTROL) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).15)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(parray),
        )
    }
    pub unsafe fn SetDisplaySurface<'a, T0__: ::windows::IntoParam<'a, IDXGISurface>>(
        &self,
        pscanoutsurface: T0__,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).16)(
            ::windows::Abi::abi(self),
            pscanoutsurface.into_param().abi(),
        )
    }
    pub unsafe fn GetDisplaySurfaceData<'a, T0__: ::windows::IntoParam<'a, IDXGISurface>>(
        &self,
        pdestination: T0__,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).17)(
            ::windows::Abi::abi(self),
            pdestination.into_param().abi(),
        )
    }
    pub unsafe fn GetFrameStatistics(
        &self,
        pstats: *mut DXGI_FRAME_STATISTICS,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).18)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(pstats),
        )
    }
    pub unsafe fn GetDisplayModeList1(
        &self,
        enumformat: DXGI_FORMAT,
        flags: u32,
        pnummodes: *mut u32,
        pdesc: *mut DXGI_MODE_DESC1,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).19)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(enumformat),
            ::std::mem::transmute(flags),
            ::std::mem::transmute(pnummodes),
            ::std::mem::transmute(pdesc),
        )
    }
    pub unsafe fn FindClosestMatchingMode1<
        'a,
        T2__: ::windows::IntoParam<'a, ::windows::IUnknown>,
    >(
        &self,
        pmodetomatch: *const DXGI_MODE_DESC1,
        pclosestmatch: *mut DXGI_MODE_DESC1,
        pconcerneddevice: T2__,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).20)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(pmodetomatch),
            ::std::mem::transmute(pclosestmatch),
            pconcerneddevice.into_param().abi(),
        )
    }
    pub unsafe fn GetDisplaySurfaceData1<'a, T0__: ::windows::IntoParam<'a, IDXGIResource>>(
        &self,
        pdestination: T0__,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).21)(
            ::windows::Abi::abi(self),
            pdestination.into_param().abi(),
        )
    }
    pub unsafe fn DuplicateOutput<'a, T0__: ::windows::IntoParam<'a, ::windows::IUnknown>>(
        &self,
        pdevice: T0__,
        ppoutputduplication: *mut ::std::option::Option<IDXGIOutputDuplication>,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).22)(
            ::windows::Abi::abi(self),
            pdevice.into_param().abi(),
            ::std::mem::transmute(ppoutputduplication),
        )
    }
    pub unsafe fn SupportsOverlays(&self) -> super::SystemServices::BOOL {
        (::windows::Interface::vtable(self).23)(::windows::Abi::abi(self))
    }
    pub unsafe fn CheckOverlaySupport<'a, T1__: ::windows::IntoParam<'a, ::windows::IUnknown>>(
        &self,
        enumformat: DXGI_FORMAT,
        pconcerneddevice: T1__,
        pflags: *mut u32,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).24)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(enumformat),
            pconcerneddevice.into_param().abi(),
            ::std::mem::transmute(pflags),
        )
    }
    pub unsafe fn CheckOverlayColorSpaceSupport<
        'a,
        T2__: ::windows::IntoParam<'a, ::windows::IUnknown>,
    >(
        &self,
        format: DXGI_FORMAT,
        colorspace: DXGI_COLOR_SPACE_TYPE,
        pconcerneddevice: T2__,
        pflags: *mut u32,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).25)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(format),
            ::std::mem::transmute(colorspace),
            pconcerneddevice.into_param().abi(),
            ::std::mem::transmute(pflags),
        )
    }
    pub unsafe fn DuplicateOutput1<'a, T0__: ::windows::IntoParam<'a, ::windows::IUnknown>>(
        &self,
        pdevice: T0__,
        flags: u32,
        supportedformatscount: u32,
        psupportedformats: *const DXGI_FORMAT,
        ppoutputduplication: *mut ::std::option::Option<IDXGIOutputDuplication>,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).26)(
            ::windows::Abi::abi(self),
            pdevice.into_param().abi(),
            ::std::mem::transmute(flags),
            ::std::mem::transmute(supportedformatscount),
            ::std::mem::transmute(psupportedformats),
            ::std::mem::transmute(ppoutputduplication),
        )
    }
    pub unsafe fn GetDesc1(&self, pdesc: *mut DXGI_OUTPUT_DESC1) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).27)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(pdesc),
        )
    }
    pub unsafe fn CheckHardwareCompositionSupport(&self, pflags: *mut u32) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).28)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(pflags),
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
        datasize: u32,
        pdata: *const ::std::ffi::c_void,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        name: *const ::windows::Guid,
        punknown: ::windows::RawPtr,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        name: *const ::windows::Guid,
        pdatasize: *mut u32,
        pdata: *mut ::std::ffi::c_void,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        riid: *const ::windows::Guid,
        ppparent: *mut *mut ::std::ffi::c_void,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        pdesc: *mut DXGI_OUTPUT_DESC,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        enumformat: DXGI_FORMAT,
        flags: u32,
        pnummodes: *mut u32,
        pdesc: *mut DXGI_MODE_DESC,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        pmodetomatch: *const DXGI_MODE_DESC,
        pclosestmatch: *mut DXGI_MODE_DESC,
        pconcerneddevice: ::windows::RawPtr,
    ) -> ::windows::ErrorCode,
    pub unsafe extern "system" fn(this: ::windows::RawPtr) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        pdevice: ::windows::RawPtr,
        exclusive: super::SystemServices::BOOL,
    ) -> ::windows::ErrorCode,
    pub unsafe extern "system" fn(this: ::windows::RawPtr),
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        pgammacaps: *mut DXGI_GAMMA_CONTROL_CAPABILITIES,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        parray: *const DXGI_GAMMA_CONTROL,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        parray: *mut DXGI_GAMMA_CONTROL,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        pscanoutsurface: ::windows::RawPtr,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        pdestination: ::windows::RawPtr,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        pstats: *mut DXGI_FRAME_STATISTICS,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        enumformat: DXGI_FORMAT,
        flags: u32,
        pnummodes: *mut u32,
        pdesc: *mut DXGI_MODE_DESC1,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        pmodetomatch: *const DXGI_MODE_DESC1,
        pclosestmatch: *mut DXGI_MODE_DESC1,
        pconcerneddevice: ::windows::RawPtr,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        pdestination: ::windows::RawPtr,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        pdevice: ::windows::RawPtr,
        ppoutputduplication: *mut ::windows::RawPtr,
    ) -> ::windows::ErrorCode,
    pub unsafe extern "system" fn(this: ::windows::RawPtr) -> super::SystemServices::BOOL,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        enumformat: DXGI_FORMAT,
        pconcerneddevice: ::windows::RawPtr,
        pflags: *mut u32,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        format: DXGI_FORMAT,
        colorspace: DXGI_COLOR_SPACE_TYPE,
        pconcerneddevice: ::windows::RawPtr,
        pflags: *mut u32,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        pdevice: ::windows::RawPtr,
        flags: u32,
        supportedformatscount: u32,
        psupportedformats: *const DXGI_FORMAT,
        ppoutputduplication: *mut ::windows::RawPtr,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        pdesc: *mut DXGI_OUTPUT_DESC1,
    ) -> ::windows::ErrorCode,
    pub unsafe extern "system" fn(this: ::windows::RawPtr, pflags: *mut u32) -> ::windows::ErrorCode,
);
#[repr(transparent)]
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
impl IDXGIOutputDuplication {
    pub unsafe fn SetPrivateData(
        &self,
        name: *const ::windows::Guid,
        datasize: u32,
        pdata: *const ::std::ffi::c_void,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).3)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(name),
            ::std::mem::transmute(datasize),
            ::std::mem::transmute(pdata),
        )
    }
    pub unsafe fn SetPrivateDataInterface<
        'a,
        T1__: ::windows::IntoParam<'a, ::windows::IUnknown>,
    >(
        &self,
        name: *const ::windows::Guid,
        punknown: T1__,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).4)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(name),
            punknown.into_param().abi(),
        )
    }
    pub unsafe fn GetPrivateData(
        &self,
        name: *const ::windows::Guid,
        pdatasize: *mut u32,
        pdata: *mut ::std::ffi::c_void,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).5)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(name),
            ::std::mem::transmute(pdatasize),
            ::std::mem::transmute(pdata),
        )
    }
    pub unsafe fn GetParent(
        &self,
        riid: *const ::windows::Guid,
        ppparent: *mut *mut ::std::ffi::c_void,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).6)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(riid),
            ::std::mem::transmute(ppparent),
        )
    }
    pub unsafe fn GetDesc(&self, pdesc: *mut DXGI_OUTDUPL_DESC) {
        (::windows::Interface::vtable(self).7)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(pdesc),
        )
    }
    pub unsafe fn AcquireNextFrame(
        &self,
        timeoutinmilliseconds: u32,
        pframeinfo: *mut DXGI_OUTDUPL_FRAME_INFO,
        ppdesktopresource: *mut ::std::option::Option<IDXGIResource>,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).8)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(timeoutinmilliseconds),
            ::std::mem::transmute(pframeinfo),
            ::std::mem::transmute(ppdesktopresource),
        )
    }
    pub unsafe fn GetFrameDirtyRects(
        &self,
        dirtyrectsbuffersize: u32,
        pdirtyrectsbuffer: *mut super::DisplayDevices::RECT,
        pdirtyrectsbuffersizerequired: *mut u32,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).9)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(dirtyrectsbuffersize),
            ::std::mem::transmute(pdirtyrectsbuffer),
            ::std::mem::transmute(pdirtyrectsbuffersizerequired),
        )
    }
    pub unsafe fn GetFrameMoveRects(
        &self,
        moverectsbuffersize: u32,
        pmoverectbuffer: *mut DXGI_OUTDUPL_MOVE_RECT,
        pmoverectsbuffersizerequired: *mut u32,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).10)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(moverectsbuffersize),
            ::std::mem::transmute(pmoverectbuffer),
            ::std::mem::transmute(pmoverectsbuffersizerequired),
        )
    }
    pub unsafe fn GetFramePointerShape(
        &self,
        pointershapebuffersize: u32,
        ppointershapebuffer: *mut ::std::ffi::c_void,
        ppointershapebuffersizerequired: *mut u32,
        ppointershapeinfo: *mut DXGI_OUTDUPL_POINTER_SHAPE_INFO,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).11)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(pointershapebuffersize),
            ::std::mem::transmute(ppointershapebuffer),
            ::std::mem::transmute(ppointershapebuffersizerequired),
            ::std::mem::transmute(ppointershapeinfo),
        )
    }
    pub unsafe fn MapDesktopSurface(
        &self,
        plockedrect: *mut DXGI_MAPPED_RECT,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).12)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(plockedrect),
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
        datasize: u32,
        pdata: *const ::std::ffi::c_void,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        name: *const ::windows::Guid,
        punknown: ::windows::RawPtr,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        name: *const ::windows::Guid,
        pdatasize: *mut u32,
        pdata: *mut ::std::ffi::c_void,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        riid: *const ::windows::Guid,
        ppparent: *mut *mut ::std::ffi::c_void,
    ) -> ::windows::ErrorCode,
    pub unsafe extern "system" fn(this: ::windows::RawPtr, pdesc: *mut DXGI_OUTDUPL_DESC),
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        timeoutinmilliseconds: u32,
        pframeinfo: *mut DXGI_OUTDUPL_FRAME_INFO,
        ppdesktopresource: *mut ::windows::RawPtr,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        dirtyrectsbuffersize: u32,
        pdirtyrectsbuffer: *mut super::DisplayDevices::RECT,
        pdirtyrectsbuffersizerequired: *mut u32,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        moverectsbuffersize: u32,
        pmoverectbuffer: *mut DXGI_OUTDUPL_MOVE_RECT,
        pmoverectsbuffersizerequired: *mut u32,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        pointershapebuffersize: u32,
        ppointershapebuffer: *mut ::std::ffi::c_void,
        ppointershapebuffersizerequired: *mut u32,
        ppointershapeinfo: *mut DXGI_OUTDUPL_POINTER_SHAPE_INFO,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        plockedrect: *mut DXGI_MAPPED_RECT,
    ) -> ::windows::ErrorCode,
    pub unsafe extern "system" fn(this: ::windows::RawPtr) -> ::windows::ErrorCode,
    pub unsafe extern "system" fn(this: ::windows::RawPtr) -> ::windows::ErrorCode,
);
#[repr(transparent)]
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
impl IDXGIResource {
    pub unsafe fn SetPrivateData(
        &self,
        name: *const ::windows::Guid,
        datasize: u32,
        pdata: *const ::std::ffi::c_void,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).3)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(name),
            ::std::mem::transmute(datasize),
            ::std::mem::transmute(pdata),
        )
    }
    pub unsafe fn SetPrivateDataInterface<
        'a,
        T1__: ::windows::IntoParam<'a, ::windows::IUnknown>,
    >(
        &self,
        name: *const ::windows::Guid,
        punknown: T1__,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).4)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(name),
            punknown.into_param().abi(),
        )
    }
    pub unsafe fn GetPrivateData(
        &self,
        name: *const ::windows::Guid,
        pdatasize: *mut u32,
        pdata: *mut ::std::ffi::c_void,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).5)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(name),
            ::std::mem::transmute(pdatasize),
            ::std::mem::transmute(pdata),
        )
    }
    pub unsafe fn GetParent(
        &self,
        riid: *const ::windows::Guid,
        ppparent: *mut *mut ::std::ffi::c_void,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).6)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(riid),
            ::std::mem::transmute(ppparent),
        )
    }
    pub unsafe fn GetDevice(
        &self,
        riid: *const ::windows::Guid,
        ppdevice: *mut *mut ::std::ffi::c_void,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).7)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(riid),
            ::std::mem::transmute(ppdevice),
        )
    }
    pub unsafe fn GetSharedHandle(
        &self,
        psharedhandle: *mut super::SystemServices::HANDLE,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).8)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(psharedhandle),
        )
    }
    pub unsafe fn GetUsage(&self, pusage: *mut u32) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).9)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(pusage),
        )
    }
    pub unsafe fn SetEvictionPriority(&self, evictionpriority: u32) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).10)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(evictionpriority),
        )
    }
    pub unsafe fn GetEvictionPriority(&self, pevictionpriority: *mut u32) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).11)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(pevictionpriority),
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
        datasize: u32,
        pdata: *const ::std::ffi::c_void,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        name: *const ::windows::Guid,
        punknown: ::windows::RawPtr,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        name: *const ::windows::Guid,
        pdatasize: *mut u32,
        pdata: *mut ::std::ffi::c_void,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        riid: *const ::windows::Guid,
        ppparent: *mut *mut ::std::ffi::c_void,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        riid: *const ::windows::Guid,
        ppdevice: *mut *mut ::std::ffi::c_void,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        psharedhandle: *mut super::SystemServices::HANDLE,
    ) -> ::windows::ErrorCode,
    pub unsafe extern "system" fn(this: ::windows::RawPtr, pusage: *mut u32) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        evictionpriority: u32,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        pevictionpriority: *mut u32,
    ) -> ::windows::ErrorCode,
);
#[repr(transparent)]
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
impl IDXGIResource1 {
    pub unsafe fn SetPrivateData(
        &self,
        name: *const ::windows::Guid,
        datasize: u32,
        pdata: *const ::std::ffi::c_void,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).3)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(name),
            ::std::mem::transmute(datasize),
            ::std::mem::transmute(pdata),
        )
    }
    pub unsafe fn SetPrivateDataInterface<
        'a,
        T1__: ::windows::IntoParam<'a, ::windows::IUnknown>,
    >(
        &self,
        name: *const ::windows::Guid,
        punknown: T1__,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).4)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(name),
            punknown.into_param().abi(),
        )
    }
    pub unsafe fn GetPrivateData(
        &self,
        name: *const ::windows::Guid,
        pdatasize: *mut u32,
        pdata: *mut ::std::ffi::c_void,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).5)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(name),
            ::std::mem::transmute(pdatasize),
            ::std::mem::transmute(pdata),
        )
    }
    pub unsafe fn GetParent(
        &self,
        riid: *const ::windows::Guid,
        ppparent: *mut *mut ::std::ffi::c_void,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).6)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(riid),
            ::std::mem::transmute(ppparent),
        )
    }
    pub unsafe fn GetDevice(
        &self,
        riid: *const ::windows::Guid,
        ppdevice: *mut *mut ::std::ffi::c_void,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).7)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(riid),
            ::std::mem::transmute(ppdevice),
        )
    }
    pub unsafe fn GetSharedHandle(
        &self,
        psharedhandle: *mut super::SystemServices::HANDLE,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).8)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(psharedhandle),
        )
    }
    pub unsafe fn GetUsage(&self, pusage: *mut u32) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).9)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(pusage),
        )
    }
    pub unsafe fn SetEvictionPriority(&self, evictionpriority: u32) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).10)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(evictionpriority),
        )
    }
    pub unsafe fn GetEvictionPriority(&self, pevictionpriority: *mut u32) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).11)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(pevictionpriority),
        )
    }
    pub unsafe fn CreateSubresourceSurface(
        &self,
        index: u32,
        ppsurface: *mut ::std::option::Option<IDXGISurface2>,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).12)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(index),
            ::std::mem::transmute(ppsurface),
        )
    }
    pub unsafe fn CreateSharedHandle<
        'a,
        T2__: ::windows::IntoParam<'a, super::SystemServices::PWSTR>,
    >(
        &self,
        pattributes: *const super::SystemServices::SECURITY_ATTRIBUTES,
        dwaccess: u32,
        lpname: T2__,
        phandle: *mut super::SystemServices::HANDLE,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).13)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(pattributes),
            ::std::mem::transmute(dwaccess),
            lpname.into_param().abi(),
            ::std::mem::transmute(phandle),
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
        datasize: u32,
        pdata: *const ::std::ffi::c_void,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        name: *const ::windows::Guid,
        punknown: ::windows::RawPtr,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        name: *const ::windows::Guid,
        pdatasize: *mut u32,
        pdata: *mut ::std::ffi::c_void,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        riid: *const ::windows::Guid,
        ppparent: *mut *mut ::std::ffi::c_void,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        riid: *const ::windows::Guid,
        ppdevice: *mut *mut ::std::ffi::c_void,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        psharedhandle: *mut super::SystemServices::HANDLE,
    ) -> ::windows::ErrorCode,
    pub unsafe extern "system" fn(this: ::windows::RawPtr, pusage: *mut u32) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        evictionpriority: u32,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        pevictionpriority: *mut u32,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        index: u32,
        ppsurface: *mut ::windows::RawPtr,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        pattributes: *const super::SystemServices::SECURITY_ATTRIBUTES,
        dwaccess: u32,
        lpname: super::SystemServices::PWSTR,
        phandle: *mut super::SystemServices::HANDLE,
    ) -> ::windows::ErrorCode,
);
#[repr(transparent)]
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
impl IDXGISurface {
    pub unsafe fn SetPrivateData(
        &self,
        name: *const ::windows::Guid,
        datasize: u32,
        pdata: *const ::std::ffi::c_void,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).3)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(name),
            ::std::mem::transmute(datasize),
            ::std::mem::transmute(pdata),
        )
    }
    pub unsafe fn SetPrivateDataInterface<
        'a,
        T1__: ::windows::IntoParam<'a, ::windows::IUnknown>,
    >(
        &self,
        name: *const ::windows::Guid,
        punknown: T1__,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).4)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(name),
            punknown.into_param().abi(),
        )
    }
    pub unsafe fn GetPrivateData(
        &self,
        name: *const ::windows::Guid,
        pdatasize: *mut u32,
        pdata: *mut ::std::ffi::c_void,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).5)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(name),
            ::std::mem::transmute(pdatasize),
            ::std::mem::transmute(pdata),
        )
    }
    pub unsafe fn GetParent(
        &self,
        riid: *const ::windows::Guid,
        ppparent: *mut *mut ::std::ffi::c_void,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).6)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(riid),
            ::std::mem::transmute(ppparent),
        )
    }
    pub unsafe fn GetDevice(
        &self,
        riid: *const ::windows::Guid,
        ppdevice: *mut *mut ::std::ffi::c_void,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).7)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(riid),
            ::std::mem::transmute(ppdevice),
        )
    }
    pub unsafe fn GetDesc(&self, pdesc: *mut DXGI_SURFACE_DESC) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).8)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(pdesc),
        )
    }
    pub unsafe fn Map(
        &self,
        plockedrect: *mut DXGI_MAPPED_RECT,
        mapflags: u32,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).9)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(plockedrect),
            ::std::mem::transmute(mapflags),
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
        datasize: u32,
        pdata: *const ::std::ffi::c_void,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        name: *const ::windows::Guid,
        punknown: ::windows::RawPtr,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        name: *const ::windows::Guid,
        pdatasize: *mut u32,
        pdata: *mut ::std::ffi::c_void,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        riid: *const ::windows::Guid,
        ppparent: *mut *mut ::std::ffi::c_void,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        riid: *const ::windows::Guid,
        ppdevice: *mut *mut ::std::ffi::c_void,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        pdesc: *mut DXGI_SURFACE_DESC,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        plockedrect: *mut DXGI_MAPPED_RECT,
        mapflags: u32,
    ) -> ::windows::ErrorCode,
    pub unsafe extern "system" fn(this: ::windows::RawPtr) -> ::windows::ErrorCode,
);
#[repr(transparent)]
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
impl IDXGISurface1 {
    pub unsafe fn SetPrivateData(
        &self,
        name: *const ::windows::Guid,
        datasize: u32,
        pdata: *const ::std::ffi::c_void,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).3)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(name),
            ::std::mem::transmute(datasize),
            ::std::mem::transmute(pdata),
        )
    }
    pub unsafe fn SetPrivateDataInterface<
        'a,
        T1__: ::windows::IntoParam<'a, ::windows::IUnknown>,
    >(
        &self,
        name: *const ::windows::Guid,
        punknown: T1__,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).4)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(name),
            punknown.into_param().abi(),
        )
    }
    pub unsafe fn GetPrivateData(
        &self,
        name: *const ::windows::Guid,
        pdatasize: *mut u32,
        pdata: *mut ::std::ffi::c_void,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).5)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(name),
            ::std::mem::transmute(pdatasize),
            ::std::mem::transmute(pdata),
        )
    }
    pub unsafe fn GetParent(
        &self,
        riid: *const ::windows::Guid,
        ppparent: *mut *mut ::std::ffi::c_void,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).6)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(riid),
            ::std::mem::transmute(ppparent),
        )
    }
    pub unsafe fn GetDevice(
        &self,
        riid: *const ::windows::Guid,
        ppdevice: *mut *mut ::std::ffi::c_void,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).7)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(riid),
            ::std::mem::transmute(ppdevice),
        )
    }
    pub unsafe fn GetDesc(&self, pdesc: *mut DXGI_SURFACE_DESC) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).8)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(pdesc),
        )
    }
    pub unsafe fn Map(
        &self,
        plockedrect: *mut DXGI_MAPPED_RECT,
        mapflags: u32,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).9)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(plockedrect),
            ::std::mem::transmute(mapflags),
        )
    }
    pub unsafe fn Unmap(&self) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).10)(::windows::Abi::abi(self))
    }
    pub unsafe fn ReleaseDC(
        &self,
        pdirtyrect: *mut super::DisplayDevices::RECT,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).12)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(pdirtyrect),
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
        datasize: u32,
        pdata: *const ::std::ffi::c_void,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        name: *const ::windows::Guid,
        punknown: ::windows::RawPtr,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        name: *const ::windows::Guid,
        pdatasize: *mut u32,
        pdata: *mut ::std::ffi::c_void,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        riid: *const ::windows::Guid,
        ppparent: *mut *mut ::std::ffi::c_void,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        riid: *const ::windows::Guid,
        ppdevice: *mut *mut ::std::ffi::c_void,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        pdesc: *mut DXGI_SURFACE_DESC,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        plockedrect: *mut DXGI_MAPPED_RECT,
        mapflags: u32,
    ) -> ::windows::ErrorCode,
    pub unsafe extern "system" fn(this: ::windows::RawPtr) -> ::windows::ErrorCode,
    pub unsafe extern "system" fn(),
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        pdirtyrect: *mut super::DisplayDevices::RECT,
    ) -> ::windows::ErrorCode,
);
#[repr(transparent)]
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
impl IDXGISurface2 {
    pub unsafe fn SetPrivateData(
        &self,
        name: *const ::windows::Guid,
        datasize: u32,
        pdata: *const ::std::ffi::c_void,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).3)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(name),
            ::std::mem::transmute(datasize),
            ::std::mem::transmute(pdata),
        )
    }
    pub unsafe fn SetPrivateDataInterface<
        'a,
        T1__: ::windows::IntoParam<'a, ::windows::IUnknown>,
    >(
        &self,
        name: *const ::windows::Guid,
        punknown: T1__,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).4)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(name),
            punknown.into_param().abi(),
        )
    }
    pub unsafe fn GetPrivateData(
        &self,
        name: *const ::windows::Guid,
        pdatasize: *mut u32,
        pdata: *mut ::std::ffi::c_void,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).5)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(name),
            ::std::mem::transmute(pdatasize),
            ::std::mem::transmute(pdata),
        )
    }
    pub unsafe fn GetParent(
        &self,
        riid: *const ::windows::Guid,
        ppparent: *mut *mut ::std::ffi::c_void,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).6)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(riid),
            ::std::mem::transmute(ppparent),
        )
    }
    pub unsafe fn GetDevice(
        &self,
        riid: *const ::windows::Guid,
        ppdevice: *mut *mut ::std::ffi::c_void,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).7)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(riid),
            ::std::mem::transmute(ppdevice),
        )
    }
    pub unsafe fn GetDesc(&self, pdesc: *mut DXGI_SURFACE_DESC) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).8)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(pdesc),
        )
    }
    pub unsafe fn Map(
        &self,
        plockedrect: *mut DXGI_MAPPED_RECT,
        mapflags: u32,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).9)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(plockedrect),
            ::std::mem::transmute(mapflags),
        )
    }
    pub unsafe fn Unmap(&self) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).10)(::windows::Abi::abi(self))
    }
    pub unsafe fn ReleaseDC(
        &self,
        pdirtyrect: *mut super::DisplayDevices::RECT,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).12)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(pdirtyrect),
        )
    }
    pub unsafe fn GetResource(
        &self,
        riid: *const ::windows::Guid,
        ppparentresource: *mut *mut ::std::ffi::c_void,
        psubresourceindex: *mut u32,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).13)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(riid),
            ::std::mem::transmute(ppparentresource),
            ::std::mem::transmute(psubresourceindex),
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
        datasize: u32,
        pdata: *const ::std::ffi::c_void,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        name: *const ::windows::Guid,
        punknown: ::windows::RawPtr,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        name: *const ::windows::Guid,
        pdatasize: *mut u32,
        pdata: *mut ::std::ffi::c_void,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        riid: *const ::windows::Guid,
        ppparent: *mut *mut ::std::ffi::c_void,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        riid: *const ::windows::Guid,
        ppdevice: *mut *mut ::std::ffi::c_void,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        pdesc: *mut DXGI_SURFACE_DESC,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        plockedrect: *mut DXGI_MAPPED_RECT,
        mapflags: u32,
    ) -> ::windows::ErrorCode,
    pub unsafe extern "system" fn(this: ::windows::RawPtr) -> ::windows::ErrorCode,
    pub unsafe extern "system" fn(),
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        pdirtyrect: *mut super::DisplayDevices::RECT,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        riid: *const ::windows::Guid,
        ppparentresource: *mut *mut ::std::ffi::c_void,
        psubresourceindex: *mut u32,
    ) -> ::windows::ErrorCode,
);
#[repr(transparent)]
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
impl IDXGISwapChain {
    pub unsafe fn SetPrivateData(
        &self,
        name: *const ::windows::Guid,
        datasize: u32,
        pdata: *const ::std::ffi::c_void,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).3)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(name),
            ::std::mem::transmute(datasize),
            ::std::mem::transmute(pdata),
        )
    }
    pub unsafe fn SetPrivateDataInterface<
        'a,
        T1__: ::windows::IntoParam<'a, ::windows::IUnknown>,
    >(
        &self,
        name: *const ::windows::Guid,
        punknown: T1__,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).4)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(name),
            punknown.into_param().abi(),
        )
    }
    pub unsafe fn GetPrivateData(
        &self,
        name: *const ::windows::Guid,
        pdatasize: *mut u32,
        pdata: *mut ::std::ffi::c_void,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).5)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(name),
            ::std::mem::transmute(pdatasize),
            ::std::mem::transmute(pdata),
        )
    }
    pub unsafe fn GetParent(
        &self,
        riid: *const ::windows::Guid,
        ppparent: *mut *mut ::std::ffi::c_void,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).6)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(riid),
            ::std::mem::transmute(ppparent),
        )
    }
    pub unsafe fn GetDevice(
        &self,
        riid: *const ::windows::Guid,
        ppdevice: *mut *mut ::std::ffi::c_void,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).7)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(riid),
            ::std::mem::transmute(ppdevice),
        )
    }
    pub unsafe fn Present(&self, syncinterval: u32, flags: u32) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).8)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(syncinterval),
            ::std::mem::transmute(flags),
        )
    }
    pub unsafe fn GetBuffer(
        &self,
        buffer: u32,
        riid: *const ::windows::Guid,
        ppsurface: *mut *mut ::std::ffi::c_void,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).9)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(buffer),
            ::std::mem::transmute(riid),
            ::std::mem::transmute(ppsurface),
        )
    }
    pub unsafe fn SetFullscreenState<
        'a,
        T0__: ::windows::IntoParam<'a, super::SystemServices::BOOL>,
        T1__: ::windows::IntoParam<'a, IDXGIOutput>,
    >(
        &self,
        fullscreen: T0__,
        ptarget: T1__,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).10)(
            ::windows::Abi::abi(self),
            fullscreen.into_param().abi(),
            ptarget.into_param().abi(),
        )
    }
    pub unsafe fn GetFullscreenState(
        &self,
        pfullscreen: *mut super::SystemServices::BOOL,
        pptarget: *mut ::std::option::Option<IDXGIOutput>,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).11)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(pfullscreen),
            ::std::mem::transmute(pptarget),
        )
    }
    pub unsafe fn GetDesc(&self, pdesc: *mut DXGI_SWAP_CHAIN_DESC) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).12)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(pdesc),
        )
    }
    pub unsafe fn ResizeBuffers(
        &self,
        buffercount: u32,
        width: u32,
        height: u32,
        newformat: DXGI_FORMAT,
        swapchainflags: u32,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).13)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(buffercount),
            ::std::mem::transmute(width),
            ::std::mem::transmute(height),
            ::std::mem::transmute(newformat),
            ::std::mem::transmute(swapchainflags),
        )
    }
    pub unsafe fn ResizeTarget(
        &self,
        pnewtargetparameters: *const DXGI_MODE_DESC,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).14)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(pnewtargetparameters),
        )
    }
    pub unsafe fn GetContainingOutput(
        &self,
        ppoutput: *mut ::std::option::Option<IDXGIOutput>,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).15)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(ppoutput),
        )
    }
    pub unsafe fn GetFrameStatistics(
        &self,
        pstats: *mut DXGI_FRAME_STATISTICS,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).16)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(pstats),
        )
    }
    pub unsafe fn GetLastPresentCount(&self, plastpresentcount: *mut u32) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).17)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(plastpresentcount),
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
        datasize: u32,
        pdata: *const ::std::ffi::c_void,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        name: *const ::windows::Guid,
        punknown: ::windows::RawPtr,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        name: *const ::windows::Guid,
        pdatasize: *mut u32,
        pdata: *mut ::std::ffi::c_void,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        riid: *const ::windows::Guid,
        ppparent: *mut *mut ::std::ffi::c_void,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        riid: *const ::windows::Guid,
        ppdevice: *mut *mut ::std::ffi::c_void,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        syncinterval: u32,
        flags: u32,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        buffer: u32,
        riid: *const ::windows::Guid,
        ppsurface: *mut *mut ::std::ffi::c_void,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        fullscreen: super::SystemServices::BOOL,
        ptarget: ::windows::RawPtr,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        pfullscreen: *mut super::SystemServices::BOOL,
        pptarget: *mut ::windows::RawPtr,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        pdesc: *mut DXGI_SWAP_CHAIN_DESC,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        buffercount: u32,
        width: u32,
        height: u32,
        newformat: DXGI_FORMAT,
        swapchainflags: u32,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        pnewtargetparameters: *const DXGI_MODE_DESC,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        ppoutput: *mut ::windows::RawPtr,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        pstats: *mut DXGI_FRAME_STATISTICS,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        plastpresentcount: *mut u32,
    ) -> ::windows::ErrorCode,
);
#[repr(transparent)]
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
impl IDXGISwapChain1 {
    pub unsafe fn SetPrivateData(
        &self,
        name: *const ::windows::Guid,
        datasize: u32,
        pdata: *const ::std::ffi::c_void,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).3)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(name),
            ::std::mem::transmute(datasize),
            ::std::mem::transmute(pdata),
        )
    }
    pub unsafe fn SetPrivateDataInterface<
        'a,
        T1__: ::windows::IntoParam<'a, ::windows::IUnknown>,
    >(
        &self,
        name: *const ::windows::Guid,
        punknown: T1__,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).4)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(name),
            punknown.into_param().abi(),
        )
    }
    pub unsafe fn GetPrivateData(
        &self,
        name: *const ::windows::Guid,
        pdatasize: *mut u32,
        pdata: *mut ::std::ffi::c_void,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).5)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(name),
            ::std::mem::transmute(pdatasize),
            ::std::mem::transmute(pdata),
        )
    }
    pub unsafe fn GetParent(
        &self,
        riid: *const ::windows::Guid,
        ppparent: *mut *mut ::std::ffi::c_void,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).6)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(riid),
            ::std::mem::transmute(ppparent),
        )
    }
    pub unsafe fn GetDevice(
        &self,
        riid: *const ::windows::Guid,
        ppdevice: *mut *mut ::std::ffi::c_void,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).7)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(riid),
            ::std::mem::transmute(ppdevice),
        )
    }
    pub unsafe fn Present(&self, syncinterval: u32, flags: u32) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).8)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(syncinterval),
            ::std::mem::transmute(flags),
        )
    }
    pub unsafe fn GetBuffer(
        &self,
        buffer: u32,
        riid: *const ::windows::Guid,
        ppsurface: *mut *mut ::std::ffi::c_void,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).9)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(buffer),
            ::std::mem::transmute(riid),
            ::std::mem::transmute(ppsurface),
        )
    }
    pub unsafe fn SetFullscreenState<
        'a,
        T0__: ::windows::IntoParam<'a, super::SystemServices::BOOL>,
        T1__: ::windows::IntoParam<'a, IDXGIOutput>,
    >(
        &self,
        fullscreen: T0__,
        ptarget: T1__,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).10)(
            ::windows::Abi::abi(self),
            fullscreen.into_param().abi(),
            ptarget.into_param().abi(),
        )
    }
    pub unsafe fn GetFullscreenState(
        &self,
        pfullscreen: *mut super::SystemServices::BOOL,
        pptarget: *mut ::std::option::Option<IDXGIOutput>,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).11)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(pfullscreen),
            ::std::mem::transmute(pptarget),
        )
    }
    pub unsafe fn GetDesc(&self, pdesc: *mut DXGI_SWAP_CHAIN_DESC) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).12)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(pdesc),
        )
    }
    pub unsafe fn ResizeBuffers(
        &self,
        buffercount: u32,
        width: u32,
        height: u32,
        newformat: DXGI_FORMAT,
        swapchainflags: u32,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).13)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(buffercount),
            ::std::mem::transmute(width),
            ::std::mem::transmute(height),
            ::std::mem::transmute(newformat),
            ::std::mem::transmute(swapchainflags),
        )
    }
    pub unsafe fn ResizeTarget(
        &self,
        pnewtargetparameters: *const DXGI_MODE_DESC,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).14)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(pnewtargetparameters),
        )
    }
    pub unsafe fn GetContainingOutput(
        &self,
        ppoutput: *mut ::std::option::Option<IDXGIOutput>,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).15)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(ppoutput),
        )
    }
    pub unsafe fn GetFrameStatistics(
        &self,
        pstats: *mut DXGI_FRAME_STATISTICS,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).16)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(pstats),
        )
    }
    pub unsafe fn GetLastPresentCount(&self, plastpresentcount: *mut u32) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).17)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(plastpresentcount),
        )
    }
    pub unsafe fn GetDesc1(&self, pdesc: *mut DXGI_SWAP_CHAIN_DESC1) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).18)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(pdesc),
        )
    }
    pub unsafe fn GetFullscreenDesc(
        &self,
        pdesc: *mut DXGI_SWAP_CHAIN_FULLSCREEN_DESC,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).19)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(pdesc),
        )
    }
    pub unsafe fn GetHwnd(
        &self,
        phwnd: *mut super::WindowsAndMessaging::HWND,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).20)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(phwnd),
        )
    }
    pub unsafe fn GetCoreWindow(
        &self,
        refiid: *const ::windows::Guid,
        ppunk: *mut *mut ::std::ffi::c_void,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).21)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(refiid),
            ::std::mem::transmute(ppunk),
        )
    }
    pub unsafe fn Present1(
        &self,
        syncinterval: u32,
        presentflags: u32,
        ppresentparameters: *const DXGI_PRESENT_PARAMETERS,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).22)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(syncinterval),
            ::std::mem::transmute(presentflags),
            ::std::mem::transmute(ppresentparameters),
        )
    }
    pub unsafe fn IsTemporaryMonoSupported(&self) -> super::SystemServices::BOOL {
        (::windows::Interface::vtable(self).23)(::windows::Abi::abi(self))
    }
    pub unsafe fn GetRestrictToOutput(
        &self,
        pprestricttooutput: *mut ::std::option::Option<IDXGIOutput>,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).24)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(pprestricttooutput),
        )
    }
    pub unsafe fn SetBackgroundColor(&self, pcolor: *const DXGI_RGBA) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).25)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(pcolor),
        )
    }
    pub unsafe fn GetBackgroundColor(&self, pcolor: *mut DXGI_RGBA) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).26)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(pcolor),
        )
    }
    pub unsafe fn SetRotation(&self, rotation: DXGI_MODE_ROTATION) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).27)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(rotation),
        )
    }
    pub unsafe fn GetRotation(&self, protation: *mut DXGI_MODE_ROTATION) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).28)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(protation),
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
        datasize: u32,
        pdata: *const ::std::ffi::c_void,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        name: *const ::windows::Guid,
        punknown: ::windows::RawPtr,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        name: *const ::windows::Guid,
        pdatasize: *mut u32,
        pdata: *mut ::std::ffi::c_void,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        riid: *const ::windows::Guid,
        ppparent: *mut *mut ::std::ffi::c_void,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        riid: *const ::windows::Guid,
        ppdevice: *mut *mut ::std::ffi::c_void,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        syncinterval: u32,
        flags: u32,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        buffer: u32,
        riid: *const ::windows::Guid,
        ppsurface: *mut *mut ::std::ffi::c_void,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        fullscreen: super::SystemServices::BOOL,
        ptarget: ::windows::RawPtr,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        pfullscreen: *mut super::SystemServices::BOOL,
        pptarget: *mut ::windows::RawPtr,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        pdesc: *mut DXGI_SWAP_CHAIN_DESC,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        buffercount: u32,
        width: u32,
        height: u32,
        newformat: DXGI_FORMAT,
        swapchainflags: u32,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        pnewtargetparameters: *const DXGI_MODE_DESC,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        ppoutput: *mut ::windows::RawPtr,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        pstats: *mut DXGI_FRAME_STATISTICS,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        plastpresentcount: *mut u32,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        pdesc: *mut DXGI_SWAP_CHAIN_DESC1,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        pdesc: *mut DXGI_SWAP_CHAIN_FULLSCREEN_DESC,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        phwnd: *mut super::WindowsAndMessaging::HWND,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        refiid: *const ::windows::Guid,
        ppunk: *mut *mut ::std::ffi::c_void,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        syncinterval: u32,
        presentflags: u32,
        ppresentparameters: *const DXGI_PRESENT_PARAMETERS,
    ) -> ::windows::ErrorCode,
    pub unsafe extern "system" fn(this: ::windows::RawPtr) -> super::SystemServices::BOOL,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        pprestricttooutput: *mut ::windows::RawPtr,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        pcolor: *const DXGI_RGBA,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        pcolor: *mut DXGI_RGBA,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        rotation: DXGI_MODE_ROTATION,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        protation: *mut DXGI_MODE_ROTATION,
    ) -> ::windows::ErrorCode,
);
#[repr(transparent)]
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
impl IDXGISwapChain2 {
    pub unsafe fn SetPrivateData(
        &self,
        name: *const ::windows::Guid,
        datasize: u32,
        pdata: *const ::std::ffi::c_void,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).3)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(name),
            ::std::mem::transmute(datasize),
            ::std::mem::transmute(pdata),
        )
    }
    pub unsafe fn SetPrivateDataInterface<
        'a,
        T1__: ::windows::IntoParam<'a, ::windows::IUnknown>,
    >(
        &self,
        name: *const ::windows::Guid,
        punknown: T1__,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).4)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(name),
            punknown.into_param().abi(),
        )
    }
    pub unsafe fn GetPrivateData(
        &self,
        name: *const ::windows::Guid,
        pdatasize: *mut u32,
        pdata: *mut ::std::ffi::c_void,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).5)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(name),
            ::std::mem::transmute(pdatasize),
            ::std::mem::transmute(pdata),
        )
    }
    pub unsafe fn GetParent(
        &self,
        riid: *const ::windows::Guid,
        ppparent: *mut *mut ::std::ffi::c_void,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).6)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(riid),
            ::std::mem::transmute(ppparent),
        )
    }
    pub unsafe fn GetDevice(
        &self,
        riid: *const ::windows::Guid,
        ppdevice: *mut *mut ::std::ffi::c_void,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).7)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(riid),
            ::std::mem::transmute(ppdevice),
        )
    }
    pub unsafe fn Present(&self, syncinterval: u32, flags: u32) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).8)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(syncinterval),
            ::std::mem::transmute(flags),
        )
    }
    pub unsafe fn GetBuffer(
        &self,
        buffer: u32,
        riid: *const ::windows::Guid,
        ppsurface: *mut *mut ::std::ffi::c_void,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).9)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(buffer),
            ::std::mem::transmute(riid),
            ::std::mem::transmute(ppsurface),
        )
    }
    pub unsafe fn SetFullscreenState<
        'a,
        T0__: ::windows::IntoParam<'a, super::SystemServices::BOOL>,
        T1__: ::windows::IntoParam<'a, IDXGIOutput>,
    >(
        &self,
        fullscreen: T0__,
        ptarget: T1__,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).10)(
            ::windows::Abi::abi(self),
            fullscreen.into_param().abi(),
            ptarget.into_param().abi(),
        )
    }
    pub unsafe fn GetFullscreenState(
        &self,
        pfullscreen: *mut super::SystemServices::BOOL,
        pptarget: *mut ::std::option::Option<IDXGIOutput>,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).11)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(pfullscreen),
            ::std::mem::transmute(pptarget),
        )
    }
    pub unsafe fn GetDesc(&self, pdesc: *mut DXGI_SWAP_CHAIN_DESC) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).12)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(pdesc),
        )
    }
    pub unsafe fn ResizeBuffers(
        &self,
        buffercount: u32,
        width: u32,
        height: u32,
        newformat: DXGI_FORMAT,
        swapchainflags: u32,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).13)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(buffercount),
            ::std::mem::transmute(width),
            ::std::mem::transmute(height),
            ::std::mem::transmute(newformat),
            ::std::mem::transmute(swapchainflags),
        )
    }
    pub unsafe fn ResizeTarget(
        &self,
        pnewtargetparameters: *const DXGI_MODE_DESC,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).14)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(pnewtargetparameters),
        )
    }
    pub unsafe fn GetContainingOutput(
        &self,
        ppoutput: *mut ::std::option::Option<IDXGIOutput>,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).15)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(ppoutput),
        )
    }
    pub unsafe fn GetFrameStatistics(
        &self,
        pstats: *mut DXGI_FRAME_STATISTICS,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).16)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(pstats),
        )
    }
    pub unsafe fn GetLastPresentCount(&self, plastpresentcount: *mut u32) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).17)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(plastpresentcount),
        )
    }
    pub unsafe fn GetDesc1(&self, pdesc: *mut DXGI_SWAP_CHAIN_DESC1) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).18)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(pdesc),
        )
    }
    pub unsafe fn GetFullscreenDesc(
        &self,
        pdesc: *mut DXGI_SWAP_CHAIN_FULLSCREEN_DESC,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).19)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(pdesc),
        )
    }
    pub unsafe fn GetHwnd(
        &self,
        phwnd: *mut super::WindowsAndMessaging::HWND,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).20)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(phwnd),
        )
    }
    pub unsafe fn GetCoreWindow(
        &self,
        refiid: *const ::windows::Guid,
        ppunk: *mut *mut ::std::ffi::c_void,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).21)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(refiid),
            ::std::mem::transmute(ppunk),
        )
    }
    pub unsafe fn Present1(
        &self,
        syncinterval: u32,
        presentflags: u32,
        ppresentparameters: *const DXGI_PRESENT_PARAMETERS,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).22)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(syncinterval),
            ::std::mem::transmute(presentflags),
            ::std::mem::transmute(ppresentparameters),
        )
    }
    pub unsafe fn IsTemporaryMonoSupported(&self) -> super::SystemServices::BOOL {
        (::windows::Interface::vtable(self).23)(::windows::Abi::abi(self))
    }
    pub unsafe fn GetRestrictToOutput(
        &self,
        pprestricttooutput: *mut ::std::option::Option<IDXGIOutput>,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).24)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(pprestricttooutput),
        )
    }
    pub unsafe fn SetBackgroundColor(&self, pcolor: *const DXGI_RGBA) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).25)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(pcolor),
        )
    }
    pub unsafe fn GetBackgroundColor(&self, pcolor: *mut DXGI_RGBA) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).26)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(pcolor),
        )
    }
    pub unsafe fn SetRotation(&self, rotation: DXGI_MODE_ROTATION) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).27)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(rotation),
        )
    }
    pub unsafe fn GetRotation(&self, protation: *mut DXGI_MODE_ROTATION) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).28)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(protation),
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
        pwidth: *mut u32,
        pheight: *mut u32,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).30)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(pwidth),
            ::std::mem::transmute(pheight),
        )
    }
    pub unsafe fn SetMaximumFrameLatency(&self, maxlatency: u32) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).31)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(maxlatency),
        )
    }
    pub unsafe fn GetMaximumFrameLatency(&self, pmaxlatency: *mut u32) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).32)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(pmaxlatency),
        )
    }
    pub unsafe fn GetFrameLatencyWaitableObject(&self) -> super::SystemServices::HANDLE {
        (::windows::Interface::vtable(self).33)(::windows::Abi::abi(self))
    }
    pub unsafe fn SetMatrixTransform(
        &self,
        pmatrix: *const DXGI_MATRIX_3X2_F,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).34)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(pmatrix),
        )
    }
    pub unsafe fn GetMatrixTransform(
        &self,
        pmatrix: *mut DXGI_MATRIX_3X2_F,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).35)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(pmatrix),
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
        datasize: u32,
        pdata: *const ::std::ffi::c_void,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        name: *const ::windows::Guid,
        punknown: ::windows::RawPtr,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        name: *const ::windows::Guid,
        pdatasize: *mut u32,
        pdata: *mut ::std::ffi::c_void,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        riid: *const ::windows::Guid,
        ppparent: *mut *mut ::std::ffi::c_void,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        riid: *const ::windows::Guid,
        ppdevice: *mut *mut ::std::ffi::c_void,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        syncinterval: u32,
        flags: u32,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        buffer: u32,
        riid: *const ::windows::Guid,
        ppsurface: *mut *mut ::std::ffi::c_void,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        fullscreen: super::SystemServices::BOOL,
        ptarget: ::windows::RawPtr,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        pfullscreen: *mut super::SystemServices::BOOL,
        pptarget: *mut ::windows::RawPtr,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        pdesc: *mut DXGI_SWAP_CHAIN_DESC,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        buffercount: u32,
        width: u32,
        height: u32,
        newformat: DXGI_FORMAT,
        swapchainflags: u32,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        pnewtargetparameters: *const DXGI_MODE_DESC,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        ppoutput: *mut ::windows::RawPtr,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        pstats: *mut DXGI_FRAME_STATISTICS,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        plastpresentcount: *mut u32,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        pdesc: *mut DXGI_SWAP_CHAIN_DESC1,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        pdesc: *mut DXGI_SWAP_CHAIN_FULLSCREEN_DESC,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        phwnd: *mut super::WindowsAndMessaging::HWND,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        refiid: *const ::windows::Guid,
        ppunk: *mut *mut ::std::ffi::c_void,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        syncinterval: u32,
        presentflags: u32,
        ppresentparameters: *const DXGI_PRESENT_PARAMETERS,
    ) -> ::windows::ErrorCode,
    pub unsafe extern "system" fn(this: ::windows::RawPtr) -> super::SystemServices::BOOL,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        pprestricttooutput: *mut ::windows::RawPtr,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        pcolor: *const DXGI_RGBA,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        pcolor: *mut DXGI_RGBA,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        rotation: DXGI_MODE_ROTATION,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        protation: *mut DXGI_MODE_ROTATION,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        width: u32,
        height: u32,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        pwidth: *mut u32,
        pheight: *mut u32,
    ) -> ::windows::ErrorCode,
    pub unsafe extern "system" fn(this: ::windows::RawPtr, maxlatency: u32) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        pmaxlatency: *mut u32,
    ) -> ::windows::ErrorCode,
    pub unsafe extern "system" fn(this: ::windows::RawPtr) -> super::SystemServices::HANDLE,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        pmatrix: *const DXGI_MATRIX_3X2_F,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        pmatrix: *mut DXGI_MATRIX_3X2_F,
    ) -> ::windows::ErrorCode,
);
#[repr(transparent)]
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
impl IDXGISwapChain3 {
    pub unsafe fn SetPrivateData(
        &self,
        name: *const ::windows::Guid,
        datasize: u32,
        pdata: *const ::std::ffi::c_void,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).3)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(name),
            ::std::mem::transmute(datasize),
            ::std::mem::transmute(pdata),
        )
    }
    pub unsafe fn SetPrivateDataInterface<
        'a,
        T1__: ::windows::IntoParam<'a, ::windows::IUnknown>,
    >(
        &self,
        name: *const ::windows::Guid,
        punknown: T1__,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).4)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(name),
            punknown.into_param().abi(),
        )
    }
    pub unsafe fn GetPrivateData(
        &self,
        name: *const ::windows::Guid,
        pdatasize: *mut u32,
        pdata: *mut ::std::ffi::c_void,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).5)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(name),
            ::std::mem::transmute(pdatasize),
            ::std::mem::transmute(pdata),
        )
    }
    pub unsafe fn GetParent(
        &self,
        riid: *const ::windows::Guid,
        ppparent: *mut *mut ::std::ffi::c_void,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).6)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(riid),
            ::std::mem::transmute(ppparent),
        )
    }
    pub unsafe fn GetDevice(
        &self,
        riid: *const ::windows::Guid,
        ppdevice: *mut *mut ::std::ffi::c_void,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).7)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(riid),
            ::std::mem::transmute(ppdevice),
        )
    }
    pub unsafe fn Present(&self, syncinterval: u32, flags: u32) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).8)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(syncinterval),
            ::std::mem::transmute(flags),
        )
    }
    pub unsafe fn GetBuffer(
        &self,
        buffer: u32,
        riid: *const ::windows::Guid,
        ppsurface: *mut *mut ::std::ffi::c_void,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).9)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(buffer),
            ::std::mem::transmute(riid),
            ::std::mem::transmute(ppsurface),
        )
    }
    pub unsafe fn SetFullscreenState<
        'a,
        T0__: ::windows::IntoParam<'a, super::SystemServices::BOOL>,
        T1__: ::windows::IntoParam<'a, IDXGIOutput>,
    >(
        &self,
        fullscreen: T0__,
        ptarget: T1__,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).10)(
            ::windows::Abi::abi(self),
            fullscreen.into_param().abi(),
            ptarget.into_param().abi(),
        )
    }
    pub unsafe fn GetFullscreenState(
        &self,
        pfullscreen: *mut super::SystemServices::BOOL,
        pptarget: *mut ::std::option::Option<IDXGIOutput>,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).11)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(pfullscreen),
            ::std::mem::transmute(pptarget),
        )
    }
    pub unsafe fn GetDesc(&self, pdesc: *mut DXGI_SWAP_CHAIN_DESC) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).12)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(pdesc),
        )
    }
    pub unsafe fn ResizeBuffers(
        &self,
        buffercount: u32,
        width: u32,
        height: u32,
        newformat: DXGI_FORMAT,
        swapchainflags: u32,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).13)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(buffercount),
            ::std::mem::transmute(width),
            ::std::mem::transmute(height),
            ::std::mem::transmute(newformat),
            ::std::mem::transmute(swapchainflags),
        )
    }
    pub unsafe fn ResizeTarget(
        &self,
        pnewtargetparameters: *const DXGI_MODE_DESC,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).14)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(pnewtargetparameters),
        )
    }
    pub unsafe fn GetContainingOutput(
        &self,
        ppoutput: *mut ::std::option::Option<IDXGIOutput>,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).15)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(ppoutput),
        )
    }
    pub unsafe fn GetFrameStatistics(
        &self,
        pstats: *mut DXGI_FRAME_STATISTICS,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).16)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(pstats),
        )
    }
    pub unsafe fn GetLastPresentCount(&self, plastpresentcount: *mut u32) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).17)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(plastpresentcount),
        )
    }
    pub unsafe fn GetDesc1(&self, pdesc: *mut DXGI_SWAP_CHAIN_DESC1) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).18)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(pdesc),
        )
    }
    pub unsafe fn GetFullscreenDesc(
        &self,
        pdesc: *mut DXGI_SWAP_CHAIN_FULLSCREEN_DESC,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).19)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(pdesc),
        )
    }
    pub unsafe fn GetHwnd(
        &self,
        phwnd: *mut super::WindowsAndMessaging::HWND,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).20)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(phwnd),
        )
    }
    pub unsafe fn GetCoreWindow(
        &self,
        refiid: *const ::windows::Guid,
        ppunk: *mut *mut ::std::ffi::c_void,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).21)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(refiid),
            ::std::mem::transmute(ppunk),
        )
    }
    pub unsafe fn Present1(
        &self,
        syncinterval: u32,
        presentflags: u32,
        ppresentparameters: *const DXGI_PRESENT_PARAMETERS,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).22)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(syncinterval),
            ::std::mem::transmute(presentflags),
            ::std::mem::transmute(ppresentparameters),
        )
    }
    pub unsafe fn IsTemporaryMonoSupported(&self) -> super::SystemServices::BOOL {
        (::windows::Interface::vtable(self).23)(::windows::Abi::abi(self))
    }
    pub unsafe fn GetRestrictToOutput(
        &self,
        pprestricttooutput: *mut ::std::option::Option<IDXGIOutput>,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).24)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(pprestricttooutput),
        )
    }
    pub unsafe fn SetBackgroundColor(&self, pcolor: *const DXGI_RGBA) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).25)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(pcolor),
        )
    }
    pub unsafe fn GetBackgroundColor(&self, pcolor: *mut DXGI_RGBA) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).26)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(pcolor),
        )
    }
    pub unsafe fn SetRotation(&self, rotation: DXGI_MODE_ROTATION) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).27)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(rotation),
        )
    }
    pub unsafe fn GetRotation(&self, protation: *mut DXGI_MODE_ROTATION) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).28)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(protation),
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
        pwidth: *mut u32,
        pheight: *mut u32,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).30)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(pwidth),
            ::std::mem::transmute(pheight),
        )
    }
    pub unsafe fn SetMaximumFrameLatency(&self, maxlatency: u32) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).31)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(maxlatency),
        )
    }
    pub unsafe fn GetMaximumFrameLatency(&self, pmaxlatency: *mut u32) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).32)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(pmaxlatency),
        )
    }
    pub unsafe fn GetFrameLatencyWaitableObject(&self) -> super::SystemServices::HANDLE {
        (::windows::Interface::vtable(self).33)(::windows::Abi::abi(self))
    }
    pub unsafe fn SetMatrixTransform(
        &self,
        pmatrix: *const DXGI_MATRIX_3X2_F,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).34)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(pmatrix),
        )
    }
    pub unsafe fn GetMatrixTransform(
        &self,
        pmatrix: *mut DXGI_MATRIX_3X2_F,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).35)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(pmatrix),
        )
    }
    pub unsafe fn GetCurrentBackBufferIndex(&self) -> u32 {
        (::windows::Interface::vtable(self).36)(::windows::Abi::abi(self))
    }
    pub unsafe fn CheckColorSpaceSupport(
        &self,
        colorspace: DXGI_COLOR_SPACE_TYPE,
        pcolorspacesupport: *mut u32,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).37)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(colorspace),
            ::std::mem::transmute(pcolorspacesupport),
        )
    }
    pub unsafe fn SetColorSpace1(&self, colorspace: DXGI_COLOR_SPACE_TYPE) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).38)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(colorspace),
        )
    }
    pub unsafe fn ResizeBuffers1(
        &self,
        buffercount: u32,
        width: u32,
        height: u32,
        format: DXGI_FORMAT,
        swapchainflags: u32,
        pcreationnodemask: *const u32,
        pppresentqueue: *mut ::std::option::Option<::windows::IUnknown>,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).39)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(buffercount),
            ::std::mem::transmute(width),
            ::std::mem::transmute(height),
            ::std::mem::transmute(format),
            ::std::mem::transmute(swapchainflags),
            ::std::mem::transmute(pcreationnodemask),
            ::std::mem::transmute(pppresentqueue),
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
        datasize: u32,
        pdata: *const ::std::ffi::c_void,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        name: *const ::windows::Guid,
        punknown: ::windows::RawPtr,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        name: *const ::windows::Guid,
        pdatasize: *mut u32,
        pdata: *mut ::std::ffi::c_void,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        riid: *const ::windows::Guid,
        ppparent: *mut *mut ::std::ffi::c_void,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        riid: *const ::windows::Guid,
        ppdevice: *mut *mut ::std::ffi::c_void,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        syncinterval: u32,
        flags: u32,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        buffer: u32,
        riid: *const ::windows::Guid,
        ppsurface: *mut *mut ::std::ffi::c_void,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        fullscreen: super::SystemServices::BOOL,
        ptarget: ::windows::RawPtr,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        pfullscreen: *mut super::SystemServices::BOOL,
        pptarget: *mut ::windows::RawPtr,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        pdesc: *mut DXGI_SWAP_CHAIN_DESC,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        buffercount: u32,
        width: u32,
        height: u32,
        newformat: DXGI_FORMAT,
        swapchainflags: u32,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        pnewtargetparameters: *const DXGI_MODE_DESC,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        ppoutput: *mut ::windows::RawPtr,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        pstats: *mut DXGI_FRAME_STATISTICS,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        plastpresentcount: *mut u32,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        pdesc: *mut DXGI_SWAP_CHAIN_DESC1,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        pdesc: *mut DXGI_SWAP_CHAIN_FULLSCREEN_DESC,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        phwnd: *mut super::WindowsAndMessaging::HWND,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        refiid: *const ::windows::Guid,
        ppunk: *mut *mut ::std::ffi::c_void,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        syncinterval: u32,
        presentflags: u32,
        ppresentparameters: *const DXGI_PRESENT_PARAMETERS,
    ) -> ::windows::ErrorCode,
    pub unsafe extern "system" fn(this: ::windows::RawPtr) -> super::SystemServices::BOOL,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        pprestricttooutput: *mut ::windows::RawPtr,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        pcolor: *const DXGI_RGBA,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        pcolor: *mut DXGI_RGBA,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        rotation: DXGI_MODE_ROTATION,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        protation: *mut DXGI_MODE_ROTATION,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        width: u32,
        height: u32,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        pwidth: *mut u32,
        pheight: *mut u32,
    ) -> ::windows::ErrorCode,
    pub unsafe extern "system" fn(this: ::windows::RawPtr, maxlatency: u32) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        pmaxlatency: *mut u32,
    ) -> ::windows::ErrorCode,
    pub unsafe extern "system" fn(this: ::windows::RawPtr) -> super::SystemServices::HANDLE,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        pmatrix: *const DXGI_MATRIX_3X2_F,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        pmatrix: *mut DXGI_MATRIX_3X2_F,
    ) -> ::windows::ErrorCode,
    pub unsafe extern "system" fn(this: ::windows::RawPtr) -> u32,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        colorspace: DXGI_COLOR_SPACE_TYPE,
        pcolorspacesupport: *mut u32,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        colorspace: DXGI_COLOR_SPACE_TYPE,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        buffercount: u32,
        width: u32,
        height: u32,
        format: DXGI_FORMAT,
        swapchainflags: u32,
        pcreationnodemask: *const u32,
        pppresentqueue: *mut ::windows::RawPtr,
    ) -> ::windows::ErrorCode,
);
#[repr(transparent)]
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
impl IDXGISwapChain4 {
    pub unsafe fn SetPrivateData(
        &self,
        name: *const ::windows::Guid,
        datasize: u32,
        pdata: *const ::std::ffi::c_void,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).3)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(name),
            ::std::mem::transmute(datasize),
            ::std::mem::transmute(pdata),
        )
    }
    pub unsafe fn SetPrivateDataInterface<
        'a,
        T1__: ::windows::IntoParam<'a, ::windows::IUnknown>,
    >(
        &self,
        name: *const ::windows::Guid,
        punknown: T1__,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).4)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(name),
            punknown.into_param().abi(),
        )
    }
    pub unsafe fn GetPrivateData(
        &self,
        name: *const ::windows::Guid,
        pdatasize: *mut u32,
        pdata: *mut ::std::ffi::c_void,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).5)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(name),
            ::std::mem::transmute(pdatasize),
            ::std::mem::transmute(pdata),
        )
    }
    pub unsafe fn GetParent(
        &self,
        riid: *const ::windows::Guid,
        ppparent: *mut *mut ::std::ffi::c_void,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).6)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(riid),
            ::std::mem::transmute(ppparent),
        )
    }
    pub unsafe fn GetDevice(
        &self,
        riid: *const ::windows::Guid,
        ppdevice: *mut *mut ::std::ffi::c_void,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).7)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(riid),
            ::std::mem::transmute(ppdevice),
        )
    }
    pub unsafe fn Present(&self, syncinterval: u32, flags: u32) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).8)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(syncinterval),
            ::std::mem::transmute(flags),
        )
    }
    pub unsafe fn GetBuffer(
        &self,
        buffer: u32,
        riid: *const ::windows::Guid,
        ppsurface: *mut *mut ::std::ffi::c_void,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).9)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(buffer),
            ::std::mem::transmute(riid),
            ::std::mem::transmute(ppsurface),
        )
    }
    pub unsafe fn SetFullscreenState<
        'a,
        T0__: ::windows::IntoParam<'a, super::SystemServices::BOOL>,
        T1__: ::windows::IntoParam<'a, IDXGIOutput>,
    >(
        &self,
        fullscreen: T0__,
        ptarget: T1__,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).10)(
            ::windows::Abi::abi(self),
            fullscreen.into_param().abi(),
            ptarget.into_param().abi(),
        )
    }
    pub unsafe fn GetFullscreenState(
        &self,
        pfullscreen: *mut super::SystemServices::BOOL,
        pptarget: *mut ::std::option::Option<IDXGIOutput>,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).11)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(pfullscreen),
            ::std::mem::transmute(pptarget),
        )
    }
    pub unsafe fn GetDesc(&self, pdesc: *mut DXGI_SWAP_CHAIN_DESC) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).12)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(pdesc),
        )
    }
    pub unsafe fn ResizeBuffers(
        &self,
        buffercount: u32,
        width: u32,
        height: u32,
        newformat: DXGI_FORMAT,
        swapchainflags: u32,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).13)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(buffercount),
            ::std::mem::transmute(width),
            ::std::mem::transmute(height),
            ::std::mem::transmute(newformat),
            ::std::mem::transmute(swapchainflags),
        )
    }
    pub unsafe fn ResizeTarget(
        &self,
        pnewtargetparameters: *const DXGI_MODE_DESC,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).14)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(pnewtargetparameters),
        )
    }
    pub unsafe fn GetContainingOutput(
        &self,
        ppoutput: *mut ::std::option::Option<IDXGIOutput>,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).15)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(ppoutput),
        )
    }
    pub unsafe fn GetFrameStatistics(
        &self,
        pstats: *mut DXGI_FRAME_STATISTICS,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).16)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(pstats),
        )
    }
    pub unsafe fn GetLastPresentCount(&self, plastpresentcount: *mut u32) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).17)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(plastpresentcount),
        )
    }
    pub unsafe fn GetDesc1(&self, pdesc: *mut DXGI_SWAP_CHAIN_DESC1) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).18)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(pdesc),
        )
    }
    pub unsafe fn GetFullscreenDesc(
        &self,
        pdesc: *mut DXGI_SWAP_CHAIN_FULLSCREEN_DESC,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).19)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(pdesc),
        )
    }
    pub unsafe fn GetHwnd(
        &self,
        phwnd: *mut super::WindowsAndMessaging::HWND,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).20)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(phwnd),
        )
    }
    pub unsafe fn GetCoreWindow(
        &self,
        refiid: *const ::windows::Guid,
        ppunk: *mut *mut ::std::ffi::c_void,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).21)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(refiid),
            ::std::mem::transmute(ppunk),
        )
    }
    pub unsafe fn Present1(
        &self,
        syncinterval: u32,
        presentflags: u32,
        ppresentparameters: *const DXGI_PRESENT_PARAMETERS,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).22)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(syncinterval),
            ::std::mem::transmute(presentflags),
            ::std::mem::transmute(ppresentparameters),
        )
    }
    pub unsafe fn IsTemporaryMonoSupported(&self) -> super::SystemServices::BOOL {
        (::windows::Interface::vtable(self).23)(::windows::Abi::abi(self))
    }
    pub unsafe fn GetRestrictToOutput(
        &self,
        pprestricttooutput: *mut ::std::option::Option<IDXGIOutput>,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).24)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(pprestricttooutput),
        )
    }
    pub unsafe fn SetBackgroundColor(&self, pcolor: *const DXGI_RGBA) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).25)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(pcolor),
        )
    }
    pub unsafe fn GetBackgroundColor(&self, pcolor: *mut DXGI_RGBA) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).26)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(pcolor),
        )
    }
    pub unsafe fn SetRotation(&self, rotation: DXGI_MODE_ROTATION) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).27)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(rotation),
        )
    }
    pub unsafe fn GetRotation(&self, protation: *mut DXGI_MODE_ROTATION) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).28)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(protation),
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
        pwidth: *mut u32,
        pheight: *mut u32,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).30)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(pwidth),
            ::std::mem::transmute(pheight),
        )
    }
    pub unsafe fn SetMaximumFrameLatency(&self, maxlatency: u32) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).31)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(maxlatency),
        )
    }
    pub unsafe fn GetMaximumFrameLatency(&self, pmaxlatency: *mut u32) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).32)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(pmaxlatency),
        )
    }
    pub unsafe fn GetFrameLatencyWaitableObject(&self) -> super::SystemServices::HANDLE {
        (::windows::Interface::vtable(self).33)(::windows::Abi::abi(self))
    }
    pub unsafe fn SetMatrixTransform(
        &self,
        pmatrix: *const DXGI_MATRIX_3X2_F,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).34)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(pmatrix),
        )
    }
    pub unsafe fn GetMatrixTransform(
        &self,
        pmatrix: *mut DXGI_MATRIX_3X2_F,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).35)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(pmatrix),
        )
    }
    pub unsafe fn GetCurrentBackBufferIndex(&self) -> u32 {
        (::windows::Interface::vtable(self).36)(::windows::Abi::abi(self))
    }
    pub unsafe fn CheckColorSpaceSupport(
        &self,
        colorspace: DXGI_COLOR_SPACE_TYPE,
        pcolorspacesupport: *mut u32,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).37)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(colorspace),
            ::std::mem::transmute(pcolorspacesupport),
        )
    }
    pub unsafe fn SetColorSpace1(&self, colorspace: DXGI_COLOR_SPACE_TYPE) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).38)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(colorspace),
        )
    }
    pub unsafe fn ResizeBuffers1(
        &self,
        buffercount: u32,
        width: u32,
        height: u32,
        format: DXGI_FORMAT,
        swapchainflags: u32,
        pcreationnodemask: *const u32,
        pppresentqueue: *mut ::std::option::Option<::windows::IUnknown>,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).39)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(buffercount),
            ::std::mem::transmute(width),
            ::std::mem::transmute(height),
            ::std::mem::transmute(format),
            ::std::mem::transmute(swapchainflags),
            ::std::mem::transmute(pcreationnodemask),
            ::std::mem::transmute(pppresentqueue),
        )
    }
    pub unsafe fn SetHDRMetaData(
        &self,
        r#type: DXGI_HDR_METADATA_TYPE,
        size: u32,
        pmetadata: *mut ::std::ffi::c_void,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).40)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(r#type),
            ::std::mem::transmute(size),
            ::std::mem::transmute(pmetadata),
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
        datasize: u32,
        pdata: *const ::std::ffi::c_void,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        name: *const ::windows::Guid,
        punknown: ::windows::RawPtr,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        name: *const ::windows::Guid,
        pdatasize: *mut u32,
        pdata: *mut ::std::ffi::c_void,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        riid: *const ::windows::Guid,
        ppparent: *mut *mut ::std::ffi::c_void,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        riid: *const ::windows::Guid,
        ppdevice: *mut *mut ::std::ffi::c_void,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        syncinterval: u32,
        flags: u32,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        buffer: u32,
        riid: *const ::windows::Guid,
        ppsurface: *mut *mut ::std::ffi::c_void,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        fullscreen: super::SystemServices::BOOL,
        ptarget: ::windows::RawPtr,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        pfullscreen: *mut super::SystemServices::BOOL,
        pptarget: *mut ::windows::RawPtr,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        pdesc: *mut DXGI_SWAP_CHAIN_DESC,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        buffercount: u32,
        width: u32,
        height: u32,
        newformat: DXGI_FORMAT,
        swapchainflags: u32,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        pnewtargetparameters: *const DXGI_MODE_DESC,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        ppoutput: *mut ::windows::RawPtr,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        pstats: *mut DXGI_FRAME_STATISTICS,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        plastpresentcount: *mut u32,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        pdesc: *mut DXGI_SWAP_CHAIN_DESC1,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        pdesc: *mut DXGI_SWAP_CHAIN_FULLSCREEN_DESC,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        phwnd: *mut super::WindowsAndMessaging::HWND,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        refiid: *const ::windows::Guid,
        ppunk: *mut *mut ::std::ffi::c_void,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        syncinterval: u32,
        presentflags: u32,
        ppresentparameters: *const DXGI_PRESENT_PARAMETERS,
    ) -> ::windows::ErrorCode,
    pub unsafe extern "system" fn(this: ::windows::RawPtr) -> super::SystemServices::BOOL,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        pprestricttooutput: *mut ::windows::RawPtr,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        pcolor: *const DXGI_RGBA,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        pcolor: *mut DXGI_RGBA,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        rotation: DXGI_MODE_ROTATION,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        protation: *mut DXGI_MODE_ROTATION,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        width: u32,
        height: u32,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        pwidth: *mut u32,
        pheight: *mut u32,
    ) -> ::windows::ErrorCode,
    pub unsafe extern "system" fn(this: ::windows::RawPtr, maxlatency: u32) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        pmaxlatency: *mut u32,
    ) -> ::windows::ErrorCode,
    pub unsafe extern "system" fn(this: ::windows::RawPtr) -> super::SystemServices::HANDLE,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        pmatrix: *const DXGI_MATRIX_3X2_F,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        pmatrix: *mut DXGI_MATRIX_3X2_F,
    ) -> ::windows::ErrorCode,
    pub unsafe extern "system" fn(this: ::windows::RawPtr) -> u32,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        colorspace: DXGI_COLOR_SPACE_TYPE,
        pcolorspacesupport: *mut u32,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        colorspace: DXGI_COLOR_SPACE_TYPE,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        buffercount: u32,
        width: u32,
        height: u32,
        format: DXGI_FORMAT,
        swapchainflags: u32,
        pcreationnodemask: *const u32,
        pppresentqueue: *mut ::windows::RawPtr,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        r#type: DXGI_HDR_METADATA_TYPE,
        size: u32,
        pmetadata: *mut ::std::ffi::c_void,
    ) -> ::windows::ErrorCode,
);
#[repr(transparent)]
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
impl IDXGISwapChainMedia {
    pub unsafe fn GetFrameStatisticsMedia(
        &self,
        pstats: *mut DXGI_FRAME_STATISTICS_MEDIA,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).3)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(pstats),
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
        desiredpresentduration: u32,
        pclosestsmallerpresentduration: *mut u32,
        pclosestlargerpresentduration: *mut u32,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).5)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(desiredpresentduration),
            ::std::mem::transmute(pclosestsmallerpresentduration),
            ::std::mem::transmute(pclosestlargerpresentduration),
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
        pstats: *mut DXGI_FRAME_STATISTICS_MEDIA,
    ) -> ::windows::ErrorCode,
    pub unsafe extern "system" fn(this: ::windows::RawPtr, duration: u32) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        desiredpresentduration: u32,
        pclosestsmallerpresentduration: *mut u32,
        pclosestlargerpresentduration: *mut u32,
    ) -> ::windows::ErrorCode,
);
