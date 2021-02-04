#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct D3D_FEATURE_LEVEL(pub i32);
impl ::std::convert::From<i32> for D3D_FEATURE_LEVEL {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
impl ::std::clone::Clone for D3D_FEATURE_LEVEL {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}
impl ::std::default::Default for D3D_FEATURE_LEVEL {
    fn default() -> Self {
        Self(0)
    }
}
impl ::std::fmt::Debug for D3D_FEATURE_LEVEL {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        write!(f, "{:?}", self.0)
    }
}
impl ::std::cmp::PartialEq for D3D_FEATURE_LEVEL {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::std::cmp::Eq for D3D_FEATURE_LEVEL {}
impl ::std::marker::Copy for D3D_FEATURE_LEVEL {}
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
unsafe impl ::windows::Abi for D3D_FEATURE_LEVEL {
    type Abi = Self;
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct D3D11_USAGE(pub i32);
impl ::std::convert::From<i32> for D3D11_USAGE {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
impl ::std::clone::Clone for D3D11_USAGE {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}
impl ::std::default::Default for D3D11_USAGE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::std::fmt::Debug for D3D11_USAGE {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        write!(f, "{:?}", self.0)
    }
}
impl ::std::cmp::PartialEq for D3D11_USAGE {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::std::cmp::Eq for D3D11_USAGE {}
impl ::std::marker::Copy for D3D11_USAGE {}
impl D3D11_USAGE {
    #![allow(non_upper_case_globals)]
    pub const D3D11_USAGE_DEFAULT: Self = Self(0i32);
    pub const D3D11_USAGE_IMMUTABLE: Self = Self(1i32);
    pub const D3D11_USAGE_DYNAMIC: Self = Self(2i32);
    pub const D3D11_USAGE_STAGING: Self = Self(3i32);
}
unsafe impl ::windows::Abi for D3D11_USAGE {
    type Abi = Self;
}
#[repr(C)]
#[allow(non_camel_case_types)]
pub struct D3D11_BUFFER_DESC {
    pub byte_width: u32,
    pub usage: D3D11_USAGE,
    pub bind_flags: u32,
    pub cpu_access_flags: u32,
    pub misc_flags: u32,
    pub structure_byte_stride: u32,
}
#[repr(C)]
#[doc(hidden)]
#[allow(non_camel_case_types)]
pub struct D3D11_BUFFER_DESC_abi(u32, D3D11_USAGE, u32, u32, u32, u32);
impl D3D11_BUFFER_DESC {}
unsafe impl ::windows::Abi for D3D11_BUFFER_DESC {
    type Abi = D3D11_BUFFER_DESC_abi;
}
impl ::core::default::Default for D3D11_BUFFER_DESC {
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
impl ::core::fmt::Debug for D3D11_BUFFER_DESC {
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
impl ::core::clone::Clone for D3D11_BUFFER_DESC {
    fn clone(&self) -> Self {
        Self {
            byte_width: self.byte_width,
            usage: self.usage,
            bind_flags: self.bind_flags,
            cpu_access_flags: self.cpu_access_flags,
            misc_flags: self.misc_flags,
            structure_byte_stride: self.structure_byte_stride,
        }
    }
}
#[repr(C)]
#[allow(non_camel_case_types)]
pub struct D3D11_SUBRESOURCE_DATA {
    pub p_sys_mem: *mut ::std::ffi::c_void,
    pub sys_mem_pitch: u32,
    pub sys_mem_slice_pitch: u32,
}
#[repr(C)]
#[doc(hidden)]
#[allow(non_camel_case_types)]
pub struct D3D11_SUBRESOURCE_DATA_abi(*mut ::std::ffi::c_void, u32, u32);
impl D3D11_SUBRESOURCE_DATA {}
unsafe impl ::windows::Abi for D3D11_SUBRESOURCE_DATA {
    type Abi = D3D11_SUBRESOURCE_DATA_abi;
}
impl ::core::default::Default for D3D11_SUBRESOURCE_DATA {
    fn default() -> Self {
        Self {
            p_sys_mem: ::std::ptr::null_mut(),
            sys_mem_pitch: 0,
            sys_mem_slice_pitch: 0,
        }
    }
}
impl ::core::fmt::Debug for D3D11_SUBRESOURCE_DATA {
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
impl ::core::clone::Clone for D3D11_SUBRESOURCE_DATA {
    fn clone(&self) -> Self {
        Self {
            p_sys_mem: <*mut ::std::ffi::c_void as std::clone::Clone>::clone(&self.p_sys_mem),
            sys_mem_pitch: self.sys_mem_pitch,
            sys_mem_slice_pitch: self.sys_mem_slice_pitch,
        }
    }
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct D3D11_RESOURCE_DIMENSION(pub i32);
impl ::std::convert::From<i32> for D3D11_RESOURCE_DIMENSION {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
impl ::std::clone::Clone for D3D11_RESOURCE_DIMENSION {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}
impl ::std::default::Default for D3D11_RESOURCE_DIMENSION {
    fn default() -> Self {
        Self(0)
    }
}
impl ::std::fmt::Debug for D3D11_RESOURCE_DIMENSION {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        write!(f, "{:?}", self.0)
    }
}
impl ::std::cmp::PartialEq for D3D11_RESOURCE_DIMENSION {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::std::cmp::Eq for D3D11_RESOURCE_DIMENSION {}
impl ::std::marker::Copy for D3D11_RESOURCE_DIMENSION {}
impl D3D11_RESOURCE_DIMENSION {
    #![allow(non_upper_case_globals)]
    pub const D3D11_RESOURCE_DIMENSION_UNKNOWN: Self = Self(0i32);
    pub const D3D11_RESOURCE_DIMENSION_BUFFER: Self = Self(1i32);
    pub const D3D11_RESOURCE_DIMENSION_TEXTURE1D: Self = Self(2i32);
    pub const D3D11_RESOURCE_DIMENSION_TEXTURE2D: Self = Self(3i32);
    pub const D3D11_RESOURCE_DIMENSION_TEXTURE3D: Self = Self(4i32);
}
unsafe impl ::windows::Abi for D3D11_RESOURCE_DIMENSION {
    type Abi = Self;
}
#[repr(transparent)]
#[allow(non_camel_case_types)]
pub struct ID3D11DeviceChild(::windows::IUnknown);
impl ::std::clone::Clone for ID3D11DeviceChild {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::std::fmt::Debug for ID3D11DeviceChild {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        write!(f, "{:?}", self.0)
    }
}
impl ::std::cmp::PartialEq for ID3D11DeviceChild {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::std::cmp::Eq for ID3D11DeviceChild {}
unsafe impl ::windows::Interface for ID3D11DeviceChild {
    type Vtable = ID3D11DeviceChild_abi;
    const IID: ::windows::Guid = ::windows::Guid::from_values(
        406971848,
        5808,
        18587,
        [188, 200, 68, 207, 176, 213, 222, 174],
    );
}
#[repr(C)]
pub struct ID3D11DeviceChild_abi(
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        iid: &::windows::Guid,
        interface: *mut ::windows::RawPtr,
    ) -> ::windows::ErrorCode,
    pub unsafe extern "system" fn(this: ::windows::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::RawPtr) -> u32,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        pp_device: *mut ::std::option::Option<ID3D11Device>,
    ),
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
#[allow(non_snake_case)]
impl ID3D11DeviceChild {
    pub unsafe fn GetDevice(&self, pp_device: *mut ::std::option::Option<ID3D11Device>) {
        (::windows::Interface::vtable(self).3)(::windows::Abi::abi(self), pp_device)
    }
    pub unsafe fn GetPrivateData(
        &self,
        guid: *const ::windows::Guid,
        p_data_size: *mut u32,
        p_data: *mut ::std::ffi::c_void,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).4)(::windows::Abi::abi(self), guid, p_data_size, p_data)
    }
    pub unsafe fn SetPrivateData(
        &self,
        guid: *const ::windows::Guid,
        data_size: u32,
        p_data: *const ::std::ffi::c_void,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).5)(::windows::Abi::abi(self), guid, data_size, p_data)
    }
    pub unsafe fn SetPrivateDataInterface<
        'a,
        T1__: ::std::convert::Into<::windows::Param<'a, ::windows::IUnknown>>,
    >(
        &self,
        guid: *const ::windows::Guid,
        p_data: T1__,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).6)(::windows::Abi::abi(self), guid, p_data.into().abi())
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
impl<'a> ::std::convert::Into<::windows::Param<'a, ::windows::IUnknown>> for ID3D11DeviceChild {
    fn into(self) -> ::windows::Param<'a, ::windows::IUnknown> {
        ::windows::Param::Owned(::std::convert::Into::<::windows::IUnknown>::into(self))
    }
}
impl<'a> ::std::convert::Into<::windows::Param<'a, ::windows::IUnknown>> for &'a ID3D11DeviceChild {
    fn into(self) -> ::windows::Param<'a, ::windows::IUnknown> {
        ::windows::Param::Owned(::std::convert::Into::<::windows::IUnknown>::into(
            ::std::clone::Clone::clone(self),
        ))
    }
}
#[repr(transparent)]
#[allow(non_camel_case_types)]
pub struct ID3D11Resource(::windows::IUnknown);
impl ::std::clone::Clone for ID3D11Resource {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::std::fmt::Debug for ID3D11Resource {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        write!(f, "{:?}", self.0)
    }
}
impl ::std::cmp::PartialEq for ID3D11Resource {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::std::cmp::Eq for ID3D11Resource {}
unsafe impl ::windows::Interface for ID3D11Resource {
    type Vtable = ID3D11Resource_abi;
    const IID: ::windows::Guid = ::windows::Guid::from_values(
        3700319219,
        53547,
        18770,
        [180, 123, 94, 69, 2, 106, 134, 45],
    );
}
#[repr(C)]
pub struct ID3D11Resource_abi(
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        iid: &::windows::Guid,
        interface: *mut ::windows::RawPtr,
    ) -> ::windows::ErrorCode,
    pub unsafe extern "system" fn(this: ::windows::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::RawPtr) -> u32,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        pp_device: *mut ::std::option::Option<ID3D11Device>,
    ),
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
#[allow(non_snake_case)]
impl ID3D11Resource {
    pub unsafe fn GetDevice(&self, pp_device: *mut ::std::option::Option<ID3D11Device>) {
        (::windows::Interface::vtable(self).3)(::windows::Abi::abi(self), pp_device)
    }
    pub unsafe fn GetPrivateData(
        &self,
        guid: *const ::windows::Guid,
        p_data_size: *mut u32,
        p_data: *mut ::std::ffi::c_void,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).4)(::windows::Abi::abi(self), guid, p_data_size, p_data)
    }
    pub unsafe fn SetPrivateData(
        &self,
        guid: *const ::windows::Guid,
        data_size: u32,
        p_data: *const ::std::ffi::c_void,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).5)(::windows::Abi::abi(self), guid, data_size, p_data)
    }
    pub unsafe fn SetPrivateDataInterface<
        'a,
        T1__: ::std::convert::Into<::windows::Param<'a, ::windows::IUnknown>>,
    >(
        &self,
        guid: *const ::windows::Guid,
        p_data: T1__,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).6)(::windows::Abi::abi(self), guid, p_data.into().abi())
    }
    pub unsafe fn GetType(&self, p_resource_dimension: *mut D3D11_RESOURCE_DIMENSION) {
        (::windows::Interface::vtable(self).7)(::windows::Abi::abi(self), p_resource_dimension)
    }
    pub unsafe fn SetEvictionPriority(&self, eviction_priority: u32) {
        (::windows::Interface::vtable(self).8)(::windows::Abi::abi(self), eviction_priority)
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
impl<'a> ::std::convert::Into<::windows::Param<'a, ::windows::IUnknown>> for ID3D11Resource {
    fn into(self) -> ::windows::Param<'a, ::windows::IUnknown> {
        ::windows::Param::Owned(::std::convert::Into::<::windows::IUnknown>::into(self))
    }
}
impl<'a> ::std::convert::Into<::windows::Param<'a, ::windows::IUnknown>> for &'a ID3D11Resource {
    fn into(self) -> ::windows::Param<'a, ::windows::IUnknown> {
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
impl<'a> ::std::convert::Into<::windows::Param<'a, ID3D11DeviceChild>> for ID3D11Resource {
    fn into(self) -> ::windows::Param<'a, ID3D11DeviceChild> {
        ::windows::Param::Owned(::std::convert::Into::<ID3D11DeviceChild>::into(self))
    }
}
impl<'a> ::std::convert::Into<::windows::Param<'a, ID3D11DeviceChild>> for &'a ID3D11Resource {
    fn into(self) -> ::windows::Param<'a, ID3D11DeviceChild> {
        ::windows::Param::Owned(::std::convert::Into::<ID3D11DeviceChild>::into(
            ::std::clone::Clone::clone(self),
        ))
    }
}
#[repr(transparent)]
#[allow(non_camel_case_types)]
pub struct ID3D11Buffer(::windows::IUnknown);
impl ::std::clone::Clone for ID3D11Buffer {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::std::fmt::Debug for ID3D11Buffer {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        write!(f, "{:?}", self.0)
    }
}
impl ::std::cmp::PartialEq for ID3D11Buffer {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::std::cmp::Eq for ID3D11Buffer {}
unsafe impl ::windows::Interface for ID3D11Buffer {
    type Vtable = ID3D11Buffer_abi;
    const IID: ::windows::Guid =
        ::windows::Guid::from_values(1213664133, 53742, 20429, [162, 80, 235, 53, 7, 34, 176, 55]);
}
#[repr(C)]
pub struct ID3D11Buffer_abi(
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        iid: &::windows::Guid,
        interface: *mut ::windows::RawPtr,
    ) -> ::windows::ErrorCode,
    pub unsafe extern "system" fn(this: ::windows::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::RawPtr) -> u32,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        pp_device: *mut ::std::option::Option<ID3D11Device>,
    ),
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
#[allow(non_snake_case)]
impl ID3D11Buffer {
    pub unsafe fn GetDevice(&self, pp_device: *mut ::std::option::Option<ID3D11Device>) {
        (::windows::Interface::vtable(self).3)(::windows::Abi::abi(self), pp_device)
    }
    pub unsafe fn GetPrivateData(
        &self,
        guid: *const ::windows::Guid,
        p_data_size: *mut u32,
        p_data: *mut ::std::ffi::c_void,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).4)(::windows::Abi::abi(self), guid, p_data_size, p_data)
    }
    pub unsafe fn SetPrivateData(
        &self,
        guid: *const ::windows::Guid,
        data_size: u32,
        p_data: *const ::std::ffi::c_void,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).5)(::windows::Abi::abi(self), guid, data_size, p_data)
    }
    pub unsafe fn SetPrivateDataInterface<
        'a,
        T1__: ::std::convert::Into<::windows::Param<'a, ::windows::IUnknown>>,
    >(
        &self,
        guid: *const ::windows::Guid,
        p_data: T1__,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).6)(::windows::Abi::abi(self), guid, p_data.into().abi())
    }
    pub unsafe fn GetType(&self, p_resource_dimension: *mut D3D11_RESOURCE_DIMENSION) {
        (::windows::Interface::vtable(self).7)(::windows::Abi::abi(self), p_resource_dimension)
    }
    pub unsafe fn SetEvictionPriority(&self, eviction_priority: u32) {
        (::windows::Interface::vtable(self).8)(::windows::Abi::abi(self), eviction_priority)
    }
    pub unsafe fn GetEvictionPriority(&self) -> u32 {
        (::windows::Interface::vtable(self).9)(::windows::Abi::abi(self))
    }
    pub unsafe fn GetDesc(&self, p_desc: *mut D3D11_BUFFER_DESC) {
        (::windows::Interface::vtable(self).10)(::windows::Abi::abi(self), p_desc)
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
impl<'a> ::std::convert::Into<::windows::Param<'a, ::windows::IUnknown>> for ID3D11Buffer {
    fn into(self) -> ::windows::Param<'a, ::windows::IUnknown> {
        ::windows::Param::Owned(::std::convert::Into::<::windows::IUnknown>::into(self))
    }
}
impl<'a> ::std::convert::Into<::windows::Param<'a, ::windows::IUnknown>> for &'a ID3D11Buffer {
    fn into(self) -> ::windows::Param<'a, ::windows::IUnknown> {
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
impl<'a> ::std::convert::Into<::windows::Param<'a, ID3D11Resource>> for ID3D11Buffer {
    fn into(self) -> ::windows::Param<'a, ID3D11Resource> {
        ::windows::Param::Owned(::std::convert::Into::<ID3D11Resource>::into(self))
    }
}
impl<'a> ::std::convert::Into<::windows::Param<'a, ID3D11Resource>> for &'a ID3D11Buffer {
    fn into(self) -> ::windows::Param<'a, ID3D11Resource> {
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
impl<'a> ::std::convert::Into<::windows::Param<'a, ID3D11DeviceChild>> for ID3D11Buffer {
    fn into(self) -> ::windows::Param<'a, ID3D11DeviceChild> {
        ::windows::Param::Owned(::std::convert::Into::<ID3D11DeviceChild>::into(self))
    }
}
impl<'a> ::std::convert::Into<::windows::Param<'a, ID3D11DeviceChild>> for &'a ID3D11Buffer {
    fn into(self) -> ::windows::Param<'a, ID3D11DeviceChild> {
        ::windows::Param::Owned(::std::convert::Into::<ID3D11DeviceChild>::into(
            ::std::clone::Clone::clone(self),
        ))
    }
}
#[repr(C)]
#[allow(non_camel_case_types)]
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
#[repr(C)]
#[doc(hidden)]
#[allow(non_camel_case_types)]
pub struct D3D11_TEXTURE1D_DESC_abi(
    u32,
    u32,
    u32,
    super::dxgi::DXGI_FORMAT,
    D3D11_USAGE,
    u32,
    u32,
    u32,
);
impl D3D11_TEXTURE1D_DESC {}
unsafe impl ::windows::Abi for D3D11_TEXTURE1D_DESC {
    type Abi = D3D11_TEXTURE1D_DESC_abi;
}
impl ::core::default::Default for D3D11_TEXTURE1D_DESC {
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
impl ::core::fmt::Debug for D3D11_TEXTURE1D_DESC {
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
impl ::core::clone::Clone for D3D11_TEXTURE1D_DESC {
    fn clone(&self) -> Self {
        Self {
            width: self.width,
            mip_levels: self.mip_levels,
            array_size: self.array_size,
            format: self.format,
            usage: self.usage,
            bind_flags: self.bind_flags,
            cpu_access_flags: self.cpu_access_flags,
            misc_flags: self.misc_flags,
        }
    }
}
#[repr(transparent)]
#[allow(non_camel_case_types)]
pub struct ID3D11Texture1D(::windows::IUnknown);
impl ::std::clone::Clone for ID3D11Texture1D {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::std::fmt::Debug for ID3D11Texture1D {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        write!(f, "{:?}", self.0)
    }
}
impl ::std::cmp::PartialEq for ID3D11Texture1D {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::std::cmp::Eq for ID3D11Texture1D {}
unsafe impl ::windows::Interface for ID3D11Texture1D {
    type Vtable = ID3D11Texture1D_abi;
    const IID: ::windows::Guid = ::windows::Guid::from_values(
        4177222695,
        50867,
        20341,
        [164, 200, 67, 154, 242, 239, 86, 76],
    );
}
#[repr(C)]
pub struct ID3D11Texture1D_abi(
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        iid: &::windows::Guid,
        interface: *mut ::windows::RawPtr,
    ) -> ::windows::ErrorCode,
    pub unsafe extern "system" fn(this: ::windows::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::RawPtr) -> u32,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        pp_device: *mut ::std::option::Option<ID3D11Device>,
    ),
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
#[allow(non_snake_case)]
impl ID3D11Texture1D {
    pub unsafe fn GetDevice(&self, pp_device: *mut ::std::option::Option<ID3D11Device>) {
        (::windows::Interface::vtable(self).3)(::windows::Abi::abi(self), pp_device)
    }
    pub unsafe fn GetPrivateData(
        &self,
        guid: *const ::windows::Guid,
        p_data_size: *mut u32,
        p_data: *mut ::std::ffi::c_void,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).4)(::windows::Abi::abi(self), guid, p_data_size, p_data)
    }
    pub unsafe fn SetPrivateData(
        &self,
        guid: *const ::windows::Guid,
        data_size: u32,
        p_data: *const ::std::ffi::c_void,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).5)(::windows::Abi::abi(self), guid, data_size, p_data)
    }
    pub unsafe fn SetPrivateDataInterface<
        'a,
        T1__: ::std::convert::Into<::windows::Param<'a, ::windows::IUnknown>>,
    >(
        &self,
        guid: *const ::windows::Guid,
        p_data: T1__,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).6)(::windows::Abi::abi(self), guid, p_data.into().abi())
    }
    pub unsafe fn GetType(&self, p_resource_dimension: *mut D3D11_RESOURCE_DIMENSION) {
        (::windows::Interface::vtable(self).7)(::windows::Abi::abi(self), p_resource_dimension)
    }
    pub unsafe fn SetEvictionPriority(&self, eviction_priority: u32) {
        (::windows::Interface::vtable(self).8)(::windows::Abi::abi(self), eviction_priority)
    }
    pub unsafe fn GetEvictionPriority(&self) -> u32 {
        (::windows::Interface::vtable(self).9)(::windows::Abi::abi(self))
    }
    pub unsafe fn GetDesc(&self, p_desc: *mut D3D11_TEXTURE1D_DESC) {
        (::windows::Interface::vtable(self).10)(::windows::Abi::abi(self), p_desc)
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
impl<'a> ::std::convert::Into<::windows::Param<'a, ::windows::IUnknown>> for ID3D11Texture1D {
    fn into(self) -> ::windows::Param<'a, ::windows::IUnknown> {
        ::windows::Param::Owned(::std::convert::Into::<::windows::IUnknown>::into(self))
    }
}
impl<'a> ::std::convert::Into<::windows::Param<'a, ::windows::IUnknown>> for &'a ID3D11Texture1D {
    fn into(self) -> ::windows::Param<'a, ::windows::IUnknown> {
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
impl<'a> ::std::convert::Into<::windows::Param<'a, ID3D11Resource>> for ID3D11Texture1D {
    fn into(self) -> ::windows::Param<'a, ID3D11Resource> {
        ::windows::Param::Owned(::std::convert::Into::<ID3D11Resource>::into(self))
    }
}
impl<'a> ::std::convert::Into<::windows::Param<'a, ID3D11Resource>> for &'a ID3D11Texture1D {
    fn into(self) -> ::windows::Param<'a, ID3D11Resource> {
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
impl<'a> ::std::convert::Into<::windows::Param<'a, ID3D11DeviceChild>> for ID3D11Texture1D {
    fn into(self) -> ::windows::Param<'a, ID3D11DeviceChild> {
        ::windows::Param::Owned(::std::convert::Into::<ID3D11DeviceChild>::into(self))
    }
}
impl<'a> ::std::convert::Into<::windows::Param<'a, ID3D11DeviceChild>> for &'a ID3D11Texture1D {
    fn into(self) -> ::windows::Param<'a, ID3D11DeviceChild> {
        ::windows::Param::Owned(::std::convert::Into::<ID3D11DeviceChild>::into(
            ::std::clone::Clone::clone(self),
        ))
    }
}
#[repr(C)]
#[allow(non_camel_case_types)]
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
#[repr(C)]
#[doc(hidden)]
#[allow(non_camel_case_types)]
pub struct D3D11_TEXTURE2D_DESC_abi(
    u32,
    u32,
    u32,
    u32,
    super::dxgi::DXGI_FORMAT,
    super::dxgi::DXGI_SAMPLE_DESC_abi,
    D3D11_USAGE,
    u32,
    u32,
    u32,
);
impl D3D11_TEXTURE2D_DESC {}
unsafe impl ::windows::Abi for D3D11_TEXTURE2D_DESC {
    type Abi = D3D11_TEXTURE2D_DESC_abi;
}
impl ::core::default::Default for D3D11_TEXTURE2D_DESC {
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
impl ::core::fmt::Debug for D3D11_TEXTURE2D_DESC {
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
impl ::core::clone::Clone for D3D11_TEXTURE2D_DESC {
    fn clone(&self) -> Self {
        Self {
            width: self.width,
            height: self.height,
            mip_levels: self.mip_levels,
            array_size: self.array_size,
            format: self.format,
            sample_desc: <super::dxgi::DXGI_SAMPLE_DESC as std::clone::Clone>::clone(
                &self.sample_desc,
            ),
            usage: self.usage,
            bind_flags: self.bind_flags,
            cpu_access_flags: self.cpu_access_flags,
            misc_flags: self.misc_flags,
        }
    }
}
#[repr(transparent)]
#[allow(non_camel_case_types)]
pub struct ID3D11Texture2D(::windows::IUnknown);
impl ::std::clone::Clone for ID3D11Texture2D {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::std::fmt::Debug for ID3D11Texture2D {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        write!(f, "{:?}", self.0)
    }
}
impl ::std::cmp::PartialEq for ID3D11Texture2D {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::std::cmp::Eq for ID3D11Texture2D {}
unsafe impl ::windows::Interface for ID3D11Texture2D {
    type Vtable = ID3D11Texture2D_abi;
    const IID: ::windows::Guid = ::windows::Guid::from_values(
        1863690994,
        53768,
        20105,
        [154, 180, 72, 149, 53, 211, 79, 156],
    );
}
#[repr(C)]
pub struct ID3D11Texture2D_abi(
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        iid: &::windows::Guid,
        interface: *mut ::windows::RawPtr,
    ) -> ::windows::ErrorCode,
    pub unsafe extern "system" fn(this: ::windows::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::RawPtr) -> u32,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        pp_device: *mut ::std::option::Option<ID3D11Device>,
    ),
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
#[allow(non_snake_case)]
impl ID3D11Texture2D {
    pub unsafe fn GetDevice(&self, pp_device: *mut ::std::option::Option<ID3D11Device>) {
        (::windows::Interface::vtable(self).3)(::windows::Abi::abi(self), pp_device)
    }
    pub unsafe fn GetPrivateData(
        &self,
        guid: *const ::windows::Guid,
        p_data_size: *mut u32,
        p_data: *mut ::std::ffi::c_void,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).4)(::windows::Abi::abi(self), guid, p_data_size, p_data)
    }
    pub unsafe fn SetPrivateData(
        &self,
        guid: *const ::windows::Guid,
        data_size: u32,
        p_data: *const ::std::ffi::c_void,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).5)(::windows::Abi::abi(self), guid, data_size, p_data)
    }
    pub unsafe fn SetPrivateDataInterface<
        'a,
        T1__: ::std::convert::Into<::windows::Param<'a, ::windows::IUnknown>>,
    >(
        &self,
        guid: *const ::windows::Guid,
        p_data: T1__,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).6)(::windows::Abi::abi(self), guid, p_data.into().abi())
    }
    pub unsafe fn GetType(&self, p_resource_dimension: *mut D3D11_RESOURCE_DIMENSION) {
        (::windows::Interface::vtable(self).7)(::windows::Abi::abi(self), p_resource_dimension)
    }
    pub unsafe fn SetEvictionPriority(&self, eviction_priority: u32) {
        (::windows::Interface::vtable(self).8)(::windows::Abi::abi(self), eviction_priority)
    }
    pub unsafe fn GetEvictionPriority(&self) -> u32 {
        (::windows::Interface::vtable(self).9)(::windows::Abi::abi(self))
    }
    pub unsafe fn GetDesc(&self, p_desc: *mut D3D11_TEXTURE2D_DESC) {
        (::windows::Interface::vtable(self).10)(::windows::Abi::abi(self), p_desc)
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
impl<'a> ::std::convert::Into<::windows::Param<'a, ::windows::IUnknown>> for ID3D11Texture2D {
    fn into(self) -> ::windows::Param<'a, ::windows::IUnknown> {
        ::windows::Param::Owned(::std::convert::Into::<::windows::IUnknown>::into(self))
    }
}
impl<'a> ::std::convert::Into<::windows::Param<'a, ::windows::IUnknown>> for &'a ID3D11Texture2D {
    fn into(self) -> ::windows::Param<'a, ::windows::IUnknown> {
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
impl<'a> ::std::convert::Into<::windows::Param<'a, ID3D11Resource>> for ID3D11Texture2D {
    fn into(self) -> ::windows::Param<'a, ID3D11Resource> {
        ::windows::Param::Owned(::std::convert::Into::<ID3D11Resource>::into(self))
    }
}
impl<'a> ::std::convert::Into<::windows::Param<'a, ID3D11Resource>> for &'a ID3D11Texture2D {
    fn into(self) -> ::windows::Param<'a, ID3D11Resource> {
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
impl<'a> ::std::convert::Into<::windows::Param<'a, ID3D11DeviceChild>> for ID3D11Texture2D {
    fn into(self) -> ::windows::Param<'a, ID3D11DeviceChild> {
        ::windows::Param::Owned(::std::convert::Into::<ID3D11DeviceChild>::into(self))
    }
}
impl<'a> ::std::convert::Into<::windows::Param<'a, ID3D11DeviceChild>> for &'a ID3D11Texture2D {
    fn into(self) -> ::windows::Param<'a, ID3D11DeviceChild> {
        ::windows::Param::Owned(::std::convert::Into::<ID3D11DeviceChild>::into(
            ::std::clone::Clone::clone(self),
        ))
    }
}
#[repr(C)]
#[allow(non_camel_case_types)]
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
#[repr(C)]
#[doc(hidden)]
#[allow(non_camel_case_types)]
pub struct D3D11_TEXTURE3D_DESC_abi(
    u32,
    u32,
    u32,
    u32,
    super::dxgi::DXGI_FORMAT,
    D3D11_USAGE,
    u32,
    u32,
    u32,
);
impl D3D11_TEXTURE3D_DESC {}
unsafe impl ::windows::Abi for D3D11_TEXTURE3D_DESC {
    type Abi = D3D11_TEXTURE3D_DESC_abi;
}
impl ::core::default::Default for D3D11_TEXTURE3D_DESC {
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
impl ::core::fmt::Debug for D3D11_TEXTURE3D_DESC {
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
impl ::core::clone::Clone for D3D11_TEXTURE3D_DESC {
    fn clone(&self) -> Self {
        Self {
            width: self.width,
            height: self.height,
            depth: self.depth,
            mip_levels: self.mip_levels,
            format: self.format,
            usage: self.usage,
            bind_flags: self.bind_flags,
            cpu_access_flags: self.cpu_access_flags,
            misc_flags: self.misc_flags,
        }
    }
}
#[repr(transparent)]
#[allow(non_camel_case_types)]
pub struct ID3D11Texture3D(::windows::IUnknown);
impl ::std::clone::Clone for ID3D11Texture3D {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::std::fmt::Debug for ID3D11Texture3D {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        write!(f, "{:?}", self.0)
    }
}
impl ::std::cmp::PartialEq for ID3D11Texture3D {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::std::cmp::Eq for ID3D11Texture3D {}
unsafe impl ::windows::Interface for ID3D11Texture3D {
    type Vtable = ID3D11Texture3D_abi;
    const IID: ::windows::Guid = ::windows::Guid::from_values(
        58623598,
        62829,
        17239,
        [168, 175, 157, 171, 190, 110, 37, 14],
    );
}
#[repr(C)]
pub struct ID3D11Texture3D_abi(
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        iid: &::windows::Guid,
        interface: *mut ::windows::RawPtr,
    ) -> ::windows::ErrorCode,
    pub unsafe extern "system" fn(this: ::windows::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::RawPtr) -> u32,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        pp_device: *mut ::std::option::Option<ID3D11Device>,
    ),
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
#[allow(non_snake_case)]
impl ID3D11Texture3D {
    pub unsafe fn GetDevice(&self, pp_device: *mut ::std::option::Option<ID3D11Device>) {
        (::windows::Interface::vtable(self).3)(::windows::Abi::abi(self), pp_device)
    }
    pub unsafe fn GetPrivateData(
        &self,
        guid: *const ::windows::Guid,
        p_data_size: *mut u32,
        p_data: *mut ::std::ffi::c_void,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).4)(::windows::Abi::abi(self), guid, p_data_size, p_data)
    }
    pub unsafe fn SetPrivateData(
        &self,
        guid: *const ::windows::Guid,
        data_size: u32,
        p_data: *const ::std::ffi::c_void,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).5)(::windows::Abi::abi(self), guid, data_size, p_data)
    }
    pub unsafe fn SetPrivateDataInterface<
        'a,
        T1__: ::std::convert::Into<::windows::Param<'a, ::windows::IUnknown>>,
    >(
        &self,
        guid: *const ::windows::Guid,
        p_data: T1__,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).6)(::windows::Abi::abi(self), guid, p_data.into().abi())
    }
    pub unsafe fn GetType(&self, p_resource_dimension: *mut D3D11_RESOURCE_DIMENSION) {
        (::windows::Interface::vtable(self).7)(::windows::Abi::abi(self), p_resource_dimension)
    }
    pub unsafe fn SetEvictionPriority(&self, eviction_priority: u32) {
        (::windows::Interface::vtable(self).8)(::windows::Abi::abi(self), eviction_priority)
    }
    pub unsafe fn GetEvictionPriority(&self) -> u32 {
        (::windows::Interface::vtable(self).9)(::windows::Abi::abi(self))
    }
    pub unsafe fn GetDesc(&self, p_desc: *mut D3D11_TEXTURE3D_DESC) {
        (::windows::Interface::vtable(self).10)(::windows::Abi::abi(self), p_desc)
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
impl<'a> ::std::convert::Into<::windows::Param<'a, ::windows::IUnknown>> for ID3D11Texture3D {
    fn into(self) -> ::windows::Param<'a, ::windows::IUnknown> {
        ::windows::Param::Owned(::std::convert::Into::<::windows::IUnknown>::into(self))
    }
}
impl<'a> ::std::convert::Into<::windows::Param<'a, ::windows::IUnknown>> for &'a ID3D11Texture3D {
    fn into(self) -> ::windows::Param<'a, ::windows::IUnknown> {
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
impl<'a> ::std::convert::Into<::windows::Param<'a, ID3D11Resource>> for ID3D11Texture3D {
    fn into(self) -> ::windows::Param<'a, ID3D11Resource> {
        ::windows::Param::Owned(::std::convert::Into::<ID3D11Resource>::into(self))
    }
}
impl<'a> ::std::convert::Into<::windows::Param<'a, ID3D11Resource>> for &'a ID3D11Texture3D {
    fn into(self) -> ::windows::Param<'a, ID3D11Resource> {
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
impl<'a> ::std::convert::Into<::windows::Param<'a, ID3D11DeviceChild>> for ID3D11Texture3D {
    fn into(self) -> ::windows::Param<'a, ID3D11DeviceChild> {
        ::windows::Param::Owned(::std::convert::Into::<ID3D11DeviceChild>::into(self))
    }
}
impl<'a> ::std::convert::Into<::windows::Param<'a, ID3D11DeviceChild>> for &'a ID3D11Texture3D {
    fn into(self) -> ::windows::Param<'a, ID3D11DeviceChild> {
        ::windows::Param::Owned(::std::convert::Into::<ID3D11DeviceChild>::into(
            ::std::clone::Clone::clone(self),
        ))
    }
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct D3D_SRV_DIMENSION(pub i32);
impl ::std::convert::From<i32> for D3D_SRV_DIMENSION {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
impl ::std::clone::Clone for D3D_SRV_DIMENSION {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}
impl ::std::default::Default for D3D_SRV_DIMENSION {
    fn default() -> Self {
        Self(0)
    }
}
impl ::std::fmt::Debug for D3D_SRV_DIMENSION {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        write!(f, "{:?}", self.0)
    }
}
impl ::std::cmp::PartialEq for D3D_SRV_DIMENSION {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::std::cmp::Eq for D3D_SRV_DIMENSION {}
impl ::std::marker::Copy for D3D_SRV_DIMENSION {}
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
unsafe impl ::windows::Abi for D3D_SRV_DIMENSION {
    type Abi = Self;
}
#[repr(C)]
#[allow(non_camel_case_types)]
pub struct D3D11_BUFFER_SRV {
    pub anonymous1: D3D11_BUFFER_SRV_1,
    pub anonymous2: ::windows::NOT_YET_SUPPORTED_TYPE,
}
#[repr(C)]
#[doc(hidden)]
#[allow(non_camel_case_types)]
pub struct D3D11_BUFFER_SRV_abi(D3D11_BUFFER_SRV_1_abi, ::windows::NOT_YET_SUPPORTED_TYPE);
impl D3D11_BUFFER_SRV {}
unsafe impl ::windows::Abi for D3D11_BUFFER_SRV {
    type Abi = D3D11_BUFFER_SRV_abi;
}
impl ::core::fmt::Debug for D3D11_BUFFER_SRV {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("D3D11_BUFFER_SRV")
            .field("anonymous1", &format_args!("{:?}", self.anonymous1))
            .field("anonymous2", &format_args!("{:?}", self.anonymous2))
            .finish()
    }
}
impl ::core::default::Default for D3D11_BUFFER_SRV {
    fn default() -> Self {
        Self {
            anonymous1: ::std::default::Default::default(),
            anonymous2: ::std::default::Default::default(),
        }
    }
}
impl ::core::clone::Clone for D3D11_BUFFER_SRV {
    fn clone(&self) -> Self {
        let mut out = Self::default();
        out.clone_from(self);
        out
    }
    fn clone_from(&mut self, source: &Self) {
        unsafe { std::ptr::copy(source, self, std::mem::size_of::<Self>()) }
    }
}
#[repr(C)]
#[allow(non_camel_case_types)]
pub union D3D11_BUFFER_SRV_1 {
    pub first_element: ::core::mem::ManuallyDrop<u32>,
    pub element_offset: ::core::mem::ManuallyDrop<u32>,
}
#[repr(C)]
#[doc(hidden)]
#[allow(non_camel_case_types)]
pub struct D3D11_BUFFER_SRV_1_abi(u32, u32);
impl D3D11_BUFFER_SRV_1 {}
unsafe impl ::windows::Abi for D3D11_BUFFER_SRV_1 {
    type Abi = D3D11_BUFFER_SRV_1_abi;
}
impl ::core::fmt::Debug for D3D11_BUFFER_SRV_1 {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("D3D11_BUFFER_SRV_1").finish()
    }
}
impl ::core::default::Default for D3D11_BUFFER_SRV_1 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::clone::Clone for D3D11_BUFFER_SRV_1 {
    fn clone(&self) -> Self {
        let mut out = Self::default();
        out.clone_from(self);
        out
    }
    fn clone_from(&mut self, source: &Self) {
        unsafe { std::ptr::copy(source, self, std::mem::size_of::<Self>()) }
    }
}
#[repr(C)]
#[allow(non_camel_case_types)]
pub struct D3D11_TEX1D_SRV {
    pub most_detailed_mip: u32,
    pub mip_levels: u32,
}
#[repr(C)]
#[doc(hidden)]
#[allow(non_camel_case_types)]
pub struct D3D11_TEX1D_SRV_abi(u32, u32);
impl D3D11_TEX1D_SRV {}
unsafe impl ::windows::Abi for D3D11_TEX1D_SRV {
    type Abi = D3D11_TEX1D_SRV_abi;
}
impl ::core::default::Default for D3D11_TEX1D_SRV {
    fn default() -> Self {
        Self {
            most_detailed_mip: 0,
            mip_levels: 0,
        }
    }
}
impl ::core::fmt::Debug for D3D11_TEX1D_SRV {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("D3D11_TEX1D_SRV")
            .field(
                "most_detailed_mip",
                &format_args!("{:?}", self.most_detailed_mip),
            )
            .field("mip_levels", &format_args!("{:?}", self.mip_levels))
            .finish()
    }
}
impl ::core::clone::Clone for D3D11_TEX1D_SRV {
    fn clone(&self) -> Self {
        Self {
            most_detailed_mip: self.most_detailed_mip,
            mip_levels: self.mip_levels,
        }
    }
}
impl ::std::marker::Copy for D3D11_TEX1D_SRV {}
#[repr(C)]
#[allow(non_camel_case_types)]
pub struct D3D11_TEX1D_ARRAY_SRV {
    pub most_detailed_mip: u32,
    pub mip_levels: u32,
    pub first_array_slice: u32,
    pub array_size: u32,
}
#[repr(C)]
#[doc(hidden)]
#[allow(non_camel_case_types)]
pub struct D3D11_TEX1D_ARRAY_SRV_abi(u32, u32, u32, u32);
impl D3D11_TEX1D_ARRAY_SRV {}
unsafe impl ::windows::Abi for D3D11_TEX1D_ARRAY_SRV {
    type Abi = D3D11_TEX1D_ARRAY_SRV_abi;
}
impl ::core::default::Default for D3D11_TEX1D_ARRAY_SRV {
    fn default() -> Self {
        Self {
            most_detailed_mip: 0,
            mip_levels: 0,
            first_array_slice: 0,
            array_size: 0,
        }
    }
}
impl ::core::fmt::Debug for D3D11_TEX1D_ARRAY_SRV {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("D3D11_TEX1D_ARRAY_SRV")
            .field(
                "most_detailed_mip",
                &format_args!("{:?}", self.most_detailed_mip),
            )
            .field("mip_levels", &format_args!("{:?}", self.mip_levels))
            .field(
                "first_array_slice",
                &format_args!("{:?}", self.first_array_slice),
            )
            .field("array_size", &format_args!("{:?}", self.array_size))
            .finish()
    }
}
impl ::core::clone::Clone for D3D11_TEX1D_ARRAY_SRV {
    fn clone(&self) -> Self {
        Self {
            most_detailed_mip: self.most_detailed_mip,
            mip_levels: self.mip_levels,
            first_array_slice: self.first_array_slice,
            array_size: self.array_size,
        }
    }
}
impl ::std::marker::Copy for D3D11_TEX1D_ARRAY_SRV {}
#[repr(C)]
#[allow(non_camel_case_types)]
pub struct D3D11_TEX2D_SRV {
    pub most_detailed_mip: u32,
    pub mip_levels: u32,
}
#[repr(C)]
#[doc(hidden)]
#[allow(non_camel_case_types)]
pub struct D3D11_TEX2D_SRV_abi(u32, u32);
impl D3D11_TEX2D_SRV {}
unsafe impl ::windows::Abi for D3D11_TEX2D_SRV {
    type Abi = D3D11_TEX2D_SRV_abi;
}
impl ::core::default::Default for D3D11_TEX2D_SRV {
    fn default() -> Self {
        Self {
            most_detailed_mip: 0,
            mip_levels: 0,
        }
    }
}
impl ::core::fmt::Debug for D3D11_TEX2D_SRV {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("D3D11_TEX2D_SRV")
            .field(
                "most_detailed_mip",
                &format_args!("{:?}", self.most_detailed_mip),
            )
            .field("mip_levels", &format_args!("{:?}", self.mip_levels))
            .finish()
    }
}
impl ::core::clone::Clone for D3D11_TEX2D_SRV {
    fn clone(&self) -> Self {
        Self {
            most_detailed_mip: self.most_detailed_mip,
            mip_levels: self.mip_levels,
        }
    }
}
impl ::std::marker::Copy for D3D11_TEX2D_SRV {}
#[repr(C)]
#[allow(non_camel_case_types)]
pub struct D3D11_TEX2D_ARRAY_SRV {
    pub most_detailed_mip: u32,
    pub mip_levels: u32,
    pub first_array_slice: u32,
    pub array_size: u32,
}
#[repr(C)]
#[doc(hidden)]
#[allow(non_camel_case_types)]
pub struct D3D11_TEX2D_ARRAY_SRV_abi(u32, u32, u32, u32);
impl D3D11_TEX2D_ARRAY_SRV {}
unsafe impl ::windows::Abi for D3D11_TEX2D_ARRAY_SRV {
    type Abi = D3D11_TEX2D_ARRAY_SRV_abi;
}
impl ::core::default::Default for D3D11_TEX2D_ARRAY_SRV {
    fn default() -> Self {
        Self {
            most_detailed_mip: 0,
            mip_levels: 0,
            first_array_slice: 0,
            array_size: 0,
        }
    }
}
impl ::core::fmt::Debug for D3D11_TEX2D_ARRAY_SRV {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("D3D11_TEX2D_ARRAY_SRV")
            .field(
                "most_detailed_mip",
                &format_args!("{:?}", self.most_detailed_mip),
            )
            .field("mip_levels", &format_args!("{:?}", self.mip_levels))
            .field(
                "first_array_slice",
                &format_args!("{:?}", self.first_array_slice),
            )
            .field("array_size", &format_args!("{:?}", self.array_size))
            .finish()
    }
}
impl ::core::clone::Clone for D3D11_TEX2D_ARRAY_SRV {
    fn clone(&self) -> Self {
        Self {
            most_detailed_mip: self.most_detailed_mip,
            mip_levels: self.mip_levels,
            first_array_slice: self.first_array_slice,
            array_size: self.array_size,
        }
    }
}
impl ::std::marker::Copy for D3D11_TEX2D_ARRAY_SRV {}
#[repr(C)]
#[allow(non_camel_case_types)]
pub struct D3D11_TEX2DMS_SRV {
    pub unused_field_nothing_to_define: u32,
}
#[repr(C)]
#[doc(hidden)]
#[allow(non_camel_case_types)]
pub struct D3D11_TEX2DMS_SRV_abi(u32);
impl D3D11_TEX2DMS_SRV {}
unsafe impl ::windows::Abi for D3D11_TEX2DMS_SRV {
    type Abi = D3D11_TEX2DMS_SRV_abi;
}
impl ::core::default::Default for D3D11_TEX2DMS_SRV {
    fn default() -> Self {
        Self {
            unused_field_nothing_to_define: 0,
        }
    }
}
impl ::core::fmt::Debug for D3D11_TEX2DMS_SRV {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("D3D11_TEX2DMS_SRV")
            .field(
                "unused_field_nothing_to_define",
                &format_args!("{:?}", self.unused_field_nothing_to_define),
            )
            .finish()
    }
}
impl ::core::clone::Clone for D3D11_TEX2DMS_SRV {
    fn clone(&self) -> Self {
        Self {
            unused_field_nothing_to_define: self.unused_field_nothing_to_define,
        }
    }
}
impl ::std::marker::Copy for D3D11_TEX2DMS_SRV {}
#[repr(C)]
#[allow(non_camel_case_types)]
pub struct D3D11_TEX2DMS_ARRAY_SRV {
    pub first_array_slice: u32,
    pub array_size: u32,
}
#[repr(C)]
#[doc(hidden)]
#[allow(non_camel_case_types)]
pub struct D3D11_TEX2DMS_ARRAY_SRV_abi(u32, u32);
impl D3D11_TEX2DMS_ARRAY_SRV {}
unsafe impl ::windows::Abi for D3D11_TEX2DMS_ARRAY_SRV {
    type Abi = D3D11_TEX2DMS_ARRAY_SRV_abi;
}
impl ::core::default::Default for D3D11_TEX2DMS_ARRAY_SRV {
    fn default() -> Self {
        Self {
            first_array_slice: 0,
            array_size: 0,
        }
    }
}
impl ::core::fmt::Debug for D3D11_TEX2DMS_ARRAY_SRV {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("D3D11_TEX2DMS_ARRAY_SRV")
            .field(
                "first_array_slice",
                &format_args!("{:?}", self.first_array_slice),
            )
            .field("array_size", &format_args!("{:?}", self.array_size))
            .finish()
    }
}
impl ::core::clone::Clone for D3D11_TEX2DMS_ARRAY_SRV {
    fn clone(&self) -> Self {
        Self {
            first_array_slice: self.first_array_slice,
            array_size: self.array_size,
        }
    }
}
impl ::std::marker::Copy for D3D11_TEX2DMS_ARRAY_SRV {}
#[repr(C)]
#[allow(non_camel_case_types)]
pub struct D3D11_TEX3D_SRV {
    pub most_detailed_mip: u32,
    pub mip_levels: u32,
}
#[repr(C)]
#[doc(hidden)]
#[allow(non_camel_case_types)]
pub struct D3D11_TEX3D_SRV_abi(u32, u32);
impl D3D11_TEX3D_SRV {}
unsafe impl ::windows::Abi for D3D11_TEX3D_SRV {
    type Abi = D3D11_TEX3D_SRV_abi;
}
impl ::core::default::Default for D3D11_TEX3D_SRV {
    fn default() -> Self {
        Self {
            most_detailed_mip: 0,
            mip_levels: 0,
        }
    }
}
impl ::core::fmt::Debug for D3D11_TEX3D_SRV {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("D3D11_TEX3D_SRV")
            .field(
                "most_detailed_mip",
                &format_args!("{:?}", self.most_detailed_mip),
            )
            .field("mip_levels", &format_args!("{:?}", self.mip_levels))
            .finish()
    }
}
impl ::core::clone::Clone for D3D11_TEX3D_SRV {
    fn clone(&self) -> Self {
        Self {
            most_detailed_mip: self.most_detailed_mip,
            mip_levels: self.mip_levels,
        }
    }
}
impl ::std::marker::Copy for D3D11_TEX3D_SRV {}
#[repr(C)]
#[allow(non_camel_case_types)]
pub struct D3D11_TEXCUBE_SRV {
    pub most_detailed_mip: u32,
    pub mip_levels: u32,
}
#[repr(C)]
#[doc(hidden)]
#[allow(non_camel_case_types)]
pub struct D3D11_TEXCUBE_SRV_abi(u32, u32);
impl D3D11_TEXCUBE_SRV {}
unsafe impl ::windows::Abi for D3D11_TEXCUBE_SRV {
    type Abi = D3D11_TEXCUBE_SRV_abi;
}
impl ::core::default::Default for D3D11_TEXCUBE_SRV {
    fn default() -> Self {
        Self {
            most_detailed_mip: 0,
            mip_levels: 0,
        }
    }
}
impl ::core::fmt::Debug for D3D11_TEXCUBE_SRV {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("D3D11_TEXCUBE_SRV")
            .field(
                "most_detailed_mip",
                &format_args!("{:?}", self.most_detailed_mip),
            )
            .field("mip_levels", &format_args!("{:?}", self.mip_levels))
            .finish()
    }
}
impl ::core::clone::Clone for D3D11_TEXCUBE_SRV {
    fn clone(&self) -> Self {
        Self {
            most_detailed_mip: self.most_detailed_mip,
            mip_levels: self.mip_levels,
        }
    }
}
impl ::std::marker::Copy for D3D11_TEXCUBE_SRV {}
#[repr(C)]
#[allow(non_camel_case_types)]
pub struct D3D11_TEXCUBE_ARRAY_SRV {
    pub most_detailed_mip: u32,
    pub mip_levels: u32,
    pub first2d_array_face: u32,
    pub num_cubes: u32,
}
#[repr(C)]
#[doc(hidden)]
#[allow(non_camel_case_types)]
pub struct D3D11_TEXCUBE_ARRAY_SRV_abi(u32, u32, u32, u32);
impl D3D11_TEXCUBE_ARRAY_SRV {}
unsafe impl ::windows::Abi for D3D11_TEXCUBE_ARRAY_SRV {
    type Abi = D3D11_TEXCUBE_ARRAY_SRV_abi;
}
impl ::core::default::Default for D3D11_TEXCUBE_ARRAY_SRV {
    fn default() -> Self {
        Self {
            most_detailed_mip: 0,
            mip_levels: 0,
            first2d_array_face: 0,
            num_cubes: 0,
        }
    }
}
impl ::core::fmt::Debug for D3D11_TEXCUBE_ARRAY_SRV {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("D3D11_TEXCUBE_ARRAY_SRV")
            .field(
                "most_detailed_mip",
                &format_args!("{:?}", self.most_detailed_mip),
            )
            .field("mip_levels", &format_args!("{:?}", self.mip_levels))
            .field(
                "first2d_array_face",
                &format_args!("{:?}", self.first2d_array_face),
            )
            .field("num_cubes", &format_args!("{:?}", self.num_cubes))
            .finish()
    }
}
impl ::core::clone::Clone for D3D11_TEXCUBE_ARRAY_SRV {
    fn clone(&self) -> Self {
        Self {
            most_detailed_mip: self.most_detailed_mip,
            mip_levels: self.mip_levels,
            first2d_array_face: self.first2d_array_face,
            num_cubes: self.num_cubes,
        }
    }
}
impl ::std::marker::Copy for D3D11_TEXCUBE_ARRAY_SRV {}
#[repr(C)]
#[allow(non_camel_case_types)]
pub struct D3D11_BUFFEREX_SRV {
    pub first_element: u32,
    pub num_elements: u32,
    pub flags: u32,
}
#[repr(C)]
#[doc(hidden)]
#[allow(non_camel_case_types)]
pub struct D3D11_BUFFEREX_SRV_abi(u32, u32, u32);
impl D3D11_BUFFEREX_SRV {}
unsafe impl ::windows::Abi for D3D11_BUFFEREX_SRV {
    type Abi = D3D11_BUFFEREX_SRV_abi;
}
impl ::core::default::Default for D3D11_BUFFEREX_SRV {
    fn default() -> Self {
        Self {
            first_element: 0,
            num_elements: 0,
            flags: 0,
        }
    }
}
impl ::core::fmt::Debug for D3D11_BUFFEREX_SRV {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("D3D11_BUFFEREX_SRV")
            .field("first_element", &format_args!("{:?}", self.first_element))
            .field("num_elements", &format_args!("{:?}", self.num_elements))
            .field("flags", &format_args!("{:?}", self.flags))
            .finish()
    }
}
impl ::core::clone::Clone for D3D11_BUFFEREX_SRV {
    fn clone(&self) -> Self {
        Self {
            first_element: self.first_element,
            num_elements: self.num_elements,
            flags: self.flags,
        }
    }
}
impl ::std::marker::Copy for D3D11_BUFFEREX_SRV {}
#[repr(C)]
#[allow(non_camel_case_types)]
pub struct D3D11_SHADER_RESOURCE_VIEW_DESC {
    pub format: super::dxgi::DXGI_FORMAT,
    pub view_dimension: D3D_SRV_DIMENSION,
    pub anonymous: D3D11_SHADER_RESOURCE_VIEW_DESC_0,
}
#[repr(C)]
#[doc(hidden)]
#[allow(non_camel_case_types)]
pub struct D3D11_SHADER_RESOURCE_VIEW_DESC_abi(
    super::dxgi::DXGI_FORMAT,
    D3D_SRV_DIMENSION,
    D3D11_SHADER_RESOURCE_VIEW_DESC_0_abi,
);
impl D3D11_SHADER_RESOURCE_VIEW_DESC {}
unsafe impl ::windows::Abi for D3D11_SHADER_RESOURCE_VIEW_DESC {
    type Abi = D3D11_SHADER_RESOURCE_VIEW_DESC_abi;
}
impl ::core::fmt::Debug for D3D11_SHADER_RESOURCE_VIEW_DESC {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("D3D11_SHADER_RESOURCE_VIEW_DESC")
            .field("format", &format_args!("{:?}", self.format))
            .field("view_dimension", &format_args!("{:?}", self.view_dimension))
            .field("anonymous", &format_args!("{:?}", self.anonymous))
            .finish()
    }
}
impl ::core::default::Default for D3D11_SHADER_RESOURCE_VIEW_DESC {
    fn default() -> Self {
        Self {
            format: ::std::default::Default::default(),
            view_dimension: ::std::default::Default::default(),
            anonymous: ::std::default::Default::default(),
        }
    }
}
impl ::core::clone::Clone for D3D11_SHADER_RESOURCE_VIEW_DESC {
    fn clone(&self) -> Self {
        let mut out = Self::default();
        out.clone_from(self);
        out
    }
    fn clone_from(&mut self, source: &Self) {
        unsafe { std::ptr::copy(source, self, std::mem::size_of::<Self>()) }
    }
}
#[repr(C)]
#[allow(non_camel_case_types)]
pub union D3D11_SHADER_RESOURCE_VIEW_DESC_0 {
    pub buffer: ::core::mem::ManuallyDrop<D3D11_BUFFER_SRV>,
    pub texture1d: ::core::mem::ManuallyDrop<D3D11_TEX1D_SRV>,
    pub texture1d_array: ::core::mem::ManuallyDrop<D3D11_TEX1D_ARRAY_SRV>,
    pub texture2d: ::core::mem::ManuallyDrop<D3D11_TEX2D_SRV>,
    pub texture2d_array: ::core::mem::ManuallyDrop<D3D11_TEX2D_ARRAY_SRV>,
    pub texture2dms: ::core::mem::ManuallyDrop<D3D11_TEX2DMS_SRV>,
    pub texture2dms_array: ::core::mem::ManuallyDrop<D3D11_TEX2DMS_ARRAY_SRV>,
    pub texture3d: ::core::mem::ManuallyDrop<D3D11_TEX3D_SRV>,
    pub texture_cube: ::core::mem::ManuallyDrop<D3D11_TEXCUBE_SRV>,
    pub texture_cube_array: ::core::mem::ManuallyDrop<D3D11_TEXCUBE_ARRAY_SRV>,
    pub buffer_ex: ::core::mem::ManuallyDrop<D3D11_BUFFEREX_SRV>,
}
#[repr(C)]
#[doc(hidden)]
#[allow(non_camel_case_types)]
pub struct D3D11_SHADER_RESOURCE_VIEW_DESC_0_abi(
    D3D11_BUFFER_SRV_abi,
    D3D11_TEX1D_SRV_abi,
    D3D11_TEX1D_ARRAY_SRV_abi,
    D3D11_TEX2D_SRV_abi,
    D3D11_TEX2D_ARRAY_SRV_abi,
    D3D11_TEX2DMS_SRV_abi,
    D3D11_TEX2DMS_ARRAY_SRV_abi,
    D3D11_TEX3D_SRV_abi,
    D3D11_TEXCUBE_SRV_abi,
    D3D11_TEXCUBE_ARRAY_SRV_abi,
    D3D11_BUFFEREX_SRV_abi,
);
impl D3D11_SHADER_RESOURCE_VIEW_DESC_0 {}
unsafe impl ::windows::Abi for D3D11_SHADER_RESOURCE_VIEW_DESC_0 {
    type Abi = D3D11_SHADER_RESOURCE_VIEW_DESC_0_abi;
}
impl ::core::fmt::Debug for D3D11_SHADER_RESOURCE_VIEW_DESC_0 {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("D3D11_SHADER_RESOURCE_VIEW_DESC_0")
            .finish()
    }
}
impl ::core::default::Default for D3D11_SHADER_RESOURCE_VIEW_DESC_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::clone::Clone for D3D11_SHADER_RESOURCE_VIEW_DESC_0 {
    fn clone(&self) -> Self {
        let mut out = Self::default();
        out.clone_from(self);
        out
    }
    fn clone_from(&mut self, source: &Self) {
        unsafe { std::ptr::copy(source, self, std::mem::size_of::<Self>()) }
    }
}
#[repr(transparent)]
#[allow(non_camel_case_types)]
pub struct ID3D11View(::windows::IUnknown);
impl ::std::clone::Clone for ID3D11View {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::std::fmt::Debug for ID3D11View {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        write!(f, "{:?}", self.0)
    }
}
impl ::std::cmp::PartialEq for ID3D11View {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::std::cmp::Eq for ID3D11View {}
unsafe impl ::windows::Interface for ID3D11View {
    type Vtable = ID3D11View_abi;
    const IID: ::windows::Guid = ::windows::Guid::from_values(
        2208109078,
        47918,
        16683,
        [183, 244, 169, 219, 235, 224, 142, 209],
    );
}
#[repr(C)]
pub struct ID3D11View_abi(
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        iid: &::windows::Guid,
        interface: *mut ::windows::RawPtr,
    ) -> ::windows::ErrorCode,
    pub unsafe extern "system" fn(this: ::windows::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::RawPtr) -> u32,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        pp_device: *mut ::std::option::Option<ID3D11Device>,
    ),
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
        pp_resource: *mut ::std::option::Option<ID3D11Resource>,
    ),
);
#[allow(non_snake_case)]
impl ID3D11View {
    pub unsafe fn GetDevice(&self, pp_device: *mut ::std::option::Option<ID3D11Device>) {
        (::windows::Interface::vtable(self).3)(::windows::Abi::abi(self), pp_device)
    }
    pub unsafe fn GetPrivateData(
        &self,
        guid: *const ::windows::Guid,
        p_data_size: *mut u32,
        p_data: *mut ::std::ffi::c_void,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).4)(::windows::Abi::abi(self), guid, p_data_size, p_data)
    }
    pub unsafe fn SetPrivateData(
        &self,
        guid: *const ::windows::Guid,
        data_size: u32,
        p_data: *const ::std::ffi::c_void,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).5)(::windows::Abi::abi(self), guid, data_size, p_data)
    }
    pub unsafe fn SetPrivateDataInterface<
        'a,
        T1__: ::std::convert::Into<::windows::Param<'a, ::windows::IUnknown>>,
    >(
        &self,
        guid: *const ::windows::Guid,
        p_data: T1__,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).6)(::windows::Abi::abi(self), guid, p_data.into().abi())
    }
    pub unsafe fn GetResource(&self, pp_resource: *mut ::std::option::Option<ID3D11Resource>) {
        (::windows::Interface::vtable(self).7)(::windows::Abi::abi(self), pp_resource)
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
impl<'a> ::std::convert::Into<::windows::Param<'a, ::windows::IUnknown>> for ID3D11View {
    fn into(self) -> ::windows::Param<'a, ::windows::IUnknown> {
        ::windows::Param::Owned(::std::convert::Into::<::windows::IUnknown>::into(self))
    }
}
impl<'a> ::std::convert::Into<::windows::Param<'a, ::windows::IUnknown>> for &'a ID3D11View {
    fn into(self) -> ::windows::Param<'a, ::windows::IUnknown> {
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
impl<'a> ::std::convert::Into<::windows::Param<'a, ID3D11DeviceChild>> for ID3D11View {
    fn into(self) -> ::windows::Param<'a, ID3D11DeviceChild> {
        ::windows::Param::Owned(::std::convert::Into::<ID3D11DeviceChild>::into(self))
    }
}
impl<'a> ::std::convert::Into<::windows::Param<'a, ID3D11DeviceChild>> for &'a ID3D11View {
    fn into(self) -> ::windows::Param<'a, ID3D11DeviceChild> {
        ::windows::Param::Owned(::std::convert::Into::<ID3D11DeviceChild>::into(
            ::std::clone::Clone::clone(self),
        ))
    }
}
#[repr(transparent)]
#[allow(non_camel_case_types)]
pub struct ID3D11ShaderResourceView(::windows::IUnknown);
impl ::std::clone::Clone for ID3D11ShaderResourceView {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::std::fmt::Debug for ID3D11ShaderResourceView {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        write!(f, "{:?}", self.0)
    }
}
impl ::std::cmp::PartialEq for ID3D11ShaderResourceView {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::std::cmp::Eq for ID3D11ShaderResourceView {}
unsafe impl ::windows::Interface for ID3D11ShaderResourceView {
    type Vtable = ID3D11ShaderResourceView_abi;
    const IID: ::windows::Guid = ::windows::Guid::from_values(
        2967498720,
        33170,
        19994,
        [177, 202, 54, 215, 65, 71, 16, 178],
    );
}
#[repr(C)]
pub struct ID3D11ShaderResourceView_abi(
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        iid: &::windows::Guid,
        interface: *mut ::windows::RawPtr,
    ) -> ::windows::ErrorCode,
    pub unsafe extern "system" fn(this: ::windows::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::RawPtr) -> u32,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        pp_device: *mut ::std::option::Option<ID3D11Device>,
    ),
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
        pp_resource: *mut ::std::option::Option<ID3D11Resource>,
    ),
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        p_desc: *mut D3D11_SHADER_RESOURCE_VIEW_DESC,
    ),
);
#[allow(non_snake_case)]
impl ID3D11ShaderResourceView {
    pub unsafe fn GetDevice(&self, pp_device: *mut ::std::option::Option<ID3D11Device>) {
        (::windows::Interface::vtable(self).3)(::windows::Abi::abi(self), pp_device)
    }
    pub unsafe fn GetPrivateData(
        &self,
        guid: *const ::windows::Guid,
        p_data_size: *mut u32,
        p_data: *mut ::std::ffi::c_void,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).4)(::windows::Abi::abi(self), guid, p_data_size, p_data)
    }
    pub unsafe fn SetPrivateData(
        &self,
        guid: *const ::windows::Guid,
        data_size: u32,
        p_data: *const ::std::ffi::c_void,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).5)(::windows::Abi::abi(self), guid, data_size, p_data)
    }
    pub unsafe fn SetPrivateDataInterface<
        'a,
        T1__: ::std::convert::Into<::windows::Param<'a, ::windows::IUnknown>>,
    >(
        &self,
        guid: *const ::windows::Guid,
        p_data: T1__,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).6)(::windows::Abi::abi(self), guid, p_data.into().abi())
    }
    pub unsafe fn GetResource(&self, pp_resource: *mut ::std::option::Option<ID3D11Resource>) {
        (::windows::Interface::vtable(self).7)(::windows::Abi::abi(self), pp_resource)
    }
    pub unsafe fn GetDesc(&self, p_desc: *mut D3D11_SHADER_RESOURCE_VIEW_DESC) {
        (::windows::Interface::vtable(self).8)(::windows::Abi::abi(self), p_desc)
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
impl<'a> ::std::convert::Into<::windows::Param<'a, ::windows::IUnknown>>
    for ID3D11ShaderResourceView
{
    fn into(self) -> ::windows::Param<'a, ::windows::IUnknown> {
        ::windows::Param::Owned(::std::convert::Into::<::windows::IUnknown>::into(self))
    }
}
impl<'a> ::std::convert::Into<::windows::Param<'a, ::windows::IUnknown>>
    for &'a ID3D11ShaderResourceView
{
    fn into(self) -> ::windows::Param<'a, ::windows::IUnknown> {
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
impl<'a> ::std::convert::Into<::windows::Param<'a, ID3D11View>> for ID3D11ShaderResourceView {
    fn into(self) -> ::windows::Param<'a, ID3D11View> {
        ::windows::Param::Owned(::std::convert::Into::<ID3D11View>::into(self))
    }
}
impl<'a> ::std::convert::Into<::windows::Param<'a, ID3D11View>> for &'a ID3D11ShaderResourceView {
    fn into(self) -> ::windows::Param<'a, ID3D11View> {
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
impl<'a> ::std::convert::Into<::windows::Param<'a, ID3D11DeviceChild>>
    for ID3D11ShaderResourceView
{
    fn into(self) -> ::windows::Param<'a, ID3D11DeviceChild> {
        ::windows::Param::Owned(::std::convert::Into::<ID3D11DeviceChild>::into(self))
    }
}
impl<'a> ::std::convert::Into<::windows::Param<'a, ID3D11DeviceChild>>
    for &'a ID3D11ShaderResourceView
{
    fn into(self) -> ::windows::Param<'a, ID3D11DeviceChild> {
        ::windows::Param::Owned(::std::convert::Into::<ID3D11DeviceChild>::into(
            ::std::clone::Clone::clone(self),
        ))
    }
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct D3D11_UAV_DIMENSION(pub i32);
impl ::std::convert::From<i32> for D3D11_UAV_DIMENSION {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
impl ::std::clone::Clone for D3D11_UAV_DIMENSION {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}
impl ::std::default::Default for D3D11_UAV_DIMENSION {
    fn default() -> Self {
        Self(0)
    }
}
impl ::std::fmt::Debug for D3D11_UAV_DIMENSION {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        write!(f, "{:?}", self.0)
    }
}
impl ::std::cmp::PartialEq for D3D11_UAV_DIMENSION {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::std::cmp::Eq for D3D11_UAV_DIMENSION {}
impl ::std::marker::Copy for D3D11_UAV_DIMENSION {}
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
unsafe impl ::windows::Abi for D3D11_UAV_DIMENSION {
    type Abi = Self;
}
#[repr(C)]
#[allow(non_camel_case_types)]
pub struct D3D11_BUFFER_UAV {
    pub first_element: u32,
    pub num_elements: u32,
    pub flags: u32,
}
#[repr(C)]
#[doc(hidden)]
#[allow(non_camel_case_types)]
pub struct D3D11_BUFFER_UAV_abi(u32, u32, u32);
impl D3D11_BUFFER_UAV {}
unsafe impl ::windows::Abi for D3D11_BUFFER_UAV {
    type Abi = D3D11_BUFFER_UAV_abi;
}
impl ::core::default::Default for D3D11_BUFFER_UAV {
    fn default() -> Self {
        Self {
            first_element: 0,
            num_elements: 0,
            flags: 0,
        }
    }
}
impl ::core::fmt::Debug for D3D11_BUFFER_UAV {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("D3D11_BUFFER_UAV")
            .field("first_element", &format_args!("{:?}", self.first_element))
            .field("num_elements", &format_args!("{:?}", self.num_elements))
            .field("flags", &format_args!("{:?}", self.flags))
            .finish()
    }
}
impl ::core::clone::Clone for D3D11_BUFFER_UAV {
    fn clone(&self) -> Self {
        Self {
            first_element: self.first_element,
            num_elements: self.num_elements,
            flags: self.flags,
        }
    }
}
impl ::std::marker::Copy for D3D11_BUFFER_UAV {}
#[repr(C)]
#[allow(non_camel_case_types)]
pub struct D3D11_TEX1D_UAV {
    pub mip_slice: u32,
}
#[repr(C)]
#[doc(hidden)]
#[allow(non_camel_case_types)]
pub struct D3D11_TEX1D_UAV_abi(u32);
impl D3D11_TEX1D_UAV {}
unsafe impl ::windows::Abi for D3D11_TEX1D_UAV {
    type Abi = D3D11_TEX1D_UAV_abi;
}
impl ::core::default::Default for D3D11_TEX1D_UAV {
    fn default() -> Self {
        Self { mip_slice: 0 }
    }
}
impl ::core::fmt::Debug for D3D11_TEX1D_UAV {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("D3D11_TEX1D_UAV")
            .field("mip_slice", &format_args!("{:?}", self.mip_slice))
            .finish()
    }
}
impl ::core::clone::Clone for D3D11_TEX1D_UAV {
    fn clone(&self) -> Self {
        Self {
            mip_slice: self.mip_slice,
        }
    }
}
impl ::std::marker::Copy for D3D11_TEX1D_UAV {}
#[repr(C)]
#[allow(non_camel_case_types)]
pub struct D3D11_TEX1D_ARRAY_UAV {
    pub mip_slice: u32,
    pub first_array_slice: u32,
    pub array_size: u32,
}
#[repr(C)]
#[doc(hidden)]
#[allow(non_camel_case_types)]
pub struct D3D11_TEX1D_ARRAY_UAV_abi(u32, u32, u32);
impl D3D11_TEX1D_ARRAY_UAV {}
unsafe impl ::windows::Abi for D3D11_TEX1D_ARRAY_UAV {
    type Abi = D3D11_TEX1D_ARRAY_UAV_abi;
}
impl ::core::default::Default for D3D11_TEX1D_ARRAY_UAV {
    fn default() -> Self {
        Self {
            mip_slice: 0,
            first_array_slice: 0,
            array_size: 0,
        }
    }
}
impl ::core::fmt::Debug for D3D11_TEX1D_ARRAY_UAV {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("D3D11_TEX1D_ARRAY_UAV")
            .field("mip_slice", &format_args!("{:?}", self.mip_slice))
            .field(
                "first_array_slice",
                &format_args!("{:?}", self.first_array_slice),
            )
            .field("array_size", &format_args!("{:?}", self.array_size))
            .finish()
    }
}
impl ::core::clone::Clone for D3D11_TEX1D_ARRAY_UAV {
    fn clone(&self) -> Self {
        Self {
            mip_slice: self.mip_slice,
            first_array_slice: self.first_array_slice,
            array_size: self.array_size,
        }
    }
}
impl ::std::marker::Copy for D3D11_TEX1D_ARRAY_UAV {}
#[repr(C)]
#[allow(non_camel_case_types)]
pub struct D3D11_TEX2D_UAV {
    pub mip_slice: u32,
}
#[repr(C)]
#[doc(hidden)]
#[allow(non_camel_case_types)]
pub struct D3D11_TEX2D_UAV_abi(u32);
impl D3D11_TEX2D_UAV {}
unsafe impl ::windows::Abi for D3D11_TEX2D_UAV {
    type Abi = D3D11_TEX2D_UAV_abi;
}
impl ::core::default::Default for D3D11_TEX2D_UAV {
    fn default() -> Self {
        Self { mip_slice: 0 }
    }
}
impl ::core::fmt::Debug for D3D11_TEX2D_UAV {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("D3D11_TEX2D_UAV")
            .field("mip_slice", &format_args!("{:?}", self.mip_slice))
            .finish()
    }
}
impl ::core::clone::Clone for D3D11_TEX2D_UAV {
    fn clone(&self) -> Self {
        Self {
            mip_slice: self.mip_slice,
        }
    }
}
impl ::std::marker::Copy for D3D11_TEX2D_UAV {}
#[repr(C)]
#[allow(non_camel_case_types)]
pub struct D3D11_TEX2D_ARRAY_UAV {
    pub mip_slice: u32,
    pub first_array_slice: u32,
    pub array_size: u32,
}
#[repr(C)]
#[doc(hidden)]
#[allow(non_camel_case_types)]
pub struct D3D11_TEX2D_ARRAY_UAV_abi(u32, u32, u32);
impl D3D11_TEX2D_ARRAY_UAV {}
unsafe impl ::windows::Abi for D3D11_TEX2D_ARRAY_UAV {
    type Abi = D3D11_TEX2D_ARRAY_UAV_abi;
}
impl ::core::default::Default for D3D11_TEX2D_ARRAY_UAV {
    fn default() -> Self {
        Self {
            mip_slice: 0,
            first_array_slice: 0,
            array_size: 0,
        }
    }
}
impl ::core::fmt::Debug for D3D11_TEX2D_ARRAY_UAV {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("D3D11_TEX2D_ARRAY_UAV")
            .field("mip_slice", &format_args!("{:?}", self.mip_slice))
            .field(
                "first_array_slice",
                &format_args!("{:?}", self.first_array_slice),
            )
            .field("array_size", &format_args!("{:?}", self.array_size))
            .finish()
    }
}
impl ::core::clone::Clone for D3D11_TEX2D_ARRAY_UAV {
    fn clone(&self) -> Self {
        Self {
            mip_slice: self.mip_slice,
            first_array_slice: self.first_array_slice,
            array_size: self.array_size,
        }
    }
}
impl ::std::marker::Copy for D3D11_TEX2D_ARRAY_UAV {}
#[repr(C)]
#[allow(non_camel_case_types)]
pub struct D3D11_TEX3D_UAV {
    pub mip_slice: u32,
    pub first_wslice: u32,
    pub wsize: u32,
}
#[repr(C)]
#[doc(hidden)]
#[allow(non_camel_case_types)]
pub struct D3D11_TEX3D_UAV_abi(u32, u32, u32);
impl D3D11_TEX3D_UAV {}
unsafe impl ::windows::Abi for D3D11_TEX3D_UAV {
    type Abi = D3D11_TEX3D_UAV_abi;
}
impl ::core::default::Default for D3D11_TEX3D_UAV {
    fn default() -> Self {
        Self {
            mip_slice: 0,
            first_wslice: 0,
            wsize: 0,
        }
    }
}
impl ::core::fmt::Debug for D3D11_TEX3D_UAV {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("D3D11_TEX3D_UAV")
            .field("mip_slice", &format_args!("{:?}", self.mip_slice))
            .field("first_wslice", &format_args!("{:?}", self.first_wslice))
            .field("wsize", &format_args!("{:?}", self.wsize))
            .finish()
    }
}
impl ::core::clone::Clone for D3D11_TEX3D_UAV {
    fn clone(&self) -> Self {
        Self {
            mip_slice: self.mip_slice,
            first_wslice: self.first_wslice,
            wsize: self.wsize,
        }
    }
}
impl ::std::marker::Copy for D3D11_TEX3D_UAV {}
#[repr(C)]
#[allow(non_camel_case_types)]
pub struct D3D11_UNORDERED_ACCESS_VIEW_DESC {
    pub format: super::dxgi::DXGI_FORMAT,
    pub view_dimension: D3D11_UAV_DIMENSION,
    pub anonymous: D3D11_UNORDERED_ACCESS_VIEW_DESC_0,
}
#[repr(C)]
#[doc(hidden)]
#[allow(non_camel_case_types)]
pub struct D3D11_UNORDERED_ACCESS_VIEW_DESC_abi(
    super::dxgi::DXGI_FORMAT,
    D3D11_UAV_DIMENSION,
    D3D11_UNORDERED_ACCESS_VIEW_DESC_0_abi,
);
impl D3D11_UNORDERED_ACCESS_VIEW_DESC {}
unsafe impl ::windows::Abi for D3D11_UNORDERED_ACCESS_VIEW_DESC {
    type Abi = D3D11_UNORDERED_ACCESS_VIEW_DESC_abi;
}
impl ::core::fmt::Debug for D3D11_UNORDERED_ACCESS_VIEW_DESC {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("D3D11_UNORDERED_ACCESS_VIEW_DESC")
            .field("format", &format_args!("{:?}", self.format))
            .field("view_dimension", &format_args!("{:?}", self.view_dimension))
            .field("anonymous", &format_args!("{:?}", self.anonymous))
            .finish()
    }
}
impl ::core::default::Default for D3D11_UNORDERED_ACCESS_VIEW_DESC {
    fn default() -> Self {
        Self {
            format: ::std::default::Default::default(),
            view_dimension: ::std::default::Default::default(),
            anonymous: ::std::default::Default::default(),
        }
    }
}
impl ::core::clone::Clone for D3D11_UNORDERED_ACCESS_VIEW_DESC {
    fn clone(&self) -> Self {
        let mut out = Self::default();
        out.clone_from(self);
        out
    }
    fn clone_from(&mut self, source: &Self) {
        unsafe { std::ptr::copy(source, self, std::mem::size_of::<Self>()) }
    }
}
#[repr(C)]
#[allow(non_camel_case_types)]
pub union D3D11_UNORDERED_ACCESS_VIEW_DESC_0 {
    pub buffer: ::core::mem::ManuallyDrop<D3D11_BUFFER_UAV>,
    pub texture1d: ::core::mem::ManuallyDrop<D3D11_TEX1D_UAV>,
    pub texture1d_array: ::core::mem::ManuallyDrop<D3D11_TEX1D_ARRAY_UAV>,
    pub texture2d: ::core::mem::ManuallyDrop<D3D11_TEX2D_UAV>,
    pub texture2d_array: ::core::mem::ManuallyDrop<D3D11_TEX2D_ARRAY_UAV>,
    pub texture3d: ::core::mem::ManuallyDrop<D3D11_TEX3D_UAV>,
}
#[repr(C)]
#[doc(hidden)]
#[allow(non_camel_case_types)]
pub struct D3D11_UNORDERED_ACCESS_VIEW_DESC_0_abi(
    D3D11_BUFFER_UAV_abi,
    D3D11_TEX1D_UAV_abi,
    D3D11_TEX1D_ARRAY_UAV_abi,
    D3D11_TEX2D_UAV_abi,
    D3D11_TEX2D_ARRAY_UAV_abi,
    D3D11_TEX3D_UAV_abi,
);
impl D3D11_UNORDERED_ACCESS_VIEW_DESC_0 {}
unsafe impl ::windows::Abi for D3D11_UNORDERED_ACCESS_VIEW_DESC_0 {
    type Abi = D3D11_UNORDERED_ACCESS_VIEW_DESC_0_abi;
}
impl ::core::fmt::Debug for D3D11_UNORDERED_ACCESS_VIEW_DESC_0 {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("D3D11_UNORDERED_ACCESS_VIEW_DESC_0")
            .finish()
    }
}
impl ::core::default::Default for D3D11_UNORDERED_ACCESS_VIEW_DESC_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::clone::Clone for D3D11_UNORDERED_ACCESS_VIEW_DESC_0 {
    fn clone(&self) -> Self {
        let mut out = Self::default();
        out.clone_from(self);
        out
    }
    fn clone_from(&mut self, source: &Self) {
        unsafe { std::ptr::copy(source, self, std::mem::size_of::<Self>()) }
    }
}
#[repr(transparent)]
#[allow(non_camel_case_types)]
pub struct ID3D11UnorderedAccessView(::windows::IUnknown);
impl ::std::clone::Clone for ID3D11UnorderedAccessView {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::std::fmt::Debug for ID3D11UnorderedAccessView {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        write!(f, "{:?}", self.0)
    }
}
impl ::std::cmp::PartialEq for ID3D11UnorderedAccessView {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::std::cmp::Eq for ID3D11UnorderedAccessView {}
unsafe impl ::windows::Interface for ID3D11UnorderedAccessView {
    type Vtable = ID3D11UnorderedAccessView_abi;
    const IID: ::windows::Guid =
        ::windows::Guid::from_values(682423561, 32604, 18678, [134, 17, 243, 22, 1, 10, 99, 128]);
}
#[repr(C)]
pub struct ID3D11UnorderedAccessView_abi(
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        iid: &::windows::Guid,
        interface: *mut ::windows::RawPtr,
    ) -> ::windows::ErrorCode,
    pub unsafe extern "system" fn(this: ::windows::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::RawPtr) -> u32,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        pp_device: *mut ::std::option::Option<ID3D11Device>,
    ),
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
        pp_resource: *mut ::std::option::Option<ID3D11Resource>,
    ),
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        p_desc: *mut D3D11_UNORDERED_ACCESS_VIEW_DESC,
    ),
);
#[allow(non_snake_case)]
impl ID3D11UnorderedAccessView {
    pub unsafe fn GetDevice(&self, pp_device: *mut ::std::option::Option<ID3D11Device>) {
        (::windows::Interface::vtable(self).3)(::windows::Abi::abi(self), pp_device)
    }
    pub unsafe fn GetPrivateData(
        &self,
        guid: *const ::windows::Guid,
        p_data_size: *mut u32,
        p_data: *mut ::std::ffi::c_void,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).4)(::windows::Abi::abi(self), guid, p_data_size, p_data)
    }
    pub unsafe fn SetPrivateData(
        &self,
        guid: *const ::windows::Guid,
        data_size: u32,
        p_data: *const ::std::ffi::c_void,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).5)(::windows::Abi::abi(self), guid, data_size, p_data)
    }
    pub unsafe fn SetPrivateDataInterface<
        'a,
        T1__: ::std::convert::Into<::windows::Param<'a, ::windows::IUnknown>>,
    >(
        &self,
        guid: *const ::windows::Guid,
        p_data: T1__,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).6)(::windows::Abi::abi(self), guid, p_data.into().abi())
    }
    pub unsafe fn GetResource(&self, pp_resource: *mut ::std::option::Option<ID3D11Resource>) {
        (::windows::Interface::vtable(self).7)(::windows::Abi::abi(self), pp_resource)
    }
    pub unsafe fn GetDesc(&self, p_desc: *mut D3D11_UNORDERED_ACCESS_VIEW_DESC) {
        (::windows::Interface::vtable(self).8)(::windows::Abi::abi(self), p_desc)
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
impl<'a> ::std::convert::Into<::windows::Param<'a, ::windows::IUnknown>>
    for ID3D11UnorderedAccessView
{
    fn into(self) -> ::windows::Param<'a, ::windows::IUnknown> {
        ::windows::Param::Owned(::std::convert::Into::<::windows::IUnknown>::into(self))
    }
}
impl<'a> ::std::convert::Into<::windows::Param<'a, ::windows::IUnknown>>
    for &'a ID3D11UnorderedAccessView
{
    fn into(self) -> ::windows::Param<'a, ::windows::IUnknown> {
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
impl<'a> ::std::convert::Into<::windows::Param<'a, ID3D11View>> for ID3D11UnorderedAccessView {
    fn into(self) -> ::windows::Param<'a, ID3D11View> {
        ::windows::Param::Owned(::std::convert::Into::<ID3D11View>::into(self))
    }
}
impl<'a> ::std::convert::Into<::windows::Param<'a, ID3D11View>> for &'a ID3D11UnorderedAccessView {
    fn into(self) -> ::windows::Param<'a, ID3D11View> {
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
impl<'a> ::std::convert::Into<::windows::Param<'a, ID3D11DeviceChild>>
    for ID3D11UnorderedAccessView
{
    fn into(self) -> ::windows::Param<'a, ID3D11DeviceChild> {
        ::windows::Param::Owned(::std::convert::Into::<ID3D11DeviceChild>::into(self))
    }
}
impl<'a> ::std::convert::Into<::windows::Param<'a, ID3D11DeviceChild>>
    for &'a ID3D11UnorderedAccessView
{
    fn into(self) -> ::windows::Param<'a, ID3D11DeviceChild> {
        ::windows::Param::Owned(::std::convert::Into::<ID3D11DeviceChild>::into(
            ::std::clone::Clone::clone(self),
        ))
    }
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct D3D11_RTV_DIMENSION(pub i32);
impl ::std::convert::From<i32> for D3D11_RTV_DIMENSION {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
impl ::std::clone::Clone for D3D11_RTV_DIMENSION {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}
impl ::std::default::Default for D3D11_RTV_DIMENSION {
    fn default() -> Self {
        Self(0)
    }
}
impl ::std::fmt::Debug for D3D11_RTV_DIMENSION {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        write!(f, "{:?}", self.0)
    }
}
impl ::std::cmp::PartialEq for D3D11_RTV_DIMENSION {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::std::cmp::Eq for D3D11_RTV_DIMENSION {}
impl ::std::marker::Copy for D3D11_RTV_DIMENSION {}
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
unsafe impl ::windows::Abi for D3D11_RTV_DIMENSION {
    type Abi = Self;
}
#[repr(C)]
#[allow(non_camel_case_types)]
pub struct D3D11_BUFFER_RTV {
    pub anonymous1: D3D11_BUFFER_RTV_1,
    pub anonymous2: ::windows::NOT_YET_SUPPORTED_TYPE,
}
#[repr(C)]
#[doc(hidden)]
#[allow(non_camel_case_types)]
pub struct D3D11_BUFFER_RTV_abi(D3D11_BUFFER_RTV_1_abi, ::windows::NOT_YET_SUPPORTED_TYPE);
impl D3D11_BUFFER_RTV {}
unsafe impl ::windows::Abi for D3D11_BUFFER_RTV {
    type Abi = D3D11_BUFFER_RTV_abi;
}
impl ::core::fmt::Debug for D3D11_BUFFER_RTV {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("D3D11_BUFFER_RTV")
            .field("anonymous1", &format_args!("{:?}", self.anonymous1))
            .field("anonymous2", &format_args!("{:?}", self.anonymous2))
            .finish()
    }
}
impl ::core::default::Default for D3D11_BUFFER_RTV {
    fn default() -> Self {
        Self {
            anonymous1: ::std::default::Default::default(),
            anonymous2: ::std::default::Default::default(),
        }
    }
}
impl ::core::clone::Clone for D3D11_BUFFER_RTV {
    fn clone(&self) -> Self {
        let mut out = Self::default();
        out.clone_from(self);
        out
    }
    fn clone_from(&mut self, source: &Self) {
        unsafe { std::ptr::copy(source, self, std::mem::size_of::<Self>()) }
    }
}
#[repr(C)]
#[allow(non_camel_case_types)]
pub union D3D11_BUFFER_RTV_1 {
    pub first_element: ::core::mem::ManuallyDrop<u32>,
    pub element_offset: ::core::mem::ManuallyDrop<u32>,
}
#[repr(C)]
#[doc(hidden)]
#[allow(non_camel_case_types)]
pub struct D3D11_BUFFER_RTV_1_abi(u32, u32);
impl D3D11_BUFFER_RTV_1 {}
unsafe impl ::windows::Abi for D3D11_BUFFER_RTV_1 {
    type Abi = D3D11_BUFFER_RTV_1_abi;
}
impl ::core::fmt::Debug for D3D11_BUFFER_RTV_1 {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("D3D11_BUFFER_RTV_1").finish()
    }
}
impl ::core::default::Default for D3D11_BUFFER_RTV_1 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::clone::Clone for D3D11_BUFFER_RTV_1 {
    fn clone(&self) -> Self {
        let mut out = Self::default();
        out.clone_from(self);
        out
    }
    fn clone_from(&mut self, source: &Self) {
        unsafe { std::ptr::copy(source, self, std::mem::size_of::<Self>()) }
    }
}
#[repr(C)]
#[allow(non_camel_case_types)]
pub struct D3D11_TEX1D_RTV {
    pub mip_slice: u32,
}
#[repr(C)]
#[doc(hidden)]
#[allow(non_camel_case_types)]
pub struct D3D11_TEX1D_RTV_abi(u32);
impl D3D11_TEX1D_RTV {}
unsafe impl ::windows::Abi for D3D11_TEX1D_RTV {
    type Abi = D3D11_TEX1D_RTV_abi;
}
impl ::core::default::Default for D3D11_TEX1D_RTV {
    fn default() -> Self {
        Self { mip_slice: 0 }
    }
}
impl ::core::fmt::Debug for D3D11_TEX1D_RTV {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("D3D11_TEX1D_RTV")
            .field("mip_slice", &format_args!("{:?}", self.mip_slice))
            .finish()
    }
}
impl ::core::clone::Clone for D3D11_TEX1D_RTV {
    fn clone(&self) -> Self {
        Self {
            mip_slice: self.mip_slice,
        }
    }
}
impl ::std::marker::Copy for D3D11_TEX1D_RTV {}
#[repr(C)]
#[allow(non_camel_case_types)]
pub struct D3D11_TEX1D_ARRAY_RTV {
    pub mip_slice: u32,
    pub first_array_slice: u32,
    pub array_size: u32,
}
#[repr(C)]
#[doc(hidden)]
#[allow(non_camel_case_types)]
pub struct D3D11_TEX1D_ARRAY_RTV_abi(u32, u32, u32);
impl D3D11_TEX1D_ARRAY_RTV {}
unsafe impl ::windows::Abi for D3D11_TEX1D_ARRAY_RTV {
    type Abi = D3D11_TEX1D_ARRAY_RTV_abi;
}
impl ::core::default::Default for D3D11_TEX1D_ARRAY_RTV {
    fn default() -> Self {
        Self {
            mip_slice: 0,
            first_array_slice: 0,
            array_size: 0,
        }
    }
}
impl ::core::fmt::Debug for D3D11_TEX1D_ARRAY_RTV {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("D3D11_TEX1D_ARRAY_RTV")
            .field("mip_slice", &format_args!("{:?}", self.mip_slice))
            .field(
                "first_array_slice",
                &format_args!("{:?}", self.first_array_slice),
            )
            .field("array_size", &format_args!("{:?}", self.array_size))
            .finish()
    }
}
impl ::core::clone::Clone for D3D11_TEX1D_ARRAY_RTV {
    fn clone(&self) -> Self {
        Self {
            mip_slice: self.mip_slice,
            first_array_slice: self.first_array_slice,
            array_size: self.array_size,
        }
    }
}
impl ::std::marker::Copy for D3D11_TEX1D_ARRAY_RTV {}
#[repr(C)]
#[allow(non_camel_case_types)]
pub struct D3D11_TEX2D_RTV {
    pub mip_slice: u32,
}
#[repr(C)]
#[doc(hidden)]
#[allow(non_camel_case_types)]
pub struct D3D11_TEX2D_RTV_abi(u32);
impl D3D11_TEX2D_RTV {}
unsafe impl ::windows::Abi for D3D11_TEX2D_RTV {
    type Abi = D3D11_TEX2D_RTV_abi;
}
impl ::core::default::Default for D3D11_TEX2D_RTV {
    fn default() -> Self {
        Self { mip_slice: 0 }
    }
}
impl ::core::fmt::Debug for D3D11_TEX2D_RTV {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("D3D11_TEX2D_RTV")
            .field("mip_slice", &format_args!("{:?}", self.mip_slice))
            .finish()
    }
}
impl ::core::clone::Clone for D3D11_TEX2D_RTV {
    fn clone(&self) -> Self {
        Self {
            mip_slice: self.mip_slice,
        }
    }
}
impl ::std::marker::Copy for D3D11_TEX2D_RTV {}
#[repr(C)]
#[allow(non_camel_case_types)]
pub struct D3D11_TEX2D_ARRAY_RTV {
    pub mip_slice: u32,
    pub first_array_slice: u32,
    pub array_size: u32,
}
#[repr(C)]
#[doc(hidden)]
#[allow(non_camel_case_types)]
pub struct D3D11_TEX2D_ARRAY_RTV_abi(u32, u32, u32);
impl D3D11_TEX2D_ARRAY_RTV {}
unsafe impl ::windows::Abi for D3D11_TEX2D_ARRAY_RTV {
    type Abi = D3D11_TEX2D_ARRAY_RTV_abi;
}
impl ::core::default::Default for D3D11_TEX2D_ARRAY_RTV {
    fn default() -> Self {
        Self {
            mip_slice: 0,
            first_array_slice: 0,
            array_size: 0,
        }
    }
}
impl ::core::fmt::Debug for D3D11_TEX2D_ARRAY_RTV {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("D3D11_TEX2D_ARRAY_RTV")
            .field("mip_slice", &format_args!("{:?}", self.mip_slice))
            .field(
                "first_array_slice",
                &format_args!("{:?}", self.first_array_slice),
            )
            .field("array_size", &format_args!("{:?}", self.array_size))
            .finish()
    }
}
impl ::core::clone::Clone for D3D11_TEX2D_ARRAY_RTV {
    fn clone(&self) -> Self {
        Self {
            mip_slice: self.mip_slice,
            first_array_slice: self.first_array_slice,
            array_size: self.array_size,
        }
    }
}
impl ::std::marker::Copy for D3D11_TEX2D_ARRAY_RTV {}
#[repr(C)]
#[allow(non_camel_case_types)]
pub struct D3D11_TEX2DMS_RTV {
    pub unused_field_nothing_to_define: u32,
}
#[repr(C)]
#[doc(hidden)]
#[allow(non_camel_case_types)]
pub struct D3D11_TEX2DMS_RTV_abi(u32);
impl D3D11_TEX2DMS_RTV {}
unsafe impl ::windows::Abi for D3D11_TEX2DMS_RTV {
    type Abi = D3D11_TEX2DMS_RTV_abi;
}
impl ::core::default::Default for D3D11_TEX2DMS_RTV {
    fn default() -> Self {
        Self {
            unused_field_nothing_to_define: 0,
        }
    }
}
impl ::core::fmt::Debug for D3D11_TEX2DMS_RTV {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("D3D11_TEX2DMS_RTV")
            .field(
                "unused_field_nothing_to_define",
                &format_args!("{:?}", self.unused_field_nothing_to_define),
            )
            .finish()
    }
}
impl ::core::clone::Clone for D3D11_TEX2DMS_RTV {
    fn clone(&self) -> Self {
        Self {
            unused_field_nothing_to_define: self.unused_field_nothing_to_define,
        }
    }
}
impl ::std::marker::Copy for D3D11_TEX2DMS_RTV {}
#[repr(C)]
#[allow(non_camel_case_types)]
pub struct D3D11_TEX2DMS_ARRAY_RTV {
    pub first_array_slice: u32,
    pub array_size: u32,
}
#[repr(C)]
#[doc(hidden)]
#[allow(non_camel_case_types)]
pub struct D3D11_TEX2DMS_ARRAY_RTV_abi(u32, u32);
impl D3D11_TEX2DMS_ARRAY_RTV {}
unsafe impl ::windows::Abi for D3D11_TEX2DMS_ARRAY_RTV {
    type Abi = D3D11_TEX2DMS_ARRAY_RTV_abi;
}
impl ::core::default::Default for D3D11_TEX2DMS_ARRAY_RTV {
    fn default() -> Self {
        Self {
            first_array_slice: 0,
            array_size: 0,
        }
    }
}
impl ::core::fmt::Debug for D3D11_TEX2DMS_ARRAY_RTV {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("D3D11_TEX2DMS_ARRAY_RTV")
            .field(
                "first_array_slice",
                &format_args!("{:?}", self.first_array_slice),
            )
            .field("array_size", &format_args!("{:?}", self.array_size))
            .finish()
    }
}
impl ::core::clone::Clone for D3D11_TEX2DMS_ARRAY_RTV {
    fn clone(&self) -> Self {
        Self {
            first_array_slice: self.first_array_slice,
            array_size: self.array_size,
        }
    }
}
impl ::std::marker::Copy for D3D11_TEX2DMS_ARRAY_RTV {}
#[repr(C)]
#[allow(non_camel_case_types)]
pub struct D3D11_TEX3D_RTV {
    pub mip_slice: u32,
    pub first_wslice: u32,
    pub wsize: u32,
}
#[repr(C)]
#[doc(hidden)]
#[allow(non_camel_case_types)]
pub struct D3D11_TEX3D_RTV_abi(u32, u32, u32);
impl D3D11_TEX3D_RTV {}
unsafe impl ::windows::Abi for D3D11_TEX3D_RTV {
    type Abi = D3D11_TEX3D_RTV_abi;
}
impl ::core::default::Default for D3D11_TEX3D_RTV {
    fn default() -> Self {
        Self {
            mip_slice: 0,
            first_wslice: 0,
            wsize: 0,
        }
    }
}
impl ::core::fmt::Debug for D3D11_TEX3D_RTV {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("D3D11_TEX3D_RTV")
            .field("mip_slice", &format_args!("{:?}", self.mip_slice))
            .field("first_wslice", &format_args!("{:?}", self.first_wslice))
            .field("wsize", &format_args!("{:?}", self.wsize))
            .finish()
    }
}
impl ::core::clone::Clone for D3D11_TEX3D_RTV {
    fn clone(&self) -> Self {
        Self {
            mip_slice: self.mip_slice,
            first_wslice: self.first_wslice,
            wsize: self.wsize,
        }
    }
}
impl ::std::marker::Copy for D3D11_TEX3D_RTV {}
#[repr(C)]
#[allow(non_camel_case_types)]
pub struct D3D11_RENDER_TARGET_VIEW_DESC {
    pub format: super::dxgi::DXGI_FORMAT,
    pub view_dimension: D3D11_RTV_DIMENSION,
    pub anonymous: D3D11_RENDER_TARGET_VIEW_DESC_0,
}
#[repr(C)]
#[doc(hidden)]
#[allow(non_camel_case_types)]
pub struct D3D11_RENDER_TARGET_VIEW_DESC_abi(
    super::dxgi::DXGI_FORMAT,
    D3D11_RTV_DIMENSION,
    D3D11_RENDER_TARGET_VIEW_DESC_0_abi,
);
impl D3D11_RENDER_TARGET_VIEW_DESC {}
unsafe impl ::windows::Abi for D3D11_RENDER_TARGET_VIEW_DESC {
    type Abi = D3D11_RENDER_TARGET_VIEW_DESC_abi;
}
impl ::core::fmt::Debug for D3D11_RENDER_TARGET_VIEW_DESC {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("D3D11_RENDER_TARGET_VIEW_DESC")
            .field("format", &format_args!("{:?}", self.format))
            .field("view_dimension", &format_args!("{:?}", self.view_dimension))
            .field("anonymous", &format_args!("{:?}", self.anonymous))
            .finish()
    }
}
impl ::core::default::Default for D3D11_RENDER_TARGET_VIEW_DESC {
    fn default() -> Self {
        Self {
            format: ::std::default::Default::default(),
            view_dimension: ::std::default::Default::default(),
            anonymous: ::std::default::Default::default(),
        }
    }
}
impl ::core::clone::Clone for D3D11_RENDER_TARGET_VIEW_DESC {
    fn clone(&self) -> Self {
        let mut out = Self::default();
        out.clone_from(self);
        out
    }
    fn clone_from(&mut self, source: &Self) {
        unsafe { std::ptr::copy(source, self, std::mem::size_of::<Self>()) }
    }
}
#[repr(C)]
#[allow(non_camel_case_types)]
pub union D3D11_RENDER_TARGET_VIEW_DESC_0 {
    pub buffer: ::core::mem::ManuallyDrop<D3D11_BUFFER_RTV>,
    pub texture1d: ::core::mem::ManuallyDrop<D3D11_TEX1D_RTV>,
    pub texture1d_array: ::core::mem::ManuallyDrop<D3D11_TEX1D_ARRAY_RTV>,
    pub texture2d: ::core::mem::ManuallyDrop<D3D11_TEX2D_RTV>,
    pub texture2d_array: ::core::mem::ManuallyDrop<D3D11_TEX2D_ARRAY_RTV>,
    pub texture2dms: ::core::mem::ManuallyDrop<D3D11_TEX2DMS_RTV>,
    pub texture2dms_array: ::core::mem::ManuallyDrop<D3D11_TEX2DMS_ARRAY_RTV>,
    pub texture3d: ::core::mem::ManuallyDrop<D3D11_TEX3D_RTV>,
}
#[repr(C)]
#[doc(hidden)]
#[allow(non_camel_case_types)]
pub struct D3D11_RENDER_TARGET_VIEW_DESC_0_abi(
    D3D11_BUFFER_RTV_abi,
    D3D11_TEX1D_RTV_abi,
    D3D11_TEX1D_ARRAY_RTV_abi,
    D3D11_TEX2D_RTV_abi,
    D3D11_TEX2D_ARRAY_RTV_abi,
    D3D11_TEX2DMS_RTV_abi,
    D3D11_TEX2DMS_ARRAY_RTV_abi,
    D3D11_TEX3D_RTV_abi,
);
impl D3D11_RENDER_TARGET_VIEW_DESC_0 {}
unsafe impl ::windows::Abi for D3D11_RENDER_TARGET_VIEW_DESC_0 {
    type Abi = D3D11_RENDER_TARGET_VIEW_DESC_0_abi;
}
impl ::core::fmt::Debug for D3D11_RENDER_TARGET_VIEW_DESC_0 {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("D3D11_RENDER_TARGET_VIEW_DESC_0").finish()
    }
}
impl ::core::default::Default for D3D11_RENDER_TARGET_VIEW_DESC_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::clone::Clone for D3D11_RENDER_TARGET_VIEW_DESC_0 {
    fn clone(&self) -> Self {
        let mut out = Self::default();
        out.clone_from(self);
        out
    }
    fn clone_from(&mut self, source: &Self) {
        unsafe { std::ptr::copy(source, self, std::mem::size_of::<Self>()) }
    }
}
#[repr(transparent)]
#[allow(non_camel_case_types)]
pub struct ID3D11RenderTargetView(::windows::IUnknown);
impl ::std::clone::Clone for ID3D11RenderTargetView {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::std::fmt::Debug for ID3D11RenderTargetView {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        write!(f, "{:?}", self.0)
    }
}
impl ::std::cmp::PartialEq for ID3D11RenderTargetView {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::std::cmp::Eq for ID3D11RenderTargetView {}
unsafe impl ::windows::Interface for ID3D11RenderTargetView {
    type Vtable = ID3D11RenderTargetView_abi;
    const IID: ::windows::Guid = ::windows::Guid::from_values(
        3755712615,
        2957,
        18533,
        [135, 91, 215, 180, 81, 108, 193, 100],
    );
}
#[repr(C)]
pub struct ID3D11RenderTargetView_abi(
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        iid: &::windows::Guid,
        interface: *mut ::windows::RawPtr,
    ) -> ::windows::ErrorCode,
    pub unsafe extern "system" fn(this: ::windows::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::RawPtr) -> u32,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        pp_device: *mut ::std::option::Option<ID3D11Device>,
    ),
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
        pp_resource: *mut ::std::option::Option<ID3D11Resource>,
    ),
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        p_desc: *mut D3D11_RENDER_TARGET_VIEW_DESC,
    ),
);
#[allow(non_snake_case)]
impl ID3D11RenderTargetView {
    pub unsafe fn GetDevice(&self, pp_device: *mut ::std::option::Option<ID3D11Device>) {
        (::windows::Interface::vtable(self).3)(::windows::Abi::abi(self), pp_device)
    }
    pub unsafe fn GetPrivateData(
        &self,
        guid: *const ::windows::Guid,
        p_data_size: *mut u32,
        p_data: *mut ::std::ffi::c_void,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).4)(::windows::Abi::abi(self), guid, p_data_size, p_data)
    }
    pub unsafe fn SetPrivateData(
        &self,
        guid: *const ::windows::Guid,
        data_size: u32,
        p_data: *const ::std::ffi::c_void,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).5)(::windows::Abi::abi(self), guid, data_size, p_data)
    }
    pub unsafe fn SetPrivateDataInterface<
        'a,
        T1__: ::std::convert::Into<::windows::Param<'a, ::windows::IUnknown>>,
    >(
        &self,
        guid: *const ::windows::Guid,
        p_data: T1__,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).6)(::windows::Abi::abi(self), guid, p_data.into().abi())
    }
    pub unsafe fn GetResource(&self, pp_resource: *mut ::std::option::Option<ID3D11Resource>) {
        (::windows::Interface::vtable(self).7)(::windows::Abi::abi(self), pp_resource)
    }
    pub unsafe fn GetDesc(&self, p_desc: *mut D3D11_RENDER_TARGET_VIEW_DESC) {
        (::windows::Interface::vtable(self).8)(::windows::Abi::abi(self), p_desc)
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
impl<'a> ::std::convert::Into<::windows::Param<'a, ::windows::IUnknown>>
    for ID3D11RenderTargetView
{
    fn into(self) -> ::windows::Param<'a, ::windows::IUnknown> {
        ::windows::Param::Owned(::std::convert::Into::<::windows::IUnknown>::into(self))
    }
}
impl<'a> ::std::convert::Into<::windows::Param<'a, ::windows::IUnknown>>
    for &'a ID3D11RenderTargetView
{
    fn into(self) -> ::windows::Param<'a, ::windows::IUnknown> {
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
impl<'a> ::std::convert::Into<::windows::Param<'a, ID3D11View>> for ID3D11RenderTargetView {
    fn into(self) -> ::windows::Param<'a, ID3D11View> {
        ::windows::Param::Owned(::std::convert::Into::<ID3D11View>::into(self))
    }
}
impl<'a> ::std::convert::Into<::windows::Param<'a, ID3D11View>> for &'a ID3D11RenderTargetView {
    fn into(self) -> ::windows::Param<'a, ID3D11View> {
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
impl<'a> ::std::convert::Into<::windows::Param<'a, ID3D11DeviceChild>> for ID3D11RenderTargetView {
    fn into(self) -> ::windows::Param<'a, ID3D11DeviceChild> {
        ::windows::Param::Owned(::std::convert::Into::<ID3D11DeviceChild>::into(self))
    }
}
impl<'a> ::std::convert::Into<::windows::Param<'a, ID3D11DeviceChild>>
    for &'a ID3D11RenderTargetView
{
    fn into(self) -> ::windows::Param<'a, ID3D11DeviceChild> {
        ::windows::Param::Owned(::std::convert::Into::<ID3D11DeviceChild>::into(
            ::std::clone::Clone::clone(self),
        ))
    }
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct D3D11_DSV_DIMENSION(pub i32);
impl ::std::convert::From<i32> for D3D11_DSV_DIMENSION {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
impl ::std::clone::Clone for D3D11_DSV_DIMENSION {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}
impl ::std::default::Default for D3D11_DSV_DIMENSION {
    fn default() -> Self {
        Self(0)
    }
}
impl ::std::fmt::Debug for D3D11_DSV_DIMENSION {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        write!(f, "{:?}", self.0)
    }
}
impl ::std::cmp::PartialEq for D3D11_DSV_DIMENSION {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::std::cmp::Eq for D3D11_DSV_DIMENSION {}
impl ::std::marker::Copy for D3D11_DSV_DIMENSION {}
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
unsafe impl ::windows::Abi for D3D11_DSV_DIMENSION {
    type Abi = Self;
}
#[repr(C)]
#[allow(non_camel_case_types)]
pub struct D3D11_TEX1D_DSV {
    pub mip_slice: u32,
}
#[repr(C)]
#[doc(hidden)]
#[allow(non_camel_case_types)]
pub struct D3D11_TEX1D_DSV_abi(u32);
impl D3D11_TEX1D_DSV {}
unsafe impl ::windows::Abi for D3D11_TEX1D_DSV {
    type Abi = D3D11_TEX1D_DSV_abi;
}
impl ::core::default::Default for D3D11_TEX1D_DSV {
    fn default() -> Self {
        Self { mip_slice: 0 }
    }
}
impl ::core::fmt::Debug for D3D11_TEX1D_DSV {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("D3D11_TEX1D_DSV")
            .field("mip_slice", &format_args!("{:?}", self.mip_slice))
            .finish()
    }
}
impl ::core::clone::Clone for D3D11_TEX1D_DSV {
    fn clone(&self) -> Self {
        Self {
            mip_slice: self.mip_slice,
        }
    }
}
impl ::std::marker::Copy for D3D11_TEX1D_DSV {}
#[repr(C)]
#[allow(non_camel_case_types)]
pub struct D3D11_TEX1D_ARRAY_DSV {
    pub mip_slice: u32,
    pub first_array_slice: u32,
    pub array_size: u32,
}
#[repr(C)]
#[doc(hidden)]
#[allow(non_camel_case_types)]
pub struct D3D11_TEX1D_ARRAY_DSV_abi(u32, u32, u32);
impl D3D11_TEX1D_ARRAY_DSV {}
unsafe impl ::windows::Abi for D3D11_TEX1D_ARRAY_DSV {
    type Abi = D3D11_TEX1D_ARRAY_DSV_abi;
}
impl ::core::default::Default for D3D11_TEX1D_ARRAY_DSV {
    fn default() -> Self {
        Self {
            mip_slice: 0,
            first_array_slice: 0,
            array_size: 0,
        }
    }
}
impl ::core::fmt::Debug for D3D11_TEX1D_ARRAY_DSV {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("D3D11_TEX1D_ARRAY_DSV")
            .field("mip_slice", &format_args!("{:?}", self.mip_slice))
            .field(
                "first_array_slice",
                &format_args!("{:?}", self.first_array_slice),
            )
            .field("array_size", &format_args!("{:?}", self.array_size))
            .finish()
    }
}
impl ::core::clone::Clone for D3D11_TEX1D_ARRAY_DSV {
    fn clone(&self) -> Self {
        Self {
            mip_slice: self.mip_slice,
            first_array_slice: self.first_array_slice,
            array_size: self.array_size,
        }
    }
}
impl ::std::marker::Copy for D3D11_TEX1D_ARRAY_DSV {}
#[repr(C)]
#[allow(non_camel_case_types)]
pub struct D3D11_TEX2D_DSV {
    pub mip_slice: u32,
}
#[repr(C)]
#[doc(hidden)]
#[allow(non_camel_case_types)]
pub struct D3D11_TEX2D_DSV_abi(u32);
impl D3D11_TEX2D_DSV {}
unsafe impl ::windows::Abi for D3D11_TEX2D_DSV {
    type Abi = D3D11_TEX2D_DSV_abi;
}
impl ::core::default::Default for D3D11_TEX2D_DSV {
    fn default() -> Self {
        Self { mip_slice: 0 }
    }
}
impl ::core::fmt::Debug for D3D11_TEX2D_DSV {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("D3D11_TEX2D_DSV")
            .field("mip_slice", &format_args!("{:?}", self.mip_slice))
            .finish()
    }
}
impl ::core::clone::Clone for D3D11_TEX2D_DSV {
    fn clone(&self) -> Self {
        Self {
            mip_slice: self.mip_slice,
        }
    }
}
impl ::std::marker::Copy for D3D11_TEX2D_DSV {}
#[repr(C)]
#[allow(non_camel_case_types)]
pub struct D3D11_TEX2D_ARRAY_DSV {
    pub mip_slice: u32,
    pub first_array_slice: u32,
    pub array_size: u32,
}
#[repr(C)]
#[doc(hidden)]
#[allow(non_camel_case_types)]
pub struct D3D11_TEX2D_ARRAY_DSV_abi(u32, u32, u32);
impl D3D11_TEX2D_ARRAY_DSV {}
unsafe impl ::windows::Abi for D3D11_TEX2D_ARRAY_DSV {
    type Abi = D3D11_TEX2D_ARRAY_DSV_abi;
}
impl ::core::default::Default for D3D11_TEX2D_ARRAY_DSV {
    fn default() -> Self {
        Self {
            mip_slice: 0,
            first_array_slice: 0,
            array_size: 0,
        }
    }
}
impl ::core::fmt::Debug for D3D11_TEX2D_ARRAY_DSV {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("D3D11_TEX2D_ARRAY_DSV")
            .field("mip_slice", &format_args!("{:?}", self.mip_slice))
            .field(
                "first_array_slice",
                &format_args!("{:?}", self.first_array_slice),
            )
            .field("array_size", &format_args!("{:?}", self.array_size))
            .finish()
    }
}
impl ::core::clone::Clone for D3D11_TEX2D_ARRAY_DSV {
    fn clone(&self) -> Self {
        Self {
            mip_slice: self.mip_slice,
            first_array_slice: self.first_array_slice,
            array_size: self.array_size,
        }
    }
}
impl ::std::marker::Copy for D3D11_TEX2D_ARRAY_DSV {}
#[repr(C)]
#[allow(non_camel_case_types)]
pub struct D3D11_TEX2DMS_DSV {
    pub unused_field_nothing_to_define: u32,
}
#[repr(C)]
#[doc(hidden)]
#[allow(non_camel_case_types)]
pub struct D3D11_TEX2DMS_DSV_abi(u32);
impl D3D11_TEX2DMS_DSV {}
unsafe impl ::windows::Abi for D3D11_TEX2DMS_DSV {
    type Abi = D3D11_TEX2DMS_DSV_abi;
}
impl ::core::default::Default for D3D11_TEX2DMS_DSV {
    fn default() -> Self {
        Self {
            unused_field_nothing_to_define: 0,
        }
    }
}
impl ::core::fmt::Debug for D3D11_TEX2DMS_DSV {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("D3D11_TEX2DMS_DSV")
            .field(
                "unused_field_nothing_to_define",
                &format_args!("{:?}", self.unused_field_nothing_to_define),
            )
            .finish()
    }
}
impl ::core::clone::Clone for D3D11_TEX2DMS_DSV {
    fn clone(&self) -> Self {
        Self {
            unused_field_nothing_to_define: self.unused_field_nothing_to_define,
        }
    }
}
impl ::std::marker::Copy for D3D11_TEX2DMS_DSV {}
#[repr(C)]
#[allow(non_camel_case_types)]
pub struct D3D11_TEX2DMS_ARRAY_DSV {
    pub first_array_slice: u32,
    pub array_size: u32,
}
#[repr(C)]
#[doc(hidden)]
#[allow(non_camel_case_types)]
pub struct D3D11_TEX2DMS_ARRAY_DSV_abi(u32, u32);
impl D3D11_TEX2DMS_ARRAY_DSV {}
unsafe impl ::windows::Abi for D3D11_TEX2DMS_ARRAY_DSV {
    type Abi = D3D11_TEX2DMS_ARRAY_DSV_abi;
}
impl ::core::default::Default for D3D11_TEX2DMS_ARRAY_DSV {
    fn default() -> Self {
        Self {
            first_array_slice: 0,
            array_size: 0,
        }
    }
}
impl ::core::fmt::Debug for D3D11_TEX2DMS_ARRAY_DSV {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("D3D11_TEX2DMS_ARRAY_DSV")
            .field(
                "first_array_slice",
                &format_args!("{:?}", self.first_array_slice),
            )
            .field("array_size", &format_args!("{:?}", self.array_size))
            .finish()
    }
}
impl ::core::clone::Clone for D3D11_TEX2DMS_ARRAY_DSV {
    fn clone(&self) -> Self {
        Self {
            first_array_slice: self.first_array_slice,
            array_size: self.array_size,
        }
    }
}
impl ::std::marker::Copy for D3D11_TEX2DMS_ARRAY_DSV {}
#[repr(C)]
#[allow(non_camel_case_types)]
pub struct D3D11_DEPTH_STENCIL_VIEW_DESC {
    pub format: super::dxgi::DXGI_FORMAT,
    pub view_dimension: D3D11_DSV_DIMENSION,
    pub flags: u32,
    pub anonymous: D3D11_DEPTH_STENCIL_VIEW_DESC_0,
}
#[repr(C)]
#[doc(hidden)]
#[allow(non_camel_case_types)]
pub struct D3D11_DEPTH_STENCIL_VIEW_DESC_abi(
    super::dxgi::DXGI_FORMAT,
    D3D11_DSV_DIMENSION,
    u32,
    D3D11_DEPTH_STENCIL_VIEW_DESC_0_abi,
);
impl D3D11_DEPTH_STENCIL_VIEW_DESC {}
unsafe impl ::windows::Abi for D3D11_DEPTH_STENCIL_VIEW_DESC {
    type Abi = D3D11_DEPTH_STENCIL_VIEW_DESC_abi;
}
impl ::core::fmt::Debug for D3D11_DEPTH_STENCIL_VIEW_DESC {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("D3D11_DEPTH_STENCIL_VIEW_DESC")
            .field("format", &format_args!("{:?}", self.format))
            .field("view_dimension", &format_args!("{:?}", self.view_dimension))
            .field("flags", &format_args!("{:?}", self.flags))
            .field("anonymous", &format_args!("{:?}", self.anonymous))
            .finish()
    }
}
impl ::core::default::Default for D3D11_DEPTH_STENCIL_VIEW_DESC {
    fn default() -> Self {
        Self {
            format: ::std::default::Default::default(),
            view_dimension: ::std::default::Default::default(),
            flags: 0,
            anonymous: ::std::default::Default::default(),
        }
    }
}
impl ::core::clone::Clone for D3D11_DEPTH_STENCIL_VIEW_DESC {
    fn clone(&self) -> Self {
        let mut out = Self::default();
        out.clone_from(self);
        out
    }
    fn clone_from(&mut self, source: &Self) {
        unsafe { std::ptr::copy(source, self, std::mem::size_of::<Self>()) }
    }
}
#[repr(C)]
#[allow(non_camel_case_types)]
pub union D3D11_DEPTH_STENCIL_VIEW_DESC_0 {
    pub texture1d: ::core::mem::ManuallyDrop<D3D11_TEX1D_DSV>,
    pub texture1d_array: ::core::mem::ManuallyDrop<D3D11_TEX1D_ARRAY_DSV>,
    pub texture2d: ::core::mem::ManuallyDrop<D3D11_TEX2D_DSV>,
    pub texture2d_array: ::core::mem::ManuallyDrop<D3D11_TEX2D_ARRAY_DSV>,
    pub texture2dms: ::core::mem::ManuallyDrop<D3D11_TEX2DMS_DSV>,
    pub texture2dms_array: ::core::mem::ManuallyDrop<D3D11_TEX2DMS_ARRAY_DSV>,
}
#[repr(C)]
#[doc(hidden)]
#[allow(non_camel_case_types)]
pub struct D3D11_DEPTH_STENCIL_VIEW_DESC_0_abi(
    D3D11_TEX1D_DSV_abi,
    D3D11_TEX1D_ARRAY_DSV_abi,
    D3D11_TEX2D_DSV_abi,
    D3D11_TEX2D_ARRAY_DSV_abi,
    D3D11_TEX2DMS_DSV_abi,
    D3D11_TEX2DMS_ARRAY_DSV_abi,
);
impl D3D11_DEPTH_STENCIL_VIEW_DESC_0 {}
unsafe impl ::windows::Abi for D3D11_DEPTH_STENCIL_VIEW_DESC_0 {
    type Abi = D3D11_DEPTH_STENCIL_VIEW_DESC_0_abi;
}
impl ::core::fmt::Debug for D3D11_DEPTH_STENCIL_VIEW_DESC_0 {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("D3D11_DEPTH_STENCIL_VIEW_DESC_0").finish()
    }
}
impl ::core::default::Default for D3D11_DEPTH_STENCIL_VIEW_DESC_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::clone::Clone for D3D11_DEPTH_STENCIL_VIEW_DESC_0 {
    fn clone(&self) -> Self {
        let mut out = Self::default();
        out.clone_from(self);
        out
    }
    fn clone_from(&mut self, source: &Self) {
        unsafe { std::ptr::copy(source, self, std::mem::size_of::<Self>()) }
    }
}
#[repr(transparent)]
#[allow(non_camel_case_types)]
pub struct ID3D11DepthStencilView(::windows::IUnknown);
impl ::std::clone::Clone for ID3D11DepthStencilView {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::std::fmt::Debug for ID3D11DepthStencilView {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        write!(f, "{:?}", self.0)
    }
}
impl ::std::cmp::PartialEq for ID3D11DepthStencilView {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::std::cmp::Eq for ID3D11DepthStencilView {}
unsafe impl ::windows::Interface for ID3D11DepthStencilView {
    type Vtable = ID3D11DepthStencilView_abi;
    const IID: ::windows::Guid = ::windows::Guid::from_values(
        2681915690,
        6262,
        18627,
        [175, 173, 37, 185, 79, 132, 169, 182],
    );
}
#[repr(C)]
pub struct ID3D11DepthStencilView_abi(
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        iid: &::windows::Guid,
        interface: *mut ::windows::RawPtr,
    ) -> ::windows::ErrorCode,
    pub unsafe extern "system" fn(this: ::windows::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::RawPtr) -> u32,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        pp_device: *mut ::std::option::Option<ID3D11Device>,
    ),
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
        pp_resource: *mut ::std::option::Option<ID3D11Resource>,
    ),
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        p_desc: *mut D3D11_DEPTH_STENCIL_VIEW_DESC,
    ),
);
#[allow(non_snake_case)]
impl ID3D11DepthStencilView {
    pub unsafe fn GetDevice(&self, pp_device: *mut ::std::option::Option<ID3D11Device>) {
        (::windows::Interface::vtable(self).3)(::windows::Abi::abi(self), pp_device)
    }
    pub unsafe fn GetPrivateData(
        &self,
        guid: *const ::windows::Guid,
        p_data_size: *mut u32,
        p_data: *mut ::std::ffi::c_void,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).4)(::windows::Abi::abi(self), guid, p_data_size, p_data)
    }
    pub unsafe fn SetPrivateData(
        &self,
        guid: *const ::windows::Guid,
        data_size: u32,
        p_data: *const ::std::ffi::c_void,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).5)(::windows::Abi::abi(self), guid, data_size, p_data)
    }
    pub unsafe fn SetPrivateDataInterface<
        'a,
        T1__: ::std::convert::Into<::windows::Param<'a, ::windows::IUnknown>>,
    >(
        &self,
        guid: *const ::windows::Guid,
        p_data: T1__,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).6)(::windows::Abi::abi(self), guid, p_data.into().abi())
    }
    pub unsafe fn GetResource(&self, pp_resource: *mut ::std::option::Option<ID3D11Resource>) {
        (::windows::Interface::vtable(self).7)(::windows::Abi::abi(self), pp_resource)
    }
    pub unsafe fn GetDesc(&self, p_desc: *mut D3D11_DEPTH_STENCIL_VIEW_DESC) {
        (::windows::Interface::vtable(self).8)(::windows::Abi::abi(self), p_desc)
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
impl<'a> ::std::convert::Into<::windows::Param<'a, ::windows::IUnknown>>
    for ID3D11DepthStencilView
{
    fn into(self) -> ::windows::Param<'a, ::windows::IUnknown> {
        ::windows::Param::Owned(::std::convert::Into::<::windows::IUnknown>::into(self))
    }
}
impl<'a> ::std::convert::Into<::windows::Param<'a, ::windows::IUnknown>>
    for &'a ID3D11DepthStencilView
{
    fn into(self) -> ::windows::Param<'a, ::windows::IUnknown> {
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
impl<'a> ::std::convert::Into<::windows::Param<'a, ID3D11View>> for ID3D11DepthStencilView {
    fn into(self) -> ::windows::Param<'a, ID3D11View> {
        ::windows::Param::Owned(::std::convert::Into::<ID3D11View>::into(self))
    }
}
impl<'a> ::std::convert::Into<::windows::Param<'a, ID3D11View>> for &'a ID3D11DepthStencilView {
    fn into(self) -> ::windows::Param<'a, ID3D11View> {
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
impl<'a> ::std::convert::Into<::windows::Param<'a, ID3D11DeviceChild>> for ID3D11DepthStencilView {
    fn into(self) -> ::windows::Param<'a, ID3D11DeviceChild> {
        ::windows::Param::Owned(::std::convert::Into::<ID3D11DeviceChild>::into(self))
    }
}
impl<'a> ::std::convert::Into<::windows::Param<'a, ID3D11DeviceChild>>
    for &'a ID3D11DepthStencilView
{
    fn into(self) -> ::windows::Param<'a, ID3D11DeviceChild> {
        ::windows::Param::Owned(::std::convert::Into::<ID3D11DeviceChild>::into(
            ::std::clone::Clone::clone(self),
        ))
    }
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct D3D11_INPUT_CLASSIFICATION(pub i32);
impl ::std::convert::From<i32> for D3D11_INPUT_CLASSIFICATION {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
impl ::std::clone::Clone for D3D11_INPUT_CLASSIFICATION {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}
impl ::std::default::Default for D3D11_INPUT_CLASSIFICATION {
    fn default() -> Self {
        Self(0)
    }
}
impl ::std::fmt::Debug for D3D11_INPUT_CLASSIFICATION {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        write!(f, "{:?}", self.0)
    }
}
impl ::std::cmp::PartialEq for D3D11_INPUT_CLASSIFICATION {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::std::cmp::Eq for D3D11_INPUT_CLASSIFICATION {}
impl ::std::marker::Copy for D3D11_INPUT_CLASSIFICATION {}
impl D3D11_INPUT_CLASSIFICATION {
    #![allow(non_upper_case_globals)]
    pub const D3D11_INPUT_PER_VERTEX_DATA: Self = Self(0i32);
    pub const D3D11_INPUT_PER_INSTANCE_DATA: Self = Self(1i32);
}
unsafe impl ::windows::Abi for D3D11_INPUT_CLASSIFICATION {
    type Abi = Self;
}
#[repr(C)]
#[allow(non_camel_case_types)]
pub struct D3D11_INPUT_ELEMENT_DESC {
    pub semantic_name: *mut i8,
    pub semantic_index: u32,
    pub format: super::dxgi::DXGI_FORMAT,
    pub input_slot: u32,
    pub aligned_byte_offset: u32,
    pub input_slot_class: D3D11_INPUT_CLASSIFICATION,
    pub instance_data_step_rate: u32,
}
#[repr(C)]
#[doc(hidden)]
#[allow(non_camel_case_types)]
pub struct D3D11_INPUT_ELEMENT_DESC_abi(
    *mut i8,
    u32,
    super::dxgi::DXGI_FORMAT,
    u32,
    u32,
    D3D11_INPUT_CLASSIFICATION,
    u32,
);
impl D3D11_INPUT_ELEMENT_DESC {}
unsafe impl ::windows::Abi for D3D11_INPUT_ELEMENT_DESC {
    type Abi = D3D11_INPUT_ELEMENT_DESC_abi;
}
impl ::core::default::Default for D3D11_INPUT_ELEMENT_DESC {
    fn default() -> Self {
        Self {
            semantic_name: ::std::ptr::null_mut(),
            semantic_index: 0,
            format: ::std::default::Default::default(),
            input_slot: 0,
            aligned_byte_offset: 0,
            input_slot_class: ::std::default::Default::default(),
            instance_data_step_rate: 0,
        }
    }
}
impl ::core::fmt::Debug for D3D11_INPUT_ELEMENT_DESC {
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
impl ::core::clone::Clone for D3D11_INPUT_ELEMENT_DESC {
    fn clone(&self) -> Self {
        Self {
            semantic_name: self.semantic_name,
            semantic_index: self.semantic_index,
            format: self.format,
            input_slot: self.input_slot,
            aligned_byte_offset: self.aligned_byte_offset,
            input_slot_class: self.input_slot_class,
            instance_data_step_rate: self.instance_data_step_rate,
        }
    }
}
#[repr(transparent)]
#[allow(non_camel_case_types)]
pub struct ID3D11InputLayout(::windows::IUnknown);
impl ::std::clone::Clone for ID3D11InputLayout {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::std::fmt::Debug for ID3D11InputLayout {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        write!(f, "{:?}", self.0)
    }
}
impl ::std::cmp::PartialEq for ID3D11InputLayout {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::std::cmp::Eq for ID3D11InputLayout {}
unsafe impl ::windows::Interface for ID3D11InputLayout {
    type Vtable = ID3D11InputLayout_abi;
    const IID: ::windows::Guid =
        ::windows::Guid::from_values(3833699804, 19696, 16421, [189, 38, 93, 232, 42, 62, 7, 183]);
}
#[repr(C)]
pub struct ID3D11InputLayout_abi(
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        iid: &::windows::Guid,
        interface: *mut ::windows::RawPtr,
    ) -> ::windows::ErrorCode,
    pub unsafe extern "system" fn(this: ::windows::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::RawPtr) -> u32,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        pp_device: *mut ::std::option::Option<ID3D11Device>,
    ),
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
#[allow(non_snake_case)]
impl ID3D11InputLayout {
    pub unsafe fn GetDevice(&self, pp_device: *mut ::std::option::Option<ID3D11Device>) {
        (::windows::Interface::vtable(self).3)(::windows::Abi::abi(self), pp_device)
    }
    pub unsafe fn GetPrivateData(
        &self,
        guid: *const ::windows::Guid,
        p_data_size: *mut u32,
        p_data: *mut ::std::ffi::c_void,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).4)(::windows::Abi::abi(self), guid, p_data_size, p_data)
    }
    pub unsafe fn SetPrivateData(
        &self,
        guid: *const ::windows::Guid,
        data_size: u32,
        p_data: *const ::std::ffi::c_void,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).5)(::windows::Abi::abi(self), guid, data_size, p_data)
    }
    pub unsafe fn SetPrivateDataInterface<
        'a,
        T1__: ::std::convert::Into<::windows::Param<'a, ::windows::IUnknown>>,
    >(
        &self,
        guid: *const ::windows::Guid,
        p_data: T1__,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).6)(::windows::Abi::abi(self), guid, p_data.into().abi())
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
impl<'a> ::std::convert::Into<::windows::Param<'a, ::windows::IUnknown>> for ID3D11InputLayout {
    fn into(self) -> ::windows::Param<'a, ::windows::IUnknown> {
        ::windows::Param::Owned(::std::convert::Into::<::windows::IUnknown>::into(self))
    }
}
impl<'a> ::std::convert::Into<::windows::Param<'a, ::windows::IUnknown>> for &'a ID3D11InputLayout {
    fn into(self) -> ::windows::Param<'a, ::windows::IUnknown> {
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
impl<'a> ::std::convert::Into<::windows::Param<'a, ID3D11DeviceChild>> for ID3D11InputLayout {
    fn into(self) -> ::windows::Param<'a, ID3D11DeviceChild> {
        ::windows::Param::Owned(::std::convert::Into::<ID3D11DeviceChild>::into(self))
    }
}
impl<'a> ::std::convert::Into<::windows::Param<'a, ID3D11DeviceChild>> for &'a ID3D11InputLayout {
    fn into(self) -> ::windows::Param<'a, ID3D11DeviceChild> {
        ::windows::Param::Owned(::std::convert::Into::<ID3D11DeviceChild>::into(
            ::std::clone::Clone::clone(self),
        ))
    }
}
#[repr(C)]
#[allow(non_camel_case_types)]
pub struct D3D11_CLASS_INSTANCE_DESC {
    pub instance_id: u32,
    pub instance_index: u32,
    pub type_id: u32,
    pub constant_buffer: u32,
    pub base_constant_buffer_offset: u32,
    pub base_texture: u32,
    pub base_sampler: u32,
    pub created: ::windows::BOOL,
}
#[repr(C)]
#[doc(hidden)]
#[allow(non_camel_case_types)]
pub struct D3D11_CLASS_INSTANCE_DESC_abi(u32, u32, u32, u32, u32, u32, u32, ::windows::BOOL);
impl D3D11_CLASS_INSTANCE_DESC {}
unsafe impl ::windows::Abi for D3D11_CLASS_INSTANCE_DESC {
    type Abi = D3D11_CLASS_INSTANCE_DESC_abi;
}
impl ::core::default::Default for D3D11_CLASS_INSTANCE_DESC {
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
impl ::core::fmt::Debug for D3D11_CLASS_INSTANCE_DESC {
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
impl ::core::clone::Clone for D3D11_CLASS_INSTANCE_DESC {
    fn clone(&self) -> Self {
        Self {
            instance_id: self.instance_id,
            instance_index: self.instance_index,
            type_id: self.type_id,
            constant_buffer: self.constant_buffer,
            base_constant_buffer_offset: self.base_constant_buffer_offset,
            base_texture: self.base_texture,
            base_sampler: self.base_sampler,
            created: <::windows::BOOL as std::clone::Clone>::clone(&self.created),
        }
    }
}
#[repr(transparent)]
#[allow(non_camel_case_types)]
pub struct ID3D11ClassInstance(::windows::IUnknown);
impl ::std::clone::Clone for ID3D11ClassInstance {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::std::fmt::Debug for ID3D11ClassInstance {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        write!(f, "{:?}", self.0)
    }
}
impl ::std::cmp::PartialEq for ID3D11ClassInstance {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::std::cmp::Eq for ID3D11ClassInstance {}
unsafe impl ::windows::Interface for ID3D11ClassInstance {
    type Vtable = ID3D11ClassInstance_abi;
    const IID: ::windows::Guid = ::windows::Guid::from_values(
        2798485418,
        45239,
        18991,
        [148, 54, 134, 98, 166, 87, 151, 203],
    );
}
#[repr(C)]
pub struct ID3D11ClassInstance_abi(
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        iid: &::windows::Guid,
        interface: *mut ::windows::RawPtr,
    ) -> ::windows::ErrorCode,
    pub unsafe extern "system" fn(this: ::windows::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::RawPtr) -> u32,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        pp_device: *mut ::std::option::Option<ID3D11Device>,
    ),
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
        pp_linkage: *mut ::std::option::Option<ID3D11ClassLinkage>,
    ),
    pub unsafe extern "system" fn(this: ::windows::RawPtr, p_desc: *mut D3D11_CLASS_INSTANCE_DESC),
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        p_instance_name: *mut i8,
        p_buffer_length: *mut usize,
    ),
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        p_type_name: *mut i8,
        p_buffer_length: *mut usize,
    ),
);
#[allow(non_snake_case)]
impl ID3D11ClassInstance {
    pub unsafe fn GetDevice(&self, pp_device: *mut ::std::option::Option<ID3D11Device>) {
        (::windows::Interface::vtable(self).3)(::windows::Abi::abi(self), pp_device)
    }
    pub unsafe fn GetPrivateData(
        &self,
        guid: *const ::windows::Guid,
        p_data_size: *mut u32,
        p_data: *mut ::std::ffi::c_void,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).4)(::windows::Abi::abi(self), guid, p_data_size, p_data)
    }
    pub unsafe fn SetPrivateData(
        &self,
        guid: *const ::windows::Guid,
        data_size: u32,
        p_data: *const ::std::ffi::c_void,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).5)(::windows::Abi::abi(self), guid, data_size, p_data)
    }
    pub unsafe fn SetPrivateDataInterface<
        'a,
        T1__: ::std::convert::Into<::windows::Param<'a, ::windows::IUnknown>>,
    >(
        &self,
        guid: *const ::windows::Guid,
        p_data: T1__,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).6)(::windows::Abi::abi(self), guid, p_data.into().abi())
    }
    pub unsafe fn GetClassLinkage(
        &self,
        pp_linkage: *mut ::std::option::Option<ID3D11ClassLinkage>,
    ) {
        (::windows::Interface::vtable(self).7)(::windows::Abi::abi(self), pp_linkage)
    }
    pub unsafe fn GetDesc(&self, p_desc: *mut D3D11_CLASS_INSTANCE_DESC) {
        (::windows::Interface::vtable(self).8)(::windows::Abi::abi(self), p_desc)
    }
    pub unsafe fn GetInstanceName(&self, p_instance_name: *mut i8, p_buffer_length: *mut usize) {
        (::windows::Interface::vtable(self).9)(
            ::windows::Abi::abi(self),
            p_instance_name,
            p_buffer_length,
        )
    }
    pub unsafe fn GetTypeName(&self, p_type_name: *mut i8, p_buffer_length: *mut usize) {
        (::windows::Interface::vtable(self).10)(
            ::windows::Abi::abi(self),
            p_type_name,
            p_buffer_length,
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
impl<'a> ::std::convert::Into<::windows::Param<'a, ::windows::IUnknown>> for ID3D11ClassInstance {
    fn into(self) -> ::windows::Param<'a, ::windows::IUnknown> {
        ::windows::Param::Owned(::std::convert::Into::<::windows::IUnknown>::into(self))
    }
}
impl<'a> ::std::convert::Into<::windows::Param<'a, ::windows::IUnknown>>
    for &'a ID3D11ClassInstance
{
    fn into(self) -> ::windows::Param<'a, ::windows::IUnknown> {
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
impl<'a> ::std::convert::Into<::windows::Param<'a, ID3D11DeviceChild>> for ID3D11ClassInstance {
    fn into(self) -> ::windows::Param<'a, ID3D11DeviceChild> {
        ::windows::Param::Owned(::std::convert::Into::<ID3D11DeviceChild>::into(self))
    }
}
impl<'a> ::std::convert::Into<::windows::Param<'a, ID3D11DeviceChild>> for &'a ID3D11ClassInstance {
    fn into(self) -> ::windows::Param<'a, ID3D11DeviceChild> {
        ::windows::Param::Owned(::std::convert::Into::<ID3D11DeviceChild>::into(
            ::std::clone::Clone::clone(self),
        ))
    }
}
#[repr(transparent)]
#[allow(non_camel_case_types)]
pub struct ID3D11ClassLinkage(::windows::IUnknown);
impl ::std::clone::Clone for ID3D11ClassLinkage {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::std::fmt::Debug for ID3D11ClassLinkage {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        write!(f, "{:?}", self.0)
    }
}
impl ::std::cmp::PartialEq for ID3D11ClassLinkage {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::std::cmp::Eq for ID3D11ClassLinkage {}
unsafe impl ::windows::Interface for ID3D11ClassLinkage {
    type Vtable = ID3D11ClassLinkage_abi;
    const IID: ::windows::Guid = ::windows::Guid::from_values(
        3723852986,
        38211,
        18148,
        [161, 43, 242, 7, 160, 254, 127, 237],
    );
}
#[repr(C)]
pub struct ID3D11ClassLinkage_abi(
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        iid: &::windows::Guid,
        interface: *mut ::windows::RawPtr,
    ) -> ::windows::ErrorCode,
    pub unsafe extern "system" fn(this: ::windows::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::RawPtr) -> u32,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        pp_device: *mut ::std::option::Option<ID3D11Device>,
    ),
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
        p_class_instance_name: *const i8,
        instance_index: u32,
        pp_instance: *mut ::std::option::Option<ID3D11ClassInstance>,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        p_class_type_name: *const i8,
        constant_buffer_offset: u32,
        constant_vector_offset: u32,
        texture_offset: u32,
        sampler_offset: u32,
        pp_instance: *mut ::std::option::Option<ID3D11ClassInstance>,
    ) -> ::windows::ErrorCode,
);
#[allow(non_snake_case)]
impl ID3D11ClassLinkage {
    pub unsafe fn GetDevice(&self, pp_device: *mut ::std::option::Option<ID3D11Device>) {
        (::windows::Interface::vtable(self).3)(::windows::Abi::abi(self), pp_device)
    }
    pub unsafe fn GetPrivateData(
        &self,
        guid: *const ::windows::Guid,
        p_data_size: *mut u32,
        p_data: *mut ::std::ffi::c_void,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).4)(::windows::Abi::abi(self), guid, p_data_size, p_data)
    }
    pub unsafe fn SetPrivateData(
        &self,
        guid: *const ::windows::Guid,
        data_size: u32,
        p_data: *const ::std::ffi::c_void,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).5)(::windows::Abi::abi(self), guid, data_size, p_data)
    }
    pub unsafe fn SetPrivateDataInterface<
        'a,
        T1__: ::std::convert::Into<::windows::Param<'a, ::windows::IUnknown>>,
    >(
        &self,
        guid: *const ::windows::Guid,
        p_data: T1__,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).6)(::windows::Abi::abi(self), guid, p_data.into().abi())
    }
    pub unsafe fn GetClassInstance(
        &self,
        p_class_instance_name: *const i8,
        instance_index: u32,
        pp_instance: *mut ::std::option::Option<ID3D11ClassInstance>,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).7)(
            ::windows::Abi::abi(self),
            p_class_instance_name,
            instance_index,
            pp_instance,
        )
    }
    pub unsafe fn CreateClassInstance(
        &self,
        p_class_type_name: *const i8,
        constant_buffer_offset: u32,
        constant_vector_offset: u32,
        texture_offset: u32,
        sampler_offset: u32,
        pp_instance: *mut ::std::option::Option<ID3D11ClassInstance>,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).8)(
            ::windows::Abi::abi(self),
            p_class_type_name,
            constant_buffer_offset,
            constant_vector_offset,
            texture_offset,
            sampler_offset,
            pp_instance,
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
impl<'a> ::std::convert::Into<::windows::Param<'a, ::windows::IUnknown>> for ID3D11ClassLinkage {
    fn into(self) -> ::windows::Param<'a, ::windows::IUnknown> {
        ::windows::Param::Owned(::std::convert::Into::<::windows::IUnknown>::into(self))
    }
}
impl<'a> ::std::convert::Into<::windows::Param<'a, ::windows::IUnknown>>
    for &'a ID3D11ClassLinkage
{
    fn into(self) -> ::windows::Param<'a, ::windows::IUnknown> {
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
impl<'a> ::std::convert::Into<::windows::Param<'a, ID3D11DeviceChild>> for ID3D11ClassLinkage {
    fn into(self) -> ::windows::Param<'a, ID3D11DeviceChild> {
        ::windows::Param::Owned(::std::convert::Into::<ID3D11DeviceChild>::into(self))
    }
}
impl<'a> ::std::convert::Into<::windows::Param<'a, ID3D11DeviceChild>> for &'a ID3D11ClassLinkage {
    fn into(self) -> ::windows::Param<'a, ID3D11DeviceChild> {
        ::windows::Param::Owned(::std::convert::Into::<ID3D11DeviceChild>::into(
            ::std::clone::Clone::clone(self),
        ))
    }
}
#[repr(transparent)]
#[allow(non_camel_case_types)]
pub struct ID3D11VertexShader(::windows::IUnknown);
impl ::std::clone::Clone for ID3D11VertexShader {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::std::fmt::Debug for ID3D11VertexShader {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        write!(f, "{:?}", self.0)
    }
}
impl ::std::cmp::PartialEq for ID3D11VertexShader {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::std::cmp::Eq for ID3D11VertexShader {}
unsafe impl ::windows::Interface for ID3D11VertexShader {
    type Vtable = ID3D11VertexShader_abi;
    const IID: ::windows::Guid = ::windows::Guid::from_values(
        993008996,
        54904,
        17033,
        [136, 151, 34, 248, 146, 139, 114, 243],
    );
}
#[repr(C)]
pub struct ID3D11VertexShader_abi(
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        iid: &::windows::Guid,
        interface: *mut ::windows::RawPtr,
    ) -> ::windows::ErrorCode,
    pub unsafe extern "system" fn(this: ::windows::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::RawPtr) -> u32,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        pp_device: *mut ::std::option::Option<ID3D11Device>,
    ),
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
#[allow(non_snake_case)]
impl ID3D11VertexShader {
    pub unsafe fn GetDevice(&self, pp_device: *mut ::std::option::Option<ID3D11Device>) {
        (::windows::Interface::vtable(self).3)(::windows::Abi::abi(self), pp_device)
    }
    pub unsafe fn GetPrivateData(
        &self,
        guid: *const ::windows::Guid,
        p_data_size: *mut u32,
        p_data: *mut ::std::ffi::c_void,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).4)(::windows::Abi::abi(self), guid, p_data_size, p_data)
    }
    pub unsafe fn SetPrivateData(
        &self,
        guid: *const ::windows::Guid,
        data_size: u32,
        p_data: *const ::std::ffi::c_void,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).5)(::windows::Abi::abi(self), guid, data_size, p_data)
    }
    pub unsafe fn SetPrivateDataInterface<
        'a,
        T1__: ::std::convert::Into<::windows::Param<'a, ::windows::IUnknown>>,
    >(
        &self,
        guid: *const ::windows::Guid,
        p_data: T1__,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).6)(::windows::Abi::abi(self), guid, p_data.into().abi())
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
impl<'a> ::std::convert::Into<::windows::Param<'a, ::windows::IUnknown>> for ID3D11VertexShader {
    fn into(self) -> ::windows::Param<'a, ::windows::IUnknown> {
        ::windows::Param::Owned(::std::convert::Into::<::windows::IUnknown>::into(self))
    }
}
impl<'a> ::std::convert::Into<::windows::Param<'a, ::windows::IUnknown>>
    for &'a ID3D11VertexShader
{
    fn into(self) -> ::windows::Param<'a, ::windows::IUnknown> {
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
impl<'a> ::std::convert::Into<::windows::Param<'a, ID3D11DeviceChild>> for ID3D11VertexShader {
    fn into(self) -> ::windows::Param<'a, ID3D11DeviceChild> {
        ::windows::Param::Owned(::std::convert::Into::<ID3D11DeviceChild>::into(self))
    }
}
impl<'a> ::std::convert::Into<::windows::Param<'a, ID3D11DeviceChild>> for &'a ID3D11VertexShader {
    fn into(self) -> ::windows::Param<'a, ID3D11DeviceChild> {
        ::windows::Param::Owned(::std::convert::Into::<ID3D11DeviceChild>::into(
            ::std::clone::Clone::clone(self),
        ))
    }
}
#[repr(transparent)]
#[allow(non_camel_case_types)]
pub struct ID3D11GeometryShader(::windows::IUnknown);
impl ::std::clone::Clone for ID3D11GeometryShader {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::std::fmt::Debug for ID3D11GeometryShader {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        write!(f, "{:?}", self.0)
    }
}
impl ::std::cmp::PartialEq for ID3D11GeometryShader {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::std::cmp::Eq for ID3D11GeometryShader {}
unsafe impl ::windows::Interface for ID3D11GeometryShader {
    type Vtable = ID3D11GeometryShader_abi;
    const IID: ::windows::Guid =
        ::windows::Guid::from_values(942824342, 61435, 16418, [186, 2, 46, 121, 91, 112, 39, 92]);
}
#[repr(C)]
pub struct ID3D11GeometryShader_abi(
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        iid: &::windows::Guid,
        interface: *mut ::windows::RawPtr,
    ) -> ::windows::ErrorCode,
    pub unsafe extern "system" fn(this: ::windows::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::RawPtr) -> u32,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        pp_device: *mut ::std::option::Option<ID3D11Device>,
    ),
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
#[allow(non_snake_case)]
impl ID3D11GeometryShader {
    pub unsafe fn GetDevice(&self, pp_device: *mut ::std::option::Option<ID3D11Device>) {
        (::windows::Interface::vtable(self).3)(::windows::Abi::abi(self), pp_device)
    }
    pub unsafe fn GetPrivateData(
        &self,
        guid: *const ::windows::Guid,
        p_data_size: *mut u32,
        p_data: *mut ::std::ffi::c_void,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).4)(::windows::Abi::abi(self), guid, p_data_size, p_data)
    }
    pub unsafe fn SetPrivateData(
        &self,
        guid: *const ::windows::Guid,
        data_size: u32,
        p_data: *const ::std::ffi::c_void,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).5)(::windows::Abi::abi(self), guid, data_size, p_data)
    }
    pub unsafe fn SetPrivateDataInterface<
        'a,
        T1__: ::std::convert::Into<::windows::Param<'a, ::windows::IUnknown>>,
    >(
        &self,
        guid: *const ::windows::Guid,
        p_data: T1__,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).6)(::windows::Abi::abi(self), guid, p_data.into().abi())
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
impl<'a> ::std::convert::Into<::windows::Param<'a, ::windows::IUnknown>> for ID3D11GeometryShader {
    fn into(self) -> ::windows::Param<'a, ::windows::IUnknown> {
        ::windows::Param::Owned(::std::convert::Into::<::windows::IUnknown>::into(self))
    }
}
impl<'a> ::std::convert::Into<::windows::Param<'a, ::windows::IUnknown>>
    for &'a ID3D11GeometryShader
{
    fn into(self) -> ::windows::Param<'a, ::windows::IUnknown> {
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
impl<'a> ::std::convert::Into<::windows::Param<'a, ID3D11DeviceChild>> for ID3D11GeometryShader {
    fn into(self) -> ::windows::Param<'a, ID3D11DeviceChild> {
        ::windows::Param::Owned(::std::convert::Into::<ID3D11DeviceChild>::into(self))
    }
}
impl<'a> ::std::convert::Into<::windows::Param<'a, ID3D11DeviceChild>>
    for &'a ID3D11GeometryShader
{
    fn into(self) -> ::windows::Param<'a, ID3D11DeviceChild> {
        ::windows::Param::Owned(::std::convert::Into::<ID3D11DeviceChild>::into(
            ::std::clone::Clone::clone(self),
        ))
    }
}
#[repr(C)]
#[allow(non_camel_case_types)]
pub struct D3D11_SO_DECLARATION_ENTRY {
    pub stream: u32,
    pub semantic_name: *mut i8,
    pub semantic_index: u32,
    pub start_component: u8,
    pub component_count: u8,
    pub output_slot: u8,
}
#[repr(C)]
#[doc(hidden)]
#[allow(non_camel_case_types)]
pub struct D3D11_SO_DECLARATION_ENTRY_abi(u32, *mut i8, u32, u8, u8, u8);
impl D3D11_SO_DECLARATION_ENTRY {}
unsafe impl ::windows::Abi for D3D11_SO_DECLARATION_ENTRY {
    type Abi = D3D11_SO_DECLARATION_ENTRY_abi;
}
impl ::core::default::Default for D3D11_SO_DECLARATION_ENTRY {
    fn default() -> Self {
        Self {
            stream: 0,
            semantic_name: ::std::ptr::null_mut(),
            semantic_index: 0,
            start_component: 0,
            component_count: 0,
            output_slot: 0,
        }
    }
}
impl ::core::fmt::Debug for D3D11_SO_DECLARATION_ENTRY {
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
impl ::core::clone::Clone for D3D11_SO_DECLARATION_ENTRY {
    fn clone(&self) -> Self {
        Self {
            stream: self.stream,
            semantic_name: self.semantic_name,
            semantic_index: self.semantic_index,
            start_component: self.start_component,
            component_count: self.component_count,
            output_slot: self.output_slot,
        }
    }
}
impl ::std::marker::Copy for D3D11_SO_DECLARATION_ENTRY {}
#[repr(transparent)]
#[allow(non_camel_case_types)]
pub struct ID3D11PixelShader(::windows::IUnknown);
impl ::std::clone::Clone for ID3D11PixelShader {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::std::fmt::Debug for ID3D11PixelShader {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        write!(f, "{:?}", self.0)
    }
}
impl ::std::cmp::PartialEq for ID3D11PixelShader {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::std::cmp::Eq for ID3D11PixelShader {}
unsafe impl ::windows::Interface for ID3D11PixelShader {
    type Vtable = ID3D11PixelShader_abi;
    const IID: ::windows::Guid = ::windows::Guid::from_values(
        3934446605,
        20956,
        20275,
        [147, 212, 219, 124, 145, 37, 174, 140],
    );
}
#[repr(C)]
pub struct ID3D11PixelShader_abi(
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        iid: &::windows::Guid,
        interface: *mut ::windows::RawPtr,
    ) -> ::windows::ErrorCode,
    pub unsafe extern "system" fn(this: ::windows::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::RawPtr) -> u32,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        pp_device: *mut ::std::option::Option<ID3D11Device>,
    ),
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
#[allow(non_snake_case)]
impl ID3D11PixelShader {
    pub unsafe fn GetDevice(&self, pp_device: *mut ::std::option::Option<ID3D11Device>) {
        (::windows::Interface::vtable(self).3)(::windows::Abi::abi(self), pp_device)
    }
    pub unsafe fn GetPrivateData(
        &self,
        guid: *const ::windows::Guid,
        p_data_size: *mut u32,
        p_data: *mut ::std::ffi::c_void,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).4)(::windows::Abi::abi(self), guid, p_data_size, p_data)
    }
    pub unsafe fn SetPrivateData(
        &self,
        guid: *const ::windows::Guid,
        data_size: u32,
        p_data: *const ::std::ffi::c_void,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).5)(::windows::Abi::abi(self), guid, data_size, p_data)
    }
    pub unsafe fn SetPrivateDataInterface<
        'a,
        T1__: ::std::convert::Into<::windows::Param<'a, ::windows::IUnknown>>,
    >(
        &self,
        guid: *const ::windows::Guid,
        p_data: T1__,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).6)(::windows::Abi::abi(self), guid, p_data.into().abi())
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
impl<'a> ::std::convert::Into<::windows::Param<'a, ::windows::IUnknown>> for ID3D11PixelShader {
    fn into(self) -> ::windows::Param<'a, ::windows::IUnknown> {
        ::windows::Param::Owned(::std::convert::Into::<::windows::IUnknown>::into(self))
    }
}
impl<'a> ::std::convert::Into<::windows::Param<'a, ::windows::IUnknown>> for &'a ID3D11PixelShader {
    fn into(self) -> ::windows::Param<'a, ::windows::IUnknown> {
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
impl<'a> ::std::convert::Into<::windows::Param<'a, ID3D11DeviceChild>> for ID3D11PixelShader {
    fn into(self) -> ::windows::Param<'a, ID3D11DeviceChild> {
        ::windows::Param::Owned(::std::convert::Into::<ID3D11DeviceChild>::into(self))
    }
}
impl<'a> ::std::convert::Into<::windows::Param<'a, ID3D11DeviceChild>> for &'a ID3D11PixelShader {
    fn into(self) -> ::windows::Param<'a, ID3D11DeviceChild> {
        ::windows::Param::Owned(::std::convert::Into::<ID3D11DeviceChild>::into(
            ::std::clone::Clone::clone(self),
        ))
    }
}
#[repr(transparent)]
#[allow(non_camel_case_types)]
pub struct ID3D11HullShader(::windows::IUnknown);
impl ::std::clone::Clone for ID3D11HullShader {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::std::fmt::Debug for ID3D11HullShader {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        write!(f, "{:?}", self.0)
    }
}
impl ::std::cmp::PartialEq for ID3D11HullShader {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::std::cmp::Eq for ID3D11HullShader {}
unsafe impl ::windows::Interface for ID3D11HullShader {
    type Vtable = ID3D11HullShader_abi;
    const IID: ::windows::Guid = ::windows::Guid::from_values(
        2388418657,
        25226,
        19598,
        [130, 100, 187, 228, 92, 179, 213, 221],
    );
}
#[repr(C)]
pub struct ID3D11HullShader_abi(
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        iid: &::windows::Guid,
        interface: *mut ::windows::RawPtr,
    ) -> ::windows::ErrorCode,
    pub unsafe extern "system" fn(this: ::windows::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::RawPtr) -> u32,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        pp_device: *mut ::std::option::Option<ID3D11Device>,
    ),
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
#[allow(non_snake_case)]
impl ID3D11HullShader {
    pub unsafe fn GetDevice(&self, pp_device: *mut ::std::option::Option<ID3D11Device>) {
        (::windows::Interface::vtable(self).3)(::windows::Abi::abi(self), pp_device)
    }
    pub unsafe fn GetPrivateData(
        &self,
        guid: *const ::windows::Guid,
        p_data_size: *mut u32,
        p_data: *mut ::std::ffi::c_void,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).4)(::windows::Abi::abi(self), guid, p_data_size, p_data)
    }
    pub unsafe fn SetPrivateData(
        &self,
        guid: *const ::windows::Guid,
        data_size: u32,
        p_data: *const ::std::ffi::c_void,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).5)(::windows::Abi::abi(self), guid, data_size, p_data)
    }
    pub unsafe fn SetPrivateDataInterface<
        'a,
        T1__: ::std::convert::Into<::windows::Param<'a, ::windows::IUnknown>>,
    >(
        &self,
        guid: *const ::windows::Guid,
        p_data: T1__,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).6)(::windows::Abi::abi(self), guid, p_data.into().abi())
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
impl<'a> ::std::convert::Into<::windows::Param<'a, ::windows::IUnknown>> for ID3D11HullShader {
    fn into(self) -> ::windows::Param<'a, ::windows::IUnknown> {
        ::windows::Param::Owned(::std::convert::Into::<::windows::IUnknown>::into(self))
    }
}
impl<'a> ::std::convert::Into<::windows::Param<'a, ::windows::IUnknown>> for &'a ID3D11HullShader {
    fn into(self) -> ::windows::Param<'a, ::windows::IUnknown> {
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
impl<'a> ::std::convert::Into<::windows::Param<'a, ID3D11DeviceChild>> for ID3D11HullShader {
    fn into(self) -> ::windows::Param<'a, ID3D11DeviceChild> {
        ::windows::Param::Owned(::std::convert::Into::<ID3D11DeviceChild>::into(self))
    }
}
impl<'a> ::std::convert::Into<::windows::Param<'a, ID3D11DeviceChild>> for &'a ID3D11HullShader {
    fn into(self) -> ::windows::Param<'a, ID3D11DeviceChild> {
        ::windows::Param::Owned(::std::convert::Into::<ID3D11DeviceChild>::into(
            ::std::clone::Clone::clone(self),
        ))
    }
}
#[repr(transparent)]
#[allow(non_camel_case_types)]
pub struct ID3D11DomainShader(::windows::IUnknown);
impl ::std::clone::Clone for ID3D11DomainShader {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::std::fmt::Debug for ID3D11DomainShader {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        write!(f, "{:?}", self.0)
    }
}
impl ::std::cmp::PartialEq for ID3D11DomainShader {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::std::cmp::Eq for ID3D11DomainShader {}
unsafe impl ::windows::Interface for ID3D11DomainShader {
    type Vtable = ID3D11DomainShader_abi;
    const IID: ::windows::Guid = ::windows::Guid::from_values(
        4118988040,
        3894,
        18700,
        [153, 119, 49, 238, 206, 38, 140, 250],
    );
}
#[repr(C)]
pub struct ID3D11DomainShader_abi(
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        iid: &::windows::Guid,
        interface: *mut ::windows::RawPtr,
    ) -> ::windows::ErrorCode,
    pub unsafe extern "system" fn(this: ::windows::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::RawPtr) -> u32,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        pp_device: *mut ::std::option::Option<ID3D11Device>,
    ),
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
#[allow(non_snake_case)]
impl ID3D11DomainShader {
    pub unsafe fn GetDevice(&self, pp_device: *mut ::std::option::Option<ID3D11Device>) {
        (::windows::Interface::vtable(self).3)(::windows::Abi::abi(self), pp_device)
    }
    pub unsafe fn GetPrivateData(
        &self,
        guid: *const ::windows::Guid,
        p_data_size: *mut u32,
        p_data: *mut ::std::ffi::c_void,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).4)(::windows::Abi::abi(self), guid, p_data_size, p_data)
    }
    pub unsafe fn SetPrivateData(
        &self,
        guid: *const ::windows::Guid,
        data_size: u32,
        p_data: *const ::std::ffi::c_void,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).5)(::windows::Abi::abi(self), guid, data_size, p_data)
    }
    pub unsafe fn SetPrivateDataInterface<
        'a,
        T1__: ::std::convert::Into<::windows::Param<'a, ::windows::IUnknown>>,
    >(
        &self,
        guid: *const ::windows::Guid,
        p_data: T1__,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).6)(::windows::Abi::abi(self), guid, p_data.into().abi())
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
impl<'a> ::std::convert::Into<::windows::Param<'a, ::windows::IUnknown>> for ID3D11DomainShader {
    fn into(self) -> ::windows::Param<'a, ::windows::IUnknown> {
        ::windows::Param::Owned(::std::convert::Into::<::windows::IUnknown>::into(self))
    }
}
impl<'a> ::std::convert::Into<::windows::Param<'a, ::windows::IUnknown>>
    for &'a ID3D11DomainShader
{
    fn into(self) -> ::windows::Param<'a, ::windows::IUnknown> {
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
impl<'a> ::std::convert::Into<::windows::Param<'a, ID3D11DeviceChild>> for ID3D11DomainShader {
    fn into(self) -> ::windows::Param<'a, ID3D11DeviceChild> {
        ::windows::Param::Owned(::std::convert::Into::<ID3D11DeviceChild>::into(self))
    }
}
impl<'a> ::std::convert::Into<::windows::Param<'a, ID3D11DeviceChild>> for &'a ID3D11DomainShader {
    fn into(self) -> ::windows::Param<'a, ID3D11DeviceChild> {
        ::windows::Param::Owned(::std::convert::Into::<ID3D11DeviceChild>::into(
            ::std::clone::Clone::clone(self),
        ))
    }
}
#[repr(transparent)]
#[allow(non_camel_case_types)]
pub struct ID3D11ComputeShader(::windows::IUnknown);
impl ::std::clone::Clone for ID3D11ComputeShader {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::std::fmt::Debug for ID3D11ComputeShader {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        write!(f, "{:?}", self.0)
    }
}
impl ::std::cmp::PartialEq for ID3D11ComputeShader {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::std::cmp::Eq for ID3D11ComputeShader {}
unsafe impl ::windows::Interface for ID3D11ComputeShader {
    type Vtable = ID3D11ComputeShader_abi;
    const IID: ::windows::Guid = ::windows::Guid::from_values(
        1331370350,
        49853,
        18782,
        [189, 1, 31, 222, 211, 142, 73, 105],
    );
}
#[repr(C)]
pub struct ID3D11ComputeShader_abi(
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        iid: &::windows::Guid,
        interface: *mut ::windows::RawPtr,
    ) -> ::windows::ErrorCode,
    pub unsafe extern "system" fn(this: ::windows::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::RawPtr) -> u32,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        pp_device: *mut ::std::option::Option<ID3D11Device>,
    ),
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
#[allow(non_snake_case)]
impl ID3D11ComputeShader {
    pub unsafe fn GetDevice(&self, pp_device: *mut ::std::option::Option<ID3D11Device>) {
        (::windows::Interface::vtable(self).3)(::windows::Abi::abi(self), pp_device)
    }
    pub unsafe fn GetPrivateData(
        &self,
        guid: *const ::windows::Guid,
        p_data_size: *mut u32,
        p_data: *mut ::std::ffi::c_void,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).4)(::windows::Abi::abi(self), guid, p_data_size, p_data)
    }
    pub unsafe fn SetPrivateData(
        &self,
        guid: *const ::windows::Guid,
        data_size: u32,
        p_data: *const ::std::ffi::c_void,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).5)(::windows::Abi::abi(self), guid, data_size, p_data)
    }
    pub unsafe fn SetPrivateDataInterface<
        'a,
        T1__: ::std::convert::Into<::windows::Param<'a, ::windows::IUnknown>>,
    >(
        &self,
        guid: *const ::windows::Guid,
        p_data: T1__,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).6)(::windows::Abi::abi(self), guid, p_data.into().abi())
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
impl<'a> ::std::convert::Into<::windows::Param<'a, ::windows::IUnknown>> for ID3D11ComputeShader {
    fn into(self) -> ::windows::Param<'a, ::windows::IUnknown> {
        ::windows::Param::Owned(::std::convert::Into::<::windows::IUnknown>::into(self))
    }
}
impl<'a> ::std::convert::Into<::windows::Param<'a, ::windows::IUnknown>>
    for &'a ID3D11ComputeShader
{
    fn into(self) -> ::windows::Param<'a, ::windows::IUnknown> {
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
impl<'a> ::std::convert::Into<::windows::Param<'a, ID3D11DeviceChild>> for ID3D11ComputeShader {
    fn into(self) -> ::windows::Param<'a, ID3D11DeviceChild> {
        ::windows::Param::Owned(::std::convert::Into::<ID3D11DeviceChild>::into(self))
    }
}
impl<'a> ::std::convert::Into<::windows::Param<'a, ID3D11DeviceChild>> for &'a ID3D11ComputeShader {
    fn into(self) -> ::windows::Param<'a, ID3D11DeviceChild> {
        ::windows::Param::Owned(::std::convert::Into::<ID3D11DeviceChild>::into(
            ::std::clone::Clone::clone(self),
        ))
    }
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct D3D11_BLEND(pub i32);
impl ::std::convert::From<i32> for D3D11_BLEND {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
impl ::std::clone::Clone for D3D11_BLEND {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}
impl ::std::default::Default for D3D11_BLEND {
    fn default() -> Self {
        Self(0)
    }
}
impl ::std::fmt::Debug for D3D11_BLEND {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        write!(f, "{:?}", self.0)
    }
}
impl ::std::cmp::PartialEq for D3D11_BLEND {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::std::cmp::Eq for D3D11_BLEND {}
impl ::std::marker::Copy for D3D11_BLEND {}
impl D3D11_BLEND {
    #![allow(non_upper_case_globals)]
    pub const D3D11_BLEND_ZERO: Self = Self(1i32);
    pub const D3D11_BLEND_ONE: Self = Self(2i32);
    pub const D3D11_BLEND_SRC_COLOR: Self = Self(3i32);
    pub const D3D11_BLEND_INV_SRC_COLOR: Self = Self(4i32);
    pub const D3D11_BLEND_SRC_ALPHA: Self = Self(5i32);
    pub const D3D11_BLEND_INV_SRC_ALPHA: Self = Self(6i32);
    pub const D3D11_BLEND_DEST_ALPHA: Self = Self(7i32);
    pub const D3D11_BLEND_INV_DEST_ALPHA: Self = Self(8i32);
    pub const D3D11_BLEND_DEST_COLOR: Self = Self(9i32);
    pub const D3D11_BLEND_INV_DEST_COLOR: Self = Self(10i32);
    pub const D3D11_BLEND_SRC_ALPHA_SAT: Self = Self(11i32);
    pub const D3D11_BLEND_BLEND_FACTOR: Self = Self(14i32);
    pub const D3D11_BLEND_INV_BLEND_FACTOR: Self = Self(15i32);
    pub const D3D11_BLEND_SRC1_COLOR: Self = Self(16i32);
    pub const D3D11_BLEND_INV_SRC1_COLOR: Self = Self(17i32);
    pub const D3D11_BLEND_SRC1_ALPHA: Self = Self(18i32);
    pub const D3D11_BLEND_INV_SRC1_ALPHA: Self = Self(19i32);
}
unsafe impl ::windows::Abi for D3D11_BLEND {
    type Abi = Self;
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct D3D11_BLEND_OP(pub i32);
impl ::std::convert::From<i32> for D3D11_BLEND_OP {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
impl ::std::clone::Clone for D3D11_BLEND_OP {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}
impl ::std::default::Default for D3D11_BLEND_OP {
    fn default() -> Self {
        Self(0)
    }
}
impl ::std::fmt::Debug for D3D11_BLEND_OP {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        write!(f, "{:?}", self.0)
    }
}
impl ::std::cmp::PartialEq for D3D11_BLEND_OP {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::std::cmp::Eq for D3D11_BLEND_OP {}
impl ::std::marker::Copy for D3D11_BLEND_OP {}
impl D3D11_BLEND_OP {
    #![allow(non_upper_case_globals)]
    pub const D3D11_BLEND_OP_ADD: Self = Self(1i32);
    pub const D3D11_BLEND_OP_SUBTRACT: Self = Self(2i32);
    pub const D3D11_BLEND_OP_REV_SUBTRACT: Self = Self(3i32);
    pub const D3D11_BLEND_OP_MIN: Self = Self(4i32);
    pub const D3D11_BLEND_OP_MAX: Self = Self(5i32);
}
unsafe impl ::windows::Abi for D3D11_BLEND_OP {
    type Abi = Self;
}
#[repr(C)]
#[allow(non_camel_case_types)]
pub struct D3D11_RENDER_TARGET_BLEND_DESC {
    pub blend_enable: ::windows::BOOL,
    pub src_blend: D3D11_BLEND,
    pub dest_blend: D3D11_BLEND,
    pub blend_op: D3D11_BLEND_OP,
    pub src_blend_alpha: D3D11_BLEND,
    pub dest_blend_alpha: D3D11_BLEND,
    pub blend_op_alpha: D3D11_BLEND_OP,
    pub render_target_write_mask: u8,
}
#[repr(C)]
#[doc(hidden)]
#[allow(non_camel_case_types)]
pub struct D3D11_RENDER_TARGET_BLEND_DESC_abi(
    ::windows::BOOL,
    D3D11_BLEND,
    D3D11_BLEND,
    D3D11_BLEND_OP,
    D3D11_BLEND,
    D3D11_BLEND,
    D3D11_BLEND_OP,
    u8,
);
impl D3D11_RENDER_TARGET_BLEND_DESC {}
unsafe impl ::windows::Abi for D3D11_RENDER_TARGET_BLEND_DESC {
    type Abi = D3D11_RENDER_TARGET_BLEND_DESC_abi;
}
impl ::core::default::Default for D3D11_RENDER_TARGET_BLEND_DESC {
    fn default() -> Self {
        Self {
            blend_enable: ::std::default::Default::default(),
            src_blend: ::std::default::Default::default(),
            dest_blend: ::std::default::Default::default(),
            blend_op: ::std::default::Default::default(),
            src_blend_alpha: ::std::default::Default::default(),
            dest_blend_alpha: ::std::default::Default::default(),
            blend_op_alpha: ::std::default::Default::default(),
            render_target_write_mask: 0,
        }
    }
}
impl ::core::fmt::Debug for D3D11_RENDER_TARGET_BLEND_DESC {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("D3D11_RENDER_TARGET_BLEND_DESC")
            .field("blend_enable", &format_args!("{:?}", self.blend_enable))
            .field("src_blend", &format_args!("{:?}", self.src_blend))
            .field("dest_blend", &format_args!("{:?}", self.dest_blend))
            .field("blend_op", &format_args!("{:?}", self.blend_op))
            .field(
                "src_blend_alpha",
                &format_args!("{:?}", self.src_blend_alpha),
            )
            .field(
                "dest_blend_alpha",
                &format_args!("{:?}", self.dest_blend_alpha),
            )
            .field("blend_op_alpha", &format_args!("{:?}", self.blend_op_alpha))
            .field(
                "render_target_write_mask",
                &format_args!("{:?}", self.render_target_write_mask),
            )
            .finish()
    }
}
impl ::core::clone::Clone for D3D11_RENDER_TARGET_BLEND_DESC {
    fn clone(&self) -> Self {
        Self {
            blend_enable: <::windows::BOOL as std::clone::Clone>::clone(&self.blend_enable),
            src_blend: self.src_blend,
            dest_blend: self.dest_blend,
            blend_op: self.blend_op,
            src_blend_alpha: self.src_blend_alpha,
            dest_blend_alpha: self.dest_blend_alpha,
            blend_op_alpha: self.blend_op_alpha,
            render_target_write_mask: self.render_target_write_mask,
        }
    }
}
#[repr(C)]
#[allow(non_camel_case_types)]
pub struct D3D11_BLEND_DESC {
    pub alpha_to_coverage_enable: ::windows::BOOL,
    pub independent_blend_enable: ::windows::BOOL,
    pub render_target: [D3D11_RENDER_TARGET_BLEND_DESC; 8usize],
}
#[repr(C)]
#[doc(hidden)]
#[allow(non_camel_case_types)]
pub struct D3D11_BLEND_DESC_abi(
    ::windows::BOOL,
    ::windows::BOOL,
    [D3D11_RENDER_TARGET_BLEND_DESC_abi; 8usize],
);
impl D3D11_BLEND_DESC {}
unsafe impl ::windows::Abi for D3D11_BLEND_DESC {
    type Abi = D3D11_BLEND_DESC_abi;
}
impl ::core::default::Default for D3D11_BLEND_DESC {
    fn default() -> Self {
        Self {
            alpha_to_coverage_enable: ::std::default::Default::default(),
            independent_blend_enable: ::std::default::Default::default(),
            render_target: [
                ::std::default::Default::default(),
                ::std::default::Default::default(),
                ::std::default::Default::default(),
                ::std::default::Default::default(),
                ::std::default::Default::default(),
                ::std::default::Default::default(),
                ::std::default::Default::default(),
                ::std::default::Default::default(),
            ],
        }
    }
}
impl ::core::fmt::Debug for D3D11_BLEND_DESC {
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
impl ::core::clone::Clone for D3D11_BLEND_DESC {
    fn clone(&self) -> Self {
        Self {
            alpha_to_coverage_enable: <::windows::BOOL as std::clone::Clone>::clone(
                &self.alpha_to_coverage_enable,
            ),
            independent_blend_enable: <::windows::BOOL as std::clone::Clone>::clone(
                &self.independent_blend_enable,
            ),
            render_target: <[D3D11_RENDER_TARGET_BLEND_DESC; 8usize] as std::clone::Clone>::clone(
                &self.render_target,
            ),
        }
    }
}
#[repr(transparent)]
#[allow(non_camel_case_types)]
pub struct ID3D11BlendState(::windows::IUnknown);
impl ::std::clone::Clone for ID3D11BlendState {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::std::fmt::Debug for ID3D11BlendState {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        write!(f, "{:?}", self.0)
    }
}
impl ::std::cmp::PartialEq for ID3D11BlendState {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::std::cmp::Eq for ID3D11BlendState {}
unsafe impl ::windows::Interface for ID3D11BlendState {
    type Vtable = ID3D11BlendState_abi;
    const IID: ::windows::Guid = ::windows::Guid::from_values(
        1974898602,
        13437,
        16729,
        [143, 69, 160, 100, 15, 1, 205, 154],
    );
}
#[repr(C)]
pub struct ID3D11BlendState_abi(
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        iid: &::windows::Guid,
        interface: *mut ::windows::RawPtr,
    ) -> ::windows::ErrorCode,
    pub unsafe extern "system" fn(this: ::windows::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::RawPtr) -> u32,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        pp_device: *mut ::std::option::Option<ID3D11Device>,
    ),
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
#[allow(non_snake_case)]
impl ID3D11BlendState {
    pub unsafe fn GetDevice(&self, pp_device: *mut ::std::option::Option<ID3D11Device>) {
        (::windows::Interface::vtable(self).3)(::windows::Abi::abi(self), pp_device)
    }
    pub unsafe fn GetPrivateData(
        &self,
        guid: *const ::windows::Guid,
        p_data_size: *mut u32,
        p_data: *mut ::std::ffi::c_void,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).4)(::windows::Abi::abi(self), guid, p_data_size, p_data)
    }
    pub unsafe fn SetPrivateData(
        &self,
        guid: *const ::windows::Guid,
        data_size: u32,
        p_data: *const ::std::ffi::c_void,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).5)(::windows::Abi::abi(self), guid, data_size, p_data)
    }
    pub unsafe fn SetPrivateDataInterface<
        'a,
        T1__: ::std::convert::Into<::windows::Param<'a, ::windows::IUnknown>>,
    >(
        &self,
        guid: *const ::windows::Guid,
        p_data: T1__,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).6)(::windows::Abi::abi(self), guid, p_data.into().abi())
    }
    pub unsafe fn GetDesc(&self, p_desc: *mut D3D11_BLEND_DESC) {
        (::windows::Interface::vtable(self).7)(::windows::Abi::abi(self), p_desc)
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
impl<'a> ::std::convert::Into<::windows::Param<'a, ::windows::IUnknown>> for ID3D11BlendState {
    fn into(self) -> ::windows::Param<'a, ::windows::IUnknown> {
        ::windows::Param::Owned(::std::convert::Into::<::windows::IUnknown>::into(self))
    }
}
impl<'a> ::std::convert::Into<::windows::Param<'a, ::windows::IUnknown>> for &'a ID3D11BlendState {
    fn into(self) -> ::windows::Param<'a, ::windows::IUnknown> {
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
impl<'a> ::std::convert::Into<::windows::Param<'a, ID3D11DeviceChild>> for ID3D11BlendState {
    fn into(self) -> ::windows::Param<'a, ID3D11DeviceChild> {
        ::windows::Param::Owned(::std::convert::Into::<ID3D11DeviceChild>::into(self))
    }
}
impl<'a> ::std::convert::Into<::windows::Param<'a, ID3D11DeviceChild>> for &'a ID3D11BlendState {
    fn into(self) -> ::windows::Param<'a, ID3D11DeviceChild> {
        ::windows::Param::Owned(::std::convert::Into::<ID3D11DeviceChild>::into(
            ::std::clone::Clone::clone(self),
        ))
    }
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct D3D11_DEPTH_WRITE_MASK(pub i32);
impl ::std::convert::From<i32> for D3D11_DEPTH_WRITE_MASK {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
impl ::std::clone::Clone for D3D11_DEPTH_WRITE_MASK {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}
impl ::std::default::Default for D3D11_DEPTH_WRITE_MASK {
    fn default() -> Self {
        Self(0)
    }
}
impl ::std::fmt::Debug for D3D11_DEPTH_WRITE_MASK {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        write!(f, "{:?}", self.0)
    }
}
impl ::std::cmp::PartialEq for D3D11_DEPTH_WRITE_MASK {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::std::cmp::Eq for D3D11_DEPTH_WRITE_MASK {}
impl ::std::marker::Copy for D3D11_DEPTH_WRITE_MASK {}
impl D3D11_DEPTH_WRITE_MASK {
    #![allow(non_upper_case_globals)]
    pub const D3D11_DEPTH_WRITE_MASK_ZERO: Self = Self(0i32);
    pub const D3D11_DEPTH_WRITE_MASK_ALL: Self = Self(1i32);
}
unsafe impl ::windows::Abi for D3D11_DEPTH_WRITE_MASK {
    type Abi = Self;
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct D3D11_COMPARISON_FUNC(pub i32);
impl ::std::convert::From<i32> for D3D11_COMPARISON_FUNC {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
impl ::std::clone::Clone for D3D11_COMPARISON_FUNC {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}
impl ::std::default::Default for D3D11_COMPARISON_FUNC {
    fn default() -> Self {
        Self(0)
    }
}
impl ::std::fmt::Debug for D3D11_COMPARISON_FUNC {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        write!(f, "{:?}", self.0)
    }
}
impl ::std::cmp::PartialEq for D3D11_COMPARISON_FUNC {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::std::cmp::Eq for D3D11_COMPARISON_FUNC {}
impl ::std::marker::Copy for D3D11_COMPARISON_FUNC {}
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
unsafe impl ::windows::Abi for D3D11_COMPARISON_FUNC {
    type Abi = Self;
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct D3D11_STENCIL_OP(pub i32);
impl ::std::convert::From<i32> for D3D11_STENCIL_OP {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
impl ::std::clone::Clone for D3D11_STENCIL_OP {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}
impl ::std::default::Default for D3D11_STENCIL_OP {
    fn default() -> Self {
        Self(0)
    }
}
impl ::std::fmt::Debug for D3D11_STENCIL_OP {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        write!(f, "{:?}", self.0)
    }
}
impl ::std::cmp::PartialEq for D3D11_STENCIL_OP {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::std::cmp::Eq for D3D11_STENCIL_OP {}
impl ::std::marker::Copy for D3D11_STENCIL_OP {}
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
unsafe impl ::windows::Abi for D3D11_STENCIL_OP {
    type Abi = Self;
}
#[repr(C)]
#[allow(non_camel_case_types)]
pub struct D3D11_DEPTH_STENCILOP_DESC {
    pub stencil_fail_op: D3D11_STENCIL_OP,
    pub stencil_depth_fail_op: D3D11_STENCIL_OP,
    pub stencil_pass_op: D3D11_STENCIL_OP,
    pub stencil_func: D3D11_COMPARISON_FUNC,
}
#[repr(C)]
#[doc(hidden)]
#[allow(non_camel_case_types)]
pub struct D3D11_DEPTH_STENCILOP_DESC_abi(
    D3D11_STENCIL_OP,
    D3D11_STENCIL_OP,
    D3D11_STENCIL_OP,
    D3D11_COMPARISON_FUNC,
);
impl D3D11_DEPTH_STENCILOP_DESC {}
unsafe impl ::windows::Abi for D3D11_DEPTH_STENCILOP_DESC {
    type Abi = D3D11_DEPTH_STENCILOP_DESC_abi;
}
impl ::core::default::Default for D3D11_DEPTH_STENCILOP_DESC {
    fn default() -> Self {
        Self {
            stencil_fail_op: ::std::default::Default::default(),
            stencil_depth_fail_op: ::std::default::Default::default(),
            stencil_pass_op: ::std::default::Default::default(),
            stencil_func: ::std::default::Default::default(),
        }
    }
}
impl ::core::fmt::Debug for D3D11_DEPTH_STENCILOP_DESC {
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
impl ::core::clone::Clone for D3D11_DEPTH_STENCILOP_DESC {
    fn clone(&self) -> Self {
        Self {
            stencil_fail_op: self.stencil_fail_op,
            stencil_depth_fail_op: self.stencil_depth_fail_op,
            stencil_pass_op: self.stencil_pass_op,
            stencil_func: self.stencil_func,
        }
    }
}
#[repr(C)]
#[allow(non_camel_case_types)]
pub struct D3D11_DEPTH_STENCIL_DESC {
    pub depth_enable: ::windows::BOOL,
    pub depth_write_mask: D3D11_DEPTH_WRITE_MASK,
    pub depth_func: D3D11_COMPARISON_FUNC,
    pub stencil_enable: ::windows::BOOL,
    pub stencil_read_mask: u8,
    pub stencil_write_mask: u8,
    pub front_face: D3D11_DEPTH_STENCILOP_DESC,
    pub back_face: D3D11_DEPTH_STENCILOP_DESC,
}
#[repr(C)]
#[doc(hidden)]
#[allow(non_camel_case_types)]
pub struct D3D11_DEPTH_STENCIL_DESC_abi(
    ::windows::BOOL,
    D3D11_DEPTH_WRITE_MASK,
    D3D11_COMPARISON_FUNC,
    ::windows::BOOL,
    u8,
    u8,
    D3D11_DEPTH_STENCILOP_DESC_abi,
    D3D11_DEPTH_STENCILOP_DESC_abi,
);
impl D3D11_DEPTH_STENCIL_DESC {}
unsafe impl ::windows::Abi for D3D11_DEPTH_STENCIL_DESC {
    type Abi = D3D11_DEPTH_STENCIL_DESC_abi;
}
impl ::core::default::Default for D3D11_DEPTH_STENCIL_DESC {
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
impl ::core::fmt::Debug for D3D11_DEPTH_STENCIL_DESC {
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
impl ::core::clone::Clone for D3D11_DEPTH_STENCIL_DESC {
    fn clone(&self) -> Self {
        Self {
            depth_enable: <::windows::BOOL as std::clone::Clone>::clone(&self.depth_enable),
            depth_write_mask: self.depth_write_mask,
            depth_func: self.depth_func,
            stencil_enable: <::windows::BOOL as std::clone::Clone>::clone(&self.stencil_enable),
            stencil_read_mask: self.stencil_read_mask,
            stencil_write_mask: self.stencil_write_mask,
            front_face: <D3D11_DEPTH_STENCILOP_DESC as std::clone::Clone>::clone(&self.front_face),
            back_face: <D3D11_DEPTH_STENCILOP_DESC as std::clone::Clone>::clone(&self.back_face),
        }
    }
}
#[repr(transparent)]
#[allow(non_camel_case_types)]
pub struct ID3D11DepthStencilState(::windows::IUnknown);
impl ::std::clone::Clone for ID3D11DepthStencilState {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::std::fmt::Debug for ID3D11DepthStencilState {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        write!(f, "{:?}", self.0)
    }
}
impl ::std::cmp::PartialEq for ID3D11DepthStencilState {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::std::cmp::Eq for ID3D11DepthStencilState {}
unsafe impl ::windows::Interface for ID3D11DepthStencilState {
    type Vtable = ID3D11DepthStencilState_abi;
    const IID: ::windows::Guid = ::windows::Guid::from_values(
        58867451,
        36239,
        19996,
        [154, 162, 246, 75, 178, 203, 253, 241],
    );
}
#[repr(C)]
pub struct ID3D11DepthStencilState_abi(
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        iid: &::windows::Guid,
        interface: *mut ::windows::RawPtr,
    ) -> ::windows::ErrorCode,
    pub unsafe extern "system" fn(this: ::windows::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::RawPtr) -> u32,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        pp_device: *mut ::std::option::Option<ID3D11Device>,
    ),
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
#[allow(non_snake_case)]
impl ID3D11DepthStencilState {
    pub unsafe fn GetDevice(&self, pp_device: *mut ::std::option::Option<ID3D11Device>) {
        (::windows::Interface::vtable(self).3)(::windows::Abi::abi(self), pp_device)
    }
    pub unsafe fn GetPrivateData(
        &self,
        guid: *const ::windows::Guid,
        p_data_size: *mut u32,
        p_data: *mut ::std::ffi::c_void,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).4)(::windows::Abi::abi(self), guid, p_data_size, p_data)
    }
    pub unsafe fn SetPrivateData(
        &self,
        guid: *const ::windows::Guid,
        data_size: u32,
        p_data: *const ::std::ffi::c_void,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).5)(::windows::Abi::abi(self), guid, data_size, p_data)
    }
    pub unsafe fn SetPrivateDataInterface<
        'a,
        T1__: ::std::convert::Into<::windows::Param<'a, ::windows::IUnknown>>,
    >(
        &self,
        guid: *const ::windows::Guid,
        p_data: T1__,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).6)(::windows::Abi::abi(self), guid, p_data.into().abi())
    }
    pub unsafe fn GetDesc(&self, p_desc: *mut D3D11_DEPTH_STENCIL_DESC) {
        (::windows::Interface::vtable(self).7)(::windows::Abi::abi(self), p_desc)
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
impl<'a> ::std::convert::Into<::windows::Param<'a, ::windows::IUnknown>>
    for ID3D11DepthStencilState
{
    fn into(self) -> ::windows::Param<'a, ::windows::IUnknown> {
        ::windows::Param::Owned(::std::convert::Into::<::windows::IUnknown>::into(self))
    }
}
impl<'a> ::std::convert::Into<::windows::Param<'a, ::windows::IUnknown>>
    for &'a ID3D11DepthStencilState
{
    fn into(self) -> ::windows::Param<'a, ::windows::IUnknown> {
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
impl<'a> ::std::convert::Into<::windows::Param<'a, ID3D11DeviceChild>> for ID3D11DepthStencilState {
    fn into(self) -> ::windows::Param<'a, ID3D11DeviceChild> {
        ::windows::Param::Owned(::std::convert::Into::<ID3D11DeviceChild>::into(self))
    }
}
impl<'a> ::std::convert::Into<::windows::Param<'a, ID3D11DeviceChild>>
    for &'a ID3D11DepthStencilState
{
    fn into(self) -> ::windows::Param<'a, ID3D11DeviceChild> {
        ::windows::Param::Owned(::std::convert::Into::<ID3D11DeviceChild>::into(
            ::std::clone::Clone::clone(self),
        ))
    }
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct D3D11_FILL_MODE(pub i32);
impl ::std::convert::From<i32> for D3D11_FILL_MODE {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
impl ::std::clone::Clone for D3D11_FILL_MODE {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}
impl ::std::default::Default for D3D11_FILL_MODE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::std::fmt::Debug for D3D11_FILL_MODE {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        write!(f, "{:?}", self.0)
    }
}
impl ::std::cmp::PartialEq for D3D11_FILL_MODE {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::std::cmp::Eq for D3D11_FILL_MODE {}
impl ::std::marker::Copy for D3D11_FILL_MODE {}
impl D3D11_FILL_MODE {
    #![allow(non_upper_case_globals)]
    pub const D3D11_FILL_WIREFRAME: Self = Self(2i32);
    pub const D3D11_FILL_SOLID: Self = Self(3i32);
}
unsafe impl ::windows::Abi for D3D11_FILL_MODE {
    type Abi = Self;
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct D3D11_CULL_MODE(pub i32);
impl ::std::convert::From<i32> for D3D11_CULL_MODE {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
impl ::std::clone::Clone for D3D11_CULL_MODE {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}
impl ::std::default::Default for D3D11_CULL_MODE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::std::fmt::Debug for D3D11_CULL_MODE {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        write!(f, "{:?}", self.0)
    }
}
impl ::std::cmp::PartialEq for D3D11_CULL_MODE {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::std::cmp::Eq for D3D11_CULL_MODE {}
impl ::std::marker::Copy for D3D11_CULL_MODE {}
impl D3D11_CULL_MODE {
    #![allow(non_upper_case_globals)]
    pub const D3D11_CULL_NONE: Self = Self(1i32);
    pub const D3D11_CULL_FRONT: Self = Self(2i32);
    pub const D3D11_CULL_BACK: Self = Self(3i32);
}
unsafe impl ::windows::Abi for D3D11_CULL_MODE {
    type Abi = Self;
}
#[repr(C)]
#[allow(non_camel_case_types)]
pub struct D3D11_RASTERIZER_DESC {
    pub fill_mode: D3D11_FILL_MODE,
    pub cull_mode: D3D11_CULL_MODE,
    pub front_counter_clockwise: ::windows::BOOL,
    pub depth_bias: i32,
    pub depth_bias_clamp: f32,
    pub slope_scaled_depth_bias: f32,
    pub depth_clip_enable: ::windows::BOOL,
    pub scissor_enable: ::windows::BOOL,
    pub multisample_enable: ::windows::BOOL,
    pub antialiased_line_enable: ::windows::BOOL,
}
#[repr(C)]
#[doc(hidden)]
#[allow(non_camel_case_types)]
pub struct D3D11_RASTERIZER_DESC_abi(
    D3D11_FILL_MODE,
    D3D11_CULL_MODE,
    ::windows::BOOL,
    i32,
    f32,
    f32,
    ::windows::BOOL,
    ::windows::BOOL,
    ::windows::BOOL,
    ::windows::BOOL,
);
impl D3D11_RASTERIZER_DESC {}
unsafe impl ::windows::Abi for D3D11_RASTERIZER_DESC {
    type Abi = D3D11_RASTERIZER_DESC_abi;
}
impl ::core::default::Default for D3D11_RASTERIZER_DESC {
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
impl ::core::fmt::Debug for D3D11_RASTERIZER_DESC {
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
impl ::core::clone::Clone for D3D11_RASTERIZER_DESC {
    fn clone(&self) -> Self {
        Self {
            fill_mode: self.fill_mode,
            cull_mode: self.cull_mode,
            front_counter_clockwise: <::windows::BOOL as std::clone::Clone>::clone(
                &self.front_counter_clockwise,
            ),
            depth_bias: self.depth_bias,
            depth_bias_clamp: self.depth_bias_clamp,
            slope_scaled_depth_bias: self.slope_scaled_depth_bias,
            depth_clip_enable: <::windows::BOOL as std::clone::Clone>::clone(
                &self.depth_clip_enable,
            ),
            scissor_enable: <::windows::BOOL as std::clone::Clone>::clone(&self.scissor_enable),
            multisample_enable: <::windows::BOOL as std::clone::Clone>::clone(
                &self.multisample_enable,
            ),
            antialiased_line_enable: <::windows::BOOL as std::clone::Clone>::clone(
                &self.antialiased_line_enable,
            ),
        }
    }
}
#[repr(transparent)]
#[allow(non_camel_case_types)]
pub struct ID3D11RasterizerState(::windows::IUnknown);
impl ::std::clone::Clone for ID3D11RasterizerState {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::std::fmt::Debug for ID3D11RasterizerState {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        write!(f, "{:?}", self.0)
    }
}
impl ::std::cmp::PartialEq for ID3D11RasterizerState {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::std::cmp::Eq for ID3D11RasterizerState {}
unsafe impl ::windows::Interface for ID3D11RasterizerState {
    type Vtable = ID3D11RasterizerState_abi;
    const IID: ::windows::Guid =
        ::windows::Guid::from_values(2612308865, 43802, 19855, [181, 6, 252, 4, 32, 11, 110, 231]);
}
#[repr(C)]
pub struct ID3D11RasterizerState_abi(
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        iid: &::windows::Guid,
        interface: *mut ::windows::RawPtr,
    ) -> ::windows::ErrorCode,
    pub unsafe extern "system" fn(this: ::windows::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::RawPtr) -> u32,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        pp_device: *mut ::std::option::Option<ID3D11Device>,
    ),
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
#[allow(non_snake_case)]
impl ID3D11RasterizerState {
    pub unsafe fn GetDevice(&self, pp_device: *mut ::std::option::Option<ID3D11Device>) {
        (::windows::Interface::vtable(self).3)(::windows::Abi::abi(self), pp_device)
    }
    pub unsafe fn GetPrivateData(
        &self,
        guid: *const ::windows::Guid,
        p_data_size: *mut u32,
        p_data: *mut ::std::ffi::c_void,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).4)(::windows::Abi::abi(self), guid, p_data_size, p_data)
    }
    pub unsafe fn SetPrivateData(
        &self,
        guid: *const ::windows::Guid,
        data_size: u32,
        p_data: *const ::std::ffi::c_void,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).5)(::windows::Abi::abi(self), guid, data_size, p_data)
    }
    pub unsafe fn SetPrivateDataInterface<
        'a,
        T1__: ::std::convert::Into<::windows::Param<'a, ::windows::IUnknown>>,
    >(
        &self,
        guid: *const ::windows::Guid,
        p_data: T1__,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).6)(::windows::Abi::abi(self), guid, p_data.into().abi())
    }
    pub unsafe fn GetDesc(&self, p_desc: *mut D3D11_RASTERIZER_DESC) {
        (::windows::Interface::vtable(self).7)(::windows::Abi::abi(self), p_desc)
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
impl<'a> ::std::convert::Into<::windows::Param<'a, ::windows::IUnknown>> for ID3D11RasterizerState {
    fn into(self) -> ::windows::Param<'a, ::windows::IUnknown> {
        ::windows::Param::Owned(::std::convert::Into::<::windows::IUnknown>::into(self))
    }
}
impl<'a> ::std::convert::Into<::windows::Param<'a, ::windows::IUnknown>>
    for &'a ID3D11RasterizerState
{
    fn into(self) -> ::windows::Param<'a, ::windows::IUnknown> {
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
impl<'a> ::std::convert::Into<::windows::Param<'a, ID3D11DeviceChild>> for ID3D11RasterizerState {
    fn into(self) -> ::windows::Param<'a, ID3D11DeviceChild> {
        ::windows::Param::Owned(::std::convert::Into::<ID3D11DeviceChild>::into(self))
    }
}
impl<'a> ::std::convert::Into<::windows::Param<'a, ID3D11DeviceChild>>
    for &'a ID3D11RasterizerState
{
    fn into(self) -> ::windows::Param<'a, ID3D11DeviceChild> {
        ::windows::Param::Owned(::std::convert::Into::<ID3D11DeviceChild>::into(
            ::std::clone::Clone::clone(self),
        ))
    }
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct D3D11_FILTER(pub i32);
impl ::std::convert::From<i32> for D3D11_FILTER {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
impl ::std::clone::Clone for D3D11_FILTER {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}
impl ::std::default::Default for D3D11_FILTER {
    fn default() -> Self {
        Self(0)
    }
}
impl ::std::fmt::Debug for D3D11_FILTER {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        write!(f, "{:?}", self.0)
    }
}
impl ::std::cmp::PartialEq for D3D11_FILTER {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::std::cmp::Eq for D3D11_FILTER {}
impl ::std::marker::Copy for D3D11_FILTER {}
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
unsafe impl ::windows::Abi for D3D11_FILTER {
    type Abi = Self;
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct D3D11_TEXTURE_ADDRESS_MODE(pub i32);
impl ::std::convert::From<i32> for D3D11_TEXTURE_ADDRESS_MODE {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
impl ::std::clone::Clone for D3D11_TEXTURE_ADDRESS_MODE {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}
impl ::std::default::Default for D3D11_TEXTURE_ADDRESS_MODE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::std::fmt::Debug for D3D11_TEXTURE_ADDRESS_MODE {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        write!(f, "{:?}", self.0)
    }
}
impl ::std::cmp::PartialEq for D3D11_TEXTURE_ADDRESS_MODE {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::std::cmp::Eq for D3D11_TEXTURE_ADDRESS_MODE {}
impl ::std::marker::Copy for D3D11_TEXTURE_ADDRESS_MODE {}
impl D3D11_TEXTURE_ADDRESS_MODE {
    #![allow(non_upper_case_globals)]
    pub const D3D11_TEXTURE_ADDRESS_WRAP: Self = Self(1i32);
    pub const D3D11_TEXTURE_ADDRESS_MIRROR: Self = Self(2i32);
    pub const D3D11_TEXTURE_ADDRESS_CLAMP: Self = Self(3i32);
    pub const D3D11_TEXTURE_ADDRESS_BORDER: Self = Self(4i32);
    pub const D3D11_TEXTURE_ADDRESS_MIRROR_ONCE: Self = Self(5i32);
}
unsafe impl ::windows::Abi for D3D11_TEXTURE_ADDRESS_MODE {
    type Abi = Self;
}
#[repr(C)]
#[allow(non_camel_case_types)]
pub struct D3D11_SAMPLER_DESC {
    pub filter: D3D11_FILTER,
    pub addressu: D3D11_TEXTURE_ADDRESS_MODE,
    pub addressv: D3D11_TEXTURE_ADDRESS_MODE,
    pub addressw: D3D11_TEXTURE_ADDRESS_MODE,
    pub mip_lod_bias: f32,
    pub max_anisotropy: u32,
    pub comparison_func: D3D11_COMPARISON_FUNC,
    pub border_color: [f32; 4usize],
    pub min_lod: f32,
    pub max_lod: f32,
}
#[repr(C)]
#[doc(hidden)]
#[allow(non_camel_case_types)]
pub struct D3D11_SAMPLER_DESC_abi(
    D3D11_FILTER,
    D3D11_TEXTURE_ADDRESS_MODE,
    D3D11_TEXTURE_ADDRESS_MODE,
    D3D11_TEXTURE_ADDRESS_MODE,
    f32,
    u32,
    D3D11_COMPARISON_FUNC,
    [f32; 4usize],
    f32,
    f32,
);
impl D3D11_SAMPLER_DESC {}
unsafe impl ::windows::Abi for D3D11_SAMPLER_DESC {
    type Abi = D3D11_SAMPLER_DESC_abi;
}
impl ::core::default::Default for D3D11_SAMPLER_DESC {
    fn default() -> Self {
        Self {
            filter: ::std::default::Default::default(),
            addressu: ::std::default::Default::default(),
            addressv: ::std::default::Default::default(),
            addressw: ::std::default::Default::default(),
            mip_lod_bias: 0.0,
            max_anisotropy: 0,
            comparison_func: ::std::default::Default::default(),
            border_color: [0.0, 0.0, 0.0, 0.0],
            min_lod: 0.0,
            max_lod: 0.0,
        }
    }
}
impl ::core::fmt::Debug for D3D11_SAMPLER_DESC {
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
impl ::core::clone::Clone for D3D11_SAMPLER_DESC {
    fn clone(&self) -> Self {
        Self {
            filter: self.filter,
            addressu: self.addressu,
            addressv: self.addressv,
            addressw: self.addressw,
            mip_lod_bias: self.mip_lod_bias,
            max_anisotropy: self.max_anisotropy,
            comparison_func: self.comparison_func,
            border_color: <[f32; 4usize] as std::clone::Clone>::clone(&self.border_color),
            min_lod: self.min_lod,
            max_lod: self.max_lod,
        }
    }
}
#[repr(transparent)]
#[allow(non_camel_case_types)]
pub struct ID3D11SamplerState(::windows::IUnknown);
impl ::std::clone::Clone for ID3D11SamplerState {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::std::fmt::Debug for ID3D11SamplerState {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        write!(f, "{:?}", self.0)
    }
}
impl ::std::cmp::PartialEq for ID3D11SamplerState {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::std::cmp::Eq for ID3D11SamplerState {}
unsafe impl ::windows::Interface for ID3D11SamplerState {
    type Vtable = ID3D11SamplerState_abi;
    const IID: ::windows::Guid = ::windows::Guid::from_values(
        3664767569,
        22092,
        17543,
        [152, 16, 240, 208, 249, 180, 227, 165],
    );
}
#[repr(C)]
pub struct ID3D11SamplerState_abi(
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        iid: &::windows::Guid,
        interface: *mut ::windows::RawPtr,
    ) -> ::windows::ErrorCode,
    pub unsafe extern "system" fn(this: ::windows::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::RawPtr) -> u32,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        pp_device: *mut ::std::option::Option<ID3D11Device>,
    ),
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
#[allow(non_snake_case)]
impl ID3D11SamplerState {
    pub unsafe fn GetDevice(&self, pp_device: *mut ::std::option::Option<ID3D11Device>) {
        (::windows::Interface::vtable(self).3)(::windows::Abi::abi(self), pp_device)
    }
    pub unsafe fn GetPrivateData(
        &self,
        guid: *const ::windows::Guid,
        p_data_size: *mut u32,
        p_data: *mut ::std::ffi::c_void,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).4)(::windows::Abi::abi(self), guid, p_data_size, p_data)
    }
    pub unsafe fn SetPrivateData(
        &self,
        guid: *const ::windows::Guid,
        data_size: u32,
        p_data: *const ::std::ffi::c_void,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).5)(::windows::Abi::abi(self), guid, data_size, p_data)
    }
    pub unsafe fn SetPrivateDataInterface<
        'a,
        T1__: ::std::convert::Into<::windows::Param<'a, ::windows::IUnknown>>,
    >(
        &self,
        guid: *const ::windows::Guid,
        p_data: T1__,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).6)(::windows::Abi::abi(self), guid, p_data.into().abi())
    }
    pub unsafe fn GetDesc(&self, p_desc: *mut D3D11_SAMPLER_DESC) {
        (::windows::Interface::vtable(self).7)(::windows::Abi::abi(self), p_desc)
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
impl<'a> ::std::convert::Into<::windows::Param<'a, ::windows::IUnknown>> for ID3D11SamplerState {
    fn into(self) -> ::windows::Param<'a, ::windows::IUnknown> {
        ::windows::Param::Owned(::std::convert::Into::<::windows::IUnknown>::into(self))
    }
}
impl<'a> ::std::convert::Into<::windows::Param<'a, ::windows::IUnknown>>
    for &'a ID3D11SamplerState
{
    fn into(self) -> ::windows::Param<'a, ::windows::IUnknown> {
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
impl<'a> ::std::convert::Into<::windows::Param<'a, ID3D11DeviceChild>> for ID3D11SamplerState {
    fn into(self) -> ::windows::Param<'a, ID3D11DeviceChild> {
        ::windows::Param::Owned(::std::convert::Into::<ID3D11DeviceChild>::into(self))
    }
}
impl<'a> ::std::convert::Into<::windows::Param<'a, ID3D11DeviceChild>> for &'a ID3D11SamplerState {
    fn into(self) -> ::windows::Param<'a, ID3D11DeviceChild> {
        ::windows::Param::Owned(::std::convert::Into::<ID3D11DeviceChild>::into(
            ::std::clone::Clone::clone(self),
        ))
    }
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct D3D11_QUERY(pub i32);
impl ::std::convert::From<i32> for D3D11_QUERY {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
impl ::std::clone::Clone for D3D11_QUERY {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}
impl ::std::default::Default for D3D11_QUERY {
    fn default() -> Self {
        Self(0)
    }
}
impl ::std::fmt::Debug for D3D11_QUERY {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        write!(f, "{:?}", self.0)
    }
}
impl ::std::cmp::PartialEq for D3D11_QUERY {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::std::cmp::Eq for D3D11_QUERY {}
impl ::std::marker::Copy for D3D11_QUERY {}
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
unsafe impl ::windows::Abi for D3D11_QUERY {
    type Abi = Self;
}
#[repr(C)]
#[allow(non_camel_case_types)]
pub struct D3D11_QUERY_DESC {
    pub query: D3D11_QUERY,
    pub misc_flags: u32,
}
#[repr(C)]
#[doc(hidden)]
#[allow(non_camel_case_types)]
pub struct D3D11_QUERY_DESC_abi(D3D11_QUERY, u32);
impl D3D11_QUERY_DESC {}
unsafe impl ::windows::Abi for D3D11_QUERY_DESC {
    type Abi = D3D11_QUERY_DESC_abi;
}
impl ::core::default::Default for D3D11_QUERY_DESC {
    fn default() -> Self {
        Self {
            query: ::std::default::Default::default(),
            misc_flags: 0,
        }
    }
}
impl ::core::fmt::Debug for D3D11_QUERY_DESC {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("D3D11_QUERY_DESC")
            .field("query", &format_args!("{:?}", self.query))
            .field("misc_flags", &format_args!("{:?}", self.misc_flags))
            .finish()
    }
}
impl ::core::clone::Clone for D3D11_QUERY_DESC {
    fn clone(&self) -> Self {
        Self {
            query: self.query,
            misc_flags: self.misc_flags,
        }
    }
}
#[repr(transparent)]
#[allow(non_camel_case_types)]
pub struct ID3D11Asynchronous(::windows::IUnknown);
impl ::std::clone::Clone for ID3D11Asynchronous {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::std::fmt::Debug for ID3D11Asynchronous {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        write!(f, "{:?}", self.0)
    }
}
impl ::std::cmp::PartialEq for ID3D11Asynchronous {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::std::cmp::Eq for ID3D11Asynchronous {}
unsafe impl ::windows::Interface for ID3D11Asynchronous {
    type Vtable = ID3D11Asynchronous_abi;
    const IID: ::windows::Guid = ::windows::Guid::from_values(
        1261818061,
        7701,
        16984,
        [156, 152, 27, 19, 51, 246, 221, 59],
    );
}
#[repr(C)]
pub struct ID3D11Asynchronous_abi(
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        iid: &::windows::Guid,
        interface: *mut ::windows::RawPtr,
    ) -> ::windows::ErrorCode,
    pub unsafe extern "system" fn(this: ::windows::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::RawPtr) -> u32,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        pp_device: *mut ::std::option::Option<ID3D11Device>,
    ),
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
#[allow(non_snake_case)]
impl ID3D11Asynchronous {
    pub unsafe fn GetDevice(&self, pp_device: *mut ::std::option::Option<ID3D11Device>) {
        (::windows::Interface::vtable(self).3)(::windows::Abi::abi(self), pp_device)
    }
    pub unsafe fn GetPrivateData(
        &self,
        guid: *const ::windows::Guid,
        p_data_size: *mut u32,
        p_data: *mut ::std::ffi::c_void,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).4)(::windows::Abi::abi(self), guid, p_data_size, p_data)
    }
    pub unsafe fn SetPrivateData(
        &self,
        guid: *const ::windows::Guid,
        data_size: u32,
        p_data: *const ::std::ffi::c_void,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).5)(::windows::Abi::abi(self), guid, data_size, p_data)
    }
    pub unsafe fn SetPrivateDataInterface<
        'a,
        T1__: ::std::convert::Into<::windows::Param<'a, ::windows::IUnknown>>,
    >(
        &self,
        guid: *const ::windows::Guid,
        p_data: T1__,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).6)(::windows::Abi::abi(self), guid, p_data.into().abi())
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
impl<'a> ::std::convert::Into<::windows::Param<'a, ::windows::IUnknown>> for ID3D11Asynchronous {
    fn into(self) -> ::windows::Param<'a, ::windows::IUnknown> {
        ::windows::Param::Owned(::std::convert::Into::<::windows::IUnknown>::into(self))
    }
}
impl<'a> ::std::convert::Into<::windows::Param<'a, ::windows::IUnknown>>
    for &'a ID3D11Asynchronous
{
    fn into(self) -> ::windows::Param<'a, ::windows::IUnknown> {
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
impl<'a> ::std::convert::Into<::windows::Param<'a, ID3D11DeviceChild>> for ID3D11Asynchronous {
    fn into(self) -> ::windows::Param<'a, ID3D11DeviceChild> {
        ::windows::Param::Owned(::std::convert::Into::<ID3D11DeviceChild>::into(self))
    }
}
impl<'a> ::std::convert::Into<::windows::Param<'a, ID3D11DeviceChild>> for &'a ID3D11Asynchronous {
    fn into(self) -> ::windows::Param<'a, ID3D11DeviceChild> {
        ::windows::Param::Owned(::std::convert::Into::<ID3D11DeviceChild>::into(
            ::std::clone::Clone::clone(self),
        ))
    }
}
#[repr(transparent)]
#[allow(non_camel_case_types)]
pub struct ID3D11Query(::windows::IUnknown);
impl ::std::clone::Clone for ID3D11Query {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::std::fmt::Debug for ID3D11Query {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        write!(f, "{:?}", self.0)
    }
}
impl ::std::cmp::PartialEq for ID3D11Query {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::std::cmp::Eq for ID3D11Query {}
unsafe impl ::windows::Interface for ID3D11Query {
    type Vtable = ID3D11Query_abi;
    const IID: ::windows::Guid =
        ::windows::Guid::from_values(3602908999, 34743, 16990, [184, 77, 68, 209, 8, 86, 10, 253]);
}
#[repr(C)]
pub struct ID3D11Query_abi(
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        iid: &::windows::Guid,
        interface: *mut ::windows::RawPtr,
    ) -> ::windows::ErrorCode,
    pub unsafe extern "system" fn(this: ::windows::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::RawPtr) -> u32,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        pp_device: *mut ::std::option::Option<ID3D11Device>,
    ),
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
#[allow(non_snake_case)]
impl ID3D11Query {
    pub unsafe fn GetDevice(&self, pp_device: *mut ::std::option::Option<ID3D11Device>) {
        (::windows::Interface::vtable(self).3)(::windows::Abi::abi(self), pp_device)
    }
    pub unsafe fn GetPrivateData(
        &self,
        guid: *const ::windows::Guid,
        p_data_size: *mut u32,
        p_data: *mut ::std::ffi::c_void,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).4)(::windows::Abi::abi(self), guid, p_data_size, p_data)
    }
    pub unsafe fn SetPrivateData(
        &self,
        guid: *const ::windows::Guid,
        data_size: u32,
        p_data: *const ::std::ffi::c_void,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).5)(::windows::Abi::abi(self), guid, data_size, p_data)
    }
    pub unsafe fn SetPrivateDataInterface<
        'a,
        T1__: ::std::convert::Into<::windows::Param<'a, ::windows::IUnknown>>,
    >(
        &self,
        guid: *const ::windows::Guid,
        p_data: T1__,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).6)(::windows::Abi::abi(self), guid, p_data.into().abi())
    }
    pub unsafe fn GetDataSize(&self) -> u32 {
        (::windows::Interface::vtable(self).7)(::windows::Abi::abi(self))
    }
    pub unsafe fn GetDesc(&self, p_desc: *mut D3D11_QUERY_DESC) {
        (::windows::Interface::vtable(self).8)(::windows::Abi::abi(self), p_desc)
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
impl<'a> ::std::convert::Into<::windows::Param<'a, ::windows::IUnknown>> for ID3D11Query {
    fn into(self) -> ::windows::Param<'a, ::windows::IUnknown> {
        ::windows::Param::Owned(::std::convert::Into::<::windows::IUnknown>::into(self))
    }
}
impl<'a> ::std::convert::Into<::windows::Param<'a, ::windows::IUnknown>> for &'a ID3D11Query {
    fn into(self) -> ::windows::Param<'a, ::windows::IUnknown> {
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
impl<'a> ::std::convert::Into<::windows::Param<'a, ID3D11Asynchronous>> for ID3D11Query {
    fn into(self) -> ::windows::Param<'a, ID3D11Asynchronous> {
        ::windows::Param::Owned(::std::convert::Into::<ID3D11Asynchronous>::into(self))
    }
}
impl<'a> ::std::convert::Into<::windows::Param<'a, ID3D11Asynchronous>> for &'a ID3D11Query {
    fn into(self) -> ::windows::Param<'a, ID3D11Asynchronous> {
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
impl<'a> ::std::convert::Into<::windows::Param<'a, ID3D11DeviceChild>> for ID3D11Query {
    fn into(self) -> ::windows::Param<'a, ID3D11DeviceChild> {
        ::windows::Param::Owned(::std::convert::Into::<ID3D11DeviceChild>::into(self))
    }
}
impl<'a> ::std::convert::Into<::windows::Param<'a, ID3D11DeviceChild>> for &'a ID3D11Query {
    fn into(self) -> ::windows::Param<'a, ID3D11DeviceChild> {
        ::windows::Param::Owned(::std::convert::Into::<ID3D11DeviceChild>::into(
            ::std::clone::Clone::clone(self),
        ))
    }
}
#[repr(transparent)]
#[allow(non_camel_case_types)]
pub struct ID3D11Predicate(::windows::IUnknown);
impl ::std::clone::Clone for ID3D11Predicate {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::std::fmt::Debug for ID3D11Predicate {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        write!(f, "{:?}", self.0)
    }
}
impl ::std::cmp::PartialEq for ID3D11Predicate {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::std::cmp::Eq for ID3D11Predicate {}
unsafe impl ::windows::Interface for ID3D11Predicate {
    type Vtable = ID3D11Predicate_abi;
    const IID: ::windows::Guid = ::windows::Guid::from_values(
        2662692573,
        40823,
        19846,
        [129, 170, 139, 171, 95, 228, 144, 226],
    );
}
#[repr(C)]
pub struct ID3D11Predicate_abi(
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        iid: &::windows::Guid,
        interface: *mut ::windows::RawPtr,
    ) -> ::windows::ErrorCode,
    pub unsafe extern "system" fn(this: ::windows::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::RawPtr) -> u32,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        pp_device: *mut ::std::option::Option<ID3D11Device>,
    ),
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
#[allow(non_snake_case)]
impl ID3D11Predicate {
    pub unsafe fn GetDevice(&self, pp_device: *mut ::std::option::Option<ID3D11Device>) {
        (::windows::Interface::vtable(self).3)(::windows::Abi::abi(self), pp_device)
    }
    pub unsafe fn GetPrivateData(
        &self,
        guid: *const ::windows::Guid,
        p_data_size: *mut u32,
        p_data: *mut ::std::ffi::c_void,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).4)(::windows::Abi::abi(self), guid, p_data_size, p_data)
    }
    pub unsafe fn SetPrivateData(
        &self,
        guid: *const ::windows::Guid,
        data_size: u32,
        p_data: *const ::std::ffi::c_void,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).5)(::windows::Abi::abi(self), guid, data_size, p_data)
    }
    pub unsafe fn SetPrivateDataInterface<
        'a,
        T1__: ::std::convert::Into<::windows::Param<'a, ::windows::IUnknown>>,
    >(
        &self,
        guid: *const ::windows::Guid,
        p_data: T1__,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).6)(::windows::Abi::abi(self), guid, p_data.into().abi())
    }
    pub unsafe fn GetDataSize(&self) -> u32 {
        (::windows::Interface::vtable(self).7)(::windows::Abi::abi(self))
    }
    pub unsafe fn GetDesc(&self, p_desc: *mut D3D11_QUERY_DESC) {
        (::windows::Interface::vtable(self).8)(::windows::Abi::abi(self), p_desc)
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
impl<'a> ::std::convert::Into<::windows::Param<'a, ::windows::IUnknown>> for ID3D11Predicate {
    fn into(self) -> ::windows::Param<'a, ::windows::IUnknown> {
        ::windows::Param::Owned(::std::convert::Into::<::windows::IUnknown>::into(self))
    }
}
impl<'a> ::std::convert::Into<::windows::Param<'a, ::windows::IUnknown>> for &'a ID3D11Predicate {
    fn into(self) -> ::windows::Param<'a, ::windows::IUnknown> {
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
impl<'a> ::std::convert::Into<::windows::Param<'a, ID3D11Query>> for ID3D11Predicate {
    fn into(self) -> ::windows::Param<'a, ID3D11Query> {
        ::windows::Param::Owned(::std::convert::Into::<ID3D11Query>::into(self))
    }
}
impl<'a> ::std::convert::Into<::windows::Param<'a, ID3D11Query>> for &'a ID3D11Predicate {
    fn into(self) -> ::windows::Param<'a, ID3D11Query> {
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
impl<'a> ::std::convert::Into<::windows::Param<'a, ID3D11Asynchronous>> for ID3D11Predicate {
    fn into(self) -> ::windows::Param<'a, ID3D11Asynchronous> {
        ::windows::Param::Owned(::std::convert::Into::<ID3D11Asynchronous>::into(self))
    }
}
impl<'a> ::std::convert::Into<::windows::Param<'a, ID3D11Asynchronous>> for &'a ID3D11Predicate {
    fn into(self) -> ::windows::Param<'a, ID3D11Asynchronous> {
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
impl<'a> ::std::convert::Into<::windows::Param<'a, ID3D11DeviceChild>> for ID3D11Predicate {
    fn into(self) -> ::windows::Param<'a, ID3D11DeviceChild> {
        ::windows::Param::Owned(::std::convert::Into::<ID3D11DeviceChild>::into(self))
    }
}
impl<'a> ::std::convert::Into<::windows::Param<'a, ID3D11DeviceChild>> for &'a ID3D11Predicate {
    fn into(self) -> ::windows::Param<'a, ID3D11DeviceChild> {
        ::windows::Param::Owned(::std::convert::Into::<ID3D11DeviceChild>::into(
            ::std::clone::Clone::clone(self),
        ))
    }
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct D3D11_COUNTER(pub i32);
impl ::std::convert::From<i32> for D3D11_COUNTER {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
impl ::std::clone::Clone for D3D11_COUNTER {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}
impl ::std::default::Default for D3D11_COUNTER {
    fn default() -> Self {
        Self(0)
    }
}
impl ::std::fmt::Debug for D3D11_COUNTER {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        write!(f, "{:?}", self.0)
    }
}
impl ::std::cmp::PartialEq for D3D11_COUNTER {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::std::cmp::Eq for D3D11_COUNTER {}
impl ::std::marker::Copy for D3D11_COUNTER {}
impl D3D11_COUNTER {
    #![allow(non_upper_case_globals)]
    pub const D3D11_COUNTER_DEVICE_DEPENDENT_0: Self = Self(1073741824i32);
}
unsafe impl ::windows::Abi for D3D11_COUNTER {
    type Abi = Self;
}
#[repr(C)]
#[allow(non_camel_case_types)]
pub struct D3D11_COUNTER_DESC {
    pub counter: D3D11_COUNTER,
    pub misc_flags: u32,
}
#[repr(C)]
#[doc(hidden)]
#[allow(non_camel_case_types)]
pub struct D3D11_COUNTER_DESC_abi(D3D11_COUNTER, u32);
impl D3D11_COUNTER_DESC {}
unsafe impl ::windows::Abi for D3D11_COUNTER_DESC {
    type Abi = D3D11_COUNTER_DESC_abi;
}
impl ::core::default::Default for D3D11_COUNTER_DESC {
    fn default() -> Self {
        Self {
            counter: ::std::default::Default::default(),
            misc_flags: 0,
        }
    }
}
impl ::core::fmt::Debug for D3D11_COUNTER_DESC {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("D3D11_COUNTER_DESC")
            .field("counter", &format_args!("{:?}", self.counter))
            .field("misc_flags", &format_args!("{:?}", self.misc_flags))
            .finish()
    }
}
impl ::core::clone::Clone for D3D11_COUNTER_DESC {
    fn clone(&self) -> Self {
        Self {
            counter: self.counter,
            misc_flags: self.misc_flags,
        }
    }
}
#[repr(transparent)]
#[allow(non_camel_case_types)]
pub struct ID3D11Counter(::windows::IUnknown);
impl ::std::clone::Clone for ID3D11Counter {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::std::fmt::Debug for ID3D11Counter {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        write!(f, "{:?}", self.0)
    }
}
impl ::std::cmp::PartialEq for ID3D11Counter {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::std::cmp::Eq for ID3D11Counter {}
unsafe impl ::windows::Interface for ID3D11Counter {
    type Vtable = ID3D11Counter_abi;
    const IID: ::windows::Guid =
        ::windows::Guid::from_values(1854687739, 41841, 18288, [180, 64, 41, 8, 96, 34, 183, 65]);
}
#[repr(C)]
pub struct ID3D11Counter_abi(
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        iid: &::windows::Guid,
        interface: *mut ::windows::RawPtr,
    ) -> ::windows::ErrorCode,
    pub unsafe extern "system" fn(this: ::windows::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::RawPtr) -> u32,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        pp_device: *mut ::std::option::Option<ID3D11Device>,
    ),
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
#[allow(non_snake_case)]
impl ID3D11Counter {
    pub unsafe fn GetDevice(&self, pp_device: *mut ::std::option::Option<ID3D11Device>) {
        (::windows::Interface::vtable(self).3)(::windows::Abi::abi(self), pp_device)
    }
    pub unsafe fn GetPrivateData(
        &self,
        guid: *const ::windows::Guid,
        p_data_size: *mut u32,
        p_data: *mut ::std::ffi::c_void,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).4)(::windows::Abi::abi(self), guid, p_data_size, p_data)
    }
    pub unsafe fn SetPrivateData(
        &self,
        guid: *const ::windows::Guid,
        data_size: u32,
        p_data: *const ::std::ffi::c_void,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).5)(::windows::Abi::abi(self), guid, data_size, p_data)
    }
    pub unsafe fn SetPrivateDataInterface<
        'a,
        T1__: ::std::convert::Into<::windows::Param<'a, ::windows::IUnknown>>,
    >(
        &self,
        guid: *const ::windows::Guid,
        p_data: T1__,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).6)(::windows::Abi::abi(self), guid, p_data.into().abi())
    }
    pub unsafe fn GetDataSize(&self) -> u32 {
        (::windows::Interface::vtable(self).7)(::windows::Abi::abi(self))
    }
    pub unsafe fn GetDesc(&self, p_desc: *mut D3D11_COUNTER_DESC) {
        (::windows::Interface::vtable(self).8)(::windows::Abi::abi(self), p_desc)
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
impl<'a> ::std::convert::Into<::windows::Param<'a, ::windows::IUnknown>> for ID3D11Counter {
    fn into(self) -> ::windows::Param<'a, ::windows::IUnknown> {
        ::windows::Param::Owned(::std::convert::Into::<::windows::IUnknown>::into(self))
    }
}
impl<'a> ::std::convert::Into<::windows::Param<'a, ::windows::IUnknown>> for &'a ID3D11Counter {
    fn into(self) -> ::windows::Param<'a, ::windows::IUnknown> {
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
impl<'a> ::std::convert::Into<::windows::Param<'a, ID3D11Asynchronous>> for ID3D11Counter {
    fn into(self) -> ::windows::Param<'a, ID3D11Asynchronous> {
        ::windows::Param::Owned(::std::convert::Into::<ID3D11Asynchronous>::into(self))
    }
}
impl<'a> ::std::convert::Into<::windows::Param<'a, ID3D11Asynchronous>> for &'a ID3D11Counter {
    fn into(self) -> ::windows::Param<'a, ID3D11Asynchronous> {
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
impl<'a> ::std::convert::Into<::windows::Param<'a, ID3D11DeviceChild>> for ID3D11Counter {
    fn into(self) -> ::windows::Param<'a, ID3D11DeviceChild> {
        ::windows::Param::Owned(::std::convert::Into::<ID3D11DeviceChild>::into(self))
    }
}
impl<'a> ::std::convert::Into<::windows::Param<'a, ID3D11DeviceChild>> for &'a ID3D11Counter {
    fn into(self) -> ::windows::Param<'a, ID3D11DeviceChild> {
        ::windows::Param::Owned(::std::convert::Into::<ID3D11DeviceChild>::into(
            ::std::clone::Clone::clone(self),
        ))
    }
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct D3D11_MAP(pub i32);
impl ::std::convert::From<i32> for D3D11_MAP {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
impl ::std::clone::Clone for D3D11_MAP {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}
impl ::std::default::Default for D3D11_MAP {
    fn default() -> Self {
        Self(0)
    }
}
impl ::std::fmt::Debug for D3D11_MAP {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        write!(f, "{:?}", self.0)
    }
}
impl ::std::cmp::PartialEq for D3D11_MAP {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::std::cmp::Eq for D3D11_MAP {}
impl ::std::marker::Copy for D3D11_MAP {}
impl D3D11_MAP {
    #![allow(non_upper_case_globals)]
    pub const D3D11_MAP_READ: Self = Self(1i32);
    pub const D3D11_MAP_WRITE: Self = Self(2i32);
    pub const D3D11_MAP_READ_WRITE: Self = Self(3i32);
    pub const D3D11_MAP_WRITE_DISCARD: Self = Self(4i32);
    pub const D3D11_MAP_WRITE_NO_OVERWRITE: Self = Self(5i32);
}
unsafe impl ::windows::Abi for D3D11_MAP {
    type Abi = Self;
}
#[repr(C)]
#[allow(non_camel_case_types)]
pub struct D3D11_MAPPED_SUBRESOURCE {
    pub p_data: *mut ::std::ffi::c_void,
    pub row_pitch: u32,
    pub depth_pitch: u32,
}
#[repr(C)]
#[doc(hidden)]
#[allow(non_camel_case_types)]
pub struct D3D11_MAPPED_SUBRESOURCE_abi(*mut ::std::ffi::c_void, u32, u32);
impl D3D11_MAPPED_SUBRESOURCE {}
unsafe impl ::windows::Abi for D3D11_MAPPED_SUBRESOURCE {
    type Abi = D3D11_MAPPED_SUBRESOURCE_abi;
}
impl ::core::default::Default for D3D11_MAPPED_SUBRESOURCE {
    fn default() -> Self {
        Self {
            p_data: ::std::ptr::null_mut(),
            row_pitch: 0,
            depth_pitch: 0,
        }
    }
}
impl ::core::fmt::Debug for D3D11_MAPPED_SUBRESOURCE {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("D3D11_MAPPED_SUBRESOURCE")
            .field("p_data", &format_args!("{:?}", self.p_data))
            .field("row_pitch", &format_args!("{:?}", self.row_pitch))
            .field("depth_pitch", &format_args!("{:?}", self.depth_pitch))
            .finish()
    }
}
impl ::core::clone::Clone for D3D11_MAPPED_SUBRESOURCE {
    fn clone(&self) -> Self {
        Self {
            p_data: <*mut ::std::ffi::c_void as std::clone::Clone>::clone(&self.p_data),
            row_pitch: self.row_pitch,
            depth_pitch: self.depth_pitch,
        }
    }
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct D3D_PRIMITIVE_TOPOLOGY(pub i32);
impl ::std::convert::From<i32> for D3D_PRIMITIVE_TOPOLOGY {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
impl ::std::clone::Clone for D3D_PRIMITIVE_TOPOLOGY {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}
impl ::std::default::Default for D3D_PRIMITIVE_TOPOLOGY {
    fn default() -> Self {
        Self(0)
    }
}
impl ::std::fmt::Debug for D3D_PRIMITIVE_TOPOLOGY {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        write!(f, "{:?}", self.0)
    }
}
impl ::std::cmp::PartialEq for D3D_PRIMITIVE_TOPOLOGY {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::std::cmp::Eq for D3D_PRIMITIVE_TOPOLOGY {}
impl ::std::marker::Copy for D3D_PRIMITIVE_TOPOLOGY {}
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
unsafe impl ::windows::Abi for D3D_PRIMITIVE_TOPOLOGY {
    type Abi = Self;
}
#[repr(C)]
#[allow(non_camel_case_types)]
pub struct D3D11_VIEWPORT {
    pub top_leftx: f32,
    pub top_lefty: f32,
    pub width: f32,
    pub height: f32,
    pub min_depth: f32,
    pub max_depth: f32,
}
#[repr(C)]
#[doc(hidden)]
#[allow(non_camel_case_types)]
pub struct D3D11_VIEWPORT_abi(f32, f32, f32, f32, f32, f32);
impl D3D11_VIEWPORT {}
unsafe impl ::windows::Abi for D3D11_VIEWPORT {
    type Abi = D3D11_VIEWPORT_abi;
}
impl ::core::default::Default for D3D11_VIEWPORT {
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
impl ::core::fmt::Debug for D3D11_VIEWPORT {
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
impl ::core::clone::Clone for D3D11_VIEWPORT {
    fn clone(&self) -> Self {
        Self {
            top_leftx: self.top_leftx,
            top_lefty: self.top_lefty,
            width: self.width,
            height: self.height,
            min_depth: self.min_depth,
            max_depth: self.max_depth,
        }
    }
}
impl ::std::marker::Copy for D3D11_VIEWPORT {}
#[repr(C)]
#[allow(non_camel_case_types)]
pub struct D3D11_BOX {
    pub left: u32,
    pub top: u32,
    pub front: u32,
    pub right: u32,
    pub bottom: u32,
    pub back: u32,
}
#[repr(C)]
#[doc(hidden)]
#[allow(non_camel_case_types)]
pub struct D3D11_BOX_abi(u32, u32, u32, u32, u32, u32);
impl D3D11_BOX {}
unsafe impl ::windows::Abi for D3D11_BOX {
    type Abi = D3D11_BOX_abi;
}
impl ::core::default::Default for D3D11_BOX {
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
impl ::core::fmt::Debug for D3D11_BOX {
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
impl ::core::clone::Clone for D3D11_BOX {
    fn clone(&self) -> Self {
        Self {
            left: self.left,
            top: self.top,
            front: self.front,
            right: self.right,
            bottom: self.bottom,
            back: self.back,
        }
    }
}
impl ::std::marker::Copy for D3D11_BOX {}
#[repr(transparent)]
#[allow(non_camel_case_types)]
pub struct ID3D11CommandList(::windows::IUnknown);
impl ::std::clone::Clone for ID3D11CommandList {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::std::fmt::Debug for ID3D11CommandList {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        write!(f, "{:?}", self.0)
    }
}
impl ::std::cmp::PartialEq for ID3D11CommandList {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::std::cmp::Eq for ID3D11CommandList {}
unsafe impl ::windows::Interface for ID3D11CommandList {
    type Vtable = ID3D11CommandList_abi;
    const IID: ::windows::Guid = ::windows::Guid::from_values(
        2722874577,
        30366,
        17399,
        [128, 19, 152, 255, 86, 108, 24, 226],
    );
}
#[repr(C)]
pub struct ID3D11CommandList_abi(
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        iid: &::windows::Guid,
        interface: *mut ::windows::RawPtr,
    ) -> ::windows::ErrorCode,
    pub unsafe extern "system" fn(this: ::windows::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::RawPtr) -> u32,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        pp_device: *mut ::std::option::Option<ID3D11Device>,
    ),
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
#[allow(non_snake_case)]
impl ID3D11CommandList {
    pub unsafe fn GetDevice(&self, pp_device: *mut ::std::option::Option<ID3D11Device>) {
        (::windows::Interface::vtable(self).3)(::windows::Abi::abi(self), pp_device)
    }
    pub unsafe fn GetPrivateData(
        &self,
        guid: *const ::windows::Guid,
        p_data_size: *mut u32,
        p_data: *mut ::std::ffi::c_void,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).4)(::windows::Abi::abi(self), guid, p_data_size, p_data)
    }
    pub unsafe fn SetPrivateData(
        &self,
        guid: *const ::windows::Guid,
        data_size: u32,
        p_data: *const ::std::ffi::c_void,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).5)(::windows::Abi::abi(self), guid, data_size, p_data)
    }
    pub unsafe fn SetPrivateDataInterface<
        'a,
        T1__: ::std::convert::Into<::windows::Param<'a, ::windows::IUnknown>>,
    >(
        &self,
        guid: *const ::windows::Guid,
        p_data: T1__,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).6)(::windows::Abi::abi(self), guid, p_data.into().abi())
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
impl<'a> ::std::convert::Into<::windows::Param<'a, ::windows::IUnknown>> for ID3D11CommandList {
    fn into(self) -> ::windows::Param<'a, ::windows::IUnknown> {
        ::windows::Param::Owned(::std::convert::Into::<::windows::IUnknown>::into(self))
    }
}
impl<'a> ::std::convert::Into<::windows::Param<'a, ::windows::IUnknown>> for &'a ID3D11CommandList {
    fn into(self) -> ::windows::Param<'a, ::windows::IUnknown> {
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
impl<'a> ::std::convert::Into<::windows::Param<'a, ID3D11DeviceChild>> for ID3D11CommandList {
    fn into(self) -> ::windows::Param<'a, ID3D11DeviceChild> {
        ::windows::Param::Owned(::std::convert::Into::<ID3D11DeviceChild>::into(self))
    }
}
impl<'a> ::std::convert::Into<::windows::Param<'a, ID3D11DeviceChild>> for &'a ID3D11CommandList {
    fn into(self) -> ::windows::Param<'a, ID3D11DeviceChild> {
        ::windows::Param::Owned(::std::convert::Into::<ID3D11DeviceChild>::into(
            ::std::clone::Clone::clone(self),
        ))
    }
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct D3D11_DEVICE_CONTEXT_TYPE(pub i32);
impl ::std::convert::From<i32> for D3D11_DEVICE_CONTEXT_TYPE {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
impl ::std::clone::Clone for D3D11_DEVICE_CONTEXT_TYPE {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}
impl ::std::default::Default for D3D11_DEVICE_CONTEXT_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::std::fmt::Debug for D3D11_DEVICE_CONTEXT_TYPE {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        write!(f, "{:?}", self.0)
    }
}
impl ::std::cmp::PartialEq for D3D11_DEVICE_CONTEXT_TYPE {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::std::cmp::Eq for D3D11_DEVICE_CONTEXT_TYPE {}
impl ::std::marker::Copy for D3D11_DEVICE_CONTEXT_TYPE {}
impl D3D11_DEVICE_CONTEXT_TYPE {
    #![allow(non_upper_case_globals)]
    pub const D3D11_DEVICE_CONTEXT_IMMEDIATE: Self = Self(0i32);
    pub const D3D11_DEVICE_CONTEXT_DEFERRED: Self = Self(1i32);
}
unsafe impl ::windows::Abi for D3D11_DEVICE_CONTEXT_TYPE {
    type Abi = Self;
}
#[repr(transparent)]
#[allow(non_camel_case_types)]
pub struct ID3D11DeviceContext(::windows::IUnknown);
impl ::std::clone::Clone for ID3D11DeviceContext {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::std::fmt::Debug for ID3D11DeviceContext {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        write!(f, "{:?}", self.0)
    }
}
impl ::std::cmp::PartialEq for ID3D11DeviceContext {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::std::cmp::Eq for ID3D11DeviceContext {}
unsafe impl ::windows::Interface for ID3D11DeviceContext {
    type Vtable = ID3D11DeviceContext_abi;
    const IID: ::windows::Guid = ::windows::Guid::from_values(
        3233786220,
        57481,
        17659,
        [142, 175, 38, 248, 121, 97, 144, 218],
    );
}
#[repr(C)]
pub struct ID3D11DeviceContext_abi(
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        iid: &::windows::Guid,
        interface: *mut ::windows::RawPtr,
    ) -> ::windows::ErrorCode,
    pub unsafe extern "system" fn(this: ::windows::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::RawPtr) -> u32,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        pp_device: *mut ::std::option::Option<ID3D11Device>,
    ),
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
        pp_constant_buffers: ::windows::RawPtr,
    ),
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        start_slot: u32,
        num_views: u32,
        pp_shader_resource_views: ::windows::RawPtr,
    ),
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        p_pixel_shader: ::windows::RawPtr,
        pp_class_instances: ::windows::RawPtr,
        num_class_instances: u32,
    ),
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        start_slot: u32,
        num_samplers: u32,
        pp_samplers: ::windows::RawPtr,
    ),
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        p_vertex_shader: ::windows::RawPtr,
        pp_class_instances: ::windows::RawPtr,
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
        pp_constant_buffers: ::windows::RawPtr,
    ),
    pub unsafe extern "system" fn(this: ::windows::RawPtr, p_input_layout: ::windows::RawPtr),
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        start_slot: u32,
        num_buffers: u32,
        pp_vertex_buffers: ::windows::RawPtr,
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
        pp_constant_buffers: ::windows::RawPtr,
    ),
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        p_shader: ::windows::RawPtr,
        pp_class_instances: ::windows::RawPtr,
        num_class_instances: u32,
    ),
    pub unsafe extern "system" fn(this: ::windows::RawPtr, topology: D3D_PRIMITIVE_TOPOLOGY),
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        start_slot: u32,
        num_views: u32,
        pp_shader_resource_views: ::windows::RawPtr,
    ),
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        start_slot: u32,
        num_samplers: u32,
        pp_samplers: ::windows::RawPtr,
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
        predicate_value: ::windows::BOOL,
    ),
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        start_slot: u32,
        num_views: u32,
        pp_shader_resource_views: ::windows::RawPtr,
    ),
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        start_slot: u32,
        num_samplers: u32,
        pp_samplers: ::windows::RawPtr,
    ),
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        num_views: u32,
        pp_render_target_views: ::windows::RawPtr,
        p_depth_stencil_view: ::windows::RawPtr,
    ),
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        num_rt_vs: u32,
        pp_render_target_views: ::windows::RawPtr,
        p_depth_stencil_view: ::windows::RawPtr,
        uav_start_slot: u32,
        num_ua_vs: u32,
        pp_unordered_access_views: ::windows::RawPtr,
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
        pp_so_targets: ::windows::RawPtr,
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
        restore_context_state: ::windows::BOOL,
    ),
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        start_slot: u32,
        num_views: u32,
        pp_shader_resource_views: ::windows::RawPtr,
    ),
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        p_hull_shader: ::windows::RawPtr,
        pp_class_instances: ::windows::RawPtr,
        num_class_instances: u32,
    ),
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        start_slot: u32,
        num_samplers: u32,
        pp_samplers: ::windows::RawPtr,
    ),
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        start_slot: u32,
        num_buffers: u32,
        pp_constant_buffers: ::windows::RawPtr,
    ),
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        start_slot: u32,
        num_views: u32,
        pp_shader_resource_views: ::windows::RawPtr,
    ),
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        p_domain_shader: ::windows::RawPtr,
        pp_class_instances: ::windows::RawPtr,
        num_class_instances: u32,
    ),
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        start_slot: u32,
        num_samplers: u32,
        pp_samplers: ::windows::RawPtr,
    ),
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        start_slot: u32,
        num_buffers: u32,
        pp_constant_buffers: ::windows::RawPtr,
    ),
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        start_slot: u32,
        num_views: u32,
        pp_shader_resource_views: ::windows::RawPtr,
    ),
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        start_slot: u32,
        num_ua_vs: u32,
        pp_unordered_access_views: ::windows::RawPtr,
        p_uav_initial_counts: *const u32,
    ),
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        p_compute_shader: ::windows::RawPtr,
        pp_class_instances: ::windows::RawPtr,
        num_class_instances: u32,
    ),
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        start_slot: u32,
        num_samplers: u32,
        pp_samplers: ::windows::RawPtr,
    ),
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        start_slot: u32,
        num_buffers: u32,
        pp_constant_buffers: ::windows::RawPtr,
    ),
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        start_slot: u32,
        num_buffers: u32,
        pp_constant_buffers: *mut ::std::option::Option<ID3D11Buffer>,
    ),
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        start_slot: u32,
        num_views: u32,
        pp_shader_resource_views: *mut ::std::option::Option<ID3D11ShaderResourceView>,
    ),
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        pp_pixel_shader: *mut ::std::option::Option<ID3D11PixelShader>,
        pp_class_instances: *mut ::std::option::Option<ID3D11ClassInstance>,
        p_num_class_instances: *mut u32,
    ),
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        start_slot: u32,
        num_samplers: u32,
        pp_samplers: *mut ::std::option::Option<ID3D11SamplerState>,
    ),
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        pp_vertex_shader: *mut ::std::option::Option<ID3D11VertexShader>,
        pp_class_instances: *mut ::std::option::Option<ID3D11ClassInstance>,
        p_num_class_instances: *mut u32,
    ),
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        start_slot: u32,
        num_buffers: u32,
        pp_constant_buffers: *mut ::std::option::Option<ID3D11Buffer>,
    ),
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        pp_input_layout: *mut ::std::option::Option<ID3D11InputLayout>,
    ),
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        start_slot: u32,
        num_buffers: u32,
        pp_vertex_buffers: *mut ::std::option::Option<ID3D11Buffer>,
        p_strides: *mut u32,
        p_offsets: *mut u32,
    ),
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        p_index_buffer: *mut ::std::option::Option<ID3D11Buffer>,
        format: *mut super::dxgi::DXGI_FORMAT,
        offset: *mut u32,
    ),
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        start_slot: u32,
        num_buffers: u32,
        pp_constant_buffers: *mut ::std::option::Option<ID3D11Buffer>,
    ),
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        pp_geometry_shader: *mut ::std::option::Option<ID3D11GeometryShader>,
        pp_class_instances: *mut ::std::option::Option<ID3D11ClassInstance>,
        p_num_class_instances: *mut u32,
    ),
    pub unsafe extern "system" fn(this: ::windows::RawPtr, p_topology: *mut D3D_PRIMITIVE_TOPOLOGY),
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        start_slot: u32,
        num_views: u32,
        pp_shader_resource_views: *mut ::std::option::Option<ID3D11ShaderResourceView>,
    ),
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        start_slot: u32,
        num_samplers: u32,
        pp_samplers: *mut ::std::option::Option<ID3D11SamplerState>,
    ),
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        pp_predicate: *mut ::std::option::Option<ID3D11Predicate>,
        p_predicate_value: *mut i32,
    ),
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        start_slot: u32,
        num_views: u32,
        pp_shader_resource_views: *mut ::std::option::Option<ID3D11ShaderResourceView>,
    ),
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        start_slot: u32,
        num_samplers: u32,
        pp_samplers: *mut ::std::option::Option<ID3D11SamplerState>,
    ),
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        num_views: u32,
        pp_render_target_views: *mut ::std::option::Option<ID3D11RenderTargetView>,
        pp_depth_stencil_view: *mut ::std::option::Option<ID3D11DepthStencilView>,
    ),
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        num_rt_vs: u32,
        pp_render_target_views: *mut ::std::option::Option<ID3D11RenderTargetView>,
        pp_depth_stencil_view: *mut ::std::option::Option<ID3D11DepthStencilView>,
        uav_start_slot: u32,
        num_ua_vs: u32,
        pp_unordered_access_views: *mut ::std::option::Option<ID3D11UnorderedAccessView>,
    ),
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        pp_blend_state: *mut ::std::option::Option<ID3D11BlendState>,
        blend_factor: *mut f32,
        p_sample_mask: *mut u32,
    ),
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        pp_depth_stencil_state: *mut ::std::option::Option<ID3D11DepthStencilState>,
        p_stencil_ref: *mut u32,
    ),
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        num_buffers: u32,
        pp_so_targets: *mut ::std::option::Option<ID3D11Buffer>,
    ),
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        pp_rasterizer_state: *mut ::std::option::Option<ID3D11RasterizerState>,
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
        pp_shader_resource_views: *mut ::std::option::Option<ID3D11ShaderResourceView>,
    ),
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        pp_hull_shader: *mut ::std::option::Option<ID3D11HullShader>,
        pp_class_instances: *mut ::std::option::Option<ID3D11ClassInstance>,
        p_num_class_instances: *mut u32,
    ),
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        start_slot: u32,
        num_samplers: u32,
        pp_samplers: *mut ::std::option::Option<ID3D11SamplerState>,
    ),
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        start_slot: u32,
        num_buffers: u32,
        pp_constant_buffers: *mut ::std::option::Option<ID3D11Buffer>,
    ),
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        start_slot: u32,
        num_views: u32,
        pp_shader_resource_views: *mut ::std::option::Option<ID3D11ShaderResourceView>,
    ),
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        pp_domain_shader: *mut ::std::option::Option<ID3D11DomainShader>,
        pp_class_instances: *mut ::std::option::Option<ID3D11ClassInstance>,
        p_num_class_instances: *mut u32,
    ),
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        start_slot: u32,
        num_samplers: u32,
        pp_samplers: *mut ::std::option::Option<ID3D11SamplerState>,
    ),
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        start_slot: u32,
        num_buffers: u32,
        pp_constant_buffers: *mut ::std::option::Option<ID3D11Buffer>,
    ),
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        start_slot: u32,
        num_views: u32,
        pp_shader_resource_views: *mut ::std::option::Option<ID3D11ShaderResourceView>,
    ),
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        start_slot: u32,
        num_ua_vs: u32,
        pp_unordered_access_views: *mut ::std::option::Option<ID3D11UnorderedAccessView>,
    ),
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        pp_compute_shader: *mut ::std::option::Option<ID3D11ComputeShader>,
        pp_class_instances: *mut ::std::option::Option<ID3D11ClassInstance>,
        p_num_class_instances: *mut u32,
    ),
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        start_slot: u32,
        num_samplers: u32,
        pp_samplers: *mut ::std::option::Option<ID3D11SamplerState>,
    ),
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        start_slot: u32,
        num_buffers: u32,
        pp_constant_buffers: *mut ::std::option::Option<ID3D11Buffer>,
    ),
    pub unsafe extern "system" fn(this: ::windows::RawPtr),
    pub unsafe extern "system" fn(this: ::windows::RawPtr),
    pub unsafe extern "system" fn(this: ::windows::RawPtr) -> D3D11_DEVICE_CONTEXT_TYPE,
    pub unsafe extern "system" fn(this: ::windows::RawPtr) -> u32,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        restore_deferred_context_state: ::windows::BOOL,
        pp_command_list: *mut ::std::option::Option<ID3D11CommandList>,
    ) -> ::windows::ErrorCode,
);
#[allow(non_snake_case)]
impl ID3D11DeviceContext {
    pub unsafe fn GetDevice(&self, pp_device: *mut ::std::option::Option<ID3D11Device>) {
        (::windows::Interface::vtable(self).3)(::windows::Abi::abi(self), pp_device)
    }
    pub unsafe fn GetPrivateData(
        &self,
        guid: *const ::windows::Guid,
        p_data_size: *mut u32,
        p_data: *mut ::std::ffi::c_void,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).4)(::windows::Abi::abi(self), guid, p_data_size, p_data)
    }
    pub unsafe fn SetPrivateData(
        &self,
        guid: *const ::windows::Guid,
        data_size: u32,
        p_data: *const ::std::ffi::c_void,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).5)(::windows::Abi::abi(self), guid, data_size, p_data)
    }
    pub unsafe fn SetPrivateDataInterface<
        'a,
        T1__: ::std::convert::Into<::windows::Param<'a, ::windows::IUnknown>>,
    >(
        &self,
        guid: *const ::windows::Guid,
        p_data: T1__,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).6)(::windows::Abi::abi(self), guid, p_data.into().abi())
    }
    pub unsafe fn VSSetConstantBuffers<
        'a,
        T2__: ::std::convert::Into<::windows::Param<'a, ID3D11Buffer>>,
    >(
        &self,
        start_slot: u32,
        num_buffers: u32,
        pp_constant_buffers: T2__,
    ) {
        (::windows::Interface::vtable(self).7)(
            ::windows::Abi::abi(self),
            start_slot,
            num_buffers,
            pp_constant_buffers.into().abi(),
        )
    }
    pub unsafe fn PSSetShaderResources<
        'a,
        T2__: ::std::convert::Into<::windows::Param<'a, ID3D11ShaderResourceView>>,
    >(
        &self,
        start_slot: u32,
        num_views: u32,
        pp_shader_resource_views: T2__,
    ) {
        (::windows::Interface::vtable(self).8)(
            ::windows::Abi::abi(self),
            start_slot,
            num_views,
            pp_shader_resource_views.into().abi(),
        )
    }
    pub unsafe fn PSSetShader<
        'a,
        T0__: ::std::convert::Into<::windows::Param<'a, ID3D11PixelShader>>,
        T1__: ::std::convert::Into<::windows::Param<'a, ID3D11ClassInstance>>,
    >(
        &self,
        p_pixel_shader: T0__,
        pp_class_instances: T1__,
        num_class_instances: u32,
    ) {
        (::windows::Interface::vtable(self).9)(
            ::windows::Abi::abi(self),
            p_pixel_shader.into().abi(),
            pp_class_instances.into().abi(),
            num_class_instances,
        )
    }
    pub unsafe fn PSSetSamplers<
        'a,
        T2__: ::std::convert::Into<::windows::Param<'a, ID3D11SamplerState>>,
    >(
        &self,
        start_slot: u32,
        num_samplers: u32,
        pp_samplers: T2__,
    ) {
        (::windows::Interface::vtable(self).10)(
            ::windows::Abi::abi(self),
            start_slot,
            num_samplers,
            pp_samplers.into().abi(),
        )
    }
    pub unsafe fn VSSetShader<
        'a,
        T0__: ::std::convert::Into<::windows::Param<'a, ID3D11VertexShader>>,
        T1__: ::std::convert::Into<::windows::Param<'a, ID3D11ClassInstance>>,
    >(
        &self,
        p_vertex_shader: T0__,
        pp_class_instances: T1__,
        num_class_instances: u32,
    ) {
        (::windows::Interface::vtable(self).11)(
            ::windows::Abi::abi(self),
            p_vertex_shader.into().abi(),
            pp_class_instances.into().abi(),
            num_class_instances,
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
            index_count,
            start_index_location,
            base_vertex_location,
        )
    }
    pub unsafe fn Draw(&self, vertex_count: u32, start_vertex_location: u32) {
        (::windows::Interface::vtable(self).13)(
            ::windows::Abi::abi(self),
            vertex_count,
            start_vertex_location,
        )
    }
    pub unsafe fn Map<'a, T0__: ::std::convert::Into<::windows::Param<'a, ID3D11Resource>>>(
        &self,
        p_resource: T0__,
        subresource: u32,
        map_type: D3D11_MAP,
        map_flags: u32,
        p_mapped_resource: *mut D3D11_MAPPED_SUBRESOURCE,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).14)(
            ::windows::Abi::abi(self),
            p_resource.into().abi(),
            subresource,
            map_type,
            map_flags,
            p_mapped_resource,
        )
    }
    pub unsafe fn Unmap<'a, T0__: ::std::convert::Into<::windows::Param<'a, ID3D11Resource>>>(
        &self,
        p_resource: T0__,
        subresource: u32,
    ) {
        (::windows::Interface::vtable(self).15)(
            ::windows::Abi::abi(self),
            p_resource.into().abi(),
            subresource,
        )
    }
    pub unsafe fn PSSetConstantBuffers<
        'a,
        T2__: ::std::convert::Into<::windows::Param<'a, ID3D11Buffer>>,
    >(
        &self,
        start_slot: u32,
        num_buffers: u32,
        pp_constant_buffers: T2__,
    ) {
        (::windows::Interface::vtable(self).16)(
            ::windows::Abi::abi(self),
            start_slot,
            num_buffers,
            pp_constant_buffers.into().abi(),
        )
    }
    pub unsafe fn IASetInputLayout<
        'a,
        T0__: ::std::convert::Into<::windows::Param<'a, ID3D11InputLayout>>,
    >(
        &self,
        p_input_layout: T0__,
    ) {
        (::windows::Interface::vtable(self).17)(
            ::windows::Abi::abi(self),
            p_input_layout.into().abi(),
        )
    }
    pub unsafe fn IASetVertexBuffers<
        'a,
        T2__: ::std::convert::Into<::windows::Param<'a, ID3D11Buffer>>,
    >(
        &self,
        start_slot: u32,
        num_buffers: u32,
        pp_vertex_buffers: T2__,
        p_strides: *const u32,
        p_offsets: *const u32,
    ) {
        (::windows::Interface::vtable(self).18)(
            ::windows::Abi::abi(self),
            start_slot,
            num_buffers,
            pp_vertex_buffers.into().abi(),
            p_strides,
            p_offsets,
        )
    }
    pub unsafe fn IASetIndexBuffer<
        'a,
        T0__: ::std::convert::Into<::windows::Param<'a, ID3D11Buffer>>,
    >(
        &self,
        p_index_buffer: T0__,
        format: super::dxgi::DXGI_FORMAT,
        offset: u32,
    ) {
        (::windows::Interface::vtable(self).19)(
            ::windows::Abi::abi(self),
            p_index_buffer.into().abi(),
            format,
            offset,
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
            index_count_per_instance,
            instance_count,
            start_index_location,
            base_vertex_location,
            start_instance_location,
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
            vertex_count_per_instance,
            instance_count,
            start_vertex_location,
            start_instance_location,
        )
    }
    pub unsafe fn GSSetConstantBuffers<
        'a,
        T2__: ::std::convert::Into<::windows::Param<'a, ID3D11Buffer>>,
    >(
        &self,
        start_slot: u32,
        num_buffers: u32,
        pp_constant_buffers: T2__,
    ) {
        (::windows::Interface::vtable(self).22)(
            ::windows::Abi::abi(self),
            start_slot,
            num_buffers,
            pp_constant_buffers.into().abi(),
        )
    }
    pub unsafe fn GSSetShader<
        'a,
        T0__: ::std::convert::Into<::windows::Param<'a, ID3D11GeometryShader>>,
        T1__: ::std::convert::Into<::windows::Param<'a, ID3D11ClassInstance>>,
    >(
        &self,
        p_shader: T0__,
        pp_class_instances: T1__,
        num_class_instances: u32,
    ) {
        (::windows::Interface::vtable(self).23)(
            ::windows::Abi::abi(self),
            p_shader.into().abi(),
            pp_class_instances.into().abi(),
            num_class_instances,
        )
    }
    pub unsafe fn IASetPrimitiveTopology(&self, topology: D3D_PRIMITIVE_TOPOLOGY) {
        (::windows::Interface::vtable(self).24)(::windows::Abi::abi(self), topology)
    }
    pub unsafe fn VSSetShaderResources<
        'a,
        T2__: ::std::convert::Into<::windows::Param<'a, ID3D11ShaderResourceView>>,
    >(
        &self,
        start_slot: u32,
        num_views: u32,
        pp_shader_resource_views: T2__,
    ) {
        (::windows::Interface::vtable(self).25)(
            ::windows::Abi::abi(self),
            start_slot,
            num_views,
            pp_shader_resource_views.into().abi(),
        )
    }
    pub unsafe fn VSSetSamplers<
        'a,
        T2__: ::std::convert::Into<::windows::Param<'a, ID3D11SamplerState>>,
    >(
        &self,
        start_slot: u32,
        num_samplers: u32,
        pp_samplers: T2__,
    ) {
        (::windows::Interface::vtable(self).26)(
            ::windows::Abi::abi(self),
            start_slot,
            num_samplers,
            pp_samplers.into().abi(),
        )
    }
    pub unsafe fn Begin<
        'a,
        T0__: ::std::convert::Into<::windows::Param<'a, ID3D11Asynchronous>>,
    >(
        &self,
        p_async: T0__,
    ) {
        (::windows::Interface::vtable(self).27)(::windows::Abi::abi(self), p_async.into().abi())
    }
    pub unsafe fn End<'a, T0__: ::std::convert::Into<::windows::Param<'a, ID3D11Asynchronous>>>(
        &self,
        p_async: T0__,
    ) {
        (::windows::Interface::vtable(self).28)(::windows::Abi::abi(self), p_async.into().abi())
    }
    pub unsafe fn GetData<
        'a,
        T0__: ::std::convert::Into<::windows::Param<'a, ID3D11Asynchronous>>,
    >(
        &self,
        p_async: T0__,
        p_data: *mut ::std::ffi::c_void,
        data_size: u32,
        get_data_flags: u32,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).29)(
            ::windows::Abi::abi(self),
            p_async.into().abi(),
            p_data,
            data_size,
            get_data_flags,
        )
    }
    pub unsafe fn SetPredication<
        'a,
        T0__: ::std::convert::Into<::windows::Param<'a, ID3D11Predicate>>,
    >(
        &self,
        p_predicate: T0__,
        predicate_value: ::windows::BOOL,
    ) {
        (::windows::Interface::vtable(self).30)(
            ::windows::Abi::abi(self),
            p_predicate.into().abi(),
            predicate_value,
        )
    }
    pub unsafe fn GSSetShaderResources<
        'a,
        T2__: ::std::convert::Into<::windows::Param<'a, ID3D11ShaderResourceView>>,
    >(
        &self,
        start_slot: u32,
        num_views: u32,
        pp_shader_resource_views: T2__,
    ) {
        (::windows::Interface::vtable(self).31)(
            ::windows::Abi::abi(self),
            start_slot,
            num_views,
            pp_shader_resource_views.into().abi(),
        )
    }
    pub unsafe fn GSSetSamplers<
        'a,
        T2__: ::std::convert::Into<::windows::Param<'a, ID3D11SamplerState>>,
    >(
        &self,
        start_slot: u32,
        num_samplers: u32,
        pp_samplers: T2__,
    ) {
        (::windows::Interface::vtable(self).32)(
            ::windows::Abi::abi(self),
            start_slot,
            num_samplers,
            pp_samplers.into().abi(),
        )
    }
    pub unsafe fn OMSetRenderTargets<
        'a,
        T1__: ::std::convert::Into<::windows::Param<'a, ID3D11RenderTargetView>>,
        T2__: ::std::convert::Into<::windows::Param<'a, ID3D11DepthStencilView>>,
    >(
        &self,
        num_views: u32,
        pp_render_target_views: T1__,
        p_depth_stencil_view: T2__,
    ) {
        (::windows::Interface::vtable(self).33)(
            ::windows::Abi::abi(self),
            num_views,
            pp_render_target_views.into().abi(),
            p_depth_stencil_view.into().abi(),
        )
    }
    pub unsafe fn OMSetRenderTargetsAndUnorderedAccessViews<
        'a,
        T1__: ::std::convert::Into<::windows::Param<'a, ID3D11RenderTargetView>>,
        T2__: ::std::convert::Into<::windows::Param<'a, ID3D11DepthStencilView>>,
        T5__: ::std::convert::Into<::windows::Param<'a, ID3D11UnorderedAccessView>>,
    >(
        &self,
        num_rt_vs: u32,
        pp_render_target_views: T1__,
        p_depth_stencil_view: T2__,
        uav_start_slot: u32,
        num_ua_vs: u32,
        pp_unordered_access_views: T5__,
        p_uav_initial_counts: *const u32,
    ) {
        (::windows::Interface::vtable(self).34)(
            ::windows::Abi::abi(self),
            num_rt_vs,
            pp_render_target_views.into().abi(),
            p_depth_stencil_view.into().abi(),
            uav_start_slot,
            num_ua_vs,
            pp_unordered_access_views.into().abi(),
            p_uav_initial_counts,
        )
    }
    pub unsafe fn OMSetBlendState<
        'a,
        T0__: ::std::convert::Into<::windows::Param<'a, ID3D11BlendState>>,
    >(
        &self,
        p_blend_state: T0__,
        blend_factor: *const f32,
        sample_mask: u32,
    ) {
        (::windows::Interface::vtable(self).35)(
            ::windows::Abi::abi(self),
            p_blend_state.into().abi(),
            blend_factor,
            sample_mask,
        )
    }
    pub unsafe fn OMSetDepthStencilState<
        'a,
        T0__: ::std::convert::Into<::windows::Param<'a, ID3D11DepthStencilState>>,
    >(
        &self,
        p_depth_stencil_state: T0__,
        stencil_ref: u32,
    ) {
        (::windows::Interface::vtable(self).36)(
            ::windows::Abi::abi(self),
            p_depth_stencil_state.into().abi(),
            stencil_ref,
        )
    }
    pub unsafe fn SOSetTargets<
        'a,
        T1__: ::std::convert::Into<::windows::Param<'a, ID3D11Buffer>>,
    >(
        &self,
        num_buffers: u32,
        pp_so_targets: T1__,
        p_offsets: *const u32,
    ) {
        (::windows::Interface::vtable(self).37)(
            ::windows::Abi::abi(self),
            num_buffers,
            pp_so_targets.into().abi(),
            p_offsets,
        )
    }
    pub unsafe fn DrawAuto(&self) {
        (::windows::Interface::vtable(self).38)(::windows::Abi::abi(self))
    }
    pub unsafe fn DrawIndexedInstancedIndirect<
        'a,
        T0__: ::std::convert::Into<::windows::Param<'a, ID3D11Buffer>>,
    >(
        &self,
        p_buffer_for_args: T0__,
        aligned_byte_offset_for_args: u32,
    ) {
        (::windows::Interface::vtable(self).39)(
            ::windows::Abi::abi(self),
            p_buffer_for_args.into().abi(),
            aligned_byte_offset_for_args,
        )
    }
    pub unsafe fn DrawInstancedIndirect<
        'a,
        T0__: ::std::convert::Into<::windows::Param<'a, ID3D11Buffer>>,
    >(
        &self,
        p_buffer_for_args: T0__,
        aligned_byte_offset_for_args: u32,
    ) {
        (::windows::Interface::vtable(self).40)(
            ::windows::Abi::abi(self),
            p_buffer_for_args.into().abi(),
            aligned_byte_offset_for_args,
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
            thread_group_countx,
            thread_group_county,
            thread_group_countz,
        )
    }
    pub unsafe fn DispatchIndirect<
        'a,
        T0__: ::std::convert::Into<::windows::Param<'a, ID3D11Buffer>>,
    >(
        &self,
        p_buffer_for_args: T0__,
        aligned_byte_offset_for_args: u32,
    ) {
        (::windows::Interface::vtable(self).42)(
            ::windows::Abi::abi(self),
            p_buffer_for_args.into().abi(),
            aligned_byte_offset_for_args,
        )
    }
    pub unsafe fn RSSetState<
        'a,
        T0__: ::std::convert::Into<::windows::Param<'a, ID3D11RasterizerState>>,
    >(
        &self,
        p_rasterizer_state: T0__,
    ) {
        (::windows::Interface::vtable(self).43)(
            ::windows::Abi::abi(self),
            p_rasterizer_state.into().abi(),
        )
    }
    pub unsafe fn RSSetViewports(&self, num_viewports: u32, p_viewports: *const D3D11_VIEWPORT) {
        (::windows::Interface::vtable(self).44)(
            ::windows::Abi::abi(self),
            num_viewports,
            p_viewports,
        )
    }
    pub unsafe fn RSSetScissorRects(
        &self,
        num_rects: u32,
        p_rects: *const super::display_devices::RECT,
    ) {
        (::windows::Interface::vtable(self).45)(::windows::Abi::abi(self), num_rects, p_rects)
    }
    pub unsafe fn CopySubresourceRegion<
        'a,
        T0__: ::std::convert::Into<::windows::Param<'a, ID3D11Resource>>,
        T5__: ::std::convert::Into<::windows::Param<'a, ID3D11Resource>>,
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
            p_dst_resource.into().abi(),
            dst_subresource,
            dstx,
            dsty,
            dstz,
            p_src_resource.into().abi(),
            src_subresource,
            p_src_box,
        )
    }
    pub unsafe fn CopyResource<
        'a,
        T0__: ::std::convert::Into<::windows::Param<'a, ID3D11Resource>>,
        T1__: ::std::convert::Into<::windows::Param<'a, ID3D11Resource>>,
    >(
        &self,
        p_dst_resource: T0__,
        p_src_resource: T1__,
    ) {
        (::windows::Interface::vtable(self).47)(
            ::windows::Abi::abi(self),
            p_dst_resource.into().abi(),
            p_src_resource.into().abi(),
        )
    }
    pub unsafe fn UpdateSubresource<
        'a,
        T0__: ::std::convert::Into<::windows::Param<'a, ID3D11Resource>>,
    >(
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
            p_dst_resource.into().abi(),
            dst_subresource,
            p_dst_box,
            p_src_data,
            src_row_pitch,
            src_depth_pitch,
        )
    }
    pub unsafe fn CopyStructureCount<
        'a,
        T0__: ::std::convert::Into<::windows::Param<'a, ID3D11Buffer>>,
        T2__: ::std::convert::Into<::windows::Param<'a, ID3D11UnorderedAccessView>>,
    >(
        &self,
        p_dst_buffer: T0__,
        dst_aligned_byte_offset: u32,
        p_src_view: T2__,
    ) {
        (::windows::Interface::vtable(self).49)(
            ::windows::Abi::abi(self),
            p_dst_buffer.into().abi(),
            dst_aligned_byte_offset,
            p_src_view.into().abi(),
        )
    }
    pub unsafe fn ClearRenderTargetView<
        'a,
        T0__: ::std::convert::Into<::windows::Param<'a, ID3D11RenderTargetView>>,
    >(
        &self,
        p_render_target_view: T0__,
        color_rgba: *const f32,
    ) {
        (::windows::Interface::vtable(self).50)(
            ::windows::Abi::abi(self),
            p_render_target_view.into().abi(),
            color_rgba,
        )
    }
    pub unsafe fn ClearUnorderedAccessViewUint<
        'a,
        T0__: ::std::convert::Into<::windows::Param<'a, ID3D11UnorderedAccessView>>,
    >(
        &self,
        p_unordered_access_view: T0__,
        values: *const u32,
    ) {
        (::windows::Interface::vtable(self).51)(
            ::windows::Abi::abi(self),
            p_unordered_access_view.into().abi(),
            values,
        )
    }
    pub unsafe fn ClearUnorderedAccessViewFloat<
        'a,
        T0__: ::std::convert::Into<::windows::Param<'a, ID3D11UnorderedAccessView>>,
    >(
        &self,
        p_unordered_access_view: T0__,
        values: *const f32,
    ) {
        (::windows::Interface::vtable(self).52)(
            ::windows::Abi::abi(self),
            p_unordered_access_view.into().abi(),
            values,
        )
    }
    pub unsafe fn ClearDepthStencilView<
        'a,
        T0__: ::std::convert::Into<::windows::Param<'a, ID3D11DepthStencilView>>,
    >(
        &self,
        p_depth_stencil_view: T0__,
        clear_flags: u32,
        depth: f32,
        stencil: u8,
    ) {
        (::windows::Interface::vtable(self).53)(
            ::windows::Abi::abi(self),
            p_depth_stencil_view.into().abi(),
            clear_flags,
            depth,
            stencil,
        )
    }
    pub unsafe fn GenerateMips<
        'a,
        T0__: ::std::convert::Into<::windows::Param<'a, ID3D11ShaderResourceView>>,
    >(
        &self,
        p_shader_resource_view: T0__,
    ) {
        (::windows::Interface::vtable(self).54)(
            ::windows::Abi::abi(self),
            p_shader_resource_view.into().abi(),
        )
    }
    pub unsafe fn SetResourceMinLOD<
        'a,
        T0__: ::std::convert::Into<::windows::Param<'a, ID3D11Resource>>,
    >(
        &self,
        p_resource: T0__,
        min_lod: f32,
    ) {
        (::windows::Interface::vtable(self).55)(
            ::windows::Abi::abi(self),
            p_resource.into().abi(),
            min_lod,
        )
    }
    pub unsafe fn GetResourceMinLOD<
        'a,
        T0__: ::std::convert::Into<::windows::Param<'a, ID3D11Resource>>,
    >(
        &self,
        p_resource: T0__,
    ) -> f32 {
        (::windows::Interface::vtable(self).56)(::windows::Abi::abi(self), p_resource.into().abi())
    }
    pub unsafe fn ResolveSubresource<
        'a,
        T0__: ::std::convert::Into<::windows::Param<'a, ID3D11Resource>>,
        T2__: ::std::convert::Into<::windows::Param<'a, ID3D11Resource>>,
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
            p_dst_resource.into().abi(),
            dst_subresource,
            p_src_resource.into().abi(),
            src_subresource,
            format,
        )
    }
    pub unsafe fn ExecuteCommandList<
        'a,
        T0__: ::std::convert::Into<::windows::Param<'a, ID3D11CommandList>>,
    >(
        &self,
        p_command_list: T0__,
        restore_context_state: ::windows::BOOL,
    ) {
        (::windows::Interface::vtable(self).58)(
            ::windows::Abi::abi(self),
            p_command_list.into().abi(),
            restore_context_state,
        )
    }
    pub unsafe fn HSSetShaderResources<
        'a,
        T2__: ::std::convert::Into<::windows::Param<'a, ID3D11ShaderResourceView>>,
    >(
        &self,
        start_slot: u32,
        num_views: u32,
        pp_shader_resource_views: T2__,
    ) {
        (::windows::Interface::vtable(self).59)(
            ::windows::Abi::abi(self),
            start_slot,
            num_views,
            pp_shader_resource_views.into().abi(),
        )
    }
    pub unsafe fn HSSetShader<
        'a,
        T0__: ::std::convert::Into<::windows::Param<'a, ID3D11HullShader>>,
        T1__: ::std::convert::Into<::windows::Param<'a, ID3D11ClassInstance>>,
    >(
        &self,
        p_hull_shader: T0__,
        pp_class_instances: T1__,
        num_class_instances: u32,
    ) {
        (::windows::Interface::vtable(self).60)(
            ::windows::Abi::abi(self),
            p_hull_shader.into().abi(),
            pp_class_instances.into().abi(),
            num_class_instances,
        )
    }
    pub unsafe fn HSSetSamplers<
        'a,
        T2__: ::std::convert::Into<::windows::Param<'a, ID3D11SamplerState>>,
    >(
        &self,
        start_slot: u32,
        num_samplers: u32,
        pp_samplers: T2__,
    ) {
        (::windows::Interface::vtable(self).61)(
            ::windows::Abi::abi(self),
            start_slot,
            num_samplers,
            pp_samplers.into().abi(),
        )
    }
    pub unsafe fn HSSetConstantBuffers<
        'a,
        T2__: ::std::convert::Into<::windows::Param<'a, ID3D11Buffer>>,
    >(
        &self,
        start_slot: u32,
        num_buffers: u32,
        pp_constant_buffers: T2__,
    ) {
        (::windows::Interface::vtable(self).62)(
            ::windows::Abi::abi(self),
            start_slot,
            num_buffers,
            pp_constant_buffers.into().abi(),
        )
    }
    pub unsafe fn DSSetShaderResources<
        'a,
        T2__: ::std::convert::Into<::windows::Param<'a, ID3D11ShaderResourceView>>,
    >(
        &self,
        start_slot: u32,
        num_views: u32,
        pp_shader_resource_views: T2__,
    ) {
        (::windows::Interface::vtable(self).63)(
            ::windows::Abi::abi(self),
            start_slot,
            num_views,
            pp_shader_resource_views.into().abi(),
        )
    }
    pub unsafe fn DSSetShader<
        'a,
        T0__: ::std::convert::Into<::windows::Param<'a, ID3D11DomainShader>>,
        T1__: ::std::convert::Into<::windows::Param<'a, ID3D11ClassInstance>>,
    >(
        &self,
        p_domain_shader: T0__,
        pp_class_instances: T1__,
        num_class_instances: u32,
    ) {
        (::windows::Interface::vtable(self).64)(
            ::windows::Abi::abi(self),
            p_domain_shader.into().abi(),
            pp_class_instances.into().abi(),
            num_class_instances,
        )
    }
    pub unsafe fn DSSetSamplers<
        'a,
        T2__: ::std::convert::Into<::windows::Param<'a, ID3D11SamplerState>>,
    >(
        &self,
        start_slot: u32,
        num_samplers: u32,
        pp_samplers: T2__,
    ) {
        (::windows::Interface::vtable(self).65)(
            ::windows::Abi::abi(self),
            start_slot,
            num_samplers,
            pp_samplers.into().abi(),
        )
    }
    pub unsafe fn DSSetConstantBuffers<
        'a,
        T2__: ::std::convert::Into<::windows::Param<'a, ID3D11Buffer>>,
    >(
        &self,
        start_slot: u32,
        num_buffers: u32,
        pp_constant_buffers: T2__,
    ) {
        (::windows::Interface::vtable(self).66)(
            ::windows::Abi::abi(self),
            start_slot,
            num_buffers,
            pp_constant_buffers.into().abi(),
        )
    }
    pub unsafe fn CSSetShaderResources<
        'a,
        T2__: ::std::convert::Into<::windows::Param<'a, ID3D11ShaderResourceView>>,
    >(
        &self,
        start_slot: u32,
        num_views: u32,
        pp_shader_resource_views: T2__,
    ) {
        (::windows::Interface::vtable(self).67)(
            ::windows::Abi::abi(self),
            start_slot,
            num_views,
            pp_shader_resource_views.into().abi(),
        )
    }
    pub unsafe fn CSSetUnorderedAccessViews<
        'a,
        T2__: ::std::convert::Into<::windows::Param<'a, ID3D11UnorderedAccessView>>,
    >(
        &self,
        start_slot: u32,
        num_ua_vs: u32,
        pp_unordered_access_views: T2__,
        p_uav_initial_counts: *const u32,
    ) {
        (::windows::Interface::vtable(self).68)(
            ::windows::Abi::abi(self),
            start_slot,
            num_ua_vs,
            pp_unordered_access_views.into().abi(),
            p_uav_initial_counts,
        )
    }
    pub unsafe fn CSSetShader<
        'a,
        T0__: ::std::convert::Into<::windows::Param<'a, ID3D11ComputeShader>>,
        T1__: ::std::convert::Into<::windows::Param<'a, ID3D11ClassInstance>>,
    >(
        &self,
        p_compute_shader: T0__,
        pp_class_instances: T1__,
        num_class_instances: u32,
    ) {
        (::windows::Interface::vtable(self).69)(
            ::windows::Abi::abi(self),
            p_compute_shader.into().abi(),
            pp_class_instances.into().abi(),
            num_class_instances,
        )
    }
    pub unsafe fn CSSetSamplers<
        'a,
        T2__: ::std::convert::Into<::windows::Param<'a, ID3D11SamplerState>>,
    >(
        &self,
        start_slot: u32,
        num_samplers: u32,
        pp_samplers: T2__,
    ) {
        (::windows::Interface::vtable(self).70)(
            ::windows::Abi::abi(self),
            start_slot,
            num_samplers,
            pp_samplers.into().abi(),
        )
    }
    pub unsafe fn CSSetConstantBuffers<
        'a,
        T2__: ::std::convert::Into<::windows::Param<'a, ID3D11Buffer>>,
    >(
        &self,
        start_slot: u32,
        num_buffers: u32,
        pp_constant_buffers: T2__,
    ) {
        (::windows::Interface::vtable(self).71)(
            ::windows::Abi::abi(self),
            start_slot,
            num_buffers,
            pp_constant_buffers.into().abi(),
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
            start_slot,
            num_buffers,
            pp_constant_buffers,
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
            start_slot,
            num_views,
            pp_shader_resource_views,
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
            pp_pixel_shader,
            pp_class_instances,
            p_num_class_instances,
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
            start_slot,
            num_samplers,
            pp_samplers,
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
            pp_vertex_shader,
            pp_class_instances,
            p_num_class_instances,
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
            start_slot,
            num_buffers,
            pp_constant_buffers,
        )
    }
    pub unsafe fn IAGetInputLayout(
        &self,
        pp_input_layout: *mut ::std::option::Option<ID3D11InputLayout>,
    ) {
        (::windows::Interface::vtable(self).78)(::windows::Abi::abi(self), pp_input_layout)
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
            start_slot,
            num_buffers,
            pp_vertex_buffers,
            p_strides,
            p_offsets,
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
            p_index_buffer,
            format,
            offset,
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
            start_slot,
            num_buffers,
            pp_constant_buffers,
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
            pp_geometry_shader,
            pp_class_instances,
            p_num_class_instances,
        )
    }
    pub unsafe fn IAGetPrimitiveTopology(&self, p_topology: *mut D3D_PRIMITIVE_TOPOLOGY) {
        (::windows::Interface::vtable(self).83)(::windows::Abi::abi(self), p_topology)
    }
    pub unsafe fn VSGetShaderResources(
        &self,
        start_slot: u32,
        num_views: u32,
        pp_shader_resource_views: *mut ::std::option::Option<ID3D11ShaderResourceView>,
    ) {
        (::windows::Interface::vtable(self).84)(
            ::windows::Abi::abi(self),
            start_slot,
            num_views,
            pp_shader_resource_views,
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
            start_slot,
            num_samplers,
            pp_samplers,
        )
    }
    pub unsafe fn GetPredication(
        &self,
        pp_predicate: *mut ::std::option::Option<ID3D11Predicate>,
        p_predicate_value: *mut i32,
    ) {
        (::windows::Interface::vtable(self).86)(
            ::windows::Abi::abi(self),
            pp_predicate,
            p_predicate_value,
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
            start_slot,
            num_views,
            pp_shader_resource_views,
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
            start_slot,
            num_samplers,
            pp_samplers,
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
            num_views,
            pp_render_target_views,
            pp_depth_stencil_view,
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
            num_rt_vs,
            pp_render_target_views,
            pp_depth_stencil_view,
            uav_start_slot,
            num_ua_vs,
            pp_unordered_access_views,
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
            pp_blend_state,
            blend_factor,
            p_sample_mask,
        )
    }
    pub unsafe fn OMGetDepthStencilState(
        &self,
        pp_depth_stencil_state: *mut ::std::option::Option<ID3D11DepthStencilState>,
        p_stencil_ref: *mut u32,
    ) {
        (::windows::Interface::vtable(self).92)(
            ::windows::Abi::abi(self),
            pp_depth_stencil_state,
            p_stencil_ref,
        )
    }
    pub unsafe fn SOGetTargets(
        &self,
        num_buffers: u32,
        pp_so_targets: *mut ::std::option::Option<ID3D11Buffer>,
    ) {
        (::windows::Interface::vtable(self).93)(
            ::windows::Abi::abi(self),
            num_buffers,
            pp_so_targets,
        )
    }
    pub unsafe fn RSGetState(
        &self,
        pp_rasterizer_state: *mut ::std::option::Option<ID3D11RasterizerState>,
    ) {
        (::windows::Interface::vtable(self).94)(::windows::Abi::abi(self), pp_rasterizer_state)
    }
    pub unsafe fn RSGetViewports(
        &self,
        p_num_viewports: *mut u32,
        p_viewports: *mut D3D11_VIEWPORT,
    ) {
        (::windows::Interface::vtable(self).95)(
            ::windows::Abi::abi(self),
            p_num_viewports,
            p_viewports,
        )
    }
    pub unsafe fn RSGetScissorRects(
        &self,
        p_num_rects: *mut u32,
        p_rects: *mut super::display_devices::RECT,
    ) {
        (::windows::Interface::vtable(self).96)(::windows::Abi::abi(self), p_num_rects, p_rects)
    }
    pub unsafe fn HSGetShaderResources(
        &self,
        start_slot: u32,
        num_views: u32,
        pp_shader_resource_views: *mut ::std::option::Option<ID3D11ShaderResourceView>,
    ) {
        (::windows::Interface::vtable(self).97)(
            ::windows::Abi::abi(self),
            start_slot,
            num_views,
            pp_shader_resource_views,
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
            pp_hull_shader,
            pp_class_instances,
            p_num_class_instances,
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
            start_slot,
            num_samplers,
            pp_samplers,
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
            start_slot,
            num_buffers,
            pp_constant_buffers,
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
            start_slot,
            num_views,
            pp_shader_resource_views,
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
            pp_domain_shader,
            pp_class_instances,
            p_num_class_instances,
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
            start_slot,
            num_samplers,
            pp_samplers,
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
            start_slot,
            num_buffers,
            pp_constant_buffers,
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
            start_slot,
            num_views,
            pp_shader_resource_views,
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
            start_slot,
            num_ua_vs,
            pp_unordered_access_views,
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
            pp_compute_shader,
            pp_class_instances,
            p_num_class_instances,
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
            start_slot,
            num_samplers,
            pp_samplers,
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
            start_slot,
            num_buffers,
            pp_constant_buffers,
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
    pub unsafe fn FinishCommandList(
        &self,
        restore_deferred_context_state: ::windows::BOOL,
        pp_command_list: *mut ::std::option::Option<ID3D11CommandList>,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).114)(
            ::windows::Abi::abi(self),
            restore_deferred_context_state,
            pp_command_list,
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
impl<'a> ::std::convert::Into<::windows::Param<'a, ::windows::IUnknown>> for ID3D11DeviceContext {
    fn into(self) -> ::windows::Param<'a, ::windows::IUnknown> {
        ::windows::Param::Owned(::std::convert::Into::<::windows::IUnknown>::into(self))
    }
}
impl<'a> ::std::convert::Into<::windows::Param<'a, ::windows::IUnknown>>
    for &'a ID3D11DeviceContext
{
    fn into(self) -> ::windows::Param<'a, ::windows::IUnknown> {
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
impl<'a> ::std::convert::Into<::windows::Param<'a, ID3D11DeviceChild>> for ID3D11DeviceContext {
    fn into(self) -> ::windows::Param<'a, ID3D11DeviceChild> {
        ::windows::Param::Owned(::std::convert::Into::<ID3D11DeviceChild>::into(self))
    }
}
impl<'a> ::std::convert::Into<::windows::Param<'a, ID3D11DeviceChild>> for &'a ID3D11DeviceContext {
    fn into(self) -> ::windows::Param<'a, ID3D11DeviceChild> {
        ::windows::Param::Owned(::std::convert::Into::<ID3D11DeviceChild>::into(
            ::std::clone::Clone::clone(self),
        ))
    }
}
#[repr(C)]
#[allow(non_camel_case_types)]
pub struct D3D11_COUNTER_INFO {
    pub last_device_dependent_counter: D3D11_COUNTER,
    pub num_simultaneous_counters: u32,
    pub num_detectable_parallel_units: u8,
}
#[repr(C)]
#[doc(hidden)]
#[allow(non_camel_case_types)]
pub struct D3D11_COUNTER_INFO_abi(D3D11_COUNTER, u32, u8);
impl D3D11_COUNTER_INFO {}
unsafe impl ::windows::Abi for D3D11_COUNTER_INFO {
    type Abi = D3D11_COUNTER_INFO_abi;
}
impl ::core::default::Default for D3D11_COUNTER_INFO {
    fn default() -> Self {
        Self {
            last_device_dependent_counter: ::std::default::Default::default(),
            num_simultaneous_counters: 0,
            num_detectable_parallel_units: 0,
        }
    }
}
impl ::core::fmt::Debug for D3D11_COUNTER_INFO {
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
impl ::core::clone::Clone for D3D11_COUNTER_INFO {
    fn clone(&self) -> Self {
        Self {
            last_device_dependent_counter: self.last_device_dependent_counter,
            num_simultaneous_counters: self.num_simultaneous_counters,
            num_detectable_parallel_units: self.num_detectable_parallel_units,
        }
    }
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct D3D11_COUNTER_TYPE(pub i32);
impl ::std::convert::From<i32> for D3D11_COUNTER_TYPE {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
impl ::std::clone::Clone for D3D11_COUNTER_TYPE {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}
impl ::std::default::Default for D3D11_COUNTER_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::std::fmt::Debug for D3D11_COUNTER_TYPE {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        write!(f, "{:?}", self.0)
    }
}
impl ::std::cmp::PartialEq for D3D11_COUNTER_TYPE {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::std::cmp::Eq for D3D11_COUNTER_TYPE {}
impl ::std::marker::Copy for D3D11_COUNTER_TYPE {}
impl D3D11_COUNTER_TYPE {
    #![allow(non_upper_case_globals)]
    pub const D3D11_COUNTER_TYPE_FLOAT32: Self = Self(0i32);
    pub const D3D11_COUNTER_TYPE_UINT16: Self = Self(1i32);
    pub const D3D11_COUNTER_TYPE_UINT32: Self = Self(2i32);
    pub const D3D11_COUNTER_TYPE_UINT64: Self = Self(3i32);
}
unsafe impl ::windows::Abi for D3D11_COUNTER_TYPE {
    type Abi = Self;
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct D3D11_FEATURE(pub i32);
impl ::std::convert::From<i32> for D3D11_FEATURE {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
impl ::std::clone::Clone for D3D11_FEATURE {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}
impl ::std::default::Default for D3D11_FEATURE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::std::fmt::Debug for D3D11_FEATURE {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        write!(f, "{:?}", self.0)
    }
}
impl ::std::cmp::PartialEq for D3D11_FEATURE {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::std::cmp::Eq for D3D11_FEATURE {}
impl ::std::marker::Copy for D3D11_FEATURE {}
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
unsafe impl ::windows::Abi for D3D11_FEATURE {
    type Abi = Self;
}
#[repr(transparent)]
#[allow(non_camel_case_types)]
pub struct ID3D11Device(::windows::IUnknown);
impl ::std::clone::Clone for ID3D11Device {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::std::fmt::Debug for ID3D11Device {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        write!(f, "{:?}", self.0)
    }
}
impl ::std::cmp::PartialEq for ID3D11Device {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::std::cmp::Eq for ID3D11Device {}
unsafe impl ::windows::Interface for ID3D11Device {
    type Vtable = ID3D11Device_abi;
    const IID: ::windows::Guid = ::windows::Guid::from_values(
        3681512923,
        44151,
        20104,
        [130, 83, 129, 157, 249, 187, 241, 64],
    );
}
#[repr(C)]
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
        pp_buffer: *mut ::std::option::Option<ID3D11Buffer>,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        p_desc: *const D3D11_TEXTURE1D_DESC,
        p_initial_data: *const D3D11_SUBRESOURCE_DATA,
        pp_texture1d: *mut ::std::option::Option<ID3D11Texture1D>,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        p_desc: *const D3D11_TEXTURE2D_DESC,
        p_initial_data: *const D3D11_SUBRESOURCE_DATA,
        pp_texture2d: *mut ::std::option::Option<ID3D11Texture2D>,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        p_desc: *const D3D11_TEXTURE3D_DESC,
        p_initial_data: *const D3D11_SUBRESOURCE_DATA,
        pp_texture3d: *mut ::std::option::Option<ID3D11Texture3D>,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        p_resource: ::windows::RawPtr,
        p_desc: *const D3D11_SHADER_RESOURCE_VIEW_DESC,
        pp_sr_view: *mut ::std::option::Option<ID3D11ShaderResourceView>,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        p_resource: ::windows::RawPtr,
        p_desc: *const D3D11_UNORDERED_ACCESS_VIEW_DESC,
        pp_ua_view: *mut ::std::option::Option<ID3D11UnorderedAccessView>,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        p_resource: ::windows::RawPtr,
        p_desc: *const D3D11_RENDER_TARGET_VIEW_DESC,
        pp_rt_view: *mut ::std::option::Option<ID3D11RenderTargetView>,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        p_resource: ::windows::RawPtr,
        p_desc: *const D3D11_DEPTH_STENCIL_VIEW_DESC,
        pp_depth_stencil_view: *mut ::std::option::Option<ID3D11DepthStencilView>,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        p_input_element_descs: *const D3D11_INPUT_ELEMENT_DESC,
        num_elements: u32,
        p_shader_bytecode_with_input_signature: *const ::std::ffi::c_void,
        bytecode_length: usize,
        pp_input_layout: *mut ::std::option::Option<ID3D11InputLayout>,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        p_shader_bytecode: *const ::std::ffi::c_void,
        bytecode_length: usize,
        p_class_linkage: ::windows::RawPtr,
        pp_vertex_shader: *mut ::std::option::Option<ID3D11VertexShader>,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        p_shader_bytecode: *const ::std::ffi::c_void,
        bytecode_length: usize,
        p_class_linkage: ::windows::RawPtr,
        pp_geometry_shader: *mut ::std::option::Option<ID3D11GeometryShader>,
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
        pp_geometry_shader: *mut ::std::option::Option<ID3D11GeometryShader>,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        p_shader_bytecode: *const ::std::ffi::c_void,
        bytecode_length: usize,
        p_class_linkage: ::windows::RawPtr,
        pp_pixel_shader: *mut ::std::option::Option<ID3D11PixelShader>,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        p_shader_bytecode: *const ::std::ffi::c_void,
        bytecode_length: usize,
        p_class_linkage: ::windows::RawPtr,
        pp_hull_shader: *mut ::std::option::Option<ID3D11HullShader>,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        p_shader_bytecode: *const ::std::ffi::c_void,
        bytecode_length: usize,
        p_class_linkage: ::windows::RawPtr,
        pp_domain_shader: *mut ::std::option::Option<ID3D11DomainShader>,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        p_shader_bytecode: *const ::std::ffi::c_void,
        bytecode_length: usize,
        p_class_linkage: ::windows::RawPtr,
        pp_compute_shader: *mut ::std::option::Option<ID3D11ComputeShader>,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        pp_linkage: *mut ::std::option::Option<ID3D11ClassLinkage>,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        p_blend_state_desc: *const D3D11_BLEND_DESC,
        pp_blend_state: *mut ::std::option::Option<ID3D11BlendState>,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        p_depth_stencil_desc: *const D3D11_DEPTH_STENCIL_DESC,
        pp_depth_stencil_state: *mut ::std::option::Option<ID3D11DepthStencilState>,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        p_rasterizer_desc: *const D3D11_RASTERIZER_DESC,
        pp_rasterizer_state: *mut ::std::option::Option<ID3D11RasterizerState>,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        p_sampler_desc: *const D3D11_SAMPLER_DESC,
        pp_sampler_state: *mut ::std::option::Option<ID3D11SamplerState>,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        p_query_desc: *const D3D11_QUERY_DESC,
        pp_query: *mut ::std::option::Option<ID3D11Query>,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        p_predicate_desc: *const D3D11_QUERY_DESC,
        pp_predicate: *mut ::std::option::Option<ID3D11Predicate>,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        p_counter_desc: *const D3D11_COUNTER_DESC,
        pp_counter: *mut ::std::option::Option<ID3D11Counter>,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        context_flags: u32,
        pp_deferred_context: *mut ::std::option::Option<ID3D11DeviceContext>,
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
        sz_name: *mut i8,
        p_name_length: *mut u32,
        sz_units: *mut i8,
        p_units_length: *mut u32,
        sz_description: *mut i8,
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
        pp_immediate_context: *mut ::std::option::Option<ID3D11DeviceContext>,
    ),
    pub unsafe extern "system" fn(this: ::windows::RawPtr, raise_flags: u32) -> ::windows::ErrorCode,
    pub unsafe extern "system" fn(this: ::windows::RawPtr) -> u32,
);
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
            p_desc,
            p_initial_data,
            pp_buffer,
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
            p_desc,
            p_initial_data,
            pp_texture1d,
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
            p_desc,
            p_initial_data,
            pp_texture2d,
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
            p_desc,
            p_initial_data,
            pp_texture3d,
        )
    }
    pub unsafe fn CreateShaderResourceView<
        'a,
        T0__: ::std::convert::Into<::windows::Param<'a, ID3D11Resource>>,
    >(
        &self,
        p_resource: T0__,
        p_desc: *const D3D11_SHADER_RESOURCE_VIEW_DESC,
        pp_sr_view: *mut ::std::option::Option<ID3D11ShaderResourceView>,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).7)(
            ::windows::Abi::abi(self),
            p_resource.into().abi(),
            p_desc,
            pp_sr_view,
        )
    }
    pub unsafe fn CreateUnorderedAccessView<
        'a,
        T0__: ::std::convert::Into<::windows::Param<'a, ID3D11Resource>>,
    >(
        &self,
        p_resource: T0__,
        p_desc: *const D3D11_UNORDERED_ACCESS_VIEW_DESC,
        pp_ua_view: *mut ::std::option::Option<ID3D11UnorderedAccessView>,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).8)(
            ::windows::Abi::abi(self),
            p_resource.into().abi(),
            p_desc,
            pp_ua_view,
        )
    }
    pub unsafe fn CreateRenderTargetView<
        'a,
        T0__: ::std::convert::Into<::windows::Param<'a, ID3D11Resource>>,
    >(
        &self,
        p_resource: T0__,
        p_desc: *const D3D11_RENDER_TARGET_VIEW_DESC,
        pp_rt_view: *mut ::std::option::Option<ID3D11RenderTargetView>,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).9)(
            ::windows::Abi::abi(self),
            p_resource.into().abi(),
            p_desc,
            pp_rt_view,
        )
    }
    pub unsafe fn CreateDepthStencilView<
        'a,
        T0__: ::std::convert::Into<::windows::Param<'a, ID3D11Resource>>,
    >(
        &self,
        p_resource: T0__,
        p_desc: *const D3D11_DEPTH_STENCIL_VIEW_DESC,
        pp_depth_stencil_view: *mut ::std::option::Option<ID3D11DepthStencilView>,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).10)(
            ::windows::Abi::abi(self),
            p_resource.into().abi(),
            p_desc,
            pp_depth_stencil_view,
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
            p_input_element_descs,
            num_elements,
            p_shader_bytecode_with_input_signature,
            bytecode_length,
            pp_input_layout,
        )
    }
    pub unsafe fn CreateVertexShader<
        'a,
        T2__: ::std::convert::Into<::windows::Param<'a, ID3D11ClassLinkage>>,
    >(
        &self,
        p_shader_bytecode: *const ::std::ffi::c_void,
        bytecode_length: usize,
        p_class_linkage: T2__,
        pp_vertex_shader: *mut ::std::option::Option<ID3D11VertexShader>,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).12)(
            ::windows::Abi::abi(self),
            p_shader_bytecode,
            bytecode_length,
            p_class_linkage.into().abi(),
            pp_vertex_shader,
        )
    }
    pub unsafe fn CreateGeometryShader<
        'a,
        T2__: ::std::convert::Into<::windows::Param<'a, ID3D11ClassLinkage>>,
    >(
        &self,
        p_shader_bytecode: *const ::std::ffi::c_void,
        bytecode_length: usize,
        p_class_linkage: T2__,
        pp_geometry_shader: *mut ::std::option::Option<ID3D11GeometryShader>,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).13)(
            ::windows::Abi::abi(self),
            p_shader_bytecode,
            bytecode_length,
            p_class_linkage.into().abi(),
            pp_geometry_shader,
        )
    }
    pub unsafe fn CreateGeometryShaderWithStreamOutput<
        'a,
        T7__: ::std::convert::Into<::windows::Param<'a, ID3D11ClassLinkage>>,
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
            p_shader_bytecode,
            bytecode_length,
            p_so_declaration,
            num_entries,
            p_buffer_strides,
            num_strides,
            rasterized_stream,
            p_class_linkage.into().abi(),
            pp_geometry_shader,
        )
    }
    pub unsafe fn CreatePixelShader<
        'a,
        T2__: ::std::convert::Into<::windows::Param<'a, ID3D11ClassLinkage>>,
    >(
        &self,
        p_shader_bytecode: *const ::std::ffi::c_void,
        bytecode_length: usize,
        p_class_linkage: T2__,
        pp_pixel_shader: *mut ::std::option::Option<ID3D11PixelShader>,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).15)(
            ::windows::Abi::abi(self),
            p_shader_bytecode,
            bytecode_length,
            p_class_linkage.into().abi(),
            pp_pixel_shader,
        )
    }
    pub unsafe fn CreateHullShader<
        'a,
        T2__: ::std::convert::Into<::windows::Param<'a, ID3D11ClassLinkage>>,
    >(
        &self,
        p_shader_bytecode: *const ::std::ffi::c_void,
        bytecode_length: usize,
        p_class_linkage: T2__,
        pp_hull_shader: *mut ::std::option::Option<ID3D11HullShader>,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).16)(
            ::windows::Abi::abi(self),
            p_shader_bytecode,
            bytecode_length,
            p_class_linkage.into().abi(),
            pp_hull_shader,
        )
    }
    pub unsafe fn CreateDomainShader<
        'a,
        T2__: ::std::convert::Into<::windows::Param<'a, ID3D11ClassLinkage>>,
    >(
        &self,
        p_shader_bytecode: *const ::std::ffi::c_void,
        bytecode_length: usize,
        p_class_linkage: T2__,
        pp_domain_shader: *mut ::std::option::Option<ID3D11DomainShader>,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).17)(
            ::windows::Abi::abi(self),
            p_shader_bytecode,
            bytecode_length,
            p_class_linkage.into().abi(),
            pp_domain_shader,
        )
    }
    pub unsafe fn CreateComputeShader<
        'a,
        T2__: ::std::convert::Into<::windows::Param<'a, ID3D11ClassLinkage>>,
    >(
        &self,
        p_shader_bytecode: *const ::std::ffi::c_void,
        bytecode_length: usize,
        p_class_linkage: T2__,
        pp_compute_shader: *mut ::std::option::Option<ID3D11ComputeShader>,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).18)(
            ::windows::Abi::abi(self),
            p_shader_bytecode,
            bytecode_length,
            p_class_linkage.into().abi(),
            pp_compute_shader,
        )
    }
    pub unsafe fn CreateClassLinkage(
        &self,
        pp_linkage: *mut ::std::option::Option<ID3D11ClassLinkage>,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).19)(::windows::Abi::abi(self), pp_linkage)
    }
    pub unsafe fn CreateBlendState(
        &self,
        p_blend_state_desc: *const D3D11_BLEND_DESC,
        pp_blend_state: *mut ::std::option::Option<ID3D11BlendState>,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).20)(
            ::windows::Abi::abi(self),
            p_blend_state_desc,
            pp_blend_state,
        )
    }
    pub unsafe fn CreateDepthStencilState(
        &self,
        p_depth_stencil_desc: *const D3D11_DEPTH_STENCIL_DESC,
        pp_depth_stencil_state: *mut ::std::option::Option<ID3D11DepthStencilState>,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).21)(
            ::windows::Abi::abi(self),
            p_depth_stencil_desc,
            pp_depth_stencil_state,
        )
    }
    pub unsafe fn CreateRasterizerState(
        &self,
        p_rasterizer_desc: *const D3D11_RASTERIZER_DESC,
        pp_rasterizer_state: *mut ::std::option::Option<ID3D11RasterizerState>,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).22)(
            ::windows::Abi::abi(self),
            p_rasterizer_desc,
            pp_rasterizer_state,
        )
    }
    pub unsafe fn CreateSamplerState(
        &self,
        p_sampler_desc: *const D3D11_SAMPLER_DESC,
        pp_sampler_state: *mut ::std::option::Option<ID3D11SamplerState>,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).23)(
            ::windows::Abi::abi(self),
            p_sampler_desc,
            pp_sampler_state,
        )
    }
    pub unsafe fn CreateQuery(
        &self,
        p_query_desc: *const D3D11_QUERY_DESC,
        pp_query: *mut ::std::option::Option<ID3D11Query>,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).24)(::windows::Abi::abi(self), p_query_desc, pp_query)
    }
    pub unsafe fn CreatePredicate(
        &self,
        p_predicate_desc: *const D3D11_QUERY_DESC,
        pp_predicate: *mut ::std::option::Option<ID3D11Predicate>,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).25)(
            ::windows::Abi::abi(self),
            p_predicate_desc,
            pp_predicate,
        )
    }
    pub unsafe fn CreateCounter(
        &self,
        p_counter_desc: *const D3D11_COUNTER_DESC,
        pp_counter: *mut ::std::option::Option<ID3D11Counter>,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).26)(
            ::windows::Abi::abi(self),
            p_counter_desc,
            pp_counter,
        )
    }
    pub unsafe fn CreateDeferredContext(
        &self,
        context_flags: u32,
        pp_deferred_context: *mut ::std::option::Option<ID3D11DeviceContext>,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).27)(
            ::windows::Abi::abi(self),
            context_flags,
            pp_deferred_context,
        )
    }
    pub unsafe fn OpenSharedResource(
        &self,
        h_resource: super::system_services::HANDLE,
        returned_interface: *const ::windows::Guid,
        pp_resource: *mut *mut ::std::ffi::c_void,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).28)(
            ::windows::Abi::abi(self),
            h_resource,
            returned_interface,
            pp_resource,
        )
    }
    pub unsafe fn CheckFormatSupport(
        &self,
        format: super::dxgi::DXGI_FORMAT,
        p_format_support: *mut u32,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).29)(::windows::Abi::abi(self), format, p_format_support)
    }
    pub unsafe fn CheckMultisampleQualityLevels(
        &self,
        format: super::dxgi::DXGI_FORMAT,
        sample_count: u32,
        p_num_quality_levels: *mut u32,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).30)(
            ::windows::Abi::abi(self),
            format,
            sample_count,
            p_num_quality_levels,
        )
    }
    pub unsafe fn CheckCounterInfo(&self, p_counter_info: *mut D3D11_COUNTER_INFO) {
        (::windows::Interface::vtable(self).31)(::windows::Abi::abi(self), p_counter_info)
    }
    pub unsafe fn CheckCounter(
        &self,
        p_desc: *const D3D11_COUNTER_DESC,
        p_type: *mut D3D11_COUNTER_TYPE,
        p_active_counters: *mut u32,
        sz_name: *mut i8,
        p_name_length: *mut u32,
        sz_units: *mut i8,
        p_units_length: *mut u32,
        sz_description: *mut i8,
        p_description_length: *mut u32,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).32)(
            ::windows::Abi::abi(self),
            p_desc,
            p_type,
            p_active_counters,
            sz_name,
            p_name_length,
            sz_units,
            p_units_length,
            sz_description,
            p_description_length,
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
            feature,
            p_feature_support_data,
            feature_support_data_size,
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
            guid,
            p_data_size,
            p_data,
        )
    }
    pub unsafe fn SetPrivateData(
        &self,
        guid: *const ::windows::Guid,
        data_size: u32,
        p_data: *const ::std::ffi::c_void,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).35)(::windows::Abi::abi(self), guid, data_size, p_data)
    }
    pub unsafe fn SetPrivateDataInterface<
        'a,
        T1__: ::std::convert::Into<::windows::Param<'a, ::windows::IUnknown>>,
    >(
        &self,
        guid: *const ::windows::Guid,
        p_data: T1__,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).36)(
            ::windows::Abi::abi(self),
            guid,
            p_data.into().abi(),
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
        (::windows::Interface::vtable(self).40)(::windows::Abi::abi(self), pp_immediate_context)
    }
    pub unsafe fn SetExceptionMode(&self, raise_flags: u32) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).41)(::windows::Abi::abi(self), raise_flags)
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
impl<'a> ::std::convert::Into<::windows::Param<'a, ::windows::IUnknown>> for ID3D11Device {
    fn into(self) -> ::windows::Param<'a, ::windows::IUnknown> {
        ::windows::Param::Owned(::std::convert::Into::<::windows::IUnknown>::into(self))
    }
}
impl<'a> ::std::convert::Into<::windows::Param<'a, ::windows::IUnknown>> for &'a ID3D11Device {
    fn into(self) -> ::windows::Param<'a, ::windows::IUnknown> {
        ::windows::Param::Owned(::std::convert::Into::<::windows::IUnknown>::into(
            ::std::clone::Clone::clone(self),
        ))
    }
}
#[repr(transparent)]
#[allow(non_camel_case_types)]
pub struct ID3DBlob(::windows::IUnknown);
impl ::std::clone::Clone for ID3DBlob {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::std::fmt::Debug for ID3DBlob {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        write!(f, "{:?}", self.0)
    }
}
impl ::std::cmp::PartialEq for ID3DBlob {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::std::cmp::Eq for ID3DBlob {}
unsafe impl ::windows::Interface for ID3DBlob {
    type Vtable = ID3DBlob_abi;
    const IID: ::windows::Guid =
        ::windows::Guid::from_values(2342910728, 20885, 16610, [172, 88, 13, 152, 156, 58, 1, 2]);
}
#[repr(C)]
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
impl<'a> ::std::convert::Into<::windows::Param<'a, ::windows::IUnknown>> for ID3DBlob {
    fn into(self) -> ::windows::Param<'a, ::windows::IUnknown> {
        ::windows::Param::Owned(::std::convert::Into::<::windows::IUnknown>::into(self))
    }
}
impl<'a> ::std::convert::Into<::windows::Param<'a, ::windows::IUnknown>> for &'a ID3DBlob {
    fn into(self) -> ::windows::Param<'a, ::windows::IUnknown> {
        ::windows::Param::Owned(::std::convert::Into::<::windows::IUnknown>::into(
            ::std::clone::Clone::clone(self),
        ))
    }
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct D3D_SHADER_VARIABLE_TYPE(pub i32);
impl ::std::convert::From<i32> for D3D_SHADER_VARIABLE_TYPE {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
impl ::std::clone::Clone for D3D_SHADER_VARIABLE_TYPE {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}
impl ::std::default::Default for D3D_SHADER_VARIABLE_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::std::fmt::Debug for D3D_SHADER_VARIABLE_TYPE {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        write!(f, "{:?}", self.0)
    }
}
impl ::std::cmp::PartialEq for D3D_SHADER_VARIABLE_TYPE {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::std::cmp::Eq for D3D_SHADER_VARIABLE_TYPE {}
impl ::std::marker::Copy for D3D_SHADER_VARIABLE_TYPE {}
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
unsafe impl ::windows::Abi for D3D_SHADER_VARIABLE_TYPE {
    type Abi = Self;
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct D3D_SHADER_VARIABLE_CLASS(pub i32);
impl ::std::convert::From<i32> for D3D_SHADER_VARIABLE_CLASS {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
impl ::std::clone::Clone for D3D_SHADER_VARIABLE_CLASS {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}
impl ::std::default::Default for D3D_SHADER_VARIABLE_CLASS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::std::fmt::Debug for D3D_SHADER_VARIABLE_CLASS {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        write!(f, "{:?}", self.0)
    }
}
impl ::std::cmp::PartialEq for D3D_SHADER_VARIABLE_CLASS {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::std::cmp::Eq for D3D_SHADER_VARIABLE_CLASS {}
impl ::std::marker::Copy for D3D_SHADER_VARIABLE_CLASS {}
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
unsafe impl ::windows::Abi for D3D_SHADER_VARIABLE_CLASS {
    type Abi = Self;
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct D3D_INTERPOLATION_MODE(pub i32);
impl ::std::convert::From<i32> for D3D_INTERPOLATION_MODE {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
impl ::std::clone::Clone for D3D_INTERPOLATION_MODE {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}
impl ::std::default::Default for D3D_INTERPOLATION_MODE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::std::fmt::Debug for D3D_INTERPOLATION_MODE {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        write!(f, "{:?}", self.0)
    }
}
impl ::std::cmp::PartialEq for D3D_INTERPOLATION_MODE {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::std::cmp::Eq for D3D_INTERPOLATION_MODE {}
impl ::std::marker::Copy for D3D_INTERPOLATION_MODE {}
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
unsafe impl ::windows::Abi for D3D_INTERPOLATION_MODE {
    type Abi = Self;
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct D3D_PARAMETER_FLAGS(pub i32);
impl ::std::convert::From<i32> for D3D_PARAMETER_FLAGS {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
impl ::std::clone::Clone for D3D_PARAMETER_FLAGS {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}
impl ::std::default::Default for D3D_PARAMETER_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::std::fmt::Debug for D3D_PARAMETER_FLAGS {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        write!(f, "{:?}", self.0)
    }
}
impl ::std::cmp::PartialEq for D3D_PARAMETER_FLAGS {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::std::cmp::Eq for D3D_PARAMETER_FLAGS {}
impl ::std::marker::Copy for D3D_PARAMETER_FLAGS {}
impl D3D_PARAMETER_FLAGS {
    #![allow(non_upper_case_globals)]
    pub const D3D_PF_NONE: Self = Self(0i32);
    pub const D3D_PF_IN: Self = Self(1i32);
    pub const D3D_PF_OUT: Self = Self(2i32);
    pub const D3D_PF_FORCE_DWORD: Self = Self(2147483647i32);
}
unsafe impl ::windows::Abi for D3D_PARAMETER_FLAGS {
    type Abi = Self;
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct D3D_CBUFFER_TYPE(pub i32);
impl ::std::convert::From<i32> for D3D_CBUFFER_TYPE {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
impl ::std::clone::Clone for D3D_CBUFFER_TYPE {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}
impl ::std::default::Default for D3D_CBUFFER_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::std::fmt::Debug for D3D_CBUFFER_TYPE {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        write!(f, "{:?}", self.0)
    }
}
impl ::std::cmp::PartialEq for D3D_CBUFFER_TYPE {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::std::cmp::Eq for D3D_CBUFFER_TYPE {}
impl ::std::marker::Copy for D3D_CBUFFER_TYPE {}
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
unsafe impl ::windows::Abi for D3D_CBUFFER_TYPE {
    type Abi = Self;
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct D3D_PRIMITIVE(pub i32);
impl ::std::convert::From<i32> for D3D_PRIMITIVE {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
impl ::std::clone::Clone for D3D_PRIMITIVE {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}
impl ::std::default::Default for D3D_PRIMITIVE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::std::fmt::Debug for D3D_PRIMITIVE {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        write!(f, "{:?}", self.0)
    }
}
impl ::std::cmp::PartialEq for D3D_PRIMITIVE {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::std::cmp::Eq for D3D_PRIMITIVE {}
impl ::std::marker::Copy for D3D_PRIMITIVE {}
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
unsafe impl ::windows::Abi for D3D_PRIMITIVE {
    type Abi = Self;
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct D3D_TESSELLATOR_OUTPUT_PRIMITIVE(pub i32);
impl ::std::convert::From<i32> for D3D_TESSELLATOR_OUTPUT_PRIMITIVE {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
impl ::std::clone::Clone for D3D_TESSELLATOR_OUTPUT_PRIMITIVE {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}
impl ::std::default::Default for D3D_TESSELLATOR_OUTPUT_PRIMITIVE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::std::fmt::Debug for D3D_TESSELLATOR_OUTPUT_PRIMITIVE {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        write!(f, "{:?}", self.0)
    }
}
impl ::std::cmp::PartialEq for D3D_TESSELLATOR_OUTPUT_PRIMITIVE {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::std::cmp::Eq for D3D_TESSELLATOR_OUTPUT_PRIMITIVE {}
impl ::std::marker::Copy for D3D_TESSELLATOR_OUTPUT_PRIMITIVE {}
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
unsafe impl ::windows::Abi for D3D_TESSELLATOR_OUTPUT_PRIMITIVE {
    type Abi = Self;
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct D3D_TESSELLATOR_PARTITIONING(pub i32);
impl ::std::convert::From<i32> for D3D_TESSELLATOR_PARTITIONING {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
impl ::std::clone::Clone for D3D_TESSELLATOR_PARTITIONING {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}
impl ::std::default::Default for D3D_TESSELLATOR_PARTITIONING {
    fn default() -> Self {
        Self(0)
    }
}
impl ::std::fmt::Debug for D3D_TESSELLATOR_PARTITIONING {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        write!(f, "{:?}", self.0)
    }
}
impl ::std::cmp::PartialEq for D3D_TESSELLATOR_PARTITIONING {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::std::cmp::Eq for D3D_TESSELLATOR_PARTITIONING {}
impl ::std::marker::Copy for D3D_TESSELLATOR_PARTITIONING {}
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
unsafe impl ::windows::Abi for D3D_TESSELLATOR_PARTITIONING {
    type Abi = Self;
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct D3D_TESSELLATOR_DOMAIN(pub i32);
impl ::std::convert::From<i32> for D3D_TESSELLATOR_DOMAIN {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
impl ::std::clone::Clone for D3D_TESSELLATOR_DOMAIN {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}
impl ::std::default::Default for D3D_TESSELLATOR_DOMAIN {
    fn default() -> Self {
        Self(0)
    }
}
impl ::std::fmt::Debug for D3D_TESSELLATOR_DOMAIN {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        write!(f, "{:?}", self.0)
    }
}
impl ::std::cmp::PartialEq for D3D_TESSELLATOR_DOMAIN {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::std::cmp::Eq for D3D_TESSELLATOR_DOMAIN {}
impl ::std::marker::Copy for D3D_TESSELLATOR_DOMAIN {}
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
unsafe impl ::windows::Abi for D3D_TESSELLATOR_DOMAIN {
    type Abi = Self;
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct D3D_SHADER_INPUT_TYPE(pub i32);
impl ::std::convert::From<i32> for D3D_SHADER_INPUT_TYPE {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
impl ::std::clone::Clone for D3D_SHADER_INPUT_TYPE {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}
impl ::std::default::Default for D3D_SHADER_INPUT_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::std::fmt::Debug for D3D_SHADER_INPUT_TYPE {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        write!(f, "{:?}", self.0)
    }
}
impl ::std::cmp::PartialEq for D3D_SHADER_INPUT_TYPE {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::std::cmp::Eq for D3D_SHADER_INPUT_TYPE {}
impl ::std::marker::Copy for D3D_SHADER_INPUT_TYPE {}
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
unsafe impl ::windows::Abi for D3D_SHADER_INPUT_TYPE {
    type Abi = Self;
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct D3D_RESOURCE_RETURN_TYPE(pub i32);
impl ::std::convert::From<i32> for D3D_RESOURCE_RETURN_TYPE {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
impl ::std::clone::Clone for D3D_RESOURCE_RETURN_TYPE {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}
impl ::std::default::Default for D3D_RESOURCE_RETURN_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::std::fmt::Debug for D3D_RESOURCE_RETURN_TYPE {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        write!(f, "{:?}", self.0)
    }
}
impl ::std::cmp::PartialEq for D3D_RESOURCE_RETURN_TYPE {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::std::cmp::Eq for D3D_RESOURCE_RETURN_TYPE {}
impl ::std::marker::Copy for D3D_RESOURCE_RETURN_TYPE {}
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
unsafe impl ::windows::Abi for D3D_RESOURCE_RETURN_TYPE {
    type Abi = Self;
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct D3D_NAME(pub i32);
impl ::std::convert::From<i32> for D3D_NAME {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
impl ::std::clone::Clone for D3D_NAME {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}
impl ::std::default::Default for D3D_NAME {
    fn default() -> Self {
        Self(0)
    }
}
impl ::std::fmt::Debug for D3D_NAME {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        write!(f, "{:?}", self.0)
    }
}
impl ::std::cmp::PartialEq for D3D_NAME {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::std::cmp::Eq for D3D_NAME {}
impl ::std::marker::Copy for D3D_NAME {}
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
unsafe impl ::windows::Abi for D3D_NAME {
    type Abi = Self;
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct D3D_REGISTER_COMPONENT_TYPE(pub i32);
impl ::std::convert::From<i32> for D3D_REGISTER_COMPONENT_TYPE {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
impl ::std::clone::Clone for D3D_REGISTER_COMPONENT_TYPE {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}
impl ::std::default::Default for D3D_REGISTER_COMPONENT_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::std::fmt::Debug for D3D_REGISTER_COMPONENT_TYPE {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        write!(f, "{:?}", self.0)
    }
}
impl ::std::cmp::PartialEq for D3D_REGISTER_COMPONENT_TYPE {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::std::cmp::Eq for D3D_REGISTER_COMPONENT_TYPE {}
impl ::std::marker::Copy for D3D_REGISTER_COMPONENT_TYPE {}
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
unsafe impl ::windows::Abi for D3D_REGISTER_COMPONENT_TYPE {
    type Abi = Self;
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct D3D_MIN_PRECISION(pub i32);
impl ::std::convert::From<i32> for D3D_MIN_PRECISION {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
impl ::std::clone::Clone for D3D_MIN_PRECISION {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}
impl ::std::default::Default for D3D_MIN_PRECISION {
    fn default() -> Self {
        Self(0)
    }
}
impl ::std::fmt::Debug for D3D_MIN_PRECISION {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        write!(f, "{:?}", self.0)
    }
}
impl ::std::cmp::PartialEq for D3D_MIN_PRECISION {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::std::cmp::Eq for D3D_MIN_PRECISION {}
impl ::std::marker::Copy for D3D_MIN_PRECISION {}
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
unsafe impl ::windows::Abi for D3D_MIN_PRECISION {
    type Abi = Self;
}
