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

#![allow(non_upper_case_globals)]

extern crate aleph_dx12_raw as dx12_raw;
use dx12_raw::windows;

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
#[derive(PartialEq, Eq)]
#[repr(transparent)]
pub struct DXC_OUT_KIND(pub i32);
impl ::std::convert::From<i32> for DXC_OUT_KIND {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
impl ::std::clone::Clone for DXC_OUT_KIND {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}
impl ::std::default::Default for DXC_OUT_KIND {
    fn default() -> Self {
        Self(0)
    }
}
impl ::std::fmt::Debug for DXC_OUT_KIND {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        write!(f, "{:?}", self.0)
    }
}
impl ::std::marker::Copy for DXC_OUT_KIND {}
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
unsafe impl dx12_raw::Abi for DXC_OUT_KIND {
    type Abi = Self;
}
#[repr(C)]
#[allow(non_snake_case)]
pub struct DxcBuffer {
    pub ptr: *mut ::std::ffi::c_void,
    pub size: usize,
    pub encoding: u32,
}
#[repr(C)]
#[doc(hidden)]
pub struct DxcBuffer_abi(*mut ::std::ffi::c_void, usize, u32);
unsafe impl dx12_raw::Abi for DxcBuffer {
    type Abi = DxcBuffer_abi;
}
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
impl ::std::clone::Clone for DxcBuffer {
    fn clone(&self) -> Self {
        Self {
            ptr: <*mut ::std::ffi::c_void as std::clone::Clone>::clone(&self.ptr),
            size: self.size,
            encoding: self.encoding,
        }
    }
}
impl ::std::cmp::PartialEq for DxcBuffer {
    fn eq(&self, other: &Self) -> bool {
        self.ptr == other.ptr && self.size == other.size && self.encoding == other.encoding
    }
}
impl ::std::cmp::Eq for DxcBuffer {}
#[allow(non_camel_case_types)]
pub type DxcCreateInstance2Proc = extern "system" fn(
    p_malloc: ::std::option::Option<windows::win32::com::IMalloc>,
    rclsid: *const dx12_raw::Guid,
    riid: *const dx12_raw::Guid,
    ppv: *mut *mut ::std::ffi::c_void,
) -> dx12_raw::ErrorCode;
#[allow(non_camel_case_types)]
pub type DxcCreateInstanceProc = extern "system" fn(
    rclsid: *const dx12_raw::Guid,
    riid: *const dx12_raw::Guid,
    ppv: *mut *mut ::std::ffi::c_void,
) -> dx12_raw::ErrorCode;
#[repr(C)]
#[allow(non_snake_case)]
pub struct DxcDefine {
    pub name: *mut u16,
    pub value: *mut u16,
}
#[repr(C)]
#[doc(hidden)]
pub struct DxcDefine_abi(*mut u16, *mut u16);
unsafe impl dx12_raw::Abi for DxcDefine {
    type Abi = DxcDefine_abi;
}
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
impl ::std::clone::Clone for DxcDefine {
    fn clone(&self) -> Self {
        Self {
            name: self.name,
            value: self.value,
        }
    }
}
impl ::std::cmp::PartialEq for DxcDefine {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name && self.value == other.value
    }
}
impl ::std::cmp::Eq for DxcDefine {}
#[repr(C)]
#[allow(non_snake_case)]
pub struct DxcShaderHash {
    pub flags: u32,
    pub hash_digest: [u8; 16usize],
}
#[repr(C)]
#[doc(hidden)]
pub struct DxcShaderHash_abi(u32, [u8; 16usize]);
unsafe impl dx12_raw::Abi for DxcShaderHash {
    type Abi = DxcShaderHash_abi;
}
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
impl ::std::clone::Clone for DxcShaderHash {
    fn clone(&self) -> Self {
        Self {
            flags: self.flags,
            hash_digest: <[u8; 16usize] as std::clone::Clone>::clone(&self.hash_digest),
        }
    }
}
impl ::std::cmp::PartialEq for DxcShaderHash {
    fn eq(&self, other: &Self) -> bool {
        self.flags == other.flags && self.hash_digest == other.hash_digest
    }
}
impl ::std::cmp::Eq for DxcShaderHash {}
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
pub struct IDxcBlob(dx12_raw::IUnknown);
impl ::std::clone::Clone for IDxcBlob {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::std::fmt::Debug for IDxcBlob {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        write!(f, "{:?}", self.0)
    }
}
impl ::std::cmp::PartialEq for IDxcBlob {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::std::cmp::Eq for IDxcBlob {}
unsafe impl dx12_raw::Interface for IDxcBlob {
    type Vtable = IDxcBlob_abi;
    const IID: dx12_raw::Guid =
        dx12_raw::Guid::from_values(2342910728, 20885, 16610, [172, 88, 13, 152, 156, 58, 1, 2]);
}
#[repr(C)]
pub struct IDxcBlob_abi(
    pub  unsafe extern "system" fn(
        this: dx12_raw::RawPtr,
        iid: &dx12_raw::Guid,
        interface: *mut dx12_raw::RawPtr,
    ) -> dx12_raw::ErrorCode,
    pub unsafe extern "system" fn(this: dx12_raw::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: dx12_raw::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: dx12_raw::RawPtr) -> *mut ::std::ffi::c_void,
    pub unsafe extern "system" fn(this: dx12_raw::RawPtr) -> usize,
);
#[allow(non_snake_case)]
impl IDxcBlob {
    pub unsafe fn GetBufferPointer(&self) -> *mut ::std::ffi::c_void {
        (dx12_raw::Interface::vtable(self).3)(dx12_raw::Abi::abi(self))
    }
    pub unsafe fn GetBufferSize(&self) -> usize {
        (dx12_raw::Interface::vtable(self).4)(dx12_raw::Abi::abi(self))
    }
}
impl ::std::convert::From<IDxcBlob> for dx12_raw::IUnknown {
    fn from(value: IDxcBlob) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IDxcBlob> for dx12_raw::IUnknown {
    fn from(value: &IDxcBlob) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::std::convert::Into<dx12_raw::Param<'a, dx12_raw::IUnknown>> for IDxcBlob {
    fn into(self) -> dx12_raw::Param<'a, dx12_raw::IUnknown> {
        dx12_raw::Param::Owned(::std::convert::Into::<dx12_raw::IUnknown>::into(self))
    }
}
impl<'a> ::std::convert::Into<dx12_raw::Param<'a, dx12_raw::IUnknown>> for &'a IDxcBlob {
    fn into(self) -> dx12_raw::Param<'a, dx12_raw::IUnknown> {
        dx12_raw::Param::Owned(::std::convert::Into::<dx12_raw::IUnknown>::into(
            ::std::clone::Clone::clone(self),
        ))
    }
}
#[repr(transparent)]
#[allow(non_camel_case_types)]
pub struct IDxcBlobEncoding(dx12_raw::IUnknown);
impl ::std::clone::Clone for IDxcBlobEncoding {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::std::fmt::Debug for IDxcBlobEncoding {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        write!(f, "{:?}", self.0)
    }
}
impl ::std::cmp::PartialEq for IDxcBlobEncoding {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::std::cmp::Eq for IDxcBlobEncoding {}
unsafe impl dx12_raw::Interface for IDxcBlobEncoding {
    type Vtable = IDxcBlobEncoding_abi;
    const IID: dx12_raw::Guid = dx12_raw::Guid::from_values(
        1916916772,
        9798,
        16785,
        [151, 192, 152, 233, 110, 66, 252, 104],
    );
}
#[repr(C)]
pub struct IDxcBlobEncoding_abi(
    pub  unsafe extern "system" fn(
        this: dx12_raw::RawPtr,
        iid: &dx12_raw::Guid,
        interface: *mut dx12_raw::RawPtr,
    ) -> dx12_raw::ErrorCode,
    pub unsafe extern "system" fn(this: dx12_raw::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: dx12_raw::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: dx12_raw::RawPtr) -> *mut ::std::ffi::c_void,
    pub unsafe extern "system" fn(this: dx12_raw::RawPtr) -> usize,
    pub  unsafe extern "system" fn(
        this: dx12_raw::RawPtr,
        p_known: *mut dx12_raw::BOOL,
        p_code_page: *mut u32,
    ) -> dx12_raw::ErrorCode,
);
#[allow(non_snake_case)]
impl IDxcBlobEncoding {
    pub unsafe fn GetBufferPointer(&self) -> *mut ::std::ffi::c_void {
        (dx12_raw::Interface::vtable(self).3)(dx12_raw::Abi::abi(self))
    }
    pub unsafe fn GetBufferSize(&self) -> usize {
        (dx12_raw::Interface::vtable(self).4)(dx12_raw::Abi::abi(self))
    }
    pub unsafe fn GetEncoding(
        &self,
        p_known: *mut dx12_raw::BOOL,
        p_code_page: *mut u32,
    ) -> dx12_raw::ErrorCode {
        (dx12_raw::Interface::vtable(self).5)(dx12_raw::Abi::abi(self), p_known, p_code_page)
    }
}
impl ::std::convert::From<IDxcBlobEncoding> for dx12_raw::IUnknown {
    fn from(value: IDxcBlobEncoding) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IDxcBlobEncoding> for dx12_raw::IUnknown {
    fn from(value: &IDxcBlobEncoding) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::std::convert::Into<dx12_raw::Param<'a, dx12_raw::IUnknown>> for IDxcBlobEncoding {
    fn into(self) -> dx12_raw::Param<'a, dx12_raw::IUnknown> {
        dx12_raw::Param::Owned(::std::convert::Into::<dx12_raw::IUnknown>::into(self))
    }
}
impl<'a> ::std::convert::Into<dx12_raw::Param<'a, dx12_raw::IUnknown>> for &'a IDxcBlobEncoding {
    fn into(self) -> dx12_raw::Param<'a, dx12_raw::IUnknown> {
        dx12_raw::Param::Owned(::std::convert::Into::<dx12_raw::IUnknown>::into(
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
impl<'a> ::std::convert::Into<dx12_raw::Param<'a, IDxcBlob>> for IDxcBlobEncoding {
    fn into(self) -> dx12_raw::Param<'a, IDxcBlob> {
        dx12_raw::Param::Owned(::std::convert::Into::<IDxcBlob>::into(self))
    }
}
impl<'a> ::std::convert::Into<dx12_raw::Param<'a, IDxcBlob>> for &'a IDxcBlobEncoding {
    fn into(self) -> dx12_raw::Param<'a, IDxcBlob> {
        dx12_raw::Param::Owned(::std::convert::Into::<IDxcBlob>::into(
            ::std::clone::Clone::clone(self),
        ))
    }
}
#[repr(transparent)]
#[allow(non_camel_case_types)]
pub struct IDxcOperationResult(dx12_raw::IUnknown);
impl ::std::clone::Clone for IDxcOperationResult {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::std::fmt::Debug for IDxcOperationResult {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        write!(f, "{:?}", self.0)
    }
}
impl ::std::cmp::PartialEq for IDxcOperationResult {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::std::cmp::Eq for IDxcOperationResult {}
unsafe impl dx12_raw::Interface for IDxcOperationResult {
    type Vtable = IDxcOperationResult_abi;
    const IID: dx12_raw::Guid = dx12_raw::Guid::from_values(
        3470477386,
        54505,
        17498,
        [185, 145, 202, 33, 202, 21, 125, 194],
    );
}
#[repr(C)]
pub struct IDxcOperationResult_abi(
    pub  unsafe extern "system" fn(
        this: dx12_raw::RawPtr,
        iid: &dx12_raw::Guid,
        interface: *mut dx12_raw::RawPtr,
    ) -> dx12_raw::ErrorCode,
    pub unsafe extern "system" fn(this: dx12_raw::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: dx12_raw::RawPtr) -> u32,
    pub  unsafe extern "system" fn(
        this: dx12_raw::RawPtr,
        p_status: *mut dx12_raw::ErrorCode,
    ) -> dx12_raw::ErrorCode,
    pub  unsafe extern "system" fn(
        this: dx12_raw::RawPtr,
        pp_result: *mut ::std::option::Option<IDxcBlob>,
    ) -> dx12_raw::ErrorCode,
    pub  unsafe extern "system" fn(
        this: dx12_raw::RawPtr,
        pp_errors: *mut ::std::option::Option<IDxcBlobEncoding>,
    ) -> dx12_raw::ErrorCode,
);
#[allow(non_snake_case)]
impl IDxcOperationResult {
    pub unsafe fn GetStatus(&self, p_status: *mut dx12_raw::ErrorCode) -> dx12_raw::ErrorCode {
        (dx12_raw::Interface::vtable(self).3)(dx12_raw::Abi::abi(self), p_status)
    }
    pub unsafe fn GetResult(
        &self,
        pp_result: *mut ::std::option::Option<IDxcBlob>,
    ) -> dx12_raw::ErrorCode {
        (dx12_raw::Interface::vtable(self).4)(dx12_raw::Abi::abi(self), pp_result)
    }
    pub unsafe fn GetErrorBuffer(
        &self,
        pp_errors: *mut ::std::option::Option<IDxcBlobEncoding>,
    ) -> dx12_raw::ErrorCode {
        (dx12_raw::Interface::vtable(self).5)(dx12_raw::Abi::abi(self), pp_errors)
    }
}
impl ::std::convert::From<IDxcOperationResult> for dx12_raw::IUnknown {
    fn from(value: IDxcOperationResult) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IDxcOperationResult> for dx12_raw::IUnknown {
    fn from(value: &IDxcOperationResult) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::std::convert::Into<dx12_raw::Param<'a, dx12_raw::IUnknown>> for IDxcOperationResult {
    fn into(self) -> dx12_raw::Param<'a, dx12_raw::IUnknown> {
        dx12_raw::Param::Owned(::std::convert::Into::<dx12_raw::IUnknown>::into(self))
    }
}
impl<'a> ::std::convert::Into<dx12_raw::Param<'a, dx12_raw::IUnknown>> for &'a IDxcOperationResult {
    fn into(self) -> dx12_raw::Param<'a, dx12_raw::IUnknown> {
        dx12_raw::Param::Owned(::std::convert::Into::<dx12_raw::IUnknown>::into(
            ::std::clone::Clone::clone(self),
        ))
    }
}
#[repr(transparent)]
#[allow(non_camel_case_types)]
pub struct IDxcAssembler(dx12_raw::IUnknown);
impl ::std::clone::Clone for IDxcAssembler {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::std::fmt::Debug for IDxcAssembler {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        write!(f, "{:?}", self.0)
    }
}
impl ::std::cmp::PartialEq for IDxcAssembler {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::std::cmp::Eq for IDxcAssembler {}
unsafe impl dx12_raw::Interface for IDxcAssembler {
    type Vtable = IDxcAssembler_abi;
    const IID: dx12_raw::Guid = dx12_raw::Guid::from_values(
        153057830,
        7199,
        18760,
        [144, 75, 230, 227, 168, 167, 113, 213],
    );
}
#[repr(C)]
pub struct IDxcAssembler_abi(
    pub  unsafe extern "system" fn(
        this: dx12_raw::RawPtr,
        iid: &dx12_raw::Guid,
        interface: *mut dx12_raw::RawPtr,
    ) -> dx12_raw::ErrorCode,
    pub unsafe extern "system" fn(this: dx12_raw::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: dx12_raw::RawPtr) -> u32,
    pub  unsafe extern "system" fn(
        this: dx12_raw::RawPtr,
        p_shader: dx12_raw::RawPtr,
        pp_result: *mut ::std::option::Option<IDxcOperationResult>,
    ) -> dx12_raw::ErrorCode,
);
#[allow(non_snake_case)]
impl IDxcAssembler {
    pub unsafe fn AssembleToContainer<
        'a,
        T0__: ::std::convert::Into<dx12_raw::Param<'a, IDxcBlob>>,
    >(
        &self,
        p_shader: T0__,
        pp_result: *mut ::std::option::Option<IDxcOperationResult>,
    ) -> dx12_raw::ErrorCode {
        (dx12_raw::Interface::vtable(self).3)(
            dx12_raw::Abi::abi(self),
            p_shader.into().abi(),
            pp_result,
        )
    }
}
impl ::std::convert::From<IDxcAssembler> for dx12_raw::IUnknown {
    fn from(value: IDxcAssembler) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IDxcAssembler> for dx12_raw::IUnknown {
    fn from(value: &IDxcAssembler) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::std::convert::Into<dx12_raw::Param<'a, dx12_raw::IUnknown>> for IDxcAssembler {
    fn into(self) -> dx12_raw::Param<'a, dx12_raw::IUnknown> {
        dx12_raw::Param::Owned(::std::convert::Into::<dx12_raw::IUnknown>::into(self))
    }
}
impl<'a> ::std::convert::Into<dx12_raw::Param<'a, dx12_raw::IUnknown>> for &'a IDxcAssembler {
    fn into(self) -> dx12_raw::Param<'a, dx12_raw::IUnknown> {
        dx12_raw::Param::Owned(::std::convert::Into::<dx12_raw::IUnknown>::into(
            ::std::clone::Clone::clone(self),
        ))
    }
}
#[repr(transparent)]
#[allow(non_camel_case_types)]
pub struct IDxcBlobUtf16(dx12_raw::IUnknown);
impl ::std::clone::Clone for IDxcBlobUtf16 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::std::fmt::Debug for IDxcBlobUtf16 {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        write!(f, "{:?}", self.0)
    }
}
impl ::std::cmp::PartialEq for IDxcBlobUtf16 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::std::cmp::Eq for IDxcBlobUtf16 {}
unsafe impl dx12_raw::Interface for IDxcBlobUtf16 {
    type Vtable = IDxcBlobUtf16_abi;
    const IID: dx12_raw::Guid = dx12_raw::Guid::from_values(
        2750959275,
        4010,
        18814,
        [163, 156, 238, 110, 214, 11, 45, 132],
    );
}
#[repr(C)]
pub struct IDxcBlobUtf16_abi(
    pub  unsafe extern "system" fn(
        this: dx12_raw::RawPtr,
        iid: &dx12_raw::Guid,
        interface: *mut dx12_raw::RawPtr,
    ) -> dx12_raw::ErrorCode,
    pub unsafe extern "system" fn(this: dx12_raw::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: dx12_raw::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: dx12_raw::RawPtr) -> *mut ::std::ffi::c_void,
    pub unsafe extern "system" fn(this: dx12_raw::RawPtr) -> usize,
    pub  unsafe extern "system" fn(
        this: dx12_raw::RawPtr,
        p_known: *mut dx12_raw::BOOL,
        p_code_page: *mut u32,
    ) -> dx12_raw::ErrorCode,
    pub unsafe extern "system" fn(this: dx12_raw::RawPtr) -> *const u16,
    pub unsafe extern "system" fn(this: dx12_raw::RawPtr) -> usize,
);
#[allow(non_snake_case)]
impl IDxcBlobUtf16 {
    pub unsafe fn GetBufferPointer(&self) -> *mut ::std::ffi::c_void {
        (dx12_raw::Interface::vtable(self).3)(dx12_raw::Abi::abi(self))
    }
    pub unsafe fn GetBufferSize(&self) -> usize {
        (dx12_raw::Interface::vtable(self).4)(dx12_raw::Abi::abi(self))
    }
    pub unsafe fn GetEncoding(
        &self,
        p_known: *mut dx12_raw::BOOL,
        p_code_page: *mut u32,
    ) -> dx12_raw::ErrorCode {
        (dx12_raw::Interface::vtable(self).5)(dx12_raw::Abi::abi(self), p_known, p_code_page)
    }
    pub unsafe fn GetStringPointer(&self) -> *const u16 {
        (dx12_raw::Interface::vtable(self).6)(dx12_raw::Abi::abi(self))
    }
    pub unsafe fn GetStringLength(&self) -> usize {
        (dx12_raw::Interface::vtable(self).7)(dx12_raw::Abi::abi(self))
    }
}
impl ::std::convert::From<IDxcBlobUtf16> for dx12_raw::IUnknown {
    fn from(value: IDxcBlobUtf16) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IDxcBlobUtf16> for dx12_raw::IUnknown {
    fn from(value: &IDxcBlobUtf16) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::std::convert::Into<dx12_raw::Param<'a, dx12_raw::IUnknown>> for IDxcBlobUtf16 {
    fn into(self) -> dx12_raw::Param<'a, dx12_raw::IUnknown> {
        dx12_raw::Param::Owned(::std::convert::Into::<dx12_raw::IUnknown>::into(self))
    }
}
impl<'a> ::std::convert::Into<dx12_raw::Param<'a, dx12_raw::IUnknown>> for &'a IDxcBlobUtf16 {
    fn into(self) -> dx12_raw::Param<'a, dx12_raw::IUnknown> {
        dx12_raw::Param::Owned(::std::convert::Into::<dx12_raw::IUnknown>::into(
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
impl<'a> ::std::convert::Into<dx12_raw::Param<'a, IDxcBlobEncoding>> for IDxcBlobUtf16 {
    fn into(self) -> dx12_raw::Param<'a, IDxcBlobEncoding> {
        dx12_raw::Param::Owned(::std::convert::Into::<IDxcBlobEncoding>::into(self))
    }
}
impl<'a> ::std::convert::Into<dx12_raw::Param<'a, IDxcBlobEncoding>> for &'a IDxcBlobUtf16 {
    fn into(self) -> dx12_raw::Param<'a, IDxcBlobEncoding> {
        dx12_raw::Param::Owned(::std::convert::Into::<IDxcBlobEncoding>::into(
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
impl<'a> ::std::convert::Into<dx12_raw::Param<'a, IDxcBlob>> for IDxcBlobUtf16 {
    fn into(self) -> dx12_raw::Param<'a, IDxcBlob> {
        dx12_raw::Param::Owned(::std::convert::Into::<IDxcBlob>::into(self))
    }
}
impl<'a> ::std::convert::Into<dx12_raw::Param<'a, IDxcBlob>> for &'a IDxcBlobUtf16 {
    fn into(self) -> dx12_raw::Param<'a, IDxcBlob> {
        dx12_raw::Param::Owned(::std::convert::Into::<IDxcBlob>::into(
            ::std::clone::Clone::clone(self),
        ))
    }
}
#[repr(transparent)]
#[allow(non_camel_case_types)]
pub struct IDxcBlobUtf8(dx12_raw::IUnknown);
impl ::std::clone::Clone for IDxcBlobUtf8 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::std::fmt::Debug for IDxcBlobUtf8 {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        write!(f, "{:?}", self.0)
    }
}
impl ::std::cmp::PartialEq for IDxcBlobUtf8 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::std::cmp::Eq for IDxcBlobUtf8 {}
unsafe impl dx12_raw::Interface for IDxcBlobUtf8 {
    type Vtable = IDxcBlobUtf8_abi;
    const IID: dx12_raw::Guid =
        dx12_raw::Guid::from_values(1034303177, 47729, 16420, [163, 1, 48, 203, 241, 37, 48, 91]);
}
#[repr(C)]
pub struct IDxcBlobUtf8_abi(
    pub  unsafe extern "system" fn(
        this: dx12_raw::RawPtr,
        iid: &dx12_raw::Guid,
        interface: *mut dx12_raw::RawPtr,
    ) -> dx12_raw::ErrorCode,
    pub unsafe extern "system" fn(this: dx12_raw::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: dx12_raw::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: dx12_raw::RawPtr) -> *mut ::std::ffi::c_void,
    pub unsafe extern "system" fn(this: dx12_raw::RawPtr) -> usize,
    pub  unsafe extern "system" fn(
        this: dx12_raw::RawPtr,
        p_known: *mut dx12_raw::BOOL,
        p_code_page: *mut u32,
    ) -> dx12_raw::ErrorCode,
    pub unsafe extern "system" fn(this: dx12_raw::RawPtr) -> *const i8,
    pub unsafe extern "system" fn(this: dx12_raw::RawPtr) -> usize,
);
#[allow(non_snake_case)]
impl IDxcBlobUtf8 {
    pub unsafe fn GetBufferPointer(&self) -> *mut ::std::ffi::c_void {
        (dx12_raw::Interface::vtable(self).3)(dx12_raw::Abi::abi(self))
    }
    pub unsafe fn GetBufferSize(&self) -> usize {
        (dx12_raw::Interface::vtable(self).4)(dx12_raw::Abi::abi(self))
    }
    pub unsafe fn GetEncoding(
        &self,
        p_known: *mut dx12_raw::BOOL,
        p_code_page: *mut u32,
    ) -> dx12_raw::ErrorCode {
        (dx12_raw::Interface::vtable(self).5)(dx12_raw::Abi::abi(self), p_known, p_code_page)
    }
    pub unsafe fn GetStringPointer(&self) -> *const i8 {
        (dx12_raw::Interface::vtable(self).6)(dx12_raw::Abi::abi(self))
    }
    pub unsafe fn GetStringLength(&self) -> usize {
        (dx12_raw::Interface::vtable(self).7)(dx12_raw::Abi::abi(self))
    }
}
impl ::std::convert::From<IDxcBlobUtf8> for dx12_raw::IUnknown {
    fn from(value: IDxcBlobUtf8) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IDxcBlobUtf8> for dx12_raw::IUnknown {
    fn from(value: &IDxcBlobUtf8) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::std::convert::Into<dx12_raw::Param<'a, dx12_raw::IUnknown>> for IDxcBlobUtf8 {
    fn into(self) -> dx12_raw::Param<'a, dx12_raw::IUnknown> {
        dx12_raw::Param::Owned(::std::convert::Into::<dx12_raw::IUnknown>::into(self))
    }
}
impl<'a> ::std::convert::Into<dx12_raw::Param<'a, dx12_raw::IUnknown>> for &'a IDxcBlobUtf8 {
    fn into(self) -> dx12_raw::Param<'a, dx12_raw::IUnknown> {
        dx12_raw::Param::Owned(::std::convert::Into::<dx12_raw::IUnknown>::into(
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
impl<'a> ::std::convert::Into<dx12_raw::Param<'a, IDxcBlobEncoding>> for IDxcBlobUtf8 {
    fn into(self) -> dx12_raw::Param<'a, IDxcBlobEncoding> {
        dx12_raw::Param::Owned(::std::convert::Into::<IDxcBlobEncoding>::into(self))
    }
}
impl<'a> ::std::convert::Into<dx12_raw::Param<'a, IDxcBlobEncoding>> for &'a IDxcBlobUtf8 {
    fn into(self) -> dx12_raw::Param<'a, IDxcBlobEncoding> {
        dx12_raw::Param::Owned(::std::convert::Into::<IDxcBlobEncoding>::into(
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
impl<'a> ::std::convert::Into<dx12_raw::Param<'a, IDxcBlob>> for IDxcBlobUtf8 {
    fn into(self) -> dx12_raw::Param<'a, IDxcBlob> {
        dx12_raw::Param::Owned(::std::convert::Into::<IDxcBlob>::into(self))
    }
}
impl<'a> ::std::convert::Into<dx12_raw::Param<'a, IDxcBlob>> for &'a IDxcBlobUtf8 {
    fn into(self) -> dx12_raw::Param<'a, IDxcBlob> {
        dx12_raw::Param::Owned(::std::convert::Into::<IDxcBlob>::into(
            ::std::clone::Clone::clone(self),
        ))
    }
}
#[repr(transparent)]
#[allow(non_camel_case_types)]
pub struct IDxcIncludeHandler(dx12_raw::IUnknown);
impl ::std::clone::Clone for IDxcIncludeHandler {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::std::fmt::Debug for IDxcIncludeHandler {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        write!(f, "{:?}", self.0)
    }
}
impl ::std::cmp::PartialEq for IDxcIncludeHandler {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::std::cmp::Eq for IDxcIncludeHandler {}
unsafe impl dx12_raw::Interface for IDxcIncludeHandler {
    type Vtable = IDxcIncludeHandler_abi;
    const IID: dx12_raw::Guid = dx12_raw::Guid::from_values(
        2137128061,
        38157,
        18047,
        [179, 227, 60, 2, 251, 73, 24, 124],
    );
}
#[repr(C)]
pub struct IDxcIncludeHandler_abi(
    pub  unsafe extern "system" fn(
        this: dx12_raw::RawPtr,
        iid: &dx12_raw::Guid,
        interface: *mut dx12_raw::RawPtr,
    ) -> dx12_raw::ErrorCode,
    pub unsafe extern "system" fn(this: dx12_raw::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: dx12_raw::RawPtr) -> u32,
    pub  unsafe extern "system" fn(
        this: dx12_raw::RawPtr,
        p_filename: *const u16,
        pp_include_source: *mut ::std::option::Option<IDxcBlob>,
    ) -> dx12_raw::ErrorCode,
);
#[allow(non_snake_case)]
impl IDxcIncludeHandler {
    pub unsafe fn LoadSource(
        &self,
        p_filename: *const u16,
        pp_include_source: *mut ::std::option::Option<IDxcBlob>,
    ) -> dx12_raw::ErrorCode {
        (dx12_raw::Interface::vtable(self).3)(
            dx12_raw::Abi::abi(self),
            p_filename,
            pp_include_source,
        )
    }
}
impl ::std::convert::From<IDxcIncludeHandler> for dx12_raw::IUnknown {
    fn from(value: IDxcIncludeHandler) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IDxcIncludeHandler> for dx12_raw::IUnknown {
    fn from(value: &IDxcIncludeHandler) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::std::convert::Into<dx12_raw::Param<'a, dx12_raw::IUnknown>> for IDxcIncludeHandler {
    fn into(self) -> dx12_raw::Param<'a, dx12_raw::IUnknown> {
        dx12_raw::Param::Owned(::std::convert::Into::<dx12_raw::IUnknown>::into(self))
    }
}
impl<'a> ::std::convert::Into<dx12_raw::Param<'a, dx12_raw::IUnknown>> for &'a IDxcIncludeHandler {
    fn into(self) -> dx12_raw::Param<'a, dx12_raw::IUnknown> {
        dx12_raw::Param::Owned(::std::convert::Into::<dx12_raw::IUnknown>::into(
            ::std::clone::Clone::clone(self),
        ))
    }
}
#[repr(transparent)]
#[allow(non_camel_case_types)]
pub struct IDxcCompiler(dx12_raw::IUnknown);
impl ::std::clone::Clone for IDxcCompiler {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::std::fmt::Debug for IDxcCompiler {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        write!(f, "{:?}", self.0)
    }
}
impl ::std::cmp::PartialEq for IDxcCompiler {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::std::cmp::Eq for IDxcCompiler {}
unsafe impl dx12_raw::Interface for IDxcCompiler {
    type Vtable = IDxcCompiler_abi;
    const IID: dx12_raw::Guid = dx12_raw::Guid::from_values(
        2350975987,
        287,
        17442,
        [141, 112, 111, 154, 203, 141, 182, 23],
    );
}
#[repr(C)]
pub struct IDxcCompiler_abi(
    pub  unsafe extern "system" fn(
        this: dx12_raw::RawPtr,
        iid: &dx12_raw::Guid,
        interface: *mut dx12_raw::RawPtr,
    ) -> dx12_raw::ErrorCode,
    pub unsafe extern "system" fn(this: dx12_raw::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: dx12_raw::RawPtr) -> u32,
    pub  unsafe extern "system" fn(
        this: dx12_raw::RawPtr,
        p_source: dx12_raw::RawPtr,
        p_source_name: *const u16,
        p_entry_point: *const u16,
        p_target_profile: *const u16,
        p_arguments: *mut *mut u16,
        arg_count: u32,
        p_defines: *const DxcDefine,
        define_count: u32,
        p_include_handler: dx12_raw::RawPtr,
        pp_result: *mut ::std::option::Option<IDxcOperationResult>,
    ) -> dx12_raw::ErrorCode,
    pub  unsafe extern "system" fn(
        this: dx12_raw::RawPtr,
        p_source: dx12_raw::RawPtr,
        p_source_name: *const u16,
        p_arguments: *mut *mut u16,
        arg_count: u32,
        p_defines: *const DxcDefine,
        define_count: u32,
        p_include_handler: dx12_raw::RawPtr,
        pp_result: *mut ::std::option::Option<IDxcOperationResult>,
    ) -> dx12_raw::ErrorCode,
    pub  unsafe extern "system" fn(
        this: dx12_raw::RawPtr,
        p_source: dx12_raw::RawPtr,
        pp_disassembly: *mut ::std::option::Option<IDxcBlobEncoding>,
    ) -> dx12_raw::ErrorCode,
);
#[allow(non_snake_case)]
impl IDxcCompiler {
    pub unsafe fn Compile<
        'a,
        T0__: ::std::convert::Into<dx12_raw::Param<'a, IDxcBlob>>,
        T8__: ::std::convert::Into<dx12_raw::Param<'a, IDxcIncludeHandler>>,
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
    ) -> dx12_raw::ErrorCode {
        (dx12_raw::Interface::vtable(self).3)(
            dx12_raw::Abi::abi(self),
            p_source.into().abi(),
            p_source_name,
            p_entry_point,
            p_target_profile,
            p_arguments,
            arg_count,
            p_defines,
            define_count,
            p_include_handler.into().abi(),
            pp_result,
        )
    }
    pub unsafe fn Preprocess<
        'a,
        T0__: ::std::convert::Into<dx12_raw::Param<'a, IDxcBlob>>,
        T6__: ::std::convert::Into<dx12_raw::Param<'a, IDxcIncludeHandler>>,
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
    ) -> dx12_raw::ErrorCode {
        (dx12_raw::Interface::vtable(self).4)(
            dx12_raw::Abi::abi(self),
            p_source.into().abi(),
            p_source_name,
            p_arguments,
            arg_count,
            p_defines,
            define_count,
            p_include_handler.into().abi(),
            pp_result,
        )
    }
    pub unsafe fn Disassemble<'a, T0__: ::std::convert::Into<dx12_raw::Param<'a, IDxcBlob>>>(
        &self,
        p_source: T0__,
        pp_disassembly: *mut ::std::option::Option<IDxcBlobEncoding>,
    ) -> dx12_raw::ErrorCode {
        (dx12_raw::Interface::vtable(self).5)(
            dx12_raw::Abi::abi(self),
            p_source.into().abi(),
            pp_disassembly,
        )
    }
}
impl ::std::convert::From<IDxcCompiler> for dx12_raw::IUnknown {
    fn from(value: IDxcCompiler) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IDxcCompiler> for dx12_raw::IUnknown {
    fn from(value: &IDxcCompiler) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::std::convert::Into<dx12_raw::Param<'a, dx12_raw::IUnknown>> for IDxcCompiler {
    fn into(self) -> dx12_raw::Param<'a, dx12_raw::IUnknown> {
        dx12_raw::Param::Owned(::std::convert::Into::<dx12_raw::IUnknown>::into(self))
    }
}
impl<'a> ::std::convert::Into<dx12_raw::Param<'a, dx12_raw::IUnknown>> for &'a IDxcCompiler {
    fn into(self) -> dx12_raw::Param<'a, dx12_raw::IUnknown> {
        dx12_raw::Param::Owned(::std::convert::Into::<dx12_raw::IUnknown>::into(
            ::std::clone::Clone::clone(self),
        ))
    }
}
#[repr(transparent)]
#[allow(non_camel_case_types)]
pub struct IDxcCompiler2(dx12_raw::IUnknown);
impl ::std::clone::Clone for IDxcCompiler2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::std::fmt::Debug for IDxcCompiler2 {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        write!(f, "{:?}", self.0)
    }
}
impl ::std::cmp::PartialEq for IDxcCompiler2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::std::cmp::Eq for IDxcCompiler2 {}
unsafe impl dx12_raw::Interface for IDxcCompiler2 {
    type Vtable = IDxcCompiler2_abi;
    const IID: dx12_raw::Guid = dx12_raw::Guid::from_values(
        2684725721,
        47291,
        17812,
        [181, 201, 14, 99, 59, 236, 77, 55],
    );
}
#[repr(C)]
pub struct IDxcCompiler2_abi(
    pub  unsafe extern "system" fn(
        this: dx12_raw::RawPtr,
        iid: &dx12_raw::Guid,
        interface: *mut dx12_raw::RawPtr,
    ) -> dx12_raw::ErrorCode,
    pub unsafe extern "system" fn(this: dx12_raw::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: dx12_raw::RawPtr) -> u32,
    pub  unsafe extern "system" fn(
        this: dx12_raw::RawPtr,
        p_source: dx12_raw::RawPtr,
        p_source_name: *const u16,
        p_entry_point: *const u16,
        p_target_profile: *const u16,
        p_arguments: *mut *mut u16,
        arg_count: u32,
        p_defines: *const DxcDefine,
        define_count: u32,
        p_include_handler: dx12_raw::RawPtr,
        pp_result: *mut ::std::option::Option<IDxcOperationResult>,
    ) -> dx12_raw::ErrorCode,
    pub  unsafe extern "system" fn(
        this: dx12_raw::RawPtr,
        p_source: dx12_raw::RawPtr,
        p_source_name: *const u16,
        p_arguments: *mut *mut u16,
        arg_count: u32,
        p_defines: *const DxcDefine,
        define_count: u32,
        p_include_handler: dx12_raw::RawPtr,
        pp_result: *mut ::std::option::Option<IDxcOperationResult>,
    ) -> dx12_raw::ErrorCode,
    pub  unsafe extern "system" fn(
        this: dx12_raw::RawPtr,
        p_source: dx12_raw::RawPtr,
        pp_disassembly: *mut ::std::option::Option<IDxcBlobEncoding>,
    ) -> dx12_raw::ErrorCode,
    pub  unsafe extern "system" fn(
        this: dx12_raw::RawPtr,
        p_source: dx12_raw::RawPtr,
        p_source_name: *const u16,
        p_entry_point: *const u16,
        p_target_profile: *const u16,
        p_arguments: *mut *mut u16,
        arg_count: u32,
        p_defines: *const DxcDefine,
        define_count: u32,
        p_include_handler: dx12_raw::RawPtr,
        pp_result: *mut ::std::option::Option<IDxcOperationResult>,
        pp_debug_blob_name: *mut *mut u16,
        pp_debug_blob: *mut ::std::option::Option<IDxcBlob>,
    ) -> dx12_raw::ErrorCode,
);
#[allow(non_snake_case)]
impl IDxcCompiler2 {
    pub unsafe fn Compile<
        'a,
        T0__: ::std::convert::Into<dx12_raw::Param<'a, IDxcBlob>>,
        T8__: ::std::convert::Into<dx12_raw::Param<'a, IDxcIncludeHandler>>,
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
    ) -> dx12_raw::ErrorCode {
        (dx12_raw::Interface::vtable(self).3)(
            dx12_raw::Abi::abi(self),
            p_source.into().abi(),
            p_source_name,
            p_entry_point,
            p_target_profile,
            p_arguments,
            arg_count,
            p_defines,
            define_count,
            p_include_handler.into().abi(),
            pp_result,
        )
    }
    pub unsafe fn Preprocess<
        'a,
        T0__: ::std::convert::Into<dx12_raw::Param<'a, IDxcBlob>>,
        T6__: ::std::convert::Into<dx12_raw::Param<'a, IDxcIncludeHandler>>,
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
    ) -> dx12_raw::ErrorCode {
        (dx12_raw::Interface::vtable(self).4)(
            dx12_raw::Abi::abi(self),
            p_source.into().abi(),
            p_source_name,
            p_arguments,
            arg_count,
            p_defines,
            define_count,
            p_include_handler.into().abi(),
            pp_result,
        )
    }
    pub unsafe fn Disassemble<'a, T0__: ::std::convert::Into<dx12_raw::Param<'a, IDxcBlob>>>(
        &self,
        p_source: T0__,
        pp_disassembly: *mut ::std::option::Option<IDxcBlobEncoding>,
    ) -> dx12_raw::ErrorCode {
        (dx12_raw::Interface::vtable(self).5)(
            dx12_raw::Abi::abi(self),
            p_source.into().abi(),
            pp_disassembly,
        )
    }
    pub unsafe fn CompileWithDebug<
        'a,
        T0__: ::std::convert::Into<dx12_raw::Param<'a, IDxcBlob>>,
        T8__: ::std::convert::Into<dx12_raw::Param<'a, IDxcIncludeHandler>>,
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
    ) -> dx12_raw::ErrorCode {
        (dx12_raw::Interface::vtable(self).6)(
            dx12_raw::Abi::abi(self),
            p_source.into().abi(),
            p_source_name,
            p_entry_point,
            p_target_profile,
            p_arguments,
            arg_count,
            p_defines,
            define_count,
            p_include_handler.into().abi(),
            pp_result,
            pp_debug_blob_name,
            pp_debug_blob,
        )
    }
}
impl ::std::convert::From<IDxcCompiler2> for dx12_raw::IUnknown {
    fn from(value: IDxcCompiler2) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IDxcCompiler2> for dx12_raw::IUnknown {
    fn from(value: &IDxcCompiler2) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::std::convert::Into<dx12_raw::Param<'a, dx12_raw::IUnknown>> for IDxcCompiler2 {
    fn into(self) -> dx12_raw::Param<'a, dx12_raw::IUnknown> {
        dx12_raw::Param::Owned(::std::convert::Into::<dx12_raw::IUnknown>::into(self))
    }
}
impl<'a> ::std::convert::Into<dx12_raw::Param<'a, dx12_raw::IUnknown>> for &'a IDxcCompiler2 {
    fn into(self) -> dx12_raw::Param<'a, dx12_raw::IUnknown> {
        dx12_raw::Param::Owned(::std::convert::Into::<dx12_raw::IUnknown>::into(
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
impl<'a> ::std::convert::Into<dx12_raw::Param<'a, IDxcCompiler>> for IDxcCompiler2 {
    fn into(self) -> dx12_raw::Param<'a, IDxcCompiler> {
        dx12_raw::Param::Owned(::std::convert::Into::<IDxcCompiler>::into(self))
    }
}
impl<'a> ::std::convert::Into<dx12_raw::Param<'a, IDxcCompiler>> for &'a IDxcCompiler2 {
    fn into(self) -> dx12_raw::Param<'a, IDxcCompiler> {
        dx12_raw::Param::Owned(::std::convert::Into::<IDxcCompiler>::into(
            ::std::clone::Clone::clone(self),
        ))
    }
}
#[repr(transparent)]
#[allow(non_camel_case_types)]
pub struct IDxcCompiler3(dx12_raw::IUnknown);
impl ::std::clone::Clone for IDxcCompiler3 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::std::fmt::Debug for IDxcCompiler3 {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        write!(f, "{:?}", self.0)
    }
}
impl ::std::cmp::PartialEq for IDxcCompiler3 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::std::cmp::Eq for IDxcCompiler3 {}
unsafe impl dx12_raw::Interface for IDxcCompiler3 {
    type Vtable = IDxcCompiler3_abi;
    const IID: dx12_raw::Guid =
        dx12_raw::Guid::from_values(579552903, 23146, 18224, [144, 12, 151, 2, 178, 32, 63, 84]);
}
#[repr(C)]
pub struct IDxcCompiler3_abi(
    pub  unsafe extern "system" fn(
        this: dx12_raw::RawPtr,
        iid: &dx12_raw::Guid,
        interface: *mut dx12_raw::RawPtr,
    ) -> dx12_raw::ErrorCode,
    pub unsafe extern "system" fn(this: dx12_raw::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: dx12_raw::RawPtr) -> u32,
    pub  unsafe extern "system" fn(
        this: dx12_raw::RawPtr,
        p_source: *const DxcBuffer,
        p_arguments: *mut *mut u16,
        arg_count: u32,
        p_include_handler: dx12_raw::RawPtr,
        riid: *const dx12_raw::Guid,
        pp_result: *mut *mut ::std::ffi::c_void,
    ) -> dx12_raw::ErrorCode,
    pub  unsafe extern "system" fn(
        this: dx12_raw::RawPtr,
        p_object: *const DxcBuffer,
        riid: *const dx12_raw::Guid,
        pp_result: *mut *mut ::std::ffi::c_void,
    ) -> dx12_raw::ErrorCode,
);
#[allow(non_snake_case)]
impl IDxcCompiler3 {
    pub unsafe fn Compile<
        'a,
        T3__: ::std::convert::Into<dx12_raw::Param<'a, IDxcIncludeHandler>>,
    >(
        &self,
        p_source: *const DxcBuffer,
        p_arguments: *mut *mut u16,
        arg_count: u32,
        p_include_handler: T3__,
        riid: *const dx12_raw::Guid,
        pp_result: *mut *mut ::std::ffi::c_void,
    ) -> dx12_raw::ErrorCode {
        (dx12_raw::Interface::vtable(self).3)(
            dx12_raw::Abi::abi(self),
            p_source,
            p_arguments,
            arg_count,
            p_include_handler.into().abi(),
            riid,
            pp_result,
        )
    }
    pub unsafe fn Disassemble(
        &self,
        p_object: *const DxcBuffer,
        riid: *const dx12_raw::Guid,
        pp_result: *mut *mut ::std::ffi::c_void,
    ) -> dx12_raw::ErrorCode {
        (dx12_raw::Interface::vtable(self).4)(dx12_raw::Abi::abi(self), p_object, riid, pp_result)
    }
}
impl ::std::convert::From<IDxcCompiler3> for dx12_raw::IUnknown {
    fn from(value: IDxcCompiler3) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IDxcCompiler3> for dx12_raw::IUnknown {
    fn from(value: &IDxcCompiler3) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::std::convert::Into<dx12_raw::Param<'a, dx12_raw::IUnknown>> for IDxcCompiler3 {
    fn into(self) -> dx12_raw::Param<'a, dx12_raw::IUnknown> {
        dx12_raw::Param::Owned(::std::convert::Into::<dx12_raw::IUnknown>::into(self))
    }
}
impl<'a> ::std::convert::Into<dx12_raw::Param<'a, dx12_raw::IUnknown>> for &'a IDxcCompiler3 {
    fn into(self) -> dx12_raw::Param<'a, dx12_raw::IUnknown> {
        dx12_raw::Param::Owned(::std::convert::Into::<dx12_raw::IUnknown>::into(
            ::std::clone::Clone::clone(self),
        ))
    }
}
#[repr(transparent)]
#[allow(non_camel_case_types)]
pub struct IDxcCompilerArgs(dx12_raw::IUnknown);
impl ::std::clone::Clone for IDxcCompilerArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::std::fmt::Debug for IDxcCompilerArgs {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        write!(f, "{:?}", self.0)
    }
}
impl ::std::cmp::PartialEq for IDxcCompilerArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::std::cmp::Eq for IDxcCompilerArgs {}
unsafe impl dx12_raw::Interface for IDxcCompilerArgs {
    type Vtable = IDxcCompilerArgs_abi;
    const IID: dx12_raw::Guid = dx12_raw::Guid::from_values(
        1945108010,
        28892,
        17912,
        [150, 144, 239, 246, 76, 2, 66, 157],
    );
}
#[repr(C)]
pub struct IDxcCompilerArgs_abi(
    pub  unsafe extern "system" fn(
        this: dx12_raw::RawPtr,
        iid: &dx12_raw::Guid,
        interface: *mut dx12_raw::RawPtr,
    ) -> dx12_raw::ErrorCode,
    pub unsafe extern "system" fn(this: dx12_raw::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: dx12_raw::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: dx12_raw::RawPtr) -> *mut *mut u16,
    pub unsafe extern "system" fn(this: dx12_raw::RawPtr) -> u32,
    pub  unsafe extern "system" fn(
        this: dx12_raw::RawPtr,
        p_arguments: *mut *mut u16,
        arg_count: u32,
    ) -> dx12_raw::ErrorCode,
    pub  unsafe extern "system" fn(
        this: dx12_raw::RawPtr,
        p_arguments: *mut *mut i8,
        arg_count: u32,
    ) -> dx12_raw::ErrorCode,
    pub  unsafe extern "system" fn(
        this: dx12_raw::RawPtr,
        p_defines: *const DxcDefine,
        define_count: u32,
    ) -> dx12_raw::ErrorCode,
);
#[allow(non_snake_case)]
impl IDxcCompilerArgs {
    pub unsafe fn GetArguments(&self) -> *mut *mut u16 {
        (dx12_raw::Interface::vtable(self).3)(dx12_raw::Abi::abi(self))
    }
    pub unsafe fn GetCount(&self) -> u32 {
        (dx12_raw::Interface::vtable(self).4)(dx12_raw::Abi::abi(self))
    }
    pub unsafe fn AddArguments(
        &self,
        p_arguments: *mut *mut u16,
        arg_count: u32,
    ) -> dx12_raw::ErrorCode {
        (dx12_raw::Interface::vtable(self).5)(dx12_raw::Abi::abi(self), p_arguments, arg_count)
    }
    pub unsafe fn AddArgumentsUTF8(
        &self,
        p_arguments: *mut *mut i8,
        arg_count: u32,
    ) -> dx12_raw::ErrorCode {
        (dx12_raw::Interface::vtable(self).6)(dx12_raw::Abi::abi(self), p_arguments, arg_count)
    }
    pub unsafe fn AddDefines(
        &self,
        p_defines: *const DxcDefine,
        define_count: u32,
    ) -> dx12_raw::ErrorCode {
        (dx12_raw::Interface::vtable(self).7)(dx12_raw::Abi::abi(self), p_defines, define_count)
    }
}
impl ::std::convert::From<IDxcCompilerArgs> for dx12_raw::IUnknown {
    fn from(value: IDxcCompilerArgs) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IDxcCompilerArgs> for dx12_raw::IUnknown {
    fn from(value: &IDxcCompilerArgs) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::std::convert::Into<dx12_raw::Param<'a, dx12_raw::IUnknown>> for IDxcCompilerArgs {
    fn into(self) -> dx12_raw::Param<'a, dx12_raw::IUnknown> {
        dx12_raw::Param::Owned(::std::convert::Into::<dx12_raw::IUnknown>::into(self))
    }
}
impl<'a> ::std::convert::Into<dx12_raw::Param<'a, dx12_raw::IUnknown>> for &'a IDxcCompilerArgs {
    fn into(self) -> dx12_raw::Param<'a, dx12_raw::IUnknown> {
        dx12_raw::Param::Owned(::std::convert::Into::<dx12_raw::IUnknown>::into(
            ::std::clone::Clone::clone(self),
        ))
    }
}
#[repr(transparent)]
#[allow(non_camel_case_types)]
pub struct IDxcContainerBuilder(dx12_raw::IUnknown);
impl ::std::clone::Clone for IDxcContainerBuilder {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::std::fmt::Debug for IDxcContainerBuilder {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        write!(f, "{:?}", self.0)
    }
}
impl ::std::cmp::PartialEq for IDxcContainerBuilder {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::std::cmp::Eq for IDxcContainerBuilder {}
unsafe impl dx12_raw::Interface for IDxcContainerBuilder {
    type Vtable = IDxcContainerBuilder_abi;
    const IID: dx12_raw::Guid = dx12_raw::Guid::from_values(
        860561232,
        8850,
        19253,
        [153, 161, 37, 88, 141, 140, 23, 254],
    );
}
#[repr(C)]
pub struct IDxcContainerBuilder_abi(
    pub  unsafe extern "system" fn(
        this: dx12_raw::RawPtr,
        iid: &dx12_raw::Guid,
        interface: *mut dx12_raw::RawPtr,
    ) -> dx12_raw::ErrorCode,
    pub unsafe extern "system" fn(this: dx12_raw::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: dx12_raw::RawPtr) -> u32,
    pub  unsafe extern "system" fn(
        this: dx12_raw::RawPtr,
        p_dxil_container_header: dx12_raw::RawPtr,
    ) -> dx12_raw::ErrorCode,
    pub  unsafe extern "system" fn(
        this: dx12_raw::RawPtr,
        four_cc: u32,
        p_source: dx12_raw::RawPtr,
    ) -> dx12_raw::ErrorCode,
    pub unsafe extern "system" fn(this: dx12_raw::RawPtr, four_cc: u32) -> dx12_raw::ErrorCode,
    pub  unsafe extern "system" fn(
        this: dx12_raw::RawPtr,
        pp_result: *mut ::std::option::Option<IDxcOperationResult>,
    ) -> dx12_raw::ErrorCode,
);
#[allow(non_snake_case)]
impl IDxcContainerBuilder {
    pub unsafe fn Load<'a, T0__: ::std::convert::Into<dx12_raw::Param<'a, IDxcBlob>>>(
        &self,
        p_dxil_container_header: T0__,
    ) -> dx12_raw::ErrorCode {
        (dx12_raw::Interface::vtable(self).3)(
            dx12_raw::Abi::abi(self),
            p_dxil_container_header.into().abi(),
        )
    }
    pub unsafe fn AddPart<'a, T1__: ::std::convert::Into<dx12_raw::Param<'a, IDxcBlob>>>(
        &self,
        four_cc: u32,
        p_source: T1__,
    ) -> dx12_raw::ErrorCode {
        (dx12_raw::Interface::vtable(self).4)(
            dx12_raw::Abi::abi(self),
            four_cc,
            p_source.into().abi(),
        )
    }
    pub unsafe fn RemovePart(&self, four_cc: u32) -> dx12_raw::ErrorCode {
        (dx12_raw::Interface::vtable(self).5)(dx12_raw::Abi::abi(self), four_cc)
    }
    pub unsafe fn SerializeContainer(
        &self,
        pp_result: *mut ::std::option::Option<IDxcOperationResult>,
    ) -> dx12_raw::ErrorCode {
        (dx12_raw::Interface::vtable(self).6)(dx12_raw::Abi::abi(self), pp_result)
    }
}
impl ::std::convert::From<IDxcContainerBuilder> for dx12_raw::IUnknown {
    fn from(value: IDxcContainerBuilder) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IDxcContainerBuilder> for dx12_raw::IUnknown {
    fn from(value: &IDxcContainerBuilder) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::std::convert::Into<dx12_raw::Param<'a, dx12_raw::IUnknown>> for IDxcContainerBuilder {
    fn into(self) -> dx12_raw::Param<'a, dx12_raw::IUnknown> {
        dx12_raw::Param::Owned(::std::convert::Into::<dx12_raw::IUnknown>::into(self))
    }
}
impl<'a> ::std::convert::Into<dx12_raw::Param<'a, dx12_raw::IUnknown>>
    for &'a IDxcContainerBuilder
{
    fn into(self) -> dx12_raw::Param<'a, dx12_raw::IUnknown> {
        dx12_raw::Param::Owned(::std::convert::Into::<dx12_raw::IUnknown>::into(
            ::std::clone::Clone::clone(self),
        ))
    }
}
#[repr(transparent)]
#[allow(non_camel_case_types)]
pub struct IDxcContainerReflection(dx12_raw::IUnknown);
impl ::std::clone::Clone for IDxcContainerReflection {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::std::fmt::Debug for IDxcContainerReflection {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        write!(f, "{:?}", self.0)
    }
}
impl ::std::cmp::PartialEq for IDxcContainerReflection {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::std::cmp::Eq for IDxcContainerReflection {}
unsafe impl dx12_raw::Interface for IDxcContainerReflection {
    type Vtable = IDxcContainerReflection_abi;
    const IID: dx12_raw::Guid = dx12_raw::Guid::from_values(
        3535936294,
        33616,
        19420,
        [151, 106, 51, 28, 230, 244, 197, 76],
    );
}
#[repr(C)]
pub struct IDxcContainerReflection_abi(
    pub  unsafe extern "system" fn(
        this: dx12_raw::RawPtr,
        iid: &dx12_raw::Guid,
        interface: *mut dx12_raw::RawPtr,
    ) -> dx12_raw::ErrorCode,
    pub unsafe extern "system" fn(this: dx12_raw::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: dx12_raw::RawPtr) -> u32,
    pub  unsafe extern "system" fn(
        this: dx12_raw::RawPtr,
        p_container: dx12_raw::RawPtr,
    ) -> dx12_raw::ErrorCode,
    pub unsafe extern "system" fn(this: dx12_raw::RawPtr, p_result: *mut u32) -> dx12_raw::ErrorCode,
    pub  unsafe extern "system" fn(
        this: dx12_raw::RawPtr,
        idx: u32,
        p_result: *mut u32,
    ) -> dx12_raw::ErrorCode,
    pub  unsafe extern "system" fn(
        this: dx12_raw::RawPtr,
        idx: u32,
        pp_result: *mut ::std::option::Option<IDxcBlob>,
    ) -> dx12_raw::ErrorCode,
    pub  unsafe extern "system" fn(
        this: dx12_raw::RawPtr,
        kind: u32,
        p_result: *mut u32,
    ) -> dx12_raw::ErrorCode,
    pub  unsafe extern "system" fn(
        this: dx12_raw::RawPtr,
        idx: u32,
        iid: *const dx12_raw::Guid,
        ppv_object: *mut *mut ::std::ffi::c_void,
    ) -> dx12_raw::ErrorCode,
);
#[allow(non_snake_case)]
impl IDxcContainerReflection {
    pub unsafe fn Load<'a, T0__: ::std::convert::Into<dx12_raw::Param<'a, IDxcBlob>>>(
        &self,
        p_container: T0__,
    ) -> dx12_raw::ErrorCode {
        (dx12_raw::Interface::vtable(self).3)(dx12_raw::Abi::abi(self), p_container.into().abi())
    }
    pub unsafe fn GetPartCount(&self, p_result: *mut u32) -> dx12_raw::ErrorCode {
        (dx12_raw::Interface::vtable(self).4)(dx12_raw::Abi::abi(self), p_result)
    }
    pub unsafe fn GetPartKind(&self, idx: u32, p_result: *mut u32) -> dx12_raw::ErrorCode {
        (dx12_raw::Interface::vtable(self).5)(dx12_raw::Abi::abi(self), idx, p_result)
    }
    pub unsafe fn GetPartContent(
        &self,
        idx: u32,
        pp_result: *mut ::std::option::Option<IDxcBlob>,
    ) -> dx12_raw::ErrorCode {
        (dx12_raw::Interface::vtable(self).6)(dx12_raw::Abi::abi(self), idx, pp_result)
    }
    pub unsafe fn FindFirstPartKind(&self, kind: u32, p_result: *mut u32) -> dx12_raw::ErrorCode {
        (dx12_raw::Interface::vtable(self).7)(dx12_raw::Abi::abi(self), kind, p_result)
    }
    pub unsafe fn GetPartReflection(
        &self,
        idx: u32,
        iid: *const dx12_raw::Guid,
        ppv_object: *mut *mut ::std::ffi::c_void,
    ) -> dx12_raw::ErrorCode {
        (dx12_raw::Interface::vtable(self).8)(dx12_raw::Abi::abi(self), idx, iid, ppv_object)
    }
}
impl ::std::convert::From<IDxcContainerReflection> for dx12_raw::IUnknown {
    fn from(value: IDxcContainerReflection) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IDxcContainerReflection> for dx12_raw::IUnknown {
    fn from(value: &IDxcContainerReflection) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::std::convert::Into<dx12_raw::Param<'a, dx12_raw::IUnknown>> for IDxcContainerReflection {
    fn into(self) -> dx12_raw::Param<'a, dx12_raw::IUnknown> {
        dx12_raw::Param::Owned(::std::convert::Into::<dx12_raw::IUnknown>::into(self))
    }
}
impl<'a> ::std::convert::Into<dx12_raw::Param<'a, dx12_raw::IUnknown>>
    for &'a IDxcContainerReflection
{
    fn into(self) -> dx12_raw::Param<'a, dx12_raw::IUnknown> {
        dx12_raw::Param::Owned(::std::convert::Into::<dx12_raw::IUnknown>::into(
            ::std::clone::Clone::clone(self),
        ))
    }
}
#[repr(transparent)]
#[allow(non_camel_case_types)]
pub struct IDxcExtraOutputs(dx12_raw::IUnknown);
impl ::std::clone::Clone for IDxcExtraOutputs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::std::fmt::Debug for IDxcExtraOutputs {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        write!(f, "{:?}", self.0)
    }
}
impl ::std::cmp::PartialEq for IDxcExtraOutputs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::std::cmp::Eq for IDxcExtraOutputs {}
unsafe impl dx12_raw::Interface for IDxcExtraOutputs {
    type Vtable = IDxcExtraOutputs_abi;
    const IID: dx12_raw::Guid = dx12_raw::Guid::from_values(
        832255906,
        42434,
        18762,
        [165, 222, 72, 1, 178, 250, 249, 137],
    );
}
#[repr(C)]
pub struct IDxcExtraOutputs_abi(
    pub  unsafe extern "system" fn(
        this: dx12_raw::RawPtr,
        iid: &dx12_raw::Guid,
        interface: *mut dx12_raw::RawPtr,
    ) -> dx12_raw::ErrorCode,
    pub unsafe extern "system" fn(this: dx12_raw::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: dx12_raw::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: dx12_raw::RawPtr) -> u32,
    pub  unsafe extern "system" fn(
        this: dx12_raw::RawPtr,
        u_index: u32,
        iid: *const dx12_raw::Guid,
        ppv_object: *mut *mut ::std::ffi::c_void,
        pp_output_type: *mut ::std::option::Option<IDxcBlobUtf16>,
        pp_output_name: *mut ::std::option::Option<IDxcBlobUtf16>,
    ) -> dx12_raw::ErrorCode,
);
#[allow(non_snake_case)]
impl IDxcExtraOutputs {
    pub unsafe fn GetOutputCount(&self) -> u32 {
        (dx12_raw::Interface::vtable(self).3)(dx12_raw::Abi::abi(self))
    }
    pub unsafe fn GetOutput(
        &self,
        u_index: u32,
        iid: *const dx12_raw::Guid,
        ppv_object: *mut *mut ::std::ffi::c_void,
        pp_output_type: *mut ::std::option::Option<IDxcBlobUtf16>,
        pp_output_name: *mut ::std::option::Option<IDxcBlobUtf16>,
    ) -> dx12_raw::ErrorCode {
        (dx12_raw::Interface::vtable(self).4)(
            dx12_raw::Abi::abi(self),
            u_index,
            iid,
            ppv_object,
            pp_output_type,
            pp_output_name,
        )
    }
}
impl ::std::convert::From<IDxcExtraOutputs> for dx12_raw::IUnknown {
    fn from(value: IDxcExtraOutputs) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IDxcExtraOutputs> for dx12_raw::IUnknown {
    fn from(value: &IDxcExtraOutputs) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::std::convert::Into<dx12_raw::Param<'a, dx12_raw::IUnknown>> for IDxcExtraOutputs {
    fn into(self) -> dx12_raw::Param<'a, dx12_raw::IUnknown> {
        dx12_raw::Param::Owned(::std::convert::Into::<dx12_raw::IUnknown>::into(self))
    }
}
impl<'a> ::std::convert::Into<dx12_raw::Param<'a, dx12_raw::IUnknown>> for &'a IDxcExtraOutputs {
    fn into(self) -> dx12_raw::Param<'a, dx12_raw::IUnknown> {
        dx12_raw::Param::Owned(::std::convert::Into::<dx12_raw::IUnknown>::into(
            ::std::clone::Clone::clone(self),
        ))
    }
}
#[repr(transparent)]
#[allow(non_camel_case_types)]
pub struct IDxcLibrary(dx12_raw::IUnknown);
impl ::std::clone::Clone for IDxcLibrary {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::std::fmt::Debug for IDxcLibrary {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        write!(f, "{:?}", self.0)
    }
}
impl ::std::cmp::PartialEq for IDxcLibrary {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::std::cmp::Eq for IDxcLibrary {}
unsafe impl dx12_raw::Interface for IDxcLibrary {
    type Vtable = IDxcLibrary_abi;
    const IID: dx12_raw::Guid = dx12_raw::Guid::from_values(
        3844099527,
        53644,
        19516,
        [189, 251, 133, 22, 115, 152, 15, 231],
    );
}
#[repr(C)]
pub struct IDxcLibrary_abi(
    pub  unsafe extern "system" fn(
        this: dx12_raw::RawPtr,
        iid: &dx12_raw::Guid,
        interface: *mut dx12_raw::RawPtr,
    ) -> dx12_raw::ErrorCode,
    pub unsafe extern "system" fn(this: dx12_raw::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: dx12_raw::RawPtr) -> u32,
    pub  unsafe extern "system" fn(
        this: dx12_raw::RawPtr,
        p_malloc: dx12_raw::RawPtr,
    ) -> dx12_raw::ErrorCode,
    pub  unsafe extern "system" fn(
        this: dx12_raw::RawPtr,
        p_blob: dx12_raw::RawPtr,
        offset: u32,
        length: u32,
        pp_result: *mut ::std::option::Option<IDxcBlob>,
    ) -> dx12_raw::ErrorCode,
    pub  unsafe extern "system" fn(
        this: dx12_raw::RawPtr,
        p_file_name: *const u16,
        code_page: *mut u32,
        p_blob_encoding: *mut ::std::option::Option<IDxcBlobEncoding>,
    ) -> dx12_raw::ErrorCode,
    pub  unsafe extern "system" fn(
        this: dx12_raw::RawPtr,
        p_text: *mut ::std::ffi::c_void,
        size: u32,
        code_page: u32,
        p_blob_encoding: *mut ::std::option::Option<IDxcBlobEncoding>,
    ) -> dx12_raw::ErrorCode,
    pub  unsafe extern "system" fn(
        this: dx12_raw::RawPtr,
        p_text: *mut ::std::ffi::c_void,
        size: u32,
        code_page: u32,
        p_blob_encoding: *mut ::std::option::Option<IDxcBlobEncoding>,
    ) -> dx12_raw::ErrorCode,
    pub  unsafe extern "system" fn(
        this: dx12_raw::RawPtr,
        p_text: *mut ::std::ffi::c_void,
        p_imalloc: dx12_raw::RawPtr,
        size: u32,
        code_page: u32,
        p_blob_encoding: *mut ::std::option::Option<IDxcBlobEncoding>,
    ) -> dx12_raw::ErrorCode,
    pub  unsafe extern "system" fn(
        this: dx12_raw::RawPtr,
        pp_result: *mut ::std::option::Option<IDxcIncludeHandler>,
    ) -> dx12_raw::ErrorCode,
    pub  unsafe extern "system" fn(
        this: dx12_raw::RawPtr,
        p_blob: dx12_raw::RawPtr,
        pp_stream: *mut ::std::option::Option<windows::win32::structured_storage::IStream>,
    ) -> dx12_raw::ErrorCode,
    pub  unsafe extern "system" fn(
        this: dx12_raw::RawPtr,
        p_blob: dx12_raw::RawPtr,
        p_blob_encoding: *mut ::std::option::Option<IDxcBlobEncoding>,
    ) -> dx12_raw::ErrorCode,
    pub  unsafe extern "system" fn(
        this: dx12_raw::RawPtr,
        p_blob: dx12_raw::RawPtr,
        p_blob_encoding: *mut ::std::option::Option<IDxcBlobEncoding>,
    ) -> dx12_raw::ErrorCode,
);
#[allow(non_snake_case)]
impl IDxcLibrary {
    pub unsafe fn SetMalloc<
        'a,
        T0__: ::std::convert::Into<dx12_raw::Param<'a, windows::win32::com::IMalloc>>,
    >(
        &self,
        p_malloc: T0__,
    ) -> dx12_raw::ErrorCode {
        (dx12_raw::Interface::vtable(self).3)(dx12_raw::Abi::abi(self), p_malloc.into().abi())
    }
    pub unsafe fn CreateBlobFromBlob<
        'a,
        T0__: ::std::convert::Into<dx12_raw::Param<'a, IDxcBlob>>,
    >(
        &self,
        p_blob: T0__,
        offset: u32,
        length: u32,
        pp_result: *mut ::std::option::Option<IDxcBlob>,
    ) -> dx12_raw::ErrorCode {
        (dx12_raw::Interface::vtable(self).4)(
            dx12_raw::Abi::abi(self),
            p_blob.into().abi(),
            offset,
            length,
            pp_result,
        )
    }
    pub unsafe fn CreateBlobFromFile(
        &self,
        p_file_name: *const u16,
        code_page: *mut u32,
        p_blob_encoding: *mut ::std::option::Option<IDxcBlobEncoding>,
    ) -> dx12_raw::ErrorCode {
        (dx12_raw::Interface::vtable(self).5)(
            dx12_raw::Abi::abi(self),
            p_file_name,
            code_page,
            p_blob_encoding,
        )
    }
    pub unsafe fn CreateBlobWithEncodingFromPinned(
        &self,
        p_text: *mut ::std::ffi::c_void,
        size: u32,
        code_page: u32,
        p_blob_encoding: *mut ::std::option::Option<IDxcBlobEncoding>,
    ) -> dx12_raw::ErrorCode {
        (dx12_raw::Interface::vtable(self).6)(
            dx12_raw::Abi::abi(self),
            p_text,
            size,
            code_page,
            p_blob_encoding,
        )
    }
    pub unsafe fn CreateBlobWithEncodingOnHeapCopy(
        &self,
        p_text: *mut ::std::ffi::c_void,
        size: u32,
        code_page: u32,
        p_blob_encoding: *mut ::std::option::Option<IDxcBlobEncoding>,
    ) -> dx12_raw::ErrorCode {
        (dx12_raw::Interface::vtable(self).7)(
            dx12_raw::Abi::abi(self),
            p_text,
            size,
            code_page,
            p_blob_encoding,
        )
    }
    pub unsafe fn CreateBlobWithEncodingOnMalloc<
        'a,
        T1__: ::std::convert::Into<dx12_raw::Param<'a, windows::win32::com::IMalloc>>,
    >(
        &self,
        p_text: *mut ::std::ffi::c_void,
        p_imalloc: T1__,
        size: u32,
        code_page: u32,
        p_blob_encoding: *mut ::std::option::Option<IDxcBlobEncoding>,
    ) -> dx12_raw::ErrorCode {
        (dx12_raw::Interface::vtable(self).8)(
            dx12_raw::Abi::abi(self),
            p_text,
            p_imalloc.into().abi(),
            size,
            code_page,
            p_blob_encoding,
        )
    }
    pub unsafe fn CreateIncludeHandler(
        &self,
        pp_result: *mut ::std::option::Option<IDxcIncludeHandler>,
    ) -> dx12_raw::ErrorCode {
        (dx12_raw::Interface::vtable(self).9)(dx12_raw::Abi::abi(self), pp_result)
    }
    pub unsafe fn CreateStreamFromBlobReadOnly<
        'a,
        T0__: ::std::convert::Into<dx12_raw::Param<'a, IDxcBlob>>,
    >(
        &self,
        p_blob: T0__,
        pp_stream: *mut ::std::option::Option<windows::win32::structured_storage::IStream>,
    ) -> dx12_raw::ErrorCode {
        (dx12_raw::Interface::vtable(self).10)(
            dx12_raw::Abi::abi(self),
            p_blob.into().abi(),
            pp_stream,
        )
    }
    pub unsafe fn GetBlobAsUtf8<'a, T0__: ::std::convert::Into<dx12_raw::Param<'a, IDxcBlob>>>(
        &self,
        p_blob: T0__,
        p_blob_encoding: *mut ::std::option::Option<IDxcBlobEncoding>,
    ) -> dx12_raw::ErrorCode {
        (dx12_raw::Interface::vtable(self).11)(
            dx12_raw::Abi::abi(self),
            p_blob.into().abi(),
            p_blob_encoding,
        )
    }
    pub unsafe fn GetBlobAsUtf16<'a, T0__: ::std::convert::Into<dx12_raw::Param<'a, IDxcBlob>>>(
        &self,
        p_blob: T0__,
        p_blob_encoding: *mut ::std::option::Option<IDxcBlobEncoding>,
    ) -> dx12_raw::ErrorCode {
        (dx12_raw::Interface::vtable(self).12)(
            dx12_raw::Abi::abi(self),
            p_blob.into().abi(),
            p_blob_encoding,
        )
    }
}
impl ::std::convert::From<IDxcLibrary> for dx12_raw::IUnknown {
    fn from(value: IDxcLibrary) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IDxcLibrary> for dx12_raw::IUnknown {
    fn from(value: &IDxcLibrary) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::std::convert::Into<dx12_raw::Param<'a, dx12_raw::IUnknown>> for IDxcLibrary {
    fn into(self) -> dx12_raw::Param<'a, dx12_raw::IUnknown> {
        dx12_raw::Param::Owned(::std::convert::Into::<dx12_raw::IUnknown>::into(self))
    }
}
impl<'a> ::std::convert::Into<dx12_raw::Param<'a, dx12_raw::IUnknown>> for &'a IDxcLibrary {
    fn into(self) -> dx12_raw::Param<'a, dx12_raw::IUnknown> {
        dx12_raw::Param::Owned(::std::convert::Into::<dx12_raw::IUnknown>::into(
            ::std::clone::Clone::clone(self),
        ))
    }
}
#[repr(transparent)]
#[allow(non_camel_case_types)]
pub struct IDxcLinker(dx12_raw::IUnknown);
impl ::std::clone::Clone for IDxcLinker {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::std::fmt::Debug for IDxcLinker {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        write!(f, "{:?}", self.0)
    }
}
impl ::std::cmp::PartialEq for IDxcLinker {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::std::cmp::Eq for IDxcLinker {}
unsafe impl dx12_raw::Interface for IDxcLinker {
    type Vtable = IDxcLinker_abi;
    const IID: dx12_raw::Guid = dx12_raw::Guid::from_values(
        4055219754,
        25309,
        17191,
        [161, 194, 66, 172, 30, 30, 120, 230],
    );
}
#[repr(C)]
pub struct IDxcLinker_abi(
    pub  unsafe extern "system" fn(
        this: dx12_raw::RawPtr,
        iid: &dx12_raw::Guid,
        interface: *mut dx12_raw::RawPtr,
    ) -> dx12_raw::ErrorCode,
    pub unsafe extern "system" fn(this: dx12_raw::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: dx12_raw::RawPtr) -> u32,
    pub  unsafe extern "system" fn(
        this: dx12_raw::RawPtr,
        p_lib_name: *const u16,
        p_lib: dx12_raw::RawPtr,
    ) -> dx12_raw::ErrorCode,
    pub  unsafe extern "system" fn(
        this: dx12_raw::RawPtr,
        p_entry_name: *const u16,
        p_target_profile: *const u16,
        p_lib_names: *const *const u16,
        lib_count: u32,
        p_arguments: *const *const u16,
        arg_count: u32,
        pp_result: *mut ::std::option::Option<IDxcOperationResult>,
    ) -> dx12_raw::ErrorCode,
);
#[allow(non_snake_case)]
impl IDxcLinker {
    pub unsafe fn RegisterLibrary<'a, T1__: ::std::convert::Into<dx12_raw::Param<'a, IDxcBlob>>>(
        &self,
        p_lib_name: *const u16,
        p_lib: T1__,
    ) -> dx12_raw::ErrorCode {
        (dx12_raw::Interface::vtable(self).3)(
            dx12_raw::Abi::abi(self),
            p_lib_name,
            p_lib.into().abi(),
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
    ) -> dx12_raw::ErrorCode {
        (dx12_raw::Interface::vtable(self).4)(
            dx12_raw::Abi::abi(self),
            p_entry_name,
            p_target_profile,
            p_lib_names,
            lib_count,
            p_arguments,
            arg_count,
            pp_result,
        )
    }
}
impl ::std::convert::From<IDxcLinker> for dx12_raw::IUnknown {
    fn from(value: IDxcLinker) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IDxcLinker> for dx12_raw::IUnknown {
    fn from(value: &IDxcLinker) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::std::convert::Into<dx12_raw::Param<'a, dx12_raw::IUnknown>> for IDxcLinker {
    fn into(self) -> dx12_raw::Param<'a, dx12_raw::IUnknown> {
        dx12_raw::Param::Owned(::std::convert::Into::<dx12_raw::IUnknown>::into(self))
    }
}
impl<'a> ::std::convert::Into<dx12_raw::Param<'a, dx12_raw::IUnknown>> for &'a IDxcLinker {
    fn into(self) -> dx12_raw::Param<'a, dx12_raw::IUnknown> {
        dx12_raw::Param::Owned(::std::convert::Into::<dx12_raw::IUnknown>::into(
            ::std::clone::Clone::clone(self),
        ))
    }
}
#[repr(transparent)]
#[allow(non_camel_case_types)]
pub struct IDxcOptimizerPass(dx12_raw::IUnknown);
impl ::std::clone::Clone for IDxcOptimizerPass {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::std::fmt::Debug for IDxcOptimizerPass {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        write!(f, "{:?}", self.0)
    }
}
impl ::std::cmp::PartialEq for IDxcOptimizerPass {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::std::cmp::Eq for IDxcOptimizerPass {}
unsafe impl dx12_raw::Interface for IDxcOptimizerPass {
    type Vtable = IDxcOptimizerPass_abi;
    const IID: dx12_raw::Guid = dx12_raw::Guid::from_values(
        2922174367,
        52258,
        17727,
        [155, 107, 177, 36, 231, 165, 32, 76],
    );
}
#[repr(C)]
pub struct IDxcOptimizerPass_abi(
    pub  unsafe extern "system" fn(
        this: dx12_raw::RawPtr,
        iid: &dx12_raw::Guid,
        interface: *mut dx12_raw::RawPtr,
    ) -> dx12_raw::ErrorCode,
    pub unsafe extern "system" fn(this: dx12_raw::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: dx12_raw::RawPtr) -> u32,
    pub  unsafe extern "system" fn(
        this: dx12_raw::RawPtr,
        pp_result: *mut *mut u16,
    ) -> dx12_raw::ErrorCode,
    pub  unsafe extern "system" fn(
        this: dx12_raw::RawPtr,
        pp_result: *mut *mut u16,
    ) -> dx12_raw::ErrorCode,
    pub unsafe extern "system" fn(this: dx12_raw::RawPtr, p_count: *mut u32) -> dx12_raw::ErrorCode,
    pub  unsafe extern "system" fn(
        this: dx12_raw::RawPtr,
        arg_index: u32,
        pp_result: *mut *mut u16,
    ) -> dx12_raw::ErrorCode,
    pub  unsafe extern "system" fn(
        this: dx12_raw::RawPtr,
        arg_index: u32,
        pp_result: *mut *mut u16,
    ) -> dx12_raw::ErrorCode,
);
#[allow(non_snake_case)]
impl IDxcOptimizerPass {
    pub unsafe fn GetOptionName(&self, pp_result: *mut *mut u16) -> dx12_raw::ErrorCode {
        (dx12_raw::Interface::vtable(self).3)(dx12_raw::Abi::abi(self), pp_result)
    }
    pub unsafe fn GetDescription(&self, pp_result: *mut *mut u16) -> dx12_raw::ErrorCode {
        (dx12_raw::Interface::vtable(self).4)(dx12_raw::Abi::abi(self), pp_result)
    }
    pub unsafe fn GetOptionArgCount(&self, p_count: *mut u32) -> dx12_raw::ErrorCode {
        (dx12_raw::Interface::vtable(self).5)(dx12_raw::Abi::abi(self), p_count)
    }
    pub unsafe fn GetOptionArgName(
        &self,
        arg_index: u32,
        pp_result: *mut *mut u16,
    ) -> dx12_raw::ErrorCode {
        (dx12_raw::Interface::vtable(self).6)(dx12_raw::Abi::abi(self), arg_index, pp_result)
    }
    pub unsafe fn GetOptionArgDescription(
        &self,
        arg_index: u32,
        pp_result: *mut *mut u16,
    ) -> dx12_raw::ErrorCode {
        (dx12_raw::Interface::vtable(self).7)(dx12_raw::Abi::abi(self), arg_index, pp_result)
    }
}
impl ::std::convert::From<IDxcOptimizerPass> for dx12_raw::IUnknown {
    fn from(value: IDxcOptimizerPass) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IDxcOptimizerPass> for dx12_raw::IUnknown {
    fn from(value: &IDxcOptimizerPass) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::std::convert::Into<dx12_raw::Param<'a, dx12_raw::IUnknown>> for IDxcOptimizerPass {
    fn into(self) -> dx12_raw::Param<'a, dx12_raw::IUnknown> {
        dx12_raw::Param::Owned(::std::convert::Into::<dx12_raw::IUnknown>::into(self))
    }
}
impl<'a> ::std::convert::Into<dx12_raw::Param<'a, dx12_raw::IUnknown>> for &'a IDxcOptimizerPass {
    fn into(self) -> dx12_raw::Param<'a, dx12_raw::IUnknown> {
        dx12_raw::Param::Owned(::std::convert::Into::<dx12_raw::IUnknown>::into(
            ::std::clone::Clone::clone(self),
        ))
    }
}
#[repr(transparent)]
#[allow(non_camel_case_types)]
pub struct IDxcOptimizer(dx12_raw::IUnknown);
impl ::std::clone::Clone for IDxcOptimizer {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::std::fmt::Debug for IDxcOptimizer {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        write!(f, "{:?}", self.0)
    }
}
impl ::std::cmp::PartialEq for IDxcOptimizer {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::std::cmp::Eq for IDxcOptimizer {}
unsafe impl dx12_raw::Interface for IDxcOptimizer {
    type Vtable = IDxcOptimizer_abi;
    const IID: dx12_raw::Guid = dx12_raw::Guid::from_values(
        628362798,
        40122,
        16411,
        [145, 25, 79, 180, 47, 57, 242, 112],
    );
}
#[repr(C)]
pub struct IDxcOptimizer_abi(
    pub  unsafe extern "system" fn(
        this: dx12_raw::RawPtr,
        iid: &dx12_raw::Guid,
        interface: *mut dx12_raw::RawPtr,
    ) -> dx12_raw::ErrorCode,
    pub unsafe extern "system" fn(this: dx12_raw::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: dx12_raw::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: dx12_raw::RawPtr, p_count: *mut u32) -> dx12_raw::ErrorCode,
    pub  unsafe extern "system" fn(
        this: dx12_raw::RawPtr,
        index: u32,
        pp_result: *mut ::std::option::Option<IDxcOptimizerPass>,
    ) -> dx12_raw::ErrorCode,
    pub  unsafe extern "system" fn(
        this: dx12_raw::RawPtr,
        p_blob: dx12_raw::RawPtr,
        pp_options: *mut *mut u16,
        option_count: u32,
        p_output_module: *mut ::std::option::Option<IDxcBlob>,
        pp_output_text: *mut ::std::option::Option<IDxcBlobEncoding>,
    ) -> dx12_raw::ErrorCode,
);
#[allow(non_snake_case)]
impl IDxcOptimizer {
    pub unsafe fn GetAvailablePassCount(&self, p_count: *mut u32) -> dx12_raw::ErrorCode {
        (dx12_raw::Interface::vtable(self).3)(dx12_raw::Abi::abi(self), p_count)
    }
    pub unsafe fn GetAvailablePass(
        &self,
        index: u32,
        pp_result: *mut ::std::option::Option<IDxcOptimizerPass>,
    ) -> dx12_raw::ErrorCode {
        (dx12_raw::Interface::vtable(self).4)(dx12_raw::Abi::abi(self), index, pp_result)
    }
    pub unsafe fn RunOptimizer<'a, T0__: ::std::convert::Into<dx12_raw::Param<'a, IDxcBlob>>>(
        &self,
        p_blob: T0__,
        pp_options: *mut *mut u16,
        option_count: u32,
        p_output_module: *mut ::std::option::Option<IDxcBlob>,
        pp_output_text: *mut ::std::option::Option<IDxcBlobEncoding>,
    ) -> dx12_raw::ErrorCode {
        (dx12_raw::Interface::vtable(self).5)(
            dx12_raw::Abi::abi(self),
            p_blob.into().abi(),
            pp_options,
            option_count,
            p_output_module,
            pp_output_text,
        )
    }
}
impl ::std::convert::From<IDxcOptimizer> for dx12_raw::IUnknown {
    fn from(value: IDxcOptimizer) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IDxcOptimizer> for dx12_raw::IUnknown {
    fn from(value: &IDxcOptimizer) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::std::convert::Into<dx12_raw::Param<'a, dx12_raw::IUnknown>> for IDxcOptimizer {
    fn into(self) -> dx12_raw::Param<'a, dx12_raw::IUnknown> {
        dx12_raw::Param::Owned(::std::convert::Into::<dx12_raw::IUnknown>::into(self))
    }
}
impl<'a> ::std::convert::Into<dx12_raw::Param<'a, dx12_raw::IUnknown>> for &'a IDxcOptimizer {
    fn into(self) -> dx12_raw::Param<'a, dx12_raw::IUnknown> {
        dx12_raw::Param::Owned(::std::convert::Into::<dx12_raw::IUnknown>::into(
            ::std::clone::Clone::clone(self),
        ))
    }
}
#[repr(transparent)]
#[allow(non_camel_case_types)]
pub struct IDxcResult(dx12_raw::IUnknown);
impl ::std::clone::Clone for IDxcResult {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::std::fmt::Debug for IDxcResult {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        write!(f, "{:?}", self.0)
    }
}
impl ::std::cmp::PartialEq for IDxcResult {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::std::cmp::Eq for IDxcResult {}
unsafe impl dx12_raw::Interface for IDxcResult {
    type Vtable = IDxcResult_abi;
    const IID: dx12_raw::Guid = dx12_raw::Guid::from_values(
        1479830746,
        56807,
        17559,
        [148, 97, 111, 135, 175, 94, 6, 89],
    );
}
#[repr(C)]
pub struct IDxcResult_abi(
    pub  unsafe extern "system" fn(
        this: dx12_raw::RawPtr,
        iid: &dx12_raw::Guid,
        interface: *mut dx12_raw::RawPtr,
    ) -> dx12_raw::ErrorCode,
    pub unsafe extern "system" fn(this: dx12_raw::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: dx12_raw::RawPtr) -> u32,
    pub  unsafe extern "system" fn(
        this: dx12_raw::RawPtr,
        p_status: *mut dx12_raw::ErrorCode,
    ) -> dx12_raw::ErrorCode,
    pub  unsafe extern "system" fn(
        this: dx12_raw::RawPtr,
        pp_result: *mut ::std::option::Option<IDxcBlob>,
    ) -> dx12_raw::ErrorCode,
    pub  unsafe extern "system" fn(
        this: dx12_raw::RawPtr,
        pp_errors: *mut ::std::option::Option<IDxcBlobEncoding>,
    ) -> dx12_raw::ErrorCode,
    pub  unsafe extern "system" fn(
        this: dx12_raw::RawPtr,
        dxc_out_kind: DXC_OUT_KIND,
    ) -> dx12_raw::BOOL,
    pub  unsafe extern "system" fn(
        this: dx12_raw::RawPtr,
        dxc_out_kind: DXC_OUT_KIND,
        iid: *const dx12_raw::Guid,
        ppv_object: *mut *mut ::std::ffi::c_void,
        pp_output_name: *mut ::std::option::Option<IDxcBlobUtf16>,
    ) -> dx12_raw::ErrorCode,
    pub unsafe extern "system" fn(this: dx12_raw::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: dx12_raw::RawPtr, index: u32) -> DXC_OUT_KIND,
    pub unsafe extern "system" fn(this: dx12_raw::RawPtr) -> DXC_OUT_KIND,
);
#[allow(non_snake_case)]
impl IDxcResult {
    pub unsafe fn GetStatus(&self, p_status: *mut dx12_raw::ErrorCode) -> dx12_raw::ErrorCode {
        (dx12_raw::Interface::vtable(self).3)(dx12_raw::Abi::abi(self), p_status)
    }
    pub unsafe fn GetResult(
        &self,
        pp_result: *mut ::std::option::Option<IDxcBlob>,
    ) -> dx12_raw::ErrorCode {
        (dx12_raw::Interface::vtable(self).4)(dx12_raw::Abi::abi(self), pp_result)
    }
    pub unsafe fn GetErrorBuffer(
        &self,
        pp_errors: *mut ::std::option::Option<IDxcBlobEncoding>,
    ) -> dx12_raw::ErrorCode {
        (dx12_raw::Interface::vtable(self).5)(dx12_raw::Abi::abi(self), pp_errors)
    }
    pub unsafe fn HasOutput(&self, dxc_out_kind: DXC_OUT_KIND) -> dx12_raw::BOOL {
        (dx12_raw::Interface::vtable(self).6)(dx12_raw::Abi::abi(self), dxc_out_kind)
    }
    pub unsafe fn GetOutput(
        &self,
        dxc_out_kind: DXC_OUT_KIND,
        iid: *const dx12_raw::Guid,
        ppv_object: *mut *mut ::std::ffi::c_void,
        pp_output_name: *mut ::std::option::Option<IDxcBlobUtf16>,
    ) -> dx12_raw::ErrorCode {
        (dx12_raw::Interface::vtable(self).7)(
            dx12_raw::Abi::abi(self),
            dxc_out_kind,
            iid,
            ppv_object,
            pp_output_name,
        )
    }
    pub unsafe fn GetNumOutputs(&self) -> u32 {
        (dx12_raw::Interface::vtable(self).8)(dx12_raw::Abi::abi(self))
    }
    pub unsafe fn GetOutputByIndex(&self, index: u32) -> DXC_OUT_KIND {
        (dx12_raw::Interface::vtable(self).9)(dx12_raw::Abi::abi(self), index)
    }
    pub unsafe fn PrimaryOutput(&self) -> DXC_OUT_KIND {
        (dx12_raw::Interface::vtable(self).10)(dx12_raw::Abi::abi(self))
    }
}
impl ::std::convert::From<IDxcResult> for dx12_raw::IUnknown {
    fn from(value: IDxcResult) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IDxcResult> for dx12_raw::IUnknown {
    fn from(value: &IDxcResult) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::std::convert::Into<dx12_raw::Param<'a, dx12_raw::IUnknown>> for IDxcResult {
    fn into(self) -> dx12_raw::Param<'a, dx12_raw::IUnknown> {
        dx12_raw::Param::Owned(::std::convert::Into::<dx12_raw::IUnknown>::into(self))
    }
}
impl<'a> ::std::convert::Into<dx12_raw::Param<'a, dx12_raw::IUnknown>> for &'a IDxcResult {
    fn into(self) -> dx12_raw::Param<'a, dx12_raw::IUnknown> {
        dx12_raw::Param::Owned(::std::convert::Into::<dx12_raw::IUnknown>::into(
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
impl<'a> ::std::convert::Into<dx12_raw::Param<'a, IDxcOperationResult>> for IDxcResult {
    fn into(self) -> dx12_raw::Param<'a, IDxcOperationResult> {
        dx12_raw::Param::Owned(::std::convert::Into::<IDxcOperationResult>::into(self))
    }
}
impl<'a> ::std::convert::Into<dx12_raw::Param<'a, IDxcOperationResult>> for &'a IDxcResult {
    fn into(self) -> dx12_raw::Param<'a, IDxcOperationResult> {
        dx12_raw::Param::Owned(::std::convert::Into::<IDxcOperationResult>::into(
            ::std::clone::Clone::clone(self),
        ))
    }
}
#[repr(transparent)]
#[allow(non_camel_case_types)]
pub struct IDxcUtils(dx12_raw::IUnknown);
impl ::std::clone::Clone for IDxcUtils {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::std::fmt::Debug for IDxcUtils {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        write!(f, "{:?}", self.0)
    }
}
impl ::std::cmp::PartialEq for IDxcUtils {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::std::cmp::Eq for IDxcUtils {}
unsafe impl dx12_raw::Interface for IDxcUtils {
    type Vtable = IDxcUtils_abi;
    const IID: dx12_raw::Guid = dx12_raw::Guid::from_values(
        1174783179,
        8217,
        18730,
        [173, 164, 101, 242, 11, 183, 214, 127],
    );
}
#[repr(C)]
pub struct IDxcUtils_abi(
    pub  unsafe extern "system" fn(
        this: dx12_raw::RawPtr,
        iid: &dx12_raw::Guid,
        interface: *mut dx12_raw::RawPtr,
    ) -> dx12_raw::ErrorCode,
    pub unsafe extern "system" fn(this: dx12_raw::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: dx12_raw::RawPtr) -> u32,
    pub  unsafe extern "system" fn(
        this: dx12_raw::RawPtr,
        p_blob: dx12_raw::RawPtr,
        offset: u32,
        length: u32,
        pp_result: *mut ::std::option::Option<IDxcBlob>,
    ) -> dx12_raw::ErrorCode,
    pub  unsafe extern "system" fn(
        this: dx12_raw::RawPtr,
        p_data: *mut ::std::ffi::c_void,
        size: u32,
        code_page: u32,
        p_blob_encoding: *mut ::std::option::Option<IDxcBlobEncoding>,
    ) -> dx12_raw::ErrorCode,
    pub  unsafe extern "system" fn(
        this: dx12_raw::RawPtr,
        p_data: *mut ::std::ffi::c_void,
        p_imalloc: dx12_raw::RawPtr,
        size: u32,
        code_page: u32,
        p_blob_encoding: *mut ::std::option::Option<IDxcBlobEncoding>,
    ) -> dx12_raw::ErrorCode,
    pub  unsafe extern "system" fn(
        this: dx12_raw::RawPtr,
        p_data: *mut ::std::ffi::c_void,
        size: u32,
        code_page: u32,
        p_blob_encoding: *mut ::std::option::Option<IDxcBlobEncoding>,
    ) -> dx12_raw::ErrorCode,
    pub  unsafe extern "system" fn(
        this: dx12_raw::RawPtr,
        p_file_name: *const u16,
        p_code_page: *mut u32,
        p_blob_encoding: *mut ::std::option::Option<IDxcBlobEncoding>,
    ) -> dx12_raw::ErrorCode,
    pub  unsafe extern "system" fn(
        this: dx12_raw::RawPtr,
        p_blob: dx12_raw::RawPtr,
        pp_stream: *mut ::std::option::Option<windows::win32::structured_storage::IStream>,
    ) -> dx12_raw::ErrorCode,
    pub  unsafe extern "system" fn(
        this: dx12_raw::RawPtr,
        pp_result: *mut ::std::option::Option<IDxcIncludeHandler>,
    ) -> dx12_raw::ErrorCode,
    pub  unsafe extern "system" fn(
        this: dx12_raw::RawPtr,
        p_blob: dx12_raw::RawPtr,
        p_blob_encoding: *mut ::std::option::Option<IDxcBlobUtf8>,
    ) -> dx12_raw::ErrorCode,
    pub  unsafe extern "system" fn(
        this: dx12_raw::RawPtr,
        p_blob: dx12_raw::RawPtr,
        p_blob_encoding: *mut ::std::option::Option<IDxcBlobUtf16>,
    ) -> dx12_raw::ErrorCode,
    pub  unsafe extern "system" fn(
        this: dx12_raw::RawPtr,
        p_shader: *const DxcBuffer,
        dxc_part: u32,
        pp_part_data: *mut *mut ::std::ffi::c_void,
        p_part_size_in_bytes: *mut u32,
    ) -> dx12_raw::ErrorCode,
    pub  unsafe extern "system" fn(
        this: dx12_raw::RawPtr,
        p_data: *const DxcBuffer,
        iid: *const dx12_raw::Guid,
        ppv_reflection: *mut *mut ::std::ffi::c_void,
    ) -> dx12_raw::ErrorCode,
    pub  unsafe extern "system" fn(
        this: dx12_raw::RawPtr,
        p_source_name: *const u16,
        p_entry_point: *const u16,
        p_target_profile: *const u16,
        p_arguments: *mut *mut u16,
        arg_count: u32,
        p_defines: *const DxcDefine,
        define_count: u32,
        pp_args: *mut ::std::option::Option<IDxcCompilerArgs>,
    ) -> dx12_raw::ErrorCode,
    pub  unsafe extern "system" fn(
        this: dx12_raw::RawPtr,
        p_pdb_blob: dx12_raw::RawPtr,
        pp_hash: *mut ::std::option::Option<IDxcBlob>,
        pp_container: *mut ::std::option::Option<IDxcBlob>,
    ) -> dx12_raw::ErrorCode,
);
#[allow(non_snake_case)]
impl IDxcUtils {
    pub unsafe fn CreateBlobFromBlob<
        'a,
        T0__: ::std::convert::Into<dx12_raw::Param<'a, IDxcBlob>>,
    >(
        &self,
        p_blob: T0__,
        offset: u32,
        length: u32,
        pp_result: *mut ::std::option::Option<IDxcBlob>,
    ) -> dx12_raw::ErrorCode {
        (dx12_raw::Interface::vtable(self).3)(
            dx12_raw::Abi::abi(self),
            p_blob.into().abi(),
            offset,
            length,
            pp_result,
        )
    }
    pub unsafe fn CreateBlobFromPinned(
        &self,
        p_data: *mut ::std::ffi::c_void,
        size: u32,
        code_page: u32,
        p_blob_encoding: *mut ::std::option::Option<IDxcBlobEncoding>,
    ) -> dx12_raw::ErrorCode {
        (dx12_raw::Interface::vtable(self).4)(
            dx12_raw::Abi::abi(self),
            p_data,
            size,
            code_page,
            p_blob_encoding,
        )
    }
    pub unsafe fn MoveToBlob<
        'a,
        T1__: ::std::convert::Into<dx12_raw::Param<'a, windows::win32::com::IMalloc>>,
    >(
        &self,
        p_data: *mut ::std::ffi::c_void,
        p_imalloc: T1__,
        size: u32,
        code_page: u32,
        p_blob_encoding: *mut ::std::option::Option<IDxcBlobEncoding>,
    ) -> dx12_raw::ErrorCode {
        (dx12_raw::Interface::vtable(self).5)(
            dx12_raw::Abi::abi(self),
            p_data,
            p_imalloc.into().abi(),
            size,
            code_page,
            p_blob_encoding,
        )
    }
    pub unsafe fn CreateBlob(
        &self,
        p_data: *mut ::std::ffi::c_void,
        size: u32,
        code_page: u32,
        p_blob_encoding: *mut ::std::option::Option<IDxcBlobEncoding>,
    ) -> dx12_raw::ErrorCode {
        (dx12_raw::Interface::vtable(self).6)(
            dx12_raw::Abi::abi(self),
            p_data,
            size,
            code_page,
            p_blob_encoding,
        )
    }
    pub unsafe fn LoadFile(
        &self,
        p_file_name: *const u16,
        p_code_page: *mut u32,
        p_blob_encoding: *mut ::std::option::Option<IDxcBlobEncoding>,
    ) -> dx12_raw::ErrorCode {
        (dx12_raw::Interface::vtable(self).7)(
            dx12_raw::Abi::abi(self),
            p_file_name,
            p_code_page,
            p_blob_encoding,
        )
    }
    pub unsafe fn CreateReadOnlyStreamFromBlob<
        'a,
        T0__: ::std::convert::Into<dx12_raw::Param<'a, IDxcBlob>>,
    >(
        &self,
        p_blob: T0__,
        pp_stream: *mut ::std::option::Option<windows::win32::structured_storage::IStream>,
    ) -> dx12_raw::ErrorCode {
        (dx12_raw::Interface::vtable(self).8)(
            dx12_raw::Abi::abi(self),
            p_blob.into().abi(),
            pp_stream,
        )
    }
    pub unsafe fn CreateDefaultIncludeHandler(
        &self,
        pp_result: *mut ::std::option::Option<IDxcIncludeHandler>,
    ) -> dx12_raw::ErrorCode {
        (dx12_raw::Interface::vtable(self).9)(dx12_raw::Abi::abi(self), pp_result)
    }
    pub unsafe fn GetBlobAsUtf8<'a, T0__: ::std::convert::Into<dx12_raw::Param<'a, IDxcBlob>>>(
        &self,
        p_blob: T0__,
        p_blob_encoding: *mut ::std::option::Option<IDxcBlobUtf8>,
    ) -> dx12_raw::ErrorCode {
        (dx12_raw::Interface::vtable(self).10)(
            dx12_raw::Abi::abi(self),
            p_blob.into().abi(),
            p_blob_encoding,
        )
    }
    pub unsafe fn GetBlobAsUtf16<'a, T0__: ::std::convert::Into<dx12_raw::Param<'a, IDxcBlob>>>(
        &self,
        p_blob: T0__,
        p_blob_encoding: *mut ::std::option::Option<IDxcBlobUtf16>,
    ) -> dx12_raw::ErrorCode {
        (dx12_raw::Interface::vtable(self).11)(
            dx12_raw::Abi::abi(self),
            p_blob.into().abi(),
            p_blob_encoding,
        )
    }
    pub unsafe fn GetDxilContainerPart(
        &self,
        p_shader: *const DxcBuffer,
        dxc_part: u32,
        pp_part_data: *mut *mut ::std::ffi::c_void,
        p_part_size_in_bytes: *mut u32,
    ) -> dx12_raw::ErrorCode {
        (dx12_raw::Interface::vtable(self).12)(
            dx12_raw::Abi::abi(self),
            p_shader,
            dxc_part,
            pp_part_data,
            p_part_size_in_bytes,
        )
    }
    pub unsafe fn CreateReflection(
        &self,
        p_data: *const DxcBuffer,
        iid: *const dx12_raw::Guid,
        ppv_reflection: *mut *mut ::std::ffi::c_void,
    ) -> dx12_raw::ErrorCode {
        (dx12_raw::Interface::vtable(self).13)(
            dx12_raw::Abi::abi(self),
            p_data,
            iid,
            ppv_reflection,
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
    ) -> dx12_raw::ErrorCode {
        (dx12_raw::Interface::vtable(self).14)(
            dx12_raw::Abi::abi(self),
            p_source_name,
            p_entry_point,
            p_target_profile,
            p_arguments,
            arg_count,
            p_defines,
            define_count,
            pp_args,
        )
    }
    pub unsafe fn GetPDBContents<'a, T0__: ::std::convert::Into<dx12_raw::Param<'a, IDxcBlob>>>(
        &self,
        p_pdb_blob: T0__,
        pp_hash: *mut ::std::option::Option<IDxcBlob>,
        pp_container: *mut ::std::option::Option<IDxcBlob>,
    ) -> dx12_raw::ErrorCode {
        (dx12_raw::Interface::vtable(self).15)(
            dx12_raw::Abi::abi(self),
            p_pdb_blob.into().abi(),
            pp_hash,
            pp_container,
        )
    }
}
impl ::std::convert::From<IDxcUtils> for dx12_raw::IUnknown {
    fn from(value: IDxcUtils) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IDxcUtils> for dx12_raw::IUnknown {
    fn from(value: &IDxcUtils) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::std::convert::Into<dx12_raw::Param<'a, dx12_raw::IUnknown>> for IDxcUtils {
    fn into(self) -> dx12_raw::Param<'a, dx12_raw::IUnknown> {
        dx12_raw::Param::Owned(::std::convert::Into::<dx12_raw::IUnknown>::into(self))
    }
}
impl<'a> ::std::convert::Into<dx12_raw::Param<'a, dx12_raw::IUnknown>> for &'a IDxcUtils {
    fn into(self) -> dx12_raw::Param<'a, dx12_raw::IUnknown> {
        dx12_raw::Param::Owned(::std::convert::Into::<dx12_raw::IUnknown>::into(
            ::std::clone::Clone::clone(self),
        ))
    }
}
#[repr(transparent)]
#[allow(non_camel_case_types)]
pub struct IDxcValidator(dx12_raw::IUnknown);
impl ::std::clone::Clone for IDxcValidator {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::std::fmt::Debug for IDxcValidator {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        write!(f, "{:?}", self.0)
    }
}
impl ::std::cmp::PartialEq for IDxcValidator {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::std::cmp::Eq for IDxcValidator {}
unsafe impl dx12_raw::Interface for IDxcValidator {
    type Vtable = IDxcValidator_abi;
    const IID: dx12_raw::Guid = dx12_raw::Guid::from_values(
        2800233426,
        8151,
        18470,
        [152, 17, 40, 87, 231, 151, 244, 154],
    );
}
#[repr(C)]
pub struct IDxcValidator_abi(
    pub  unsafe extern "system" fn(
        this: dx12_raw::RawPtr,
        iid: &dx12_raw::Guid,
        interface: *mut dx12_raw::RawPtr,
    ) -> dx12_raw::ErrorCode,
    pub unsafe extern "system" fn(this: dx12_raw::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: dx12_raw::RawPtr) -> u32,
    pub  unsafe extern "system" fn(
        this: dx12_raw::RawPtr,
        p_shader: dx12_raw::RawPtr,
        flags: u32,
        pp_result: *mut ::std::option::Option<IDxcOperationResult>,
    ) -> dx12_raw::ErrorCode,
);
#[allow(non_snake_case)]
impl IDxcValidator {
    pub unsafe fn Validate<'a, T0__: ::std::convert::Into<dx12_raw::Param<'a, IDxcBlob>>>(
        &self,
        p_shader: T0__,
        flags: u32,
        pp_result: *mut ::std::option::Option<IDxcOperationResult>,
    ) -> dx12_raw::ErrorCode {
        (dx12_raw::Interface::vtable(self).3)(
            dx12_raw::Abi::abi(self),
            p_shader.into().abi(),
            flags,
            pp_result,
        )
    }
}
impl ::std::convert::From<IDxcValidator> for dx12_raw::IUnknown {
    fn from(value: IDxcValidator) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IDxcValidator> for dx12_raw::IUnknown {
    fn from(value: &IDxcValidator) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::std::convert::Into<dx12_raw::Param<'a, dx12_raw::IUnknown>> for IDxcValidator {
    fn into(self) -> dx12_raw::Param<'a, dx12_raw::IUnknown> {
        dx12_raw::Param::Owned(::std::convert::Into::<dx12_raw::IUnknown>::into(self))
    }
}
impl<'a> ::std::convert::Into<dx12_raw::Param<'a, dx12_raw::IUnknown>> for &'a IDxcValidator {
    fn into(self) -> dx12_raw::Param<'a, dx12_raw::IUnknown> {
        dx12_raw::Param::Owned(::std::convert::Into::<dx12_raw::IUnknown>::into(
            ::std::clone::Clone::clone(self),
        ))
    }
}
#[repr(transparent)]
#[allow(non_camel_case_types)]
pub struct IDxcVersionInfo(dx12_raw::IUnknown);
impl ::std::clone::Clone for IDxcVersionInfo {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::std::fmt::Debug for IDxcVersionInfo {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        write!(f, "{:?}", self.0)
    }
}
impl ::std::cmp::PartialEq for IDxcVersionInfo {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::std::cmp::Eq for IDxcVersionInfo {}
unsafe impl dx12_raw::Interface for IDxcVersionInfo {
    type Vtable = IDxcVersionInfo_abi;
    const IID: dx12_raw::Guid = dx12_raw::Guid::from_values(
        2957990736,
        8281,
        20242,
        [168, 255, 161, 224, 205, 225, 204, 126],
    );
}
#[repr(C)]
pub struct IDxcVersionInfo_abi(
    pub  unsafe extern "system" fn(
        this: dx12_raw::RawPtr,
        iid: &dx12_raw::Guid,
        interface: *mut dx12_raw::RawPtr,
    ) -> dx12_raw::ErrorCode,
    pub unsafe extern "system" fn(this: dx12_raw::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: dx12_raw::RawPtr) -> u32,
    pub  unsafe extern "system" fn(
        this: dx12_raw::RawPtr,
        p_major: *mut u32,
        p_minor: *mut u32,
    ) -> dx12_raw::ErrorCode,
    pub unsafe extern "system" fn(this: dx12_raw::RawPtr, p_flags: *mut u32) -> dx12_raw::ErrorCode,
);
#[allow(non_snake_case)]
impl IDxcVersionInfo {
    pub unsafe fn GetVersion(&self, p_major: *mut u32, p_minor: *mut u32) -> dx12_raw::ErrorCode {
        (dx12_raw::Interface::vtable(self).3)(dx12_raw::Abi::abi(self), p_major, p_minor)
    }
    pub unsafe fn GetFlags(&self, p_flags: *mut u32) -> dx12_raw::ErrorCode {
        (dx12_raw::Interface::vtable(self).4)(dx12_raw::Abi::abi(self), p_flags)
    }
}
impl ::std::convert::From<IDxcVersionInfo> for dx12_raw::IUnknown {
    fn from(value: IDxcVersionInfo) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IDxcVersionInfo> for dx12_raw::IUnknown {
    fn from(value: &IDxcVersionInfo) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::std::convert::Into<dx12_raw::Param<'a, dx12_raw::IUnknown>> for IDxcVersionInfo {
    fn into(self) -> dx12_raw::Param<'a, dx12_raw::IUnknown> {
        dx12_raw::Param::Owned(::std::convert::Into::<dx12_raw::IUnknown>::into(self))
    }
}
impl<'a> ::std::convert::Into<dx12_raw::Param<'a, dx12_raw::IUnknown>> for &'a IDxcVersionInfo {
    fn into(self) -> dx12_raw::Param<'a, dx12_raw::IUnknown> {
        dx12_raw::Param::Owned(::std::convert::Into::<dx12_raw::IUnknown>::into(
            ::std::clone::Clone::clone(self),
        ))
    }
}
#[repr(transparent)]
#[allow(non_camel_case_types)]
pub struct IDxcVersionInfo2(dx12_raw::IUnknown);
impl ::std::clone::Clone for IDxcVersionInfo2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::std::fmt::Debug for IDxcVersionInfo2 {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        write!(f, "{:?}", self.0)
    }
}
impl ::std::cmp::PartialEq for IDxcVersionInfo2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::std::cmp::Eq for IDxcVersionInfo2 {}
unsafe impl dx12_raw::Interface for IDxcVersionInfo2 {
    type Vtable = IDxcVersionInfo2_abi;
    const IID: dx12_raw::Guid = dx12_raw::Guid::from_values(
        4217963716,
        17136,
        19298,
        [156, 70, 152, 58, 247, 218, 124, 131],
    );
}
#[repr(C)]
pub struct IDxcVersionInfo2_abi(
    pub  unsafe extern "system" fn(
        this: dx12_raw::RawPtr,
        iid: &dx12_raw::Guid,
        interface: *mut dx12_raw::RawPtr,
    ) -> dx12_raw::ErrorCode,
    pub unsafe extern "system" fn(this: dx12_raw::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: dx12_raw::RawPtr) -> u32,
    pub  unsafe extern "system" fn(
        this: dx12_raw::RawPtr,
        p_major: *mut u32,
        p_minor: *mut u32,
    ) -> dx12_raw::ErrorCode,
    pub unsafe extern "system" fn(this: dx12_raw::RawPtr, p_flags: *mut u32) -> dx12_raw::ErrorCode,
    pub  unsafe extern "system" fn(
        this: dx12_raw::RawPtr,
        p_commit_count: *mut u32,
        p_commit_hash: *mut *mut i8,
    ) -> dx12_raw::ErrorCode,
);
#[allow(non_snake_case)]
impl IDxcVersionInfo2 {
    pub unsafe fn GetVersion(&self, p_major: *mut u32, p_minor: *mut u32) -> dx12_raw::ErrorCode {
        (dx12_raw::Interface::vtable(self).3)(dx12_raw::Abi::abi(self), p_major, p_minor)
    }
    pub unsafe fn GetFlags(&self, p_flags: *mut u32) -> dx12_raw::ErrorCode {
        (dx12_raw::Interface::vtable(self).4)(dx12_raw::Abi::abi(self), p_flags)
    }
    pub unsafe fn GetCommitInfo(
        &self,
        p_commit_count: *mut u32,
        p_commit_hash: *mut *mut i8,
    ) -> dx12_raw::ErrorCode {
        (dx12_raw::Interface::vtable(self).5)(
            dx12_raw::Abi::abi(self),
            p_commit_count,
            p_commit_hash,
        )
    }
}
impl ::std::convert::From<IDxcVersionInfo2> for dx12_raw::IUnknown {
    fn from(value: IDxcVersionInfo2) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IDxcVersionInfo2> for dx12_raw::IUnknown {
    fn from(value: &IDxcVersionInfo2) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::std::convert::Into<dx12_raw::Param<'a, dx12_raw::IUnknown>> for IDxcVersionInfo2 {
    fn into(self) -> dx12_raw::Param<'a, dx12_raw::IUnknown> {
        dx12_raw::Param::Owned(::std::convert::Into::<dx12_raw::IUnknown>::into(self))
    }
}
impl<'a> ::std::convert::Into<dx12_raw::Param<'a, dx12_raw::IUnknown>> for &'a IDxcVersionInfo2 {
    fn into(self) -> dx12_raw::Param<'a, dx12_raw::IUnknown> {
        dx12_raw::Param::Owned(::std::convert::Into::<dx12_raw::IUnknown>::into(
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
impl<'a> ::std::convert::Into<dx12_raw::Param<'a, IDxcVersionInfo>> for IDxcVersionInfo2 {
    fn into(self) -> dx12_raw::Param<'a, IDxcVersionInfo> {
        dx12_raw::Param::Owned(::std::convert::Into::<IDxcVersionInfo>::into(self))
    }
}
impl<'a> ::std::convert::Into<dx12_raw::Param<'a, IDxcVersionInfo>> for &'a IDxcVersionInfo2 {
    fn into(self) -> dx12_raw::Param<'a, IDxcVersionInfo> {
        dx12_raw::Param::Owned(::std::convert::Into::<IDxcVersionInfo>::into(
            ::std::clone::Clone::clone(self),
        ))
    }
}
