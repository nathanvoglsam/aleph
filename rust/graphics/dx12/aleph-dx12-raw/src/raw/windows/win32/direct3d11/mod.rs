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
pub struct D3D_FEATURE_LEVEL(pub i32);
impl D3D_FEATURE_LEVEL {
    #![allow(non_upper_case_globals)]
    pub const D3D_FEATURE_LEVEL_1_0_CORE: Self = Self(4096i32);
    pub const D3D_FEATURE_LEVEL_9_1: Self = Self(37120i32);
    pub const D3D_FEATURE_LEVEL_9_2: Self = Self(37376i32);
    pub const D3D_FEATURE_LEVEL_9_3: Self = Self(37632i32);
    pub const D3D_FEATURE_LEVEL_10_0: Self = Self(40960i32);
    pub const D3D_FEATURE_LEVEL_10_1: Self = Self(41216i32);
    pub const D3D_FEATURE_LEVEL_11_0: Self = Self(45056i32);
    pub const D3D_FEATURE_LEVEL_11_1: Self = Self(45312i32);
    pub const D3D_FEATURE_LEVEL_12_0: Self = Self(49152i32);
    pub const D3D_FEATURE_LEVEL_12_1: Self = Self(49408i32);
}
impl ::std::convert::From<i32> for D3D_FEATURE_LEVEL {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::Abi for D3D_FEATURE_LEVEL {
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
pub struct D3D11_USAGE(pub i32);
impl D3D11_USAGE {
    #![allow(non_upper_case_globals)]
    pub const D3D11_USAGE_DEFAULT: Self = Self(0i32);
    pub const D3D11_USAGE_IMMUTABLE: Self = Self(1i32);
    pub const D3D11_USAGE_DYNAMIC: Self = Self(2i32);
    pub const D3D11_USAGE_STAGING: Self = Self(3i32);
}
impl ::std::convert::From<i32> for D3D11_USAGE {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::Abi for D3D11_USAGE {
    type Abi = Self;
}
#[repr(C)]
#[allow(non_snake_case)]
#[derive(:: std :: clone :: Clone)]
pub struct D3D11_BUFFER_DESC {
    pub byte_width: u32,
    pub usage: D3D11_USAGE,
    pub bind_flags: u32,
    pub cpu_access_flags: u32,
    pub misc_flags: u32,
    pub structure_byte_stride: u32,
}
impl D3D11_BUFFER_DESC {}
impl ::std::default::Default for D3D11_BUFFER_DESC {
    fn default() -> Self {
        Self {
            byte_width: 0,
            usage: ::std::default::Default::default(),
            bind_flags: 0,
            cpu_access_flags: 0,
            misc_flags: 0,
            structure_byte_stride: 0,
        }
    }
}
impl ::std::fmt::Debug for D3D11_BUFFER_DESC {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("D3D11_BUFFER_DESC")
            .field("byte_width", &format_args!("{:?}", self.byte_width))
            .field("usage", &format_args!("{:?}", self.usage))
            .field("bind_flags", &format_args!("{:?}", self.bind_flags))
            .field(
                "cpu_access_flags",
                &format_args!("{:?}", self.cpu_access_flags),
            )
            .field("misc_flags", &format_args!("{:?}", self.misc_flags))
            .field(
                "structure_byte_stride",
                &format_args!("{:?}", self.structure_byte_stride),
            )
            .finish()
    }
}
impl ::std::cmp::PartialEq for D3D11_BUFFER_DESC {
    fn eq(&self, other: &Self) -> bool {
        self.byte_width == other.byte_width
            && self.usage == other.usage
            && self.bind_flags == other.bind_flags
            && self.cpu_access_flags == other.cpu_access_flags
            && self.misc_flags == other.misc_flags
            && self.structure_byte_stride == other.structure_byte_stride
    }
}
impl ::std::cmp::Eq for D3D11_BUFFER_DESC {}
unsafe impl ::windows::Abi for D3D11_BUFFER_DESC {
    type Abi = Self;
}
#[repr(C)]
#[allow(non_snake_case)]
#[derive(:: std :: clone :: Clone)]
pub struct D3D11_SUBRESOURCE_DATA {
    pub p_sys_mem: *mut ::std::ffi::c_void,
    pub sys_mem_pitch: u32,
    pub sys_mem_slice_pitch: u32,
}
impl D3D11_SUBRESOURCE_DATA {}
impl ::std::default::Default for D3D11_SUBRESOURCE_DATA {
    fn default() -> Self {
        Self {
            p_sys_mem: ::std::ptr::null_mut(),
            sys_mem_pitch: 0,
            sys_mem_slice_pitch: 0,
        }
    }
}
impl ::std::fmt::Debug for D3D11_SUBRESOURCE_DATA {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("D3D11_SUBRESOURCE_DATA")
            .field("p_sys_mem", &format_args!("{:?}", self.p_sys_mem))
            .field("sys_mem_pitch", &format_args!("{:?}", self.sys_mem_pitch))
            .field(
                "sys_mem_slice_pitch",
                &format_args!("{:?}", self.sys_mem_slice_pitch),
            )
            .finish()
    }
}
impl ::std::cmp::PartialEq for D3D11_SUBRESOURCE_DATA {
    fn eq(&self, other: &Self) -> bool {
        self.p_sys_mem == other.p_sys_mem
            && self.sys_mem_pitch == other.sys_mem_pitch
            && self.sys_mem_slice_pitch == other.sys_mem_slice_pitch
    }
}
impl ::std::cmp::Eq for D3D11_SUBRESOURCE_DATA {}
unsafe impl ::windows::Abi for D3D11_SUBRESOURCE_DATA {
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
pub struct D3D11_RESOURCE_DIMENSION(pub i32);
impl D3D11_RESOURCE_DIMENSION {
    #![allow(non_upper_case_globals)]
    pub const D3D11_RESOURCE_DIMENSION_UNKNOWN: Self = Self(0i32);
    pub const D3D11_RESOURCE_DIMENSION_BUFFER: Self = Self(1i32);
    pub const D3D11_RESOURCE_DIMENSION_TEXTURE1D: Self = Self(2i32);
    pub const D3D11_RESOURCE_DIMENSION_TEXTURE2D: Self = Self(3i32);
    pub const D3D11_RESOURCE_DIMENSION_TEXTURE3D: Self = Self(4i32);
}
impl ::std::convert::From<i32> for D3D11_RESOURCE_DIMENSION {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::Abi for D3D11_RESOURCE_DIMENSION {
    type Abi = Self;
}
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct ID3D11DeviceChild(::windows::IUnknown);
impl ID3D11DeviceChild {}
unsafe impl ::windows::Interface for ID3D11DeviceChild {
    type Vtable = ID3D11DeviceChild_abi;
    const IID: ::windows::Guid = ::windows::Guid::from_values(
        406971848,
        5808,
        18587,
        [188, 200, 68, 207, 176, 213, 222, 174],
    );
}
#[allow(non_snake_case)]
impl ID3D11DeviceChild {
    pub unsafe fn GetDevice(&self, pp_device: *mut ::std::option::Option<ID3D11Device>) {
        (::windows::Interface::vtable(self).3)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(pp_device),
        )
    }
    pub unsafe fn GetPrivateData(
        &self,
        guid: *const ::windows::Guid,
        p_data_size: *mut u32,
        p_data: *mut ::std::ffi::c_void,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).4)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(guid),
            ::std::mem::transmute(p_data_size),
            ::std::mem::transmute(p_data),
        )
    }
    pub unsafe fn SetPrivateData(
        &self,
        guid: *const ::windows::Guid,
        data_size: u32,
        p_data: *const ::std::ffi::c_void,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).5)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(guid),
            ::std::mem::transmute(data_size),
            ::std::mem::transmute(p_data),
        )
    }
    pub unsafe fn SetPrivateDataInterface<
        'a,
        T1__: ::windows::IntoParam<'a, ::windows::IUnknown>,
    >(
        &self,
        guid: *const ::windows::Guid,
        p_data: T1__,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).6)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(guid),
            p_data.into_param().abi(),
        )
    }
}
impl ::std::convert::From<ID3D11DeviceChild> for ::windows::IUnknown {
    fn from(value: ID3D11DeviceChild) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&ID3D11DeviceChild> for ::windows::IUnknown {
    fn from(value: &ID3D11DeviceChild) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::IntoParam<'a, ::windows::IUnknown> for ID3D11DeviceChild {
    fn into_param(self) -> ::windows::Param<'a, ::windows::IUnknown> {
        ::windows::Param::Owned(::std::convert::Into::<::windows::IUnknown>::into(self))
    }
}
impl<'a> ::windows::IntoParam<'a, ::windows::IUnknown> for &'a ID3D11DeviceChild {
    fn into_param(self) -> ::windows::Param<'a, ::windows::IUnknown> {
        ::windows::Param::Owned(::std::convert::Into::<::windows::IUnknown>::into(
            ::std::clone::Clone::clone(self),
        ))
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ID3D11DeviceChild_abi(
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        iid: &::windows::Guid,
        interface: *mut ::windows::RawPtr,
    ) -> ::windows::ErrorCode,
    pub unsafe extern "system" fn(this: ::windows::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::RawPtr, pp_device: *mut ::windows::RawPtr),
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        guid: *const ::windows::Guid,
        p_data_size: *mut u32,
        p_data: *mut ::std::ffi::c_void,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        guid: *const ::windows::Guid,
        data_size: u32,
        p_data: *const ::std::ffi::c_void,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        guid: *const ::windows::Guid,
        p_data: ::windows::RawPtr,
    ) -> ::windows::ErrorCode,
);
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct ID3D11Resource(::windows::IUnknown);
impl ID3D11Resource {}
unsafe impl ::windows::Interface for ID3D11Resource {
    type Vtable = ID3D11Resource_abi;
    const IID: ::windows::Guid = ::windows::Guid::from_values(
        3700319219,
        53547,
        18770,
        [180, 123, 94, 69, 2, 106, 134, 45],
    );
}
#[allow(non_snake_case)]
impl ID3D11Resource {
    pub unsafe fn GetDevice(&self, pp_device: *mut ::std::option::Option<ID3D11Device>) {
        (::windows::Interface::vtable(self).3)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(pp_device),
        )
    }
    pub unsafe fn GetPrivateData(
        &self,
        guid: *const ::windows::Guid,
        p_data_size: *mut u32,
        p_data: *mut ::std::ffi::c_void,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).4)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(guid),
            ::std::mem::transmute(p_data_size),
            ::std::mem::transmute(p_data),
        )
    }
    pub unsafe fn SetPrivateData(
        &self,
        guid: *const ::windows::Guid,
        data_size: u32,
        p_data: *const ::std::ffi::c_void,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).5)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(guid),
            ::std::mem::transmute(data_size),
            ::std::mem::transmute(p_data),
        )
    }
    pub unsafe fn SetPrivateDataInterface<
        'a,
        T1__: ::windows::IntoParam<'a, ::windows::IUnknown>,
    >(
        &self,
        guid: *const ::windows::Guid,
        p_data: T1__,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).6)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(guid),
            p_data.into_param().abi(),
        )
    }
    pub unsafe fn GetType(&self, p_resource_dimension: *mut D3D11_RESOURCE_DIMENSION) {
        (::windows::Interface::vtable(self).7)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(p_resource_dimension),
        )
    }
    pub unsafe fn SetEvictionPriority(&self, eviction_priority: u32) {
        (::windows::Interface::vtable(self).8)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(eviction_priority),
        )
    }
    pub unsafe fn GetEvictionPriority(&self) -> u32 {
        (::windows::Interface::vtable(self).9)(::windows::Abi::abi(self))
    }
}
impl ::std::convert::From<ID3D11Resource> for ::windows::IUnknown {
    fn from(value: ID3D11Resource) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&ID3D11Resource> for ::windows::IUnknown {
    fn from(value: &ID3D11Resource) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::IntoParam<'a, ::windows::IUnknown> for ID3D11Resource {
    fn into_param(self) -> ::windows::Param<'a, ::windows::IUnknown> {
        ::windows::Param::Owned(::std::convert::Into::<::windows::IUnknown>::into(self))
    }
}
impl<'a> ::windows::IntoParam<'a, ::windows::IUnknown> for &'a ID3D11Resource {
    fn into_param(self) -> ::windows::Param<'a, ::windows::IUnknown> {
        ::windows::Param::Owned(::std::convert::Into::<::windows::IUnknown>::into(
            ::std::clone::Clone::clone(self),
        ))
    }
}
impl ::std::convert::From<ID3D11Resource> for ID3D11DeviceChild {
    fn from(value: ID3D11Resource) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&ID3D11Resource> for ID3D11DeviceChild {
    fn from(value: &ID3D11Resource) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::IntoParam<'a, ID3D11DeviceChild> for ID3D11Resource {
    fn into_param(self) -> ::windows::Param<'a, ID3D11DeviceChild> {
        ::windows::Param::Owned(::std::convert::Into::<ID3D11DeviceChild>::into(self))
    }
}
impl<'a> ::windows::IntoParam<'a, ID3D11DeviceChild> for &'a ID3D11Resource {
    fn into_param(self) -> ::windows::Param<'a, ID3D11DeviceChild> {
        ::windows::Param::Owned(::std::convert::Into::<ID3D11DeviceChild>::into(
            ::std::clone::Clone::clone(self),
        ))
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ID3D11Resource_abi(
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        iid: &::windows::Guid,
        interface: *mut ::windows::RawPtr,
    ) -> ::windows::ErrorCode,
    pub unsafe extern "system" fn(this: ::windows::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::RawPtr, pp_device: *mut ::windows::RawPtr),
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        guid: *const ::windows::Guid,
        p_data_size: *mut u32,
        p_data: *mut ::std::ffi::c_void,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        guid: *const ::windows::Guid,
        data_size: u32,
        p_data: *const ::std::ffi::c_void,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        guid: *const ::windows::Guid,
        p_data: ::windows::RawPtr,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        p_resource_dimension: *mut D3D11_RESOURCE_DIMENSION,
    ),
    pub unsafe extern "system" fn(this: ::windows::RawPtr, eviction_priority: u32),
    pub unsafe extern "system" fn(this: ::windows::RawPtr) -> u32,
);
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct ID3D11Buffer(::windows::IUnknown);
impl ID3D11Buffer {}
unsafe impl ::windows::Interface for ID3D11Buffer {
    type Vtable = ID3D11Buffer_abi;
    const IID: ::windows::Guid =
        ::windows::Guid::from_values(1213664133, 53742, 20429, [162, 80, 235, 53, 7, 34, 176, 55]);
}
#[allow(non_snake_case)]
impl ID3D11Buffer {
    pub unsafe fn GetDevice(&self, pp_device: *mut ::std::option::Option<ID3D11Device>) {
        (::windows::Interface::vtable(self).3)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(pp_device),
        )
    }
    pub unsafe fn GetPrivateData(
        &self,
        guid: *const ::windows::Guid,
        p_data_size: *mut u32,
        p_data: *mut ::std::ffi::c_void,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).4)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(guid),
            ::std::mem::transmute(p_data_size),
            ::std::mem::transmute(p_data),
        )
    }
    pub unsafe fn SetPrivateData(
        &self,
        guid: *const ::windows::Guid,
        data_size: u32,
        p_data: *const ::std::ffi::c_void,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).5)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(guid),
            ::std::mem::transmute(data_size),
            ::std::mem::transmute(p_data),
        )
    }
    pub unsafe fn SetPrivateDataInterface<
        'a,
        T1__: ::windows::IntoParam<'a, ::windows::IUnknown>,
    >(
        &self,
        guid: *const ::windows::Guid,
        p_data: T1__,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).6)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(guid),
            p_data.into_param().abi(),
        )
    }
    pub unsafe fn GetType(&self, p_resource_dimension: *mut D3D11_RESOURCE_DIMENSION) {
        (::windows::Interface::vtable(self).7)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(p_resource_dimension),
        )
    }
    pub unsafe fn SetEvictionPriority(&self, eviction_priority: u32) {
        (::windows::Interface::vtable(self).8)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(eviction_priority),
        )
    }
    pub unsafe fn GetEvictionPriority(&self) -> u32 {
        (::windows::Interface::vtable(self).9)(::windows::Abi::abi(self))
    }
    pub unsafe fn GetDesc(&self, p_desc: *mut D3D11_BUFFER_DESC) {
        (::windows::Interface::vtable(self).10)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(p_desc),
        )
    }
}
impl ::std::convert::From<ID3D11Buffer> for ::windows::IUnknown {
    fn from(value: ID3D11Buffer) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&ID3D11Buffer> for ::windows::IUnknown {
    fn from(value: &ID3D11Buffer) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::IntoParam<'a, ::windows::IUnknown> for ID3D11Buffer {
    fn into_param(self) -> ::windows::Param<'a, ::windows::IUnknown> {
        ::windows::Param::Owned(::std::convert::Into::<::windows::IUnknown>::into(self))
    }
}
impl<'a> ::windows::IntoParam<'a, ::windows::IUnknown> for &'a ID3D11Buffer {
    fn into_param(self) -> ::windows::Param<'a, ::windows::IUnknown> {
        ::windows::Param::Owned(::std::convert::Into::<::windows::IUnknown>::into(
            ::std::clone::Clone::clone(self),
        ))
    }
}
impl ::std::convert::From<ID3D11Buffer> for ID3D11Resource {
    fn from(value: ID3D11Buffer) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&ID3D11Buffer> for ID3D11Resource {
    fn from(value: &ID3D11Buffer) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::IntoParam<'a, ID3D11Resource> for ID3D11Buffer {
    fn into_param(self) -> ::windows::Param<'a, ID3D11Resource> {
        ::windows::Param::Owned(::std::convert::Into::<ID3D11Resource>::into(self))
    }
}
impl<'a> ::windows::IntoParam<'a, ID3D11Resource> for &'a ID3D11Buffer {
    fn into_param(self) -> ::windows::Param<'a, ID3D11Resource> {
        ::windows::Param::Owned(::std::convert::Into::<ID3D11Resource>::into(
            ::std::clone::Clone::clone(self),
        ))
    }
}
impl ::std::convert::From<ID3D11Buffer> for ID3D11DeviceChild {
    fn from(value: ID3D11Buffer) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&ID3D11Buffer> for ID3D11DeviceChild {
    fn from(value: &ID3D11Buffer) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::IntoParam<'a, ID3D11DeviceChild> for ID3D11Buffer {
    fn into_param(self) -> ::windows::Param<'a, ID3D11DeviceChild> {
        ::windows::Param::Owned(::std::convert::Into::<ID3D11DeviceChild>::into(self))
    }
}
impl<'a> ::windows::IntoParam<'a, ID3D11DeviceChild> for &'a ID3D11Buffer {
    fn into_param(self) -> ::windows::Param<'a, ID3D11DeviceChild> {
        ::windows::Param::Owned(::std::convert::Into::<ID3D11DeviceChild>::into(
            ::std::clone::Clone::clone(self),
        ))
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ID3D11Buffer_abi(
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        iid: &::windows::Guid,
        interface: *mut ::windows::RawPtr,
    ) -> ::windows::ErrorCode,
    pub unsafe extern "system" fn(this: ::windows::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::RawPtr, pp_device: *mut ::windows::RawPtr),
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        guid: *const ::windows::Guid,
        p_data_size: *mut u32,
        p_data: *mut ::std::ffi::c_void,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        guid: *const ::windows::Guid,
        data_size: u32,
        p_data: *const ::std::ffi::c_void,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        guid: *const ::windows::Guid,
        p_data: ::windows::RawPtr,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        p_resource_dimension: *mut D3D11_RESOURCE_DIMENSION,
    ),
    pub unsafe extern "system" fn(this: ::windows::RawPtr, eviction_priority: u32),
    pub unsafe extern "system" fn(this: ::windows::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::RawPtr, p_desc: *mut D3D11_BUFFER_DESC),
);
#[repr(C)]
#[allow(non_snake_case)]
#[derive(:: std :: clone :: Clone)]
pub struct D3D11_TEXTURE1D_DESC {
    pub width: u32,
    pub mip_levels: u32,
    pub array_size: u32,
    pub format: super::dxgi::DXGI_FORMAT,
    pub usage: D3D11_USAGE,
    pub bind_flags: u32,
    pub cpu_access_flags: u32,
    pub misc_flags: u32,
}
impl D3D11_TEXTURE1D_DESC {}
impl ::std::default::Default for D3D11_TEXTURE1D_DESC {
    fn default() -> Self {
        Self {
            width: 0,
            mip_levels: 0,
            array_size: 0,
            format: ::std::default::Default::default(),
            usage: ::std::default::Default::default(),
            bind_flags: 0,
            cpu_access_flags: 0,
            misc_flags: 0,
        }
    }
}
impl ::std::fmt::Debug for D3D11_TEXTURE1D_DESC {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("D3D11_TEXTURE1D_DESC")
            .field("width", &format_args!("{:?}", self.width))
            .field("mip_levels", &format_args!("{:?}", self.mip_levels))
            .field("array_size", &format_args!("{:?}", self.array_size))
            .field("format", &format_args!("{:?}", self.format))
            .field("usage", &format_args!("{:?}", self.usage))
            .field("bind_flags", &format_args!("{:?}", self.bind_flags))
            .field(
                "cpu_access_flags",
                &format_args!("{:?}", self.cpu_access_flags),
            )
            .field("misc_flags", &format_args!("{:?}", self.misc_flags))
            .finish()
    }
}
impl ::std::cmp::PartialEq for D3D11_TEXTURE1D_DESC {
    fn eq(&self, other: &Self) -> bool {
        self.width == other.width
            && self.mip_levels == other.mip_levels
            && self.array_size == other.array_size
            && self.format == other.format
            && self.usage == other.usage
            && self.bind_flags == other.bind_flags
            && self.cpu_access_flags == other.cpu_access_flags
            && self.misc_flags == other.misc_flags
    }
}
impl ::std::cmp::Eq for D3D11_TEXTURE1D_DESC {}
unsafe impl ::windows::Abi for D3D11_TEXTURE1D_DESC {
    type Abi = Self;
}
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct ID3D11Texture1D(::windows::IUnknown);
impl ID3D11Texture1D {}
unsafe impl ::windows::Interface for ID3D11Texture1D {
    type Vtable = ID3D11Texture1D_abi;
    const IID: ::windows::Guid = ::windows::Guid::from_values(
        4177222695,
        50867,
        20341,
        [164, 200, 67, 154, 242, 239, 86, 76],
    );
}
#[allow(non_snake_case)]
impl ID3D11Texture1D {
    pub unsafe fn GetDevice(&self, pp_device: *mut ::std::option::Option<ID3D11Device>) {
        (::windows::Interface::vtable(self).3)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(pp_device),
        )
    }
    pub unsafe fn GetPrivateData(
        &self,
        guid: *const ::windows::Guid,
        p_data_size: *mut u32,
        p_data: *mut ::std::ffi::c_void,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).4)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(guid),
            ::std::mem::transmute(p_data_size),
            ::std::mem::transmute(p_data),
        )
    }
    pub unsafe fn SetPrivateData(
        &self,
        guid: *const ::windows::Guid,
        data_size: u32,
        p_data: *const ::std::ffi::c_void,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).5)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(guid),
            ::std::mem::transmute(data_size),
            ::std::mem::transmute(p_data),
        )
    }
    pub unsafe fn SetPrivateDataInterface<
        'a,
        T1__: ::windows::IntoParam<'a, ::windows::IUnknown>,
    >(
        &self,
        guid: *const ::windows::Guid,
        p_data: T1__,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).6)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(guid),
            p_data.into_param().abi(),
        )
    }
    pub unsafe fn GetType(&self, p_resource_dimension: *mut D3D11_RESOURCE_DIMENSION) {
        (::windows::Interface::vtable(self).7)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(p_resource_dimension),
        )
    }
    pub unsafe fn SetEvictionPriority(&self, eviction_priority: u32) {
        (::windows::Interface::vtable(self).8)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(eviction_priority),
        )
    }
    pub unsafe fn GetEvictionPriority(&self) -> u32 {
        (::windows::Interface::vtable(self).9)(::windows::Abi::abi(self))
    }
    pub unsafe fn GetDesc(&self, p_desc: *mut D3D11_TEXTURE1D_DESC) {
        (::windows::Interface::vtable(self).10)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(p_desc),
        )
    }
}
impl ::std::convert::From<ID3D11Texture1D> for ::windows::IUnknown {
    fn from(value: ID3D11Texture1D) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&ID3D11Texture1D> for ::windows::IUnknown {
    fn from(value: &ID3D11Texture1D) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::IntoParam<'a, ::windows::IUnknown> for ID3D11Texture1D {
    fn into_param(self) -> ::windows::Param<'a, ::windows::IUnknown> {
        ::windows::Param::Owned(::std::convert::Into::<::windows::IUnknown>::into(self))
    }
}
impl<'a> ::windows::IntoParam<'a, ::windows::IUnknown> for &'a ID3D11Texture1D {
    fn into_param(self) -> ::windows::Param<'a, ::windows::IUnknown> {
        ::windows::Param::Owned(::std::convert::Into::<::windows::IUnknown>::into(
            ::std::clone::Clone::clone(self),
        ))
    }
}
impl ::std::convert::From<ID3D11Texture1D> for ID3D11Resource {
    fn from(value: ID3D11Texture1D) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&ID3D11Texture1D> for ID3D11Resource {
    fn from(value: &ID3D11Texture1D) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::IntoParam<'a, ID3D11Resource> for ID3D11Texture1D {
    fn into_param(self) -> ::windows::Param<'a, ID3D11Resource> {
        ::windows::Param::Owned(::std::convert::Into::<ID3D11Resource>::into(self))
    }
}
impl<'a> ::windows::IntoParam<'a, ID3D11Resource> for &'a ID3D11Texture1D {
    fn into_param(self) -> ::windows::Param<'a, ID3D11Resource> {
        ::windows::Param::Owned(::std::convert::Into::<ID3D11Resource>::into(
            ::std::clone::Clone::clone(self),
        ))
    }
}
impl ::std::convert::From<ID3D11Texture1D> for ID3D11DeviceChild {
    fn from(value: ID3D11Texture1D) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&ID3D11Texture1D> for ID3D11DeviceChild {
    fn from(value: &ID3D11Texture1D) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::IntoParam<'a, ID3D11DeviceChild> for ID3D11Texture1D {
    fn into_param(self) -> ::windows::Param<'a, ID3D11DeviceChild> {
        ::windows::Param::Owned(::std::convert::Into::<ID3D11DeviceChild>::into(self))
    }
}
impl<'a> ::windows::IntoParam<'a, ID3D11DeviceChild> for &'a ID3D11Texture1D {
    fn into_param(self) -> ::windows::Param<'a, ID3D11DeviceChild> {
        ::windows::Param::Owned(::std::convert::Into::<ID3D11DeviceChild>::into(
            ::std::clone::Clone::clone(self),
        ))
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ID3D11Texture1D_abi(
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        iid: &::windows::Guid,
        interface: *mut ::windows::RawPtr,
    ) -> ::windows::ErrorCode,
    pub unsafe extern "system" fn(this: ::windows::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::RawPtr, pp_device: *mut ::windows::RawPtr),
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        guid: *const ::windows::Guid,
        p_data_size: *mut u32,
        p_data: *mut ::std::ffi::c_void,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        guid: *const ::windows::Guid,
        data_size: u32,
        p_data: *const ::std::ffi::c_void,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        guid: *const ::windows::Guid,
        p_data: ::windows::RawPtr,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        p_resource_dimension: *mut D3D11_RESOURCE_DIMENSION,
    ),
    pub unsafe extern "system" fn(this: ::windows::RawPtr, eviction_priority: u32),
    pub unsafe extern "system" fn(this: ::windows::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::RawPtr, p_desc: *mut D3D11_TEXTURE1D_DESC),
);
#[repr(C)]
#[allow(non_snake_case)]
#[derive(:: std :: clone :: Clone)]
pub struct D3D11_TEXTURE2D_DESC {
    pub width: u32,
    pub height: u32,
    pub mip_levels: u32,
    pub array_size: u32,
    pub format: super::dxgi::DXGI_FORMAT,
    pub sample_desc: super::dxgi::DXGI_SAMPLE_DESC,
    pub usage: D3D11_USAGE,
    pub bind_flags: u32,
    pub cpu_access_flags: u32,
    pub misc_flags: u32,
}
impl D3D11_TEXTURE2D_DESC {}
impl ::std::default::Default for D3D11_TEXTURE2D_DESC {
    fn default() -> Self {
        Self {
            width: 0,
            height: 0,
            mip_levels: 0,
            array_size: 0,
            format: ::std::default::Default::default(),
            sample_desc: ::std::default::Default::default(),
            usage: ::std::default::Default::default(),
            bind_flags: 0,
            cpu_access_flags: 0,
            misc_flags: 0,
        }
    }
}
impl ::std::fmt::Debug for D3D11_TEXTURE2D_DESC {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("D3D11_TEXTURE2D_DESC")
            .field("width", &format_args!("{:?}", self.width))
            .field("height", &format_args!("{:?}", self.height))
            .field("mip_levels", &format_args!("{:?}", self.mip_levels))
            .field("array_size", &format_args!("{:?}", self.array_size))
            .field("format", &format_args!("{:?}", self.format))
            .field("sample_desc", &format_args!("{:?}", self.sample_desc))
            .field("usage", &format_args!("{:?}", self.usage))
            .field("bind_flags", &format_args!("{:?}", self.bind_flags))
            .field(
                "cpu_access_flags",
                &format_args!("{:?}", self.cpu_access_flags),
            )
            .field("misc_flags", &format_args!("{:?}", self.misc_flags))
            .finish()
    }
}
impl ::std::cmp::PartialEq for D3D11_TEXTURE2D_DESC {
    fn eq(&self, other: &Self) -> bool {
        self.width == other.width
            && self.height == other.height
            && self.mip_levels == other.mip_levels
            && self.array_size == other.array_size
            && self.format == other.format
            && self.sample_desc == other.sample_desc
            && self.usage == other.usage
            && self.bind_flags == other.bind_flags
            && self.cpu_access_flags == other.cpu_access_flags
            && self.misc_flags == other.misc_flags
    }
}
impl ::std::cmp::Eq for D3D11_TEXTURE2D_DESC {}
unsafe impl ::windows::Abi for D3D11_TEXTURE2D_DESC {
    type Abi = Self;
}
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct ID3D11Texture2D(::windows::IUnknown);
impl ID3D11Texture2D {}
unsafe impl ::windows::Interface for ID3D11Texture2D {
    type Vtable = ID3D11Texture2D_abi;
    const IID: ::windows::Guid = ::windows::Guid::from_values(
        1863690994,
        53768,
        20105,
        [154, 180, 72, 149, 53, 211, 79, 156],
    );
}
#[allow(non_snake_case)]
impl ID3D11Texture2D {
    pub unsafe fn GetDevice(&self, pp_device: *mut ::std::option::Option<ID3D11Device>) {
        (::windows::Interface::vtable(self).3)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(pp_device),
        )
    }
    pub unsafe fn GetPrivateData(
        &self,
        guid: *const ::windows::Guid,
        p_data_size: *mut u32,
        p_data: *mut ::std::ffi::c_void,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).4)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(guid),
            ::std::mem::transmute(p_data_size),
            ::std::mem::transmute(p_data),
        )
    }
    pub unsafe fn SetPrivateData(
        &self,
        guid: *const ::windows::Guid,
        data_size: u32,
        p_data: *const ::std::ffi::c_void,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).5)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(guid),
            ::std::mem::transmute(data_size),
            ::std::mem::transmute(p_data),
        )
    }
    pub unsafe fn SetPrivateDataInterface<
        'a,
        T1__: ::windows::IntoParam<'a, ::windows::IUnknown>,
    >(
        &self,
        guid: *const ::windows::Guid,
        p_data: T1__,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).6)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(guid),
            p_data.into_param().abi(),
        )
    }
    pub unsafe fn GetType(&self, p_resource_dimension: *mut D3D11_RESOURCE_DIMENSION) {
        (::windows::Interface::vtable(self).7)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(p_resource_dimension),
        )
    }
    pub unsafe fn SetEvictionPriority(&self, eviction_priority: u32) {
        (::windows::Interface::vtable(self).8)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(eviction_priority),
        )
    }
    pub unsafe fn GetEvictionPriority(&self) -> u32 {
        (::windows::Interface::vtable(self).9)(::windows::Abi::abi(self))
    }
    pub unsafe fn GetDesc(&self, p_desc: *mut D3D11_TEXTURE2D_DESC) {
        (::windows::Interface::vtable(self).10)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(p_desc),
        )
    }
}
impl ::std::convert::From<ID3D11Texture2D> for ::windows::IUnknown {
    fn from(value: ID3D11Texture2D) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&ID3D11Texture2D> for ::windows::IUnknown {
    fn from(value: &ID3D11Texture2D) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::IntoParam<'a, ::windows::IUnknown> for ID3D11Texture2D {
    fn into_param(self) -> ::windows::Param<'a, ::windows::IUnknown> {
        ::windows::Param::Owned(::std::convert::Into::<::windows::IUnknown>::into(self))
    }
}
impl<'a> ::windows::IntoParam<'a, ::windows::IUnknown> for &'a ID3D11Texture2D {
    fn into_param(self) -> ::windows::Param<'a, ::windows::IUnknown> {
        ::windows::Param::Owned(::std::convert::Into::<::windows::IUnknown>::into(
            ::std::clone::Clone::clone(self),
        ))
    }
}
impl ::std::convert::From<ID3D11Texture2D> for ID3D11Resource {
    fn from(value: ID3D11Texture2D) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&ID3D11Texture2D> for ID3D11Resource {
    fn from(value: &ID3D11Texture2D) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::IntoParam<'a, ID3D11Resource> for ID3D11Texture2D {
    fn into_param(self) -> ::windows::Param<'a, ID3D11Resource> {
        ::windows::Param::Owned(::std::convert::Into::<ID3D11Resource>::into(self))
    }
}
impl<'a> ::windows::IntoParam<'a, ID3D11Resource> for &'a ID3D11Texture2D {
    fn into_param(self) -> ::windows::Param<'a, ID3D11Resource> {
        ::windows::Param::Owned(::std::convert::Into::<ID3D11Resource>::into(
            ::std::clone::Clone::clone(self),
        ))
    }
}
impl ::std::convert::From<ID3D11Texture2D> for ID3D11DeviceChild {
    fn from(value: ID3D11Texture2D) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&ID3D11Texture2D> for ID3D11DeviceChild {
    fn from(value: &ID3D11Texture2D) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::IntoParam<'a, ID3D11DeviceChild> for ID3D11Texture2D {
    fn into_param(self) -> ::windows::Param<'a, ID3D11DeviceChild> {
        ::windows::Param::Owned(::std::convert::Into::<ID3D11DeviceChild>::into(self))
    }
}
impl<'a> ::windows::IntoParam<'a, ID3D11DeviceChild> for &'a ID3D11Texture2D {
    fn into_param(self) -> ::windows::Param<'a, ID3D11DeviceChild> {
        ::windows::Param::Owned(::std::convert::Into::<ID3D11DeviceChild>::into(
            ::std::clone::Clone::clone(self),
        ))
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ID3D11Texture2D_abi(
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        iid: &::windows::Guid,
        interface: *mut ::windows::RawPtr,
    ) -> ::windows::ErrorCode,
    pub unsafe extern "system" fn(this: ::windows::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::RawPtr, pp_device: *mut ::windows::RawPtr),
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        guid: *const ::windows::Guid,
        p_data_size: *mut u32,
        p_data: *mut ::std::ffi::c_void,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        guid: *const ::windows::Guid,
        data_size: u32,
        p_data: *const ::std::ffi::c_void,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        guid: *const ::windows::Guid,
        p_data: ::windows::RawPtr,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        p_resource_dimension: *mut D3D11_RESOURCE_DIMENSION,
    ),
    pub unsafe extern "system" fn(this: ::windows::RawPtr, eviction_priority: u32),
    pub unsafe extern "system" fn(this: ::windows::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::RawPtr, p_desc: *mut D3D11_TEXTURE2D_DESC),
);
#[repr(C)]
#[allow(non_snake_case)]
#[derive(:: std :: clone :: Clone)]
pub struct D3D11_TEXTURE3D_DESC {
    pub width: u32,
    pub height: u32,
    pub depth: u32,
    pub mip_levels: u32,
    pub format: super::dxgi::DXGI_FORMAT,
    pub usage: D3D11_USAGE,
    pub bind_flags: u32,
    pub cpu_access_flags: u32,
    pub misc_flags: u32,
}
impl D3D11_TEXTURE3D_DESC {}
impl ::std::default::Default for D3D11_TEXTURE3D_DESC {
    fn default() -> Self {
        Self {
            width: 0,
            height: 0,
            depth: 0,
            mip_levels: 0,
            format: ::std::default::Default::default(),
            usage: ::std::default::Default::default(),
            bind_flags: 0,
            cpu_access_flags: 0,
            misc_flags: 0,
        }
    }
}
impl ::std::fmt::Debug for D3D11_TEXTURE3D_DESC {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("D3D11_TEXTURE3D_DESC")
            .field("width", &format_args!("{:?}", self.width))
            .field("height", &format_args!("{:?}", self.height))
            .field("depth", &format_args!("{:?}", self.depth))
            .field("mip_levels", &format_args!("{:?}", self.mip_levels))
            .field("format", &format_args!("{:?}", self.format))
            .field("usage", &format_args!("{:?}", self.usage))
            .field("bind_flags", &format_args!("{:?}", self.bind_flags))
            .field(
                "cpu_access_flags",
                &format_args!("{:?}", self.cpu_access_flags),
            )
            .field("misc_flags", &format_args!("{:?}", self.misc_flags))
            .finish()
    }
}
impl ::std::cmp::PartialEq for D3D11_TEXTURE3D_DESC {
    fn eq(&self, other: &Self) -> bool {
        self.width == other.width
            && self.height == other.height
            && self.depth == other.depth
            && self.mip_levels == other.mip_levels
            && self.format == other.format
            && self.usage == other.usage
            && self.bind_flags == other.bind_flags
            && self.cpu_access_flags == other.cpu_access_flags
            && self.misc_flags == other.misc_flags
    }
}
impl ::std::cmp::Eq for D3D11_TEXTURE3D_DESC {}
unsafe impl ::windows::Abi for D3D11_TEXTURE3D_DESC {
    type Abi = Self;
}
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct ID3D11Texture3D(::windows::IUnknown);
impl ID3D11Texture3D {}
unsafe impl ::windows::Interface for ID3D11Texture3D {
    type Vtable = ID3D11Texture3D_abi;
    const IID: ::windows::Guid = ::windows::Guid::from_values(
        58623598,
        62829,
        17239,
        [168, 175, 157, 171, 190, 110, 37, 14],
    );
}
#[allow(non_snake_case)]
impl ID3D11Texture3D {
    pub unsafe fn GetDevice(&self, pp_device: *mut ::std::option::Option<ID3D11Device>) {
        (::windows::Interface::vtable(self).3)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(pp_device),
        )
    }
    pub unsafe fn GetPrivateData(
        &self,
        guid: *const ::windows::Guid,
        p_data_size: *mut u32,
        p_data: *mut ::std::ffi::c_void,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).4)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(guid),
            ::std::mem::transmute(p_data_size),
            ::std::mem::transmute(p_data),
        )
    }
    pub unsafe fn SetPrivateData(
        &self,
        guid: *const ::windows::Guid,
        data_size: u32,
        p_data: *const ::std::ffi::c_void,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).5)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(guid),
            ::std::mem::transmute(data_size),
            ::std::mem::transmute(p_data),
        )
    }
    pub unsafe fn SetPrivateDataInterface<
        'a,
        T1__: ::windows::IntoParam<'a, ::windows::IUnknown>,
    >(
        &self,
        guid: *const ::windows::Guid,
        p_data: T1__,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).6)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(guid),
            p_data.into_param().abi(),
        )
    }
    pub unsafe fn GetType(&self, p_resource_dimension: *mut D3D11_RESOURCE_DIMENSION) {
        (::windows::Interface::vtable(self).7)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(p_resource_dimension),
        )
    }
    pub unsafe fn SetEvictionPriority(&self, eviction_priority: u32) {
        (::windows::Interface::vtable(self).8)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(eviction_priority),
        )
    }
    pub unsafe fn GetEvictionPriority(&self) -> u32 {
        (::windows::Interface::vtable(self).9)(::windows::Abi::abi(self))
    }
    pub unsafe fn GetDesc(&self, p_desc: *mut D3D11_TEXTURE3D_DESC) {
        (::windows::Interface::vtable(self).10)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(p_desc),
        )
    }
}
impl ::std::convert::From<ID3D11Texture3D> for ::windows::IUnknown {
    fn from(value: ID3D11Texture3D) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&ID3D11Texture3D> for ::windows::IUnknown {
    fn from(value: &ID3D11Texture3D) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::IntoParam<'a, ::windows::IUnknown> for ID3D11Texture3D {
    fn into_param(self) -> ::windows::Param<'a, ::windows::IUnknown> {
        ::windows::Param::Owned(::std::convert::Into::<::windows::IUnknown>::into(self))
    }
}
impl<'a> ::windows::IntoParam<'a, ::windows::IUnknown> for &'a ID3D11Texture3D {
    fn into_param(self) -> ::windows::Param<'a, ::windows::IUnknown> {
        ::windows::Param::Owned(::std::convert::Into::<::windows::IUnknown>::into(
            ::std::clone::Clone::clone(self),
        ))
    }
}
impl ::std::convert::From<ID3D11Texture3D> for ID3D11Resource {
    fn from(value: ID3D11Texture3D) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&ID3D11Texture3D> for ID3D11Resource {
    fn from(value: &ID3D11Texture3D) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::IntoParam<'a, ID3D11Resource> for ID3D11Texture3D {
    fn into_param(self) -> ::windows::Param<'a, ID3D11Resource> {
        ::windows::Param::Owned(::std::convert::Into::<ID3D11Resource>::into(self))
    }
}
impl<'a> ::windows::IntoParam<'a, ID3D11Resource> for &'a ID3D11Texture3D {
    fn into_param(self) -> ::windows::Param<'a, ID3D11Resource> {
        ::windows::Param::Owned(::std::convert::Into::<ID3D11Resource>::into(
            ::std::clone::Clone::clone(self),
        ))
    }
}
impl ::std::convert::From<ID3D11Texture3D> for ID3D11DeviceChild {
    fn from(value: ID3D11Texture3D) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&ID3D11Texture3D> for ID3D11DeviceChild {
    fn from(value: &ID3D11Texture3D) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::IntoParam<'a, ID3D11DeviceChild> for ID3D11Texture3D {
    fn into_param(self) -> ::windows::Param<'a, ID3D11DeviceChild> {
        ::windows::Param::Owned(::std::convert::Into::<ID3D11DeviceChild>::into(self))
    }
}
impl<'a> ::windows::IntoParam<'a, ID3D11DeviceChild> for &'a ID3D11Texture3D {
    fn into_param(self) -> ::windows::Param<'a, ID3D11DeviceChild> {
        ::windows::Param::Owned(::std::convert::Into::<ID3D11DeviceChild>::into(
            ::std::clone::Clone::clone(self),
        ))
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ID3D11Texture3D_abi(
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        iid: &::windows::Guid,
        interface: *mut ::windows::RawPtr,
    ) -> ::windows::ErrorCode,
    pub unsafe extern "system" fn(this: ::windows::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::RawPtr, pp_device: *mut ::windows::RawPtr),
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        guid: *const ::windows::Guid,
        p_data_size: *mut u32,
        p_data: *mut ::std::ffi::c_void,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        guid: *const ::windows::Guid,
        data_size: u32,
        p_data: *const ::std::ffi::c_void,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        guid: *const ::windows::Guid,
        p_data: ::windows::RawPtr,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        p_resource_dimension: *mut D3D11_RESOURCE_DIMENSION,
    ),
    pub unsafe extern "system" fn(this: ::windows::RawPtr, eviction_priority: u32),
    pub unsafe extern "system" fn(this: ::windows::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::RawPtr, p_desc: *mut D3D11_TEXTURE3D_DESC),
);
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
pub struct D3D_SRV_DIMENSION(pub i32);
impl D3D_SRV_DIMENSION {
    #![allow(non_upper_case_globals)]
    pub const D3D_SRV_DIMENSION_UNKNOWN: Self = Self(0i32);
    pub const D3D_SRV_DIMENSION_BUFFER: Self = Self(1i32);
    pub const D3D_SRV_DIMENSION_TEXTURE1D: Self = Self(2i32);
    pub const D3D_SRV_DIMENSION_TEXTURE1DARRAY: Self = Self(3i32);
    pub const D3D_SRV_DIMENSION_TEXTURE2D: Self = Self(4i32);
    pub const D3D_SRV_DIMENSION_TEXTURE2DARRAY: Self = Self(5i32);
    pub const D3D_SRV_DIMENSION_TEXTURE2DMS: Self = Self(6i32);
    pub const D3D_SRV_DIMENSION_TEXTURE2DMSARRAY: Self = Self(7i32);
    pub const D3D_SRV_DIMENSION_TEXTURE3D: Self = Self(8i32);
    pub const D3D_SRV_DIMENSION_TEXTURECUBE: Self = Self(9i32);
    pub const D3D_SRV_DIMENSION_TEXTURECUBEARRAY: Self = Self(10i32);
    pub const D3D_SRV_DIMENSION_BUFFEREX: Self = Self(11i32);
    pub const D3D10_SRV_DIMENSION_UNKNOWN: Self = Self(0i32);
    pub const D3D10_SRV_DIMENSION_BUFFER: Self = Self(1i32);
    pub const D3D10_SRV_DIMENSION_TEXTURE1D: Self = Self(2i32);
    pub const D3D10_SRV_DIMENSION_TEXTURE1DARRAY: Self = Self(3i32);
    pub const D3D10_SRV_DIMENSION_TEXTURE2D: Self = Self(4i32);
    pub const D3D10_SRV_DIMENSION_TEXTURE2DARRAY: Self = Self(5i32);
    pub const D3D10_SRV_DIMENSION_TEXTURE2DMS: Self = Self(6i32);
    pub const D3D10_SRV_DIMENSION_TEXTURE2DMSARRAY: Self = Self(7i32);
    pub const D3D10_SRV_DIMENSION_TEXTURE3D: Self = Self(8i32);
    pub const D3D10_SRV_DIMENSION_TEXTURECUBE: Self = Self(9i32);
    pub const D3D10_1_SRV_DIMENSION_UNKNOWN: Self = Self(0i32);
    pub const D3D10_1_SRV_DIMENSION_BUFFER: Self = Self(1i32);
    pub const D3D10_1_SRV_DIMENSION_TEXTURE1D: Self = Self(2i32);
    pub const D3D10_1_SRV_DIMENSION_TEXTURE1DARRAY: Self = Self(3i32);
    pub const D3D10_1_SRV_DIMENSION_TEXTURE2D: Self = Self(4i32);
    pub const D3D10_1_SRV_DIMENSION_TEXTURE2DARRAY: Self = Self(5i32);
    pub const D3D10_1_SRV_DIMENSION_TEXTURE2DMS: Self = Self(6i32);
    pub const D3D10_1_SRV_DIMENSION_TEXTURE2DMSARRAY: Self = Self(7i32);
    pub const D3D10_1_SRV_DIMENSION_TEXTURE3D: Self = Self(8i32);
    pub const D3D10_1_SRV_DIMENSION_TEXTURECUBE: Self = Self(9i32);
    pub const D3D10_1_SRV_DIMENSION_TEXTURECUBEARRAY: Self = Self(10i32);
    pub const D3D11_SRV_DIMENSION_UNKNOWN: Self = Self(0i32);
    pub const D3D11_SRV_DIMENSION_BUFFER: Self = Self(1i32);
    pub const D3D11_SRV_DIMENSION_TEXTURE1D: Self = Self(2i32);
    pub const D3D11_SRV_DIMENSION_TEXTURE1DARRAY: Self = Self(3i32);
    pub const D3D11_SRV_DIMENSION_TEXTURE2D: Self = Self(4i32);
    pub const D3D11_SRV_DIMENSION_TEXTURE2DARRAY: Self = Self(5i32);
    pub const D3D11_SRV_DIMENSION_TEXTURE2DMS: Self = Self(6i32);
    pub const D3D11_SRV_DIMENSION_TEXTURE2DMSARRAY: Self = Self(7i32);
    pub const D3D11_SRV_DIMENSION_TEXTURE3D: Self = Self(8i32);
    pub const D3D11_SRV_DIMENSION_TEXTURECUBE: Self = Self(9i32);
    pub const D3D11_SRV_DIMENSION_TEXTURECUBEARRAY: Self = Self(10i32);
    pub const D3D11_SRV_DIMENSION_BUFFEREX: Self = Self(11i32);
}
impl ::std::convert::From<i32> for D3D_SRV_DIMENSION {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::Abi for D3D_SRV_DIMENSION {
    type Abi = Self;
}
#[repr(C)]
#[allow(non_snake_case)]
#[derive(:: std :: clone :: Clone)]
pub struct D3D11_SHADER_RESOURCE_VIEW_DESC {
    pub format: super::dxgi::DXGI_FORMAT,
    pub view_dimension: D3D_SRV_DIMENSION,
    pub anonymous: ::windows::NOT_YET_SUPPORTED_TYPE,
}
impl D3D11_SHADER_RESOURCE_VIEW_DESC {}
impl ::std::default::Default for D3D11_SHADER_RESOURCE_VIEW_DESC {
    fn default() -> Self {
        Self {
            format: ::std::default::Default::default(),
            view_dimension: ::std::default::Default::default(),
            anonymous: ::std::default::Default::default(),
        }
    }
}
impl ::std::fmt::Debug for D3D11_SHADER_RESOURCE_VIEW_DESC {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("D3D11_SHADER_RESOURCE_VIEW_DESC")
            .field("format", &format_args!("{:?}", self.format))
            .field("view_dimension", &format_args!("{:?}", self.view_dimension))
            .field("anonymous", &format_args!("{:?}", self.anonymous))
            .finish()
    }
}
impl ::std::cmp::PartialEq for D3D11_SHADER_RESOURCE_VIEW_DESC {
    fn eq(&self, other: &Self) -> bool {
        self.format == other.format
            && self.view_dimension == other.view_dimension
            && self.anonymous == other.anonymous
    }
}
impl ::std::cmp::Eq for D3D11_SHADER_RESOURCE_VIEW_DESC {}
unsafe impl ::windows::Abi for D3D11_SHADER_RESOURCE_VIEW_DESC {
    type Abi = Self;
}
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct ID3D11View(::windows::IUnknown);
impl ID3D11View {}
unsafe impl ::windows::Interface for ID3D11View {
    type Vtable = ID3D11View_abi;
    const IID: ::windows::Guid = ::windows::Guid::from_values(
        2208109078,
        47918,
        16683,
        [183, 244, 169, 219, 235, 224, 142, 209],
    );
}
#[allow(non_snake_case)]
impl ID3D11View {
    pub unsafe fn GetDevice(&self, pp_device: *mut ::std::option::Option<ID3D11Device>) {
        (::windows::Interface::vtable(self).3)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(pp_device),
        )
    }
    pub unsafe fn GetPrivateData(
        &self,
        guid: *const ::windows::Guid,
        p_data_size: *mut u32,
        p_data: *mut ::std::ffi::c_void,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).4)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(guid),
            ::std::mem::transmute(p_data_size),
            ::std::mem::transmute(p_data),
        )
    }
    pub unsafe fn SetPrivateData(
        &self,
        guid: *const ::windows::Guid,
        data_size: u32,
        p_data: *const ::std::ffi::c_void,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).5)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(guid),
            ::std::mem::transmute(data_size),
            ::std::mem::transmute(p_data),
        )
    }
    pub unsafe fn SetPrivateDataInterface<
        'a,
        T1__: ::windows::IntoParam<'a, ::windows::IUnknown>,
    >(
        &self,
        guid: *const ::windows::Guid,
        p_data: T1__,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).6)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(guid),
            p_data.into_param().abi(),
        )
    }
    pub unsafe fn GetResource(&self, pp_resource: *mut ::std::option::Option<ID3D11Resource>) {
        (::windows::Interface::vtable(self).7)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(pp_resource),
        )
    }
}
impl ::std::convert::From<ID3D11View> for ::windows::IUnknown {
    fn from(value: ID3D11View) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&ID3D11View> for ::windows::IUnknown {
    fn from(value: &ID3D11View) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::IntoParam<'a, ::windows::IUnknown> for ID3D11View {
    fn into_param(self) -> ::windows::Param<'a, ::windows::IUnknown> {
        ::windows::Param::Owned(::std::convert::Into::<::windows::IUnknown>::into(self))
    }
}
impl<'a> ::windows::IntoParam<'a, ::windows::IUnknown> for &'a ID3D11View {
    fn into_param(self) -> ::windows::Param<'a, ::windows::IUnknown> {
        ::windows::Param::Owned(::std::convert::Into::<::windows::IUnknown>::into(
            ::std::clone::Clone::clone(self),
        ))
    }
}
impl ::std::convert::From<ID3D11View> for ID3D11DeviceChild {
    fn from(value: ID3D11View) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&ID3D11View> for ID3D11DeviceChild {
    fn from(value: &ID3D11View) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::IntoParam<'a, ID3D11DeviceChild> for ID3D11View {
    fn into_param(self) -> ::windows::Param<'a, ID3D11DeviceChild> {
        ::windows::Param::Owned(::std::convert::Into::<ID3D11DeviceChild>::into(self))
    }
}
impl<'a> ::windows::IntoParam<'a, ID3D11DeviceChild> for &'a ID3D11View {
    fn into_param(self) -> ::windows::Param<'a, ID3D11DeviceChild> {
        ::windows::Param::Owned(::std::convert::Into::<ID3D11DeviceChild>::into(
            ::std::clone::Clone::clone(self),
        ))
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ID3D11View_abi(
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        iid: &::windows::Guid,
        interface: *mut ::windows::RawPtr,
    ) -> ::windows::ErrorCode,
    pub unsafe extern "system" fn(this: ::windows::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::RawPtr, pp_device: *mut ::windows::RawPtr),
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        guid: *const ::windows::Guid,
        p_data_size: *mut u32,
        p_data: *mut ::std::ffi::c_void,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        guid: *const ::windows::Guid,
        data_size: u32,
        p_data: *const ::std::ffi::c_void,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        guid: *const ::windows::Guid,
        p_data: ::windows::RawPtr,
    ) -> ::windows::ErrorCode,
    pub unsafe extern "system" fn(this: ::windows::RawPtr, pp_resource: *mut ::windows::RawPtr),
);
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct ID3D11ShaderResourceView(::windows::IUnknown);
impl ID3D11ShaderResourceView {}
unsafe impl ::windows::Interface for ID3D11ShaderResourceView {
    type Vtable = ID3D11ShaderResourceView_abi;
    const IID: ::windows::Guid = ::windows::Guid::from_values(
        2967498720,
        33170,
        19994,
        [177, 202, 54, 215, 65, 71, 16, 178],
    );
}
#[allow(non_snake_case)]
impl ID3D11ShaderResourceView {
    pub unsafe fn GetDevice(&self, pp_device: *mut ::std::option::Option<ID3D11Device>) {
        (::windows::Interface::vtable(self).3)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(pp_device),
        )
    }
    pub unsafe fn GetPrivateData(
        &self,
        guid: *const ::windows::Guid,
        p_data_size: *mut u32,
        p_data: *mut ::std::ffi::c_void,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).4)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(guid),
            ::std::mem::transmute(p_data_size),
            ::std::mem::transmute(p_data),
        )
    }
    pub unsafe fn SetPrivateData(
        &self,
        guid: *const ::windows::Guid,
        data_size: u32,
        p_data: *const ::std::ffi::c_void,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).5)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(guid),
            ::std::mem::transmute(data_size),
            ::std::mem::transmute(p_data),
        )
    }
    pub unsafe fn SetPrivateDataInterface<
        'a,
        T1__: ::windows::IntoParam<'a, ::windows::IUnknown>,
    >(
        &self,
        guid: *const ::windows::Guid,
        p_data: T1__,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).6)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(guid),
            p_data.into_param().abi(),
        )
    }
    pub unsafe fn GetResource(&self, pp_resource: *mut ::std::option::Option<ID3D11Resource>) {
        (::windows::Interface::vtable(self).7)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(pp_resource),
        )
    }
    pub unsafe fn GetDesc(&self, p_desc: *mut D3D11_SHADER_RESOURCE_VIEW_DESC) {
        (::windows::Interface::vtable(self).8)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(p_desc),
        )
    }
}
impl ::std::convert::From<ID3D11ShaderResourceView> for ::windows::IUnknown {
    fn from(value: ID3D11ShaderResourceView) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&ID3D11ShaderResourceView> for ::windows::IUnknown {
    fn from(value: &ID3D11ShaderResourceView) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::IntoParam<'a, ::windows::IUnknown> for ID3D11ShaderResourceView {
    fn into_param(self) -> ::windows::Param<'a, ::windows::IUnknown> {
        ::windows::Param::Owned(::std::convert::Into::<::windows::IUnknown>::into(self))
    }
}
impl<'a> ::windows::IntoParam<'a, ::windows::IUnknown> for &'a ID3D11ShaderResourceView {
    fn into_param(self) -> ::windows::Param<'a, ::windows::IUnknown> {
        ::windows::Param::Owned(::std::convert::Into::<::windows::IUnknown>::into(
            ::std::clone::Clone::clone(self),
        ))
    }
}
impl ::std::convert::From<ID3D11ShaderResourceView> for ID3D11View {
    fn from(value: ID3D11ShaderResourceView) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&ID3D11ShaderResourceView> for ID3D11View {
    fn from(value: &ID3D11ShaderResourceView) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::IntoParam<'a, ID3D11View> for ID3D11ShaderResourceView {
    fn into_param(self) -> ::windows::Param<'a, ID3D11View> {
        ::windows::Param::Owned(::std::convert::Into::<ID3D11View>::into(self))
    }
}
impl<'a> ::windows::IntoParam<'a, ID3D11View> for &'a ID3D11ShaderResourceView {
    fn into_param(self) -> ::windows::Param<'a, ID3D11View> {
        ::windows::Param::Owned(::std::convert::Into::<ID3D11View>::into(
            ::std::clone::Clone::clone(self),
        ))
    }
}
impl ::std::convert::From<ID3D11ShaderResourceView> for ID3D11DeviceChild {
    fn from(value: ID3D11ShaderResourceView) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&ID3D11ShaderResourceView> for ID3D11DeviceChild {
    fn from(value: &ID3D11ShaderResourceView) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::IntoParam<'a, ID3D11DeviceChild> for ID3D11ShaderResourceView {
    fn into_param(self) -> ::windows::Param<'a, ID3D11DeviceChild> {
        ::windows::Param::Owned(::std::convert::Into::<ID3D11DeviceChild>::into(self))
    }
}
impl<'a> ::windows::IntoParam<'a, ID3D11DeviceChild> for &'a ID3D11ShaderResourceView {
    fn into_param(self) -> ::windows::Param<'a, ID3D11DeviceChild> {
        ::windows::Param::Owned(::std::convert::Into::<ID3D11DeviceChild>::into(
            ::std::clone::Clone::clone(self),
        ))
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ID3D11ShaderResourceView_abi(
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        iid: &::windows::Guid,
        interface: *mut ::windows::RawPtr,
    ) -> ::windows::ErrorCode,
    pub unsafe extern "system" fn(this: ::windows::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::RawPtr, pp_device: *mut ::windows::RawPtr),
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        guid: *const ::windows::Guid,
        p_data_size: *mut u32,
        p_data: *mut ::std::ffi::c_void,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        guid: *const ::windows::Guid,
        data_size: u32,
        p_data: *const ::std::ffi::c_void,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        guid: *const ::windows::Guid,
        p_data: ::windows::RawPtr,
    ) -> ::windows::ErrorCode,
    pub unsafe extern "system" fn(this: ::windows::RawPtr, pp_resource: *mut ::windows::RawPtr),
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        p_desc: *mut D3D11_SHADER_RESOURCE_VIEW_DESC,
    ),
);
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
pub struct D3D11_UAV_DIMENSION(pub i32);
impl D3D11_UAV_DIMENSION {
    #![allow(non_upper_case_globals)]
    pub const D3D11_UAV_DIMENSION_UNKNOWN: Self = Self(0i32);
    pub const D3D11_UAV_DIMENSION_BUFFER: Self = Self(1i32);
    pub const D3D11_UAV_DIMENSION_TEXTURE1D: Self = Self(2i32);
    pub const D3D11_UAV_DIMENSION_TEXTURE1DARRAY: Self = Self(3i32);
    pub const D3D11_UAV_DIMENSION_TEXTURE2D: Self = Self(4i32);
    pub const D3D11_UAV_DIMENSION_TEXTURE2DARRAY: Self = Self(5i32);
    pub const D3D11_UAV_DIMENSION_TEXTURE3D: Self = Self(8i32);
}
impl ::std::convert::From<i32> for D3D11_UAV_DIMENSION {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::Abi for D3D11_UAV_DIMENSION {
    type Abi = Self;
}
#[repr(C)]
#[allow(non_snake_case)]
#[derive(:: std :: clone :: Clone)]
pub struct D3D11_UNORDERED_ACCESS_VIEW_DESC {
    pub format: super::dxgi::DXGI_FORMAT,
    pub view_dimension: D3D11_UAV_DIMENSION,
    pub anonymous: ::windows::NOT_YET_SUPPORTED_TYPE,
}
impl D3D11_UNORDERED_ACCESS_VIEW_DESC {}
impl ::std::default::Default for D3D11_UNORDERED_ACCESS_VIEW_DESC {
    fn default() -> Self {
        Self {
            format: ::std::default::Default::default(),
            view_dimension: ::std::default::Default::default(),
            anonymous: ::std::default::Default::default(),
        }
    }
}
impl ::std::fmt::Debug for D3D11_UNORDERED_ACCESS_VIEW_DESC {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("D3D11_UNORDERED_ACCESS_VIEW_DESC")
            .field("format", &format_args!("{:?}", self.format))
            .field("view_dimension", &format_args!("{:?}", self.view_dimension))
            .field("anonymous", &format_args!("{:?}", self.anonymous))
            .finish()
    }
}
impl ::std::cmp::PartialEq for D3D11_UNORDERED_ACCESS_VIEW_DESC {
    fn eq(&self, other: &Self) -> bool {
        self.format == other.format
            && self.view_dimension == other.view_dimension
            && self.anonymous == other.anonymous
    }
}
impl ::std::cmp::Eq for D3D11_UNORDERED_ACCESS_VIEW_DESC {}
unsafe impl ::windows::Abi for D3D11_UNORDERED_ACCESS_VIEW_DESC {
    type Abi = Self;
}
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct ID3D11UnorderedAccessView(::windows::IUnknown);
impl ID3D11UnorderedAccessView {}
unsafe impl ::windows::Interface for ID3D11UnorderedAccessView {
    type Vtable = ID3D11UnorderedAccessView_abi;
    const IID: ::windows::Guid =
        ::windows::Guid::from_values(682423561, 32604, 18678, [134, 17, 243, 22, 1, 10, 99, 128]);
}
#[allow(non_snake_case)]
impl ID3D11UnorderedAccessView {
    pub unsafe fn GetDevice(&self, pp_device: *mut ::std::option::Option<ID3D11Device>) {
        (::windows::Interface::vtable(self).3)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(pp_device),
        )
    }
    pub unsafe fn GetPrivateData(
        &self,
        guid: *const ::windows::Guid,
        p_data_size: *mut u32,
        p_data: *mut ::std::ffi::c_void,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).4)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(guid),
            ::std::mem::transmute(p_data_size),
            ::std::mem::transmute(p_data),
        )
    }
    pub unsafe fn SetPrivateData(
        &self,
        guid: *const ::windows::Guid,
        data_size: u32,
        p_data: *const ::std::ffi::c_void,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).5)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(guid),
            ::std::mem::transmute(data_size),
            ::std::mem::transmute(p_data),
        )
    }
    pub unsafe fn SetPrivateDataInterface<
        'a,
        T1__: ::windows::IntoParam<'a, ::windows::IUnknown>,
    >(
        &self,
        guid: *const ::windows::Guid,
        p_data: T1__,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).6)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(guid),
            p_data.into_param().abi(),
        )
    }
    pub unsafe fn GetResource(&self, pp_resource: *mut ::std::option::Option<ID3D11Resource>) {
        (::windows::Interface::vtable(self).7)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(pp_resource),
        )
    }
    pub unsafe fn GetDesc(&self, p_desc: *mut D3D11_UNORDERED_ACCESS_VIEW_DESC) {
        (::windows::Interface::vtable(self).8)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(p_desc),
        )
    }
}
impl ::std::convert::From<ID3D11UnorderedAccessView> for ::windows::IUnknown {
    fn from(value: ID3D11UnorderedAccessView) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&ID3D11UnorderedAccessView> for ::windows::IUnknown {
    fn from(value: &ID3D11UnorderedAccessView) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::IntoParam<'a, ::windows::IUnknown> for ID3D11UnorderedAccessView {
    fn into_param(self) -> ::windows::Param<'a, ::windows::IUnknown> {
        ::windows::Param::Owned(::std::convert::Into::<::windows::IUnknown>::into(self))
    }
}
impl<'a> ::windows::IntoParam<'a, ::windows::IUnknown> for &'a ID3D11UnorderedAccessView {
    fn into_param(self) -> ::windows::Param<'a, ::windows::IUnknown> {
        ::windows::Param::Owned(::std::convert::Into::<::windows::IUnknown>::into(
            ::std::clone::Clone::clone(self),
        ))
    }
}
impl ::std::convert::From<ID3D11UnorderedAccessView> for ID3D11View {
    fn from(value: ID3D11UnorderedAccessView) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&ID3D11UnorderedAccessView> for ID3D11View {
    fn from(value: &ID3D11UnorderedAccessView) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::IntoParam<'a, ID3D11View> for ID3D11UnorderedAccessView {
    fn into_param(self) -> ::windows::Param<'a, ID3D11View> {
        ::windows::Param::Owned(::std::convert::Into::<ID3D11View>::into(self))
    }
}
impl<'a> ::windows::IntoParam<'a, ID3D11View> for &'a ID3D11UnorderedAccessView {
    fn into_param(self) -> ::windows::Param<'a, ID3D11View> {
        ::windows::Param::Owned(::std::convert::Into::<ID3D11View>::into(
            ::std::clone::Clone::clone(self),
        ))
    }
}
impl ::std::convert::From<ID3D11UnorderedAccessView> for ID3D11DeviceChild {
    fn from(value: ID3D11UnorderedAccessView) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&ID3D11UnorderedAccessView> for ID3D11DeviceChild {
    fn from(value: &ID3D11UnorderedAccessView) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::IntoParam<'a, ID3D11DeviceChild> for ID3D11UnorderedAccessView {
    fn into_param(self) -> ::windows::Param<'a, ID3D11DeviceChild> {
        ::windows::Param::Owned(::std::convert::Into::<ID3D11DeviceChild>::into(self))
    }
}
impl<'a> ::windows::IntoParam<'a, ID3D11DeviceChild> for &'a ID3D11UnorderedAccessView {
    fn into_param(self) -> ::windows::Param<'a, ID3D11DeviceChild> {
        ::windows::Param::Owned(::std::convert::Into::<ID3D11DeviceChild>::into(
            ::std::clone::Clone::clone(self),
        ))
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ID3D11UnorderedAccessView_abi(
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        iid: &::windows::Guid,
        interface: *mut ::windows::RawPtr,
    ) -> ::windows::ErrorCode,
    pub unsafe extern "system" fn(this: ::windows::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::RawPtr, pp_device: *mut ::windows::RawPtr),
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        guid: *const ::windows::Guid,
        p_data_size: *mut u32,
        p_data: *mut ::std::ffi::c_void,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        guid: *const ::windows::Guid,
        data_size: u32,
        p_data: *const ::std::ffi::c_void,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        guid: *const ::windows::Guid,
        p_data: ::windows::RawPtr,
    ) -> ::windows::ErrorCode,
    pub unsafe extern "system" fn(this: ::windows::RawPtr, pp_resource: *mut ::windows::RawPtr),
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        p_desc: *mut D3D11_UNORDERED_ACCESS_VIEW_DESC,
    ),
);
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
pub struct D3D11_RTV_DIMENSION(pub i32);
impl D3D11_RTV_DIMENSION {
    #![allow(non_upper_case_globals)]
    pub const D3D11_RTV_DIMENSION_UNKNOWN: Self = Self(0i32);
    pub const D3D11_RTV_DIMENSION_BUFFER: Self = Self(1i32);
    pub const D3D11_RTV_DIMENSION_TEXTURE1D: Self = Self(2i32);
    pub const D3D11_RTV_DIMENSION_TEXTURE1DARRAY: Self = Self(3i32);
    pub const D3D11_RTV_DIMENSION_TEXTURE2D: Self = Self(4i32);
    pub const D3D11_RTV_DIMENSION_TEXTURE2DARRAY: Self = Self(5i32);
    pub const D3D11_RTV_DIMENSION_TEXTURE2DMS: Self = Self(6i32);
    pub const D3D11_RTV_DIMENSION_TEXTURE2DMSARRAY: Self = Self(7i32);
    pub const D3D11_RTV_DIMENSION_TEXTURE3D: Self = Self(8i32);
}
impl ::std::convert::From<i32> for D3D11_RTV_DIMENSION {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::Abi for D3D11_RTV_DIMENSION {
    type Abi = Self;
}
#[repr(C)]
#[allow(non_snake_case)]
#[derive(:: std :: clone :: Clone)]
pub struct D3D11_RENDER_TARGET_VIEW_DESC {
    pub format: super::dxgi::DXGI_FORMAT,
    pub view_dimension: D3D11_RTV_DIMENSION,
    pub anonymous: ::windows::NOT_YET_SUPPORTED_TYPE,
}
impl D3D11_RENDER_TARGET_VIEW_DESC {}
impl ::std::default::Default for D3D11_RENDER_TARGET_VIEW_DESC {
    fn default() -> Self {
        Self {
            format: ::std::default::Default::default(),
            view_dimension: ::std::default::Default::default(),
            anonymous: ::std::default::Default::default(),
        }
    }
}
impl ::std::fmt::Debug for D3D11_RENDER_TARGET_VIEW_DESC {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("D3D11_RENDER_TARGET_VIEW_DESC")
            .field("format", &format_args!("{:?}", self.format))
            .field("view_dimension", &format_args!("{:?}", self.view_dimension))
            .field("anonymous", &format_args!("{:?}", self.anonymous))
            .finish()
    }
}
impl ::std::cmp::PartialEq for D3D11_RENDER_TARGET_VIEW_DESC {
    fn eq(&self, other: &Self) -> bool {
        self.format == other.format
            && self.view_dimension == other.view_dimension
            && self.anonymous == other.anonymous
    }
}
impl ::std::cmp::Eq for D3D11_RENDER_TARGET_VIEW_DESC {}
unsafe impl ::windows::Abi for D3D11_RENDER_TARGET_VIEW_DESC {
    type Abi = Self;
}
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct ID3D11RenderTargetView(::windows::IUnknown);
impl ID3D11RenderTargetView {}
unsafe impl ::windows::Interface for ID3D11RenderTargetView {
    type Vtable = ID3D11RenderTargetView_abi;
    const IID: ::windows::Guid = ::windows::Guid::from_values(
        3755712615,
        2957,
        18533,
        [135, 91, 215, 180, 81, 108, 193, 100],
    );
}
#[allow(non_snake_case)]
impl ID3D11RenderTargetView {
    pub unsafe fn GetDevice(&self, pp_device: *mut ::std::option::Option<ID3D11Device>) {
        (::windows::Interface::vtable(self).3)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(pp_device),
        )
    }
    pub unsafe fn GetPrivateData(
        &self,
        guid: *const ::windows::Guid,
        p_data_size: *mut u32,
        p_data: *mut ::std::ffi::c_void,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).4)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(guid),
            ::std::mem::transmute(p_data_size),
            ::std::mem::transmute(p_data),
        )
    }
    pub unsafe fn SetPrivateData(
        &self,
        guid: *const ::windows::Guid,
        data_size: u32,
        p_data: *const ::std::ffi::c_void,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).5)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(guid),
            ::std::mem::transmute(data_size),
            ::std::mem::transmute(p_data),
        )
    }
    pub unsafe fn SetPrivateDataInterface<
        'a,
        T1__: ::windows::IntoParam<'a, ::windows::IUnknown>,
    >(
        &self,
        guid: *const ::windows::Guid,
        p_data: T1__,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).6)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(guid),
            p_data.into_param().abi(),
        )
    }
    pub unsafe fn GetResource(&self, pp_resource: *mut ::std::option::Option<ID3D11Resource>) {
        (::windows::Interface::vtable(self).7)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(pp_resource),
        )
    }
    pub unsafe fn GetDesc(&self, p_desc: *mut D3D11_RENDER_TARGET_VIEW_DESC) {
        (::windows::Interface::vtable(self).8)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(p_desc),
        )
    }
}
impl ::std::convert::From<ID3D11RenderTargetView> for ::windows::IUnknown {
    fn from(value: ID3D11RenderTargetView) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&ID3D11RenderTargetView> for ::windows::IUnknown {
    fn from(value: &ID3D11RenderTargetView) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::IntoParam<'a, ::windows::IUnknown> for ID3D11RenderTargetView {
    fn into_param(self) -> ::windows::Param<'a, ::windows::IUnknown> {
        ::windows::Param::Owned(::std::convert::Into::<::windows::IUnknown>::into(self))
    }
}
impl<'a> ::windows::IntoParam<'a, ::windows::IUnknown> for &'a ID3D11RenderTargetView {
    fn into_param(self) -> ::windows::Param<'a, ::windows::IUnknown> {
        ::windows::Param::Owned(::std::convert::Into::<::windows::IUnknown>::into(
            ::std::clone::Clone::clone(self),
        ))
    }
}
impl ::std::convert::From<ID3D11RenderTargetView> for ID3D11View {
    fn from(value: ID3D11RenderTargetView) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&ID3D11RenderTargetView> for ID3D11View {
    fn from(value: &ID3D11RenderTargetView) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::IntoParam<'a, ID3D11View> for ID3D11RenderTargetView {
    fn into_param(self) -> ::windows::Param<'a, ID3D11View> {
        ::windows::Param::Owned(::std::convert::Into::<ID3D11View>::into(self))
    }
}
impl<'a> ::windows::IntoParam<'a, ID3D11View> for &'a ID3D11RenderTargetView {
    fn into_param(self) -> ::windows::Param<'a, ID3D11View> {
        ::windows::Param::Owned(::std::convert::Into::<ID3D11View>::into(
            ::std::clone::Clone::clone(self),
        ))
    }
}
impl ::std::convert::From<ID3D11RenderTargetView> for ID3D11DeviceChild {
    fn from(value: ID3D11RenderTargetView) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&ID3D11RenderTargetView> for ID3D11DeviceChild {
    fn from(value: &ID3D11RenderTargetView) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::IntoParam<'a, ID3D11DeviceChild> for ID3D11RenderTargetView {
    fn into_param(self) -> ::windows::Param<'a, ID3D11DeviceChild> {
        ::windows::Param::Owned(::std::convert::Into::<ID3D11DeviceChild>::into(self))
    }
}
impl<'a> ::windows::IntoParam<'a, ID3D11DeviceChild> for &'a ID3D11RenderTargetView {
    fn into_param(self) -> ::windows::Param<'a, ID3D11DeviceChild> {
        ::windows::Param::Owned(::std::convert::Into::<ID3D11DeviceChild>::into(
            ::std::clone::Clone::clone(self),
        ))
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ID3D11RenderTargetView_abi(
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        iid: &::windows::Guid,
        interface: *mut ::windows::RawPtr,
    ) -> ::windows::ErrorCode,
    pub unsafe extern "system" fn(this: ::windows::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::RawPtr, pp_device: *mut ::windows::RawPtr),
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        guid: *const ::windows::Guid,
        p_data_size: *mut u32,
        p_data: *mut ::std::ffi::c_void,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        guid: *const ::windows::Guid,
        data_size: u32,
        p_data: *const ::std::ffi::c_void,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        guid: *const ::windows::Guid,
        p_data: ::windows::RawPtr,
    ) -> ::windows::ErrorCode,
    pub unsafe extern "system" fn(this: ::windows::RawPtr, pp_resource: *mut ::windows::RawPtr),
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        p_desc: *mut D3D11_RENDER_TARGET_VIEW_DESC,
    ),
);
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
pub struct D3D11_DSV_DIMENSION(pub i32);
impl D3D11_DSV_DIMENSION {
    #![allow(non_upper_case_globals)]
    pub const D3D11_DSV_DIMENSION_UNKNOWN: Self = Self(0i32);
    pub const D3D11_DSV_DIMENSION_TEXTURE1D: Self = Self(1i32);
    pub const D3D11_DSV_DIMENSION_TEXTURE1DARRAY: Self = Self(2i32);
    pub const D3D11_DSV_DIMENSION_TEXTURE2D: Self = Self(3i32);
    pub const D3D11_DSV_DIMENSION_TEXTURE2DARRAY: Self = Self(4i32);
    pub const D3D11_DSV_DIMENSION_TEXTURE2DMS: Self = Self(5i32);
    pub const D3D11_DSV_DIMENSION_TEXTURE2DMSARRAY: Self = Self(6i32);
}
impl ::std::convert::From<i32> for D3D11_DSV_DIMENSION {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::Abi for D3D11_DSV_DIMENSION {
    type Abi = Self;
}
#[repr(C)]
#[allow(non_snake_case)]
#[derive(:: std :: clone :: Clone)]
pub struct D3D11_DEPTH_STENCIL_VIEW_DESC {
    pub format: super::dxgi::DXGI_FORMAT,
    pub view_dimension: D3D11_DSV_DIMENSION,
    pub flags: u32,
    pub anonymous: ::windows::NOT_YET_SUPPORTED_TYPE,
}
impl D3D11_DEPTH_STENCIL_VIEW_DESC {}
impl ::std::default::Default for D3D11_DEPTH_STENCIL_VIEW_DESC {
    fn default() -> Self {
        Self {
            format: ::std::default::Default::default(),
            view_dimension: ::std::default::Default::default(),
            flags: 0,
            anonymous: ::std::default::Default::default(),
        }
    }
}
impl ::std::fmt::Debug for D3D11_DEPTH_STENCIL_VIEW_DESC {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("D3D11_DEPTH_STENCIL_VIEW_DESC")
            .field("format", &format_args!("{:?}", self.format))
            .field("view_dimension", &format_args!("{:?}", self.view_dimension))
            .field("flags", &format_args!("{:?}", self.flags))
            .field("anonymous", &format_args!("{:?}", self.anonymous))
            .finish()
    }
}
impl ::std::cmp::PartialEq for D3D11_DEPTH_STENCIL_VIEW_DESC {
    fn eq(&self, other: &Self) -> bool {
        self.format == other.format
            && self.view_dimension == other.view_dimension
            && self.flags == other.flags
            && self.anonymous == other.anonymous
    }
}
impl ::std::cmp::Eq for D3D11_DEPTH_STENCIL_VIEW_DESC {}
unsafe impl ::windows::Abi for D3D11_DEPTH_STENCIL_VIEW_DESC {
    type Abi = Self;
}
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct ID3D11DepthStencilView(::windows::IUnknown);
impl ID3D11DepthStencilView {}
unsafe impl ::windows::Interface for ID3D11DepthStencilView {
    type Vtable = ID3D11DepthStencilView_abi;
    const IID: ::windows::Guid = ::windows::Guid::from_values(
        2681915690,
        6262,
        18627,
        [175, 173, 37, 185, 79, 132, 169, 182],
    );
}
#[allow(non_snake_case)]
impl ID3D11DepthStencilView {
    pub unsafe fn GetDevice(&self, pp_device: *mut ::std::option::Option<ID3D11Device>) {
        (::windows::Interface::vtable(self).3)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(pp_device),
        )
    }
    pub unsafe fn GetPrivateData(
        &self,
        guid: *const ::windows::Guid,
        p_data_size: *mut u32,
        p_data: *mut ::std::ffi::c_void,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).4)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(guid),
            ::std::mem::transmute(p_data_size),
            ::std::mem::transmute(p_data),
        )
    }
    pub unsafe fn SetPrivateData(
        &self,
        guid: *const ::windows::Guid,
        data_size: u32,
        p_data: *const ::std::ffi::c_void,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).5)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(guid),
            ::std::mem::transmute(data_size),
            ::std::mem::transmute(p_data),
        )
    }
    pub unsafe fn SetPrivateDataInterface<
        'a,
        T1__: ::windows::IntoParam<'a, ::windows::IUnknown>,
    >(
        &self,
        guid: *const ::windows::Guid,
        p_data: T1__,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).6)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(guid),
            p_data.into_param().abi(),
        )
    }
    pub unsafe fn GetResource(&self, pp_resource: *mut ::std::option::Option<ID3D11Resource>) {
        (::windows::Interface::vtable(self).7)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(pp_resource),
        )
    }
    pub unsafe fn GetDesc(&self, p_desc: *mut D3D11_DEPTH_STENCIL_VIEW_DESC) {
        (::windows::Interface::vtable(self).8)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(p_desc),
        )
    }
}
impl ::std::convert::From<ID3D11DepthStencilView> for ::windows::IUnknown {
    fn from(value: ID3D11DepthStencilView) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&ID3D11DepthStencilView> for ::windows::IUnknown {
    fn from(value: &ID3D11DepthStencilView) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::IntoParam<'a, ::windows::IUnknown> for ID3D11DepthStencilView {
    fn into_param(self) -> ::windows::Param<'a, ::windows::IUnknown> {
        ::windows::Param::Owned(::std::convert::Into::<::windows::IUnknown>::into(self))
    }
}
impl<'a> ::windows::IntoParam<'a, ::windows::IUnknown> for &'a ID3D11DepthStencilView {
    fn into_param(self) -> ::windows::Param<'a, ::windows::IUnknown> {
        ::windows::Param::Owned(::std::convert::Into::<::windows::IUnknown>::into(
            ::std::clone::Clone::clone(self),
        ))
    }
}
impl ::std::convert::From<ID3D11DepthStencilView> for ID3D11View {
    fn from(value: ID3D11DepthStencilView) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&ID3D11DepthStencilView> for ID3D11View {
    fn from(value: &ID3D11DepthStencilView) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::IntoParam<'a, ID3D11View> for ID3D11DepthStencilView {
    fn into_param(self) -> ::windows::Param<'a, ID3D11View> {
        ::windows::Param::Owned(::std::convert::Into::<ID3D11View>::into(self))
    }
}
impl<'a> ::windows::IntoParam<'a, ID3D11View> for &'a ID3D11DepthStencilView {
    fn into_param(self) -> ::windows::Param<'a, ID3D11View> {
        ::windows::Param::Owned(::std::convert::Into::<ID3D11View>::into(
            ::std::clone::Clone::clone(self),
        ))
    }
}
impl ::std::convert::From<ID3D11DepthStencilView> for ID3D11DeviceChild {
    fn from(value: ID3D11DepthStencilView) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&ID3D11DepthStencilView> for ID3D11DeviceChild {
    fn from(value: &ID3D11DepthStencilView) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::IntoParam<'a, ID3D11DeviceChild> for ID3D11DepthStencilView {
    fn into_param(self) -> ::windows::Param<'a, ID3D11DeviceChild> {
        ::windows::Param::Owned(::std::convert::Into::<ID3D11DeviceChild>::into(self))
    }
}
impl<'a> ::windows::IntoParam<'a, ID3D11DeviceChild> for &'a ID3D11DepthStencilView {
    fn into_param(self) -> ::windows::Param<'a, ID3D11DeviceChild> {
        ::windows::Param::Owned(::std::convert::Into::<ID3D11DeviceChild>::into(
            ::std::clone::Clone::clone(self),
        ))
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ID3D11DepthStencilView_abi(
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        iid: &::windows::Guid,
        interface: *mut ::windows::RawPtr,
    ) -> ::windows::ErrorCode,
    pub unsafe extern "system" fn(this: ::windows::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::RawPtr, pp_device: *mut ::windows::RawPtr),
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        guid: *const ::windows::Guid,
        p_data_size: *mut u32,
        p_data: *mut ::std::ffi::c_void,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        guid: *const ::windows::Guid,
        data_size: u32,
        p_data: *const ::std::ffi::c_void,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        guid: *const ::windows::Guid,
        p_data: ::windows::RawPtr,
    ) -> ::windows::ErrorCode,
    pub unsafe extern "system" fn(this: ::windows::RawPtr, pp_resource: *mut ::windows::RawPtr),
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        p_desc: *mut D3D11_DEPTH_STENCIL_VIEW_DESC,
    ),
);
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
pub struct D3D11_INPUT_CLASSIFICATION(pub i32);
impl D3D11_INPUT_CLASSIFICATION {
    #![allow(non_upper_case_globals)]
    pub const D3D11_INPUT_PER_VERTEX_DATA: Self = Self(0i32);
    pub const D3D11_INPUT_PER_INSTANCE_DATA: Self = Self(1i32);
}
impl ::std::convert::From<i32> for D3D11_INPUT_CLASSIFICATION {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::Abi for D3D11_INPUT_CLASSIFICATION {
    type Abi = Self;
}
#[repr(C)]
#[allow(non_snake_case)]
#[derive(:: std :: clone :: Clone)]
pub struct D3D11_INPUT_ELEMENT_DESC {
    pub semantic_name: super::system_services::PSTR,
    pub semantic_index: u32,
    pub format: super::dxgi::DXGI_FORMAT,
    pub input_slot: u32,
    pub aligned_byte_offset: u32,
    pub input_slot_class: D3D11_INPUT_CLASSIFICATION,
    pub instance_data_step_rate: u32,
}
impl D3D11_INPUT_ELEMENT_DESC {}
impl ::std::default::Default for D3D11_INPUT_ELEMENT_DESC {
    fn default() -> Self {
        Self {
            semantic_name: ::std::default::Default::default(),
            semantic_index: 0,
            format: ::std::default::Default::default(),
            input_slot: 0,
            aligned_byte_offset: 0,
            input_slot_class: ::std::default::Default::default(),
            instance_data_step_rate: 0,
        }
    }
}
impl ::std::fmt::Debug for D3D11_INPUT_ELEMENT_DESC {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("D3D11_INPUT_ELEMENT_DESC")
            .field("semantic_name", &format_args!("{:?}", self.semantic_name))
            .field("semantic_index", &format_args!("{:?}", self.semantic_index))
            .field("format", &format_args!("{:?}", self.format))
            .field("input_slot", &format_args!("{:?}", self.input_slot))
            .field(
                "aligned_byte_offset",
                &format_args!("{:?}", self.aligned_byte_offset),
            )
            .field(
                "input_slot_class",
                &format_args!("{:?}", self.input_slot_class),
            )
            .field(
                "instance_data_step_rate",
                &format_args!("{:?}", self.instance_data_step_rate),
            )
            .finish()
    }
}
impl ::std::cmp::PartialEq for D3D11_INPUT_ELEMENT_DESC {
    fn eq(&self, other: &Self) -> bool {
        self.semantic_name == other.semantic_name
            && self.semantic_index == other.semantic_index
            && self.format == other.format
            && self.input_slot == other.input_slot
            && self.aligned_byte_offset == other.aligned_byte_offset
            && self.input_slot_class == other.input_slot_class
            && self.instance_data_step_rate == other.instance_data_step_rate
    }
}
impl ::std::cmp::Eq for D3D11_INPUT_ELEMENT_DESC {}
unsafe impl ::windows::Abi for D3D11_INPUT_ELEMENT_DESC {
    type Abi = Self;
}
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct ID3D11InputLayout(::windows::IUnknown);
impl ID3D11InputLayout {}
unsafe impl ::windows::Interface for ID3D11InputLayout {
    type Vtable = ID3D11InputLayout_abi;
    const IID: ::windows::Guid =
        ::windows::Guid::from_values(3833699804, 19696, 16421, [189, 38, 93, 232, 42, 62, 7, 183]);
}
#[allow(non_snake_case)]
impl ID3D11InputLayout {
    pub unsafe fn GetDevice(&self, pp_device: *mut ::std::option::Option<ID3D11Device>) {
        (::windows::Interface::vtable(self).3)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(pp_device),
        )
    }
    pub unsafe fn GetPrivateData(
        &self,
        guid: *const ::windows::Guid,
        p_data_size: *mut u32,
        p_data: *mut ::std::ffi::c_void,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).4)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(guid),
            ::std::mem::transmute(p_data_size),
            ::std::mem::transmute(p_data),
        )
    }
    pub unsafe fn SetPrivateData(
        &self,
        guid: *const ::windows::Guid,
        data_size: u32,
        p_data: *const ::std::ffi::c_void,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).5)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(guid),
            ::std::mem::transmute(data_size),
            ::std::mem::transmute(p_data),
        )
    }
    pub unsafe fn SetPrivateDataInterface<
        'a,
        T1__: ::windows::IntoParam<'a, ::windows::IUnknown>,
    >(
        &self,
        guid: *const ::windows::Guid,
        p_data: T1__,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).6)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(guid),
            p_data.into_param().abi(),
        )
    }
}
impl ::std::convert::From<ID3D11InputLayout> for ::windows::IUnknown {
    fn from(value: ID3D11InputLayout) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&ID3D11InputLayout> for ::windows::IUnknown {
    fn from(value: &ID3D11InputLayout) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::IntoParam<'a, ::windows::IUnknown> for ID3D11InputLayout {
    fn into_param(self) -> ::windows::Param<'a, ::windows::IUnknown> {
        ::windows::Param::Owned(::std::convert::Into::<::windows::IUnknown>::into(self))
    }
}
impl<'a> ::windows::IntoParam<'a, ::windows::IUnknown> for &'a ID3D11InputLayout {
    fn into_param(self) -> ::windows::Param<'a, ::windows::IUnknown> {
        ::windows::Param::Owned(::std::convert::Into::<::windows::IUnknown>::into(
            ::std::clone::Clone::clone(self),
        ))
    }
}
impl ::std::convert::From<ID3D11InputLayout> for ID3D11DeviceChild {
    fn from(value: ID3D11InputLayout) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&ID3D11InputLayout> for ID3D11DeviceChild {
    fn from(value: &ID3D11InputLayout) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::IntoParam<'a, ID3D11DeviceChild> for ID3D11InputLayout {
    fn into_param(self) -> ::windows::Param<'a, ID3D11DeviceChild> {
        ::windows::Param::Owned(::std::convert::Into::<ID3D11DeviceChild>::into(self))
    }
}
impl<'a> ::windows::IntoParam<'a, ID3D11DeviceChild> for &'a ID3D11InputLayout {
    fn into_param(self) -> ::windows::Param<'a, ID3D11DeviceChild> {
        ::windows::Param::Owned(::std::convert::Into::<ID3D11DeviceChild>::into(
            ::std::clone::Clone::clone(self),
        ))
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ID3D11InputLayout_abi(
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        iid: &::windows::Guid,
        interface: *mut ::windows::RawPtr,
    ) -> ::windows::ErrorCode,
    pub unsafe extern "system" fn(this: ::windows::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::RawPtr, pp_device: *mut ::windows::RawPtr),
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        guid: *const ::windows::Guid,
        p_data_size: *mut u32,
        p_data: *mut ::std::ffi::c_void,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        guid: *const ::windows::Guid,
        data_size: u32,
        p_data: *const ::std::ffi::c_void,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        guid: *const ::windows::Guid,
        p_data: ::windows::RawPtr,
    ) -> ::windows::ErrorCode,
);
#[repr(C)]
#[allow(non_snake_case)]
#[derive(:: std :: clone :: Clone)]
pub struct D3D11_CLASS_INSTANCE_DESC {
    pub instance_id: u32,
    pub instance_index: u32,
    pub type_id: u32,
    pub constant_buffer: u32,
    pub base_constant_buffer_offset: u32,
    pub base_texture: u32,
    pub base_sampler: u32,
    pub created: super::system_services::BOOL,
}
impl D3D11_CLASS_INSTANCE_DESC {}
impl ::std::default::Default for D3D11_CLASS_INSTANCE_DESC {
    fn default() -> Self {
        Self {
            instance_id: 0,
            instance_index: 0,
            type_id: 0,
            constant_buffer: 0,
            base_constant_buffer_offset: 0,
            base_texture: 0,
            base_sampler: 0,
            created: ::std::default::Default::default(),
        }
    }
}
impl ::std::fmt::Debug for D3D11_CLASS_INSTANCE_DESC {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("D3D11_CLASS_INSTANCE_DESC")
            .field("instance_id", &format_args!("{:?}", self.instance_id))
            .field("instance_index", &format_args!("{:?}", self.instance_index))
            .field("type_id", &format_args!("{:?}", self.type_id))
            .field(
                "constant_buffer",
                &format_args!("{:?}", self.constant_buffer),
            )
            .field(
                "base_constant_buffer_offset",
                &format_args!("{:?}", self.base_constant_buffer_offset),
            )
            .field("base_texture", &format_args!("{:?}", self.base_texture))
            .field("base_sampler", &format_args!("{:?}", self.base_sampler))
            .field("created", &format_args!("{:?}", self.created))
            .finish()
    }
}
impl ::std::cmp::PartialEq for D3D11_CLASS_INSTANCE_DESC {
    fn eq(&self, other: &Self) -> bool {
        self.instance_id == other.instance_id
            && self.instance_index == other.instance_index
            && self.type_id == other.type_id
            && self.constant_buffer == other.constant_buffer
            && self.base_constant_buffer_offset == other.base_constant_buffer_offset
            && self.base_texture == other.base_texture
            && self.base_sampler == other.base_sampler
            && self.created == other.created
    }
}
impl ::std::cmp::Eq for D3D11_CLASS_INSTANCE_DESC {}
unsafe impl ::windows::Abi for D3D11_CLASS_INSTANCE_DESC {
    type Abi = Self;
}
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct ID3D11ClassInstance(::windows::IUnknown);
impl ID3D11ClassInstance {}
unsafe impl ::windows::Interface for ID3D11ClassInstance {
    type Vtable = ID3D11ClassInstance_abi;
    const IID: ::windows::Guid = ::windows::Guid::from_values(
        2798485418,
        45239,
        18991,
        [148, 54, 134, 98, 166, 87, 151, 203],
    );
}
#[allow(non_snake_case)]
impl ID3D11ClassInstance {
    pub unsafe fn GetDevice(&self, pp_device: *mut ::std::option::Option<ID3D11Device>) {
        (::windows::Interface::vtable(self).3)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(pp_device),
        )
    }
    pub unsafe fn GetPrivateData(
        &self,
        guid: *const ::windows::Guid,
        p_data_size: *mut u32,
        p_data: *mut ::std::ffi::c_void,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).4)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(guid),
            ::std::mem::transmute(p_data_size),
            ::std::mem::transmute(p_data),
        )
    }
    pub unsafe fn SetPrivateData(
        &self,
        guid: *const ::windows::Guid,
        data_size: u32,
        p_data: *const ::std::ffi::c_void,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).5)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(guid),
            ::std::mem::transmute(data_size),
            ::std::mem::transmute(p_data),
        )
    }
    pub unsafe fn SetPrivateDataInterface<
        'a,
        T1__: ::windows::IntoParam<'a, ::windows::IUnknown>,
    >(
        &self,
        guid: *const ::windows::Guid,
        p_data: T1__,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).6)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(guid),
            p_data.into_param().abi(),
        )
    }
    pub unsafe fn GetClassLinkage(
        &self,
        pp_linkage: *mut ::std::option::Option<ID3D11ClassLinkage>,
    ) {
        (::windows::Interface::vtable(self).7)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(pp_linkage),
        )
    }
    pub unsafe fn GetDesc(&self, p_desc: *mut D3D11_CLASS_INSTANCE_DESC) {
        (::windows::Interface::vtable(self).8)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(p_desc),
        )
    }
    pub unsafe fn GetInstanceName(
        &self,
        p_instance_name: super::system_services::PSTR,
        p_buffer_length: *mut usize,
    ) {
        (::windows::Interface::vtable(self).9)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(p_instance_name),
            ::std::mem::transmute(p_buffer_length),
        )
    }
    pub unsafe fn GetTypeName(
        &self,
        p_type_name: super::system_services::PSTR,
        p_buffer_length: *mut usize,
    ) {
        (::windows::Interface::vtable(self).10)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(p_type_name),
            ::std::mem::transmute(p_buffer_length),
        )
    }
}
impl ::std::convert::From<ID3D11ClassInstance> for ::windows::IUnknown {
    fn from(value: ID3D11ClassInstance) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&ID3D11ClassInstance> for ::windows::IUnknown {
    fn from(value: &ID3D11ClassInstance) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::IntoParam<'a, ::windows::IUnknown> for ID3D11ClassInstance {
    fn into_param(self) -> ::windows::Param<'a, ::windows::IUnknown> {
        ::windows::Param::Owned(::std::convert::Into::<::windows::IUnknown>::into(self))
    }
}
impl<'a> ::windows::IntoParam<'a, ::windows::IUnknown> for &'a ID3D11ClassInstance {
    fn into_param(self) -> ::windows::Param<'a, ::windows::IUnknown> {
        ::windows::Param::Owned(::std::convert::Into::<::windows::IUnknown>::into(
            ::std::clone::Clone::clone(self),
        ))
    }
}
impl ::std::convert::From<ID3D11ClassInstance> for ID3D11DeviceChild {
    fn from(value: ID3D11ClassInstance) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&ID3D11ClassInstance> for ID3D11DeviceChild {
    fn from(value: &ID3D11ClassInstance) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::IntoParam<'a, ID3D11DeviceChild> for ID3D11ClassInstance {
    fn into_param(self) -> ::windows::Param<'a, ID3D11DeviceChild> {
        ::windows::Param::Owned(::std::convert::Into::<ID3D11DeviceChild>::into(self))
    }
}
impl<'a> ::windows::IntoParam<'a, ID3D11DeviceChild> for &'a ID3D11ClassInstance {
    fn into_param(self) -> ::windows::Param<'a, ID3D11DeviceChild> {
        ::windows::Param::Owned(::std::convert::Into::<ID3D11DeviceChild>::into(
            ::std::clone::Clone::clone(self),
        ))
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ID3D11ClassInstance_abi(
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        iid: &::windows::Guid,
        interface: *mut ::windows::RawPtr,
    ) -> ::windows::ErrorCode,
    pub unsafe extern "system" fn(this: ::windows::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::RawPtr, pp_device: *mut ::windows::RawPtr),
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        guid: *const ::windows::Guid,
        p_data_size: *mut u32,
        p_data: *mut ::std::ffi::c_void,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        guid: *const ::windows::Guid,
        data_size: u32,
        p_data: *const ::std::ffi::c_void,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        guid: *const ::windows::Guid,
        p_data: ::windows::RawPtr,
    ) -> ::windows::ErrorCode,
    pub unsafe extern "system" fn(this: ::windows::RawPtr, pp_linkage: *mut ::windows::RawPtr),
    pub unsafe extern "system" fn(this: ::windows::RawPtr, p_desc: *mut D3D11_CLASS_INSTANCE_DESC),
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        p_instance_name: super::system_services::PSTR,
        p_buffer_length: *mut usize,
    ),
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        p_type_name: super::system_services::PSTR,
        p_buffer_length: *mut usize,
    ),
);
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct ID3D11ClassLinkage(::windows::IUnknown);
impl ID3D11ClassLinkage {}
unsafe impl ::windows::Interface for ID3D11ClassLinkage {
    type Vtable = ID3D11ClassLinkage_abi;
    const IID: ::windows::Guid = ::windows::Guid::from_values(
        3723852986,
        38211,
        18148,
        [161, 43, 242, 7, 160, 254, 127, 237],
    );
}
#[allow(non_snake_case)]
impl ID3D11ClassLinkage {
    pub unsafe fn GetDevice(&self, pp_device: *mut ::std::option::Option<ID3D11Device>) {
        (::windows::Interface::vtable(self).3)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(pp_device),
        )
    }
    pub unsafe fn GetPrivateData(
        &self,
        guid: *const ::windows::Guid,
        p_data_size: *mut u32,
        p_data: *mut ::std::ffi::c_void,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).4)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(guid),
            ::std::mem::transmute(p_data_size),
            ::std::mem::transmute(p_data),
        )
    }
    pub unsafe fn SetPrivateData(
        &self,
        guid: *const ::windows::Guid,
        data_size: u32,
        p_data: *const ::std::ffi::c_void,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).5)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(guid),
            ::std::mem::transmute(data_size),
            ::std::mem::transmute(p_data),
        )
    }
    pub unsafe fn SetPrivateDataInterface<
        'a,
        T1__: ::windows::IntoParam<'a, ::windows::IUnknown>,
    >(
        &self,
        guid: *const ::windows::Guid,
        p_data: T1__,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).6)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(guid),
            p_data.into_param().abi(),
        )
    }
    pub unsafe fn GetClassInstance<
        'a,
        T0__: ::windows::IntoParam<'a, super::system_services::PSTR>,
    >(
        &self,
        p_class_instance_name: T0__,
        instance_index: u32,
        pp_instance: *mut ::std::option::Option<ID3D11ClassInstance>,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).7)(
            ::windows::Abi::abi(self),
            p_class_instance_name.into_param().abi(),
            ::std::mem::transmute(instance_index),
            ::std::mem::transmute(pp_instance),
        )
    }
    pub unsafe fn CreateClassInstance<
        'a,
        T0__: ::windows::IntoParam<'a, super::system_services::PSTR>,
    >(
        &self,
        p_class_type_name: T0__,
        constant_buffer_offset: u32,
        constant_vector_offset: u32,
        texture_offset: u32,
        sampler_offset: u32,
        pp_instance: *mut ::std::option::Option<ID3D11ClassInstance>,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).8)(
            ::windows::Abi::abi(self),
            p_class_type_name.into_param().abi(),
            ::std::mem::transmute(constant_buffer_offset),
            ::std::mem::transmute(constant_vector_offset),
            ::std::mem::transmute(texture_offset),
            ::std::mem::transmute(sampler_offset),
            ::std::mem::transmute(pp_instance),
        )
    }
}
impl ::std::convert::From<ID3D11ClassLinkage> for ::windows::IUnknown {
    fn from(value: ID3D11ClassLinkage) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&ID3D11ClassLinkage> for ::windows::IUnknown {
    fn from(value: &ID3D11ClassLinkage) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::IntoParam<'a, ::windows::IUnknown> for ID3D11ClassLinkage {
    fn into_param(self) -> ::windows::Param<'a, ::windows::IUnknown> {
        ::windows::Param::Owned(::std::convert::Into::<::windows::IUnknown>::into(self))
    }
}
impl<'a> ::windows::IntoParam<'a, ::windows::IUnknown> for &'a ID3D11ClassLinkage {
    fn into_param(self) -> ::windows::Param<'a, ::windows::IUnknown> {
        ::windows::Param::Owned(::std::convert::Into::<::windows::IUnknown>::into(
            ::std::clone::Clone::clone(self),
        ))
    }
}
impl ::std::convert::From<ID3D11ClassLinkage> for ID3D11DeviceChild {
    fn from(value: ID3D11ClassLinkage) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&ID3D11ClassLinkage> for ID3D11DeviceChild {
    fn from(value: &ID3D11ClassLinkage) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::IntoParam<'a, ID3D11DeviceChild> for ID3D11ClassLinkage {
    fn into_param(self) -> ::windows::Param<'a, ID3D11DeviceChild> {
        ::windows::Param::Owned(::std::convert::Into::<ID3D11DeviceChild>::into(self))
    }
}
impl<'a> ::windows::IntoParam<'a, ID3D11DeviceChild> for &'a ID3D11ClassLinkage {
    fn into_param(self) -> ::windows::Param<'a, ID3D11DeviceChild> {
        ::windows::Param::Owned(::std::convert::Into::<ID3D11DeviceChild>::into(
            ::std::clone::Clone::clone(self),
        ))
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ID3D11ClassLinkage_abi(
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        iid: &::windows::Guid,
        interface: *mut ::windows::RawPtr,
    ) -> ::windows::ErrorCode,
    pub unsafe extern "system" fn(this: ::windows::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::RawPtr, pp_device: *mut ::windows::RawPtr),
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        guid: *const ::windows::Guid,
        p_data_size: *mut u32,
        p_data: *mut ::std::ffi::c_void,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        guid: *const ::windows::Guid,
        data_size: u32,
        p_data: *const ::std::ffi::c_void,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        guid: *const ::windows::Guid,
        p_data: ::windows::RawPtr,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        p_class_instance_name: super::system_services::PSTR,
        instance_index: u32,
        pp_instance: *mut ::windows::RawPtr,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        p_class_type_name: super::system_services::PSTR,
        constant_buffer_offset: u32,
        constant_vector_offset: u32,
        texture_offset: u32,
        sampler_offset: u32,
        pp_instance: *mut ::windows::RawPtr,
    ) -> ::windows::ErrorCode,
);
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct ID3D11VertexShader(::windows::IUnknown);
impl ID3D11VertexShader {}
unsafe impl ::windows::Interface for ID3D11VertexShader {
    type Vtable = ID3D11VertexShader_abi;
    const IID: ::windows::Guid = ::windows::Guid::from_values(
        993008996,
        54904,
        17033,
        [136, 151, 34, 248, 146, 139, 114, 243],
    );
}
#[allow(non_snake_case)]
impl ID3D11VertexShader {
    pub unsafe fn GetDevice(&self, pp_device: *mut ::std::option::Option<ID3D11Device>) {
        (::windows::Interface::vtable(self).3)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(pp_device),
        )
    }
    pub unsafe fn GetPrivateData(
        &self,
        guid: *const ::windows::Guid,
        p_data_size: *mut u32,
        p_data: *mut ::std::ffi::c_void,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).4)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(guid),
            ::std::mem::transmute(p_data_size),
            ::std::mem::transmute(p_data),
        )
    }
    pub unsafe fn SetPrivateData(
        &self,
        guid: *const ::windows::Guid,
        data_size: u32,
        p_data: *const ::std::ffi::c_void,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).5)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(guid),
            ::std::mem::transmute(data_size),
            ::std::mem::transmute(p_data),
        )
    }
    pub unsafe fn SetPrivateDataInterface<
        'a,
        T1__: ::windows::IntoParam<'a, ::windows::IUnknown>,
    >(
        &self,
        guid: *const ::windows::Guid,
        p_data: T1__,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).6)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(guid),
            p_data.into_param().abi(),
        )
    }
}
impl ::std::convert::From<ID3D11VertexShader> for ::windows::IUnknown {
    fn from(value: ID3D11VertexShader) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&ID3D11VertexShader> for ::windows::IUnknown {
    fn from(value: &ID3D11VertexShader) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::IntoParam<'a, ::windows::IUnknown> for ID3D11VertexShader {
    fn into_param(self) -> ::windows::Param<'a, ::windows::IUnknown> {
        ::windows::Param::Owned(::std::convert::Into::<::windows::IUnknown>::into(self))
    }
}
impl<'a> ::windows::IntoParam<'a, ::windows::IUnknown> for &'a ID3D11VertexShader {
    fn into_param(self) -> ::windows::Param<'a, ::windows::IUnknown> {
        ::windows::Param::Owned(::std::convert::Into::<::windows::IUnknown>::into(
            ::std::clone::Clone::clone(self),
        ))
    }
}
impl ::std::convert::From<ID3D11VertexShader> for ID3D11DeviceChild {
    fn from(value: ID3D11VertexShader) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&ID3D11VertexShader> for ID3D11DeviceChild {
    fn from(value: &ID3D11VertexShader) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::IntoParam<'a, ID3D11DeviceChild> for ID3D11VertexShader {
    fn into_param(self) -> ::windows::Param<'a, ID3D11DeviceChild> {
        ::windows::Param::Owned(::std::convert::Into::<ID3D11DeviceChild>::into(self))
    }
}
impl<'a> ::windows::IntoParam<'a, ID3D11DeviceChild> for &'a ID3D11VertexShader {
    fn into_param(self) -> ::windows::Param<'a, ID3D11DeviceChild> {
        ::windows::Param::Owned(::std::convert::Into::<ID3D11DeviceChild>::into(
            ::std::clone::Clone::clone(self),
        ))
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ID3D11VertexShader_abi(
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        iid: &::windows::Guid,
        interface: *mut ::windows::RawPtr,
    ) -> ::windows::ErrorCode,
    pub unsafe extern "system" fn(this: ::windows::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::RawPtr, pp_device: *mut ::windows::RawPtr),
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        guid: *const ::windows::Guid,
        p_data_size: *mut u32,
        p_data: *mut ::std::ffi::c_void,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        guid: *const ::windows::Guid,
        data_size: u32,
        p_data: *const ::std::ffi::c_void,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        guid: *const ::windows::Guid,
        p_data: ::windows::RawPtr,
    ) -> ::windows::ErrorCode,
);
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct ID3D11GeometryShader(::windows::IUnknown);
impl ID3D11GeometryShader {}
unsafe impl ::windows::Interface for ID3D11GeometryShader {
    type Vtable = ID3D11GeometryShader_abi;
    const IID: ::windows::Guid =
        ::windows::Guid::from_values(942824342, 61435, 16418, [186, 2, 46, 121, 91, 112, 39, 92]);
}
#[allow(non_snake_case)]
impl ID3D11GeometryShader {
    pub unsafe fn GetDevice(&self, pp_device: *mut ::std::option::Option<ID3D11Device>) {
        (::windows::Interface::vtable(self).3)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(pp_device),
        )
    }
    pub unsafe fn GetPrivateData(
        &self,
        guid: *const ::windows::Guid,
        p_data_size: *mut u32,
        p_data: *mut ::std::ffi::c_void,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).4)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(guid),
            ::std::mem::transmute(p_data_size),
            ::std::mem::transmute(p_data),
        )
    }
    pub unsafe fn SetPrivateData(
        &self,
        guid: *const ::windows::Guid,
        data_size: u32,
        p_data: *const ::std::ffi::c_void,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).5)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(guid),
            ::std::mem::transmute(data_size),
            ::std::mem::transmute(p_data),
        )
    }
    pub unsafe fn SetPrivateDataInterface<
        'a,
        T1__: ::windows::IntoParam<'a, ::windows::IUnknown>,
    >(
        &self,
        guid: *const ::windows::Guid,
        p_data: T1__,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).6)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(guid),
            p_data.into_param().abi(),
        )
    }
}
impl ::std::convert::From<ID3D11GeometryShader> for ::windows::IUnknown {
    fn from(value: ID3D11GeometryShader) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&ID3D11GeometryShader> for ::windows::IUnknown {
    fn from(value: &ID3D11GeometryShader) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::IntoParam<'a, ::windows::IUnknown> for ID3D11GeometryShader {
    fn into_param(self) -> ::windows::Param<'a, ::windows::IUnknown> {
        ::windows::Param::Owned(::std::convert::Into::<::windows::IUnknown>::into(self))
    }
}
impl<'a> ::windows::IntoParam<'a, ::windows::IUnknown> for &'a ID3D11GeometryShader {
    fn into_param(self) -> ::windows::Param<'a, ::windows::IUnknown> {
        ::windows::Param::Owned(::std::convert::Into::<::windows::IUnknown>::into(
            ::std::clone::Clone::clone(self),
        ))
    }
}
impl ::std::convert::From<ID3D11GeometryShader> for ID3D11DeviceChild {
    fn from(value: ID3D11GeometryShader) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&ID3D11GeometryShader> for ID3D11DeviceChild {
    fn from(value: &ID3D11GeometryShader) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::IntoParam<'a, ID3D11DeviceChild> for ID3D11GeometryShader {
    fn into_param(self) -> ::windows::Param<'a, ID3D11DeviceChild> {
        ::windows::Param::Owned(::std::convert::Into::<ID3D11DeviceChild>::into(self))
    }
}
impl<'a> ::windows::IntoParam<'a, ID3D11DeviceChild> for &'a ID3D11GeometryShader {
    fn into_param(self) -> ::windows::Param<'a, ID3D11DeviceChild> {
        ::windows::Param::Owned(::std::convert::Into::<ID3D11DeviceChild>::into(
            ::std::clone::Clone::clone(self),
        ))
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ID3D11GeometryShader_abi(
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        iid: &::windows::Guid,
        interface: *mut ::windows::RawPtr,
    ) -> ::windows::ErrorCode,
    pub unsafe extern "system" fn(this: ::windows::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::RawPtr, pp_device: *mut ::windows::RawPtr),
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        guid: *const ::windows::Guid,
        p_data_size: *mut u32,
        p_data: *mut ::std::ffi::c_void,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        guid: *const ::windows::Guid,
        data_size: u32,
        p_data: *const ::std::ffi::c_void,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        guid: *const ::windows::Guid,
        p_data: ::windows::RawPtr,
    ) -> ::windows::ErrorCode,
);
#[repr(C)]
#[allow(non_snake_case)]
#[derive(:: std :: clone :: Clone)]
pub struct D3D11_SO_DECLARATION_ENTRY {
    pub stream: u32,
    pub semantic_name: super::system_services::PSTR,
    pub semantic_index: u32,
    pub start_component: u8,
    pub component_count: u8,
    pub output_slot: u8,
}
impl D3D11_SO_DECLARATION_ENTRY {}
impl ::std::default::Default for D3D11_SO_DECLARATION_ENTRY {
    fn default() -> Self {
        Self {
            stream: 0,
            semantic_name: ::std::default::Default::default(),
            semantic_index: 0,
            start_component: 0,
            component_count: 0,
            output_slot: 0,
        }
    }
}
impl ::std::fmt::Debug for D3D11_SO_DECLARATION_ENTRY {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("D3D11_SO_DECLARATION_ENTRY")
            .field("stream", &format_args!("{:?}", self.stream))
            .field("semantic_name", &format_args!("{:?}", self.semantic_name))
            .field("semantic_index", &format_args!("{:?}", self.semantic_index))
            .field(
                "start_component",
                &format_args!("{:?}", self.start_component),
            )
            .field(
                "component_count",
                &format_args!("{:?}", self.component_count),
            )
            .field("output_slot", &format_args!("{:?}", self.output_slot))
            .finish()
    }
}
impl ::std::cmp::PartialEq for D3D11_SO_DECLARATION_ENTRY {
    fn eq(&self, other: &Self) -> bool {
        self.stream == other.stream
            && self.semantic_name == other.semantic_name
            && self.semantic_index == other.semantic_index
            && self.start_component == other.start_component
            && self.component_count == other.component_count
            && self.output_slot == other.output_slot
    }
}
impl ::std::cmp::Eq for D3D11_SO_DECLARATION_ENTRY {}
unsafe impl ::windows::Abi for D3D11_SO_DECLARATION_ENTRY {
    type Abi = Self;
}
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct ID3D11PixelShader(::windows::IUnknown);
impl ID3D11PixelShader {}
unsafe impl ::windows::Interface for ID3D11PixelShader {
    type Vtable = ID3D11PixelShader_abi;
    const IID: ::windows::Guid = ::windows::Guid::from_values(
        3934446605,
        20956,
        20275,
        [147, 212, 219, 124, 145, 37, 174, 140],
    );
}
#[allow(non_snake_case)]
impl ID3D11PixelShader {
    pub unsafe fn GetDevice(&self, pp_device: *mut ::std::option::Option<ID3D11Device>) {
        (::windows::Interface::vtable(self).3)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(pp_device),
        )
    }
    pub unsafe fn GetPrivateData(
        &self,
        guid: *const ::windows::Guid,
        p_data_size: *mut u32,
        p_data: *mut ::std::ffi::c_void,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).4)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(guid),
            ::std::mem::transmute(p_data_size),
            ::std::mem::transmute(p_data),
        )
    }
    pub unsafe fn SetPrivateData(
        &self,
        guid: *const ::windows::Guid,
        data_size: u32,
        p_data: *const ::std::ffi::c_void,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).5)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(guid),
            ::std::mem::transmute(data_size),
            ::std::mem::transmute(p_data),
        )
    }
    pub unsafe fn SetPrivateDataInterface<
        'a,
        T1__: ::windows::IntoParam<'a, ::windows::IUnknown>,
    >(
        &self,
        guid: *const ::windows::Guid,
        p_data: T1__,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).6)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(guid),
            p_data.into_param().abi(),
        )
    }
}
impl ::std::convert::From<ID3D11PixelShader> for ::windows::IUnknown {
    fn from(value: ID3D11PixelShader) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&ID3D11PixelShader> for ::windows::IUnknown {
    fn from(value: &ID3D11PixelShader) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::IntoParam<'a, ::windows::IUnknown> for ID3D11PixelShader {
    fn into_param(self) -> ::windows::Param<'a, ::windows::IUnknown> {
        ::windows::Param::Owned(::std::convert::Into::<::windows::IUnknown>::into(self))
    }
}
impl<'a> ::windows::IntoParam<'a, ::windows::IUnknown> for &'a ID3D11PixelShader {
    fn into_param(self) -> ::windows::Param<'a, ::windows::IUnknown> {
        ::windows::Param::Owned(::std::convert::Into::<::windows::IUnknown>::into(
            ::std::clone::Clone::clone(self),
        ))
    }
}
impl ::std::convert::From<ID3D11PixelShader> for ID3D11DeviceChild {
    fn from(value: ID3D11PixelShader) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&ID3D11PixelShader> for ID3D11DeviceChild {
    fn from(value: &ID3D11PixelShader) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::IntoParam<'a, ID3D11DeviceChild> for ID3D11PixelShader {
    fn into_param(self) -> ::windows::Param<'a, ID3D11DeviceChild> {
        ::windows::Param::Owned(::std::convert::Into::<ID3D11DeviceChild>::into(self))
    }
}
impl<'a> ::windows::IntoParam<'a, ID3D11DeviceChild> for &'a ID3D11PixelShader {
    fn into_param(self) -> ::windows::Param<'a, ID3D11DeviceChild> {
        ::windows::Param::Owned(::std::convert::Into::<ID3D11DeviceChild>::into(
            ::std::clone::Clone::clone(self),
        ))
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ID3D11PixelShader_abi(
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        iid: &::windows::Guid,
        interface: *mut ::windows::RawPtr,
    ) -> ::windows::ErrorCode,
    pub unsafe extern "system" fn(this: ::windows::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::RawPtr, pp_device: *mut ::windows::RawPtr),
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        guid: *const ::windows::Guid,
        p_data_size: *mut u32,
        p_data: *mut ::std::ffi::c_void,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        guid: *const ::windows::Guid,
        data_size: u32,
        p_data: *const ::std::ffi::c_void,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        guid: *const ::windows::Guid,
        p_data: ::windows::RawPtr,
    ) -> ::windows::ErrorCode,
);
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct ID3D11HullShader(::windows::IUnknown);
impl ID3D11HullShader {}
unsafe impl ::windows::Interface for ID3D11HullShader {
    type Vtable = ID3D11HullShader_abi;
    const IID: ::windows::Guid = ::windows::Guid::from_values(
        2388418657,
        25226,
        19598,
        [130, 100, 187, 228, 92, 179, 213, 221],
    );
}
#[allow(non_snake_case)]
impl ID3D11HullShader {
    pub unsafe fn GetDevice(&self, pp_device: *mut ::std::option::Option<ID3D11Device>) {
        (::windows::Interface::vtable(self).3)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(pp_device),
        )
    }
    pub unsafe fn GetPrivateData(
        &self,
        guid: *const ::windows::Guid,
        p_data_size: *mut u32,
        p_data: *mut ::std::ffi::c_void,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).4)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(guid),
            ::std::mem::transmute(p_data_size),
            ::std::mem::transmute(p_data),
        )
    }
    pub unsafe fn SetPrivateData(
        &self,
        guid: *const ::windows::Guid,
        data_size: u32,
        p_data: *const ::std::ffi::c_void,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).5)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(guid),
            ::std::mem::transmute(data_size),
            ::std::mem::transmute(p_data),
        )
    }
    pub unsafe fn SetPrivateDataInterface<
        'a,
        T1__: ::windows::IntoParam<'a, ::windows::IUnknown>,
    >(
        &self,
        guid: *const ::windows::Guid,
        p_data: T1__,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).6)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(guid),
            p_data.into_param().abi(),
        )
    }
}
impl ::std::convert::From<ID3D11HullShader> for ::windows::IUnknown {
    fn from(value: ID3D11HullShader) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&ID3D11HullShader> for ::windows::IUnknown {
    fn from(value: &ID3D11HullShader) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::IntoParam<'a, ::windows::IUnknown> for ID3D11HullShader {
    fn into_param(self) -> ::windows::Param<'a, ::windows::IUnknown> {
        ::windows::Param::Owned(::std::convert::Into::<::windows::IUnknown>::into(self))
    }
}
impl<'a> ::windows::IntoParam<'a, ::windows::IUnknown> for &'a ID3D11HullShader {
    fn into_param(self) -> ::windows::Param<'a, ::windows::IUnknown> {
        ::windows::Param::Owned(::std::convert::Into::<::windows::IUnknown>::into(
            ::std::clone::Clone::clone(self),
        ))
    }
}
impl ::std::convert::From<ID3D11HullShader> for ID3D11DeviceChild {
    fn from(value: ID3D11HullShader) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&ID3D11HullShader> for ID3D11DeviceChild {
    fn from(value: &ID3D11HullShader) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::IntoParam<'a, ID3D11DeviceChild> for ID3D11HullShader {
    fn into_param(self) -> ::windows::Param<'a, ID3D11DeviceChild> {
        ::windows::Param::Owned(::std::convert::Into::<ID3D11DeviceChild>::into(self))
    }
}
impl<'a> ::windows::IntoParam<'a, ID3D11DeviceChild> for &'a ID3D11HullShader {
    fn into_param(self) -> ::windows::Param<'a, ID3D11DeviceChild> {
        ::windows::Param::Owned(::std::convert::Into::<ID3D11DeviceChild>::into(
            ::std::clone::Clone::clone(self),
        ))
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ID3D11HullShader_abi(
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        iid: &::windows::Guid,
        interface: *mut ::windows::RawPtr,
    ) -> ::windows::ErrorCode,
    pub unsafe extern "system" fn(this: ::windows::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::RawPtr, pp_device: *mut ::windows::RawPtr),
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        guid: *const ::windows::Guid,
        p_data_size: *mut u32,
        p_data: *mut ::std::ffi::c_void,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        guid: *const ::windows::Guid,
        data_size: u32,
        p_data: *const ::std::ffi::c_void,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        guid: *const ::windows::Guid,
        p_data: ::windows::RawPtr,
    ) -> ::windows::ErrorCode,
);
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct ID3D11DomainShader(::windows::IUnknown);
impl ID3D11DomainShader {}
unsafe impl ::windows::Interface for ID3D11DomainShader {
    type Vtable = ID3D11DomainShader_abi;
    const IID: ::windows::Guid = ::windows::Guid::from_values(
        4118988040,
        3894,
        18700,
        [153, 119, 49, 238, 206, 38, 140, 250],
    );
}
#[allow(non_snake_case)]
impl ID3D11DomainShader {
    pub unsafe fn GetDevice(&self, pp_device: *mut ::std::option::Option<ID3D11Device>) {
        (::windows::Interface::vtable(self).3)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(pp_device),
        )
    }
    pub unsafe fn GetPrivateData(
        &self,
        guid: *const ::windows::Guid,
        p_data_size: *mut u32,
        p_data: *mut ::std::ffi::c_void,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).4)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(guid),
            ::std::mem::transmute(p_data_size),
            ::std::mem::transmute(p_data),
        )
    }
    pub unsafe fn SetPrivateData(
        &self,
        guid: *const ::windows::Guid,
        data_size: u32,
        p_data: *const ::std::ffi::c_void,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).5)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(guid),
            ::std::mem::transmute(data_size),
            ::std::mem::transmute(p_data),
        )
    }
    pub unsafe fn SetPrivateDataInterface<
        'a,
        T1__: ::windows::IntoParam<'a, ::windows::IUnknown>,
    >(
        &self,
        guid: *const ::windows::Guid,
        p_data: T1__,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).6)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(guid),
            p_data.into_param().abi(),
        )
    }
}
impl ::std::convert::From<ID3D11DomainShader> for ::windows::IUnknown {
    fn from(value: ID3D11DomainShader) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&ID3D11DomainShader> for ::windows::IUnknown {
    fn from(value: &ID3D11DomainShader) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::IntoParam<'a, ::windows::IUnknown> for ID3D11DomainShader {
    fn into_param(self) -> ::windows::Param<'a, ::windows::IUnknown> {
        ::windows::Param::Owned(::std::convert::Into::<::windows::IUnknown>::into(self))
    }
}
impl<'a> ::windows::IntoParam<'a, ::windows::IUnknown> for &'a ID3D11DomainShader {
    fn into_param(self) -> ::windows::Param<'a, ::windows::IUnknown> {
        ::windows::Param::Owned(::std::convert::Into::<::windows::IUnknown>::into(
            ::std::clone::Clone::clone(self),
        ))
    }
}
impl ::std::convert::From<ID3D11DomainShader> for ID3D11DeviceChild {
    fn from(value: ID3D11DomainShader) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&ID3D11DomainShader> for ID3D11DeviceChild {
    fn from(value: &ID3D11DomainShader) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::IntoParam<'a, ID3D11DeviceChild> for ID3D11DomainShader {
    fn into_param(self) -> ::windows::Param<'a, ID3D11DeviceChild> {
        ::windows::Param::Owned(::std::convert::Into::<ID3D11DeviceChild>::into(self))
    }
}
impl<'a> ::windows::IntoParam<'a, ID3D11DeviceChild> for &'a ID3D11DomainShader {
    fn into_param(self) -> ::windows::Param<'a, ID3D11DeviceChild> {
        ::windows::Param::Owned(::std::convert::Into::<ID3D11DeviceChild>::into(
            ::std::clone::Clone::clone(self),
        ))
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ID3D11DomainShader_abi(
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        iid: &::windows::Guid,
        interface: *mut ::windows::RawPtr,
    ) -> ::windows::ErrorCode,
    pub unsafe extern "system" fn(this: ::windows::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::RawPtr, pp_device: *mut ::windows::RawPtr),
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        guid: *const ::windows::Guid,
        p_data_size: *mut u32,
        p_data: *mut ::std::ffi::c_void,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        guid: *const ::windows::Guid,
        data_size: u32,
        p_data: *const ::std::ffi::c_void,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        guid: *const ::windows::Guid,
        p_data: ::windows::RawPtr,
    ) -> ::windows::ErrorCode,
);
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct ID3D11ComputeShader(::windows::IUnknown);
impl ID3D11ComputeShader {}
unsafe impl ::windows::Interface for ID3D11ComputeShader {
    type Vtable = ID3D11ComputeShader_abi;
    const IID: ::windows::Guid = ::windows::Guid::from_values(
        1331370350,
        49853,
        18782,
        [189, 1, 31, 222, 211, 142, 73, 105],
    );
}
#[allow(non_snake_case)]
impl ID3D11ComputeShader {
    pub unsafe fn GetDevice(&self, pp_device: *mut ::std::option::Option<ID3D11Device>) {
        (::windows::Interface::vtable(self).3)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(pp_device),
        )
    }
    pub unsafe fn GetPrivateData(
        &self,
        guid: *const ::windows::Guid,
        p_data_size: *mut u32,
        p_data: *mut ::std::ffi::c_void,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).4)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(guid),
            ::std::mem::transmute(p_data_size),
            ::std::mem::transmute(p_data),
        )
    }
    pub unsafe fn SetPrivateData(
        &self,
        guid: *const ::windows::Guid,
        data_size: u32,
        p_data: *const ::std::ffi::c_void,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).5)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(guid),
            ::std::mem::transmute(data_size),
            ::std::mem::transmute(p_data),
        )
    }
    pub unsafe fn SetPrivateDataInterface<
        'a,
        T1__: ::windows::IntoParam<'a, ::windows::IUnknown>,
    >(
        &self,
        guid: *const ::windows::Guid,
        p_data: T1__,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).6)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(guid),
            p_data.into_param().abi(),
        )
    }
}
impl ::std::convert::From<ID3D11ComputeShader> for ::windows::IUnknown {
    fn from(value: ID3D11ComputeShader) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&ID3D11ComputeShader> for ::windows::IUnknown {
    fn from(value: &ID3D11ComputeShader) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::IntoParam<'a, ::windows::IUnknown> for ID3D11ComputeShader {
    fn into_param(self) -> ::windows::Param<'a, ::windows::IUnknown> {
        ::windows::Param::Owned(::std::convert::Into::<::windows::IUnknown>::into(self))
    }
}
impl<'a> ::windows::IntoParam<'a, ::windows::IUnknown> for &'a ID3D11ComputeShader {
    fn into_param(self) -> ::windows::Param<'a, ::windows::IUnknown> {
        ::windows::Param::Owned(::std::convert::Into::<::windows::IUnknown>::into(
            ::std::clone::Clone::clone(self),
        ))
    }
}
impl ::std::convert::From<ID3D11ComputeShader> for ID3D11DeviceChild {
    fn from(value: ID3D11ComputeShader) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&ID3D11ComputeShader> for ID3D11DeviceChild {
    fn from(value: &ID3D11ComputeShader) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::IntoParam<'a, ID3D11DeviceChild> for ID3D11ComputeShader {
    fn into_param(self) -> ::windows::Param<'a, ID3D11DeviceChild> {
        ::windows::Param::Owned(::std::convert::Into::<ID3D11DeviceChild>::into(self))
    }
}
impl<'a> ::windows::IntoParam<'a, ID3D11DeviceChild> for &'a ID3D11ComputeShader {
    fn into_param(self) -> ::windows::Param<'a, ID3D11DeviceChild> {
        ::windows::Param::Owned(::std::convert::Into::<ID3D11DeviceChild>::into(
            ::std::clone::Clone::clone(self),
        ))
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ID3D11ComputeShader_abi(
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        iid: &::windows::Guid,
        interface: *mut ::windows::RawPtr,
    ) -> ::windows::ErrorCode,
    pub unsafe extern "system" fn(this: ::windows::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::RawPtr, pp_device: *mut ::windows::RawPtr),
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        guid: *const ::windows::Guid,
        p_data_size: *mut u32,
        p_data: *mut ::std::ffi::c_void,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        guid: *const ::windows::Guid,
        data_size: u32,
        p_data: *const ::std::ffi::c_void,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        guid: *const ::windows::Guid,
        p_data: ::windows::RawPtr,
    ) -> ::windows::ErrorCode,
);
#[repr(C)]
#[allow(non_snake_case)]
#[derive(:: std :: clone :: Clone)]
pub struct D3D11_BLEND_DESC {
    pub alpha_to_coverage_enable: super::system_services::BOOL,
    pub independent_blend_enable: super::system_services::BOOL,
    pub render_target: ::windows::NOT_YET_SUPPORTED_TYPE,
}
impl D3D11_BLEND_DESC {}
impl ::std::default::Default for D3D11_BLEND_DESC {
    fn default() -> Self {
        Self {
            alpha_to_coverage_enable: ::std::default::Default::default(),
            independent_blend_enable: ::std::default::Default::default(),
            render_target: ::std::default::Default::default(),
        }
    }
}
impl ::std::fmt::Debug for D3D11_BLEND_DESC {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("D3D11_BLEND_DESC")
            .field(
                "alpha_to_coverage_enable",
                &format_args!("{:?}", self.alpha_to_coverage_enable),
            )
            .field(
                "independent_blend_enable",
                &format_args!("{:?}", self.independent_blend_enable),
            )
            .field("render_target", &format_args!("{:?}", self.render_target))
            .finish()
    }
}
impl ::std::cmp::PartialEq for D3D11_BLEND_DESC {
    fn eq(&self, other: &Self) -> bool {
        self.alpha_to_coverage_enable == other.alpha_to_coverage_enable
            && self.independent_blend_enable == other.independent_blend_enable
            && self.render_target == other.render_target
    }
}
impl ::std::cmp::Eq for D3D11_BLEND_DESC {}
unsafe impl ::windows::Abi for D3D11_BLEND_DESC {
    type Abi = Self;
}
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct ID3D11BlendState(::windows::IUnknown);
impl ID3D11BlendState {}
unsafe impl ::windows::Interface for ID3D11BlendState {
    type Vtable = ID3D11BlendState_abi;
    const IID: ::windows::Guid = ::windows::Guid::from_values(
        1974898602,
        13437,
        16729,
        [143, 69, 160, 100, 15, 1, 205, 154],
    );
}
#[allow(non_snake_case)]
impl ID3D11BlendState {
    pub unsafe fn GetDevice(&self, pp_device: *mut ::std::option::Option<ID3D11Device>) {
        (::windows::Interface::vtable(self).3)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(pp_device),
        )
    }
    pub unsafe fn GetPrivateData(
        &self,
        guid: *const ::windows::Guid,
        p_data_size: *mut u32,
        p_data: *mut ::std::ffi::c_void,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).4)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(guid),
            ::std::mem::transmute(p_data_size),
            ::std::mem::transmute(p_data),
        )
    }
    pub unsafe fn SetPrivateData(
        &self,
        guid: *const ::windows::Guid,
        data_size: u32,
        p_data: *const ::std::ffi::c_void,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).5)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(guid),
            ::std::mem::transmute(data_size),
            ::std::mem::transmute(p_data),
        )
    }
    pub unsafe fn SetPrivateDataInterface<
        'a,
        T1__: ::windows::IntoParam<'a, ::windows::IUnknown>,
    >(
        &self,
        guid: *const ::windows::Guid,
        p_data: T1__,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).6)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(guid),
            p_data.into_param().abi(),
        )
    }
    pub unsafe fn GetDesc(&self, p_desc: *mut D3D11_BLEND_DESC) {
        (::windows::Interface::vtable(self).7)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(p_desc),
        )
    }
}
impl ::std::convert::From<ID3D11BlendState> for ::windows::IUnknown {
    fn from(value: ID3D11BlendState) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&ID3D11BlendState> for ::windows::IUnknown {
    fn from(value: &ID3D11BlendState) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::IntoParam<'a, ::windows::IUnknown> for ID3D11BlendState {
    fn into_param(self) -> ::windows::Param<'a, ::windows::IUnknown> {
        ::windows::Param::Owned(::std::convert::Into::<::windows::IUnknown>::into(self))
    }
}
impl<'a> ::windows::IntoParam<'a, ::windows::IUnknown> for &'a ID3D11BlendState {
    fn into_param(self) -> ::windows::Param<'a, ::windows::IUnknown> {
        ::windows::Param::Owned(::std::convert::Into::<::windows::IUnknown>::into(
            ::std::clone::Clone::clone(self),
        ))
    }
}
impl ::std::convert::From<ID3D11BlendState> for ID3D11DeviceChild {
    fn from(value: ID3D11BlendState) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&ID3D11BlendState> for ID3D11DeviceChild {
    fn from(value: &ID3D11BlendState) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::IntoParam<'a, ID3D11DeviceChild> for ID3D11BlendState {
    fn into_param(self) -> ::windows::Param<'a, ID3D11DeviceChild> {
        ::windows::Param::Owned(::std::convert::Into::<ID3D11DeviceChild>::into(self))
    }
}
impl<'a> ::windows::IntoParam<'a, ID3D11DeviceChild> for &'a ID3D11BlendState {
    fn into_param(self) -> ::windows::Param<'a, ID3D11DeviceChild> {
        ::windows::Param::Owned(::std::convert::Into::<ID3D11DeviceChild>::into(
            ::std::clone::Clone::clone(self),
        ))
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ID3D11BlendState_abi(
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        iid: &::windows::Guid,
        interface: *mut ::windows::RawPtr,
    ) -> ::windows::ErrorCode,
    pub unsafe extern "system" fn(this: ::windows::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::RawPtr, pp_device: *mut ::windows::RawPtr),
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        guid: *const ::windows::Guid,
        p_data_size: *mut u32,
        p_data: *mut ::std::ffi::c_void,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        guid: *const ::windows::Guid,
        data_size: u32,
        p_data: *const ::std::ffi::c_void,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        guid: *const ::windows::Guid,
        p_data: ::windows::RawPtr,
    ) -> ::windows::ErrorCode,
    pub unsafe extern "system" fn(this: ::windows::RawPtr, p_desc: *mut D3D11_BLEND_DESC),
);
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
pub struct D3D11_DEPTH_WRITE_MASK(pub i32);
impl D3D11_DEPTH_WRITE_MASK {
    #![allow(non_upper_case_globals)]
    pub const D3D11_DEPTH_WRITE_MASK_ZERO: Self = Self(0i32);
    pub const D3D11_DEPTH_WRITE_MASK_ALL: Self = Self(1i32);
}
impl ::std::convert::From<i32> for D3D11_DEPTH_WRITE_MASK {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::Abi for D3D11_DEPTH_WRITE_MASK {
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
pub struct D3D11_COMPARISON_FUNC(pub i32);
impl D3D11_COMPARISON_FUNC {
    #![allow(non_upper_case_globals)]
    pub const D3D11_COMPARISON_NEVER: Self = Self(1i32);
    pub const D3D11_COMPARISON_LESS: Self = Self(2i32);
    pub const D3D11_COMPARISON_EQUAL: Self = Self(3i32);
    pub const D3D11_COMPARISON_LESS_EQUAL: Self = Self(4i32);
    pub const D3D11_COMPARISON_GREATER: Self = Self(5i32);
    pub const D3D11_COMPARISON_NOT_EQUAL: Self = Self(6i32);
    pub const D3D11_COMPARISON_GREATER_EQUAL: Self = Self(7i32);
    pub const D3D11_COMPARISON_ALWAYS: Self = Self(8i32);
}
impl ::std::convert::From<i32> for D3D11_COMPARISON_FUNC {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::Abi for D3D11_COMPARISON_FUNC {
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
pub struct D3D11_STENCIL_OP(pub i32);
impl D3D11_STENCIL_OP {
    #![allow(non_upper_case_globals)]
    pub const D3D11_STENCIL_OP_KEEP: Self = Self(1i32);
    pub const D3D11_STENCIL_OP_ZERO: Self = Self(2i32);
    pub const D3D11_STENCIL_OP_REPLACE: Self = Self(3i32);
    pub const D3D11_STENCIL_OP_INCR_SAT: Self = Self(4i32);
    pub const D3D11_STENCIL_OP_DECR_SAT: Self = Self(5i32);
    pub const D3D11_STENCIL_OP_INVERT: Self = Self(6i32);
    pub const D3D11_STENCIL_OP_INCR: Self = Self(7i32);
    pub const D3D11_STENCIL_OP_DECR: Self = Self(8i32);
}
impl ::std::convert::From<i32> for D3D11_STENCIL_OP {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::Abi for D3D11_STENCIL_OP {
    type Abi = Self;
}
#[repr(C)]
#[allow(non_snake_case)]
#[derive(:: std :: clone :: Clone)]
pub struct D3D11_DEPTH_STENCILOP_DESC {
    pub stencil_fail_op: D3D11_STENCIL_OP,
    pub stencil_depth_fail_op: D3D11_STENCIL_OP,
    pub stencil_pass_op: D3D11_STENCIL_OP,
    pub stencil_func: D3D11_COMPARISON_FUNC,
}
impl D3D11_DEPTH_STENCILOP_DESC {}
impl ::std::default::Default for D3D11_DEPTH_STENCILOP_DESC {
    fn default() -> Self {
        Self {
            stencil_fail_op: ::std::default::Default::default(),
            stencil_depth_fail_op: ::std::default::Default::default(),
            stencil_pass_op: ::std::default::Default::default(),
            stencil_func: ::std::default::Default::default(),
        }
    }
}
impl ::std::fmt::Debug for D3D11_DEPTH_STENCILOP_DESC {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("D3D11_DEPTH_STENCILOP_DESC")
            .field(
                "stencil_fail_op",
                &format_args!("{:?}", self.stencil_fail_op),
            )
            .field(
                "stencil_depth_fail_op",
                &format_args!("{:?}", self.stencil_depth_fail_op),
            )
            .field(
                "stencil_pass_op",
                &format_args!("{:?}", self.stencil_pass_op),
            )
            .field("stencil_func", &format_args!("{:?}", self.stencil_func))
            .finish()
    }
}
impl ::std::cmp::PartialEq for D3D11_DEPTH_STENCILOP_DESC {
    fn eq(&self, other: &Self) -> bool {
        self.stencil_fail_op == other.stencil_fail_op
            && self.stencil_depth_fail_op == other.stencil_depth_fail_op
            && self.stencil_pass_op == other.stencil_pass_op
            && self.stencil_func == other.stencil_func
    }
}
impl ::std::cmp::Eq for D3D11_DEPTH_STENCILOP_DESC {}
unsafe impl ::windows::Abi for D3D11_DEPTH_STENCILOP_DESC {
    type Abi = Self;
}
#[repr(C)]
#[allow(non_snake_case)]
#[derive(:: std :: clone :: Clone)]
pub struct D3D11_DEPTH_STENCIL_DESC {
    pub depth_enable: super::system_services::BOOL,
    pub depth_write_mask: D3D11_DEPTH_WRITE_MASK,
    pub depth_func: D3D11_COMPARISON_FUNC,
    pub stencil_enable: super::system_services::BOOL,
    pub stencil_read_mask: u8,
    pub stencil_write_mask: u8,
    pub front_face: D3D11_DEPTH_STENCILOP_DESC,
    pub back_face: D3D11_DEPTH_STENCILOP_DESC,
}
impl D3D11_DEPTH_STENCIL_DESC {}
impl ::std::default::Default for D3D11_DEPTH_STENCIL_DESC {
    fn default() -> Self {
        Self {
            depth_enable: ::std::default::Default::default(),
            depth_write_mask: ::std::default::Default::default(),
            depth_func: ::std::default::Default::default(),
            stencil_enable: ::std::default::Default::default(),
            stencil_read_mask: 0,
            stencil_write_mask: 0,
            front_face: ::std::default::Default::default(),
            back_face: ::std::default::Default::default(),
        }
    }
}
impl ::std::fmt::Debug for D3D11_DEPTH_STENCIL_DESC {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("D3D11_DEPTH_STENCIL_DESC")
            .field("depth_enable", &format_args!("{:?}", self.depth_enable))
            .field(
                "depth_write_mask",
                &format_args!("{:?}", self.depth_write_mask),
            )
            .field("depth_func", &format_args!("{:?}", self.depth_func))
            .field("stencil_enable", &format_args!("{:?}", self.stencil_enable))
            .field(
                "stencil_read_mask",
                &format_args!("{:?}", self.stencil_read_mask),
            )
            .field(
                "stencil_write_mask",
                &format_args!("{:?}", self.stencil_write_mask),
            )
            .field("front_face", &format_args!("{:?}", self.front_face))
            .field("back_face", &format_args!("{:?}", self.back_face))
            .finish()
    }
}
impl ::std::cmp::PartialEq for D3D11_DEPTH_STENCIL_DESC {
    fn eq(&self, other: &Self) -> bool {
        self.depth_enable == other.depth_enable
            && self.depth_write_mask == other.depth_write_mask
            && self.depth_func == other.depth_func
            && self.stencil_enable == other.stencil_enable
            && self.stencil_read_mask == other.stencil_read_mask
            && self.stencil_write_mask == other.stencil_write_mask
            && self.front_face == other.front_face
            && self.back_face == other.back_face
    }
}
impl ::std::cmp::Eq for D3D11_DEPTH_STENCIL_DESC {}
unsafe impl ::windows::Abi for D3D11_DEPTH_STENCIL_DESC {
    type Abi = Self;
}
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct ID3D11DepthStencilState(::windows::IUnknown);
impl ID3D11DepthStencilState {}
unsafe impl ::windows::Interface for ID3D11DepthStencilState {
    type Vtable = ID3D11DepthStencilState_abi;
    const IID: ::windows::Guid = ::windows::Guid::from_values(
        58867451,
        36239,
        19996,
        [154, 162, 246, 75, 178, 203, 253, 241],
    );
}
#[allow(non_snake_case)]
impl ID3D11DepthStencilState {
    pub unsafe fn GetDevice(&self, pp_device: *mut ::std::option::Option<ID3D11Device>) {
        (::windows::Interface::vtable(self).3)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(pp_device),
        )
    }
    pub unsafe fn GetPrivateData(
        &self,
        guid: *const ::windows::Guid,
        p_data_size: *mut u32,
        p_data: *mut ::std::ffi::c_void,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).4)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(guid),
            ::std::mem::transmute(p_data_size),
            ::std::mem::transmute(p_data),
        )
    }
    pub unsafe fn SetPrivateData(
        &self,
        guid: *const ::windows::Guid,
        data_size: u32,
        p_data: *const ::std::ffi::c_void,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).5)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(guid),
            ::std::mem::transmute(data_size),
            ::std::mem::transmute(p_data),
        )
    }
    pub unsafe fn SetPrivateDataInterface<
        'a,
        T1__: ::windows::IntoParam<'a, ::windows::IUnknown>,
    >(
        &self,
        guid: *const ::windows::Guid,
        p_data: T1__,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).6)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(guid),
            p_data.into_param().abi(),
        )
    }
    pub unsafe fn GetDesc(&self, p_desc: *mut D3D11_DEPTH_STENCIL_DESC) {
        (::windows::Interface::vtable(self).7)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(p_desc),
        )
    }
}
impl ::std::convert::From<ID3D11DepthStencilState> for ::windows::IUnknown {
    fn from(value: ID3D11DepthStencilState) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&ID3D11DepthStencilState> for ::windows::IUnknown {
    fn from(value: &ID3D11DepthStencilState) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::IntoParam<'a, ::windows::IUnknown> for ID3D11DepthStencilState {
    fn into_param(self) -> ::windows::Param<'a, ::windows::IUnknown> {
        ::windows::Param::Owned(::std::convert::Into::<::windows::IUnknown>::into(self))
    }
}
impl<'a> ::windows::IntoParam<'a, ::windows::IUnknown> for &'a ID3D11DepthStencilState {
    fn into_param(self) -> ::windows::Param<'a, ::windows::IUnknown> {
        ::windows::Param::Owned(::std::convert::Into::<::windows::IUnknown>::into(
            ::std::clone::Clone::clone(self),
        ))
    }
}
impl ::std::convert::From<ID3D11DepthStencilState> for ID3D11DeviceChild {
    fn from(value: ID3D11DepthStencilState) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&ID3D11DepthStencilState> for ID3D11DeviceChild {
    fn from(value: &ID3D11DepthStencilState) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::IntoParam<'a, ID3D11DeviceChild> for ID3D11DepthStencilState {
    fn into_param(self) -> ::windows::Param<'a, ID3D11DeviceChild> {
        ::windows::Param::Owned(::std::convert::Into::<ID3D11DeviceChild>::into(self))
    }
}
impl<'a> ::windows::IntoParam<'a, ID3D11DeviceChild> for &'a ID3D11DepthStencilState {
    fn into_param(self) -> ::windows::Param<'a, ID3D11DeviceChild> {
        ::windows::Param::Owned(::std::convert::Into::<ID3D11DeviceChild>::into(
            ::std::clone::Clone::clone(self),
        ))
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ID3D11DepthStencilState_abi(
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        iid: &::windows::Guid,
        interface: *mut ::windows::RawPtr,
    ) -> ::windows::ErrorCode,
    pub unsafe extern "system" fn(this: ::windows::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::RawPtr, pp_device: *mut ::windows::RawPtr),
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        guid: *const ::windows::Guid,
        p_data_size: *mut u32,
        p_data: *mut ::std::ffi::c_void,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        guid: *const ::windows::Guid,
        data_size: u32,
        p_data: *const ::std::ffi::c_void,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        guid: *const ::windows::Guid,
        p_data: ::windows::RawPtr,
    ) -> ::windows::ErrorCode,
    pub unsafe extern "system" fn(this: ::windows::RawPtr, p_desc: *mut D3D11_DEPTH_STENCIL_DESC),
);
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
pub struct D3D11_FILL_MODE(pub i32);
impl D3D11_FILL_MODE {
    #![allow(non_upper_case_globals)]
    pub const D3D11_FILL_WIREFRAME: Self = Self(2i32);
    pub const D3D11_FILL_SOLID: Self = Self(3i32);
}
impl ::std::convert::From<i32> for D3D11_FILL_MODE {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::Abi for D3D11_FILL_MODE {
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
pub struct D3D11_CULL_MODE(pub i32);
impl D3D11_CULL_MODE {
    #![allow(non_upper_case_globals)]
    pub const D3D11_CULL_NONE: Self = Self(1i32);
    pub const D3D11_CULL_FRONT: Self = Self(2i32);
    pub const D3D11_CULL_BACK: Self = Self(3i32);
}
impl ::std::convert::From<i32> for D3D11_CULL_MODE {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::Abi for D3D11_CULL_MODE {
    type Abi = Self;
}
#[repr(C)]
#[allow(non_snake_case)]
#[derive(:: std :: clone :: Clone)]
pub struct D3D11_RASTERIZER_DESC {
    pub fill_mode: D3D11_FILL_MODE,
    pub cull_mode: D3D11_CULL_MODE,
    pub front_counter_clockwise: super::system_services::BOOL,
    pub depth_bias: i32,
    pub depth_bias_clamp: f32,
    pub slope_scaled_depth_bias: f32,
    pub depth_clip_enable: super::system_services::BOOL,
    pub scissor_enable: super::system_services::BOOL,
    pub multisample_enable: super::system_services::BOOL,
    pub antialiased_line_enable: super::system_services::BOOL,
}
impl D3D11_RASTERIZER_DESC {}
impl ::std::default::Default for D3D11_RASTERIZER_DESC {
    fn default() -> Self {
        Self {
            fill_mode: ::std::default::Default::default(),
            cull_mode: ::std::default::Default::default(),
            front_counter_clockwise: ::std::default::Default::default(),
            depth_bias: 0,
            depth_bias_clamp: 0.0,
            slope_scaled_depth_bias: 0.0,
            depth_clip_enable: ::std::default::Default::default(),
            scissor_enable: ::std::default::Default::default(),
            multisample_enable: ::std::default::Default::default(),
            antialiased_line_enable: ::std::default::Default::default(),
        }
    }
}
impl ::std::fmt::Debug for D3D11_RASTERIZER_DESC {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("D3D11_RASTERIZER_DESC")
            .field("fill_mode", &format_args!("{:?}", self.fill_mode))
            .field("cull_mode", &format_args!("{:?}", self.cull_mode))
            .field(
                "front_counter_clockwise",
                &format_args!("{:?}", self.front_counter_clockwise),
            )
            .field("depth_bias", &format_args!("{:?}", self.depth_bias))
            .field(
                "depth_bias_clamp",
                &format_args!("{:?}", self.depth_bias_clamp),
            )
            .field(
                "slope_scaled_depth_bias",
                &format_args!("{:?}", self.slope_scaled_depth_bias),
            )
            .field(
                "depth_clip_enable",
                &format_args!("{:?}", self.depth_clip_enable),
            )
            .field("scissor_enable", &format_args!("{:?}", self.scissor_enable))
            .field(
                "multisample_enable",
                &format_args!("{:?}", self.multisample_enable),
            )
            .field(
                "antialiased_line_enable",
                &format_args!("{:?}", self.antialiased_line_enable),
            )
            .finish()
    }
}
impl ::std::cmp::PartialEq for D3D11_RASTERIZER_DESC {
    fn eq(&self, other: &Self) -> bool {
        self.fill_mode == other.fill_mode
            && self.cull_mode == other.cull_mode
            && self.front_counter_clockwise == other.front_counter_clockwise
            && self.depth_bias == other.depth_bias
            && self.depth_bias_clamp == other.depth_bias_clamp
            && self.slope_scaled_depth_bias == other.slope_scaled_depth_bias
            && self.depth_clip_enable == other.depth_clip_enable
            && self.scissor_enable == other.scissor_enable
            && self.multisample_enable == other.multisample_enable
            && self.antialiased_line_enable == other.antialiased_line_enable
    }
}
impl ::std::cmp::Eq for D3D11_RASTERIZER_DESC {}
unsafe impl ::windows::Abi for D3D11_RASTERIZER_DESC {
    type Abi = Self;
}
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct ID3D11RasterizerState(::windows::IUnknown);
impl ID3D11RasterizerState {}
unsafe impl ::windows::Interface for ID3D11RasterizerState {
    type Vtable = ID3D11RasterizerState_abi;
    const IID: ::windows::Guid =
        ::windows::Guid::from_values(2612308865, 43802, 19855, [181, 6, 252, 4, 32, 11, 110, 231]);
}
#[allow(non_snake_case)]
impl ID3D11RasterizerState {
    pub unsafe fn GetDevice(&self, pp_device: *mut ::std::option::Option<ID3D11Device>) {
        (::windows::Interface::vtable(self).3)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(pp_device),
        )
    }
    pub unsafe fn GetPrivateData(
        &self,
        guid: *const ::windows::Guid,
        p_data_size: *mut u32,
        p_data: *mut ::std::ffi::c_void,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).4)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(guid),
            ::std::mem::transmute(p_data_size),
            ::std::mem::transmute(p_data),
        )
    }
    pub unsafe fn SetPrivateData(
        &self,
        guid: *const ::windows::Guid,
        data_size: u32,
        p_data: *const ::std::ffi::c_void,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).5)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(guid),
            ::std::mem::transmute(data_size),
            ::std::mem::transmute(p_data),
        )
    }
    pub unsafe fn SetPrivateDataInterface<
        'a,
        T1__: ::windows::IntoParam<'a, ::windows::IUnknown>,
    >(
        &self,
        guid: *const ::windows::Guid,
        p_data: T1__,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).6)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(guid),
            p_data.into_param().abi(),
        )
    }
    pub unsafe fn GetDesc(&self, p_desc: *mut D3D11_RASTERIZER_DESC) {
        (::windows::Interface::vtable(self).7)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(p_desc),
        )
    }
}
impl ::std::convert::From<ID3D11RasterizerState> for ::windows::IUnknown {
    fn from(value: ID3D11RasterizerState) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&ID3D11RasterizerState> for ::windows::IUnknown {
    fn from(value: &ID3D11RasterizerState) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::IntoParam<'a, ::windows::IUnknown> for ID3D11RasterizerState {
    fn into_param(self) -> ::windows::Param<'a, ::windows::IUnknown> {
        ::windows::Param::Owned(::std::convert::Into::<::windows::IUnknown>::into(self))
    }
}
impl<'a> ::windows::IntoParam<'a, ::windows::IUnknown> for &'a ID3D11RasterizerState {
    fn into_param(self) -> ::windows::Param<'a, ::windows::IUnknown> {
        ::windows::Param::Owned(::std::convert::Into::<::windows::IUnknown>::into(
            ::std::clone::Clone::clone(self),
        ))
    }
}
impl ::std::convert::From<ID3D11RasterizerState> for ID3D11DeviceChild {
    fn from(value: ID3D11RasterizerState) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&ID3D11RasterizerState> for ID3D11DeviceChild {
    fn from(value: &ID3D11RasterizerState) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::IntoParam<'a, ID3D11DeviceChild> for ID3D11RasterizerState {
    fn into_param(self) -> ::windows::Param<'a, ID3D11DeviceChild> {
        ::windows::Param::Owned(::std::convert::Into::<ID3D11DeviceChild>::into(self))
    }
}
impl<'a> ::windows::IntoParam<'a, ID3D11DeviceChild> for &'a ID3D11RasterizerState {
    fn into_param(self) -> ::windows::Param<'a, ID3D11DeviceChild> {
        ::windows::Param::Owned(::std::convert::Into::<ID3D11DeviceChild>::into(
            ::std::clone::Clone::clone(self),
        ))
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ID3D11RasterizerState_abi(
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        iid: &::windows::Guid,
        interface: *mut ::windows::RawPtr,
    ) -> ::windows::ErrorCode,
    pub unsafe extern "system" fn(this: ::windows::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::RawPtr, pp_device: *mut ::windows::RawPtr),
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        guid: *const ::windows::Guid,
        p_data_size: *mut u32,
        p_data: *mut ::std::ffi::c_void,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        guid: *const ::windows::Guid,
        data_size: u32,
        p_data: *const ::std::ffi::c_void,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        guid: *const ::windows::Guid,
        p_data: ::windows::RawPtr,
    ) -> ::windows::ErrorCode,
    pub unsafe extern "system" fn(this: ::windows::RawPtr, p_desc: *mut D3D11_RASTERIZER_DESC),
);
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
pub struct D3D11_FILTER(pub i32);
impl D3D11_FILTER {
    #![allow(non_upper_case_globals)]
    pub const D3D11_FILTER_MIN_MAG_MIP_POINT: Self = Self(0i32);
    pub const D3D11_FILTER_MIN_MAG_POINT_MIP_LINEAR: Self = Self(1i32);
    pub const D3D11_FILTER_MIN_POINT_MAG_LINEAR_MIP_POINT: Self = Self(4i32);
    pub const D3D11_FILTER_MIN_POINT_MAG_MIP_LINEAR: Self = Self(5i32);
    pub const D3D11_FILTER_MIN_LINEAR_MAG_MIP_POINT: Self = Self(16i32);
    pub const D3D11_FILTER_MIN_LINEAR_MAG_POINT_MIP_LINEAR: Self = Self(17i32);
    pub const D3D11_FILTER_MIN_MAG_LINEAR_MIP_POINT: Self = Self(20i32);
    pub const D3D11_FILTER_MIN_MAG_MIP_LINEAR: Self = Self(21i32);
    pub const D3D11_FILTER_ANISOTROPIC: Self = Self(85i32);
    pub const D3D11_FILTER_COMPARISON_MIN_MAG_MIP_POINT: Self = Self(128i32);
    pub const D3D11_FILTER_COMPARISON_MIN_MAG_POINT_MIP_LINEAR: Self = Self(129i32);
    pub const D3D11_FILTER_COMPARISON_MIN_POINT_MAG_LINEAR_MIP_POINT: Self = Self(132i32);
    pub const D3D11_FILTER_COMPARISON_MIN_POINT_MAG_MIP_LINEAR: Self = Self(133i32);
    pub const D3D11_FILTER_COMPARISON_MIN_LINEAR_MAG_MIP_POINT: Self = Self(144i32);
    pub const D3D11_FILTER_COMPARISON_MIN_LINEAR_MAG_POINT_MIP_LINEAR: Self = Self(145i32);
    pub const D3D11_FILTER_COMPARISON_MIN_MAG_LINEAR_MIP_POINT: Self = Self(148i32);
    pub const D3D11_FILTER_COMPARISON_MIN_MAG_MIP_LINEAR: Self = Self(149i32);
    pub const D3D11_FILTER_COMPARISON_ANISOTROPIC: Self = Self(213i32);
    pub const D3D11_FILTER_MINIMUM_MIN_MAG_MIP_POINT: Self = Self(256i32);
    pub const D3D11_FILTER_MINIMUM_MIN_MAG_POINT_MIP_LINEAR: Self = Self(257i32);
    pub const D3D11_FILTER_MINIMUM_MIN_POINT_MAG_LINEAR_MIP_POINT: Self = Self(260i32);
    pub const D3D11_FILTER_MINIMUM_MIN_POINT_MAG_MIP_LINEAR: Self = Self(261i32);
    pub const D3D11_FILTER_MINIMUM_MIN_LINEAR_MAG_MIP_POINT: Self = Self(272i32);
    pub const D3D11_FILTER_MINIMUM_MIN_LINEAR_MAG_POINT_MIP_LINEAR: Self = Self(273i32);
    pub const D3D11_FILTER_MINIMUM_MIN_MAG_LINEAR_MIP_POINT: Self = Self(276i32);
    pub const D3D11_FILTER_MINIMUM_MIN_MAG_MIP_LINEAR: Self = Self(277i32);
    pub const D3D11_FILTER_MINIMUM_ANISOTROPIC: Self = Self(341i32);
    pub const D3D11_FILTER_MAXIMUM_MIN_MAG_MIP_POINT: Self = Self(384i32);
    pub const D3D11_FILTER_MAXIMUM_MIN_MAG_POINT_MIP_LINEAR: Self = Self(385i32);
    pub const D3D11_FILTER_MAXIMUM_MIN_POINT_MAG_LINEAR_MIP_POINT: Self = Self(388i32);
    pub const D3D11_FILTER_MAXIMUM_MIN_POINT_MAG_MIP_LINEAR: Self = Self(389i32);
    pub const D3D11_FILTER_MAXIMUM_MIN_LINEAR_MAG_MIP_POINT: Self = Self(400i32);
    pub const D3D11_FILTER_MAXIMUM_MIN_LINEAR_MAG_POINT_MIP_LINEAR: Self = Self(401i32);
    pub const D3D11_FILTER_MAXIMUM_MIN_MAG_LINEAR_MIP_POINT: Self = Self(404i32);
    pub const D3D11_FILTER_MAXIMUM_MIN_MAG_MIP_LINEAR: Self = Self(405i32);
    pub const D3D11_FILTER_MAXIMUM_ANISOTROPIC: Self = Self(469i32);
}
impl ::std::convert::From<i32> for D3D11_FILTER {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::Abi for D3D11_FILTER {
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
pub struct D3D11_TEXTURE_ADDRESS_MODE(pub i32);
impl D3D11_TEXTURE_ADDRESS_MODE {
    #![allow(non_upper_case_globals)]
    pub const D3D11_TEXTURE_ADDRESS_WRAP: Self = Self(1i32);
    pub const D3D11_TEXTURE_ADDRESS_MIRROR: Self = Self(2i32);
    pub const D3D11_TEXTURE_ADDRESS_CLAMP: Self = Self(3i32);
    pub const D3D11_TEXTURE_ADDRESS_BORDER: Self = Self(4i32);
    pub const D3D11_TEXTURE_ADDRESS_MIRROR_ONCE: Self = Self(5i32);
}
impl ::std::convert::From<i32> for D3D11_TEXTURE_ADDRESS_MODE {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::Abi for D3D11_TEXTURE_ADDRESS_MODE {
    type Abi = Self;
}
#[repr(C)]
#[allow(non_snake_case)]
#[derive(:: std :: clone :: Clone)]
pub struct D3D11_SAMPLER_DESC {
    pub filter: D3D11_FILTER,
    pub addressu: D3D11_TEXTURE_ADDRESS_MODE,
    pub addressv: D3D11_TEXTURE_ADDRESS_MODE,
    pub addressw: D3D11_TEXTURE_ADDRESS_MODE,
    pub mip_lod_bias: f32,
    pub max_anisotropy: u32,
    pub comparison_func: D3D11_COMPARISON_FUNC,
    pub border_color: ::windows::NOT_YET_SUPPORTED_TYPE,
    pub min_lod: f32,
    pub max_lod: f32,
}
impl D3D11_SAMPLER_DESC {}
impl ::std::default::Default for D3D11_SAMPLER_DESC {
    fn default() -> Self {
        Self {
            filter: ::std::default::Default::default(),
            addressu: ::std::default::Default::default(),
            addressv: ::std::default::Default::default(),
            addressw: ::std::default::Default::default(),
            mip_lod_bias: 0.0,
            max_anisotropy: 0,
            comparison_func: ::std::default::Default::default(),
            border_color: ::std::default::Default::default(),
            min_lod: 0.0,
            max_lod: 0.0,
        }
    }
}
impl ::std::fmt::Debug for D3D11_SAMPLER_DESC {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("D3D11_SAMPLER_DESC")
            .field("filter", &format_args!("{:?}", self.filter))
            .field("addressu", &format_args!("{:?}", self.addressu))
            .field("addressv", &format_args!("{:?}", self.addressv))
            .field("addressw", &format_args!("{:?}", self.addressw))
            .field("mip_lod_bias", &format_args!("{:?}", self.mip_lod_bias))
            .field("max_anisotropy", &format_args!("{:?}", self.max_anisotropy))
            .field(
                "comparison_func",
                &format_args!("{:?}", self.comparison_func),
            )
            .field("border_color", &format_args!("{:?}", self.border_color))
            .field("min_lod", &format_args!("{:?}", self.min_lod))
            .field("max_lod", &format_args!("{:?}", self.max_lod))
            .finish()
    }
}
impl ::std::cmp::PartialEq for D3D11_SAMPLER_DESC {
    fn eq(&self, other: &Self) -> bool {
        self.filter == other.filter
            && self.addressu == other.addressu
            && self.addressv == other.addressv
            && self.addressw == other.addressw
            && self.mip_lod_bias == other.mip_lod_bias
            && self.max_anisotropy == other.max_anisotropy
            && self.comparison_func == other.comparison_func
            && self.border_color == other.border_color
            && self.min_lod == other.min_lod
            && self.max_lod == other.max_lod
    }
}
impl ::std::cmp::Eq for D3D11_SAMPLER_DESC {}
unsafe impl ::windows::Abi for D3D11_SAMPLER_DESC {
    type Abi = Self;
}
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct ID3D11SamplerState(::windows::IUnknown);
impl ID3D11SamplerState {}
unsafe impl ::windows::Interface for ID3D11SamplerState {
    type Vtable = ID3D11SamplerState_abi;
    const IID: ::windows::Guid = ::windows::Guid::from_values(
        3664767569,
        22092,
        17543,
        [152, 16, 240, 208, 249, 180, 227, 165],
    );
}
#[allow(non_snake_case)]
impl ID3D11SamplerState {
    pub unsafe fn GetDevice(&self, pp_device: *mut ::std::option::Option<ID3D11Device>) {
        (::windows::Interface::vtable(self).3)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(pp_device),
        )
    }
    pub unsafe fn GetPrivateData(
        &self,
        guid: *const ::windows::Guid,
        p_data_size: *mut u32,
        p_data: *mut ::std::ffi::c_void,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).4)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(guid),
            ::std::mem::transmute(p_data_size),
            ::std::mem::transmute(p_data),
        )
    }
    pub unsafe fn SetPrivateData(
        &self,
        guid: *const ::windows::Guid,
        data_size: u32,
        p_data: *const ::std::ffi::c_void,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).5)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(guid),
            ::std::mem::transmute(data_size),
            ::std::mem::transmute(p_data),
        )
    }
    pub unsafe fn SetPrivateDataInterface<
        'a,
        T1__: ::windows::IntoParam<'a, ::windows::IUnknown>,
    >(
        &self,
        guid: *const ::windows::Guid,
        p_data: T1__,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).6)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(guid),
            p_data.into_param().abi(),
        )
    }
    pub unsafe fn GetDesc(&self, p_desc: *mut D3D11_SAMPLER_DESC) {
        (::windows::Interface::vtable(self).7)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(p_desc),
        )
    }
}
impl ::std::convert::From<ID3D11SamplerState> for ::windows::IUnknown {
    fn from(value: ID3D11SamplerState) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&ID3D11SamplerState> for ::windows::IUnknown {
    fn from(value: &ID3D11SamplerState) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::IntoParam<'a, ::windows::IUnknown> for ID3D11SamplerState {
    fn into_param(self) -> ::windows::Param<'a, ::windows::IUnknown> {
        ::windows::Param::Owned(::std::convert::Into::<::windows::IUnknown>::into(self))
    }
}
impl<'a> ::windows::IntoParam<'a, ::windows::IUnknown> for &'a ID3D11SamplerState {
    fn into_param(self) -> ::windows::Param<'a, ::windows::IUnknown> {
        ::windows::Param::Owned(::std::convert::Into::<::windows::IUnknown>::into(
            ::std::clone::Clone::clone(self),
        ))
    }
}
impl ::std::convert::From<ID3D11SamplerState> for ID3D11DeviceChild {
    fn from(value: ID3D11SamplerState) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&ID3D11SamplerState> for ID3D11DeviceChild {
    fn from(value: &ID3D11SamplerState) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::IntoParam<'a, ID3D11DeviceChild> for ID3D11SamplerState {
    fn into_param(self) -> ::windows::Param<'a, ID3D11DeviceChild> {
        ::windows::Param::Owned(::std::convert::Into::<ID3D11DeviceChild>::into(self))
    }
}
impl<'a> ::windows::IntoParam<'a, ID3D11DeviceChild> for &'a ID3D11SamplerState {
    fn into_param(self) -> ::windows::Param<'a, ID3D11DeviceChild> {
        ::windows::Param::Owned(::std::convert::Into::<ID3D11DeviceChild>::into(
            ::std::clone::Clone::clone(self),
        ))
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ID3D11SamplerState_abi(
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        iid: &::windows::Guid,
        interface: *mut ::windows::RawPtr,
    ) -> ::windows::ErrorCode,
    pub unsafe extern "system" fn(this: ::windows::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::RawPtr, pp_device: *mut ::windows::RawPtr),
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        guid: *const ::windows::Guid,
        p_data_size: *mut u32,
        p_data: *mut ::std::ffi::c_void,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        guid: *const ::windows::Guid,
        data_size: u32,
        p_data: *const ::std::ffi::c_void,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        guid: *const ::windows::Guid,
        p_data: ::windows::RawPtr,
    ) -> ::windows::ErrorCode,
    pub unsafe extern "system" fn(this: ::windows::RawPtr, p_desc: *mut D3D11_SAMPLER_DESC),
);
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
pub struct D3D11_QUERY(pub i32);
impl D3D11_QUERY {
    #![allow(non_upper_case_globals)]
    pub const D3D11_QUERY_EVENT: Self = Self(0i32);
    pub const D3D11_QUERY_OCCLUSION: Self = Self(1i32);
    pub const D3D11_QUERY_TIMESTAMP: Self = Self(2i32);
    pub const D3D11_QUERY_TIMESTAMP_DISJOINT: Self = Self(3i32);
    pub const D3D11_QUERY_PIPELINE_STATISTICS: Self = Self(4i32);
    pub const D3D11_QUERY_OCCLUSION_PREDICATE: Self = Self(5i32);
    pub const D3D11_QUERY_SO_STATISTICS: Self = Self(6i32);
    pub const D3D11_QUERY_SO_OVERFLOW_PREDICATE: Self = Self(7i32);
    pub const D3D11_QUERY_SO_STATISTICS_STREAM0: Self = Self(8i32);
    pub const D3D11_QUERY_SO_OVERFLOW_PREDICATE_STREAM0: Self = Self(9i32);
    pub const D3D11_QUERY_SO_STATISTICS_STREAM1: Self = Self(10i32);
    pub const D3D11_QUERY_SO_OVERFLOW_PREDICATE_STREAM1: Self = Self(11i32);
    pub const D3D11_QUERY_SO_STATISTICS_STREAM2: Self = Self(12i32);
    pub const D3D11_QUERY_SO_OVERFLOW_PREDICATE_STREAM2: Self = Self(13i32);
    pub const D3D11_QUERY_SO_STATISTICS_STREAM3: Self = Self(14i32);
    pub const D3D11_QUERY_SO_OVERFLOW_PREDICATE_STREAM3: Self = Self(15i32);
}
impl ::std::convert::From<i32> for D3D11_QUERY {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::Abi for D3D11_QUERY {
    type Abi = Self;
}
#[repr(C)]
#[allow(non_snake_case)]
#[derive(:: std :: clone :: Clone)]
pub struct D3D11_QUERY_DESC {
    pub query: D3D11_QUERY,
    pub misc_flags: u32,
}
impl D3D11_QUERY_DESC {}
impl ::std::default::Default for D3D11_QUERY_DESC {
    fn default() -> Self {
        Self {
            query: ::std::default::Default::default(),
            misc_flags: 0,
        }
    }
}
impl ::std::fmt::Debug for D3D11_QUERY_DESC {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("D3D11_QUERY_DESC")
            .field("query", &format_args!("{:?}", self.query))
            .field("misc_flags", &format_args!("{:?}", self.misc_flags))
            .finish()
    }
}
impl ::std::cmp::PartialEq for D3D11_QUERY_DESC {
    fn eq(&self, other: &Self) -> bool {
        self.query == other.query && self.misc_flags == other.misc_flags
    }
}
impl ::std::cmp::Eq for D3D11_QUERY_DESC {}
unsafe impl ::windows::Abi for D3D11_QUERY_DESC {
    type Abi = Self;
}
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct ID3D11Asynchronous(::windows::IUnknown);
impl ID3D11Asynchronous {}
unsafe impl ::windows::Interface for ID3D11Asynchronous {
    type Vtable = ID3D11Asynchronous_abi;
    const IID: ::windows::Guid = ::windows::Guid::from_values(
        1261818061,
        7701,
        16984,
        [156, 152, 27, 19, 51, 246, 221, 59],
    );
}
#[allow(non_snake_case)]
impl ID3D11Asynchronous {
    pub unsafe fn GetDevice(&self, pp_device: *mut ::std::option::Option<ID3D11Device>) {
        (::windows::Interface::vtable(self).3)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(pp_device),
        )
    }
    pub unsafe fn GetPrivateData(
        &self,
        guid: *const ::windows::Guid,
        p_data_size: *mut u32,
        p_data: *mut ::std::ffi::c_void,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).4)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(guid),
            ::std::mem::transmute(p_data_size),
            ::std::mem::transmute(p_data),
        )
    }
    pub unsafe fn SetPrivateData(
        &self,
        guid: *const ::windows::Guid,
        data_size: u32,
        p_data: *const ::std::ffi::c_void,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).5)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(guid),
            ::std::mem::transmute(data_size),
            ::std::mem::transmute(p_data),
        )
    }
    pub unsafe fn SetPrivateDataInterface<
        'a,
        T1__: ::windows::IntoParam<'a, ::windows::IUnknown>,
    >(
        &self,
        guid: *const ::windows::Guid,
        p_data: T1__,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).6)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(guid),
            p_data.into_param().abi(),
        )
    }
    pub unsafe fn GetDataSize(&self) -> u32 {
        (::windows::Interface::vtable(self).7)(::windows::Abi::abi(self))
    }
}
impl ::std::convert::From<ID3D11Asynchronous> for ::windows::IUnknown {
    fn from(value: ID3D11Asynchronous) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&ID3D11Asynchronous> for ::windows::IUnknown {
    fn from(value: &ID3D11Asynchronous) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::IntoParam<'a, ::windows::IUnknown> for ID3D11Asynchronous {
    fn into_param(self) -> ::windows::Param<'a, ::windows::IUnknown> {
        ::windows::Param::Owned(::std::convert::Into::<::windows::IUnknown>::into(self))
    }
}
impl<'a> ::windows::IntoParam<'a, ::windows::IUnknown> for &'a ID3D11Asynchronous {
    fn into_param(self) -> ::windows::Param<'a, ::windows::IUnknown> {
        ::windows::Param::Owned(::std::convert::Into::<::windows::IUnknown>::into(
            ::std::clone::Clone::clone(self),
        ))
    }
}
impl ::std::convert::From<ID3D11Asynchronous> for ID3D11DeviceChild {
    fn from(value: ID3D11Asynchronous) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&ID3D11Asynchronous> for ID3D11DeviceChild {
    fn from(value: &ID3D11Asynchronous) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::IntoParam<'a, ID3D11DeviceChild> for ID3D11Asynchronous {
    fn into_param(self) -> ::windows::Param<'a, ID3D11DeviceChild> {
        ::windows::Param::Owned(::std::convert::Into::<ID3D11DeviceChild>::into(self))
    }
}
impl<'a> ::windows::IntoParam<'a, ID3D11DeviceChild> for &'a ID3D11Asynchronous {
    fn into_param(self) -> ::windows::Param<'a, ID3D11DeviceChild> {
        ::windows::Param::Owned(::std::convert::Into::<ID3D11DeviceChild>::into(
            ::std::clone::Clone::clone(self),
        ))
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ID3D11Asynchronous_abi(
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        iid: &::windows::Guid,
        interface: *mut ::windows::RawPtr,
    ) -> ::windows::ErrorCode,
    pub unsafe extern "system" fn(this: ::windows::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::RawPtr, pp_device: *mut ::windows::RawPtr),
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        guid: *const ::windows::Guid,
        p_data_size: *mut u32,
        p_data: *mut ::std::ffi::c_void,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        guid: *const ::windows::Guid,
        data_size: u32,
        p_data: *const ::std::ffi::c_void,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        guid: *const ::windows::Guid,
        p_data: ::windows::RawPtr,
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
pub struct ID3D11Query(::windows::IUnknown);
impl ID3D11Query {}
unsafe impl ::windows::Interface for ID3D11Query {
    type Vtable = ID3D11Query_abi;
    const IID: ::windows::Guid =
        ::windows::Guid::from_values(3602908999, 34743, 16990, [184, 77, 68, 209, 8, 86, 10, 253]);
}
#[allow(non_snake_case)]
impl ID3D11Query {
    pub unsafe fn GetDevice(&self, pp_device: *mut ::std::option::Option<ID3D11Device>) {
        (::windows::Interface::vtable(self).3)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(pp_device),
        )
    }
    pub unsafe fn GetPrivateData(
        &self,
        guid: *const ::windows::Guid,
        p_data_size: *mut u32,
        p_data: *mut ::std::ffi::c_void,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).4)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(guid),
            ::std::mem::transmute(p_data_size),
            ::std::mem::transmute(p_data),
        )
    }
    pub unsafe fn SetPrivateData(
        &self,
        guid: *const ::windows::Guid,
        data_size: u32,
        p_data: *const ::std::ffi::c_void,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).5)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(guid),
            ::std::mem::transmute(data_size),
            ::std::mem::transmute(p_data),
        )
    }
    pub unsafe fn SetPrivateDataInterface<
        'a,
        T1__: ::windows::IntoParam<'a, ::windows::IUnknown>,
    >(
        &self,
        guid: *const ::windows::Guid,
        p_data: T1__,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).6)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(guid),
            p_data.into_param().abi(),
        )
    }
    pub unsafe fn GetDataSize(&self) -> u32 {
        (::windows::Interface::vtable(self).7)(::windows::Abi::abi(self))
    }
    pub unsafe fn GetDesc(&self, p_desc: *mut D3D11_QUERY_DESC) {
        (::windows::Interface::vtable(self).8)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(p_desc),
        )
    }
}
impl ::std::convert::From<ID3D11Query> for ::windows::IUnknown {
    fn from(value: ID3D11Query) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&ID3D11Query> for ::windows::IUnknown {
    fn from(value: &ID3D11Query) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::IntoParam<'a, ::windows::IUnknown> for ID3D11Query {
    fn into_param(self) -> ::windows::Param<'a, ::windows::IUnknown> {
        ::windows::Param::Owned(::std::convert::Into::<::windows::IUnknown>::into(self))
    }
}
impl<'a> ::windows::IntoParam<'a, ::windows::IUnknown> for &'a ID3D11Query {
    fn into_param(self) -> ::windows::Param<'a, ::windows::IUnknown> {
        ::windows::Param::Owned(::std::convert::Into::<::windows::IUnknown>::into(
            ::std::clone::Clone::clone(self),
        ))
    }
}
impl ::std::convert::From<ID3D11Query> for ID3D11Asynchronous {
    fn from(value: ID3D11Query) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&ID3D11Query> for ID3D11Asynchronous {
    fn from(value: &ID3D11Query) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::IntoParam<'a, ID3D11Asynchronous> for ID3D11Query {
    fn into_param(self) -> ::windows::Param<'a, ID3D11Asynchronous> {
        ::windows::Param::Owned(::std::convert::Into::<ID3D11Asynchronous>::into(self))
    }
}
impl<'a> ::windows::IntoParam<'a, ID3D11Asynchronous> for &'a ID3D11Query {
    fn into_param(self) -> ::windows::Param<'a, ID3D11Asynchronous> {
        ::windows::Param::Owned(::std::convert::Into::<ID3D11Asynchronous>::into(
            ::std::clone::Clone::clone(self),
        ))
    }
}
impl ::std::convert::From<ID3D11Query> for ID3D11DeviceChild {
    fn from(value: ID3D11Query) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&ID3D11Query> for ID3D11DeviceChild {
    fn from(value: &ID3D11Query) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::IntoParam<'a, ID3D11DeviceChild> for ID3D11Query {
    fn into_param(self) -> ::windows::Param<'a, ID3D11DeviceChild> {
        ::windows::Param::Owned(::std::convert::Into::<ID3D11DeviceChild>::into(self))
    }
}
impl<'a> ::windows::IntoParam<'a, ID3D11DeviceChild> for &'a ID3D11Query {
    fn into_param(self) -> ::windows::Param<'a, ID3D11DeviceChild> {
        ::windows::Param::Owned(::std::convert::Into::<ID3D11DeviceChild>::into(
            ::std::clone::Clone::clone(self),
        ))
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ID3D11Query_abi(
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        iid: &::windows::Guid,
        interface: *mut ::windows::RawPtr,
    ) -> ::windows::ErrorCode,
    pub unsafe extern "system" fn(this: ::windows::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::RawPtr, pp_device: *mut ::windows::RawPtr),
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        guid: *const ::windows::Guid,
        p_data_size: *mut u32,
        p_data: *mut ::std::ffi::c_void,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        guid: *const ::windows::Guid,
        data_size: u32,
        p_data: *const ::std::ffi::c_void,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        guid: *const ::windows::Guid,
        p_data: ::windows::RawPtr,
    ) -> ::windows::ErrorCode,
    pub unsafe extern "system" fn(this: ::windows::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::RawPtr, p_desc: *mut D3D11_QUERY_DESC),
);
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct ID3D11Predicate(::windows::IUnknown);
impl ID3D11Predicate {}
unsafe impl ::windows::Interface for ID3D11Predicate {
    type Vtable = ID3D11Predicate_abi;
    const IID: ::windows::Guid = ::windows::Guid::from_values(
        2662692573,
        40823,
        19846,
        [129, 170, 139, 171, 95, 228, 144, 226],
    );
}
#[allow(non_snake_case)]
impl ID3D11Predicate {
    pub unsafe fn GetDevice(&self, pp_device: *mut ::std::option::Option<ID3D11Device>) {
        (::windows::Interface::vtable(self).3)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(pp_device),
        )
    }
    pub unsafe fn GetPrivateData(
        &self,
        guid: *const ::windows::Guid,
        p_data_size: *mut u32,
        p_data: *mut ::std::ffi::c_void,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).4)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(guid),
            ::std::mem::transmute(p_data_size),
            ::std::mem::transmute(p_data),
        )
    }
    pub unsafe fn SetPrivateData(
        &self,
        guid: *const ::windows::Guid,
        data_size: u32,
        p_data: *const ::std::ffi::c_void,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).5)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(guid),
            ::std::mem::transmute(data_size),
            ::std::mem::transmute(p_data),
        )
    }
    pub unsafe fn SetPrivateDataInterface<
        'a,
        T1__: ::windows::IntoParam<'a, ::windows::IUnknown>,
    >(
        &self,
        guid: *const ::windows::Guid,
        p_data: T1__,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).6)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(guid),
            p_data.into_param().abi(),
        )
    }
    pub unsafe fn GetDataSize(&self) -> u32 {
        (::windows::Interface::vtable(self).7)(::windows::Abi::abi(self))
    }
    pub unsafe fn GetDesc(&self, p_desc: *mut D3D11_QUERY_DESC) {
        (::windows::Interface::vtable(self).8)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(p_desc),
        )
    }
}
impl ::std::convert::From<ID3D11Predicate> for ::windows::IUnknown {
    fn from(value: ID3D11Predicate) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&ID3D11Predicate> for ::windows::IUnknown {
    fn from(value: &ID3D11Predicate) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::IntoParam<'a, ::windows::IUnknown> for ID3D11Predicate {
    fn into_param(self) -> ::windows::Param<'a, ::windows::IUnknown> {
        ::windows::Param::Owned(::std::convert::Into::<::windows::IUnknown>::into(self))
    }
}
impl<'a> ::windows::IntoParam<'a, ::windows::IUnknown> for &'a ID3D11Predicate {
    fn into_param(self) -> ::windows::Param<'a, ::windows::IUnknown> {
        ::windows::Param::Owned(::std::convert::Into::<::windows::IUnknown>::into(
            ::std::clone::Clone::clone(self),
        ))
    }
}
impl ::std::convert::From<ID3D11Predicate> for ID3D11Query {
    fn from(value: ID3D11Predicate) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&ID3D11Predicate> for ID3D11Query {
    fn from(value: &ID3D11Predicate) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::IntoParam<'a, ID3D11Query> for ID3D11Predicate {
    fn into_param(self) -> ::windows::Param<'a, ID3D11Query> {
        ::windows::Param::Owned(::std::convert::Into::<ID3D11Query>::into(self))
    }
}
impl<'a> ::windows::IntoParam<'a, ID3D11Query> for &'a ID3D11Predicate {
    fn into_param(self) -> ::windows::Param<'a, ID3D11Query> {
        ::windows::Param::Owned(::std::convert::Into::<ID3D11Query>::into(
            ::std::clone::Clone::clone(self),
        ))
    }
}
impl ::std::convert::From<ID3D11Predicate> for ID3D11Asynchronous {
    fn from(value: ID3D11Predicate) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&ID3D11Predicate> for ID3D11Asynchronous {
    fn from(value: &ID3D11Predicate) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::IntoParam<'a, ID3D11Asynchronous> for ID3D11Predicate {
    fn into_param(self) -> ::windows::Param<'a, ID3D11Asynchronous> {
        ::windows::Param::Owned(::std::convert::Into::<ID3D11Asynchronous>::into(self))
    }
}
impl<'a> ::windows::IntoParam<'a, ID3D11Asynchronous> for &'a ID3D11Predicate {
    fn into_param(self) -> ::windows::Param<'a, ID3D11Asynchronous> {
        ::windows::Param::Owned(::std::convert::Into::<ID3D11Asynchronous>::into(
            ::std::clone::Clone::clone(self),
        ))
    }
}
impl ::std::convert::From<ID3D11Predicate> for ID3D11DeviceChild {
    fn from(value: ID3D11Predicate) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&ID3D11Predicate> for ID3D11DeviceChild {
    fn from(value: &ID3D11Predicate) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::IntoParam<'a, ID3D11DeviceChild> for ID3D11Predicate {
    fn into_param(self) -> ::windows::Param<'a, ID3D11DeviceChild> {
        ::windows::Param::Owned(::std::convert::Into::<ID3D11DeviceChild>::into(self))
    }
}
impl<'a> ::windows::IntoParam<'a, ID3D11DeviceChild> for &'a ID3D11Predicate {
    fn into_param(self) -> ::windows::Param<'a, ID3D11DeviceChild> {
        ::windows::Param::Owned(::std::convert::Into::<ID3D11DeviceChild>::into(
            ::std::clone::Clone::clone(self),
        ))
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ID3D11Predicate_abi(
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        iid: &::windows::Guid,
        interface: *mut ::windows::RawPtr,
    ) -> ::windows::ErrorCode,
    pub unsafe extern "system" fn(this: ::windows::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::RawPtr, pp_device: *mut ::windows::RawPtr),
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        guid: *const ::windows::Guid,
        p_data_size: *mut u32,
        p_data: *mut ::std::ffi::c_void,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        guid: *const ::windows::Guid,
        data_size: u32,
        p_data: *const ::std::ffi::c_void,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        guid: *const ::windows::Guid,
        p_data: ::windows::RawPtr,
    ) -> ::windows::ErrorCode,
    pub unsafe extern "system" fn(this: ::windows::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::RawPtr, p_desc: *mut D3D11_QUERY_DESC),
);
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
pub struct D3D11_COUNTER(pub i32);
impl D3D11_COUNTER {
    #![allow(non_upper_case_globals)]
    pub const D3D11_COUNTER_DEVICE_DEPENDENT_0: Self = Self(1073741824i32);
}
impl ::std::convert::From<i32> for D3D11_COUNTER {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::Abi for D3D11_COUNTER {
    type Abi = Self;
}
#[repr(C)]
#[allow(non_snake_case)]
#[derive(:: std :: clone :: Clone)]
pub struct D3D11_COUNTER_DESC {
    pub counter: D3D11_COUNTER,
    pub misc_flags: u32,
}
impl D3D11_COUNTER_DESC {}
impl ::std::default::Default for D3D11_COUNTER_DESC {
    fn default() -> Self {
        Self {
            counter: ::std::default::Default::default(),
            misc_flags: 0,
        }
    }
}
impl ::std::fmt::Debug for D3D11_COUNTER_DESC {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("D3D11_COUNTER_DESC")
            .field("counter", &format_args!("{:?}", self.counter))
            .field("misc_flags", &format_args!("{:?}", self.misc_flags))
            .finish()
    }
}
impl ::std::cmp::PartialEq for D3D11_COUNTER_DESC {
    fn eq(&self, other: &Self) -> bool {
        self.counter == other.counter && self.misc_flags == other.misc_flags
    }
}
impl ::std::cmp::Eq for D3D11_COUNTER_DESC {}
unsafe impl ::windows::Abi for D3D11_COUNTER_DESC {
    type Abi = Self;
}
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct ID3D11Counter(::windows::IUnknown);
impl ID3D11Counter {}
unsafe impl ::windows::Interface for ID3D11Counter {
    type Vtable = ID3D11Counter_abi;
    const IID: ::windows::Guid =
        ::windows::Guid::from_values(1854687739, 41841, 18288, [180, 64, 41, 8, 96, 34, 183, 65]);
}
#[allow(non_snake_case)]
impl ID3D11Counter {
    pub unsafe fn GetDevice(&self, pp_device: *mut ::std::option::Option<ID3D11Device>) {
        (::windows::Interface::vtable(self).3)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(pp_device),
        )
    }
    pub unsafe fn GetPrivateData(
        &self,
        guid: *const ::windows::Guid,
        p_data_size: *mut u32,
        p_data: *mut ::std::ffi::c_void,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).4)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(guid),
            ::std::mem::transmute(p_data_size),
            ::std::mem::transmute(p_data),
        )
    }
    pub unsafe fn SetPrivateData(
        &self,
        guid: *const ::windows::Guid,
        data_size: u32,
        p_data: *const ::std::ffi::c_void,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).5)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(guid),
            ::std::mem::transmute(data_size),
            ::std::mem::transmute(p_data),
        )
    }
    pub unsafe fn SetPrivateDataInterface<
        'a,
        T1__: ::windows::IntoParam<'a, ::windows::IUnknown>,
    >(
        &self,
        guid: *const ::windows::Guid,
        p_data: T1__,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).6)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(guid),
            p_data.into_param().abi(),
        )
    }
    pub unsafe fn GetDataSize(&self) -> u32 {
        (::windows::Interface::vtable(self).7)(::windows::Abi::abi(self))
    }
    pub unsafe fn GetDesc(&self, p_desc: *mut D3D11_COUNTER_DESC) {
        (::windows::Interface::vtable(self).8)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(p_desc),
        )
    }
}
impl ::std::convert::From<ID3D11Counter> for ::windows::IUnknown {
    fn from(value: ID3D11Counter) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&ID3D11Counter> for ::windows::IUnknown {
    fn from(value: &ID3D11Counter) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::IntoParam<'a, ::windows::IUnknown> for ID3D11Counter {
    fn into_param(self) -> ::windows::Param<'a, ::windows::IUnknown> {
        ::windows::Param::Owned(::std::convert::Into::<::windows::IUnknown>::into(self))
    }
}
impl<'a> ::windows::IntoParam<'a, ::windows::IUnknown> for &'a ID3D11Counter {
    fn into_param(self) -> ::windows::Param<'a, ::windows::IUnknown> {
        ::windows::Param::Owned(::std::convert::Into::<::windows::IUnknown>::into(
            ::std::clone::Clone::clone(self),
        ))
    }
}
impl ::std::convert::From<ID3D11Counter> for ID3D11Asynchronous {
    fn from(value: ID3D11Counter) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&ID3D11Counter> for ID3D11Asynchronous {
    fn from(value: &ID3D11Counter) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::IntoParam<'a, ID3D11Asynchronous> for ID3D11Counter {
    fn into_param(self) -> ::windows::Param<'a, ID3D11Asynchronous> {
        ::windows::Param::Owned(::std::convert::Into::<ID3D11Asynchronous>::into(self))
    }
}
impl<'a> ::windows::IntoParam<'a, ID3D11Asynchronous> for &'a ID3D11Counter {
    fn into_param(self) -> ::windows::Param<'a, ID3D11Asynchronous> {
        ::windows::Param::Owned(::std::convert::Into::<ID3D11Asynchronous>::into(
            ::std::clone::Clone::clone(self),
        ))
    }
}
impl ::std::convert::From<ID3D11Counter> for ID3D11DeviceChild {
    fn from(value: ID3D11Counter) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&ID3D11Counter> for ID3D11DeviceChild {
    fn from(value: &ID3D11Counter) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::IntoParam<'a, ID3D11DeviceChild> for ID3D11Counter {
    fn into_param(self) -> ::windows::Param<'a, ID3D11DeviceChild> {
        ::windows::Param::Owned(::std::convert::Into::<ID3D11DeviceChild>::into(self))
    }
}
impl<'a> ::windows::IntoParam<'a, ID3D11DeviceChild> for &'a ID3D11Counter {
    fn into_param(self) -> ::windows::Param<'a, ID3D11DeviceChild> {
        ::windows::Param::Owned(::std::convert::Into::<ID3D11DeviceChild>::into(
            ::std::clone::Clone::clone(self),
        ))
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ID3D11Counter_abi(
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        iid: &::windows::Guid,
        interface: *mut ::windows::RawPtr,
    ) -> ::windows::ErrorCode,
    pub unsafe extern "system" fn(this: ::windows::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::RawPtr, pp_device: *mut ::windows::RawPtr),
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        guid: *const ::windows::Guid,
        p_data_size: *mut u32,
        p_data: *mut ::std::ffi::c_void,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        guid: *const ::windows::Guid,
        data_size: u32,
        p_data: *const ::std::ffi::c_void,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        guid: *const ::windows::Guid,
        p_data: ::windows::RawPtr,
    ) -> ::windows::ErrorCode,
    pub unsafe extern "system" fn(this: ::windows::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::RawPtr, p_desc: *mut D3D11_COUNTER_DESC),
);
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
pub struct D3D11_MAP(pub i32);
impl D3D11_MAP {
    #![allow(non_upper_case_globals)]
    pub const D3D11_MAP_READ: Self = Self(1i32);
    pub const D3D11_MAP_WRITE: Self = Self(2i32);
    pub const D3D11_MAP_READ_WRITE: Self = Self(3i32);
    pub const D3D11_MAP_WRITE_DISCARD: Self = Self(4i32);
    pub const D3D11_MAP_WRITE_NO_OVERWRITE: Self = Self(5i32);
}
impl ::std::convert::From<i32> for D3D11_MAP {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::Abi for D3D11_MAP {
    type Abi = Self;
}
#[repr(C)]
#[allow(non_snake_case)]
#[derive(:: std :: clone :: Clone)]
pub struct D3D11_MAPPED_SUBRESOURCE {
    pub p_data: *mut ::std::ffi::c_void,
    pub row_pitch: u32,
    pub depth_pitch: u32,
}
impl D3D11_MAPPED_SUBRESOURCE {}
impl ::std::default::Default for D3D11_MAPPED_SUBRESOURCE {
    fn default() -> Self {
        Self {
            p_data: ::std::ptr::null_mut(),
            row_pitch: 0,
            depth_pitch: 0,
        }
    }
}
impl ::std::fmt::Debug for D3D11_MAPPED_SUBRESOURCE {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("D3D11_MAPPED_SUBRESOURCE")
            .field("p_data", &format_args!("{:?}", self.p_data))
            .field("row_pitch", &format_args!("{:?}", self.row_pitch))
            .field("depth_pitch", &format_args!("{:?}", self.depth_pitch))
            .finish()
    }
}
impl ::std::cmp::PartialEq for D3D11_MAPPED_SUBRESOURCE {
    fn eq(&self, other: &Self) -> bool {
        self.p_data == other.p_data
            && self.row_pitch == other.row_pitch
            && self.depth_pitch == other.depth_pitch
    }
}
impl ::std::cmp::Eq for D3D11_MAPPED_SUBRESOURCE {}
unsafe impl ::windows::Abi for D3D11_MAPPED_SUBRESOURCE {
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
pub struct D3D_PRIMITIVE_TOPOLOGY(pub i32);
impl D3D_PRIMITIVE_TOPOLOGY {
    #![allow(non_upper_case_globals)]
    pub const D3D_PRIMITIVE_TOPOLOGY_UNDEFINED: Self = Self(0i32);
    pub const D3D_PRIMITIVE_TOPOLOGY_POINTLIST: Self = Self(1i32);
    pub const D3D_PRIMITIVE_TOPOLOGY_LINELIST: Self = Self(2i32);
    pub const D3D_PRIMITIVE_TOPOLOGY_LINESTRIP: Self = Self(3i32);
    pub const D3D_PRIMITIVE_TOPOLOGY_TRIANGLELIST: Self = Self(4i32);
    pub const D3D_PRIMITIVE_TOPOLOGY_TRIANGLESTRIP: Self = Self(5i32);
    pub const D3D_PRIMITIVE_TOPOLOGY_LINELIST_ADJ: Self = Self(10i32);
    pub const D3D_PRIMITIVE_TOPOLOGY_LINESTRIP_ADJ: Self = Self(11i32);
    pub const D3D_PRIMITIVE_TOPOLOGY_TRIANGLELIST_ADJ: Self = Self(12i32);
    pub const D3D_PRIMITIVE_TOPOLOGY_TRIANGLESTRIP_ADJ: Self = Self(13i32);
    pub const D3D_PRIMITIVE_TOPOLOGY_1_CONTROL_POINT_PATCHLIST: Self = Self(33i32);
    pub const D3D_PRIMITIVE_TOPOLOGY_2_CONTROL_POINT_PATCHLIST: Self = Self(34i32);
    pub const D3D_PRIMITIVE_TOPOLOGY_3_CONTROL_POINT_PATCHLIST: Self = Self(35i32);
    pub const D3D_PRIMITIVE_TOPOLOGY_4_CONTROL_POINT_PATCHLIST: Self = Self(36i32);
    pub const D3D_PRIMITIVE_TOPOLOGY_5_CONTROL_POINT_PATCHLIST: Self = Self(37i32);
    pub const D3D_PRIMITIVE_TOPOLOGY_6_CONTROL_POINT_PATCHLIST: Self = Self(38i32);
    pub const D3D_PRIMITIVE_TOPOLOGY_7_CONTROL_POINT_PATCHLIST: Self = Self(39i32);
    pub const D3D_PRIMITIVE_TOPOLOGY_8_CONTROL_POINT_PATCHLIST: Self = Self(40i32);
    pub const D3D_PRIMITIVE_TOPOLOGY_9_CONTROL_POINT_PATCHLIST: Self = Self(41i32);
    pub const D3D_PRIMITIVE_TOPOLOGY_10_CONTROL_POINT_PATCHLIST: Self = Self(42i32);
    pub const D3D_PRIMITIVE_TOPOLOGY_11_CONTROL_POINT_PATCHLIST: Self = Self(43i32);
    pub const D3D_PRIMITIVE_TOPOLOGY_12_CONTROL_POINT_PATCHLIST: Self = Self(44i32);
    pub const D3D_PRIMITIVE_TOPOLOGY_13_CONTROL_POINT_PATCHLIST: Self = Self(45i32);
    pub const D3D_PRIMITIVE_TOPOLOGY_14_CONTROL_POINT_PATCHLIST: Self = Self(46i32);
    pub const D3D_PRIMITIVE_TOPOLOGY_15_CONTROL_POINT_PATCHLIST: Self = Self(47i32);
    pub const D3D_PRIMITIVE_TOPOLOGY_16_CONTROL_POINT_PATCHLIST: Self = Self(48i32);
    pub const D3D_PRIMITIVE_TOPOLOGY_17_CONTROL_POINT_PATCHLIST: Self = Self(49i32);
    pub const D3D_PRIMITIVE_TOPOLOGY_18_CONTROL_POINT_PATCHLIST: Self = Self(50i32);
    pub const D3D_PRIMITIVE_TOPOLOGY_19_CONTROL_POINT_PATCHLIST: Self = Self(51i32);
    pub const D3D_PRIMITIVE_TOPOLOGY_20_CONTROL_POINT_PATCHLIST: Self = Self(52i32);
    pub const D3D_PRIMITIVE_TOPOLOGY_21_CONTROL_POINT_PATCHLIST: Self = Self(53i32);
    pub const D3D_PRIMITIVE_TOPOLOGY_22_CONTROL_POINT_PATCHLIST: Self = Self(54i32);
    pub const D3D_PRIMITIVE_TOPOLOGY_23_CONTROL_POINT_PATCHLIST: Self = Self(55i32);
    pub const D3D_PRIMITIVE_TOPOLOGY_24_CONTROL_POINT_PATCHLIST: Self = Self(56i32);
    pub const D3D_PRIMITIVE_TOPOLOGY_25_CONTROL_POINT_PATCHLIST: Self = Self(57i32);
    pub const D3D_PRIMITIVE_TOPOLOGY_26_CONTROL_POINT_PATCHLIST: Self = Self(58i32);
    pub const D3D_PRIMITIVE_TOPOLOGY_27_CONTROL_POINT_PATCHLIST: Self = Self(59i32);
    pub const D3D_PRIMITIVE_TOPOLOGY_28_CONTROL_POINT_PATCHLIST: Self = Self(60i32);
    pub const D3D_PRIMITIVE_TOPOLOGY_29_CONTROL_POINT_PATCHLIST: Self = Self(61i32);
    pub const D3D_PRIMITIVE_TOPOLOGY_30_CONTROL_POINT_PATCHLIST: Self = Self(62i32);
    pub const D3D_PRIMITIVE_TOPOLOGY_31_CONTROL_POINT_PATCHLIST: Self = Self(63i32);
    pub const D3D_PRIMITIVE_TOPOLOGY_32_CONTROL_POINT_PATCHLIST: Self = Self(64i32);
    pub const D3D10_PRIMITIVE_TOPOLOGY_UNDEFINED: Self = Self(0i32);
    pub const D3D10_PRIMITIVE_TOPOLOGY_POINTLIST: Self = Self(1i32);
    pub const D3D10_PRIMITIVE_TOPOLOGY_LINELIST: Self = Self(2i32);
    pub const D3D10_PRIMITIVE_TOPOLOGY_LINESTRIP: Self = Self(3i32);
    pub const D3D10_PRIMITIVE_TOPOLOGY_TRIANGLELIST: Self = Self(4i32);
    pub const D3D10_PRIMITIVE_TOPOLOGY_TRIANGLESTRIP: Self = Self(5i32);
    pub const D3D10_PRIMITIVE_TOPOLOGY_LINELIST_ADJ: Self = Self(10i32);
    pub const D3D10_PRIMITIVE_TOPOLOGY_LINESTRIP_ADJ: Self = Self(11i32);
    pub const D3D10_PRIMITIVE_TOPOLOGY_TRIANGLELIST_ADJ: Self = Self(12i32);
    pub const D3D10_PRIMITIVE_TOPOLOGY_TRIANGLESTRIP_ADJ: Self = Self(13i32);
    pub const D3D11_PRIMITIVE_TOPOLOGY_UNDEFINED: Self = Self(0i32);
    pub const D3D11_PRIMITIVE_TOPOLOGY_POINTLIST: Self = Self(1i32);
    pub const D3D11_PRIMITIVE_TOPOLOGY_LINELIST: Self = Self(2i32);
    pub const D3D11_PRIMITIVE_TOPOLOGY_LINESTRIP: Self = Self(3i32);
    pub const D3D11_PRIMITIVE_TOPOLOGY_TRIANGLELIST: Self = Self(4i32);
    pub const D3D11_PRIMITIVE_TOPOLOGY_TRIANGLESTRIP: Self = Self(5i32);
    pub const D3D11_PRIMITIVE_TOPOLOGY_LINELIST_ADJ: Self = Self(10i32);
    pub const D3D11_PRIMITIVE_TOPOLOGY_LINESTRIP_ADJ: Self = Self(11i32);
    pub const D3D11_PRIMITIVE_TOPOLOGY_TRIANGLELIST_ADJ: Self = Self(12i32);
    pub const D3D11_PRIMITIVE_TOPOLOGY_TRIANGLESTRIP_ADJ: Self = Self(13i32);
    pub const D3D11_PRIMITIVE_TOPOLOGY_1_CONTROL_POINT_PATCHLIST: Self = Self(33i32);
    pub const D3D11_PRIMITIVE_TOPOLOGY_2_CONTROL_POINT_PATCHLIST: Self = Self(34i32);
    pub const D3D11_PRIMITIVE_TOPOLOGY_3_CONTROL_POINT_PATCHLIST: Self = Self(35i32);
    pub const D3D11_PRIMITIVE_TOPOLOGY_4_CONTROL_POINT_PATCHLIST: Self = Self(36i32);
    pub const D3D11_PRIMITIVE_TOPOLOGY_5_CONTROL_POINT_PATCHLIST: Self = Self(37i32);
    pub const D3D11_PRIMITIVE_TOPOLOGY_6_CONTROL_POINT_PATCHLIST: Self = Self(38i32);
    pub const D3D11_PRIMITIVE_TOPOLOGY_7_CONTROL_POINT_PATCHLIST: Self = Self(39i32);
    pub const D3D11_PRIMITIVE_TOPOLOGY_8_CONTROL_POINT_PATCHLIST: Self = Self(40i32);
    pub const D3D11_PRIMITIVE_TOPOLOGY_9_CONTROL_POINT_PATCHLIST: Self = Self(41i32);
    pub const D3D11_PRIMITIVE_TOPOLOGY_10_CONTROL_POINT_PATCHLIST: Self = Self(42i32);
    pub const D3D11_PRIMITIVE_TOPOLOGY_11_CONTROL_POINT_PATCHLIST: Self = Self(43i32);
    pub const D3D11_PRIMITIVE_TOPOLOGY_12_CONTROL_POINT_PATCHLIST: Self = Self(44i32);
    pub const D3D11_PRIMITIVE_TOPOLOGY_13_CONTROL_POINT_PATCHLIST: Self = Self(45i32);
    pub const D3D11_PRIMITIVE_TOPOLOGY_14_CONTROL_POINT_PATCHLIST: Self = Self(46i32);
    pub const D3D11_PRIMITIVE_TOPOLOGY_15_CONTROL_POINT_PATCHLIST: Self = Self(47i32);
    pub const D3D11_PRIMITIVE_TOPOLOGY_16_CONTROL_POINT_PATCHLIST: Self = Self(48i32);
    pub const D3D11_PRIMITIVE_TOPOLOGY_17_CONTROL_POINT_PATCHLIST: Self = Self(49i32);
    pub const D3D11_PRIMITIVE_TOPOLOGY_18_CONTROL_POINT_PATCHLIST: Self = Self(50i32);
    pub const D3D11_PRIMITIVE_TOPOLOGY_19_CONTROL_POINT_PATCHLIST: Self = Self(51i32);
    pub const D3D11_PRIMITIVE_TOPOLOGY_20_CONTROL_POINT_PATCHLIST: Self = Self(52i32);
    pub const D3D11_PRIMITIVE_TOPOLOGY_21_CONTROL_POINT_PATCHLIST: Self = Self(53i32);
    pub const D3D11_PRIMITIVE_TOPOLOGY_22_CONTROL_POINT_PATCHLIST: Self = Self(54i32);
    pub const D3D11_PRIMITIVE_TOPOLOGY_23_CONTROL_POINT_PATCHLIST: Self = Self(55i32);
    pub const D3D11_PRIMITIVE_TOPOLOGY_24_CONTROL_POINT_PATCHLIST: Self = Self(56i32);
    pub const D3D11_PRIMITIVE_TOPOLOGY_25_CONTROL_POINT_PATCHLIST: Self = Self(57i32);
    pub const D3D11_PRIMITIVE_TOPOLOGY_26_CONTROL_POINT_PATCHLIST: Self = Self(58i32);
    pub const D3D11_PRIMITIVE_TOPOLOGY_27_CONTROL_POINT_PATCHLIST: Self = Self(59i32);
    pub const D3D11_PRIMITIVE_TOPOLOGY_28_CONTROL_POINT_PATCHLIST: Self = Self(60i32);
    pub const D3D11_PRIMITIVE_TOPOLOGY_29_CONTROL_POINT_PATCHLIST: Self = Self(61i32);
    pub const D3D11_PRIMITIVE_TOPOLOGY_30_CONTROL_POINT_PATCHLIST: Self = Self(62i32);
    pub const D3D11_PRIMITIVE_TOPOLOGY_31_CONTROL_POINT_PATCHLIST: Self = Self(63i32);
    pub const D3D11_PRIMITIVE_TOPOLOGY_32_CONTROL_POINT_PATCHLIST: Self = Self(64i32);
}
impl ::std::convert::From<i32> for D3D_PRIMITIVE_TOPOLOGY {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::Abi for D3D_PRIMITIVE_TOPOLOGY {
    type Abi = Self;
}
#[repr(C)]
#[allow(non_snake_case)]
#[derive(:: std :: clone :: Clone)]
pub struct D3D11_VIEWPORT {
    pub top_leftx: f32,
    pub top_lefty: f32,
    pub width: f32,
    pub height: f32,
    pub min_depth: f32,
    pub max_depth: f32,
}
impl D3D11_VIEWPORT {}
impl ::std::default::Default for D3D11_VIEWPORT {
    fn default() -> Self {
        Self {
            top_leftx: 0.0,
            top_lefty: 0.0,
            width: 0.0,
            height: 0.0,
            min_depth: 0.0,
            max_depth: 0.0,
        }
    }
}
impl ::std::fmt::Debug for D3D11_VIEWPORT {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("D3D11_VIEWPORT")
            .field("top_leftx", &format_args!("{:?}", self.top_leftx))
            .field("top_lefty", &format_args!("{:?}", self.top_lefty))
            .field("width", &format_args!("{:?}", self.width))
            .field("height", &format_args!("{:?}", self.height))
            .field("min_depth", &format_args!("{:?}", self.min_depth))
            .field("max_depth", &format_args!("{:?}", self.max_depth))
            .finish()
    }
}
impl ::std::cmp::PartialEq for D3D11_VIEWPORT {
    fn eq(&self, other: &Self) -> bool {
        self.top_leftx == other.top_leftx
            && self.top_lefty == other.top_lefty
            && self.width == other.width
            && self.height == other.height
            && self.min_depth == other.min_depth
            && self.max_depth == other.max_depth
    }
}
impl ::std::cmp::Eq for D3D11_VIEWPORT {}
unsafe impl ::windows::Abi for D3D11_VIEWPORT {
    type Abi = Self;
}
#[repr(C)]
#[allow(non_snake_case)]
#[derive(:: std :: clone :: Clone)]
pub struct D3D11_BOX {
    pub left: u32,
    pub top: u32,
    pub front: u32,
    pub right: u32,
    pub bottom: u32,
    pub back: u32,
}
impl D3D11_BOX {}
impl ::std::default::Default for D3D11_BOX {
    fn default() -> Self {
        Self {
            left: 0,
            top: 0,
            front: 0,
            right: 0,
            bottom: 0,
            back: 0,
        }
    }
}
impl ::std::fmt::Debug for D3D11_BOX {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("D3D11_BOX")
            .field("left", &format_args!("{:?}", self.left))
            .field("top", &format_args!("{:?}", self.top))
            .field("front", &format_args!("{:?}", self.front))
            .field("right", &format_args!("{:?}", self.right))
            .field("bottom", &format_args!("{:?}", self.bottom))
            .field("back", &format_args!("{:?}", self.back))
            .finish()
    }
}
impl ::std::cmp::PartialEq for D3D11_BOX {
    fn eq(&self, other: &Self) -> bool {
        self.left == other.left
            && self.top == other.top
            && self.front == other.front
            && self.right == other.right
            && self.bottom == other.bottom
            && self.back == other.back
    }
}
impl ::std::cmp::Eq for D3D11_BOX {}
unsafe impl ::windows::Abi for D3D11_BOX {
    type Abi = Self;
}
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct ID3D11CommandList(::windows::IUnknown);
impl ID3D11CommandList {}
unsafe impl ::windows::Interface for ID3D11CommandList {
    type Vtable = ID3D11CommandList_abi;
    const IID: ::windows::Guid = ::windows::Guid::from_values(
        2722874577,
        30366,
        17399,
        [128, 19, 152, 255, 86, 108, 24, 226],
    );
}
#[allow(non_snake_case)]
impl ID3D11CommandList {
    pub unsafe fn GetDevice(&self, pp_device: *mut ::std::option::Option<ID3D11Device>) {
        (::windows::Interface::vtable(self).3)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(pp_device),
        )
    }
    pub unsafe fn GetPrivateData(
        &self,
        guid: *const ::windows::Guid,
        p_data_size: *mut u32,
        p_data: *mut ::std::ffi::c_void,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).4)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(guid),
            ::std::mem::transmute(p_data_size),
            ::std::mem::transmute(p_data),
        )
    }
    pub unsafe fn SetPrivateData(
        &self,
        guid: *const ::windows::Guid,
        data_size: u32,
        p_data: *const ::std::ffi::c_void,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).5)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(guid),
            ::std::mem::transmute(data_size),
            ::std::mem::transmute(p_data),
        )
    }
    pub unsafe fn SetPrivateDataInterface<
        'a,
        T1__: ::windows::IntoParam<'a, ::windows::IUnknown>,
    >(
        &self,
        guid: *const ::windows::Guid,
        p_data: T1__,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).6)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(guid),
            p_data.into_param().abi(),
        )
    }
    pub unsafe fn GetContextFlags(&self) -> u32 {
        (::windows::Interface::vtable(self).7)(::windows::Abi::abi(self))
    }
}
impl ::std::convert::From<ID3D11CommandList> for ::windows::IUnknown {
    fn from(value: ID3D11CommandList) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&ID3D11CommandList> for ::windows::IUnknown {
    fn from(value: &ID3D11CommandList) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::IntoParam<'a, ::windows::IUnknown> for ID3D11CommandList {
    fn into_param(self) -> ::windows::Param<'a, ::windows::IUnknown> {
        ::windows::Param::Owned(::std::convert::Into::<::windows::IUnknown>::into(self))
    }
}
impl<'a> ::windows::IntoParam<'a, ::windows::IUnknown> for &'a ID3D11CommandList {
    fn into_param(self) -> ::windows::Param<'a, ::windows::IUnknown> {
        ::windows::Param::Owned(::std::convert::Into::<::windows::IUnknown>::into(
            ::std::clone::Clone::clone(self),
        ))
    }
}
impl ::std::convert::From<ID3D11CommandList> for ID3D11DeviceChild {
    fn from(value: ID3D11CommandList) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&ID3D11CommandList> for ID3D11DeviceChild {
    fn from(value: &ID3D11CommandList) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::IntoParam<'a, ID3D11DeviceChild> for ID3D11CommandList {
    fn into_param(self) -> ::windows::Param<'a, ID3D11DeviceChild> {
        ::windows::Param::Owned(::std::convert::Into::<ID3D11DeviceChild>::into(self))
    }
}
impl<'a> ::windows::IntoParam<'a, ID3D11DeviceChild> for &'a ID3D11CommandList {
    fn into_param(self) -> ::windows::Param<'a, ID3D11DeviceChild> {
        ::windows::Param::Owned(::std::convert::Into::<ID3D11DeviceChild>::into(
            ::std::clone::Clone::clone(self),
        ))
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ID3D11CommandList_abi(
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        iid: &::windows::Guid,
        interface: *mut ::windows::RawPtr,
    ) -> ::windows::ErrorCode,
    pub unsafe extern "system" fn(this: ::windows::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::RawPtr, pp_device: *mut ::windows::RawPtr),
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        guid: *const ::windows::Guid,
        p_data_size: *mut u32,
        p_data: *mut ::std::ffi::c_void,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        guid: *const ::windows::Guid,
        data_size: u32,
        p_data: *const ::std::ffi::c_void,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        guid: *const ::windows::Guid,
        p_data: ::windows::RawPtr,
    ) -> ::windows::ErrorCode,
    pub unsafe extern "system" fn(this: ::windows::RawPtr) -> u32,
);
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
pub struct D3D11_DEVICE_CONTEXT_TYPE(pub i32);
impl D3D11_DEVICE_CONTEXT_TYPE {
    #![allow(non_upper_case_globals)]
    pub const D3D11_DEVICE_CONTEXT_IMMEDIATE: Self = Self(0i32);
    pub const D3D11_DEVICE_CONTEXT_DEFERRED: Self = Self(1i32);
}
impl ::std::convert::From<i32> for D3D11_DEVICE_CONTEXT_TYPE {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::Abi for D3D11_DEVICE_CONTEXT_TYPE {
    type Abi = Self;
}
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct ID3D11DeviceContext(::windows::IUnknown);
impl ID3D11DeviceContext {}
unsafe impl ::windows::Interface for ID3D11DeviceContext {
    type Vtable = ID3D11DeviceContext_abi;
    const IID: ::windows::Guid = ::windows::Guid::from_values(
        3233786220,
        57481,
        17659,
        [142, 175, 38, 248, 121, 97, 144, 218],
    );
}
#[allow(non_snake_case)]
impl ID3D11DeviceContext {
    pub unsafe fn GetDevice(&self, pp_device: *mut ::std::option::Option<ID3D11Device>) {
        (::windows::Interface::vtable(self).3)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(pp_device),
        )
    }
    pub unsafe fn GetPrivateData(
        &self,
        guid: *const ::windows::Guid,
        p_data_size: *mut u32,
        p_data: *mut ::std::ffi::c_void,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).4)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(guid),
            ::std::mem::transmute(p_data_size),
            ::std::mem::transmute(p_data),
        )
    }
    pub unsafe fn SetPrivateData(
        &self,
        guid: *const ::windows::Guid,
        data_size: u32,
        p_data: *const ::std::ffi::c_void,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).5)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(guid),
            ::std::mem::transmute(data_size),
            ::std::mem::transmute(p_data),
        )
    }
    pub unsafe fn SetPrivateDataInterface<
        'a,
        T1__: ::windows::IntoParam<'a, ::windows::IUnknown>,
    >(
        &self,
        guid: *const ::windows::Guid,
        p_data: T1__,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).6)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(guid),
            p_data.into_param().abi(),
        )
    }
    pub unsafe fn VSSetConstantBuffers(
        &self,
        start_slot: u32,
        num_buffers: u32,
        pp_constant_buffers: *mut ::std::option::Option<ID3D11Buffer>,
    ) {
        (::windows::Interface::vtable(self).7)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(start_slot),
            ::std::mem::transmute(num_buffers),
            ::std::mem::transmute(pp_constant_buffers),
        )
    }
    pub unsafe fn PSSetShaderResources(
        &self,
        start_slot: u32,
        num_views: u32,
        pp_shader_resource_views: *mut ::std::option::Option<ID3D11ShaderResourceView>,
    ) {
        (::windows::Interface::vtable(self).8)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(start_slot),
            ::std::mem::transmute(num_views),
            ::std::mem::transmute(pp_shader_resource_views),
        )
    }
    pub unsafe fn PSSetShader<'a, T0__: ::windows::IntoParam<'a, ID3D11PixelShader>>(
        &self,
        p_pixel_shader: T0__,
        pp_class_instances: *mut ::std::option::Option<ID3D11ClassInstance>,
        num_class_instances: u32,
    ) {
        (::windows::Interface::vtable(self).9)(
            ::windows::Abi::abi(self),
            p_pixel_shader.into_param().abi(),
            ::std::mem::transmute(pp_class_instances),
            ::std::mem::transmute(num_class_instances),
        )
    }
    pub unsafe fn PSSetSamplers(
        &self,
        start_slot: u32,
        num_samplers: u32,
        pp_samplers: *mut ::std::option::Option<ID3D11SamplerState>,
    ) {
        (::windows::Interface::vtable(self).10)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(start_slot),
            ::std::mem::transmute(num_samplers),
            ::std::mem::transmute(pp_samplers),
        )
    }
    pub unsafe fn VSSetShader<'a, T0__: ::windows::IntoParam<'a, ID3D11VertexShader>>(
        &self,
        p_vertex_shader: T0__,
        pp_class_instances: *mut ::std::option::Option<ID3D11ClassInstance>,
        num_class_instances: u32,
    ) {
        (::windows::Interface::vtable(self).11)(
            ::windows::Abi::abi(self),
            p_vertex_shader.into_param().abi(),
            ::std::mem::transmute(pp_class_instances),
            ::std::mem::transmute(num_class_instances),
        )
    }
    pub unsafe fn DrawIndexed(
        &self,
        index_count: u32,
        start_index_location: u32,
        base_vertex_location: i32,
    ) {
        (::windows::Interface::vtable(self).12)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(index_count),
            ::std::mem::transmute(start_index_location),
            ::std::mem::transmute(base_vertex_location),
        )
    }
    pub unsafe fn Draw(&self, vertex_count: u32, start_vertex_location: u32) {
        (::windows::Interface::vtable(self).13)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(vertex_count),
            ::std::mem::transmute(start_vertex_location),
        )
    }
    pub unsafe fn Map<'a, T0__: ::windows::IntoParam<'a, ID3D11Resource>>(
        &self,
        p_resource: T0__,
        subresource: u32,
        map_type: D3D11_MAP,
        map_flags: u32,
        p_mapped_resource: *mut D3D11_MAPPED_SUBRESOURCE,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).14)(
            ::windows::Abi::abi(self),
            p_resource.into_param().abi(),
            ::std::mem::transmute(subresource),
            ::std::mem::transmute(map_type),
            ::std::mem::transmute(map_flags),
            ::std::mem::transmute(p_mapped_resource),
        )
    }
    pub unsafe fn Unmap<'a, T0__: ::windows::IntoParam<'a, ID3D11Resource>>(
        &self,
        p_resource: T0__,
        subresource: u32,
    ) {
        (::windows::Interface::vtable(self).15)(
            ::windows::Abi::abi(self),
            p_resource.into_param().abi(),
            ::std::mem::transmute(subresource),
        )
    }
    pub unsafe fn PSSetConstantBuffers(
        &self,
        start_slot: u32,
        num_buffers: u32,
        pp_constant_buffers: *mut ::std::option::Option<ID3D11Buffer>,
    ) {
        (::windows::Interface::vtable(self).16)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(start_slot),
            ::std::mem::transmute(num_buffers),
            ::std::mem::transmute(pp_constant_buffers),
        )
    }
    pub unsafe fn IASetInputLayout<'a, T0__: ::windows::IntoParam<'a, ID3D11InputLayout>>(
        &self,
        p_input_layout: T0__,
    ) {
        (::windows::Interface::vtable(self).17)(
            ::windows::Abi::abi(self),
            p_input_layout.into_param().abi(),
        )
    }
    pub unsafe fn IASetVertexBuffers(
        &self,
        start_slot: u32,
        num_buffers: u32,
        pp_vertex_buffers: *mut ::std::option::Option<ID3D11Buffer>,
        p_strides: *const u32,
        p_offsets: *const u32,
    ) {
        (::windows::Interface::vtable(self).18)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(start_slot),
            ::std::mem::transmute(num_buffers),
            ::std::mem::transmute(pp_vertex_buffers),
            ::std::mem::transmute(p_strides),
            ::std::mem::transmute(p_offsets),
        )
    }
    pub unsafe fn IASetIndexBuffer<'a, T0__: ::windows::IntoParam<'a, ID3D11Buffer>>(
        &self,
        p_index_buffer: T0__,
        format: super::dxgi::DXGI_FORMAT,
        offset: u32,
    ) {
        (::windows::Interface::vtable(self).19)(
            ::windows::Abi::abi(self),
            p_index_buffer.into_param().abi(),
            ::std::mem::transmute(format),
            ::std::mem::transmute(offset),
        )
    }
    pub unsafe fn DrawIndexedInstanced(
        &self,
        index_count_per_instance: u32,
        instance_count: u32,
        start_index_location: u32,
        base_vertex_location: i32,
        start_instance_location: u32,
    ) {
        (::windows::Interface::vtable(self).20)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(index_count_per_instance),
            ::std::mem::transmute(instance_count),
            ::std::mem::transmute(start_index_location),
            ::std::mem::transmute(base_vertex_location),
            ::std::mem::transmute(start_instance_location),
        )
    }
    pub unsafe fn DrawInstanced(
        &self,
        vertex_count_per_instance: u32,
        instance_count: u32,
        start_vertex_location: u32,
        start_instance_location: u32,
    ) {
        (::windows::Interface::vtable(self).21)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(vertex_count_per_instance),
            ::std::mem::transmute(instance_count),
            ::std::mem::transmute(start_vertex_location),
            ::std::mem::transmute(start_instance_location),
        )
    }
    pub unsafe fn GSSetConstantBuffers(
        &self,
        start_slot: u32,
        num_buffers: u32,
        pp_constant_buffers: *mut ::std::option::Option<ID3D11Buffer>,
    ) {
        (::windows::Interface::vtable(self).22)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(start_slot),
            ::std::mem::transmute(num_buffers),
            ::std::mem::transmute(pp_constant_buffers),
        )
    }
    pub unsafe fn GSSetShader<'a, T0__: ::windows::IntoParam<'a, ID3D11GeometryShader>>(
        &self,
        p_shader: T0__,
        pp_class_instances: *mut ::std::option::Option<ID3D11ClassInstance>,
        num_class_instances: u32,
    ) {
        (::windows::Interface::vtable(self).23)(
            ::windows::Abi::abi(self),
            p_shader.into_param().abi(),
            ::std::mem::transmute(pp_class_instances),
            ::std::mem::transmute(num_class_instances),
        )
    }
    pub unsafe fn IASetPrimitiveTopology(&self, topology: D3D_PRIMITIVE_TOPOLOGY) {
        (::windows::Interface::vtable(self).24)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(topology),
        )
    }
    pub unsafe fn VSSetShaderResources(
        &self,
        start_slot: u32,
        num_views: u32,
        pp_shader_resource_views: *mut ::std::option::Option<ID3D11ShaderResourceView>,
    ) {
        (::windows::Interface::vtable(self).25)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(start_slot),
            ::std::mem::transmute(num_views),
            ::std::mem::transmute(pp_shader_resource_views),
        )
    }
    pub unsafe fn VSSetSamplers(
        &self,
        start_slot: u32,
        num_samplers: u32,
        pp_samplers: *mut ::std::option::Option<ID3D11SamplerState>,
    ) {
        (::windows::Interface::vtable(self).26)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(start_slot),
            ::std::mem::transmute(num_samplers),
            ::std::mem::transmute(pp_samplers),
        )
    }
    pub unsafe fn Begin<'a, T0__: ::windows::IntoParam<'a, ID3D11Asynchronous>>(
        &self,
        p_async: T0__,
    ) {
        (::windows::Interface::vtable(self).27)(
            ::windows::Abi::abi(self),
            p_async.into_param().abi(),
        )
    }
    pub unsafe fn End<'a, T0__: ::windows::IntoParam<'a, ID3D11Asynchronous>>(
        &self,
        p_async: T0__,
    ) {
        (::windows::Interface::vtable(self).28)(
            ::windows::Abi::abi(self),
            p_async.into_param().abi(),
        )
    }
    pub unsafe fn GetData<'a, T0__: ::windows::IntoParam<'a, ID3D11Asynchronous>>(
        &self,
        p_async: T0__,
        p_data: *mut ::std::ffi::c_void,
        data_size: u32,
        get_data_flags: u32,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).29)(
            ::windows::Abi::abi(self),
            p_async.into_param().abi(),
            ::std::mem::transmute(p_data),
            ::std::mem::transmute(data_size),
            ::std::mem::transmute(get_data_flags),
        )
    }
    pub unsafe fn SetPredication<
        'a,
        T0__: ::windows::IntoParam<'a, ID3D11Predicate>,
        T1__: ::windows::IntoParam<'a, super::system_services::BOOL>,
    >(
        &self,
        p_predicate: T0__,
        predicate_value: T1__,
    ) {
        (::windows::Interface::vtable(self).30)(
            ::windows::Abi::abi(self),
            p_predicate.into_param().abi(),
            predicate_value.into_param().abi(),
        )
    }
    pub unsafe fn GSSetShaderResources(
        &self,
        start_slot: u32,
        num_views: u32,
        pp_shader_resource_views: *mut ::std::option::Option<ID3D11ShaderResourceView>,
    ) {
        (::windows::Interface::vtable(self).31)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(start_slot),
            ::std::mem::transmute(num_views),
            ::std::mem::transmute(pp_shader_resource_views),
        )
    }
    pub unsafe fn GSSetSamplers(
        &self,
        start_slot: u32,
        num_samplers: u32,
        pp_samplers: *mut ::std::option::Option<ID3D11SamplerState>,
    ) {
        (::windows::Interface::vtable(self).32)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(start_slot),
            ::std::mem::transmute(num_samplers),
            ::std::mem::transmute(pp_samplers),
        )
    }
    pub unsafe fn OMSetRenderTargets<'a, T2__: ::windows::IntoParam<'a, ID3D11DepthStencilView>>(
        &self,
        num_views: u32,
        pp_render_target_views: *mut ::std::option::Option<ID3D11RenderTargetView>,
        p_depth_stencil_view: T2__,
    ) {
        (::windows::Interface::vtable(self).33)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(num_views),
            ::std::mem::transmute(pp_render_target_views),
            p_depth_stencil_view.into_param().abi(),
        )
    }
    pub unsafe fn OMSetRenderTargetsAndUnorderedAccessViews<
        'a,
        T2__: ::windows::IntoParam<'a, ID3D11DepthStencilView>,
    >(
        &self,
        num_rt_vs: u32,
        pp_render_target_views: *mut ::std::option::Option<ID3D11RenderTargetView>,
        p_depth_stencil_view: T2__,
        uav_start_slot: u32,
        num_ua_vs: u32,
        pp_unordered_access_views: *mut ::std::option::Option<ID3D11UnorderedAccessView>,
        p_uav_initial_counts: *const u32,
    ) {
        (::windows::Interface::vtable(self).34)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(num_rt_vs),
            ::std::mem::transmute(pp_render_target_views),
            p_depth_stencil_view.into_param().abi(),
            ::std::mem::transmute(uav_start_slot),
            ::std::mem::transmute(num_ua_vs),
            ::std::mem::transmute(pp_unordered_access_views),
            ::std::mem::transmute(p_uav_initial_counts),
        )
    }
    pub unsafe fn OMSetBlendState<'a, T0__: ::windows::IntoParam<'a, ID3D11BlendState>>(
        &self,
        p_blend_state: T0__,
        blend_factor: *const f32,
        sample_mask: u32,
    ) {
        (::windows::Interface::vtable(self).35)(
            ::windows::Abi::abi(self),
            p_blend_state.into_param().abi(),
            ::std::mem::transmute(blend_factor),
            ::std::mem::transmute(sample_mask),
        )
    }
    pub unsafe fn OMSetDepthStencilState<
        'a,
        T0__: ::windows::IntoParam<'a, ID3D11DepthStencilState>,
    >(
        &self,
        p_depth_stencil_state: T0__,
        stencil_ref: u32,
    ) {
        (::windows::Interface::vtable(self).36)(
            ::windows::Abi::abi(self),
            p_depth_stencil_state.into_param().abi(),
            ::std::mem::transmute(stencil_ref),
        )
    }
    pub unsafe fn SOSetTargets(
        &self,
        num_buffers: u32,
        pp_so_targets: *mut ::std::option::Option<ID3D11Buffer>,
        p_offsets: *const u32,
    ) {
        (::windows::Interface::vtable(self).37)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(num_buffers),
            ::std::mem::transmute(pp_so_targets),
            ::std::mem::transmute(p_offsets),
        )
    }
    pub unsafe fn DrawAuto(&self) {
        (::windows::Interface::vtable(self).38)(::windows::Abi::abi(self))
    }
    pub unsafe fn DrawIndexedInstancedIndirect<'a, T0__: ::windows::IntoParam<'a, ID3D11Buffer>>(
        &self,
        p_buffer_for_args: T0__,
        aligned_byte_offset_for_args: u32,
    ) {
        (::windows::Interface::vtable(self).39)(
            ::windows::Abi::abi(self),
            p_buffer_for_args.into_param().abi(),
            ::std::mem::transmute(aligned_byte_offset_for_args),
        )
    }
    pub unsafe fn DrawInstancedIndirect<'a, T0__: ::windows::IntoParam<'a, ID3D11Buffer>>(
        &self,
        p_buffer_for_args: T0__,
        aligned_byte_offset_for_args: u32,
    ) {
        (::windows::Interface::vtable(self).40)(
            ::windows::Abi::abi(self),
            p_buffer_for_args.into_param().abi(),
            ::std::mem::transmute(aligned_byte_offset_for_args),
        )
    }
    pub unsafe fn Dispatch(
        &self,
        thread_group_countx: u32,
        thread_group_county: u32,
        thread_group_countz: u32,
    ) {
        (::windows::Interface::vtable(self).41)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(thread_group_countx),
            ::std::mem::transmute(thread_group_county),
            ::std::mem::transmute(thread_group_countz),
        )
    }
    pub unsafe fn DispatchIndirect<'a, T0__: ::windows::IntoParam<'a, ID3D11Buffer>>(
        &self,
        p_buffer_for_args: T0__,
        aligned_byte_offset_for_args: u32,
    ) {
        (::windows::Interface::vtable(self).42)(
            ::windows::Abi::abi(self),
            p_buffer_for_args.into_param().abi(),
            ::std::mem::transmute(aligned_byte_offset_for_args),
        )
    }
    pub unsafe fn RSSetState<'a, T0__: ::windows::IntoParam<'a, ID3D11RasterizerState>>(
        &self,
        p_rasterizer_state: T0__,
    ) {
        (::windows::Interface::vtable(self).43)(
            ::windows::Abi::abi(self),
            p_rasterizer_state.into_param().abi(),
        )
    }
    pub unsafe fn RSSetViewports(&self, num_viewports: u32, p_viewports: *const D3D11_VIEWPORT) {
        (::windows::Interface::vtable(self).44)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(num_viewports),
            ::std::mem::transmute(p_viewports),
        )
    }
    pub unsafe fn RSSetScissorRects(
        &self,
        num_rects: u32,
        p_rects: *const super::display_devices::RECT,
    ) {
        (::windows::Interface::vtable(self).45)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(num_rects),
            ::std::mem::transmute(p_rects),
        )
    }
    pub unsafe fn CopySubresourceRegion<
        'a,
        T0__: ::windows::IntoParam<'a, ID3D11Resource>,
        T5__: ::windows::IntoParam<'a, ID3D11Resource>,
    >(
        &self,
        p_dst_resource: T0__,
        dst_subresource: u32,
        dstx: u32,
        dsty: u32,
        dstz: u32,
        p_src_resource: T5__,
        src_subresource: u32,
        p_src_box: *const D3D11_BOX,
    ) {
        (::windows::Interface::vtable(self).46)(
            ::windows::Abi::abi(self),
            p_dst_resource.into_param().abi(),
            ::std::mem::transmute(dst_subresource),
            ::std::mem::transmute(dstx),
            ::std::mem::transmute(dsty),
            ::std::mem::transmute(dstz),
            p_src_resource.into_param().abi(),
            ::std::mem::transmute(src_subresource),
            ::std::mem::transmute(p_src_box),
        )
    }
    pub unsafe fn CopyResource<
        'a,
        T0__: ::windows::IntoParam<'a, ID3D11Resource>,
        T1__: ::windows::IntoParam<'a, ID3D11Resource>,
    >(
        &self,
        p_dst_resource: T0__,
        p_src_resource: T1__,
    ) {
        (::windows::Interface::vtable(self).47)(
            ::windows::Abi::abi(self),
            p_dst_resource.into_param().abi(),
            p_src_resource.into_param().abi(),
        )
    }
    pub unsafe fn UpdateSubresource<'a, T0__: ::windows::IntoParam<'a, ID3D11Resource>>(
        &self,
        p_dst_resource: T0__,
        dst_subresource: u32,
        p_dst_box: *const D3D11_BOX,
        p_src_data: *const ::std::ffi::c_void,
        src_row_pitch: u32,
        src_depth_pitch: u32,
    ) {
        (::windows::Interface::vtable(self).48)(
            ::windows::Abi::abi(self),
            p_dst_resource.into_param().abi(),
            ::std::mem::transmute(dst_subresource),
            ::std::mem::transmute(p_dst_box),
            ::std::mem::transmute(p_src_data),
            ::std::mem::transmute(src_row_pitch),
            ::std::mem::transmute(src_depth_pitch),
        )
    }
    pub unsafe fn CopyStructureCount<
        'a,
        T0__: ::windows::IntoParam<'a, ID3D11Buffer>,
        T2__: ::windows::IntoParam<'a, ID3D11UnorderedAccessView>,
    >(
        &self,
        p_dst_buffer: T0__,
        dst_aligned_byte_offset: u32,
        p_src_view: T2__,
    ) {
        (::windows::Interface::vtable(self).49)(
            ::windows::Abi::abi(self),
            p_dst_buffer.into_param().abi(),
            ::std::mem::transmute(dst_aligned_byte_offset),
            p_src_view.into_param().abi(),
        )
    }
    pub unsafe fn ClearRenderTargetView<
        'a,
        T0__: ::windows::IntoParam<'a, ID3D11RenderTargetView>,
    >(
        &self,
        p_render_target_view: T0__,
        color_rgba: *const f32,
    ) {
        (::windows::Interface::vtable(self).50)(
            ::windows::Abi::abi(self),
            p_render_target_view.into_param().abi(),
            ::std::mem::transmute(color_rgba),
        )
    }
    pub unsafe fn ClearUnorderedAccessViewUint<
        'a,
        T0__: ::windows::IntoParam<'a, ID3D11UnorderedAccessView>,
    >(
        &self,
        p_unordered_access_view: T0__,
        values: *const u32,
    ) {
        (::windows::Interface::vtable(self).51)(
            ::windows::Abi::abi(self),
            p_unordered_access_view.into_param().abi(),
            ::std::mem::transmute(values),
        )
    }
    pub unsafe fn ClearUnorderedAccessViewFloat<
        'a,
        T0__: ::windows::IntoParam<'a, ID3D11UnorderedAccessView>,
    >(
        &self,
        p_unordered_access_view: T0__,
        values: *const f32,
    ) {
        (::windows::Interface::vtable(self).52)(
            ::windows::Abi::abi(self),
            p_unordered_access_view.into_param().abi(),
            ::std::mem::transmute(values),
        )
    }
    pub unsafe fn ClearDepthStencilView<
        'a,
        T0__: ::windows::IntoParam<'a, ID3D11DepthStencilView>,
    >(
        &self,
        p_depth_stencil_view: T0__,
        clear_flags: u32,
        depth: f32,
        stencil: u8,
    ) {
        (::windows::Interface::vtable(self).53)(
            ::windows::Abi::abi(self),
            p_depth_stencil_view.into_param().abi(),
            ::std::mem::transmute(clear_flags),
            ::std::mem::transmute(depth),
            ::std::mem::transmute(stencil),
        )
    }
    pub unsafe fn GenerateMips<'a, T0__: ::windows::IntoParam<'a, ID3D11ShaderResourceView>>(
        &self,
        p_shader_resource_view: T0__,
    ) {
        (::windows::Interface::vtable(self).54)(
            ::windows::Abi::abi(self),
            p_shader_resource_view.into_param().abi(),
        )
    }
    pub unsafe fn SetResourceMinLOD<'a, T0__: ::windows::IntoParam<'a, ID3D11Resource>>(
        &self,
        p_resource: T0__,
        min_lod: f32,
    ) {
        (::windows::Interface::vtable(self).55)(
            ::windows::Abi::abi(self),
            p_resource.into_param().abi(),
            ::std::mem::transmute(min_lod),
        )
    }
    pub unsafe fn GetResourceMinLOD<'a, T0__: ::windows::IntoParam<'a, ID3D11Resource>>(
        &self,
        p_resource: T0__,
    ) -> f32 {
        (::windows::Interface::vtable(self).56)(
            ::windows::Abi::abi(self),
            p_resource.into_param().abi(),
        )
    }
    pub unsafe fn ResolveSubresource<
        'a,
        T0__: ::windows::IntoParam<'a, ID3D11Resource>,
        T2__: ::windows::IntoParam<'a, ID3D11Resource>,
    >(
        &self,
        p_dst_resource: T0__,
        dst_subresource: u32,
        p_src_resource: T2__,
        src_subresource: u32,
        format: super::dxgi::DXGI_FORMAT,
    ) {
        (::windows::Interface::vtable(self).57)(
            ::windows::Abi::abi(self),
            p_dst_resource.into_param().abi(),
            ::std::mem::transmute(dst_subresource),
            p_src_resource.into_param().abi(),
            ::std::mem::transmute(src_subresource),
            ::std::mem::transmute(format),
        )
    }
    pub unsafe fn ExecuteCommandList<
        'a,
        T0__: ::windows::IntoParam<'a, ID3D11CommandList>,
        T1__: ::windows::IntoParam<'a, super::system_services::BOOL>,
    >(
        &self,
        p_command_list: T0__,
        restore_context_state: T1__,
    ) {
        (::windows::Interface::vtable(self).58)(
            ::windows::Abi::abi(self),
            p_command_list.into_param().abi(),
            restore_context_state.into_param().abi(),
        )
    }
    pub unsafe fn HSSetShaderResources(
        &self,
        start_slot: u32,
        num_views: u32,
        pp_shader_resource_views: *mut ::std::option::Option<ID3D11ShaderResourceView>,
    ) {
        (::windows::Interface::vtable(self).59)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(start_slot),
            ::std::mem::transmute(num_views),
            ::std::mem::transmute(pp_shader_resource_views),
        )
    }
    pub unsafe fn HSSetShader<'a, T0__: ::windows::IntoParam<'a, ID3D11HullShader>>(
        &self,
        p_hull_shader: T0__,
        pp_class_instances: *mut ::std::option::Option<ID3D11ClassInstance>,
        num_class_instances: u32,
    ) {
        (::windows::Interface::vtable(self).60)(
            ::windows::Abi::abi(self),
            p_hull_shader.into_param().abi(),
            ::std::mem::transmute(pp_class_instances),
            ::std::mem::transmute(num_class_instances),
        )
    }
    pub unsafe fn HSSetSamplers(
        &self,
        start_slot: u32,
        num_samplers: u32,
        pp_samplers: *mut ::std::option::Option<ID3D11SamplerState>,
    ) {
        (::windows::Interface::vtable(self).61)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(start_slot),
            ::std::mem::transmute(num_samplers),
            ::std::mem::transmute(pp_samplers),
        )
    }
    pub unsafe fn HSSetConstantBuffers(
        &self,
        start_slot: u32,
        num_buffers: u32,
        pp_constant_buffers: *mut ::std::option::Option<ID3D11Buffer>,
    ) {
        (::windows::Interface::vtable(self).62)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(start_slot),
            ::std::mem::transmute(num_buffers),
            ::std::mem::transmute(pp_constant_buffers),
        )
    }
    pub unsafe fn DSSetShaderResources(
        &self,
        start_slot: u32,
        num_views: u32,
        pp_shader_resource_views: *mut ::std::option::Option<ID3D11ShaderResourceView>,
    ) {
        (::windows::Interface::vtable(self).63)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(start_slot),
            ::std::mem::transmute(num_views),
            ::std::mem::transmute(pp_shader_resource_views),
        )
    }
    pub unsafe fn DSSetShader<'a, T0__: ::windows::IntoParam<'a, ID3D11DomainShader>>(
        &self,
        p_domain_shader: T0__,
        pp_class_instances: *mut ::std::option::Option<ID3D11ClassInstance>,
        num_class_instances: u32,
    ) {
        (::windows::Interface::vtable(self).64)(
            ::windows::Abi::abi(self),
            p_domain_shader.into_param().abi(),
            ::std::mem::transmute(pp_class_instances),
            ::std::mem::transmute(num_class_instances),
        )
    }
    pub unsafe fn DSSetSamplers(
        &self,
        start_slot: u32,
        num_samplers: u32,
        pp_samplers: *mut ::std::option::Option<ID3D11SamplerState>,
    ) {
        (::windows::Interface::vtable(self).65)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(start_slot),
            ::std::mem::transmute(num_samplers),
            ::std::mem::transmute(pp_samplers),
        )
    }
    pub unsafe fn DSSetConstantBuffers(
        &self,
        start_slot: u32,
        num_buffers: u32,
        pp_constant_buffers: *mut ::std::option::Option<ID3D11Buffer>,
    ) {
        (::windows::Interface::vtable(self).66)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(start_slot),
            ::std::mem::transmute(num_buffers),
            ::std::mem::transmute(pp_constant_buffers),
        )
    }
    pub unsafe fn CSSetShaderResources(
        &self,
        start_slot: u32,
        num_views: u32,
        pp_shader_resource_views: *mut ::std::option::Option<ID3D11ShaderResourceView>,
    ) {
        (::windows::Interface::vtable(self).67)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(start_slot),
            ::std::mem::transmute(num_views),
            ::std::mem::transmute(pp_shader_resource_views),
        )
    }
    pub unsafe fn CSSetUnorderedAccessViews(
        &self,
        start_slot: u32,
        num_ua_vs: u32,
        pp_unordered_access_views: *mut ::std::option::Option<ID3D11UnorderedAccessView>,
        p_uav_initial_counts: *const u32,
    ) {
        (::windows::Interface::vtable(self).68)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(start_slot),
            ::std::mem::transmute(num_ua_vs),
            ::std::mem::transmute(pp_unordered_access_views),
            ::std::mem::transmute(p_uav_initial_counts),
        )
    }
    pub unsafe fn CSSetShader<'a, T0__: ::windows::IntoParam<'a, ID3D11ComputeShader>>(
        &self,
        p_compute_shader: T0__,
        pp_class_instances: *mut ::std::option::Option<ID3D11ClassInstance>,
        num_class_instances: u32,
    ) {
        (::windows::Interface::vtable(self).69)(
            ::windows::Abi::abi(self),
            p_compute_shader.into_param().abi(),
            ::std::mem::transmute(pp_class_instances),
            ::std::mem::transmute(num_class_instances),
        )
    }
    pub unsafe fn CSSetSamplers(
        &self,
        start_slot: u32,
        num_samplers: u32,
        pp_samplers: *mut ::std::option::Option<ID3D11SamplerState>,
    ) {
        (::windows::Interface::vtable(self).70)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(start_slot),
            ::std::mem::transmute(num_samplers),
            ::std::mem::transmute(pp_samplers),
        )
    }
    pub unsafe fn CSSetConstantBuffers(
        &self,
        start_slot: u32,
        num_buffers: u32,
        pp_constant_buffers: *mut ::std::option::Option<ID3D11Buffer>,
    ) {
        (::windows::Interface::vtable(self).71)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(start_slot),
            ::std::mem::transmute(num_buffers),
            ::std::mem::transmute(pp_constant_buffers),
        )
    }
    pub unsafe fn VSGetConstantBuffers(
        &self,
        start_slot: u32,
        num_buffers: u32,
        pp_constant_buffers: *mut ::std::option::Option<ID3D11Buffer>,
    ) {
        (::windows::Interface::vtable(self).72)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(start_slot),
            ::std::mem::transmute(num_buffers),
            ::std::mem::transmute(pp_constant_buffers),
        )
    }
    pub unsafe fn PSGetShaderResources(
        &self,
        start_slot: u32,
        num_views: u32,
        pp_shader_resource_views: *mut ::std::option::Option<ID3D11ShaderResourceView>,
    ) {
        (::windows::Interface::vtable(self).73)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(start_slot),
            ::std::mem::transmute(num_views),
            ::std::mem::transmute(pp_shader_resource_views),
        )
    }
    pub unsafe fn PSGetShader(
        &self,
        pp_pixel_shader: *mut ::std::option::Option<ID3D11PixelShader>,
        pp_class_instances: *mut ::std::option::Option<ID3D11ClassInstance>,
        p_num_class_instances: *mut u32,
    ) {
        (::windows::Interface::vtable(self).74)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(pp_pixel_shader),
            ::std::mem::transmute(pp_class_instances),
            ::std::mem::transmute(p_num_class_instances),
        )
    }
    pub unsafe fn PSGetSamplers(
        &self,
        start_slot: u32,
        num_samplers: u32,
        pp_samplers: *mut ::std::option::Option<ID3D11SamplerState>,
    ) {
        (::windows::Interface::vtable(self).75)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(start_slot),
            ::std::mem::transmute(num_samplers),
            ::std::mem::transmute(pp_samplers),
        )
    }
    pub unsafe fn VSGetShader(
        &self,
        pp_vertex_shader: *mut ::std::option::Option<ID3D11VertexShader>,
        pp_class_instances: *mut ::std::option::Option<ID3D11ClassInstance>,
        p_num_class_instances: *mut u32,
    ) {
        (::windows::Interface::vtable(self).76)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(pp_vertex_shader),
            ::std::mem::transmute(pp_class_instances),
            ::std::mem::transmute(p_num_class_instances),
        )
    }
    pub unsafe fn PSGetConstantBuffers(
        &self,
        start_slot: u32,
        num_buffers: u32,
        pp_constant_buffers: *mut ::std::option::Option<ID3D11Buffer>,
    ) {
        (::windows::Interface::vtable(self).77)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(start_slot),
            ::std::mem::transmute(num_buffers),
            ::std::mem::transmute(pp_constant_buffers),
        )
    }
    pub unsafe fn IAGetInputLayout(
        &self,
        pp_input_layout: *mut ::std::option::Option<ID3D11InputLayout>,
    ) {
        (::windows::Interface::vtable(self).78)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(pp_input_layout),
        )
    }
    pub unsafe fn IAGetVertexBuffers(
        &self,
        start_slot: u32,
        num_buffers: u32,
        pp_vertex_buffers: *mut ::std::option::Option<ID3D11Buffer>,
        p_strides: *mut u32,
        p_offsets: *mut u32,
    ) {
        (::windows::Interface::vtable(self).79)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(start_slot),
            ::std::mem::transmute(num_buffers),
            ::std::mem::transmute(pp_vertex_buffers),
            ::std::mem::transmute(p_strides),
            ::std::mem::transmute(p_offsets),
        )
    }
    pub unsafe fn IAGetIndexBuffer(
        &self,
        p_index_buffer: *mut ::std::option::Option<ID3D11Buffer>,
        format: *mut super::dxgi::DXGI_FORMAT,
        offset: *mut u32,
    ) {
        (::windows::Interface::vtable(self).80)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(p_index_buffer),
            ::std::mem::transmute(format),
            ::std::mem::transmute(offset),
        )
    }
    pub unsafe fn GSGetConstantBuffers(
        &self,
        start_slot: u32,
        num_buffers: u32,
        pp_constant_buffers: *mut ::std::option::Option<ID3D11Buffer>,
    ) {
        (::windows::Interface::vtable(self).81)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(start_slot),
            ::std::mem::transmute(num_buffers),
            ::std::mem::transmute(pp_constant_buffers),
        )
    }
    pub unsafe fn GSGetShader(
        &self,
        pp_geometry_shader: *mut ::std::option::Option<ID3D11GeometryShader>,
        pp_class_instances: *mut ::std::option::Option<ID3D11ClassInstance>,
        p_num_class_instances: *mut u32,
    ) {
        (::windows::Interface::vtable(self).82)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(pp_geometry_shader),
            ::std::mem::transmute(pp_class_instances),
            ::std::mem::transmute(p_num_class_instances),
        )
    }
    pub unsafe fn IAGetPrimitiveTopology(&self, p_topology: *mut D3D_PRIMITIVE_TOPOLOGY) {
        (::windows::Interface::vtable(self).83)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(p_topology),
        )
    }
    pub unsafe fn VSGetShaderResources(
        &self,
        start_slot: u32,
        num_views: u32,
        pp_shader_resource_views: *mut ::std::option::Option<ID3D11ShaderResourceView>,
    ) {
        (::windows::Interface::vtable(self).84)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(start_slot),
            ::std::mem::transmute(num_views),
            ::std::mem::transmute(pp_shader_resource_views),
        )
    }
    pub unsafe fn VSGetSamplers(
        &self,
        start_slot: u32,
        num_samplers: u32,
        pp_samplers: *mut ::std::option::Option<ID3D11SamplerState>,
    ) {
        (::windows::Interface::vtable(self).85)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(start_slot),
            ::std::mem::transmute(num_samplers),
            ::std::mem::transmute(pp_samplers),
        )
    }
    pub unsafe fn GetPredication(
        &self,
        pp_predicate: *mut ::std::option::Option<ID3D11Predicate>,
        p_predicate_value: *mut super::system_services::BOOL,
    ) {
        (::windows::Interface::vtable(self).86)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(pp_predicate),
            ::std::mem::transmute(p_predicate_value),
        )
    }
    pub unsafe fn GSGetShaderResources(
        &self,
        start_slot: u32,
        num_views: u32,
        pp_shader_resource_views: *mut ::std::option::Option<ID3D11ShaderResourceView>,
    ) {
        (::windows::Interface::vtable(self).87)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(start_slot),
            ::std::mem::transmute(num_views),
            ::std::mem::transmute(pp_shader_resource_views),
        )
    }
    pub unsafe fn GSGetSamplers(
        &self,
        start_slot: u32,
        num_samplers: u32,
        pp_samplers: *mut ::std::option::Option<ID3D11SamplerState>,
    ) {
        (::windows::Interface::vtable(self).88)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(start_slot),
            ::std::mem::transmute(num_samplers),
            ::std::mem::transmute(pp_samplers),
        )
    }
    pub unsafe fn OMGetRenderTargets(
        &self,
        num_views: u32,
        pp_render_target_views: *mut ::std::option::Option<ID3D11RenderTargetView>,
        pp_depth_stencil_view: *mut ::std::option::Option<ID3D11DepthStencilView>,
    ) {
        (::windows::Interface::vtable(self).89)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(num_views),
            ::std::mem::transmute(pp_render_target_views),
            ::std::mem::transmute(pp_depth_stencil_view),
        )
    }
    pub unsafe fn OMGetRenderTargetsAndUnorderedAccessViews(
        &self,
        num_rt_vs: u32,
        pp_render_target_views: *mut ::std::option::Option<ID3D11RenderTargetView>,
        pp_depth_stencil_view: *mut ::std::option::Option<ID3D11DepthStencilView>,
        uav_start_slot: u32,
        num_ua_vs: u32,
        pp_unordered_access_views: *mut ::std::option::Option<ID3D11UnorderedAccessView>,
    ) {
        (::windows::Interface::vtable(self).90)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(num_rt_vs),
            ::std::mem::transmute(pp_render_target_views),
            ::std::mem::transmute(pp_depth_stencil_view),
            ::std::mem::transmute(uav_start_slot),
            ::std::mem::transmute(num_ua_vs),
            ::std::mem::transmute(pp_unordered_access_views),
        )
    }
    pub unsafe fn OMGetBlendState(
        &self,
        pp_blend_state: *mut ::std::option::Option<ID3D11BlendState>,
        blend_factor: *mut f32,
        p_sample_mask: *mut u32,
    ) {
        (::windows::Interface::vtable(self).91)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(pp_blend_state),
            ::std::mem::transmute(blend_factor),
            ::std::mem::transmute(p_sample_mask),
        )
    }
    pub unsafe fn OMGetDepthStencilState(
        &self,
        pp_depth_stencil_state: *mut ::std::option::Option<ID3D11DepthStencilState>,
        p_stencil_ref: *mut u32,
    ) {
        (::windows::Interface::vtable(self).92)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(pp_depth_stencil_state),
            ::std::mem::transmute(p_stencil_ref),
        )
    }
    pub unsafe fn SOGetTargets(
        &self,
        num_buffers: u32,
        pp_so_targets: *mut ::std::option::Option<ID3D11Buffer>,
    ) {
        (::windows::Interface::vtable(self).93)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(num_buffers),
            ::std::mem::transmute(pp_so_targets),
        )
    }
    pub unsafe fn RSGetState(
        &self,
        pp_rasterizer_state: *mut ::std::option::Option<ID3D11RasterizerState>,
    ) {
        (::windows::Interface::vtable(self).94)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(pp_rasterizer_state),
        )
    }
    pub unsafe fn RSGetViewports(
        &self,
        p_num_viewports: *mut u32,
        p_viewports: *mut D3D11_VIEWPORT,
    ) {
        (::windows::Interface::vtable(self).95)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(p_num_viewports),
            ::std::mem::transmute(p_viewports),
        )
    }
    pub unsafe fn RSGetScissorRects(
        &self,
        p_num_rects: *mut u32,
        p_rects: *mut super::display_devices::RECT,
    ) {
        (::windows::Interface::vtable(self).96)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(p_num_rects),
            ::std::mem::transmute(p_rects),
        )
    }
    pub unsafe fn HSGetShaderResources(
        &self,
        start_slot: u32,
        num_views: u32,
        pp_shader_resource_views: *mut ::std::option::Option<ID3D11ShaderResourceView>,
    ) {
        (::windows::Interface::vtable(self).97)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(start_slot),
            ::std::mem::transmute(num_views),
            ::std::mem::transmute(pp_shader_resource_views),
        )
    }
    pub unsafe fn HSGetShader(
        &self,
        pp_hull_shader: *mut ::std::option::Option<ID3D11HullShader>,
        pp_class_instances: *mut ::std::option::Option<ID3D11ClassInstance>,
        p_num_class_instances: *mut u32,
    ) {
        (::windows::Interface::vtable(self).98)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(pp_hull_shader),
            ::std::mem::transmute(pp_class_instances),
            ::std::mem::transmute(p_num_class_instances),
        )
    }
    pub unsafe fn HSGetSamplers(
        &self,
        start_slot: u32,
        num_samplers: u32,
        pp_samplers: *mut ::std::option::Option<ID3D11SamplerState>,
    ) {
        (::windows::Interface::vtable(self).99)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(start_slot),
            ::std::mem::transmute(num_samplers),
            ::std::mem::transmute(pp_samplers),
        )
    }
    pub unsafe fn HSGetConstantBuffers(
        &self,
        start_slot: u32,
        num_buffers: u32,
        pp_constant_buffers: *mut ::std::option::Option<ID3D11Buffer>,
    ) {
        (::windows::Interface::vtable(self).100)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(start_slot),
            ::std::mem::transmute(num_buffers),
            ::std::mem::transmute(pp_constant_buffers),
        )
    }
    pub unsafe fn DSGetShaderResources(
        &self,
        start_slot: u32,
        num_views: u32,
        pp_shader_resource_views: *mut ::std::option::Option<ID3D11ShaderResourceView>,
    ) {
        (::windows::Interface::vtable(self).101)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(start_slot),
            ::std::mem::transmute(num_views),
            ::std::mem::transmute(pp_shader_resource_views),
        )
    }
    pub unsafe fn DSGetShader(
        &self,
        pp_domain_shader: *mut ::std::option::Option<ID3D11DomainShader>,
        pp_class_instances: *mut ::std::option::Option<ID3D11ClassInstance>,
        p_num_class_instances: *mut u32,
    ) {
        (::windows::Interface::vtable(self).102)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(pp_domain_shader),
            ::std::mem::transmute(pp_class_instances),
            ::std::mem::transmute(p_num_class_instances),
        )
    }
    pub unsafe fn DSGetSamplers(
        &self,
        start_slot: u32,
        num_samplers: u32,
        pp_samplers: *mut ::std::option::Option<ID3D11SamplerState>,
    ) {
        (::windows::Interface::vtable(self).103)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(start_slot),
            ::std::mem::transmute(num_samplers),
            ::std::mem::transmute(pp_samplers),
        )
    }
    pub unsafe fn DSGetConstantBuffers(
        &self,
        start_slot: u32,
        num_buffers: u32,
        pp_constant_buffers: *mut ::std::option::Option<ID3D11Buffer>,
    ) {
        (::windows::Interface::vtable(self).104)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(start_slot),
            ::std::mem::transmute(num_buffers),
            ::std::mem::transmute(pp_constant_buffers),
        )
    }
    pub unsafe fn CSGetShaderResources(
        &self,
        start_slot: u32,
        num_views: u32,
        pp_shader_resource_views: *mut ::std::option::Option<ID3D11ShaderResourceView>,
    ) {
        (::windows::Interface::vtable(self).105)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(start_slot),
            ::std::mem::transmute(num_views),
            ::std::mem::transmute(pp_shader_resource_views),
        )
    }
    pub unsafe fn CSGetUnorderedAccessViews(
        &self,
        start_slot: u32,
        num_ua_vs: u32,
        pp_unordered_access_views: *mut ::std::option::Option<ID3D11UnorderedAccessView>,
    ) {
        (::windows::Interface::vtable(self).106)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(start_slot),
            ::std::mem::transmute(num_ua_vs),
            ::std::mem::transmute(pp_unordered_access_views),
        )
    }
    pub unsafe fn CSGetShader(
        &self,
        pp_compute_shader: *mut ::std::option::Option<ID3D11ComputeShader>,
        pp_class_instances: *mut ::std::option::Option<ID3D11ClassInstance>,
        p_num_class_instances: *mut u32,
    ) {
        (::windows::Interface::vtable(self).107)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(pp_compute_shader),
            ::std::mem::transmute(pp_class_instances),
            ::std::mem::transmute(p_num_class_instances),
        )
    }
    pub unsafe fn CSGetSamplers(
        &self,
        start_slot: u32,
        num_samplers: u32,
        pp_samplers: *mut ::std::option::Option<ID3D11SamplerState>,
    ) {
        (::windows::Interface::vtable(self).108)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(start_slot),
            ::std::mem::transmute(num_samplers),
            ::std::mem::transmute(pp_samplers),
        )
    }
    pub unsafe fn CSGetConstantBuffers(
        &self,
        start_slot: u32,
        num_buffers: u32,
        pp_constant_buffers: *mut ::std::option::Option<ID3D11Buffer>,
    ) {
        (::windows::Interface::vtable(self).109)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(start_slot),
            ::std::mem::transmute(num_buffers),
            ::std::mem::transmute(pp_constant_buffers),
        )
    }
    pub unsafe fn ClearState(&self) {
        (::windows::Interface::vtable(self).110)(::windows::Abi::abi(self))
    }
    pub unsafe fn Flush(&self) {
        (::windows::Interface::vtable(self).111)(::windows::Abi::abi(self))
    }
    pub unsafe fn GetType(&self) -> D3D11_DEVICE_CONTEXT_TYPE {
        (::windows::Interface::vtable(self).112)(::windows::Abi::abi(self))
    }
    pub unsafe fn GetContextFlags(&self) -> u32 {
        (::windows::Interface::vtable(self).113)(::windows::Abi::abi(self))
    }
    pub unsafe fn FinishCommandList<
        'a,
        T0__: ::windows::IntoParam<'a, super::system_services::BOOL>,
    >(
        &self,
        restore_deferred_context_state: T0__,
        pp_command_list: *mut ::std::option::Option<ID3D11CommandList>,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).114)(
            ::windows::Abi::abi(self),
            restore_deferred_context_state.into_param().abi(),
            ::std::mem::transmute(pp_command_list),
        )
    }
}
impl ::std::convert::From<ID3D11DeviceContext> for ::windows::IUnknown {
    fn from(value: ID3D11DeviceContext) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&ID3D11DeviceContext> for ::windows::IUnknown {
    fn from(value: &ID3D11DeviceContext) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::IntoParam<'a, ::windows::IUnknown> for ID3D11DeviceContext {
    fn into_param(self) -> ::windows::Param<'a, ::windows::IUnknown> {
        ::windows::Param::Owned(::std::convert::Into::<::windows::IUnknown>::into(self))
    }
}
impl<'a> ::windows::IntoParam<'a, ::windows::IUnknown> for &'a ID3D11DeviceContext {
    fn into_param(self) -> ::windows::Param<'a, ::windows::IUnknown> {
        ::windows::Param::Owned(::std::convert::Into::<::windows::IUnknown>::into(
            ::std::clone::Clone::clone(self),
        ))
    }
}
impl ::std::convert::From<ID3D11DeviceContext> for ID3D11DeviceChild {
    fn from(value: ID3D11DeviceContext) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&ID3D11DeviceContext> for ID3D11DeviceChild {
    fn from(value: &ID3D11DeviceContext) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::IntoParam<'a, ID3D11DeviceChild> for ID3D11DeviceContext {
    fn into_param(self) -> ::windows::Param<'a, ID3D11DeviceChild> {
        ::windows::Param::Owned(::std::convert::Into::<ID3D11DeviceChild>::into(self))
    }
}
impl<'a> ::windows::IntoParam<'a, ID3D11DeviceChild> for &'a ID3D11DeviceContext {
    fn into_param(self) -> ::windows::Param<'a, ID3D11DeviceChild> {
        ::windows::Param::Owned(::std::convert::Into::<ID3D11DeviceChild>::into(
            ::std::clone::Clone::clone(self),
        ))
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ID3D11DeviceContext_abi(
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        iid: &::windows::Guid,
        interface: *mut ::windows::RawPtr,
    ) -> ::windows::ErrorCode,
    pub unsafe extern "system" fn(this: ::windows::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::RawPtr, pp_device: *mut ::windows::RawPtr),
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        guid: *const ::windows::Guid,
        p_data_size: *mut u32,
        p_data: *mut ::std::ffi::c_void,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        guid: *const ::windows::Guid,
        data_size: u32,
        p_data: *const ::std::ffi::c_void,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        guid: *const ::windows::Guid,
        p_data: ::windows::RawPtr,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        start_slot: u32,
        num_buffers: u32,
        pp_constant_buffers: *mut ::windows::RawPtr,
    ),
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        start_slot: u32,
        num_views: u32,
        pp_shader_resource_views: *mut ::windows::RawPtr,
    ),
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        p_pixel_shader: ::windows::RawPtr,
        pp_class_instances: *mut ::windows::RawPtr,
        num_class_instances: u32,
    ),
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        start_slot: u32,
        num_samplers: u32,
        pp_samplers: *mut ::windows::RawPtr,
    ),
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        p_vertex_shader: ::windows::RawPtr,
        pp_class_instances: *mut ::windows::RawPtr,
        num_class_instances: u32,
    ),
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        index_count: u32,
        start_index_location: u32,
        base_vertex_location: i32,
    ),
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        vertex_count: u32,
        start_vertex_location: u32,
    ),
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        p_resource: ::windows::RawPtr,
        subresource: u32,
        map_type: D3D11_MAP,
        map_flags: u32,
        p_mapped_resource: *mut D3D11_MAPPED_SUBRESOURCE,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        p_resource: ::windows::RawPtr,
        subresource: u32,
    ),
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        start_slot: u32,
        num_buffers: u32,
        pp_constant_buffers: *mut ::windows::RawPtr,
    ),
    pub unsafe extern "system" fn(this: ::windows::RawPtr, p_input_layout: ::windows::RawPtr),
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        start_slot: u32,
        num_buffers: u32,
        pp_vertex_buffers: *mut ::windows::RawPtr,
        p_strides: *const u32,
        p_offsets: *const u32,
    ),
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        p_index_buffer: ::windows::RawPtr,
        format: super::dxgi::DXGI_FORMAT,
        offset: u32,
    ),
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        index_count_per_instance: u32,
        instance_count: u32,
        start_index_location: u32,
        base_vertex_location: i32,
        start_instance_location: u32,
    ),
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        vertex_count_per_instance: u32,
        instance_count: u32,
        start_vertex_location: u32,
        start_instance_location: u32,
    ),
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        start_slot: u32,
        num_buffers: u32,
        pp_constant_buffers: *mut ::windows::RawPtr,
    ),
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        p_shader: ::windows::RawPtr,
        pp_class_instances: *mut ::windows::RawPtr,
        num_class_instances: u32,
    ),
    pub unsafe extern "system" fn(this: ::windows::RawPtr, topology: D3D_PRIMITIVE_TOPOLOGY),
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        start_slot: u32,
        num_views: u32,
        pp_shader_resource_views: *mut ::windows::RawPtr,
    ),
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        start_slot: u32,
        num_samplers: u32,
        pp_samplers: *mut ::windows::RawPtr,
    ),
    pub unsafe extern "system" fn(this: ::windows::RawPtr, p_async: ::windows::RawPtr),
    pub unsafe extern "system" fn(this: ::windows::RawPtr, p_async: ::windows::RawPtr),
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        p_async: ::windows::RawPtr,
        p_data: *mut ::std::ffi::c_void,
        data_size: u32,
        get_data_flags: u32,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        p_predicate: ::windows::RawPtr,
        predicate_value: super::system_services::BOOL,
    ),
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        start_slot: u32,
        num_views: u32,
        pp_shader_resource_views: *mut ::windows::RawPtr,
    ),
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        start_slot: u32,
        num_samplers: u32,
        pp_samplers: *mut ::windows::RawPtr,
    ),
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        num_views: u32,
        pp_render_target_views: *mut ::windows::RawPtr,
        p_depth_stencil_view: ::windows::RawPtr,
    ),
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        num_rt_vs: u32,
        pp_render_target_views: *mut ::windows::RawPtr,
        p_depth_stencil_view: ::windows::RawPtr,
        uav_start_slot: u32,
        num_ua_vs: u32,
        pp_unordered_access_views: *mut ::windows::RawPtr,
        p_uav_initial_counts: *const u32,
    ),
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        p_blend_state: ::windows::RawPtr,
        blend_factor: *const f32,
        sample_mask: u32,
    ),
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        p_depth_stencil_state: ::windows::RawPtr,
        stencil_ref: u32,
    ),
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        num_buffers: u32,
        pp_so_targets: *mut ::windows::RawPtr,
        p_offsets: *const u32,
    ),
    pub unsafe extern "system" fn(this: ::windows::RawPtr),
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        p_buffer_for_args: ::windows::RawPtr,
        aligned_byte_offset_for_args: u32,
    ),
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        p_buffer_for_args: ::windows::RawPtr,
        aligned_byte_offset_for_args: u32,
    ),
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        thread_group_countx: u32,
        thread_group_county: u32,
        thread_group_countz: u32,
    ),
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        p_buffer_for_args: ::windows::RawPtr,
        aligned_byte_offset_for_args: u32,
    ),
    pub unsafe extern "system" fn(this: ::windows::RawPtr, p_rasterizer_state: ::windows::RawPtr),
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        num_viewports: u32,
        p_viewports: *const D3D11_VIEWPORT,
    ),
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        num_rects: u32,
        p_rects: *const super::display_devices::RECT,
    ),
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        p_dst_resource: ::windows::RawPtr,
        dst_subresource: u32,
        dstx: u32,
        dsty: u32,
        dstz: u32,
        p_src_resource: ::windows::RawPtr,
        src_subresource: u32,
        p_src_box: *const D3D11_BOX,
    ),
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        p_dst_resource: ::windows::RawPtr,
        p_src_resource: ::windows::RawPtr,
    ),
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        p_dst_resource: ::windows::RawPtr,
        dst_subresource: u32,
        p_dst_box: *const D3D11_BOX,
        p_src_data: *const ::std::ffi::c_void,
        src_row_pitch: u32,
        src_depth_pitch: u32,
    ),
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        p_dst_buffer: ::windows::RawPtr,
        dst_aligned_byte_offset: u32,
        p_src_view: ::windows::RawPtr,
    ),
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        p_render_target_view: ::windows::RawPtr,
        color_rgba: *const f32,
    ),
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        p_unordered_access_view: ::windows::RawPtr,
        values: *const u32,
    ),
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        p_unordered_access_view: ::windows::RawPtr,
        values: *const f32,
    ),
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        p_depth_stencil_view: ::windows::RawPtr,
        clear_flags: u32,
        depth: f32,
        stencil: u8,
    ),
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        p_shader_resource_view: ::windows::RawPtr,
    ),
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        p_resource: ::windows::RawPtr,
        min_lod: f32,
    ),
    pub unsafe extern "system" fn(this: ::windows::RawPtr, p_resource: ::windows::RawPtr) -> f32,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        p_dst_resource: ::windows::RawPtr,
        dst_subresource: u32,
        p_src_resource: ::windows::RawPtr,
        src_subresource: u32,
        format: super::dxgi::DXGI_FORMAT,
    ),
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        p_command_list: ::windows::RawPtr,
        restore_context_state: super::system_services::BOOL,
    ),
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        start_slot: u32,
        num_views: u32,
        pp_shader_resource_views: *mut ::windows::RawPtr,
    ),
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        p_hull_shader: ::windows::RawPtr,
        pp_class_instances: *mut ::windows::RawPtr,
        num_class_instances: u32,
    ),
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        start_slot: u32,
        num_samplers: u32,
        pp_samplers: *mut ::windows::RawPtr,
    ),
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        start_slot: u32,
        num_buffers: u32,
        pp_constant_buffers: *mut ::windows::RawPtr,
    ),
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        start_slot: u32,
        num_views: u32,
        pp_shader_resource_views: *mut ::windows::RawPtr,
    ),
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        p_domain_shader: ::windows::RawPtr,
        pp_class_instances: *mut ::windows::RawPtr,
        num_class_instances: u32,
    ),
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        start_slot: u32,
        num_samplers: u32,
        pp_samplers: *mut ::windows::RawPtr,
    ),
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        start_slot: u32,
        num_buffers: u32,
        pp_constant_buffers: *mut ::windows::RawPtr,
    ),
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        start_slot: u32,
        num_views: u32,
        pp_shader_resource_views: *mut ::windows::RawPtr,
    ),
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        start_slot: u32,
        num_ua_vs: u32,
        pp_unordered_access_views: *mut ::windows::RawPtr,
        p_uav_initial_counts: *const u32,
    ),
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        p_compute_shader: ::windows::RawPtr,
        pp_class_instances: *mut ::windows::RawPtr,
        num_class_instances: u32,
    ),
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        start_slot: u32,
        num_samplers: u32,
        pp_samplers: *mut ::windows::RawPtr,
    ),
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        start_slot: u32,
        num_buffers: u32,
        pp_constant_buffers: *mut ::windows::RawPtr,
    ),
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        start_slot: u32,
        num_buffers: u32,
        pp_constant_buffers: *mut ::windows::RawPtr,
    ),
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        start_slot: u32,
        num_views: u32,
        pp_shader_resource_views: *mut ::windows::RawPtr,
    ),
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        pp_pixel_shader: *mut ::windows::RawPtr,
        pp_class_instances: *mut ::windows::RawPtr,
        p_num_class_instances: *mut u32,
    ),
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        start_slot: u32,
        num_samplers: u32,
        pp_samplers: *mut ::windows::RawPtr,
    ),
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        pp_vertex_shader: *mut ::windows::RawPtr,
        pp_class_instances: *mut ::windows::RawPtr,
        p_num_class_instances: *mut u32,
    ),
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        start_slot: u32,
        num_buffers: u32,
        pp_constant_buffers: *mut ::windows::RawPtr,
    ),
    pub unsafe extern "system" fn(this: ::windows::RawPtr, pp_input_layout: *mut ::windows::RawPtr),
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        start_slot: u32,
        num_buffers: u32,
        pp_vertex_buffers: *mut ::windows::RawPtr,
        p_strides: *mut u32,
        p_offsets: *mut u32,
    ),
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        p_index_buffer: *mut ::windows::RawPtr,
        format: *mut super::dxgi::DXGI_FORMAT,
        offset: *mut u32,
    ),
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        start_slot: u32,
        num_buffers: u32,
        pp_constant_buffers: *mut ::windows::RawPtr,
    ),
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        pp_geometry_shader: *mut ::windows::RawPtr,
        pp_class_instances: *mut ::windows::RawPtr,
        p_num_class_instances: *mut u32,
    ),
    pub unsafe extern "system" fn(this: ::windows::RawPtr, p_topology: *mut D3D_PRIMITIVE_TOPOLOGY),
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        start_slot: u32,
        num_views: u32,
        pp_shader_resource_views: *mut ::windows::RawPtr,
    ),
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        start_slot: u32,
        num_samplers: u32,
        pp_samplers: *mut ::windows::RawPtr,
    ),
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        pp_predicate: *mut ::windows::RawPtr,
        p_predicate_value: *mut super::system_services::BOOL,
    ),
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        start_slot: u32,
        num_views: u32,
        pp_shader_resource_views: *mut ::windows::RawPtr,
    ),
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        start_slot: u32,
        num_samplers: u32,
        pp_samplers: *mut ::windows::RawPtr,
    ),
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        num_views: u32,
        pp_render_target_views: *mut ::windows::RawPtr,
        pp_depth_stencil_view: *mut ::windows::RawPtr,
    ),
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        num_rt_vs: u32,
        pp_render_target_views: *mut ::windows::RawPtr,
        pp_depth_stencil_view: *mut ::windows::RawPtr,
        uav_start_slot: u32,
        num_ua_vs: u32,
        pp_unordered_access_views: *mut ::windows::RawPtr,
    ),
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        pp_blend_state: *mut ::windows::RawPtr,
        blend_factor: *mut f32,
        p_sample_mask: *mut u32,
    ),
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        pp_depth_stencil_state: *mut ::windows::RawPtr,
        p_stencil_ref: *mut u32,
    ),
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        num_buffers: u32,
        pp_so_targets: *mut ::windows::RawPtr,
    ),
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        pp_rasterizer_state: *mut ::windows::RawPtr,
    ),
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        p_num_viewports: *mut u32,
        p_viewports: *mut D3D11_VIEWPORT,
    ),
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        p_num_rects: *mut u32,
        p_rects: *mut super::display_devices::RECT,
    ),
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        start_slot: u32,
        num_views: u32,
        pp_shader_resource_views: *mut ::windows::RawPtr,
    ),
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        pp_hull_shader: *mut ::windows::RawPtr,
        pp_class_instances: *mut ::windows::RawPtr,
        p_num_class_instances: *mut u32,
    ),
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        start_slot: u32,
        num_samplers: u32,
        pp_samplers: *mut ::windows::RawPtr,
    ),
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        start_slot: u32,
        num_buffers: u32,
        pp_constant_buffers: *mut ::windows::RawPtr,
    ),
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        start_slot: u32,
        num_views: u32,
        pp_shader_resource_views: *mut ::windows::RawPtr,
    ),
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        pp_domain_shader: *mut ::windows::RawPtr,
        pp_class_instances: *mut ::windows::RawPtr,
        p_num_class_instances: *mut u32,
    ),
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        start_slot: u32,
        num_samplers: u32,
        pp_samplers: *mut ::windows::RawPtr,
    ),
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        start_slot: u32,
        num_buffers: u32,
        pp_constant_buffers: *mut ::windows::RawPtr,
    ),
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        start_slot: u32,
        num_views: u32,
        pp_shader_resource_views: *mut ::windows::RawPtr,
    ),
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        start_slot: u32,
        num_ua_vs: u32,
        pp_unordered_access_views: *mut ::windows::RawPtr,
    ),
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        pp_compute_shader: *mut ::windows::RawPtr,
        pp_class_instances: *mut ::windows::RawPtr,
        p_num_class_instances: *mut u32,
    ),
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        start_slot: u32,
        num_samplers: u32,
        pp_samplers: *mut ::windows::RawPtr,
    ),
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        start_slot: u32,
        num_buffers: u32,
        pp_constant_buffers: *mut ::windows::RawPtr,
    ),
    pub unsafe extern "system" fn(this: ::windows::RawPtr),
    pub unsafe extern "system" fn(this: ::windows::RawPtr),
    pub unsafe extern "system" fn(this: ::windows::RawPtr) -> D3D11_DEVICE_CONTEXT_TYPE,
    pub unsafe extern "system" fn(this: ::windows::RawPtr) -> u32,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        restore_deferred_context_state: super::system_services::BOOL,
        pp_command_list: *mut ::windows::RawPtr,
    ) -> ::windows::ErrorCode,
);
#[repr(C)]
#[allow(non_snake_case)]
#[derive(:: std :: clone :: Clone)]
pub struct D3D11_COUNTER_INFO {
    pub last_device_dependent_counter: D3D11_COUNTER,
    pub num_simultaneous_counters: u32,
    pub num_detectable_parallel_units: u8,
}
impl D3D11_COUNTER_INFO {}
impl ::std::default::Default for D3D11_COUNTER_INFO {
    fn default() -> Self {
        Self {
            last_device_dependent_counter: ::std::default::Default::default(),
            num_simultaneous_counters: 0,
            num_detectable_parallel_units: 0,
        }
    }
}
impl ::std::fmt::Debug for D3D11_COUNTER_INFO {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("D3D11_COUNTER_INFO")
            .field(
                "last_device_dependent_counter",
                &format_args!("{:?}", self.last_device_dependent_counter),
            )
            .field(
                "num_simultaneous_counters",
                &format_args!("{:?}", self.num_simultaneous_counters),
            )
            .field(
                "num_detectable_parallel_units",
                &format_args!("{:?}", self.num_detectable_parallel_units),
            )
            .finish()
    }
}
impl ::std::cmp::PartialEq for D3D11_COUNTER_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.last_device_dependent_counter == other.last_device_dependent_counter
            && self.num_simultaneous_counters == other.num_simultaneous_counters
            && self.num_detectable_parallel_units == other.num_detectable_parallel_units
    }
}
impl ::std::cmp::Eq for D3D11_COUNTER_INFO {}
unsafe impl ::windows::Abi for D3D11_COUNTER_INFO {
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
pub struct D3D11_COUNTER_TYPE(pub i32);
impl D3D11_COUNTER_TYPE {
    #![allow(non_upper_case_globals)]
    pub const D3D11_COUNTER_TYPE_FLOAT32: Self = Self(0i32);
    pub const D3D11_COUNTER_TYPE_UINT16: Self = Self(1i32);
    pub const D3D11_COUNTER_TYPE_UINT32: Self = Self(2i32);
    pub const D3D11_COUNTER_TYPE_UINT64: Self = Self(3i32);
}
impl ::std::convert::From<i32> for D3D11_COUNTER_TYPE {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::Abi for D3D11_COUNTER_TYPE {
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
pub struct D3D11_FEATURE(pub i32);
impl D3D11_FEATURE {
    #![allow(non_upper_case_globals)]
    pub const D3D11_FEATURE_THREADING: Self = Self(0i32);
    pub const D3D11_FEATURE_DOUBLES: Self = Self(1i32);
    pub const D3D11_FEATURE_FORMAT_SUPPORT: Self = Self(2i32);
    pub const D3D11_FEATURE_FORMAT_SUPPORT2: Self = Self(3i32);
    pub const D3D11_FEATURE_D3D10_X_HARDWARE_OPTIONS: Self = Self(4i32);
    pub const D3D11_FEATURE_D3D11_OPTIONS: Self = Self(5i32);
    pub const D3D11_FEATURE_ARCHITECTURE_INFO: Self = Self(6i32);
    pub const D3D11_FEATURE_D3D9_OPTIONS: Self = Self(7i32);
    pub const D3D11_FEATURE_SHADER_MIN_PRECISION_SUPPORT: Self = Self(8i32);
    pub const D3D11_FEATURE_D3D9_SHADOW_SUPPORT: Self = Self(9i32);
    pub const D3D11_FEATURE_D3D11_OPTIONS1: Self = Self(10i32);
    pub const D3D11_FEATURE_D3D9_SIMPLE_INSTANCING_SUPPORT: Self = Self(11i32);
    pub const D3D11_FEATURE_MARKER_SUPPORT: Self = Self(12i32);
    pub const D3D11_FEATURE_D3D9_OPTIONS1: Self = Self(13i32);
    pub const D3D11_FEATURE_D3D11_OPTIONS2: Self = Self(14i32);
    pub const D3D11_FEATURE_D3D11_OPTIONS3: Self = Self(15i32);
    pub const D3D11_FEATURE_GPU_VIRTUAL_ADDRESS_SUPPORT: Self = Self(16i32);
    pub const D3D11_FEATURE_D3D11_OPTIONS4: Self = Self(17i32);
    pub const D3D11_FEATURE_SHADER_CACHE: Self = Self(18i32);
    pub const D3D11_FEATURE_D3D11_OPTIONS5: Self = Self(19i32);
}
impl ::std::convert::From<i32> for D3D11_FEATURE {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::Abi for D3D11_FEATURE {
    type Abi = Self;
}
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct ID3D11Device(::windows::IUnknown);
impl ID3D11Device {}
unsafe impl ::windows::Interface for ID3D11Device {
    type Vtable = ID3D11Device_abi;
    const IID: ::windows::Guid = ::windows::Guid::from_values(
        3681512923,
        44151,
        20104,
        [130, 83, 129, 157, 249, 187, 241, 64],
    );
}
#[allow(non_snake_case)]
impl ID3D11Device {
    pub unsafe fn CreateBuffer(
        &self,
        p_desc: *const D3D11_BUFFER_DESC,
        p_initial_data: *const D3D11_SUBRESOURCE_DATA,
        pp_buffer: *mut ::std::option::Option<ID3D11Buffer>,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).3)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(p_desc),
            ::std::mem::transmute(p_initial_data),
            ::std::mem::transmute(pp_buffer),
        )
    }
    pub unsafe fn CreateTexture1D(
        &self,
        p_desc: *const D3D11_TEXTURE1D_DESC,
        p_initial_data: *const D3D11_SUBRESOURCE_DATA,
        pp_texture1d: *mut ::std::option::Option<ID3D11Texture1D>,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).4)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(p_desc),
            ::std::mem::transmute(p_initial_data),
            ::std::mem::transmute(pp_texture1d),
        )
    }
    pub unsafe fn CreateTexture2D(
        &self,
        p_desc: *const D3D11_TEXTURE2D_DESC,
        p_initial_data: *const D3D11_SUBRESOURCE_DATA,
        pp_texture2d: *mut ::std::option::Option<ID3D11Texture2D>,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).5)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(p_desc),
            ::std::mem::transmute(p_initial_data),
            ::std::mem::transmute(pp_texture2d),
        )
    }
    pub unsafe fn CreateTexture3D(
        &self,
        p_desc: *const D3D11_TEXTURE3D_DESC,
        p_initial_data: *const D3D11_SUBRESOURCE_DATA,
        pp_texture3d: *mut ::std::option::Option<ID3D11Texture3D>,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).6)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(p_desc),
            ::std::mem::transmute(p_initial_data),
            ::std::mem::transmute(pp_texture3d),
        )
    }
    pub unsafe fn CreateShaderResourceView<'a, T0__: ::windows::IntoParam<'a, ID3D11Resource>>(
        &self,
        p_resource: T0__,
        p_desc: *const D3D11_SHADER_RESOURCE_VIEW_DESC,
        pp_sr_view: *mut ::std::option::Option<ID3D11ShaderResourceView>,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).7)(
            ::windows::Abi::abi(self),
            p_resource.into_param().abi(),
            ::std::mem::transmute(p_desc),
            ::std::mem::transmute(pp_sr_view),
        )
    }
    pub unsafe fn CreateUnorderedAccessView<'a, T0__: ::windows::IntoParam<'a, ID3D11Resource>>(
        &self,
        p_resource: T0__,
        p_desc: *const D3D11_UNORDERED_ACCESS_VIEW_DESC,
        pp_ua_view: *mut ::std::option::Option<ID3D11UnorderedAccessView>,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).8)(
            ::windows::Abi::abi(self),
            p_resource.into_param().abi(),
            ::std::mem::transmute(p_desc),
            ::std::mem::transmute(pp_ua_view),
        )
    }
    pub unsafe fn CreateRenderTargetView<'a, T0__: ::windows::IntoParam<'a, ID3D11Resource>>(
        &self,
        p_resource: T0__,
        p_desc: *const D3D11_RENDER_TARGET_VIEW_DESC,
        pp_rt_view: *mut ::std::option::Option<ID3D11RenderTargetView>,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).9)(
            ::windows::Abi::abi(self),
            p_resource.into_param().abi(),
            ::std::mem::transmute(p_desc),
            ::std::mem::transmute(pp_rt_view),
        )
    }
    pub unsafe fn CreateDepthStencilView<'a, T0__: ::windows::IntoParam<'a, ID3D11Resource>>(
        &self,
        p_resource: T0__,
        p_desc: *const D3D11_DEPTH_STENCIL_VIEW_DESC,
        pp_depth_stencil_view: *mut ::std::option::Option<ID3D11DepthStencilView>,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).10)(
            ::windows::Abi::abi(self),
            p_resource.into_param().abi(),
            ::std::mem::transmute(p_desc),
            ::std::mem::transmute(pp_depth_stencil_view),
        )
    }
    pub unsafe fn CreateInputLayout(
        &self,
        p_input_element_descs: *const D3D11_INPUT_ELEMENT_DESC,
        num_elements: u32,
        p_shader_bytecode_with_input_signature: *const ::std::ffi::c_void,
        bytecode_length: usize,
        pp_input_layout: *mut ::std::option::Option<ID3D11InputLayout>,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).11)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(p_input_element_descs),
            ::std::mem::transmute(num_elements),
            ::std::mem::transmute(p_shader_bytecode_with_input_signature),
            ::std::mem::transmute(bytecode_length),
            ::std::mem::transmute(pp_input_layout),
        )
    }
    pub unsafe fn CreateVertexShader<'a, T2__: ::windows::IntoParam<'a, ID3D11ClassLinkage>>(
        &self,
        p_shader_bytecode: *const ::std::ffi::c_void,
        bytecode_length: usize,
        p_class_linkage: T2__,
        pp_vertex_shader: *mut ::std::option::Option<ID3D11VertexShader>,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).12)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(p_shader_bytecode),
            ::std::mem::transmute(bytecode_length),
            p_class_linkage.into_param().abi(),
            ::std::mem::transmute(pp_vertex_shader),
        )
    }
    pub unsafe fn CreateGeometryShader<'a, T2__: ::windows::IntoParam<'a, ID3D11ClassLinkage>>(
        &self,
        p_shader_bytecode: *const ::std::ffi::c_void,
        bytecode_length: usize,
        p_class_linkage: T2__,
        pp_geometry_shader: *mut ::std::option::Option<ID3D11GeometryShader>,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).13)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(p_shader_bytecode),
            ::std::mem::transmute(bytecode_length),
            p_class_linkage.into_param().abi(),
            ::std::mem::transmute(pp_geometry_shader),
        )
    }
    pub unsafe fn CreateGeometryShaderWithStreamOutput<
        'a,
        T7__: ::windows::IntoParam<'a, ID3D11ClassLinkage>,
    >(
        &self,
        p_shader_bytecode: *const ::std::ffi::c_void,
        bytecode_length: usize,
        p_so_declaration: *const D3D11_SO_DECLARATION_ENTRY,
        num_entries: u32,
        p_buffer_strides: *const u32,
        num_strides: u32,
        rasterized_stream: u32,
        p_class_linkage: T7__,
        pp_geometry_shader: *mut ::std::option::Option<ID3D11GeometryShader>,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).14)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(p_shader_bytecode),
            ::std::mem::transmute(bytecode_length),
            ::std::mem::transmute(p_so_declaration),
            ::std::mem::transmute(num_entries),
            ::std::mem::transmute(p_buffer_strides),
            ::std::mem::transmute(num_strides),
            ::std::mem::transmute(rasterized_stream),
            p_class_linkage.into_param().abi(),
            ::std::mem::transmute(pp_geometry_shader),
        )
    }
    pub unsafe fn CreatePixelShader<'a, T2__: ::windows::IntoParam<'a, ID3D11ClassLinkage>>(
        &self,
        p_shader_bytecode: *const ::std::ffi::c_void,
        bytecode_length: usize,
        p_class_linkage: T2__,
        pp_pixel_shader: *mut ::std::option::Option<ID3D11PixelShader>,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).15)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(p_shader_bytecode),
            ::std::mem::transmute(bytecode_length),
            p_class_linkage.into_param().abi(),
            ::std::mem::transmute(pp_pixel_shader),
        )
    }
    pub unsafe fn CreateHullShader<'a, T2__: ::windows::IntoParam<'a, ID3D11ClassLinkage>>(
        &self,
        p_shader_bytecode: *const ::std::ffi::c_void,
        bytecode_length: usize,
        p_class_linkage: T2__,
        pp_hull_shader: *mut ::std::option::Option<ID3D11HullShader>,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).16)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(p_shader_bytecode),
            ::std::mem::transmute(bytecode_length),
            p_class_linkage.into_param().abi(),
            ::std::mem::transmute(pp_hull_shader),
        )
    }
    pub unsafe fn CreateDomainShader<'a, T2__: ::windows::IntoParam<'a, ID3D11ClassLinkage>>(
        &self,
        p_shader_bytecode: *const ::std::ffi::c_void,
        bytecode_length: usize,
        p_class_linkage: T2__,
        pp_domain_shader: *mut ::std::option::Option<ID3D11DomainShader>,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).17)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(p_shader_bytecode),
            ::std::mem::transmute(bytecode_length),
            p_class_linkage.into_param().abi(),
            ::std::mem::transmute(pp_domain_shader),
        )
    }
    pub unsafe fn CreateComputeShader<'a, T2__: ::windows::IntoParam<'a, ID3D11ClassLinkage>>(
        &self,
        p_shader_bytecode: *const ::std::ffi::c_void,
        bytecode_length: usize,
        p_class_linkage: T2__,
        pp_compute_shader: *mut ::std::option::Option<ID3D11ComputeShader>,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).18)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(p_shader_bytecode),
            ::std::mem::transmute(bytecode_length),
            p_class_linkage.into_param().abi(),
            ::std::mem::transmute(pp_compute_shader),
        )
    }
    pub unsafe fn CreateClassLinkage(
        &self,
        pp_linkage: *mut ::std::option::Option<ID3D11ClassLinkage>,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).19)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(pp_linkage),
        )
    }
    pub unsafe fn CreateBlendState(
        &self,
        p_blend_state_desc: *const D3D11_BLEND_DESC,
        pp_blend_state: *mut ::std::option::Option<ID3D11BlendState>,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).20)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(p_blend_state_desc),
            ::std::mem::transmute(pp_blend_state),
        )
    }
    pub unsafe fn CreateDepthStencilState(
        &self,
        p_depth_stencil_desc: *const D3D11_DEPTH_STENCIL_DESC,
        pp_depth_stencil_state: *mut ::std::option::Option<ID3D11DepthStencilState>,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).21)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(p_depth_stencil_desc),
            ::std::mem::transmute(pp_depth_stencil_state),
        )
    }
    pub unsafe fn CreateRasterizerState(
        &self,
        p_rasterizer_desc: *const D3D11_RASTERIZER_DESC,
        pp_rasterizer_state: *mut ::std::option::Option<ID3D11RasterizerState>,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).22)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(p_rasterizer_desc),
            ::std::mem::transmute(pp_rasterizer_state),
        )
    }
    pub unsafe fn CreateSamplerState(
        &self,
        p_sampler_desc: *const D3D11_SAMPLER_DESC,
        pp_sampler_state: *mut ::std::option::Option<ID3D11SamplerState>,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).23)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(p_sampler_desc),
            ::std::mem::transmute(pp_sampler_state),
        )
    }
    pub unsafe fn CreateQuery(
        &self,
        p_query_desc: *const D3D11_QUERY_DESC,
        pp_query: *mut ::std::option::Option<ID3D11Query>,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).24)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(p_query_desc),
            ::std::mem::transmute(pp_query),
        )
    }
    pub unsafe fn CreatePredicate(
        &self,
        p_predicate_desc: *const D3D11_QUERY_DESC,
        pp_predicate: *mut ::std::option::Option<ID3D11Predicate>,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).25)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(p_predicate_desc),
            ::std::mem::transmute(pp_predicate),
        )
    }
    pub unsafe fn CreateCounter(
        &self,
        p_counter_desc: *const D3D11_COUNTER_DESC,
        pp_counter: *mut ::std::option::Option<ID3D11Counter>,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).26)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(p_counter_desc),
            ::std::mem::transmute(pp_counter),
        )
    }
    pub unsafe fn CreateDeferredContext(
        &self,
        context_flags: u32,
        pp_deferred_context: *mut ::std::option::Option<ID3D11DeviceContext>,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).27)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(context_flags),
            ::std::mem::transmute(pp_deferred_context),
        )
    }
    pub unsafe fn OpenSharedResource<
        'a,
        T0__: ::windows::IntoParam<'a, super::system_services::HANDLE>,
    >(
        &self,
        h_resource: T0__,
        returned_interface: *const ::windows::Guid,
        pp_resource: *mut *mut ::std::ffi::c_void,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).28)(
            ::windows::Abi::abi(self),
            h_resource.into_param().abi(),
            ::std::mem::transmute(returned_interface),
            ::std::mem::transmute(pp_resource),
        )
    }
    pub unsafe fn CheckFormatSupport(
        &self,
        format: super::dxgi::DXGI_FORMAT,
        p_format_support: *mut u32,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).29)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(format),
            ::std::mem::transmute(p_format_support),
        )
    }
    pub unsafe fn CheckMultisampleQualityLevels(
        &self,
        format: super::dxgi::DXGI_FORMAT,
        sample_count: u32,
        p_num_quality_levels: *mut u32,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).30)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(format),
            ::std::mem::transmute(sample_count),
            ::std::mem::transmute(p_num_quality_levels),
        )
    }
    pub unsafe fn CheckCounterInfo(&self, p_counter_info: *mut D3D11_COUNTER_INFO) {
        (::windows::Interface::vtable(self).31)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(p_counter_info),
        )
    }
    pub unsafe fn CheckCounter(
        &self,
        p_desc: *const D3D11_COUNTER_DESC,
        p_type: *mut D3D11_COUNTER_TYPE,
        p_active_counters: *mut u32,
        sz_name: super::system_services::PSTR,
        p_name_length: *mut u32,
        sz_units: super::system_services::PSTR,
        p_units_length: *mut u32,
        sz_description: super::system_services::PSTR,
        p_description_length: *mut u32,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).32)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(p_desc),
            ::std::mem::transmute(p_type),
            ::std::mem::transmute(p_active_counters),
            ::std::mem::transmute(sz_name),
            ::std::mem::transmute(p_name_length),
            ::std::mem::transmute(sz_units),
            ::std::mem::transmute(p_units_length),
            ::std::mem::transmute(sz_description),
            ::std::mem::transmute(p_description_length),
        )
    }
    pub unsafe fn CheckFeatureSupport(
        &self,
        feature: D3D11_FEATURE,
        p_feature_support_data: *mut ::std::ffi::c_void,
        feature_support_data_size: u32,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).33)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(feature),
            ::std::mem::transmute(p_feature_support_data),
            ::std::mem::transmute(feature_support_data_size),
        )
    }
    pub unsafe fn GetPrivateData(
        &self,
        guid: *const ::windows::Guid,
        p_data_size: *mut u32,
        p_data: *mut ::std::ffi::c_void,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).34)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(guid),
            ::std::mem::transmute(p_data_size),
            ::std::mem::transmute(p_data),
        )
    }
    pub unsafe fn SetPrivateData(
        &self,
        guid: *const ::windows::Guid,
        data_size: u32,
        p_data: *const ::std::ffi::c_void,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).35)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(guid),
            ::std::mem::transmute(data_size),
            ::std::mem::transmute(p_data),
        )
    }
    pub unsafe fn SetPrivateDataInterface<
        'a,
        T1__: ::windows::IntoParam<'a, ::windows::IUnknown>,
    >(
        &self,
        guid: *const ::windows::Guid,
        p_data: T1__,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).36)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(guid),
            p_data.into_param().abi(),
        )
    }
    pub unsafe fn GetFeatureLevel(&self) -> D3D_FEATURE_LEVEL {
        (::windows::Interface::vtable(self).37)(::windows::Abi::abi(self))
    }
    pub unsafe fn GetCreationFlags(&self) -> u32 {
        (::windows::Interface::vtable(self).38)(::windows::Abi::abi(self))
    }
    pub unsafe fn GetDeviceRemovedReason(&self) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).39)(::windows::Abi::abi(self))
    }
    pub unsafe fn GetImmediateContext(
        &self,
        pp_immediate_context: *mut ::std::option::Option<ID3D11DeviceContext>,
    ) {
        (::windows::Interface::vtable(self).40)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(pp_immediate_context),
        )
    }
    pub unsafe fn SetExceptionMode(&self, raise_flags: u32) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).41)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(raise_flags),
        )
    }
    pub unsafe fn GetExceptionMode(&self) -> u32 {
        (::windows::Interface::vtable(self).42)(::windows::Abi::abi(self))
    }
}
impl ::std::convert::From<ID3D11Device> for ::windows::IUnknown {
    fn from(value: ID3D11Device) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&ID3D11Device> for ::windows::IUnknown {
    fn from(value: &ID3D11Device) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::IntoParam<'a, ::windows::IUnknown> for ID3D11Device {
    fn into_param(self) -> ::windows::Param<'a, ::windows::IUnknown> {
        ::windows::Param::Owned(::std::convert::Into::<::windows::IUnknown>::into(self))
    }
}
impl<'a> ::windows::IntoParam<'a, ::windows::IUnknown> for &'a ID3D11Device {
    fn into_param(self) -> ::windows::Param<'a, ::windows::IUnknown> {
        ::windows::Param::Owned(::std::convert::Into::<::windows::IUnknown>::into(
            ::std::clone::Clone::clone(self),
        ))
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ID3D11Device_abi(
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        iid: &::windows::Guid,
        interface: *mut ::windows::RawPtr,
    ) -> ::windows::ErrorCode,
    pub unsafe extern "system" fn(this: ::windows::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::RawPtr) -> u32,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        p_desc: *const D3D11_BUFFER_DESC,
        p_initial_data: *const D3D11_SUBRESOURCE_DATA,
        pp_buffer: *mut ::windows::RawPtr,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        p_desc: *const D3D11_TEXTURE1D_DESC,
        p_initial_data: *const D3D11_SUBRESOURCE_DATA,
        pp_texture1d: *mut ::windows::RawPtr,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        p_desc: *const D3D11_TEXTURE2D_DESC,
        p_initial_data: *const D3D11_SUBRESOURCE_DATA,
        pp_texture2d: *mut ::windows::RawPtr,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        p_desc: *const D3D11_TEXTURE3D_DESC,
        p_initial_data: *const D3D11_SUBRESOURCE_DATA,
        pp_texture3d: *mut ::windows::RawPtr,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        p_resource: ::windows::RawPtr,
        p_desc: *const D3D11_SHADER_RESOURCE_VIEW_DESC,
        pp_sr_view: *mut ::windows::RawPtr,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        p_resource: ::windows::RawPtr,
        p_desc: *const D3D11_UNORDERED_ACCESS_VIEW_DESC,
        pp_ua_view: *mut ::windows::RawPtr,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        p_resource: ::windows::RawPtr,
        p_desc: *const D3D11_RENDER_TARGET_VIEW_DESC,
        pp_rt_view: *mut ::windows::RawPtr,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        p_resource: ::windows::RawPtr,
        p_desc: *const D3D11_DEPTH_STENCIL_VIEW_DESC,
        pp_depth_stencil_view: *mut ::windows::RawPtr,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        p_input_element_descs: *const D3D11_INPUT_ELEMENT_DESC,
        num_elements: u32,
        p_shader_bytecode_with_input_signature: *const ::std::ffi::c_void,
        bytecode_length: usize,
        pp_input_layout: *mut ::windows::RawPtr,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        p_shader_bytecode: *const ::std::ffi::c_void,
        bytecode_length: usize,
        p_class_linkage: ::windows::RawPtr,
        pp_vertex_shader: *mut ::windows::RawPtr,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        p_shader_bytecode: *const ::std::ffi::c_void,
        bytecode_length: usize,
        p_class_linkage: ::windows::RawPtr,
        pp_geometry_shader: *mut ::windows::RawPtr,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        p_shader_bytecode: *const ::std::ffi::c_void,
        bytecode_length: usize,
        p_so_declaration: *const D3D11_SO_DECLARATION_ENTRY,
        num_entries: u32,
        p_buffer_strides: *const u32,
        num_strides: u32,
        rasterized_stream: u32,
        p_class_linkage: ::windows::RawPtr,
        pp_geometry_shader: *mut ::windows::RawPtr,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        p_shader_bytecode: *const ::std::ffi::c_void,
        bytecode_length: usize,
        p_class_linkage: ::windows::RawPtr,
        pp_pixel_shader: *mut ::windows::RawPtr,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        p_shader_bytecode: *const ::std::ffi::c_void,
        bytecode_length: usize,
        p_class_linkage: ::windows::RawPtr,
        pp_hull_shader: *mut ::windows::RawPtr,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        p_shader_bytecode: *const ::std::ffi::c_void,
        bytecode_length: usize,
        p_class_linkage: ::windows::RawPtr,
        pp_domain_shader: *mut ::windows::RawPtr,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        p_shader_bytecode: *const ::std::ffi::c_void,
        bytecode_length: usize,
        p_class_linkage: ::windows::RawPtr,
        pp_compute_shader: *mut ::windows::RawPtr,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        pp_linkage: *mut ::windows::RawPtr,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        p_blend_state_desc: *const D3D11_BLEND_DESC,
        pp_blend_state: *mut ::windows::RawPtr,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        p_depth_stencil_desc: *const D3D11_DEPTH_STENCIL_DESC,
        pp_depth_stencil_state: *mut ::windows::RawPtr,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        p_rasterizer_desc: *const D3D11_RASTERIZER_DESC,
        pp_rasterizer_state: *mut ::windows::RawPtr,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        p_sampler_desc: *const D3D11_SAMPLER_DESC,
        pp_sampler_state: *mut ::windows::RawPtr,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        p_query_desc: *const D3D11_QUERY_DESC,
        pp_query: *mut ::windows::RawPtr,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        p_predicate_desc: *const D3D11_QUERY_DESC,
        pp_predicate: *mut ::windows::RawPtr,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        p_counter_desc: *const D3D11_COUNTER_DESC,
        pp_counter: *mut ::windows::RawPtr,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        context_flags: u32,
        pp_deferred_context: *mut ::windows::RawPtr,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        h_resource: super::system_services::HANDLE,
        returned_interface: *const ::windows::Guid,
        pp_resource: *mut *mut ::std::ffi::c_void,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        format: super::dxgi::DXGI_FORMAT,
        p_format_support: *mut u32,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        format: super::dxgi::DXGI_FORMAT,
        sample_count: u32,
        p_num_quality_levels: *mut u32,
    ) -> ::windows::ErrorCode,
    pub unsafe extern "system" fn(this: ::windows::RawPtr, p_counter_info: *mut D3D11_COUNTER_INFO),
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        p_desc: *const D3D11_COUNTER_DESC,
        p_type: *mut D3D11_COUNTER_TYPE,
        p_active_counters: *mut u32,
        sz_name: super::system_services::PSTR,
        p_name_length: *mut u32,
        sz_units: super::system_services::PSTR,
        p_units_length: *mut u32,
        sz_description: super::system_services::PSTR,
        p_description_length: *mut u32,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        feature: D3D11_FEATURE,
        p_feature_support_data: *mut ::std::ffi::c_void,
        feature_support_data_size: u32,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        guid: *const ::windows::Guid,
        p_data_size: *mut u32,
        p_data: *mut ::std::ffi::c_void,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        guid: *const ::windows::Guid,
        data_size: u32,
        p_data: *const ::std::ffi::c_void,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        guid: *const ::windows::Guid,
        p_data: ::windows::RawPtr,
    ) -> ::windows::ErrorCode,
    pub unsafe extern "system" fn(this: ::windows::RawPtr) -> D3D_FEATURE_LEVEL,
    pub unsafe extern "system" fn(this: ::windows::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::RawPtr) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        pp_immediate_context: *mut ::windows::RawPtr,
    ),
    pub unsafe extern "system" fn(this: ::windows::RawPtr, raise_flags: u32) -> ::windows::ErrorCode,
    pub unsafe extern "system" fn(this: ::windows::RawPtr) -> u32,
);
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct ID3DBlob(::windows::IUnknown);
impl ID3DBlob {}
unsafe impl ::windows::Interface for ID3DBlob {
    type Vtable = ID3DBlob_abi;
    const IID: ::windows::Guid =
        ::windows::Guid::from_values(2342910728, 20885, 16610, [172, 88, 13, 152, 156, 58, 1, 2]);
}
#[allow(non_snake_case)]
impl ID3DBlob {
    pub unsafe fn GetBufferPointer(&self) -> *mut ::std::ffi::c_void {
        (::windows::Interface::vtable(self).3)(::windows::Abi::abi(self))
    }
    pub unsafe fn GetBufferSize(&self) -> usize {
        (::windows::Interface::vtable(self).4)(::windows::Abi::abi(self))
    }
}
impl ::std::convert::From<ID3DBlob> for ::windows::IUnknown {
    fn from(value: ID3DBlob) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&ID3DBlob> for ::windows::IUnknown {
    fn from(value: &ID3DBlob) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::IntoParam<'a, ::windows::IUnknown> for ID3DBlob {
    fn into_param(self) -> ::windows::Param<'a, ::windows::IUnknown> {
        ::windows::Param::Owned(::std::convert::Into::<::windows::IUnknown>::into(self))
    }
}
impl<'a> ::windows::IntoParam<'a, ::windows::IUnknown> for &'a ID3DBlob {
    fn into_param(self) -> ::windows::Param<'a, ::windows::IUnknown> {
        ::windows::Param::Owned(::std::convert::Into::<::windows::IUnknown>::into(
            ::std::clone::Clone::clone(self),
        ))
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ID3DBlob_abi(
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        iid: &::windows::Guid,
        interface: *mut ::windows::RawPtr,
    ) -> ::windows::ErrorCode,
    pub unsafe extern "system" fn(this: ::windows::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::RawPtr) -> *mut ::std::ffi::c_void,
    pub unsafe extern "system" fn(this: ::windows::RawPtr) -> usize,
);
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
pub struct D3D_SHADER_VARIABLE_TYPE(pub i32);
impl D3D_SHADER_VARIABLE_TYPE {
    #![allow(non_upper_case_globals)]
    pub const D3D_SVT_VOID: Self = Self(0i32);
    pub const D3D_SVT_BOOL: Self = Self(1i32);
    pub const D3D_SVT_INT: Self = Self(2i32);
    pub const D3D_SVT_FLOAT: Self = Self(3i32);
    pub const D3D_SVT_STRING: Self = Self(4i32);
    pub const D3D_SVT_TEXTURE: Self = Self(5i32);
    pub const D3D_SVT_TEXTURE1D: Self = Self(6i32);
    pub const D3D_SVT_TEXTURE2D: Self = Self(7i32);
    pub const D3D_SVT_TEXTURE3D: Self = Self(8i32);
    pub const D3D_SVT_TEXTURECUBE: Self = Self(9i32);
    pub const D3D_SVT_SAMPLER: Self = Self(10i32);
    pub const D3D_SVT_SAMPLER1D: Self = Self(11i32);
    pub const D3D_SVT_SAMPLER2D: Self = Self(12i32);
    pub const D3D_SVT_SAMPLER3D: Self = Self(13i32);
    pub const D3D_SVT_SAMPLERCUBE: Self = Self(14i32);
    pub const D3D_SVT_PIXELSHADER: Self = Self(15i32);
    pub const D3D_SVT_VERTEXSHADER: Self = Self(16i32);
    pub const D3D_SVT_PIXELFRAGMENT: Self = Self(17i32);
    pub const D3D_SVT_VERTEXFRAGMENT: Self = Self(18i32);
    pub const D3D_SVT_UINT: Self = Self(19i32);
    pub const D3D_SVT_UINT8: Self = Self(20i32);
    pub const D3D_SVT_GEOMETRYSHADER: Self = Self(21i32);
    pub const D3D_SVT_RASTERIZER: Self = Self(22i32);
    pub const D3D_SVT_DEPTHSTENCIL: Self = Self(23i32);
    pub const D3D_SVT_BLEND: Self = Self(24i32);
    pub const D3D_SVT_BUFFER: Self = Self(25i32);
    pub const D3D_SVT_CBUFFER: Self = Self(26i32);
    pub const D3D_SVT_TBUFFER: Self = Self(27i32);
    pub const D3D_SVT_TEXTURE1DARRAY: Self = Self(28i32);
    pub const D3D_SVT_TEXTURE2DARRAY: Self = Self(29i32);
    pub const D3D_SVT_RENDERTARGETVIEW: Self = Self(30i32);
    pub const D3D_SVT_DEPTHSTENCILVIEW: Self = Self(31i32);
    pub const D3D_SVT_TEXTURE2DMS: Self = Self(32i32);
    pub const D3D_SVT_TEXTURE2DMSARRAY: Self = Self(33i32);
    pub const D3D_SVT_TEXTURECUBEARRAY: Self = Self(34i32);
    pub const D3D_SVT_HULLSHADER: Self = Self(35i32);
    pub const D3D_SVT_DOMAINSHADER: Self = Self(36i32);
    pub const D3D_SVT_INTERFACE_POINTER: Self = Self(37i32);
    pub const D3D_SVT_COMPUTESHADER: Self = Self(38i32);
    pub const D3D_SVT_DOUBLE: Self = Self(39i32);
    pub const D3D_SVT_RWTEXTURE1D: Self = Self(40i32);
    pub const D3D_SVT_RWTEXTURE1DARRAY: Self = Self(41i32);
    pub const D3D_SVT_RWTEXTURE2D: Self = Self(42i32);
    pub const D3D_SVT_RWTEXTURE2DARRAY: Self = Self(43i32);
    pub const D3D_SVT_RWTEXTURE3D: Self = Self(44i32);
    pub const D3D_SVT_RWBUFFER: Self = Self(45i32);
    pub const D3D_SVT_BYTEADDRESS_BUFFER: Self = Self(46i32);
    pub const D3D_SVT_RWBYTEADDRESS_BUFFER: Self = Self(47i32);
    pub const D3D_SVT_STRUCTURED_BUFFER: Self = Self(48i32);
    pub const D3D_SVT_RWSTRUCTURED_BUFFER: Self = Self(49i32);
    pub const D3D_SVT_APPEND_STRUCTURED_BUFFER: Self = Self(50i32);
    pub const D3D_SVT_CONSUME_STRUCTURED_BUFFER: Self = Self(51i32);
    pub const D3D_SVT_MIN8FLOAT: Self = Self(52i32);
    pub const D3D_SVT_MIN10FLOAT: Self = Self(53i32);
    pub const D3D_SVT_MIN16FLOAT: Self = Self(54i32);
    pub const D3D_SVT_MIN12INT: Self = Self(55i32);
    pub const D3D_SVT_MIN16INT: Self = Self(56i32);
    pub const D3D_SVT_MIN16UINT: Self = Self(57i32);
    pub const D3D10_SVT_VOID: Self = Self(0i32);
    pub const D3D10_SVT_BOOL: Self = Self(1i32);
    pub const D3D10_SVT_INT: Self = Self(2i32);
    pub const D3D10_SVT_FLOAT: Self = Self(3i32);
    pub const D3D10_SVT_STRING: Self = Self(4i32);
    pub const D3D10_SVT_TEXTURE: Self = Self(5i32);
    pub const D3D10_SVT_TEXTURE1D: Self = Self(6i32);
    pub const D3D10_SVT_TEXTURE2D: Self = Self(7i32);
    pub const D3D10_SVT_TEXTURE3D: Self = Self(8i32);
    pub const D3D10_SVT_TEXTURECUBE: Self = Self(9i32);
    pub const D3D10_SVT_SAMPLER: Self = Self(10i32);
    pub const D3D10_SVT_SAMPLER1D: Self = Self(11i32);
    pub const D3D10_SVT_SAMPLER2D: Self = Self(12i32);
    pub const D3D10_SVT_SAMPLER3D: Self = Self(13i32);
    pub const D3D10_SVT_SAMPLERCUBE: Self = Self(14i32);
    pub const D3D10_SVT_PIXELSHADER: Self = Self(15i32);
    pub const D3D10_SVT_VERTEXSHADER: Self = Self(16i32);
    pub const D3D10_SVT_PIXELFRAGMENT: Self = Self(17i32);
    pub const D3D10_SVT_VERTEXFRAGMENT: Self = Self(18i32);
    pub const D3D10_SVT_UINT: Self = Self(19i32);
    pub const D3D10_SVT_UINT8: Self = Self(20i32);
    pub const D3D10_SVT_GEOMETRYSHADER: Self = Self(21i32);
    pub const D3D10_SVT_RASTERIZER: Self = Self(22i32);
    pub const D3D10_SVT_DEPTHSTENCIL: Self = Self(23i32);
    pub const D3D10_SVT_BLEND: Self = Self(24i32);
    pub const D3D10_SVT_BUFFER: Self = Self(25i32);
    pub const D3D10_SVT_CBUFFER: Self = Self(26i32);
    pub const D3D10_SVT_TBUFFER: Self = Self(27i32);
    pub const D3D10_SVT_TEXTURE1DARRAY: Self = Self(28i32);
    pub const D3D10_SVT_TEXTURE2DARRAY: Self = Self(29i32);
    pub const D3D10_SVT_RENDERTARGETVIEW: Self = Self(30i32);
    pub const D3D10_SVT_DEPTHSTENCILVIEW: Self = Self(31i32);
    pub const D3D10_SVT_TEXTURE2DMS: Self = Self(32i32);
    pub const D3D10_SVT_TEXTURE2DMSARRAY: Self = Self(33i32);
    pub const D3D10_SVT_TEXTURECUBEARRAY: Self = Self(34i32);
    pub const D3D11_SVT_HULLSHADER: Self = Self(35i32);
    pub const D3D11_SVT_DOMAINSHADER: Self = Self(36i32);
    pub const D3D11_SVT_INTERFACE_POINTER: Self = Self(37i32);
    pub const D3D11_SVT_COMPUTESHADER: Self = Self(38i32);
    pub const D3D11_SVT_DOUBLE: Self = Self(39i32);
    pub const D3D11_SVT_RWTEXTURE1D: Self = Self(40i32);
    pub const D3D11_SVT_RWTEXTURE1DARRAY: Self = Self(41i32);
    pub const D3D11_SVT_RWTEXTURE2D: Self = Self(42i32);
    pub const D3D11_SVT_RWTEXTURE2DARRAY: Self = Self(43i32);
    pub const D3D11_SVT_RWTEXTURE3D: Self = Self(44i32);
    pub const D3D11_SVT_RWBUFFER: Self = Self(45i32);
    pub const D3D11_SVT_BYTEADDRESS_BUFFER: Self = Self(46i32);
    pub const D3D11_SVT_RWBYTEADDRESS_BUFFER: Self = Self(47i32);
    pub const D3D11_SVT_STRUCTURED_BUFFER: Self = Self(48i32);
    pub const D3D11_SVT_RWSTRUCTURED_BUFFER: Self = Self(49i32);
    pub const D3D11_SVT_APPEND_STRUCTURED_BUFFER: Self = Self(50i32);
    pub const D3D11_SVT_CONSUME_STRUCTURED_BUFFER: Self = Self(51i32);
    pub const D3D_SVT_FORCE_DWORD: Self = Self(2147483647i32);
}
impl ::std::convert::From<i32> for D3D_SHADER_VARIABLE_TYPE {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::Abi for D3D_SHADER_VARIABLE_TYPE {
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
pub struct D3D_SHADER_VARIABLE_CLASS(pub i32);
impl D3D_SHADER_VARIABLE_CLASS {
    #![allow(non_upper_case_globals)]
    pub const D3D_SVC_SCALAR: Self = Self(0i32);
    pub const D3D_SVC_VECTOR: Self = Self(1i32);
    pub const D3D_SVC_MATRIX_ROWS: Self = Self(2i32);
    pub const D3D_SVC_MATRIX_COLUMNS: Self = Self(3i32);
    pub const D3D_SVC_OBJECT: Self = Self(4i32);
    pub const D3D_SVC_STRUCT: Self = Self(5i32);
    pub const D3D_SVC_INTERFACE_CLASS: Self = Self(6i32);
    pub const D3D_SVC_INTERFACE_POINTER: Self = Self(7i32);
    pub const D3D10_SVC_SCALAR: Self = Self(0i32);
    pub const D3D10_SVC_VECTOR: Self = Self(1i32);
    pub const D3D10_SVC_MATRIX_ROWS: Self = Self(2i32);
    pub const D3D10_SVC_MATRIX_COLUMNS: Self = Self(3i32);
    pub const D3D10_SVC_OBJECT: Self = Self(4i32);
    pub const D3D10_SVC_STRUCT: Self = Self(5i32);
    pub const D3D11_SVC_INTERFACE_CLASS: Self = Self(6i32);
    pub const D3D11_SVC_INTERFACE_POINTER: Self = Self(7i32);
    pub const D3D_SVC_FORCE_DWORD: Self = Self(2147483647i32);
}
impl ::std::convert::From<i32> for D3D_SHADER_VARIABLE_CLASS {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::Abi for D3D_SHADER_VARIABLE_CLASS {
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
pub struct D3D_INTERPOLATION_MODE(pub i32);
impl D3D_INTERPOLATION_MODE {
    #![allow(non_upper_case_globals)]
    pub const D3D_INTERPOLATION_UNDEFINED: Self = Self(0i32);
    pub const D3D_INTERPOLATION_CONSTANT: Self = Self(1i32);
    pub const D3D_INTERPOLATION_LINEAR: Self = Self(2i32);
    pub const D3D_INTERPOLATION_LINEAR_CENTROID: Self = Self(3i32);
    pub const D3D_INTERPOLATION_LINEAR_NOPERSPECTIVE: Self = Self(4i32);
    pub const D3D_INTERPOLATION_LINEAR_NOPERSPECTIVE_CENTROID: Self = Self(5i32);
    pub const D3D_INTERPOLATION_LINEAR_SAMPLE: Self = Self(6i32);
    pub const D3D_INTERPOLATION_LINEAR_NOPERSPECTIVE_SAMPLE: Self = Self(7i32);
}
impl ::std::convert::From<i32> for D3D_INTERPOLATION_MODE {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::Abi for D3D_INTERPOLATION_MODE {
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
pub struct D3D_PARAMETER_FLAGS(pub i32);
impl D3D_PARAMETER_FLAGS {
    #![allow(non_upper_case_globals)]
    pub const D3D_PF_NONE: Self = Self(0i32);
    pub const D3D_PF_IN: Self = Self(1i32);
    pub const D3D_PF_OUT: Self = Self(2i32);
    pub const D3D_PF_FORCE_DWORD: Self = Self(2147483647i32);
}
impl ::std::convert::From<i32> for D3D_PARAMETER_FLAGS {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::Abi for D3D_PARAMETER_FLAGS {
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
pub struct D3D_CBUFFER_TYPE(pub i32);
impl D3D_CBUFFER_TYPE {
    #![allow(non_upper_case_globals)]
    pub const D3D_CT_CBUFFER: Self = Self(0i32);
    pub const D3D_CT_TBUFFER: Self = Self(1i32);
    pub const D3D_CT_INTERFACE_POINTERS: Self = Self(2i32);
    pub const D3D_CT_RESOURCE_BIND_INFO: Self = Self(3i32);
    pub const D3D10_CT_CBUFFER: Self = Self(0i32);
    pub const D3D10_CT_TBUFFER: Self = Self(1i32);
    pub const D3D11_CT_CBUFFER: Self = Self(0i32);
    pub const D3D11_CT_TBUFFER: Self = Self(1i32);
    pub const D3D11_CT_INTERFACE_POINTERS: Self = Self(2i32);
    pub const D3D11_CT_RESOURCE_BIND_INFO: Self = Self(3i32);
}
impl ::std::convert::From<i32> for D3D_CBUFFER_TYPE {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::Abi for D3D_CBUFFER_TYPE {
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
pub struct D3D_PRIMITIVE(pub i32);
impl D3D_PRIMITIVE {
    #![allow(non_upper_case_globals)]
    pub const D3D_PRIMITIVE_UNDEFINED: Self = Self(0i32);
    pub const D3D_PRIMITIVE_POINT: Self = Self(1i32);
    pub const D3D_PRIMITIVE_LINE: Self = Self(2i32);
    pub const D3D_PRIMITIVE_TRIANGLE: Self = Self(3i32);
    pub const D3D_PRIMITIVE_LINE_ADJ: Self = Self(6i32);
    pub const D3D_PRIMITIVE_TRIANGLE_ADJ: Self = Self(7i32);
    pub const D3D_PRIMITIVE_1_CONTROL_POINT_PATCH: Self = Self(8i32);
    pub const D3D_PRIMITIVE_2_CONTROL_POINT_PATCH: Self = Self(9i32);
    pub const D3D_PRIMITIVE_3_CONTROL_POINT_PATCH: Self = Self(10i32);
    pub const D3D_PRIMITIVE_4_CONTROL_POINT_PATCH: Self = Self(11i32);
    pub const D3D_PRIMITIVE_5_CONTROL_POINT_PATCH: Self = Self(12i32);
    pub const D3D_PRIMITIVE_6_CONTROL_POINT_PATCH: Self = Self(13i32);
    pub const D3D_PRIMITIVE_7_CONTROL_POINT_PATCH: Self = Self(14i32);
    pub const D3D_PRIMITIVE_8_CONTROL_POINT_PATCH: Self = Self(15i32);
    pub const D3D_PRIMITIVE_9_CONTROL_POINT_PATCH: Self = Self(16i32);
    pub const D3D_PRIMITIVE_10_CONTROL_POINT_PATCH: Self = Self(17i32);
    pub const D3D_PRIMITIVE_11_CONTROL_POINT_PATCH: Self = Self(18i32);
    pub const D3D_PRIMITIVE_12_CONTROL_POINT_PATCH: Self = Self(19i32);
    pub const D3D_PRIMITIVE_13_CONTROL_POINT_PATCH: Self = Self(20i32);
    pub const D3D_PRIMITIVE_14_CONTROL_POINT_PATCH: Self = Self(21i32);
    pub const D3D_PRIMITIVE_15_CONTROL_POINT_PATCH: Self = Self(22i32);
    pub const D3D_PRIMITIVE_16_CONTROL_POINT_PATCH: Self = Self(23i32);
    pub const D3D_PRIMITIVE_17_CONTROL_POINT_PATCH: Self = Self(24i32);
    pub const D3D_PRIMITIVE_18_CONTROL_POINT_PATCH: Self = Self(25i32);
    pub const D3D_PRIMITIVE_19_CONTROL_POINT_PATCH: Self = Self(26i32);
    pub const D3D_PRIMITIVE_20_CONTROL_POINT_PATCH: Self = Self(27i32);
    pub const D3D_PRIMITIVE_21_CONTROL_POINT_PATCH: Self = Self(28i32);
    pub const D3D_PRIMITIVE_22_CONTROL_POINT_PATCH: Self = Self(29i32);
    pub const D3D_PRIMITIVE_23_CONTROL_POINT_PATCH: Self = Self(30i32);
    pub const D3D_PRIMITIVE_24_CONTROL_POINT_PATCH: Self = Self(31i32);
    pub const D3D_PRIMITIVE_25_CONTROL_POINT_PATCH: Self = Self(32i32);
    pub const D3D_PRIMITIVE_26_CONTROL_POINT_PATCH: Self = Self(33i32);
    pub const D3D_PRIMITIVE_27_CONTROL_POINT_PATCH: Self = Self(34i32);
    pub const D3D_PRIMITIVE_28_CONTROL_POINT_PATCH: Self = Self(35i32);
    pub const D3D_PRIMITIVE_29_CONTROL_POINT_PATCH: Self = Self(36i32);
    pub const D3D_PRIMITIVE_30_CONTROL_POINT_PATCH: Self = Self(37i32);
    pub const D3D_PRIMITIVE_31_CONTROL_POINT_PATCH: Self = Self(38i32);
    pub const D3D_PRIMITIVE_32_CONTROL_POINT_PATCH: Self = Self(39i32);
    pub const D3D10_PRIMITIVE_UNDEFINED: Self = Self(0i32);
    pub const D3D10_PRIMITIVE_POINT: Self = Self(1i32);
    pub const D3D10_PRIMITIVE_LINE: Self = Self(2i32);
    pub const D3D10_PRIMITIVE_TRIANGLE: Self = Self(3i32);
    pub const D3D10_PRIMITIVE_LINE_ADJ: Self = Self(6i32);
    pub const D3D10_PRIMITIVE_TRIANGLE_ADJ: Self = Self(7i32);
    pub const D3D11_PRIMITIVE_UNDEFINED: Self = Self(0i32);
    pub const D3D11_PRIMITIVE_POINT: Self = Self(1i32);
    pub const D3D11_PRIMITIVE_LINE: Self = Self(2i32);
    pub const D3D11_PRIMITIVE_TRIANGLE: Self = Self(3i32);
    pub const D3D11_PRIMITIVE_LINE_ADJ: Self = Self(6i32);
    pub const D3D11_PRIMITIVE_TRIANGLE_ADJ: Self = Self(7i32);
    pub const D3D11_PRIMITIVE_1_CONTROL_POINT_PATCH: Self = Self(8i32);
    pub const D3D11_PRIMITIVE_2_CONTROL_POINT_PATCH: Self = Self(9i32);
    pub const D3D11_PRIMITIVE_3_CONTROL_POINT_PATCH: Self = Self(10i32);
    pub const D3D11_PRIMITIVE_4_CONTROL_POINT_PATCH: Self = Self(11i32);
    pub const D3D11_PRIMITIVE_5_CONTROL_POINT_PATCH: Self = Self(12i32);
    pub const D3D11_PRIMITIVE_6_CONTROL_POINT_PATCH: Self = Self(13i32);
    pub const D3D11_PRIMITIVE_7_CONTROL_POINT_PATCH: Self = Self(14i32);
    pub const D3D11_PRIMITIVE_8_CONTROL_POINT_PATCH: Self = Self(15i32);
    pub const D3D11_PRIMITIVE_9_CONTROL_POINT_PATCH: Self = Self(16i32);
    pub const D3D11_PRIMITIVE_10_CONTROL_POINT_PATCH: Self = Self(17i32);
    pub const D3D11_PRIMITIVE_11_CONTROL_POINT_PATCH: Self = Self(18i32);
    pub const D3D11_PRIMITIVE_12_CONTROL_POINT_PATCH: Self = Self(19i32);
    pub const D3D11_PRIMITIVE_13_CONTROL_POINT_PATCH: Self = Self(20i32);
    pub const D3D11_PRIMITIVE_14_CONTROL_POINT_PATCH: Self = Self(21i32);
    pub const D3D11_PRIMITIVE_15_CONTROL_POINT_PATCH: Self = Self(22i32);
    pub const D3D11_PRIMITIVE_16_CONTROL_POINT_PATCH: Self = Self(23i32);
    pub const D3D11_PRIMITIVE_17_CONTROL_POINT_PATCH: Self = Self(24i32);
    pub const D3D11_PRIMITIVE_18_CONTROL_POINT_PATCH: Self = Self(25i32);
    pub const D3D11_PRIMITIVE_19_CONTROL_POINT_PATCH: Self = Self(26i32);
    pub const D3D11_PRIMITIVE_20_CONTROL_POINT_PATCH: Self = Self(27i32);
    pub const D3D11_PRIMITIVE_21_CONTROL_POINT_PATCH: Self = Self(28i32);
    pub const D3D11_PRIMITIVE_22_CONTROL_POINT_PATCH: Self = Self(29i32);
    pub const D3D11_PRIMITIVE_23_CONTROL_POINT_PATCH: Self = Self(30i32);
    pub const D3D11_PRIMITIVE_24_CONTROL_POINT_PATCH: Self = Self(31i32);
    pub const D3D11_PRIMITIVE_25_CONTROL_POINT_PATCH: Self = Self(32i32);
    pub const D3D11_PRIMITIVE_26_CONTROL_POINT_PATCH: Self = Self(33i32);
    pub const D3D11_PRIMITIVE_27_CONTROL_POINT_PATCH: Self = Self(34i32);
    pub const D3D11_PRIMITIVE_28_CONTROL_POINT_PATCH: Self = Self(35i32);
    pub const D3D11_PRIMITIVE_29_CONTROL_POINT_PATCH: Self = Self(36i32);
    pub const D3D11_PRIMITIVE_30_CONTROL_POINT_PATCH: Self = Self(37i32);
    pub const D3D11_PRIMITIVE_31_CONTROL_POINT_PATCH: Self = Self(38i32);
    pub const D3D11_PRIMITIVE_32_CONTROL_POINT_PATCH: Self = Self(39i32);
}
impl ::std::convert::From<i32> for D3D_PRIMITIVE {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::Abi for D3D_PRIMITIVE {
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
pub struct D3D_TESSELLATOR_OUTPUT_PRIMITIVE(pub i32);
impl D3D_TESSELLATOR_OUTPUT_PRIMITIVE {
    #![allow(non_upper_case_globals)]
    pub const D3D_TESSELLATOR_OUTPUT_UNDEFINED: Self = Self(0i32);
    pub const D3D_TESSELLATOR_OUTPUT_POINT: Self = Self(1i32);
    pub const D3D_TESSELLATOR_OUTPUT_LINE: Self = Self(2i32);
    pub const D3D_TESSELLATOR_OUTPUT_TRIANGLE_CW: Self = Self(3i32);
    pub const D3D_TESSELLATOR_OUTPUT_TRIANGLE_CCW: Self = Self(4i32);
    pub const D3D11_TESSELLATOR_OUTPUT_UNDEFINED: Self = Self(0i32);
    pub const D3D11_TESSELLATOR_OUTPUT_POINT: Self = Self(1i32);
    pub const D3D11_TESSELLATOR_OUTPUT_LINE: Self = Self(2i32);
    pub const D3D11_TESSELLATOR_OUTPUT_TRIANGLE_CW: Self = Self(3i32);
    pub const D3D11_TESSELLATOR_OUTPUT_TRIANGLE_CCW: Self = Self(4i32);
}
impl ::std::convert::From<i32> for D3D_TESSELLATOR_OUTPUT_PRIMITIVE {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::Abi for D3D_TESSELLATOR_OUTPUT_PRIMITIVE {
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
pub struct D3D_TESSELLATOR_PARTITIONING(pub i32);
impl D3D_TESSELLATOR_PARTITIONING {
    #![allow(non_upper_case_globals)]
    pub const D3D_TESSELLATOR_PARTITIONING_UNDEFINED: Self = Self(0i32);
    pub const D3D_TESSELLATOR_PARTITIONING_INTEGER: Self = Self(1i32);
    pub const D3D_TESSELLATOR_PARTITIONING_POW2: Self = Self(2i32);
    pub const D3D_TESSELLATOR_PARTITIONING_FRACTIONAL_ODD: Self = Self(3i32);
    pub const D3D_TESSELLATOR_PARTITIONING_FRACTIONAL_EVEN: Self = Self(4i32);
    pub const D3D11_TESSELLATOR_PARTITIONING_UNDEFINED: Self = Self(0i32);
    pub const D3D11_TESSELLATOR_PARTITIONING_INTEGER: Self = Self(1i32);
    pub const D3D11_TESSELLATOR_PARTITIONING_POW2: Self = Self(2i32);
    pub const D3D11_TESSELLATOR_PARTITIONING_FRACTIONAL_ODD: Self = Self(3i32);
    pub const D3D11_TESSELLATOR_PARTITIONING_FRACTIONAL_EVEN: Self = Self(4i32);
}
impl ::std::convert::From<i32> for D3D_TESSELLATOR_PARTITIONING {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::Abi for D3D_TESSELLATOR_PARTITIONING {
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
pub struct D3D_TESSELLATOR_DOMAIN(pub i32);
impl D3D_TESSELLATOR_DOMAIN {
    #![allow(non_upper_case_globals)]
    pub const D3D_TESSELLATOR_DOMAIN_UNDEFINED: Self = Self(0i32);
    pub const D3D_TESSELLATOR_DOMAIN_ISOLINE: Self = Self(1i32);
    pub const D3D_TESSELLATOR_DOMAIN_TRI: Self = Self(2i32);
    pub const D3D_TESSELLATOR_DOMAIN_QUAD: Self = Self(3i32);
    pub const D3D11_TESSELLATOR_DOMAIN_UNDEFINED: Self = Self(0i32);
    pub const D3D11_TESSELLATOR_DOMAIN_ISOLINE: Self = Self(1i32);
    pub const D3D11_TESSELLATOR_DOMAIN_TRI: Self = Self(2i32);
    pub const D3D11_TESSELLATOR_DOMAIN_QUAD: Self = Self(3i32);
}
impl ::std::convert::From<i32> for D3D_TESSELLATOR_DOMAIN {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::Abi for D3D_TESSELLATOR_DOMAIN {
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
pub struct D3D_SHADER_INPUT_TYPE(pub i32);
impl D3D_SHADER_INPUT_TYPE {
    #![allow(non_upper_case_globals)]
    pub const D3D_SIT_CBUFFER: Self = Self(0i32);
    pub const D3D_SIT_TBUFFER: Self = Self(1i32);
    pub const D3D_SIT_TEXTURE: Self = Self(2i32);
    pub const D3D_SIT_SAMPLER: Self = Self(3i32);
    pub const D3D_SIT_UAV_RWTYPED: Self = Self(4i32);
    pub const D3D_SIT_STRUCTURED: Self = Self(5i32);
    pub const D3D_SIT_UAV_RWSTRUCTURED: Self = Self(6i32);
    pub const D3D_SIT_BYTEADDRESS: Self = Self(7i32);
    pub const D3D_SIT_UAV_RWBYTEADDRESS: Self = Self(8i32);
    pub const D3D_SIT_UAV_APPEND_STRUCTURED: Self = Self(9i32);
    pub const D3D_SIT_UAV_CONSUME_STRUCTURED: Self = Self(10i32);
    pub const D3D_SIT_UAV_RWSTRUCTURED_WITH_COUNTER: Self = Self(11i32);
    pub const D3D_SIT_RTACCELERATIONSTRUCTURE: Self = Self(12i32);
    pub const D3D_SIT_UAV_FEEDBACKTEXTURE: Self = Self(13i32);
    pub const D3D10_SIT_CBUFFER: Self = Self(0i32);
    pub const D3D10_SIT_TBUFFER: Self = Self(1i32);
    pub const D3D10_SIT_TEXTURE: Self = Self(2i32);
    pub const D3D10_SIT_SAMPLER: Self = Self(3i32);
    pub const D3D11_SIT_UAV_RWTYPED: Self = Self(4i32);
    pub const D3D11_SIT_STRUCTURED: Self = Self(5i32);
    pub const D3D11_SIT_UAV_RWSTRUCTURED: Self = Self(6i32);
    pub const D3D11_SIT_BYTEADDRESS: Self = Self(7i32);
    pub const D3D11_SIT_UAV_RWBYTEADDRESS: Self = Self(8i32);
    pub const D3D11_SIT_UAV_APPEND_STRUCTURED: Self = Self(9i32);
    pub const D3D11_SIT_UAV_CONSUME_STRUCTURED: Self = Self(10i32);
    pub const D3D11_SIT_UAV_RWSTRUCTURED_WITH_COUNTER: Self = Self(11i32);
}
impl ::std::convert::From<i32> for D3D_SHADER_INPUT_TYPE {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::Abi for D3D_SHADER_INPUT_TYPE {
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
pub struct D3D_RESOURCE_RETURN_TYPE(pub i32);
impl D3D_RESOURCE_RETURN_TYPE {
    #![allow(non_upper_case_globals)]
    pub const D3D_RETURN_TYPE_UNORM: Self = Self(1i32);
    pub const D3D_RETURN_TYPE_SNORM: Self = Self(2i32);
    pub const D3D_RETURN_TYPE_SINT: Self = Self(3i32);
    pub const D3D_RETURN_TYPE_UINT: Self = Self(4i32);
    pub const D3D_RETURN_TYPE_FLOAT: Self = Self(5i32);
    pub const D3D_RETURN_TYPE_MIXED: Self = Self(6i32);
    pub const D3D_RETURN_TYPE_DOUBLE: Self = Self(7i32);
    pub const D3D_RETURN_TYPE_CONTINUED: Self = Self(8i32);
    pub const D3D10_RETURN_TYPE_UNORM: Self = Self(1i32);
    pub const D3D10_RETURN_TYPE_SNORM: Self = Self(2i32);
    pub const D3D10_RETURN_TYPE_SINT: Self = Self(3i32);
    pub const D3D10_RETURN_TYPE_UINT: Self = Self(4i32);
    pub const D3D10_RETURN_TYPE_FLOAT: Self = Self(5i32);
    pub const D3D10_RETURN_TYPE_MIXED: Self = Self(6i32);
    pub const D3D11_RETURN_TYPE_UNORM: Self = Self(1i32);
    pub const D3D11_RETURN_TYPE_SNORM: Self = Self(2i32);
    pub const D3D11_RETURN_TYPE_SINT: Self = Self(3i32);
    pub const D3D11_RETURN_TYPE_UINT: Self = Self(4i32);
    pub const D3D11_RETURN_TYPE_FLOAT: Self = Self(5i32);
    pub const D3D11_RETURN_TYPE_MIXED: Self = Self(6i32);
    pub const D3D11_RETURN_TYPE_DOUBLE: Self = Self(7i32);
    pub const D3D11_RETURN_TYPE_CONTINUED: Self = Self(8i32);
}
impl ::std::convert::From<i32> for D3D_RESOURCE_RETURN_TYPE {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::Abi for D3D_RESOURCE_RETURN_TYPE {
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
pub struct D3D_NAME(pub i32);
impl D3D_NAME {
    #![allow(non_upper_case_globals)]
    pub const D3D_NAME_UNDEFINED: Self = Self(0i32);
    pub const D3D_NAME_POSITION: Self = Self(1i32);
    pub const D3D_NAME_CLIP_DISTANCE: Self = Self(2i32);
    pub const D3D_NAME_CULL_DISTANCE: Self = Self(3i32);
    pub const D3D_NAME_RENDER_TARGET_ARRAY_INDEX: Self = Self(4i32);
    pub const D3D_NAME_VIEWPORT_ARRAY_INDEX: Self = Self(5i32);
    pub const D3D_NAME_VERTEX_ID: Self = Self(6i32);
    pub const D3D_NAME_PRIMITIVE_ID: Self = Self(7i32);
    pub const D3D_NAME_INSTANCE_ID: Self = Self(8i32);
    pub const D3D_NAME_IS_FRONT_FACE: Self = Self(9i32);
    pub const D3D_NAME_SAMPLE_INDEX: Self = Self(10i32);
    pub const D3D_NAME_FINAL_QUAD_EDGE_TESSFACTOR: Self = Self(11i32);
    pub const D3D_NAME_FINAL_QUAD_INSIDE_TESSFACTOR: Self = Self(12i32);
    pub const D3D_NAME_FINAL_TRI_EDGE_TESSFACTOR: Self = Self(13i32);
    pub const D3D_NAME_FINAL_TRI_INSIDE_TESSFACTOR: Self = Self(14i32);
    pub const D3D_NAME_FINAL_LINE_DETAIL_TESSFACTOR: Self = Self(15i32);
    pub const D3D_NAME_FINAL_LINE_DENSITY_TESSFACTOR: Self = Self(16i32);
    pub const D3D_NAME_BARYCENTRICS: Self = Self(23i32);
    pub const D3D_NAME_SHADINGRATE: Self = Self(24i32);
    pub const D3D_NAME_CULLPRIMITIVE: Self = Self(25i32);
    pub const D3D_NAME_TARGET: Self = Self(64i32);
    pub const D3D_NAME_DEPTH: Self = Self(65i32);
    pub const D3D_NAME_COVERAGE: Self = Self(66i32);
    pub const D3D_NAME_DEPTH_GREATER_EQUAL: Self = Self(67i32);
    pub const D3D_NAME_DEPTH_LESS_EQUAL: Self = Self(68i32);
    pub const D3D_NAME_STENCIL_REF: Self = Self(69i32);
    pub const D3D_NAME_INNER_COVERAGE: Self = Self(70i32);
    pub const D3D10_NAME_UNDEFINED: Self = Self(0i32);
    pub const D3D10_NAME_POSITION: Self = Self(1i32);
    pub const D3D10_NAME_CLIP_DISTANCE: Self = Self(2i32);
    pub const D3D10_NAME_CULL_DISTANCE: Self = Self(3i32);
    pub const D3D10_NAME_RENDER_TARGET_ARRAY_INDEX: Self = Self(4i32);
    pub const D3D10_NAME_VIEWPORT_ARRAY_INDEX: Self = Self(5i32);
    pub const D3D10_NAME_VERTEX_ID: Self = Self(6i32);
    pub const D3D10_NAME_PRIMITIVE_ID: Self = Self(7i32);
    pub const D3D10_NAME_INSTANCE_ID: Self = Self(8i32);
    pub const D3D10_NAME_IS_FRONT_FACE: Self = Self(9i32);
    pub const D3D10_NAME_SAMPLE_INDEX: Self = Self(10i32);
    pub const D3D10_NAME_TARGET: Self = Self(64i32);
    pub const D3D10_NAME_DEPTH: Self = Self(65i32);
    pub const D3D10_NAME_COVERAGE: Self = Self(66i32);
    pub const D3D11_NAME_FINAL_QUAD_EDGE_TESSFACTOR: Self = Self(11i32);
    pub const D3D11_NAME_FINAL_QUAD_INSIDE_TESSFACTOR: Self = Self(12i32);
    pub const D3D11_NAME_FINAL_TRI_EDGE_TESSFACTOR: Self = Self(13i32);
    pub const D3D11_NAME_FINAL_TRI_INSIDE_TESSFACTOR: Self = Self(14i32);
    pub const D3D11_NAME_FINAL_LINE_DETAIL_TESSFACTOR: Self = Self(15i32);
    pub const D3D11_NAME_FINAL_LINE_DENSITY_TESSFACTOR: Self = Self(16i32);
    pub const D3D11_NAME_DEPTH_GREATER_EQUAL: Self = Self(67i32);
    pub const D3D11_NAME_DEPTH_LESS_EQUAL: Self = Self(68i32);
    pub const D3D11_NAME_STENCIL_REF: Self = Self(69i32);
    pub const D3D11_NAME_INNER_COVERAGE: Self = Self(70i32);
    pub const D3D12_NAME_BARYCENTRICS: Self = Self(23i32);
    pub const D3D12_NAME_SHADINGRATE: Self = Self(24i32);
    pub const D3D12_NAME_CULLPRIMITIVE: Self = Self(25i32);
}
impl ::std::convert::From<i32> for D3D_NAME {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::Abi for D3D_NAME {
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
pub struct D3D_REGISTER_COMPONENT_TYPE(pub i32);
impl D3D_REGISTER_COMPONENT_TYPE {
    #![allow(non_upper_case_globals)]
    pub const D3D_REGISTER_COMPONENT_UNKNOWN: Self = Self(0i32);
    pub const D3D_REGISTER_COMPONENT_UINT32: Self = Self(1i32);
    pub const D3D_REGISTER_COMPONENT_SINT32: Self = Self(2i32);
    pub const D3D_REGISTER_COMPONENT_FLOAT32: Self = Self(3i32);
    pub const D3D10_REGISTER_COMPONENT_UNKNOWN: Self = Self(0i32);
    pub const D3D10_REGISTER_COMPONENT_UINT32: Self = Self(1i32);
    pub const D3D10_REGISTER_COMPONENT_SINT32: Self = Self(2i32);
    pub const D3D10_REGISTER_COMPONENT_FLOAT32: Self = Self(3i32);
}
impl ::std::convert::From<i32> for D3D_REGISTER_COMPONENT_TYPE {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::Abi for D3D_REGISTER_COMPONENT_TYPE {
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
pub struct D3D_MIN_PRECISION(pub i32);
impl D3D_MIN_PRECISION {
    #![allow(non_upper_case_globals)]
    pub const D3D_MIN_PRECISION_DEFAULT: Self = Self(0i32);
    pub const D3D_MIN_PRECISION_FLOAT_16: Self = Self(1i32);
    pub const D3D_MIN_PRECISION_FLOAT_2_8: Self = Self(2i32);
    pub const D3D_MIN_PRECISION_RESERVED: Self = Self(3i32);
    pub const D3D_MIN_PRECISION_SINT_16: Self = Self(4i32);
    pub const D3D_MIN_PRECISION_UINT_16: Self = Self(5i32);
    pub const D3D_MIN_PRECISION_ANY_16: Self = Self(240i32);
    pub const D3D_MIN_PRECISION_ANY_10: Self = Self(241i32);
}
impl ::std::convert::From<i32> for D3D_MIN_PRECISION {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::Abi for D3D_MIN_PRECISION {
    type Abi = Self;
}
