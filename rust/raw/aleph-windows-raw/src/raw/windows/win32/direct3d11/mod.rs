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
#[repr(transparent)]
#[allow(non_camel_case_types)]
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
    pub unsafe extern "system" fn(),
    pub unsafe extern "system" fn(),
    pub unsafe extern "system" fn(),
    pub unsafe extern "system" fn(),
    pub unsafe extern "system" fn(),
    pub unsafe extern "system" fn(),
    pub unsafe extern "system" fn(),
    pub unsafe extern "system" fn(),
    pub unsafe extern "system" fn(),
    pub unsafe extern "system" fn(),
    pub unsafe extern "system" fn(),
    pub unsafe extern "system" fn(),
    pub unsafe extern "system" fn(),
    pub unsafe extern "system" fn(),
    pub unsafe extern "system" fn(),
    pub unsafe extern "system" fn(),
    pub unsafe extern "system" fn(),
    pub unsafe extern "system" fn(),
    pub unsafe extern "system" fn(),
    pub unsafe extern "system" fn(),
    pub unsafe extern "system" fn(),
    pub unsafe extern "system" fn(),
    pub unsafe extern "system" fn(),
    pub unsafe extern "system" fn(),
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
    pub unsafe extern "system" fn(),
    pub unsafe extern "system" fn(),
    pub unsafe extern "system" fn(),
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
#[allow(non_camel_case_types)]
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
#[allow(non_camel_case_types)]
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
    pub unsafe fn IASetPrimitiveTopology(&self, topology: D3D_PRIMITIVE_TOPOLOGY) {
        (::windows::Interface::vtable(self).24)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(topology),
        )
    }
    pub unsafe fn DrawAuto(&self) {
        (::windows::Interface::vtable(self).38)(::windows::Abi::abi(self))
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
    pub unsafe fn IAGetPrimitiveTopology(&self, p_topology: *mut D3D_PRIMITIVE_TOPOLOGY) {
        (::windows::Interface::vtable(self).83)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(p_topology),
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
    pub unsafe fn ClearState(&self) {
        (::windows::Interface::vtable(self).110)(::windows::Abi::abi(self))
    }
    pub unsafe fn Flush(&self) {
        (::windows::Interface::vtable(self).111)(::windows::Abi::abi(self))
    }
    pub unsafe fn GetContextFlags(&self) -> u32 {
        (::windows::Interface::vtable(self).113)(::windows::Abi::abi(self))
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
    pub unsafe extern "system" fn(),
    pub unsafe extern "system" fn(),
    pub unsafe extern "system" fn(),
    pub unsafe extern "system" fn(),
    pub unsafe extern "system" fn(),
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
    pub unsafe extern "system" fn(),
    pub unsafe extern "system" fn(),
    pub unsafe extern "system" fn(),
    pub unsafe extern "system" fn(),
    pub unsafe extern "system" fn(),
    pub unsafe extern "system" fn(),
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
    pub unsafe extern "system" fn(),
    pub unsafe extern "system" fn(),
    pub unsafe extern "system" fn(this: ::windows::RawPtr, topology: D3D_PRIMITIVE_TOPOLOGY),
    pub unsafe extern "system" fn(),
    pub unsafe extern "system" fn(),
    pub unsafe extern "system" fn(),
    pub unsafe extern "system" fn(),
    pub unsafe extern "system" fn(),
    pub unsafe extern "system" fn(),
    pub unsafe extern "system" fn(),
    pub unsafe extern "system" fn(),
    pub unsafe extern "system" fn(),
    pub unsafe extern "system" fn(),
    pub unsafe extern "system" fn(),
    pub unsafe extern "system" fn(),
    pub unsafe extern "system" fn(),
    pub unsafe extern "system" fn(this: ::windows::RawPtr),
    pub unsafe extern "system" fn(),
    pub unsafe extern "system" fn(),
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        thread_group_countx: u32,
        thread_group_county: u32,
        thread_group_countz: u32,
    ),
    pub unsafe extern "system" fn(),
    pub unsafe extern "system" fn(),
    pub unsafe extern "system" fn(),
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        num_rects: u32,
        p_rects: *const super::display_devices::RECT,
    ),
    pub unsafe extern "system" fn(),
    pub unsafe extern "system" fn(),
    pub unsafe extern "system" fn(),
    pub unsafe extern "system" fn(),
    pub unsafe extern "system" fn(),
    pub unsafe extern "system" fn(),
    pub unsafe extern "system" fn(),
    pub unsafe extern "system" fn(),
    pub unsafe extern "system" fn(),
    pub unsafe extern "system" fn(),
    pub unsafe extern "system" fn(),
    pub unsafe extern "system" fn(),
    pub unsafe extern "system" fn(),
    pub unsafe extern "system" fn(),
    pub unsafe extern "system" fn(),
    pub unsafe extern "system" fn(),
    pub unsafe extern "system" fn(),
    pub unsafe extern "system" fn(),
    pub unsafe extern "system" fn(),
    pub unsafe extern "system" fn(),
    pub unsafe extern "system" fn(),
    pub unsafe extern "system" fn(),
    pub unsafe extern "system" fn(),
    pub unsafe extern "system" fn(),
    pub unsafe extern "system" fn(),
    pub unsafe extern "system" fn(),
    pub unsafe extern "system" fn(),
    pub unsafe extern "system" fn(),
    pub unsafe extern "system" fn(),
    pub unsafe extern "system" fn(),
    pub unsafe extern "system" fn(),
    pub unsafe extern "system" fn(),
    pub unsafe extern "system" fn(),
    pub unsafe extern "system" fn(),
    pub unsafe extern "system" fn(),
    pub unsafe extern "system" fn(),
    pub unsafe extern "system" fn(),
    pub unsafe extern "system" fn(this: ::windows::RawPtr, p_topology: *mut D3D_PRIMITIVE_TOPOLOGY),
    pub unsafe extern "system" fn(),
    pub unsafe extern "system" fn(),
    pub unsafe extern "system" fn(),
    pub unsafe extern "system" fn(),
    pub unsafe extern "system" fn(),
    pub unsafe extern "system" fn(),
    pub unsafe extern "system" fn(),
    pub unsafe extern "system" fn(),
    pub unsafe extern "system" fn(),
    pub unsafe extern "system" fn(),
    pub unsafe extern "system" fn(),
    pub unsafe extern "system" fn(),
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        p_num_rects: *mut u32,
        p_rects: *mut super::display_devices::RECT,
    ),
    pub unsafe extern "system" fn(),
    pub unsafe extern "system" fn(),
    pub unsafe extern "system" fn(),
    pub unsafe extern "system" fn(),
    pub unsafe extern "system" fn(),
    pub unsafe extern "system" fn(),
    pub unsafe extern "system" fn(),
    pub unsafe extern "system" fn(),
    pub unsafe extern "system" fn(),
    pub unsafe extern "system" fn(),
    pub unsafe extern "system" fn(),
    pub unsafe extern "system" fn(),
    pub unsafe extern "system" fn(),
    pub unsafe extern "system" fn(this: ::windows::RawPtr),
    pub unsafe extern "system" fn(this: ::windows::RawPtr),
    pub unsafe extern "system" fn(),
    pub unsafe extern "system" fn(this: ::windows::RawPtr) -> u32,
    pub unsafe extern "system" fn(),
);
#[repr(transparent)]
#[allow(non_camel_case_types)]
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
