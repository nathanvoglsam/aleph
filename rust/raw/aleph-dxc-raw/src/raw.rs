//
//
// This file is a part of Aleph
//
// https://github.com/nathanvoglsam/aleph
//
// MIT License
//
// Copyright (c) 2020 Aleph Engine
//
// Permission is hereby granted, free of charge, to any person obtaining a copy
// of this software and associated documentation files (the "Software"), to deal
// in the Software without restriction, including without limitation the rights
// to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
// copies of the Software, and to permit persons to whom the Software is
// furnished to do so, subject to the following conditions:
//
// The above copyright notice and this permission notice shall be included in all
// copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
// AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
// LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
// OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
// SOFTWARE.
//

#![allow(unused_variables, non_upper_case_globals, non_snake_case)]

pub struct Apis {}
impl Apis {}
impl ::windows::RuntimeName for Apis {
    const NAME: &'static str = "Windows.Win32.DXC.Apis";
}
pub const CLSID_DxcAssembler: &'static str = "d728db68-f903-4f80-94cd-dccf76ec7151";
pub const CLSID_DxcCompiler: &'static str = "73e22d93-e6ce-47f3-b5bf-f0664f39c1b0";
pub const CLSID_DxcCompilerArgs: &'static str = "3e56ae82-224d-470f-a1a1-fe3016ee9f9d";
pub const CLSID_DxcContainerBuilder: &'static str = "94134294-411f-4574-b4d0-8741e25240d2";
pub const CLSID_DxcContainerReflection: &'static str = "b9f54489-55b8-400c-ba3a-1675e4728b91";
pub const CLSID_DxcDiaDataSource: &'static str = "cd1f6b73-2ab0-484d-8edc-ebe7a43ca09f";
pub const CLSID_DxcLibrary: &'static str = "6245d6af-66e0-48fd-80b4-4d271796748c";
pub const CLSID_DxcLinker: &'static str = "ef6a8087-b0ea-4d56-9e45-d07e1a8b7806";
pub const CLSID_DxcOptimizer: &'static str = "ae2cd79f-cc22-453f-9b6b-b124e7a5204c";
pub const CLSID_DxcValidator: &'static str = "8ca3e215-f728-4cf3-8cdd-88af917587a1";
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
pub struct DXC_OUT_KIND(pub i32);
impl DXC_OUT_KIND {
    #![allow(non_upper_case_globals)]
    pub const DXC_OUT_NONE: Self = Self(0i32);
    pub const DXC_OUT_OBJECT: Self = Self(1i32);
    pub const DXC_OUT_ERRORS: Self = Self(2i32);
    pub const DXC_OUT_PDB: Self = Self(3i32);
    pub const DXC_OUT_SHADER_HASH: Self = Self(4i32);
    pub const DXC_OUT_DISASSEMBLY: Self = Self(5i32);
    pub const DXC_OUT_HLSL: Self = Self(6i32);
    pub const DXC_OUT_TEXT: Self = Self(7i32);
    pub const DXC_OUT_REFLECTION: Self = Self(8i32);
    pub const DXC_OUT_ROOT_SIGNATURE: Self = Self(9i32);
    pub const DXC_OUT_EXTRA_OUTPUTS: Self = Self(10i32);
    pub const DXC_OUT_FORCE_DWORD: Self = Self(-1i32);
}
impl ::std::convert::From<i32> for DXC_OUT_KIND {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::Abi for DXC_OUT_KIND {
    type Abi = Self;
}
#[repr(C)]
#[allow(non_snake_case)]
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
pub struct DxcBuffer {
    pub ptr: *mut ::std::ffi::c_void,
    pub size: usize,
    pub encoding: u32,
}
impl DxcBuffer {}
impl ::std::default::Default for DxcBuffer {
    fn default() -> Self {
        Self {
            ptr: ::std::ptr::null_mut(),
            size: 0,
            encoding: 0,
        }
    }
}
impl ::std::fmt::Debug for DxcBuffer {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("DxcBuffer")
            .field("ptr", &format_args!("{:?}", self.ptr))
            .field("size", &format_args!("{:?}", self.size))
            .field("encoding", &format_args!("{:?}", self.encoding))
            .finish()
    }
}
impl ::std::cmp::PartialEq for DxcBuffer {
    fn eq(&self, other: &Self) -> bool {
        self.ptr == other.ptr && self.size == other.size && self.encoding == other.encoding
    }
}
impl ::std::cmp::Eq for DxcBuffer {}
unsafe impl ::windows::Abi for DxcBuffer {
    type Abi = Self;
}
#[allow(non_camel_case_types)]
pub type DxcCreateInstance2Proc = extern "system" fn(
    p_malloc: ::windows::RawPtr,
    rclsid: *const ::windows::Guid,
    riid: *const ::windows::Guid,
    ppv: *mut *mut ::std::ffi::c_void,
) -> ::windows::ErrorCode;
#[allow(non_camel_case_types)]
pub type DxcCreateInstanceProc = extern "system" fn(
    rclsid: *const ::windows::Guid,
    riid: *const ::windows::Guid,
    ppv: *mut *mut ::std::ffi::c_void,
) -> ::windows::ErrorCode;
#[repr(C)]
#[allow(non_snake_case)]
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
pub struct DxcDefine {
    pub name: *mut u16,
    pub value: *mut u16,
}
impl DxcDefine {}
impl ::std::default::Default for DxcDefine {
    fn default() -> Self {
        Self {
            name: ::std::ptr::null_mut(),
            value: ::std::ptr::null_mut(),
        }
    }
}
impl ::std::fmt::Debug for DxcDefine {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("DxcDefine")
            .field("name", &format_args!("{:?}", self.name))
            .field("value", &format_args!("{:?}", self.value))
            .finish()
    }
}
impl ::std::cmp::PartialEq for DxcDefine {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name && self.value == other.value
    }
}
impl ::std::cmp::Eq for DxcDefine {}
unsafe impl ::windows::Abi for DxcDefine {
    type Abi = Self;
}
#[repr(C)]
#[allow(non_snake_case)]
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
pub struct DxcShaderHash {
    pub flags: u32,
    pub hash_digest: [u8; 16usize],
}
impl DxcShaderHash {}
impl ::std::default::Default for DxcShaderHash {
    fn default() -> Self {
        Self {
            flags: 0,
            hash_digest: [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        }
    }
}
impl ::std::fmt::Debug for DxcShaderHash {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("DxcShaderHash")
            .field("flags", &format_args!("{:?}", self.flags))
            .field("hash_digest", &format_args!("{:?}", self.hash_digest))
            .finish()
    }
}
impl ::std::cmp::PartialEq for DxcShaderHash {
    fn eq(&self, other: &Self) -> bool {
        self.flags == other.flags && self.hash_digest == other.hash_digest
    }
}
impl ::std::cmp::Eq for DxcShaderHash {}
unsafe impl ::windows::Abi for DxcShaderHash {
    type Abi = Self;
}
pub const DxcValidatorFlags_Default: u32 = 0u32;
pub const DxcValidatorFlags_InPlaceEdit: u32 = 1u32;
pub const DxcValidatorFlags_ModuleOnly: u32 = 4u32;
pub const DxcValidatorFlags_RootSignatureOnly: u32 = 2u32;
pub const DxcValidatorFlags_ValidMask: u32 = 7u32;
pub const DxcVersionInfoFlags_Debug: u32 = 1u32;
pub const DxcVersionInfoFlags_Internal: u32 = 2u32;
pub const DxcVersionInfoFlags_None: u32 = 0u32;
#[repr(transparent)]
#[allow(non_camel_case_types)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct IDxcBlob(::windows::IUnknown);
impl IDxcBlob {}
unsafe impl ::windows::Interface for IDxcBlob {
    type Vtable = IDxcBlob_abi;
    const IID: ::windows::Guid = ::windows::Guid::zeroed();
}
#[allow(non_snake_case)]
impl IDxcBlob {
    pub unsafe fn GetBufferPointer(&self) -> *mut ::std::ffi::c_void {
        (::windows::Interface::vtable(self).3)(::windows::Abi::abi(self))
    }
    pub unsafe fn GetBufferSize(&self) -> usize {
        (::windows::Interface::vtable(self).4)(::windows::Abi::abi(self))
    }
}
impl ::std::convert::From<IDxcBlob> for ::windows::IUnknown {
    fn from(value: IDxcBlob) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IDxcBlob> for ::windows::IUnknown {
    fn from(value: &IDxcBlob) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::IntoParam<'a, ::windows::IUnknown> for IDxcBlob {
    fn into_param(self) -> ::windows::Param<'a, ::windows::IUnknown> {
        ::windows::Param::Owned(::std::convert::Into::<::windows::IUnknown>::into(self))
    }
}
impl<'a> ::windows::IntoParam<'a, ::windows::IUnknown> for &'a IDxcBlob {
    fn into_param(self) -> ::windows::Param<'a, ::windows::IUnknown> {
        ::windows::Param::Owned(::std::convert::Into::<::windows::IUnknown>::into(
            ::std::clone::Clone::clone(self),
        ))
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDxcBlob_abi(
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
#[repr(transparent)]
#[allow(non_camel_case_types)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct IDxcBlobEncoding(::windows::IUnknown);
impl IDxcBlobEncoding {}
unsafe impl ::windows::Interface for IDxcBlobEncoding {
    type Vtable = IDxcBlobEncoding_abi;
    const IID: ::windows::Guid = ::windows::Guid::zeroed();
}
#[allow(non_snake_case)]
impl IDxcBlobEncoding {
    pub unsafe fn GetBufferPointer(&self) -> *mut ::std::ffi::c_void {
        (::windows::Interface::vtable(self).3)(::windows::Abi::abi(self))
    }
    pub unsafe fn GetBufferSize(&self) -> usize {
        (::windows::Interface::vtable(self).4)(::windows::Abi::abi(self))
    }
    pub unsafe fn GetEncoding(
        &self,
        p_known: *mut windows_raw::win32::system_services::BOOL,
        p_code_page: *mut u32,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).5)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(p_known),
            ::std::mem::transmute(p_code_page),
        )
    }
}
impl ::std::convert::From<IDxcBlobEncoding> for ::windows::IUnknown {
    fn from(value: IDxcBlobEncoding) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IDxcBlobEncoding> for ::windows::IUnknown {
    fn from(value: &IDxcBlobEncoding) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::IntoParam<'a, ::windows::IUnknown> for IDxcBlobEncoding {
    fn into_param(self) -> ::windows::Param<'a, ::windows::IUnknown> {
        ::windows::Param::Owned(::std::convert::Into::<::windows::IUnknown>::into(self))
    }
}
impl<'a> ::windows::IntoParam<'a, ::windows::IUnknown> for &'a IDxcBlobEncoding {
    fn into_param(self) -> ::windows::Param<'a, ::windows::IUnknown> {
        ::windows::Param::Owned(::std::convert::Into::<::windows::IUnknown>::into(
            ::std::clone::Clone::clone(self),
        ))
    }
}
impl ::std::convert::From<IDxcBlobEncoding> for IDxcBlob {
    fn from(value: IDxcBlobEncoding) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IDxcBlobEncoding> for IDxcBlob {
    fn from(value: &IDxcBlobEncoding) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::IntoParam<'a, IDxcBlob> for IDxcBlobEncoding {
    fn into_param(self) -> ::windows::Param<'a, IDxcBlob> {
        ::windows::Param::Owned(::std::convert::Into::<IDxcBlob>::into(self))
    }
}
impl<'a> ::windows::IntoParam<'a, IDxcBlob> for &'a IDxcBlobEncoding {
    fn into_param(self) -> ::windows::Param<'a, IDxcBlob> {
        ::windows::Param::Owned(::std::convert::Into::<IDxcBlob>::into(
            ::std::clone::Clone::clone(self),
        ))
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDxcBlobEncoding_abi(
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        iid: &::windows::Guid,
        interface: *mut ::windows::RawPtr,
    ) -> ::windows::ErrorCode,
    pub unsafe extern "system" fn(this: ::windows::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::RawPtr) -> *mut ::std::ffi::c_void,
    pub unsafe extern "system" fn(this: ::windows::RawPtr) -> usize,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        p_known: *mut windows_raw::win32::system_services::BOOL,
        p_code_page: *mut u32,
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
pub struct IDxcOperationResult(::windows::IUnknown);
impl IDxcOperationResult {}
unsafe impl ::windows::Interface for IDxcOperationResult {
    type Vtable = IDxcOperationResult_abi;
    const IID: ::windows::Guid = ::windows::Guid::zeroed();
}
#[allow(non_snake_case)]
impl IDxcOperationResult {
    pub unsafe fn GetStatus(&self, p_status: *mut ::windows::ErrorCode) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).3)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(p_status),
        )
    }
    pub unsafe fn GetResult(
        &self,
        pp_result: *mut ::std::option::Option<IDxcBlob>,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).4)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(pp_result),
        )
    }
    pub unsafe fn GetErrorBuffer(
        &self,
        pp_errors: *mut ::std::option::Option<IDxcBlobEncoding>,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).5)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(pp_errors),
        )
    }
}
impl ::std::convert::From<IDxcOperationResult> for ::windows::IUnknown {
    fn from(value: IDxcOperationResult) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IDxcOperationResult> for ::windows::IUnknown {
    fn from(value: &IDxcOperationResult) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::IntoParam<'a, ::windows::IUnknown> for IDxcOperationResult {
    fn into_param(self) -> ::windows::Param<'a, ::windows::IUnknown> {
        ::windows::Param::Owned(::std::convert::Into::<::windows::IUnknown>::into(self))
    }
}
impl<'a> ::windows::IntoParam<'a, ::windows::IUnknown> for &'a IDxcOperationResult {
    fn into_param(self) -> ::windows::Param<'a, ::windows::IUnknown> {
        ::windows::Param::Owned(::std::convert::Into::<::windows::IUnknown>::into(
            ::std::clone::Clone::clone(self),
        ))
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDxcOperationResult_abi(
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        iid: &::windows::Guid,
        interface: *mut ::windows::RawPtr,
    ) -> ::windows::ErrorCode,
    pub unsafe extern "system" fn(this: ::windows::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::RawPtr) -> u32,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        p_status: *mut ::windows::ErrorCode,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        pp_result: *mut ::windows::RawPtr,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        pp_errors: *mut ::windows::RawPtr,
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
pub struct IDxcAssembler(::windows::IUnknown);
impl IDxcAssembler {}
unsafe impl ::windows::Interface for IDxcAssembler {
    type Vtable = IDxcAssembler_abi;
    const IID: ::windows::Guid = ::windows::Guid::zeroed();
}
#[allow(non_snake_case)]
impl IDxcAssembler {
    pub unsafe fn AssembleToContainer<'a, T0__: ::windows::IntoParam<'a, IDxcBlob>>(
        &self,
        p_shader: T0__,
        pp_result: *mut ::std::option::Option<IDxcOperationResult>,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).3)(
            ::windows::Abi::abi(self),
            p_shader.into_param().abi(),
            ::std::mem::transmute(pp_result),
        )
    }
}
impl ::std::convert::From<IDxcAssembler> for ::windows::IUnknown {
    fn from(value: IDxcAssembler) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IDxcAssembler> for ::windows::IUnknown {
    fn from(value: &IDxcAssembler) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::IntoParam<'a, ::windows::IUnknown> for IDxcAssembler {
    fn into_param(self) -> ::windows::Param<'a, ::windows::IUnknown> {
        ::windows::Param::Owned(::std::convert::Into::<::windows::IUnknown>::into(self))
    }
}
impl<'a> ::windows::IntoParam<'a, ::windows::IUnknown> for &'a IDxcAssembler {
    fn into_param(self) -> ::windows::Param<'a, ::windows::IUnknown> {
        ::windows::Param::Owned(::std::convert::Into::<::windows::IUnknown>::into(
            ::std::clone::Clone::clone(self),
        ))
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDxcAssembler_abi(
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        iid: &::windows::Guid,
        interface: *mut ::windows::RawPtr,
    ) -> ::windows::ErrorCode,
    pub unsafe extern "system" fn(this: ::windows::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::RawPtr) -> u32,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        p_shader: ::windows::RawPtr,
        pp_result: *mut ::windows::RawPtr,
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
pub struct IDxcBlobUtf16(::windows::IUnknown);
impl IDxcBlobUtf16 {}
unsafe impl ::windows::Interface for IDxcBlobUtf16 {
    type Vtable = IDxcBlobUtf16_abi;
    const IID: ::windows::Guid = ::windows::Guid::zeroed();
}
#[allow(non_snake_case)]
impl IDxcBlobUtf16 {
    pub unsafe fn GetBufferPointer(&self) -> *mut ::std::ffi::c_void {
        (::windows::Interface::vtable(self).3)(::windows::Abi::abi(self))
    }
    pub unsafe fn GetBufferSize(&self) -> usize {
        (::windows::Interface::vtable(self).4)(::windows::Abi::abi(self))
    }
    pub unsafe fn GetEncoding(
        &self,
        p_known: *mut windows_raw::win32::system_services::BOOL,
        p_code_page: *mut u32,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).5)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(p_known),
            ::std::mem::transmute(p_code_page),
        )
    }
    pub unsafe fn GetStringPointer(&self) -> *mut u16 {
        (::windows::Interface::vtable(self).6)(::windows::Abi::abi(self))
    }
    pub unsafe fn GetStringLength(&self) -> usize {
        (::windows::Interface::vtable(self).7)(::windows::Abi::abi(self))
    }
}
impl ::std::convert::From<IDxcBlobUtf16> for ::windows::IUnknown {
    fn from(value: IDxcBlobUtf16) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IDxcBlobUtf16> for ::windows::IUnknown {
    fn from(value: &IDxcBlobUtf16) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::IntoParam<'a, ::windows::IUnknown> for IDxcBlobUtf16 {
    fn into_param(self) -> ::windows::Param<'a, ::windows::IUnknown> {
        ::windows::Param::Owned(::std::convert::Into::<::windows::IUnknown>::into(self))
    }
}
impl<'a> ::windows::IntoParam<'a, ::windows::IUnknown> for &'a IDxcBlobUtf16 {
    fn into_param(self) -> ::windows::Param<'a, ::windows::IUnknown> {
        ::windows::Param::Owned(::std::convert::Into::<::windows::IUnknown>::into(
            ::std::clone::Clone::clone(self),
        ))
    }
}
impl ::std::convert::From<IDxcBlobUtf16> for IDxcBlobEncoding {
    fn from(value: IDxcBlobUtf16) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IDxcBlobUtf16> for IDxcBlobEncoding {
    fn from(value: &IDxcBlobUtf16) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::IntoParam<'a, IDxcBlobEncoding> for IDxcBlobUtf16 {
    fn into_param(self) -> ::windows::Param<'a, IDxcBlobEncoding> {
        ::windows::Param::Owned(::std::convert::Into::<IDxcBlobEncoding>::into(self))
    }
}
impl<'a> ::windows::IntoParam<'a, IDxcBlobEncoding> for &'a IDxcBlobUtf16 {
    fn into_param(self) -> ::windows::Param<'a, IDxcBlobEncoding> {
        ::windows::Param::Owned(::std::convert::Into::<IDxcBlobEncoding>::into(
            ::std::clone::Clone::clone(self),
        ))
    }
}
impl ::std::convert::From<IDxcBlobUtf16> for IDxcBlob {
    fn from(value: IDxcBlobUtf16) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IDxcBlobUtf16> for IDxcBlob {
    fn from(value: &IDxcBlobUtf16) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::IntoParam<'a, IDxcBlob> for IDxcBlobUtf16 {
    fn into_param(self) -> ::windows::Param<'a, IDxcBlob> {
        ::windows::Param::Owned(::std::convert::Into::<IDxcBlob>::into(self))
    }
}
impl<'a> ::windows::IntoParam<'a, IDxcBlob> for &'a IDxcBlobUtf16 {
    fn into_param(self) -> ::windows::Param<'a, IDxcBlob> {
        ::windows::Param::Owned(::std::convert::Into::<IDxcBlob>::into(
            ::std::clone::Clone::clone(self),
        ))
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDxcBlobUtf16_abi(
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        iid: &::windows::Guid,
        interface: *mut ::windows::RawPtr,
    ) -> ::windows::ErrorCode,
    pub unsafe extern "system" fn(this: ::windows::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::RawPtr) -> *mut ::std::ffi::c_void,
    pub unsafe extern "system" fn(this: ::windows::RawPtr) -> usize,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        p_known: *mut windows_raw::win32::system_services::BOOL,
        p_code_page: *mut u32,
    ) -> ::windows::ErrorCode,
    pub unsafe extern "system" fn(this: ::windows::RawPtr) -> *mut u16,
    pub unsafe extern "system" fn(this: ::windows::RawPtr) -> usize,
);
#[repr(transparent)]
#[allow(non_camel_case_types)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct IDxcBlobUtf8(::windows::IUnknown);
impl IDxcBlobUtf8 {}
unsafe impl ::windows::Interface for IDxcBlobUtf8 {
    type Vtable = IDxcBlobUtf8_abi;
    const IID: ::windows::Guid = ::windows::Guid::zeroed();
}
#[allow(non_snake_case)]
impl IDxcBlobUtf8 {
    pub unsafe fn GetBufferPointer(&self) -> *mut ::std::ffi::c_void {
        (::windows::Interface::vtable(self).3)(::windows::Abi::abi(self))
    }
    pub unsafe fn GetBufferSize(&self) -> usize {
        (::windows::Interface::vtable(self).4)(::windows::Abi::abi(self))
    }
    pub unsafe fn GetEncoding(
        &self,
        p_known: *mut windows_raw::win32::system_services::BOOL,
        p_code_page: *mut u32,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).5)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(p_known),
            ::std::mem::transmute(p_code_page),
        )
    }
    pub unsafe fn GetStringPointer(&self) -> *mut i8 {
        (::windows::Interface::vtable(self).6)(::windows::Abi::abi(self))
    }
    pub unsafe fn GetStringLength(&self) -> usize {
        (::windows::Interface::vtable(self).7)(::windows::Abi::abi(self))
    }
}
impl ::std::convert::From<IDxcBlobUtf8> for ::windows::IUnknown {
    fn from(value: IDxcBlobUtf8) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IDxcBlobUtf8> for ::windows::IUnknown {
    fn from(value: &IDxcBlobUtf8) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::IntoParam<'a, ::windows::IUnknown> for IDxcBlobUtf8 {
    fn into_param(self) -> ::windows::Param<'a, ::windows::IUnknown> {
        ::windows::Param::Owned(::std::convert::Into::<::windows::IUnknown>::into(self))
    }
}
impl<'a> ::windows::IntoParam<'a, ::windows::IUnknown> for &'a IDxcBlobUtf8 {
    fn into_param(self) -> ::windows::Param<'a, ::windows::IUnknown> {
        ::windows::Param::Owned(::std::convert::Into::<::windows::IUnknown>::into(
            ::std::clone::Clone::clone(self),
        ))
    }
}
impl ::std::convert::From<IDxcBlobUtf8> for IDxcBlobEncoding {
    fn from(value: IDxcBlobUtf8) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IDxcBlobUtf8> for IDxcBlobEncoding {
    fn from(value: &IDxcBlobUtf8) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::IntoParam<'a, IDxcBlobEncoding> for IDxcBlobUtf8 {
    fn into_param(self) -> ::windows::Param<'a, IDxcBlobEncoding> {
        ::windows::Param::Owned(::std::convert::Into::<IDxcBlobEncoding>::into(self))
    }
}
impl<'a> ::windows::IntoParam<'a, IDxcBlobEncoding> for &'a IDxcBlobUtf8 {
    fn into_param(self) -> ::windows::Param<'a, IDxcBlobEncoding> {
        ::windows::Param::Owned(::std::convert::Into::<IDxcBlobEncoding>::into(
            ::std::clone::Clone::clone(self),
        ))
    }
}
impl ::std::convert::From<IDxcBlobUtf8> for IDxcBlob {
    fn from(value: IDxcBlobUtf8) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IDxcBlobUtf8> for IDxcBlob {
    fn from(value: &IDxcBlobUtf8) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::IntoParam<'a, IDxcBlob> for IDxcBlobUtf8 {
    fn into_param(self) -> ::windows::Param<'a, IDxcBlob> {
        ::windows::Param::Owned(::std::convert::Into::<IDxcBlob>::into(self))
    }
}
impl<'a> ::windows::IntoParam<'a, IDxcBlob> for &'a IDxcBlobUtf8 {
    fn into_param(self) -> ::windows::Param<'a, IDxcBlob> {
        ::windows::Param::Owned(::std::convert::Into::<IDxcBlob>::into(
            ::std::clone::Clone::clone(self),
        ))
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDxcBlobUtf8_abi(
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        iid: &::windows::Guid,
        interface: *mut ::windows::RawPtr,
    ) -> ::windows::ErrorCode,
    pub unsafe extern "system" fn(this: ::windows::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::RawPtr) -> *mut ::std::ffi::c_void,
    pub unsafe extern "system" fn(this: ::windows::RawPtr) -> usize,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        p_known: *mut windows_raw::win32::system_services::BOOL,
        p_code_page: *mut u32,
    ) -> ::windows::ErrorCode,
    pub unsafe extern "system" fn(this: ::windows::RawPtr) -> *mut i8,
    pub unsafe extern "system" fn(this: ::windows::RawPtr) -> usize,
);
#[repr(transparent)]
#[allow(non_camel_case_types)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct IDxcIncludeHandler(::windows::IUnknown);
impl IDxcIncludeHandler {}
unsafe impl ::windows::Interface for IDxcIncludeHandler {
    type Vtable = IDxcIncludeHandler_abi;
    const IID: ::windows::Guid = ::windows::Guid::zeroed();
}
#[allow(non_snake_case)]
impl IDxcIncludeHandler {
    pub unsafe fn LoadSource(
        &self,
        p_filename: *const u16,
        pp_include_source: *mut ::std::option::Option<IDxcBlob>,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).3)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(p_filename),
            ::std::mem::transmute(pp_include_source),
        )
    }
}
impl ::std::convert::From<IDxcIncludeHandler> for ::windows::IUnknown {
    fn from(value: IDxcIncludeHandler) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IDxcIncludeHandler> for ::windows::IUnknown {
    fn from(value: &IDxcIncludeHandler) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::IntoParam<'a, ::windows::IUnknown> for IDxcIncludeHandler {
    fn into_param(self) -> ::windows::Param<'a, ::windows::IUnknown> {
        ::windows::Param::Owned(::std::convert::Into::<::windows::IUnknown>::into(self))
    }
}
impl<'a> ::windows::IntoParam<'a, ::windows::IUnknown> for &'a IDxcIncludeHandler {
    fn into_param(self) -> ::windows::Param<'a, ::windows::IUnknown> {
        ::windows::Param::Owned(::std::convert::Into::<::windows::IUnknown>::into(
            ::std::clone::Clone::clone(self),
        ))
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDxcIncludeHandler_abi(
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        iid: &::windows::Guid,
        interface: *mut ::windows::RawPtr,
    ) -> ::windows::ErrorCode,
    pub unsafe extern "system" fn(this: ::windows::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::RawPtr) -> u32,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        p_filename: *const u16,
        pp_include_source: *mut ::windows::RawPtr,
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
pub struct IDxcCompiler(::windows::IUnknown);
impl IDxcCompiler {}
unsafe impl ::windows::Interface for IDxcCompiler {
    type Vtable = IDxcCompiler_abi;
    const IID: ::windows::Guid = ::windows::Guid::zeroed();
}
#[allow(non_snake_case)]
impl IDxcCompiler {
    pub unsafe fn Compile<
        'a,
        T0__: ::windows::IntoParam<'a, IDxcBlob>,
        T8__: ::windows::IntoParam<'a, IDxcIncludeHandler>,
    >(
        &self,
        p_source: T0__,
        p_source_name: *const u16,
        p_entry_point: *const u16,
        p_target_profile: *const u16,
        p_arguments: *mut *mut u16,
        arg_count: u32,
        p_defines: *const DxcDefine,
        define_count: u32,
        p_include_handler: T8__,
        pp_result: *mut ::std::option::Option<IDxcOperationResult>,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).3)(
            ::windows::Abi::abi(self),
            p_source.into_param().abi(),
            ::std::mem::transmute(p_source_name),
            ::std::mem::transmute(p_entry_point),
            ::std::mem::transmute(p_target_profile),
            ::std::mem::transmute(p_arguments),
            ::std::mem::transmute(arg_count),
            ::std::mem::transmute(p_defines),
            ::std::mem::transmute(define_count),
            p_include_handler.into_param().abi(),
            ::std::mem::transmute(pp_result),
        )
    }
    pub unsafe fn Preprocess<
        'a,
        T0__: ::windows::IntoParam<'a, IDxcBlob>,
        T6__: ::windows::IntoParam<'a, IDxcIncludeHandler>,
    >(
        &self,
        p_source: T0__,
        p_source_name: *const u16,
        p_arguments: *mut *mut u16,
        arg_count: u32,
        p_defines: *const DxcDefine,
        define_count: u32,
        p_include_handler: T6__,
        pp_result: *mut ::std::option::Option<IDxcOperationResult>,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).4)(
            ::windows::Abi::abi(self),
            p_source.into_param().abi(),
            ::std::mem::transmute(p_source_name),
            ::std::mem::transmute(p_arguments),
            ::std::mem::transmute(arg_count),
            ::std::mem::transmute(p_defines),
            ::std::mem::transmute(define_count),
            p_include_handler.into_param().abi(),
            ::std::mem::transmute(pp_result),
        )
    }
    pub unsafe fn Disassemble<'a, T0__: ::windows::IntoParam<'a, IDxcBlob>>(
        &self,
        p_source: T0__,
        pp_disassembly: *mut ::std::option::Option<IDxcBlobEncoding>,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).5)(
            ::windows::Abi::abi(self),
            p_source.into_param().abi(),
            ::std::mem::transmute(pp_disassembly),
        )
    }
}
impl ::std::convert::From<IDxcCompiler> for ::windows::IUnknown {
    fn from(value: IDxcCompiler) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IDxcCompiler> for ::windows::IUnknown {
    fn from(value: &IDxcCompiler) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::IntoParam<'a, ::windows::IUnknown> for IDxcCompiler {
    fn into_param(self) -> ::windows::Param<'a, ::windows::IUnknown> {
        ::windows::Param::Owned(::std::convert::Into::<::windows::IUnknown>::into(self))
    }
}
impl<'a> ::windows::IntoParam<'a, ::windows::IUnknown> for &'a IDxcCompiler {
    fn into_param(self) -> ::windows::Param<'a, ::windows::IUnknown> {
        ::windows::Param::Owned(::std::convert::Into::<::windows::IUnknown>::into(
            ::std::clone::Clone::clone(self),
        ))
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDxcCompiler_abi(
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        iid: &::windows::Guid,
        interface: *mut ::windows::RawPtr,
    ) -> ::windows::ErrorCode,
    pub unsafe extern "system" fn(this: ::windows::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::RawPtr) -> u32,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        p_source: ::windows::RawPtr,
        p_source_name: *const u16,
        p_entry_point: *const u16,
        p_target_profile: *const u16,
        p_arguments: *mut *mut u16,
        arg_count: u32,
        p_defines: *const DxcDefine,
        define_count: u32,
        p_include_handler: ::windows::RawPtr,
        pp_result: *mut ::windows::RawPtr,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        p_source: ::windows::RawPtr,
        p_source_name: *const u16,
        p_arguments: *mut *mut u16,
        arg_count: u32,
        p_defines: *const DxcDefine,
        define_count: u32,
        p_include_handler: ::windows::RawPtr,
        pp_result: *mut ::windows::RawPtr,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        p_source: ::windows::RawPtr,
        pp_disassembly: *mut ::windows::RawPtr,
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
pub struct IDxcCompiler2(::windows::IUnknown);
impl IDxcCompiler2 {}
unsafe impl ::windows::Interface for IDxcCompiler2 {
    type Vtable = IDxcCompiler2_abi;
    const IID: ::windows::Guid = ::windows::Guid::zeroed();
}
#[allow(non_snake_case)]
impl IDxcCompiler2 {
    pub unsafe fn Compile<
        'a,
        T0__: ::windows::IntoParam<'a, IDxcBlob>,
        T8__: ::windows::IntoParam<'a, IDxcIncludeHandler>,
    >(
        &self,
        p_source: T0__,
        p_source_name: *const u16,
        p_entry_point: *const u16,
        p_target_profile: *const u16,
        p_arguments: *mut *mut u16,
        arg_count: u32,
        p_defines: *const DxcDefine,
        define_count: u32,
        p_include_handler: T8__,
        pp_result: *mut ::std::option::Option<IDxcOperationResult>,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).3)(
            ::windows::Abi::abi(self),
            p_source.into_param().abi(),
            ::std::mem::transmute(p_source_name),
            ::std::mem::transmute(p_entry_point),
            ::std::mem::transmute(p_target_profile),
            ::std::mem::transmute(p_arguments),
            ::std::mem::transmute(arg_count),
            ::std::mem::transmute(p_defines),
            ::std::mem::transmute(define_count),
            p_include_handler.into_param().abi(),
            ::std::mem::transmute(pp_result),
        )
    }
    pub unsafe fn Preprocess<
        'a,
        T0__: ::windows::IntoParam<'a, IDxcBlob>,
        T6__: ::windows::IntoParam<'a, IDxcIncludeHandler>,
    >(
        &self,
        p_source: T0__,
        p_source_name: *const u16,
        p_arguments: *mut *mut u16,
        arg_count: u32,
        p_defines: *const DxcDefine,
        define_count: u32,
        p_include_handler: T6__,
        pp_result: *mut ::std::option::Option<IDxcOperationResult>,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).4)(
            ::windows::Abi::abi(self),
            p_source.into_param().abi(),
            ::std::mem::transmute(p_source_name),
            ::std::mem::transmute(p_arguments),
            ::std::mem::transmute(arg_count),
            ::std::mem::transmute(p_defines),
            ::std::mem::transmute(define_count),
            p_include_handler.into_param().abi(),
            ::std::mem::transmute(pp_result),
        )
    }
    pub unsafe fn Disassemble<'a, T0__: ::windows::IntoParam<'a, IDxcBlob>>(
        &self,
        p_source: T0__,
        pp_disassembly: *mut ::std::option::Option<IDxcBlobEncoding>,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).5)(
            ::windows::Abi::abi(self),
            p_source.into_param().abi(),
            ::std::mem::transmute(pp_disassembly),
        )
    }
    pub unsafe fn CompileWithDebug<
        'a,
        T0__: ::windows::IntoParam<'a, IDxcBlob>,
        T8__: ::windows::IntoParam<'a, IDxcIncludeHandler>,
    >(
        &self,
        p_source: T0__,
        p_source_name: *const u16,
        p_entry_point: *const u16,
        p_target_profile: *const u16,
        p_arguments: *mut *mut u16,
        arg_count: u32,
        p_defines: *const DxcDefine,
        define_count: u32,
        p_include_handler: T8__,
        pp_result: *mut ::std::option::Option<IDxcOperationResult>,
        pp_debug_blob_name: *mut *mut u16,
        pp_debug_blob: *mut ::std::option::Option<IDxcBlob>,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).6)(
            ::windows::Abi::abi(self),
            p_source.into_param().abi(),
            ::std::mem::transmute(p_source_name),
            ::std::mem::transmute(p_entry_point),
            ::std::mem::transmute(p_target_profile),
            ::std::mem::transmute(p_arguments),
            ::std::mem::transmute(arg_count),
            ::std::mem::transmute(p_defines),
            ::std::mem::transmute(define_count),
            p_include_handler.into_param().abi(),
            ::std::mem::transmute(pp_result),
            ::std::mem::transmute(pp_debug_blob_name),
            ::std::mem::transmute(pp_debug_blob),
        )
    }
}
impl ::std::convert::From<IDxcCompiler2> for ::windows::IUnknown {
    fn from(value: IDxcCompiler2) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IDxcCompiler2> for ::windows::IUnknown {
    fn from(value: &IDxcCompiler2) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::IntoParam<'a, ::windows::IUnknown> for IDxcCompiler2 {
    fn into_param(self) -> ::windows::Param<'a, ::windows::IUnknown> {
        ::windows::Param::Owned(::std::convert::Into::<::windows::IUnknown>::into(self))
    }
}
impl<'a> ::windows::IntoParam<'a, ::windows::IUnknown> for &'a IDxcCompiler2 {
    fn into_param(self) -> ::windows::Param<'a, ::windows::IUnknown> {
        ::windows::Param::Owned(::std::convert::Into::<::windows::IUnknown>::into(
            ::std::clone::Clone::clone(self),
        ))
    }
}
impl ::std::convert::From<IDxcCompiler2> for IDxcCompiler {
    fn from(value: IDxcCompiler2) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IDxcCompiler2> for IDxcCompiler {
    fn from(value: &IDxcCompiler2) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::IntoParam<'a, IDxcCompiler> for IDxcCompiler2 {
    fn into_param(self) -> ::windows::Param<'a, IDxcCompiler> {
        ::windows::Param::Owned(::std::convert::Into::<IDxcCompiler>::into(self))
    }
}
impl<'a> ::windows::IntoParam<'a, IDxcCompiler> for &'a IDxcCompiler2 {
    fn into_param(self) -> ::windows::Param<'a, IDxcCompiler> {
        ::windows::Param::Owned(::std::convert::Into::<IDxcCompiler>::into(
            ::std::clone::Clone::clone(self),
        ))
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDxcCompiler2_abi(
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        iid: &::windows::Guid,
        interface: *mut ::windows::RawPtr,
    ) -> ::windows::ErrorCode,
    pub unsafe extern "system" fn(this: ::windows::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::RawPtr) -> u32,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        p_source: ::windows::RawPtr,
        p_source_name: *const u16,
        p_entry_point: *const u16,
        p_target_profile: *const u16,
        p_arguments: *mut *mut u16,
        arg_count: u32,
        p_defines: *const DxcDefine,
        define_count: u32,
        p_include_handler: ::windows::RawPtr,
        pp_result: *mut ::windows::RawPtr,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        p_source: ::windows::RawPtr,
        p_source_name: *const u16,
        p_arguments: *mut *mut u16,
        arg_count: u32,
        p_defines: *const DxcDefine,
        define_count: u32,
        p_include_handler: ::windows::RawPtr,
        pp_result: *mut ::windows::RawPtr,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        p_source: ::windows::RawPtr,
        pp_disassembly: *mut ::windows::RawPtr,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        p_source: ::windows::RawPtr,
        p_source_name: *const u16,
        p_entry_point: *const u16,
        p_target_profile: *const u16,
        p_arguments: *mut *mut u16,
        arg_count: u32,
        p_defines: *const DxcDefine,
        define_count: u32,
        p_include_handler: ::windows::RawPtr,
        pp_result: *mut ::windows::RawPtr,
        pp_debug_blob_name: *mut *mut u16,
        pp_debug_blob: *mut ::windows::RawPtr,
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
pub struct IDxcCompiler3(::windows::IUnknown);
impl IDxcCompiler3 {}
unsafe impl ::windows::Interface for IDxcCompiler3 {
    type Vtable = IDxcCompiler3_abi;
    const IID: ::windows::Guid = ::windows::Guid::zeroed();
}
#[allow(non_snake_case)]
impl IDxcCompiler3 {
    pub unsafe fn Compile<'a, T3__: ::windows::IntoParam<'a, IDxcIncludeHandler>>(
        &self,
        p_source: *const DxcBuffer,
        p_arguments: *mut *mut u16,
        arg_count: u32,
        p_include_handler: T3__,
        riid: *const ::windows::Guid,
        pp_result: *mut *mut ::std::ffi::c_void,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).3)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(p_source),
            ::std::mem::transmute(p_arguments),
            ::std::mem::transmute(arg_count),
            p_include_handler.into_param().abi(),
            ::std::mem::transmute(riid),
            ::std::mem::transmute(pp_result),
        )
    }
    pub unsafe fn Disassemble(
        &self,
        p_object: *const DxcBuffer,
        riid: *const ::windows::Guid,
        pp_result: *mut *mut ::std::ffi::c_void,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).4)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(p_object),
            ::std::mem::transmute(riid),
            ::std::mem::transmute(pp_result),
        )
    }
}
impl ::std::convert::From<IDxcCompiler3> for ::windows::IUnknown {
    fn from(value: IDxcCompiler3) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IDxcCompiler3> for ::windows::IUnknown {
    fn from(value: &IDxcCompiler3) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::IntoParam<'a, ::windows::IUnknown> for IDxcCompiler3 {
    fn into_param(self) -> ::windows::Param<'a, ::windows::IUnknown> {
        ::windows::Param::Owned(::std::convert::Into::<::windows::IUnknown>::into(self))
    }
}
impl<'a> ::windows::IntoParam<'a, ::windows::IUnknown> for &'a IDxcCompiler3 {
    fn into_param(self) -> ::windows::Param<'a, ::windows::IUnknown> {
        ::windows::Param::Owned(::std::convert::Into::<::windows::IUnknown>::into(
            ::std::clone::Clone::clone(self),
        ))
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDxcCompiler3_abi(
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        iid: &::windows::Guid,
        interface: *mut ::windows::RawPtr,
    ) -> ::windows::ErrorCode,
    pub unsafe extern "system" fn(this: ::windows::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::RawPtr) -> u32,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        p_source: *const DxcBuffer,
        p_arguments: *mut *mut u16,
        arg_count: u32,
        p_include_handler: ::windows::RawPtr,
        riid: *const ::windows::Guid,
        pp_result: *mut *mut ::std::ffi::c_void,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        p_object: *const DxcBuffer,
        riid: *const ::windows::Guid,
        pp_result: *mut *mut ::std::ffi::c_void,
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
pub struct IDxcCompilerArgs(::windows::IUnknown);
impl IDxcCompilerArgs {}
unsafe impl ::windows::Interface for IDxcCompilerArgs {
    type Vtable = IDxcCompilerArgs_abi;
    const IID: ::windows::Guid = ::windows::Guid::zeroed();
}
#[allow(non_snake_case)]
impl IDxcCompilerArgs {
    pub unsafe fn GetArguments(&self) -> *mut *mut u16 {
        (::windows::Interface::vtable(self).3)(::windows::Abi::abi(self))
    }
    pub unsafe fn GetCount(&self) -> u32 {
        (::windows::Interface::vtable(self).4)(::windows::Abi::abi(self))
    }
    pub unsafe fn AddArguments(
        &self,
        p_arguments: *mut *mut u16,
        arg_count: u32,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).5)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(p_arguments),
            ::std::mem::transmute(arg_count),
        )
    }
    pub unsafe fn AddArgumentsUTF8(
        &self,
        p_arguments: *mut *mut i8,
        arg_count: u32,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).6)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(p_arguments),
            ::std::mem::transmute(arg_count),
        )
    }
    pub unsafe fn AddDefines(
        &self,
        p_defines: *const DxcDefine,
        define_count: u32,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).7)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(p_defines),
            ::std::mem::transmute(define_count),
        )
    }
}
impl ::std::convert::From<IDxcCompilerArgs> for ::windows::IUnknown {
    fn from(value: IDxcCompilerArgs) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IDxcCompilerArgs> for ::windows::IUnknown {
    fn from(value: &IDxcCompilerArgs) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::IntoParam<'a, ::windows::IUnknown> for IDxcCompilerArgs {
    fn into_param(self) -> ::windows::Param<'a, ::windows::IUnknown> {
        ::windows::Param::Owned(::std::convert::Into::<::windows::IUnknown>::into(self))
    }
}
impl<'a> ::windows::IntoParam<'a, ::windows::IUnknown> for &'a IDxcCompilerArgs {
    fn into_param(self) -> ::windows::Param<'a, ::windows::IUnknown> {
        ::windows::Param::Owned(::std::convert::Into::<::windows::IUnknown>::into(
            ::std::clone::Clone::clone(self),
        ))
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDxcCompilerArgs_abi(
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        iid: &::windows::Guid,
        interface: *mut ::windows::RawPtr,
    ) -> ::windows::ErrorCode,
    pub unsafe extern "system" fn(this: ::windows::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::RawPtr) -> *mut *mut u16,
    pub unsafe extern "system" fn(this: ::windows::RawPtr) -> u32,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        p_arguments: *mut *mut u16,
        arg_count: u32,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        p_arguments: *mut *mut i8,
        arg_count: u32,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        p_defines: *const DxcDefine,
        define_count: u32,
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
pub struct IDxcContainerBuilder(::windows::IUnknown);
impl IDxcContainerBuilder {}
unsafe impl ::windows::Interface for IDxcContainerBuilder {
    type Vtable = IDxcContainerBuilder_abi;
    const IID: ::windows::Guid = ::windows::Guid::zeroed();
}
#[allow(non_snake_case)]
impl IDxcContainerBuilder {
    pub unsafe fn Load<'a, T0__: ::windows::IntoParam<'a, IDxcBlob>>(
        &self,
        p_dxil_container_header: T0__,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).3)(
            ::windows::Abi::abi(self),
            p_dxil_container_header.into_param().abi(),
        )
    }
    pub unsafe fn AddPart<'a, T1__: ::windows::IntoParam<'a, IDxcBlob>>(
        &self,
        four_cc: u32,
        p_source: T1__,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).4)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(four_cc),
            p_source.into_param().abi(),
        )
    }
    pub unsafe fn RemovePart(&self, four_cc: u32) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).5)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(four_cc),
        )
    }
    pub unsafe fn SerializeContainer(
        &self,
        pp_result: *mut ::std::option::Option<IDxcOperationResult>,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).6)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(pp_result),
        )
    }
}
impl ::std::convert::From<IDxcContainerBuilder> for ::windows::IUnknown {
    fn from(value: IDxcContainerBuilder) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IDxcContainerBuilder> for ::windows::IUnknown {
    fn from(value: &IDxcContainerBuilder) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::IntoParam<'a, ::windows::IUnknown> for IDxcContainerBuilder {
    fn into_param(self) -> ::windows::Param<'a, ::windows::IUnknown> {
        ::windows::Param::Owned(::std::convert::Into::<::windows::IUnknown>::into(self))
    }
}
impl<'a> ::windows::IntoParam<'a, ::windows::IUnknown> for &'a IDxcContainerBuilder {
    fn into_param(self) -> ::windows::Param<'a, ::windows::IUnknown> {
        ::windows::Param::Owned(::std::convert::Into::<::windows::IUnknown>::into(
            ::std::clone::Clone::clone(self),
        ))
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDxcContainerBuilder_abi(
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        iid: &::windows::Guid,
        interface: *mut ::windows::RawPtr,
    ) -> ::windows::ErrorCode,
    pub unsafe extern "system" fn(this: ::windows::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::RawPtr) -> u32,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        p_dxil_container_header: ::windows::RawPtr,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        four_cc: u32,
        p_source: ::windows::RawPtr,
    ) -> ::windows::ErrorCode,
    pub unsafe extern "system" fn(this: ::windows::RawPtr, four_cc: u32) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        pp_result: *mut ::windows::RawPtr,
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
pub struct IDxcContainerReflection(::windows::IUnknown);
impl IDxcContainerReflection {}
unsafe impl ::windows::Interface for IDxcContainerReflection {
    type Vtable = IDxcContainerReflection_abi;
    const IID: ::windows::Guid = ::windows::Guid::zeroed();
}
#[allow(non_snake_case)]
impl IDxcContainerReflection {
    pub unsafe fn Load<'a, T0__: ::windows::IntoParam<'a, IDxcBlob>>(
        &self,
        p_container: T0__,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).3)(
            ::windows::Abi::abi(self),
            p_container.into_param().abi(),
        )
    }
    pub unsafe fn GetPartCount(&self, p_result: *mut u32) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).4)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(p_result),
        )
    }
    pub unsafe fn GetPartKind(&self, idx: u32, p_result: *mut u32) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).5)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(idx),
            ::std::mem::transmute(p_result),
        )
    }
    pub unsafe fn GetPartContent(
        &self,
        idx: u32,
        pp_result: *mut ::std::option::Option<IDxcBlob>,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).6)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(idx),
            ::std::mem::transmute(pp_result),
        )
    }
    pub unsafe fn FindFirstPartKind(&self, kind: u32, p_result: *mut u32) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).7)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(kind),
            ::std::mem::transmute(p_result),
        )
    }
    pub unsafe fn GetPartReflection(
        &self,
        idx: u32,
        iid: *const ::windows::Guid,
        ppv_object: *mut *mut ::std::ffi::c_void,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).8)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(idx),
            ::std::mem::transmute(iid),
            ::std::mem::transmute(ppv_object),
        )
    }
}
impl ::std::convert::From<IDxcContainerReflection> for ::windows::IUnknown {
    fn from(value: IDxcContainerReflection) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IDxcContainerReflection> for ::windows::IUnknown {
    fn from(value: &IDxcContainerReflection) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::IntoParam<'a, ::windows::IUnknown> for IDxcContainerReflection {
    fn into_param(self) -> ::windows::Param<'a, ::windows::IUnknown> {
        ::windows::Param::Owned(::std::convert::Into::<::windows::IUnknown>::into(self))
    }
}
impl<'a> ::windows::IntoParam<'a, ::windows::IUnknown> for &'a IDxcContainerReflection {
    fn into_param(self) -> ::windows::Param<'a, ::windows::IUnknown> {
        ::windows::Param::Owned(::std::convert::Into::<::windows::IUnknown>::into(
            ::std::clone::Clone::clone(self),
        ))
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDxcContainerReflection_abi(
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        iid: &::windows::Guid,
        interface: *mut ::windows::RawPtr,
    ) -> ::windows::ErrorCode,
    pub unsafe extern "system" fn(this: ::windows::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::RawPtr) -> u32,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        p_container: ::windows::RawPtr,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        p_result: *mut u32,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        idx: u32,
        p_result: *mut u32,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        idx: u32,
        pp_result: *mut ::windows::RawPtr,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        kind: u32,
        p_result: *mut u32,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        idx: u32,
        iid: *const ::windows::Guid,
        ppv_object: *mut *mut ::std::ffi::c_void,
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
pub struct IDxcExtraOutputs(::windows::IUnknown);
impl IDxcExtraOutputs {}
unsafe impl ::windows::Interface for IDxcExtraOutputs {
    type Vtable = IDxcExtraOutputs_abi;
    const IID: ::windows::Guid = ::windows::Guid::zeroed();
}
#[allow(non_snake_case)]
impl IDxcExtraOutputs {
    pub unsafe fn GetOutputCount(&self) -> u32 {
        (::windows::Interface::vtable(self).3)(::windows::Abi::abi(self))
    }
    pub unsafe fn GetOutput(
        &self,
        u_index: u32,
        iid: *const ::windows::Guid,
        ppv_object: *mut *mut ::std::ffi::c_void,
        pp_output_type: *mut ::std::option::Option<IDxcBlobUtf16>,
        pp_output_name: *mut ::std::option::Option<IDxcBlobUtf16>,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).4)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(u_index),
            ::std::mem::transmute(iid),
            ::std::mem::transmute(ppv_object),
            ::std::mem::transmute(pp_output_type),
            ::std::mem::transmute(pp_output_name),
        )
    }
}
impl ::std::convert::From<IDxcExtraOutputs> for ::windows::IUnknown {
    fn from(value: IDxcExtraOutputs) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IDxcExtraOutputs> for ::windows::IUnknown {
    fn from(value: &IDxcExtraOutputs) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::IntoParam<'a, ::windows::IUnknown> for IDxcExtraOutputs {
    fn into_param(self) -> ::windows::Param<'a, ::windows::IUnknown> {
        ::windows::Param::Owned(::std::convert::Into::<::windows::IUnknown>::into(self))
    }
}
impl<'a> ::windows::IntoParam<'a, ::windows::IUnknown> for &'a IDxcExtraOutputs {
    fn into_param(self) -> ::windows::Param<'a, ::windows::IUnknown> {
        ::windows::Param::Owned(::std::convert::Into::<::windows::IUnknown>::into(
            ::std::clone::Clone::clone(self),
        ))
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDxcExtraOutputs_abi(
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        iid: &::windows::Guid,
        interface: *mut ::windows::RawPtr,
    ) -> ::windows::ErrorCode,
    pub unsafe extern "system" fn(this: ::windows::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::RawPtr) -> u32,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        u_index: u32,
        iid: *const ::windows::Guid,
        ppv_object: *mut *mut ::std::ffi::c_void,
        pp_output_type: *mut ::windows::RawPtr,
        pp_output_name: *mut ::windows::RawPtr,
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
pub struct IDxcLibrary(::windows::IUnknown);
impl IDxcLibrary {}
unsafe impl ::windows::Interface for IDxcLibrary {
    type Vtable = IDxcLibrary_abi;
    const IID: ::windows::Guid = ::windows::Guid::zeroed();
}
#[allow(non_snake_case)]
impl IDxcLibrary {
    pub unsafe fn SetMalloc<
        'a,
        T0__: ::windows::IntoParam<'a, windows_raw::win32::com::IMalloc>,
    >(
        &self,
        p_malloc: T0__,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).3)(
            ::windows::Abi::abi(self),
            p_malloc.into_param().abi(),
        )
    }
    pub unsafe fn CreateBlobFromBlob<'a, T0__: ::windows::IntoParam<'a, IDxcBlob>>(
        &self,
        p_blob: T0__,
        offset: u32,
        length: u32,
        pp_result: *mut ::std::option::Option<IDxcBlob>,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).4)(
            ::windows::Abi::abi(self),
            p_blob.into_param().abi(),
            ::std::mem::transmute(offset),
            ::std::mem::transmute(length),
            ::std::mem::transmute(pp_result),
        )
    }
    pub unsafe fn CreateBlobFromFile(
        &self,
        p_file_name: *const u16,
        code_page: *mut u32,
        p_blob_encoding: *mut ::std::option::Option<IDxcBlobEncoding>,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).5)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(p_file_name),
            ::std::mem::transmute(code_page),
            ::std::mem::transmute(p_blob_encoding),
        )
    }
    pub unsafe fn CreateBlobWithEncodingFromPinned(
        &self,
        p_text: *mut ::std::ffi::c_void,
        size: u32,
        code_page: u32,
        p_blob_encoding: *mut ::std::option::Option<IDxcBlobEncoding>,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).6)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(p_text),
            ::std::mem::transmute(size),
            ::std::mem::transmute(code_page),
            ::std::mem::transmute(p_blob_encoding),
        )
    }
    pub unsafe fn CreateBlobWithEncodingOnHeapCopy(
        &self,
        p_text: *mut ::std::ffi::c_void,
        size: u32,
        code_page: u32,
        p_blob_encoding: *mut ::std::option::Option<IDxcBlobEncoding>,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).7)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(p_text),
            ::std::mem::transmute(size),
            ::std::mem::transmute(code_page),
            ::std::mem::transmute(p_blob_encoding),
        )
    }
    pub unsafe fn CreateBlobWithEncodingOnMalloc(
        &self,
        p_text: *mut ::std::ffi::c_void,
        p_imalloc: ::std::option::Option<windows_raw::win32::com::IMalloc>,
        size: u32,
        code_page: u32,
        p_blob_encoding: *mut ::std::option::Option<IDxcBlobEncoding>,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).8)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(p_text),
            ::std::mem::transmute(p_imalloc),
            ::std::mem::transmute(size),
            ::std::mem::transmute(code_page),
            ::std::mem::transmute(p_blob_encoding),
        )
    }
    pub unsafe fn CreateIncludeHandler(
        &self,
        pp_result: *mut ::std::option::Option<IDxcIncludeHandler>,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).9)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(pp_result),
        )
    }
    pub unsafe fn CreateStreamFromBlobReadOnly<'a, T0__: ::windows::IntoParam<'a, IDxcBlob>>(
        &self,
        p_blob: T0__,
        pp_stream: *mut ::std::option::Option<windows_raw::win32::structured_storage::IStream>,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).10)(
            ::windows::Abi::abi(self),
            p_blob.into_param().abi(),
            ::std::mem::transmute(pp_stream),
        )
    }
    pub unsafe fn GetBlobAsUtf8<'a, T0__: ::windows::IntoParam<'a, IDxcBlob>>(
        &self,
        p_blob: T0__,
        p_blob_encoding: *mut ::std::option::Option<IDxcBlobEncoding>,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).11)(
            ::windows::Abi::abi(self),
            p_blob.into_param().abi(),
            ::std::mem::transmute(p_blob_encoding),
        )
    }
    pub unsafe fn GetBlobAsUtf16<'a, T0__: ::windows::IntoParam<'a, IDxcBlob>>(
        &self,
        p_blob: T0__,
        p_blob_encoding: *mut ::std::option::Option<IDxcBlobEncoding>,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).12)(
            ::windows::Abi::abi(self),
            p_blob.into_param().abi(),
            ::std::mem::transmute(p_blob_encoding),
        )
    }
}
impl ::std::convert::From<IDxcLibrary> for ::windows::IUnknown {
    fn from(value: IDxcLibrary) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IDxcLibrary> for ::windows::IUnknown {
    fn from(value: &IDxcLibrary) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::IntoParam<'a, ::windows::IUnknown> for IDxcLibrary {
    fn into_param(self) -> ::windows::Param<'a, ::windows::IUnknown> {
        ::windows::Param::Owned(::std::convert::Into::<::windows::IUnknown>::into(self))
    }
}
impl<'a> ::windows::IntoParam<'a, ::windows::IUnknown> for &'a IDxcLibrary {
    fn into_param(self) -> ::windows::Param<'a, ::windows::IUnknown> {
        ::windows::Param::Owned(::std::convert::Into::<::windows::IUnknown>::into(
            ::std::clone::Clone::clone(self),
        ))
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDxcLibrary_abi(
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        iid: &::windows::Guid,
        interface: *mut ::windows::RawPtr,
    ) -> ::windows::ErrorCode,
    pub unsafe extern "system" fn(this: ::windows::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::RawPtr) -> u32,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        p_malloc: ::windows::RawPtr,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        p_blob: ::windows::RawPtr,
        offset: u32,
        length: u32,
        pp_result: *mut ::windows::RawPtr,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        p_file_name: *const u16,
        code_page: *mut u32,
        p_blob_encoding: *mut ::windows::RawPtr,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        p_text: *mut ::std::ffi::c_void,
        size: u32,
        code_page: u32,
        p_blob_encoding: *mut ::windows::RawPtr,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        p_text: *mut ::std::ffi::c_void,
        size: u32,
        code_page: u32,
        p_blob_encoding: *mut ::windows::RawPtr,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        p_text: *mut ::std::ffi::c_void,
        p_imalloc: ::windows::RawPtr,
        size: u32,
        code_page: u32,
        p_blob_encoding: *mut ::windows::RawPtr,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        pp_result: *mut ::windows::RawPtr,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        p_blob: ::windows::RawPtr,
        pp_stream: *mut ::windows::RawPtr,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        p_blob: ::windows::RawPtr,
        p_blob_encoding: *mut ::windows::RawPtr,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        p_blob: ::windows::RawPtr,
        p_blob_encoding: *mut ::windows::RawPtr,
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
pub struct IDxcLinker(::windows::IUnknown);
impl IDxcLinker {}
unsafe impl ::windows::Interface for IDxcLinker {
    type Vtable = IDxcLinker_abi;
    const IID: ::windows::Guid = ::windows::Guid::zeroed();
}
#[allow(non_snake_case)]
impl IDxcLinker {
    pub unsafe fn RegisterLibrary<'a, T1__: ::windows::IntoParam<'a, IDxcBlob>>(
        &self,
        p_lib_name: *const u16,
        p_lib: T1__,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).3)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(p_lib_name),
            p_lib.into_param().abi(),
        )
    }
    pub unsafe fn Link(
        &self,
        p_entry_name: *const u16,
        p_target_profile: *const u16,
        p_lib_names: *const *const u16,
        lib_count: u32,
        p_arguments: *const *const u16,
        arg_count: u32,
        pp_result: *mut ::std::option::Option<IDxcOperationResult>,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).4)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(p_entry_name),
            ::std::mem::transmute(p_target_profile),
            ::std::mem::transmute(p_lib_names),
            ::std::mem::transmute(lib_count),
            ::std::mem::transmute(p_arguments),
            ::std::mem::transmute(arg_count),
            ::std::mem::transmute(pp_result),
        )
    }
}
impl ::std::convert::From<IDxcLinker> for ::windows::IUnknown {
    fn from(value: IDxcLinker) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IDxcLinker> for ::windows::IUnknown {
    fn from(value: &IDxcLinker) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::IntoParam<'a, ::windows::IUnknown> for IDxcLinker {
    fn into_param(self) -> ::windows::Param<'a, ::windows::IUnknown> {
        ::windows::Param::Owned(::std::convert::Into::<::windows::IUnknown>::into(self))
    }
}
impl<'a> ::windows::IntoParam<'a, ::windows::IUnknown> for &'a IDxcLinker {
    fn into_param(self) -> ::windows::Param<'a, ::windows::IUnknown> {
        ::windows::Param::Owned(::std::convert::Into::<::windows::IUnknown>::into(
            ::std::clone::Clone::clone(self),
        ))
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDxcLinker_abi(
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        iid: &::windows::Guid,
        interface: *mut ::windows::RawPtr,
    ) -> ::windows::ErrorCode,
    pub unsafe extern "system" fn(this: ::windows::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::RawPtr) -> u32,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        p_lib_name: *const u16,
        p_lib: ::windows::RawPtr,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        p_entry_name: *const u16,
        p_target_profile: *const u16,
        p_lib_names: *const *const u16,
        lib_count: u32,
        p_arguments: *const *const u16,
        arg_count: u32,
        pp_result: *mut ::windows::RawPtr,
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
pub struct IDxcOptimizerPass(::windows::IUnknown);
impl IDxcOptimizerPass {}
unsafe impl ::windows::Interface for IDxcOptimizerPass {
    type Vtable = IDxcOptimizerPass_abi;
    const IID: ::windows::Guid = ::windows::Guid::zeroed();
}
#[allow(non_snake_case)]
impl IDxcOptimizerPass {
    pub unsafe fn GetOptionName(&self, pp_result: *mut *mut u16) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).3)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(pp_result),
        )
    }
    pub unsafe fn GetDescription(&self, pp_result: *mut *mut u16) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).4)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(pp_result),
        )
    }
    pub unsafe fn GetOptionArgCount(&self, p_count: *mut u32) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).5)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(p_count),
        )
    }
    pub unsafe fn GetOptionArgName(
        &self,
        arg_index: u32,
        pp_result: *mut *mut u16,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).6)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(arg_index),
            ::std::mem::transmute(pp_result),
        )
    }
    pub unsafe fn GetOptionArgDescription(
        &self,
        arg_index: u32,
        pp_result: *mut *mut u16,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).7)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(arg_index),
            ::std::mem::transmute(pp_result),
        )
    }
}
impl ::std::convert::From<IDxcOptimizerPass> for ::windows::IUnknown {
    fn from(value: IDxcOptimizerPass) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IDxcOptimizerPass> for ::windows::IUnknown {
    fn from(value: &IDxcOptimizerPass) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::IntoParam<'a, ::windows::IUnknown> for IDxcOptimizerPass {
    fn into_param(self) -> ::windows::Param<'a, ::windows::IUnknown> {
        ::windows::Param::Owned(::std::convert::Into::<::windows::IUnknown>::into(self))
    }
}
impl<'a> ::windows::IntoParam<'a, ::windows::IUnknown> for &'a IDxcOptimizerPass {
    fn into_param(self) -> ::windows::Param<'a, ::windows::IUnknown> {
        ::windows::Param::Owned(::std::convert::Into::<::windows::IUnknown>::into(
            ::std::clone::Clone::clone(self),
        ))
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDxcOptimizerPass_abi(
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        iid: &::windows::Guid,
        interface: *mut ::windows::RawPtr,
    ) -> ::windows::ErrorCode,
    pub unsafe extern "system" fn(this: ::windows::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::RawPtr) -> u32,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        pp_result: *mut *mut u16,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        pp_result: *mut *mut u16,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        p_count: *mut u32,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        arg_index: u32,
        pp_result: *mut *mut u16,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        arg_index: u32,
        pp_result: *mut *mut u16,
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
pub struct IDxcOptimizer(::windows::IUnknown);
impl IDxcOptimizer {}
unsafe impl ::windows::Interface for IDxcOptimizer {
    type Vtable = IDxcOptimizer_abi;
    const IID: ::windows::Guid = ::windows::Guid::zeroed();
}
#[allow(non_snake_case)]
impl IDxcOptimizer {
    pub unsafe fn GetAvailablePassCount(&self, p_count: *mut u32) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).3)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(p_count),
        )
    }
    pub unsafe fn GetAvailablePass(
        &self,
        index: u32,
        pp_result: *mut ::std::option::Option<IDxcOptimizerPass>,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).4)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(index),
            ::std::mem::transmute(pp_result),
        )
    }
    pub unsafe fn RunOptimizer(
        &self,
        p_blob: ::std::option::Option<IDxcBlob>,
        pp_options: *mut *mut u16,
        option_count: u32,
        p_output_module: *mut ::std::option::Option<IDxcBlob>,
        pp_output_text: *mut ::std::option::Option<IDxcBlobEncoding>,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).5)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(p_blob),
            ::std::mem::transmute(pp_options),
            ::std::mem::transmute(option_count),
            ::std::mem::transmute(p_output_module),
            ::std::mem::transmute(pp_output_text),
        )
    }
}
impl ::std::convert::From<IDxcOptimizer> for ::windows::IUnknown {
    fn from(value: IDxcOptimizer) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IDxcOptimizer> for ::windows::IUnknown {
    fn from(value: &IDxcOptimizer) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::IntoParam<'a, ::windows::IUnknown> for IDxcOptimizer {
    fn into_param(self) -> ::windows::Param<'a, ::windows::IUnknown> {
        ::windows::Param::Owned(::std::convert::Into::<::windows::IUnknown>::into(self))
    }
}
impl<'a> ::windows::IntoParam<'a, ::windows::IUnknown> for &'a IDxcOptimizer {
    fn into_param(self) -> ::windows::Param<'a, ::windows::IUnknown> {
        ::windows::Param::Owned(::std::convert::Into::<::windows::IUnknown>::into(
            ::std::clone::Clone::clone(self),
        ))
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDxcOptimizer_abi(
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        iid: &::windows::Guid,
        interface: *mut ::windows::RawPtr,
    ) -> ::windows::ErrorCode,
    pub unsafe extern "system" fn(this: ::windows::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::RawPtr) -> u32,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        p_count: *mut u32,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        index: u32,
        pp_result: *mut ::windows::RawPtr,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        p_blob: ::windows::RawPtr,
        pp_options: *mut *mut u16,
        option_count: u32,
        p_output_module: *mut ::windows::RawPtr,
        pp_output_text: *mut ::windows::RawPtr,
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
pub struct IDxcResult(::windows::IUnknown);
impl IDxcResult {}
unsafe impl ::windows::Interface for IDxcResult {
    type Vtable = IDxcResult_abi;
    const IID: ::windows::Guid = ::windows::Guid::zeroed();
}
#[allow(non_snake_case)]
impl IDxcResult {
    pub unsafe fn GetStatus(&self, p_status: *mut ::windows::ErrorCode) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).3)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(p_status),
        )
    }
    pub unsafe fn GetResult(
        &self,
        pp_result: *mut ::std::option::Option<IDxcBlob>,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).4)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(pp_result),
        )
    }
    pub unsafe fn GetErrorBuffer(
        &self,
        pp_errors: *mut ::std::option::Option<IDxcBlobEncoding>,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).5)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(pp_errors),
        )
    }
    pub unsafe fn HasOutput(
        &self,
        dxc_out_kind: DXC_OUT_KIND,
    ) -> windows_raw::win32::system_services::BOOL {
        (::windows::Interface::vtable(self).6)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(dxc_out_kind),
        )
    }
    pub unsafe fn GetOutput(
        &self,
        dxc_out_kind: DXC_OUT_KIND,
        iid: *const ::windows::Guid,
        ppv_object: *mut *mut ::std::ffi::c_void,
        pp_output_name: *mut ::std::option::Option<IDxcBlobUtf16>,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).7)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(dxc_out_kind),
            ::std::mem::transmute(iid),
            ::std::mem::transmute(ppv_object),
            ::std::mem::transmute(pp_output_name),
        )
    }
    pub unsafe fn GetNumOutputs(&self) -> u32 {
        (::windows::Interface::vtable(self).8)(::windows::Abi::abi(self))
    }
    pub unsafe fn GetOutputByIndex(&self, index: u32) -> DXC_OUT_KIND {
        (::windows::Interface::vtable(self).9)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(index),
        )
    }
    pub unsafe fn PrimaryOutput(&self) -> DXC_OUT_KIND {
        (::windows::Interface::vtable(self).10)(::windows::Abi::abi(self))
    }
}
impl ::std::convert::From<IDxcResult> for ::windows::IUnknown {
    fn from(value: IDxcResult) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IDxcResult> for ::windows::IUnknown {
    fn from(value: &IDxcResult) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::IntoParam<'a, ::windows::IUnknown> for IDxcResult {
    fn into_param(self) -> ::windows::Param<'a, ::windows::IUnknown> {
        ::windows::Param::Owned(::std::convert::Into::<::windows::IUnknown>::into(self))
    }
}
impl<'a> ::windows::IntoParam<'a, ::windows::IUnknown> for &'a IDxcResult {
    fn into_param(self) -> ::windows::Param<'a, ::windows::IUnknown> {
        ::windows::Param::Owned(::std::convert::Into::<::windows::IUnknown>::into(
            ::std::clone::Clone::clone(self),
        ))
    }
}
impl ::std::convert::From<IDxcResult> for IDxcOperationResult {
    fn from(value: IDxcResult) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IDxcResult> for IDxcOperationResult {
    fn from(value: &IDxcResult) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::IntoParam<'a, IDxcOperationResult> for IDxcResult {
    fn into_param(self) -> ::windows::Param<'a, IDxcOperationResult> {
        ::windows::Param::Owned(::std::convert::Into::<IDxcOperationResult>::into(self))
    }
}
impl<'a> ::windows::IntoParam<'a, IDxcOperationResult> for &'a IDxcResult {
    fn into_param(self) -> ::windows::Param<'a, IDxcOperationResult> {
        ::windows::Param::Owned(::std::convert::Into::<IDxcOperationResult>::into(
            ::std::clone::Clone::clone(self),
        ))
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDxcResult_abi(
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        iid: &::windows::Guid,
        interface: *mut ::windows::RawPtr,
    ) -> ::windows::ErrorCode,
    pub unsafe extern "system" fn(this: ::windows::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::RawPtr) -> u32,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        p_status: *mut ::windows::ErrorCode,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        pp_result: *mut ::windows::RawPtr,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        pp_errors: *mut ::windows::RawPtr,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        dxc_out_kind: DXC_OUT_KIND,
    ) -> windows_raw::win32::system_services::BOOL,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        dxc_out_kind: DXC_OUT_KIND,
        iid: *const ::windows::Guid,
        ppv_object: *mut *mut ::std::ffi::c_void,
        pp_output_name: *mut ::windows::RawPtr,
    ) -> ::windows::ErrorCode,
    pub unsafe extern "system" fn(this: ::windows::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::RawPtr, index: u32) -> DXC_OUT_KIND,
    pub unsafe extern "system" fn(this: ::windows::RawPtr) -> DXC_OUT_KIND,
);
#[repr(transparent)]
#[allow(non_camel_case_types)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct IDxcUtils(::windows::IUnknown);
impl IDxcUtils {}
unsafe impl ::windows::Interface for IDxcUtils {
    type Vtable = IDxcUtils_abi;
    const IID: ::windows::Guid = ::windows::Guid::zeroed();
}
#[allow(non_snake_case)]
impl IDxcUtils {
    pub unsafe fn CreateBlobFromBlob<'a, T0__: ::windows::IntoParam<'a, IDxcBlob>>(
        &self,
        p_blob: T0__,
        offset: u32,
        length: u32,
        pp_result: *mut ::std::option::Option<IDxcBlob>,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).3)(
            ::windows::Abi::abi(self),
            p_blob.into_param().abi(),
            ::std::mem::transmute(offset),
            ::std::mem::transmute(length),
            ::std::mem::transmute(pp_result),
        )
    }
    pub unsafe fn CreateBlobFromPinned(
        &self,
        p_data: *mut ::std::ffi::c_void,
        size: u32,
        code_page: u32,
        p_blob_encoding: *mut ::std::option::Option<IDxcBlobEncoding>,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).4)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(p_data),
            ::std::mem::transmute(size),
            ::std::mem::transmute(code_page),
            ::std::mem::transmute(p_blob_encoding),
        )
    }
    pub unsafe fn MoveToBlob(
        &self,
        p_data: *mut ::std::ffi::c_void,
        p_imalloc: ::std::option::Option<windows_raw::win32::com::IMalloc>,
        size: u32,
        code_page: u32,
        p_blob_encoding: *mut ::std::option::Option<IDxcBlobEncoding>,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).5)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(p_data),
            ::std::mem::transmute(p_imalloc),
            ::std::mem::transmute(size),
            ::std::mem::transmute(code_page),
            ::std::mem::transmute(p_blob_encoding),
        )
    }
    pub unsafe fn CreateBlob(
        &self,
        p_data: *mut ::std::ffi::c_void,
        size: u32,
        code_page: u32,
        p_blob_encoding: *mut ::std::option::Option<IDxcBlobEncoding>,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).6)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(p_data),
            ::std::mem::transmute(size),
            ::std::mem::transmute(code_page),
            ::std::mem::transmute(p_blob_encoding),
        )
    }
    pub unsafe fn LoadFile(
        &self,
        p_file_name: *const u16,
        p_code_page: *mut u32,
        p_blob_encoding: *mut ::std::option::Option<IDxcBlobEncoding>,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).7)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(p_file_name),
            ::std::mem::transmute(p_code_page),
            ::std::mem::transmute(p_blob_encoding),
        )
    }
    pub unsafe fn CreateReadOnlyStreamFromBlob<'a, T0__: ::windows::IntoParam<'a, IDxcBlob>>(
        &self,
        p_blob: T0__,
        pp_stream: *mut ::std::option::Option<windows_raw::win32::structured_storage::IStream>,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).8)(
            ::windows::Abi::abi(self),
            p_blob.into_param().abi(),
            ::std::mem::transmute(pp_stream),
        )
    }
    pub unsafe fn CreateDefaultIncludeHandler(
        &self,
        pp_result: *mut ::std::option::Option<IDxcIncludeHandler>,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).9)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(pp_result),
        )
    }
    pub unsafe fn GetBlobAsUtf8<'a, T0__: ::windows::IntoParam<'a, IDxcBlob>>(
        &self,
        p_blob: T0__,
        p_blob_encoding: *mut ::std::option::Option<IDxcBlobUtf8>,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).10)(
            ::windows::Abi::abi(self),
            p_blob.into_param().abi(),
            ::std::mem::transmute(p_blob_encoding),
        )
    }
    pub unsafe fn GetBlobAsUtf16<'a, T0__: ::windows::IntoParam<'a, IDxcBlob>>(
        &self,
        p_blob: T0__,
        p_blob_encoding: *mut ::std::option::Option<IDxcBlobUtf16>,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).11)(
            ::windows::Abi::abi(self),
            p_blob.into_param().abi(),
            ::std::mem::transmute(p_blob_encoding),
        )
    }
    pub unsafe fn GetDxilContainerPart(
        &self,
        p_shader: *const DxcBuffer,
        dxc_part: u32,
        pp_part_data: *mut *mut ::std::ffi::c_void,
        p_part_size_in_bytes: *mut u32,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).12)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(p_shader),
            ::std::mem::transmute(dxc_part),
            ::std::mem::transmute(pp_part_data),
            ::std::mem::transmute(p_part_size_in_bytes),
        )
    }
    pub unsafe fn CreateReflection(
        &self,
        p_data: *const DxcBuffer,
        iid: *const ::windows::Guid,
        ppv_reflection: *mut *mut ::std::ffi::c_void,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).13)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(p_data),
            ::std::mem::transmute(iid),
            ::std::mem::transmute(ppv_reflection),
        )
    }
    pub unsafe fn BuildArguments(
        &self,
        p_source_name: *const u16,
        p_entry_point: *const u16,
        p_target_profile: *const u16,
        p_arguments: *mut *mut u16,
        arg_count: u32,
        p_defines: *const DxcDefine,
        define_count: u32,
        pp_args: *mut ::std::option::Option<IDxcCompilerArgs>,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).14)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(p_source_name),
            ::std::mem::transmute(p_entry_point),
            ::std::mem::transmute(p_target_profile),
            ::std::mem::transmute(p_arguments),
            ::std::mem::transmute(arg_count),
            ::std::mem::transmute(p_defines),
            ::std::mem::transmute(define_count),
            ::std::mem::transmute(pp_args),
        )
    }
    pub unsafe fn GetPDBContents<'a, T0__: ::windows::IntoParam<'a, IDxcBlob>>(
        &self,
        p_pdb_blob: T0__,
        pp_hash: *mut ::std::option::Option<IDxcBlob>,
        pp_container: *mut ::std::option::Option<IDxcBlob>,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).15)(
            ::windows::Abi::abi(self),
            p_pdb_blob.into_param().abi(),
            ::std::mem::transmute(pp_hash),
            ::std::mem::transmute(pp_container),
        )
    }
}
impl ::std::convert::From<IDxcUtils> for ::windows::IUnknown {
    fn from(value: IDxcUtils) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IDxcUtils> for ::windows::IUnknown {
    fn from(value: &IDxcUtils) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::IntoParam<'a, ::windows::IUnknown> for IDxcUtils {
    fn into_param(self) -> ::windows::Param<'a, ::windows::IUnknown> {
        ::windows::Param::Owned(::std::convert::Into::<::windows::IUnknown>::into(self))
    }
}
impl<'a> ::windows::IntoParam<'a, ::windows::IUnknown> for &'a IDxcUtils {
    fn into_param(self) -> ::windows::Param<'a, ::windows::IUnknown> {
        ::windows::Param::Owned(::std::convert::Into::<::windows::IUnknown>::into(
            ::std::clone::Clone::clone(self),
        ))
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDxcUtils_abi(
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        iid: &::windows::Guid,
        interface: *mut ::windows::RawPtr,
    ) -> ::windows::ErrorCode,
    pub unsafe extern "system" fn(this: ::windows::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::RawPtr) -> u32,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        p_blob: ::windows::RawPtr,
        offset: u32,
        length: u32,
        pp_result: *mut ::windows::RawPtr,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        p_data: *mut ::std::ffi::c_void,
        size: u32,
        code_page: u32,
        p_blob_encoding: *mut ::windows::RawPtr,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        p_data: *mut ::std::ffi::c_void,
        p_imalloc: ::windows::RawPtr,
        size: u32,
        code_page: u32,
        p_blob_encoding: *mut ::windows::RawPtr,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        p_data: *mut ::std::ffi::c_void,
        size: u32,
        code_page: u32,
        p_blob_encoding: *mut ::windows::RawPtr,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        p_file_name: *const u16,
        p_code_page: *mut u32,
        p_blob_encoding: *mut ::windows::RawPtr,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        p_blob: ::windows::RawPtr,
        pp_stream: *mut ::windows::RawPtr,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        pp_result: *mut ::windows::RawPtr,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        p_blob: ::windows::RawPtr,
        p_blob_encoding: *mut ::windows::RawPtr,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        p_blob: ::windows::RawPtr,
        p_blob_encoding: *mut ::windows::RawPtr,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        p_shader: *const DxcBuffer,
        dxc_part: u32,
        pp_part_data: *mut *mut ::std::ffi::c_void,
        p_part_size_in_bytes: *mut u32,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        p_data: *const DxcBuffer,
        iid: *const ::windows::Guid,
        ppv_reflection: *mut *mut ::std::ffi::c_void,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        p_source_name: *const u16,
        p_entry_point: *const u16,
        p_target_profile: *const u16,
        p_arguments: *mut *mut u16,
        arg_count: u32,
        p_defines: *const DxcDefine,
        define_count: u32,
        pp_args: *mut ::windows::RawPtr,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        p_pdb_blob: ::windows::RawPtr,
        pp_hash: *mut ::windows::RawPtr,
        pp_container: *mut ::windows::RawPtr,
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
pub struct IDxcValidator(::windows::IUnknown);
impl IDxcValidator {}
unsafe impl ::windows::Interface for IDxcValidator {
    type Vtable = IDxcValidator_abi;
    const IID: ::windows::Guid = ::windows::Guid::zeroed();
}
#[allow(non_snake_case)]
impl IDxcValidator {
    pub unsafe fn Validate<'a, T0__: ::windows::IntoParam<'a, IDxcBlob>>(
        &self,
        p_shader: T0__,
        flags: u32,
        pp_result: *mut ::std::option::Option<IDxcOperationResult>,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).3)(
            ::windows::Abi::abi(self),
            p_shader.into_param().abi(),
            ::std::mem::transmute(flags),
            ::std::mem::transmute(pp_result),
        )
    }
}
impl ::std::convert::From<IDxcValidator> for ::windows::IUnknown {
    fn from(value: IDxcValidator) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IDxcValidator> for ::windows::IUnknown {
    fn from(value: &IDxcValidator) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::IntoParam<'a, ::windows::IUnknown> for IDxcValidator {
    fn into_param(self) -> ::windows::Param<'a, ::windows::IUnknown> {
        ::windows::Param::Owned(::std::convert::Into::<::windows::IUnknown>::into(self))
    }
}
impl<'a> ::windows::IntoParam<'a, ::windows::IUnknown> for &'a IDxcValidator {
    fn into_param(self) -> ::windows::Param<'a, ::windows::IUnknown> {
        ::windows::Param::Owned(::std::convert::Into::<::windows::IUnknown>::into(
            ::std::clone::Clone::clone(self),
        ))
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDxcValidator_abi(
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        iid: &::windows::Guid,
        interface: *mut ::windows::RawPtr,
    ) -> ::windows::ErrorCode,
    pub unsafe extern "system" fn(this: ::windows::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::RawPtr) -> u32,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        p_shader: ::windows::RawPtr,
        flags: u32,
        pp_result: *mut ::windows::RawPtr,
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
pub struct IDxcVersionInfo(::windows::IUnknown);
impl IDxcVersionInfo {}
unsafe impl ::windows::Interface for IDxcVersionInfo {
    type Vtable = IDxcVersionInfo_abi;
    const IID: ::windows::Guid = ::windows::Guid::zeroed();
}
#[allow(non_snake_case)]
impl IDxcVersionInfo {
    pub unsafe fn GetVersion(&self, p_major: *mut u32, p_minor: *mut u32) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).3)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(p_major),
            ::std::mem::transmute(p_minor),
        )
    }
    pub unsafe fn GetFlags(&self, p_flags: *mut u32) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).4)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(p_flags),
        )
    }
}
impl ::std::convert::From<IDxcVersionInfo> for ::windows::IUnknown {
    fn from(value: IDxcVersionInfo) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IDxcVersionInfo> for ::windows::IUnknown {
    fn from(value: &IDxcVersionInfo) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::IntoParam<'a, ::windows::IUnknown> for IDxcVersionInfo {
    fn into_param(self) -> ::windows::Param<'a, ::windows::IUnknown> {
        ::windows::Param::Owned(::std::convert::Into::<::windows::IUnknown>::into(self))
    }
}
impl<'a> ::windows::IntoParam<'a, ::windows::IUnknown> for &'a IDxcVersionInfo {
    fn into_param(self) -> ::windows::Param<'a, ::windows::IUnknown> {
        ::windows::Param::Owned(::std::convert::Into::<::windows::IUnknown>::into(
            ::std::clone::Clone::clone(self),
        ))
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDxcVersionInfo_abi(
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        iid: &::windows::Guid,
        interface: *mut ::windows::RawPtr,
    ) -> ::windows::ErrorCode,
    pub unsafe extern "system" fn(this: ::windows::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::RawPtr) -> u32,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        p_major: *mut u32,
        p_minor: *mut u32,
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
pub struct IDxcVersionInfo2(::windows::IUnknown);
impl IDxcVersionInfo2 {}
unsafe impl ::windows::Interface for IDxcVersionInfo2 {
    type Vtable = IDxcVersionInfo2_abi;
    const IID: ::windows::Guid = ::windows::Guid::zeroed();
}
#[allow(non_snake_case)]
impl IDxcVersionInfo2 {
    pub unsafe fn GetVersion(&self, p_major: *mut u32, p_minor: *mut u32) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).3)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(p_major),
            ::std::mem::transmute(p_minor),
        )
    }
    pub unsafe fn GetFlags(&self, p_flags: *mut u32) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).4)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(p_flags),
        )
    }
    pub unsafe fn GetCommitInfo(
        &self,
        p_commit_count: *mut u32,
        p_commit_hash: *mut *mut i8,
    ) -> ::windows::ErrorCode {
        (::windows::Interface::vtable(self).5)(
            ::windows::Abi::abi(self),
            ::std::mem::transmute(p_commit_count),
            ::std::mem::transmute(p_commit_hash),
        )
    }
}
impl ::std::convert::From<IDxcVersionInfo2> for ::windows::IUnknown {
    fn from(value: IDxcVersionInfo2) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IDxcVersionInfo2> for ::windows::IUnknown {
    fn from(value: &IDxcVersionInfo2) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::IntoParam<'a, ::windows::IUnknown> for IDxcVersionInfo2 {
    fn into_param(self) -> ::windows::Param<'a, ::windows::IUnknown> {
        ::windows::Param::Owned(::std::convert::Into::<::windows::IUnknown>::into(self))
    }
}
impl<'a> ::windows::IntoParam<'a, ::windows::IUnknown> for &'a IDxcVersionInfo2 {
    fn into_param(self) -> ::windows::Param<'a, ::windows::IUnknown> {
        ::windows::Param::Owned(::std::convert::Into::<::windows::IUnknown>::into(
            ::std::clone::Clone::clone(self),
        ))
    }
}
impl ::std::convert::From<IDxcVersionInfo2> for IDxcVersionInfo {
    fn from(value: IDxcVersionInfo2) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IDxcVersionInfo2> for IDxcVersionInfo {
    fn from(value: &IDxcVersionInfo2) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::IntoParam<'a, IDxcVersionInfo> for IDxcVersionInfo2 {
    fn into_param(self) -> ::windows::Param<'a, IDxcVersionInfo> {
        ::windows::Param::Owned(::std::convert::Into::<IDxcVersionInfo>::into(self))
    }
}
impl<'a> ::windows::IntoParam<'a, IDxcVersionInfo> for &'a IDxcVersionInfo2 {
    fn into_param(self) -> ::windows::Param<'a, IDxcVersionInfo> {
        ::windows::Param::Owned(::std::convert::Into::<IDxcVersionInfo>::into(
            ::std::clone::Clone::clone(self),
        ))
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDxcVersionInfo2_abi(
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        iid: &::windows::Guid,
        interface: *mut ::windows::RawPtr,
    ) -> ::windows::ErrorCode,
    pub unsafe extern "system" fn(this: ::windows::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::RawPtr) -> u32,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        p_major: *mut u32,
        p_minor: *mut u32,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        p_flags: *mut u32,
    ) -> ::windows::ErrorCode,
    pub  unsafe extern "system" fn(
        this: ::windows::RawPtr,
        p_commit_count: *mut u32,
        p_commit_hash: *mut *mut i8,
    ) -> ::windows::ErrorCode,
);
