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
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

use std::ffi::*;
use std::fmt::{Debug, Formatter};
use std::num::NonZeroU32;
use std::ptr::NonNull;

macro_rules! bitflags_traits {
    ($name:ident) => {
        impl ::std::ops::BitOr for $name {
            type Output = $name;

            #[inline(always)]
            fn bitor(self, rhs: Self) -> Self::Output {
                Self(self.0 | rhs.0)
            }
        }

        impl ::std::ops::BitOrAssign for $name {
            #[inline(always)]
            fn bitor_assign(&mut self, rhs: Self) {
                *self = *self | rhs;
            }
        }

        impl ::std::ops::BitAnd for $name {
            type Output = $name;

            #[inline(always)]
            fn bitand(self, rhs: Self) -> Self::Output {
                Self(self.0 & rhs.0)
            }
        }

        impl ::std::ops::BitAndAssign for $name {
            #[inline(always)]
            fn bitand_assign(&mut self, rhs: Self) {
                *self = *self & rhs;
            }
        }

        impl ::std::ops::BitXor for $name {
            type Output = $name;

            #[inline(always)]
            fn bitxor(self, rhs: Self) -> Self::Output {
                Self(self.0 ^ rhs.0)
            }
        }

        impl ::std::ops::BitXorAssign for $name {
            #[inline(always)]
            fn bitxor_assign(&mut self, rhs: Self) {
                *self = *self ^ rhs;
            }
        }

        impl ::std::ops::Not for $name {
            type Output = $name;

            #[inline(always)]
            fn not(self) -> Self::Output {
                Self(!self.0)
            }
        }
    };
}

#[cfg(target_pointer_width = "32")]
mod nan_boxed {
    use std::ffi::*;
    use std::mem::size_of;

    use crate::JSTag;

    const JS_FLOAT64_TAG_ADDEND: u64 = 0x7ff80000 - (JSTag::FIRST.0 as u64) + 1;

    #[repr(transparent)]
    #[derive(Copy, Clone)]
    pub struct JSValue(u64);

    impl JSValue {
        pub(crate) const __NAN: JSValue =
            JSValue(0x7ff8000000000000 - (JS_FLOAT64_TAG_ADDEND << 32));

        pub(crate) const fn __new_val(tag: JSTag, v: i32) -> JSValue {
            let tag = (tag.0 as u64) << 32;
            JSValue(tag | (v as u64))
        }

        pub(crate) const fn __new_ptr(tag: JSTag, v: *mut c_void) -> JSValue {
            assert!(size_of::<usize>() <= size_of::<JSValue>());
            let tag = (tag.0 as u64) << 32;
            let v = v as u64;
            JSValue(tag | v)
        }

        pub(crate) const fn __new_float64(d: f64) -> JSValue {
            // normalize NaN
            let v: u64 = unsafe { std::mem::transmute(d) };
            if (v & 0x7fffffffffffffff) > 0x7ff0000000000000 {
                Self::__NAN
            } else {
                JSValue(v - (JS_FLOAT64_TAG_ADDEND << 32))
            }
        }

        pub(crate) const fn __get_tag(&self) -> JSTag {
            JSTag((self.0 >> 32) as c_int)
        }

        pub(crate) const fn __get_norm_tag(&self) -> JSTag {
            let tag = self.__get_tag();
            if js_tag_is_float64(tag) {
                JSTag::FLOAT64
            } else {
                tag
            }
        }

        pub(crate) const fn __get_int(&self) -> i32 {
            self.0 as c_int
        }

        pub(crate) const fn __get_bool(&self) -> bool {
            self.0 != 0
        }

        pub(crate) const fn __get_float64(&self) -> f64 {
            let v = self.0;
            let v = v + (JS_FLOAT64_TAG_ADDEND << 32);
            return unsafe { std::mem::transmute(v) };
        }

        pub(crate) const fn __get_ptr(&self) -> *mut c_void {
            assert!(size_of::<usize>() <= size_of::<JSValue>());
            self.0 as usize as *mut c_void
        }

        pub(crate) const fn __is_nan(&self) -> bool {
            let tag = self.__get_tag();
            tag.0 == (Self::__NAN.0 >> 32) as c_int
        }
    }

    const fn js_tag_is_float64(tag: JSTag) -> bool {
        ((tag.0 - JSTag::FIRST.0) as c_uint) >= (JSTag::FLOAT64.0 - JSTag::FIRST.0) as c_uint
    }
}

#[cfg(target_pointer_width = "32")]
pub use nan_boxed::*;

#[cfg(target_pointer_width = "64")]
mod non_nan_boxed {
    use crate::JSTag;
    use std::ffi::*;

    #[repr(C)]
    #[derive(Copy, Clone)]
    union JSValueUnion {
        int32: i32,
        float64: f64,
        ptr: *mut c_void,
    }

    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct JSValue {
        u: JSValueUnion,
        tag: i64,
    }

    impl JSValue {
        pub(crate) const __NAN: JSValue = JSValue {
            u: JSValueUnion { float64: f64::NAN },
            tag: JSTag::FLOAT64.0 as i64,
        };

        pub(crate) const fn __new_val(tag: JSTag, v: i32) -> JSValue {
            let u = JSValueUnion { int32: v };
            JSValue {
                u,
                tag: tag.0 as i64,
            }
        }

        pub(crate) const fn __new_ptr(tag: JSTag, v: *mut c_void) -> JSValue {
            let u = JSValueUnion { ptr: v };
            JSValue {
                u,
                tag: tag.0 as i64,
            }
        }

        pub(crate) const fn __new_float64(d: f64) -> JSValue {
            JSValue {
                u: JSValueUnion { float64: d },
                tag: JSTag::FLOAT64.0 as i64,
            }
        }

        pub(crate) const fn __get_tag(&self) -> JSTag {
            JSTag(self.tag as c_int)
        }

        pub(crate) const fn __get_norm_tag(&self) -> JSTag {
            self.__get_tag()
        }

        pub(crate) const fn __get_int(&self) -> i32 {
            unsafe { self.u.int32 }
        }

        pub(crate) const fn __get_bool(&self) -> bool {
            self.__get_int() != 0
        }

        pub(crate) const fn __get_float64(&self) -> f64 {
            unsafe { self.u.float64 }
        }

        pub(crate) const fn __get_ptr(&self) -> *mut c_void {
            unsafe { self.u.ptr }
        }

        pub(crate) const fn __is_nan(&self) -> bool {
            if self.tag != (JSTag::FLOAT64.0 as i64) {
                false
            } else {
                let d = unsafe { self.u.float64 };
                let d: u64 = unsafe { std::mem::transmute(d) };
                (d & 0x7fffffffffffffff) > 0x7ff0000000000000
            }
        }
    }
}

#[cfg(target_pointer_width = "64")]
pub use non_nan_boxed::*;

pub const QJS_VERSION_MAJOR: u32 = 0;
pub const QJS_VERSION_MINOR: u32 = 5;
pub const QJS_VERSION_PATCH: u32 = 0;
pub const QJS_VERSION_SUFFIX: &str = "";

pub const JS_DEFAULT_STACK_SIZE: c_int = 256 * 1024;

#[repr(C)]
struct Private {
    _private: (),
}

#[repr(C)]
pub struct JSRuntime(Private);

#[repr(C)]
pub struct JSContext(Private);

#[repr(C)]
pub struct JSObject(Private);

#[repr(C)]
pub struct JSClass(Private);

#[repr(C)]
pub struct JSGCObjectHeader(Private);

#[repr(C)]
pub struct JSModuleDef(Private);

#[repr(transparent)]
#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Debug)]
pub struct JSClassID(pub NonZeroU32);

#[repr(transparent)]
#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Debug)]
pub struct JSAtom(pub NonZeroU32);

#[repr(transparent)]
#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Default)]
pub struct JSBool(pub c_int);

impl JSBool {
    pub const TRUE: Self = Self::new(true);
    pub const FALSE: Self = Self::new(false);

    pub const fn new(v: bool) -> Self {
        Self(v as c_int)
    }

    pub const fn to_bool(self) -> bool {
        self.0 != 0
    }

    pub const fn is_true(&self) -> bool {
        self.0 != 0
    }

    pub const fn is_false(&self) -> bool {
        !self.is_true()
    }
}

impl Debug for JSBool {
    #[inline]
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        bool::fmt(&self.to_bool(), f)
    }
}

impl Into<bool> for JSBool {
    #[inline(always)]
    fn into(self) -> bool {
        self.to_bool()
    }
}

impl From<bool> for JSBool {
    #[inline(always)]
    fn from(value: bool) -> Self {
        Self::new(value)
    }
}

impl JSValue {
    pub const NULL: Self = JSValue::__new_val(JSTag::NULL, 0);
    pub const UNDEFINED: Self = JSValue::__new_val(JSTag::UNDEFINED, 0);
    pub const FALSE: Self = JSValue::__new_val(JSTag::BOOL, 0);
    pub const TRUE: Self = JSValue::__new_val(JSTag::BOOL, 1);
    pub const EXCEPTION: Self = JSValue::__new_val(JSTag::EXCEPTION, 0);
    pub const UNINITIALIZED: Self = JSValue::__new_val(JSTag::UNINITIALIZED, 0);
    pub const NAN: JSValue = Self::__NAN;

    pub const fn new_val(tag: JSTag, v: i32) -> JSValue {
        Self::__new_val(tag, v)
    }

    pub const fn new_ptr(tag: JSTag, v: *mut c_void) -> JSValue {
        Self::__new_ptr(tag, v)
    }

    pub const fn new_float64(d: f64) -> JSValue {
        Self::__new_float64(d)
    }

    pub const fn new_bool(val: bool) -> JSValue {
        return Self::__new_val(JSTag::BOOL, val as i32);
    }

    pub const fn new_i32(val: i32) -> JSValue {
        return Self::__new_val(JSTag::INT, val);
    }

    pub const fn new_f64(val: f64) -> JSValue {
        return Self::__new_float64(val);
    }

    pub const fn new_catch_offset(val: i32) -> JSValue {
        return Self::__new_val(JSTag::CATCH_OFFSET, val);
    }

    pub const fn new_i64(val: i64) -> JSValue {
        let min = i32::MIN as i64;
        let max = i32::MAX as i64;
        if val < min || val > max {
            Self::new_f64(val as f64)
        } else {
            Self::new_i32(val as i32)
        }
    }

    pub const fn new_u32(val: u32) -> JSValue {
        let vi64 = val as i64;
        let min = i32::MIN as i64;
        let max = i32::MAX as i64;
        if vi64 < min || vi64 > max {
            Self::new_f64(val as f64)
        } else {
            Self::new_i32(val as i32)
        }
    }

    pub unsafe fn new_string(ctx: NonNull<JSContext>, str: *const c_char) -> JSValue {
        let len = {
            let str = CStr::from_ptr(str);
            str.count_bytes()
        };
        return JS_NewStringLen(ctx, str, len);
    }

    pub const fn get_obj(&self) -> *mut JSObject {
        self.get_ptr() as *mut JSObject
    }

    pub const fn get_tag(&self) -> JSTag {
        self.__get_tag()
    }

    pub const fn get_norm_tag(&self) -> JSTag {
        self.__get_norm_tag()
    }

    pub const fn get_int(&self) -> i32 {
        self.__get_int()
    }

    pub const fn get_bool(&self) -> bool {
        self.__get_bool()
    }

    pub const fn get_float64(&self) -> f64 {
        self.__get_float64()
    }

    pub const fn get_ptr(&self) -> *mut c_void {
        self.__get_ptr()
    }

    pub const fn has_ref_count(&self) -> bool {
        (self.get_tag().0 as c_uint) >= (JSTag::FIRST.0 as c_uint)
    }

    pub const fn is_nan(&self) -> bool {
        self.__is_nan()
    }

    pub const fn is_number(&self) -> bool {
        let tag = self.get_tag();
        tag.0 == JSTag::INT.0 || tag.0 == JSTag::FLOAT64.0
    }

    pub const fn is_big_int(&self) -> bool {
        let tag = self.get_tag();
        tag.0 == JSTag::BIG_INT.0
    }

    pub const fn is_bool(&self) -> bool {
        return self.get_tag().0 == JSTag::BOOL.0;
    }

    pub const fn is_null(&self) -> bool {
        return self.get_tag().0 == JSTag::NULL.0;
    }

    pub const fn is_undefined(&self) -> bool {
        return self.get_tag().0 == JSTag::UNDEFINED.0;
    }

    pub const fn is_exception(&self) -> bool {
        return self.get_tag().0 == JSTag::EXCEPTION.0;
    }

    pub const fn is_uninitialized(&self) -> bool {
        return self.get_tag().0 == JSTag::UNINITIALIZED.0;
    }

    pub const fn is_string(&self) -> bool {
        return self.get_tag().0 == JSTag::STRING.0;
    }

    pub const fn is_symbol(&self) -> bool {
        return self.get_tag().0 == JSTag::SYMBOL.0;
    }

    pub const fn is_object(&self) -> bool {
        return self.get_tag().0 == JSTag::OBJECT.0;
    }

    pub unsafe fn free_value(&self, ctx: NonNull<JSContext>) {
        if self.has_ref_count() {
            let p = self.get_ptr() as *mut JSRefCountHeader;
            (*p).ref_count -= 1;
            if (*p).ref_count <= 0 {
                __JS_FreeValue(ctx, *self);
            }
        }
    }

    pub unsafe fn free_value_rt(&self, rt: NonNull<JSRuntime>) {
        if self.has_ref_count() {
            let p = self.get_ptr() as *mut JSRefCountHeader;
            (*p).ref_count -= 1;
            if (*p).ref_count <= 0 {
                __JS_FreeValueRT(rt, *self);
            }
        }
    }

    pub unsafe fn dup_value(&self) -> JSValue {
        if self.has_ref_count() {
            let p = self.get_ptr() as *mut JSRefCountHeader;
            (*p).ref_count += 1;
        }
        *self
    }
}

#[repr(transparent)]
#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Default)]
pub struct JSTag(pub c_int);

impl JSTag {
    pub const FIRST: JSTag = JSTag(-9);
    pub const BIG_INT: JSTag = JSTag(-9);
    pub const SYMBOL: JSTag = JSTag(-8);
    pub const STRING: JSTag = JSTag(-7);
    pub const MODULE: JSTag = JSTag(-3);
    pub const FUNCTION_BYTECODE: JSTag = JSTag(-2);
    pub const OBJECT: JSTag = JSTag(-1);
    pub const INT: JSTag = JSTag(0);
    pub const BOOL: JSTag = JSTag(1);
    pub const NULL: JSTag = JSTag(2);
    pub const UNDEFINED: JSTag = JSTag(3);
    pub const UNINITIALIZED: JSTag = JSTag(4);
    pub const CATCH_OFFSET: JSTag = JSTag(5);
    pub const EXCEPTION: JSTag = JSTag(6);
    pub const FLOAT64: JSTag = JSTag(7);
}

impl Debug for JSTag {
    #[inline]
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(match *self {
            Self::BIG_INT => "BIG_INT",
            Self::SYMBOL => "SYMBOL",
            Self::STRING => "STRING",
            Self::MODULE => "MODULE",
            Self::FUNCTION_BYTECODE => "FUNCTION_BYTECODE",
            Self::OBJECT => "OBJECT",
            Self::INT => "INT",
            Self::BOOL => "BOOL",
            Self::NULL => "NULL",
            Self::UNDEFINED => "UNDEFINED",
            Self::UNINITIALIZED => "UNINITIALIZED",
            Self::CATCH_OFFSET => "CATCH_OFFSET",
            Self::EXCEPTION => "EXCEPTION",
            Self::FLOAT64 => "FLOAT64",
            _ => "(Unknown)",
        })
    }
}

#[repr(transparent)]
#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Debug, Default)]
pub struct JSProp(pub c_int);
bitflags_traits!(JSProp);

impl JSProp {
    pub const CONFIGURABLE: Self = Self(1 << 0);
    pub const WRITABLE: Self = Self(1 << 1);
    pub const ENUMERABLE: Self = Self(1 << 2);
    pub const C_W_E: Self = Self(Self::CONFIGURABLE.0 | Self::WRITABLE.0 | Self::ENUMERABLE.0);
    pub const LENGTH: Self = Self(1 << 3);

    pub const TMASK: Self = Self(3 << 4);
    pub const NORMAL: Self = Self(0 << 4);
    pub const GETSET: Self = Self(1 << 4);
    pub const VARREF: Self = Self(2 << 4);
    pub const AUTOINIT: Self = Self(3 << 4);

    pub const HAS_SHIFT: Self = Self(8);
    pub const HAS_CONFIGURABLE: Self = Self(1 << 8);
    pub const HAS_WRITABLE: Self = Self(1 << 9);
    pub const HAS_ENUMERABLE: Self = Self(1 << 10);
    pub const HAS_GET: Self = Self(1 << 11);
    pub const HAS_SET: Self = Self(1 << 12);
    pub const HAS_VALUE: Self = Self(1 << 13);

    pub const THROW: Self = Self(1 << 14);
    pub const THROW_STRICT: Self = Self(1 << 15);

    pub const NO_ADD: Self = Self(1 << 16);
    pub const NO_EXOTIC: Self = Self(1 << 17);
    pub const DEFINE_PROPERTY: Self = Self(1 << 18);
    pub const REFLECT_DEFINE_PROPERTY: Self = Self(1 << 19);
}

#[repr(transparent)]
#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Debug, Default)]
pub struct JSEvalType(pub c_int);
bitflags_traits!(JSEvalType);

impl JSEvalType {
    pub const GLOBAL: Self = Self(0 << 0);
    pub const MODULE: Self = Self(1 << 0);
    pub const DIRECT: Self = Self(2 << 0);
    pub const INDIRECT: Self = Self(3 << 0);
    pub const MASK: Self = Self(3 << 0);
}

#[repr(transparent)]
#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Debug, Default)]
pub struct JSEvalFlag(pub c_int);
bitflags_traits!(JSEvalFlag);

impl JSEvalFlag {
    pub const STRICT: Self = Self(1 << 3);
    pub const UNUSED: Self = Self(1 << 4);
    pub const COMPILE_ONLY: Self = Self(1 << 5);
    pub const BACKTRACE_BARRIER: Self = Self(1 << 6);
    pub const ASYNC: Self = Self(1 << 7);
}

#[repr(transparent)]
#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Debug, Default)]
pub struct JSGpn(pub c_int);
bitflags_traits!(JSGpn);

impl JSGpn {
    pub const STRING_MASK: Self = Self(1 << 0);
    pub const SYMBOL_MASK: Self = Self(1 << 1);
    pub const PRIVATE_MASK: Self = Self(1 << 2);
    pub const ENUM_ONLY: Self = Self(1 << 4);
    pub const SET_ENUM: Self = Self(1 << 5);
}

#[repr(transparent)]
#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Default)]
pub struct JSPromiseStateEnum(pub c_int);

impl JSPromiseStateEnum {
    pub const PENDING: Self = Self(0);
    pub const FULFILLED: Self = Self(1);
    pub const REJECTED: Self = Self(2);
}

impl Debug for JSPromiseStateEnum {
    #[inline]
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(match *self {
            Self::PENDING => "PENDING",
            Self::FULFILLED => "FULFILLED",
            Self::REJECTED => "REJECTED",
            _ => "(Unknown)",
        })
    }
}

#[repr(transparent)]
#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Debug, Default)]
pub struct JSWriteObj(pub c_int);
bitflags_traits!(JSWriteObj);

impl JSWriteObj {
    pub const BYTECODE: Self = Self(1 << 0);
    pub const BSWAP: Self = Self(0);
    pub const SAB: Self = Self(1 << 2);
    pub const REFERENCE: Self = Self(1 << 3);
    pub const STRIP_SOURCE: Self = Self(1 << 4);
    pub const STRIP_DEBUG: Self = Self(1 << 5);
}

#[repr(transparent)]
#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Debug, Default)]
pub struct JSReadObj(pub c_int);
bitflags_traits!(JSReadObj);

impl JSReadObj {
    pub const BYTECODE: Self = Self(1 << 0);
    pub const ROM_DATA: Self = Self(0);
    pub const SAB: Self = Self(1 << 2);
    pub const REFERENCE: Self = Self(1 << 3);
}

#[repr(transparent)]
#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Default)]
pub struct JSCFunctionEnum(pub c_int);

impl JSCFunctionEnum {
    pub const GENERIC: Self = Self(0);
    pub const GENERIC_MAGIC: Self = Self(1);
    pub const CONSTRUCTOR: Self = Self(2);
    pub const CONSTRUCTOR_MAGIC: Self = Self(3);
    pub const CONSTRUCTOR_OR_FUNC: Self = Self(4);
    pub const CONSTRUCTOR_OR_FUNC_MAGIC: Self = Self(5);
    pub const F_F: Self = Self(6);
    pub const F_F_F: Self = Self(7);
    pub const GETTER: Self = Self(8);
    pub const SETTER: Self = Self(9);
    pub const GETTER_MAGIC: Self = Self(10);
    pub const SETTER_MAGIC: Self = Self(11);
    pub const ITERATOR_NEXT: Self = Self(12);
}

impl Debug for JSCFunctionEnum {
    #[inline]
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(match *self {
            Self::GENERIC => "GENERIC",
            Self::GENERIC_MAGIC => "GENERIC_MAGIC",
            Self::CONSTRUCTOR => "CONSTRUCTOR",
            Self::CONSTRUCTOR_MAGIC => "CONSTRUCTOR_MAGIC",
            Self::CONSTRUCTOR_OR_FUNC => "CONSTRUCTOR_OR_FUNC",
            Self::CONSTRUCTOR_OR_FUNC_MAGIC => "CONSTRUCTOR_OR_FUNC_MAGIC",
            Self::F_F => "F_F",
            Self::F_F_F => "F_F_F",
            Self::GETTER => "GETTER",
            Self::SETTER => "SETTER",
            Self::GETTER_MAGIC => "GETTER_MAGIC",
            Self::SETTER_MAGIC => "SETTER_MAGIC",
            Self::ITERATOR_NEXT => "ITERATOR_NEXT",
            _ => "(Unknown)",
        })
    }
}

#[repr(transparent)]
#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Default)]
pub struct JSDef(pub u8);

impl JSDef {
    pub const CFUNC: Self = Self(0);
    pub const CGETSET: Self = Self(1);
    pub const CGETSET_MAGIC: Self = Self(2);
    pub const PROP_STRING: Self = Self(3);
    pub const PROP_INT32: Self = Self(4);
    pub const PROP_INT64: Self = Self(5);
    pub const PROP_DOUBLE: Self = Self(6);
    pub const PROP_UNDEFINED: Self = Self(7);
    pub const OBJECT: Self = Self(8);
    pub const ALIAS: Self = Self(9);
}

impl Debug for JSDef {
    #[inline]
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(match *self {
            Self::CFUNC => "CFUNC",
            Self::CGETSET => "CGETSET",
            Self::CGETSET_MAGIC => "CGETSET_MAGIC",
            Self::PROP_STRING => "PROP_STRING",
            Self::PROP_INT32 => "PROP_INT32",
            Self::PROP_INT64 => "PROP_INT64",
            Self::PROP_DOUBLE => "PROP_DOUBLE",
            Self::PROP_UNDEFINED => "PROP_UNDEFINED",
            Self::OBJECT => "OBJECT",
            Self::ALIAS => "ALIAS",
            _ => "(Unknown)",
        })
    }
}

#[repr(transparent)]
#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Debug, Default)]
pub struct JSCallFlag(pub c_int);
bitflags_traits!(JSCallFlag);

impl JSCallFlag {
    pub const CONSTRUCTOR: Self = Self(1 << 0);
}

mod fns {
    #![cfg_attr(rustfmt, rustfmt_skip)]

    use std::ffi::{c_int, c_void};
    use std::ptr::NonNull;
    use crate::*;

    pub type JSCFunctionFn = extern "C" fn(ctx: NonNull<JSContext>, this_val: JSValue, argc: c_int, argv: *mut JSValue) -> JSValue;
    pub type JSCFunctionMagicFn = extern "C" fn(ctx: NonNull<JSContext>, this_val: JSValue, argc: c_int, argv: *mut JSValue, magic: c_int) -> JSValue;
    pub type JSCFunctionDataFn = extern "C" fn(ctx: NonNull<JSContext>, this_val: JSValue, argc: c_int, argv: *mut JSValue, magic: c_int, func_data: *mut JSValue) -> JSValue;

    pub type JSMallocFn = extern "C" fn(s: *mut JSMallocState, size: usize) -> *mut c_void;
    pub type JSFreeFn = extern "C" fn(s: *mut JSMallocState, ptr: *mut c_void);
    pub type JSReallocFn = extern "C" fn(s: *mut JSMallocState, ptr: *mut c_void, size: usize) -> *mut c_void;
    pub type JSUsableSizeFn = extern "C" fn(ptr: *const c_void) -> usize;

    pub type JSMarkFunc = extern "C" fn(rt: NonNull<JSGCObjectHeader>, gp: *mut JSGCObjectHeader);

    pub type JSGetOwnPropertyFn = extern "C" fn(ctx: NonNull<JSContext>, desc: *mut JSPropertyDescriptor, obj: JSValue, prop: JSAtom) -> c_int;
    pub type JSGetOwnPropertyNamesFn = extern "C" fn(ctx: NonNull<JSContext>, ptab: *mut *mut JSPropertyEnum, plen: *mut u32, obj: JSValue) -> c_int;
    pub type JSDeletePropertyFn = extern "C" fn(ctx: NonNull<JSContext>, obj: JSValue, prop: JSAtom) -> c_int;
    pub type JSDefineOwnPropertyFn = extern "C" fn(ctx: NonNull<JSContext>, this_obj: JSValue, prop: JSAtom, val: JSValue, getter: JSValue, setter: JSValue, flags: c_int) -> c_int;
    pub type JSHasPropertyFn = extern "C" fn(ctx: NonNull<JSContext>, obj: JSValue, atom: JSAtom) -> c_int;
    pub type JSGetPropertyFn = extern "C" fn(ctx: NonNull<JSContext>, obj: JSValue, atom: JSAtom, receiver: JSValue) -> JSValue;
    pub type JSSetPropertyFn = extern "C" fn(ctx: NonNull<JSContext>, obj: JSValue, atom: JSAtom, value: JSValue, receiver: JSValue, flags: c_int) -> c_int;

    pub type JSClassFinalizerFn = extern "C" fn(rt: NonNull<JSRuntime>, val: JSValue);
    pub type JSClassGCMarkFn = extern "C" fn(rt: NonNull<JSRuntime>, val: JSValue, mark_func: JSMarkFunc);
    pub type JSClassCallFn = extern "C" fn(ctx: NonNull<JSContext>, func_obj: JSValue, this_val: JSValue, argc: c_int, argv: *mut JSValue, flags: c_int) -> JSValue;

    pub type JSFreeArrayBufferDataFn = extern "C" fn(rt: NonNull<JSRuntime>, opaque: *mut c_void, ptr: *mut c_void);

    pub type JSSabMallocFn = extern "C" fn(opaque: *mut c_void, size: usize) -> *mut c_void;
    pub type JSSabFreeFn = extern "C" fn(opaque: *mut c_void, ptr: *mut c_void);
    pub type JSSabDupFn = extern "C" fn(opaque: *mut c_void, ptr: *mut c_void);

    pub type JSHostPromiseRejectionTrackerFn = extern "C" fn(ctx: NonNull<JSContext>, promise: JSValue, reason: JSValue, is_handled: JSBool, opaque: *mut c_void);

    pub type JSInterruptHandlerFn = extern "C" fn(rt: NonNull<JSRuntime>, opaque: *mut c_void) -> c_int;

    pub type JSModuleNormalizeFn = extern "C" fn(ctx: NonNull<JSContext>, module_base_name: *const c_char, module_name: *const c_char, opaque: *mut c_void) -> *mut c_char;
    pub type JSModuleLoaderFn = extern "C" fn(ctx: NonNull<JSContext>, module_name: *const c_char, opaque: *mut c_void) -> Option<NonNull<JSModuleDef>>;

    pub type JSJobFn = extern "C" fn(ctx: NonNull<JSContext>, argc: c_int, argv: *mut JSValue) -> JSValue;

    pub type JSGenericMagicFn = extern "C" fn(ctx: NonNull<JSContext>, this_val: JSValue, argc: c_int, argv: *mut JSValue, magic: c_int) -> JSValue;
    pub type JSConstructorMagicFn = extern "C" fn(ctx: NonNull<JSContext>, new_target: JSValue, argc: c_int, argv: *mut JSValue, magic: c_int) -> JSValue;
    pub type JSFFFn = extern "C" fn(f64) -> f64;
    pub type JSFFFFn = extern "C" fn(f64, f64) -> f64;
    pub type JSGetterFn = extern "C" fn(ctx: NonNull<JSContext>, this_val: JSValue) -> JSValue;
    pub type JSSetterFn = extern "C" fn(ctx: NonNull<JSContext>, this_val: JSValue, val: JSValue) -> JSValue;
    pub type JSGetterMagicFn = extern "C" fn(ctx: NonNull<JSContext>, this_val: JSValue, magic: c_int) -> JSValue;
    pub type JSSetterMagicFn = extern "C" fn(ctx: NonNull<JSContext>, this_val: JSValue, val: JSValue, magic: c_int) -> JSValue;
    pub type JSIteratorNextFn = extern "C" fn(ctx: NonNull<JSContext>, this_val: JSValue, argc: c_int, argv: *mut JSValue, pdone: *mut c_int, magic: c_int) -> JSValue;

    pub type JSModuleInitFn = extern "C" fn(ctx: NonNull<JSContext>, m: NonNull<JSModuleDef>) -> c_int;
}
pub use fns::*;

#[repr(C)]
#[derive(Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Debug, Default)]
pub struct JSRefCountHeader {
    pub ref_count: c_int,
}

#[repr(C)]
#[derive(Clone)]
pub struct JSMallocState {
    pub malloc_count: usize,
    pub malloc_size: usize,
    pub malloc_limit: usize,
    pub opaque: *mut c_void,
}

#[repr(C)]
#[derive(Clone, Default)]
pub struct JSMallocFunctions {
    pub js_malloc: Option<JSMallocFn>,
    pub js_free: Option<JSFreeFn>,
    pub js_realloc: Option<JSReallocFn>,
    pub js_malloc_usable_size: Option<JSUsableSizeFn>,
}

#[repr(C)]
#[derive(Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Debug, Default)]
pub struct JSMemoryUsage {
    pub malloc_size: i64,
    pub malloc_limit: i64,
    pub memory_used_size: i64,
    pub malloc_count: i64,
    pub memory_used_count: i64,
    pub atom_count: i64,
    pub atom_size: i64,
    pub str_count: i64,
    pub str_size: i64,
    pub obj_count: i64,
    pub obj_size: i64,
    pub prop_count: i64,
    pub prop_size: i64,
    pub shape_count: i64,
    pub shape_size: i64,
    pub js_func_count: i64,
    pub js_func_size: i64,
    pub js_func_code_size: i64,
    pub js_func_pc2line_count: i64,
    pub js_func_pc2line_size: i64,
    pub c_func_count: i64,
    pub array_count: i64,
    pub fast_array_count: i64,
    pub fast_array_elements: i64,
    pub binary_object_count: i64,
    pub binary_object_size: i64,
}

#[repr(C)]
#[derive(Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Debug, Default)]
pub struct JSPropertyEnum {
    pub is_enumerable: JSBool,
    pub atom: Option<JSAtom>,
}

#[repr(C)]
#[derive(Clone)]
pub struct JSPropertyDescriptor {
    pub flags: c_int,
    pub value: JSValue,
    pub getter: JSValue,
    pub setter: JSValue,
}

#[repr(C)]
#[derive(Clone)]
pub struct JSClassExoticMethods {
    pub get_own_property: JSGetOwnPropertyFn,
    pub get_own_property_names: JSGetOwnPropertyNamesFn,
    pub delete_property: JSDeletePropertyFn,
    pub define_own_property: JSDefineOwnPropertyFn,
    pub has_property: Option<JSHasPropertyFn>,
    pub get_property: Option<JSGetPropertyFn>,
    pub set_property: Option<JSSetPropertyFn>,
}

#[repr(C)]
#[derive(Clone)]
pub struct JSClassDef {
    pub class_name: *const c_char,
    pub finalizer: JSClassFinalizerFn,
    pub gc_mark: JSClassGCMarkFn,
    pub call: JSClassCallFn,
    pub exotic: *mut JSClassExoticMethods,
}

#[repr(C)]
#[derive(Clone)]
pub struct JSSharedArrayBufferFunctions {
    pub sab_alloc: JSSabMallocFn,
    pub sab_free: JSSabFreeFn,
    pub sab_dup: JSSabDupFn,
    pub sab_opaque: *mut c_void,
}

// /* Structure to retrieve (de)serialized SharedArrayBuffer objects. */
// #[repr(C)]
// pub struct JSSABTab {
//     pub tab: *mut *mut u8,
//     pub len: usize,
// }

#[repr(C)]
#[derive(Copy, Clone)]
pub union JSCFunctionType {
    pub generic: JSCFunctionFn,
    pub generic_magic: JSGenericMagicFn,
    pub constructor: JSCFunctionFn,
    pub constructor_magic: JSConstructorMagicFn,
    pub constructor_or_func: JSCFunctionFn,
    pub f_f: JSFFFn,
    pub f_f_f: JSFFFFn,
    pub getter: JSGetterFn,
    pub setter: JSSetterFn,
    pub getter_magic: JSGetterMagicFn,
    pub setter_magic: JSSetterMagicFn,
    pub iterator_next: JSIteratorNextFn,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct JSCFunctionListEntryUnionFunc {
    pub length: u8, /* XXX: should move outside union */
    pub cproto: u8, /* XXX: should move outside union */
    pub cfunc: JSCFunctionType,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct JSCFunctionListEntryUnionGetSet {
    pub get: JSCFunctionType,
    pub set: JSCFunctionType,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct JSCFunctionListEntryUnionAlias {
    pub name: *const c_char,
    pub base: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct JSCFunctionListEntryUnionPropList {
    pub tab: *const JSCFunctionListEntry,
    pub len: c_int,
}

#[repr(C)]
pub union JSCFunctionListEntryUnion {
    pub func: JSCFunctionListEntryUnionFunc,
    pub getset: JSCFunctionListEntryUnionGetSet,
    pub alias: JSCFunctionListEntryUnionAlias,
    pub prop_list: JSCFunctionListEntryUnionPropList,
    pub _str: *const c_char, /* pure ASCII or UTF-8 encoded */
    pub _i32: i32,
    pub _i64: i64,
    pub _f64: f64,
}

#[repr(C)]
pub struct JSCFunctionListEntry {
    pub name: *const c_char, /* pure ASCII or UTF-8 encoded */
    pub prop_flags: u8,
    pub def_type: u8,
    pub magic: i16,
    pub u: JSCFunctionListEntryUnion,
}

type Dun = JSCFunctionListEntryUnion;
impl JSCFunctionListEntry {
    pub const fn func_def(
        name: *const c_char,
        length: u8,
        func1: JSCFunctionFn,
    ) -> JSCFunctionListEntry {
        JSCFunctionListEntry {
            name,
            prop_flags: (JSProp::WRITABLE.0 | JSProp::CONFIGURABLE.0) as u8,
            def_type: JSDef::CFUNC.0,
            magic: 0,
            u: Dun {
                func: JSCFunctionListEntryUnionFunc {
                    length,
                    cproto: JSCFunctionEnum::GENERIC.0 as u8,
                    cfunc: JSCFunctionType { generic: func1 },
                },
            },
        }
    }

    pub const fn func_def2(
        name: *const c_char,
        length: u8,
        func1: JSCFunctionFn,
        prop_flags: JSProp,
    ) -> JSCFunctionListEntry {
        JSCFunctionListEntry {
            name,
            prop_flags: prop_flags.0 as u8,
            def_type: JSDef::CFUNC.0,
            magic: 0,
            u: Dun {
                func: JSCFunctionListEntryUnionFunc {
                    length,
                    cproto: JSCFunctionEnum::GENERIC.0 as u8,
                    cfunc: JSCFunctionType { generic: func1 },
                },
            },
        }
    }

    pub const fn func_magic_def(
        name: *const c_char,
        length: u8,
        func1: JSCFunctionMagicFn,
        magic: i16,
    ) -> JSCFunctionListEntry {
        JSCFunctionListEntry {
            name,
            prop_flags: (JSProp::WRITABLE.0 | JSProp::CONFIGURABLE.0) as u8,
            def_type: JSDef::CFUNC.0,
            magic,
            u: Dun {
                func: JSCFunctionListEntryUnionFunc {
                    length,
                    cproto: JSCFunctionEnum::GENERIC_MAGIC.0 as u8,
                    cfunc: JSCFunctionType {
                        generic_magic: func1,
                    },
                },
            },
        }
    }

    pub const fn func_special_def(
        name: *const c_char,
        length: u8,
        cproto: JSCFunctionEnum,
        func1: JSCFunctionType,
    ) -> JSCFunctionListEntry {
        JSCFunctionListEntry {
            name,
            prop_flags: (JSProp::WRITABLE.0 | JSProp::CONFIGURABLE.0) as u8,
            def_type: JSDef::CFUNC.0,
            magic: 0,
            u: Dun {
                func: JSCFunctionListEntryUnionFunc {
                    length,
                    cproto: cproto.0 as u8,
                    cfunc: func1,
                },
            },
        }
    }

    pub const fn iterator_next_def(
        name: *const c_char,
        length: u8,
        func1: JSIteratorNextFn,
        magic: i16,
    ) -> JSCFunctionListEntry {
        JSCFunctionListEntry {
            name,
            prop_flags: (JSProp::WRITABLE.0 | JSProp::CONFIGURABLE.0) as u8,
            def_type: JSDef::CFUNC.0,
            magic,
            u: Dun {
                func: JSCFunctionListEntryUnionFunc {
                    length,
                    cproto: JSCFunctionEnum::ITERATOR_NEXT.0 as u8,
                    cfunc: JSCFunctionType {
                        iterator_next: func1,
                    },
                },
            },
        }
    }

    pub const fn getset_def(
        name: *const c_char,
        fgetter: JSGetterFn,
        fsetter: JSSetterFn,
    ) -> JSCFunctionListEntry {
        JSCFunctionListEntry {
            name,
            prop_flags: JSProp::CONFIGURABLE.0 as u8,
            def_type: JSDef::CGETSET.0,
            magic: 0,
            u: Dun {
                getset: JSCFunctionListEntryUnionGetSet {
                    get: JSCFunctionType { getter: fgetter },
                    set: JSCFunctionType { setter: fsetter },
                },
            },
        }
    }

    pub const fn getset_magic_def(
        name: *const c_char,
        fgetter: JSGetterMagicFn,
        fsetter: JSSetterMagicFn,
        magic: i16,
    ) -> JSCFunctionListEntry {
        JSCFunctionListEntry {
            name,
            prop_flags: JSProp::CONFIGURABLE.0 as u8,
            def_type: JSDef::CGETSET_MAGIC.0,
            magic,
            u: Dun {
                getset: JSCFunctionListEntryUnionGetSet {
                    get: JSCFunctionType {
                        getter_magic: fgetter,
                    },
                    set: JSCFunctionType {
                        setter_magic: fsetter,
                    },
                },
            },
        }
    }

    pub const fn prop_string_def(
        name: *const c_char,
        cstr: *const c_char,
        prop_flags: JSProp,
    ) -> JSCFunctionListEntry {
        JSCFunctionListEntry {
            name,
            prop_flags: prop_flags.0 as u8,
            def_type: JSDef::PROP_STRING.0,
            magic: 0,
            u: Dun { _str: cstr },
        }
    }

    pub const fn prop_int32_def(
        name: *const c_char,
        val: i32,
        prop_flags: JSProp,
    ) -> JSCFunctionListEntry {
        JSCFunctionListEntry {
            name,
            prop_flags: prop_flags.0 as u8,
            def_type: JSDef::PROP_INT32.0,
            magic: 0,
            u: Dun { _i32: val },
        }
    }

    pub const fn prop_int64_def(
        name: *const c_char,
        val: i64,
        prop_flags: JSProp,
    ) -> JSCFunctionListEntry {
        JSCFunctionListEntry {
            name,
            prop_flags: prop_flags.0 as u8,
            def_type: JSDef::PROP_INT64.0,
            magic: 0,
            u: Dun { _i64: val },
        }
    }

    pub const fn prop_double_def(
        name: *const c_char,
        val: f64,
        prop_flags: JSProp,
    ) -> JSCFunctionListEntry {
        JSCFunctionListEntry {
            name,
            prop_flags: prop_flags.0 as u8,
            def_type: JSDef::PROP_DOUBLE.0,
            magic: 0,
            u: Dun { _f64: val },
        }
    }

    pub const fn prop_undefined_def(
        name: *const c_char,
        prop_flags: JSProp,
    ) -> JSCFunctionListEntry {
        JSCFunctionListEntry {
            name,
            prop_flags: prop_flags.0 as u8,
            def_type: JSDef::PROP_UNDEFINED.0,
            magic: 0,
            u: Dun { _i32: 0 },
        }
    }

    pub const fn object_def(
        name: *const c_char,
        tab: *const JSCFunctionListEntry,
        len: c_int,
        prop_flags: JSProp,
    ) -> JSCFunctionListEntry {
        JSCFunctionListEntry {
            name,
            prop_flags: prop_flags.0 as u8,
            def_type: JSDef::OBJECT.0,
            magic: 0,
            u: Dun {
                prop_list: JSCFunctionListEntryUnionPropList { tab, len },
            },
        }
    }

    pub const fn alias_def(name: *const c_char, from: *const c_char) -> JSCFunctionListEntry {
        JSCFunctionListEntry {
            name,
            prop_flags: (JSProp::WRITABLE.0 | JSProp::CONFIGURABLE.0) as u8,
            def_type: JSDef::ALIAS.0,
            magic: 0,
            u: Dun {
                alias: JSCFunctionListEntryUnionAlias {
                    name: from,
                    base: -1,
                },
            },
        }
    }

    pub const fn alias_base_def(
        name: *const c_char,
        from: *const c_char,
        base: c_int,
    ) -> JSCFunctionListEntry {
        JSCFunctionListEntry {
            name,
            prop_flags: (JSProp::WRITABLE.0 | JSProp::CONFIGURABLE.0) as u8,
            def_type: JSDef::ALIAS.0,
            magic: 0,
            u: Dun {
                alias: JSCFunctionListEntryUnionAlias { name: from, base },
            },
        }
    }
}

pub unsafe fn JS_ToCStringLen(
    ctx: NonNull<JSContext>,
    plen: *mut usize,
    v1: JSValue,
) -> *const c_char {
    return JS_ToCStringLen2(ctx, plen, v1, JSBool::FALSE);
}

pub unsafe fn JS_ToCString(ctx: NonNull<JSContext>, v1: JSValue) -> *const c_char {
    return JS_ToCStringLen2(ctx, std::ptr::null_mut(), v1, JSBool::FALSE);
}

pub unsafe fn JS_NewCFunction(
    ctx: NonNull<JSContext>,
    func: JSCFunctionFn,
    name: *const c_char,
    length: c_int,
) -> JSValue {
    return JS_NewCFunction2(ctx, func, name, length, JSCFunctionEnum::GENERIC, 0);
}

pub unsafe fn JS_NewCFunctionMagic(
    ctx: NonNull<JSContext>,
    func: JSCFunctionMagicFn,
    name: *const c_char,
    length: c_int,
    cproto: JSCFunctionEnum,
    magic: c_int,
) -> JSValue {
    let ft = JSCFunctionType {
        generic_magic: func,
    };
    return JS_NewCFunction2(ctx, ft.generic, name, length, cproto, magic);
}

extern "C" {
    #![cfg_attr(rustfmt, rustfmt_skip)]

    pub fn JS_NewRuntime() -> Option<NonNull<JSRuntime>>;
    pub fn JS_SetRuntimeInfo(rt: NonNull<JSRuntime>, info: *const c_char);
    pub fn JS_SetMemoryLimit(rt: NonNull<JSRuntime>, limit: usize);
    pub fn JS_SetDumpFlags(rt: NonNull<JSRuntime>, flags: u64);
    pub fn JS_GetGCThreshold(rt: NonNull<JSRuntime>) -> usize;
    pub fn JS_SetGCThreshold(rt: NonNull<JSRuntime>, gc_threshold: usize);
    pub fn JS_SetMaxStackSize(rt: NonNull<JSRuntime>, stack_size: usize);
    pub fn JS_UpdateStackTop(rt: NonNull<JSRuntime>);
    pub fn JS_NewRuntime2(mf: *const JSMallocFunctions, opaque: *mut c_void) -> Option<NonNull<JSRuntime>>;
    pub fn JS_FreeRuntime(rt: NonNull<JSRuntime>);
    pub fn JS_GetRuntimeOpaque(rt: NonNull<JSRuntime>) -> *mut c_void;
    pub fn JS_SetRuntimeOpaque(rt: NonNull<JSRuntime>, opaque: *mut c_void);

    pub fn JS_MarkValue(rt: NonNull<JSRuntime>, val: JSValue, mark_func: Option<JSMarkFunc>);
    pub fn JS_RunGC(rt: NonNull<JSRuntime>);
    pub fn JS_IsLiveObject(rt: NonNull<JSRuntime>, obj: JSValue) -> JSBool;

    pub fn JS_NewContext(rt: NonNull<JSRuntime>) -> Option<NonNull<JSContext>>;
    pub fn JS_FreeContext(s: NonNull<JSContext>);
    pub fn JS_DupContext(ctx: NonNull<JSContext>) -> Option<NonNull<JSContext>>;
    pub fn JS_GetContextOpaque(ctx: NonNull<JSContext>) -> *mut c_void;
    pub fn JS_SetContextOpaque(ctx: NonNull<JSContext>, opaque: *mut c_void);
    pub fn JS_GetRuntime(ctx: NonNull<JSContext>) -> Option<NonNull<JSRuntime>>;
    pub fn JS_SetClassProto(ctx: NonNull<JSContext>, class_id: JSClassID, obj: JSValue);
    pub fn JS_GetClassProto(ctx: NonNull<JSContext>, class_id: JSClassID) -> JSValue;

    pub fn JS_NewContextRaw(rt: NonNull<JSRuntime>) -> Option<NonNull<JSContext>>;
    pub fn JS_AddIntrinsicBaseObjects(ctx: NonNull<JSContext>);
    pub fn JS_AddIntrinsicDate(ctx: NonNull<JSContext>);
    pub fn JS_AddIntrinsicEval(ctx: NonNull<JSContext>);
    pub fn JS_AddIntrinsicRegExpCompiler(ctx: NonNull<JSContext>);
    pub fn JS_AddIntrinsicRegExp(ctx: NonNull<JSContext>);
    pub fn JS_AddIntrinsicJSON(ctx: NonNull<JSContext>);
    pub fn JS_AddIntrinsicProxy(ctx: NonNull<JSContext>);
    pub fn JS_AddIntrinsicMapSet(ctx: NonNull<JSContext>);
    pub fn JS_AddIntrinsicTypedArrays(ctx: NonNull<JSContext>);
    pub fn JS_AddIntrinsicPromise(ctx: NonNull<JSContext>);
    pub fn JS_AddIntrinsicBigInt(ctx: NonNull<JSContext>);
    pub fn JS_AddIntrinsicWeakRef(ctx: NonNull<JSContext>);
    pub fn JS_AddPerformance(ctx: NonNull<JSContext>);

    pub fn JS_IsEqual(ctx: NonNull<JSContext>, op1: JSValue, op2: JSValue) -> c_int;
    pub fn JS_IsStrictEqual(ctx: NonNull<JSContext>, op1: JSValue, op2: JSValue) -> JSBool;
    pub fn JS_IsSameValue(ctx: NonNull<JSContext>, op1: JSValue, op2: JSValue) -> JSBool;
    pub fn JS_IsSameValueZero(ctx: NonNull<JSContext>, op1: JSValue, op2: JSValue) -> JSBool;

    pub fn js_malloc_rt(rt: NonNull<JSRuntime>, size: usize) -> *mut c_void;
    pub fn js_free_rt(rt: NonNull<JSRuntime>, ptr: *mut c_void);
    pub fn js_realloc_rt(rt: NonNull<JSRuntime>, ptr: *mut c_void, size: usize) -> *mut c_void;
    pub fn js_malloc_usable_size_rt(rt: NonNull<JSRuntime>, ptr: *const c_void) -> usize;
    pub fn js_mallocz_rt(rt: NonNull<JSRuntime>, size: usize);

    pub fn js_malloc(ctx: NonNull<JSContext>, size: usize) -> *mut c_void;
    pub fn js_free(ctx: NonNull<JSContext>, ptr: *mut c_void);
    pub fn js_realloc(ctx: NonNull<JSContext>, ptr: *mut c_void, size: usize) -> *mut c_void;
    pub fn js_malloc_usable_size(ctx: NonNull<JSContext>, ptr: *const c_void) -> usize;
    pub fn js_realloc2(ctx: NonNull<JSContext>, ptr: *mut c_void, size: usize, pslack: *mut usize) -> *mut c_void;
    pub fn js_mallocz(ctx: NonNull<JSContext>, size: usize) -> *mut c_void;
    pub fn js_strdup(ctx: NonNull<JSContext>, str: *const c_char) -> *const c_char;
    pub fn js_strndup(ctx: NonNull<JSContext>, str: *const c_char, n: usize) -> *const c_char;

    pub fn JS_ComputeMemoryUsage(rt: NonNull<JSRuntime>, s: *mut JSMemoryUsage);
    // fn JS_DumpMemoryUsage(fp: *mut FILE, s: *const JSMemoryUsage, rt: NonNull<JSRuntime>);

    pub fn JS_NewAtomLen(ctx: NonNull<JSContext>, str: *const c_char, len: usize) -> Option<JSAtom>;
    pub fn JS_NewAtom(ctx: NonNull<JSContext>, str: *const c_char) -> Option<JSAtom>;
    pub fn JS_NewAtomUInt32(ctx: NonNull<JSContext>, n: u32) -> Option<JSAtom>;
    pub fn JS_DupAtom(ctx: NonNull<JSContext>, v: JSAtom) -> Option<JSAtom>;
    pub fn JS_FreeAtom(ctx: NonNull<JSContext>, v: JSAtom);
    pub fn JS_FreeAtomRT(rt: NonNull<JSRuntime>, v: JSAtom);
    pub fn JS_AtomToValue(ctx: NonNull<JSContext>, atom: JSAtom) -> JSValue;
    pub fn JS_AtomToString(ctx: NonNull<JSContext>, atom: JSAtom) -> JSValue;
    pub fn JS_AtomToCString(ctx: NonNull<JSContext>, atom: JSAtom) -> *const c_char;
    pub fn JS_ValueToAtom(ctx: NonNull<JSContext>, val: JSValue) -> Option<JSAtom>;

    pub fn JS_NewClassID(rt: NonNull<JSRuntime>, pclass_id: *mut Option<JSClassID>) -> Option<JSClassID>;
    pub fn JS_GetClassID(v: JSValue) -> Option<JSClassID>;
    pub fn JS_NewClass(rt: NonNull<JSRuntime>, class_id: JSClassID, class_def: *const JSClassDef) -> c_int;
    pub fn JS_IsRegisteredClass(rt: NonNull<JSRuntime>, class_id: JSClassID) -> c_int;

    pub fn JS_NewBigInt64(ctx: NonNull<JSContext>, v: i64) -> JSValue;
    pub fn JS_NewBigUint64(ctx: NonNull<JSContext>, v: u64) -> JSValue;

    pub fn JS_Throw(ctx: NonNull<JSContext>, obj: JSValue) -> JSValue;
    pub fn JS_GetException(ctx: NonNull<JSContext>) -> JSValue;
    pub fn JS_IsError(ctx: NonNull<JSContext>, val: JSValue) -> JSBool;
    pub fn JS_ResetUncatchableError(ctx: NonNull<JSContext>);
    pub fn JS_NewError(ctx: NonNull<JSContext>) -> JSValue;
    pub fn JS_ThrowPlainError(ctx: NonNull<JSContext>, fmt: *const c_char, ...) -> JSValue;
    pub fn JS_ThrowSyntaxError(ctx: NonNull<JSContext>, fmt: *const c_char, ...) -> JSValue;
    pub fn JS_ThrowTypeError(ctx: NonNull<JSContext>, fmt: *const c_char, ...) -> JSValue;
    pub fn JS_ThrowReferenceError(ctx: NonNull<JSContext>, fmt: *const c_char, ...) -> JSValue;
    pub fn JS_ThrowRangeError(ctx: NonNull<JSContext>, fmt: *const c_char, ...) -> JSValue;
    pub fn JS_ThrowInternalError(ctx: NonNull<JSContext>, fmt: *const c_char, ...) -> JSValue;
    pub fn JS_ThrowOutOfMemory(ctx: NonNull<JSContext>) -> JSValue;

    pub fn __JS_FreeValue(ctx: NonNull<JSContext>, v: JSValue);
    pub fn __JS_FreeValueRT(rt: NonNull<JSRuntime>, v: JSValue);

    pub fn JS_ToBool(ctx: NonNull<JSContext>, v: JSValue) -> c_int;
    pub fn JS_ToInt32(ctx: NonNull<JSContext>, pres: *mut i32, v: JSValue) -> c_int;
    pub fn JS_ToInt64(ctx: NonNull<JSContext>, pres: *mut i64, v: JSValue) -> c_int;
    pub fn JS_ToIndex(ctx: NonNull<JSContext>, plen: *mut u64, v: JSValue) -> c_int;
    pub fn JS_ToFloat64(ctx: NonNull<JSContext>, pres: *mut f64, v: JSValue) -> c_int;
    pub fn JS_ToBigInt64(ctx: NonNull<JSContext>, pres: *mut i64, v: JSValue) -> c_int;
    pub fn JS_ToBigUint64(ctx: NonNull<JSContext>, plen: *mut u64, v: JSValue) -> c_int;
    pub fn JS_ToInt64Ext(ctx: NonNull<JSContext>, pres: *mut i64, v: JSValue) -> c_int;

    pub fn JS_NewStringLen(ctx: NonNull<JSContext>, str1: *const c_char, len1: usize) -> JSValue;
    pub fn JS_NewAtomString(ctx: NonNull<JSContext>, str: *const c_char) -> JSValue;

    pub fn JS_ToString(ctx: NonNull<JSContext>, v: JSValue) -> JSValue;
    pub fn JS_ToPropertyKey(ctx: NonNull<JSContext>, v: JSValue) -> JSValue;
    pub fn JS_ToCStringLen2(ctx: NonNull<JSContext>, plen: *mut usize, v1: JSValue, cesu8: JSBool) -> *const c_char;

    pub fn JS_FreeCString(ctx: NonNull<JSContext>, ptr: *const c_char);

    pub fn JS_NewObjectProtoClass(ctx: NonNull<JSContext>, proto: JSValue, class_id: JSClassID) -> JSValue;
    pub fn JS_NewObjectClass(ctx: NonNull<JSContext>, class_id: c_int) -> JSValue;
    pub fn JS_NewObjectProto(ctx: NonNull<JSContext>, proto: JSValue) -> JSValue;
    pub fn JS_NewObject(ctx: NonNull<JSContext>) -> JSValue;

    pub fn JS_IsFunction(ctx: NonNull<JSContext>, v: JSValue) -> JSBool;
    pub fn JS_IsConstructor(ctx: NonNull<JSContext>, v: JSValue) -> JSBool;
    pub fn JS_SetConstructorBit(ctx: NonNull<JSContext>, func_obj: JSValue, val: JSBool) -> JSBool;

    pub fn JS_NewArray(ctx: NonNull<JSContext>) -> JSValue;
    pub fn JS_IsArray(ctx: NonNull<JSContext>, v: JSValue) -> c_int;

    pub fn JS_NewDate(ctx: NonNull<JSContext>, epoch_ms: f64) -> JSValue;

    pub fn JS_GetProperty(ctx: NonNull<JSContext>, this_obj: JSValue, prop: JSAtom) -> JSValue;
    pub fn JS_GetPropertyUint32(ctx: NonNull<JSContext>, this_obj: JSValue, idx: u32) -> JSValue;
    pub fn JS_GetPropertyInt64(ctx: NonNull<JSContext>, this_obj: JSValue, idx: i64) -> JSValue;
    pub fn JS_GetPropertyStr(ctx: NonNull<JSContext>, this_obj: JSValue, prop: *const c_char) -> JSValue;

    pub fn JS_SetProperty(ctx: NonNull<JSContext>, this_obj: JSValue, prop: JSAtom, v: JSValue) -> c_int;
    pub fn JS_SetPropertyUint32(ctx: NonNull<JSContext>, this_obj: JSValue, idx: u32, v: JSValue) -> c_int;
    pub fn JS_SetPropertyInt64(ctx: NonNull<JSContext>, this_obj: JSValue, idx: i64, v: JSValue) -> c_int;
    pub fn JS_SetPropertyStr(ctx: NonNull<JSContext>, this_obj: JSValue, prop: *const c_char, v: JSValue) -> c_int;
    pub fn JS_HasProperty(ctx: NonNull<JSContext>, this_obj: JSValue, prop: JSAtom) -> c_int;
    pub fn JS_IsExtensible(ctx: NonNull<JSContext>, obj: JSValue) -> c_int;
    pub fn JS_PreventExtensions(ctx: NonNull<JSContext>, obj: JSValue) -> c_int;
    pub fn JS_DeleteProperty(ctx: NonNull<JSContext>, obj: JSValue, prop: JSAtom, flags: c_int) -> c_int;
    pub fn JS_SetPrototype(ctx: NonNull<JSContext>, obj: JSValue, proto_val: JSValue) -> c_int;
    pub fn JS_GetPrototype(ctx: NonNull<JSContext>, v: JSValue) -> JSValue;
    pub fn JS_GetLength(ctx: NonNull<JSContext>, obj: JSValue, pres: *mut i64) -> c_int;

    pub fn JS_GetOwnPropertyNames(ctx: NonNull<JSContext>, ptab: *mut *mut JSPropertyEnum, plen: *mut u32, obj: JSValue, flags: c_uint) -> c_int;
    pub fn JS_GetOwnProperty(ctx: NonNull<JSContext>, desc: *mut JSPropertyDescriptor, obj: JSValue, prop: JSAtom) -> c_int;
    pub fn JS_FreePropertyEnum(ctx: NonNull<JSContext>, tab: *mut JSPropertyEnum, len: u32);

    pub fn JS_Call(ctx: NonNull<JSContext>, func_obj: JSValue, this_obj: JSValue, argc: c_int, argv: *mut JSValue) -> JSValue;
    pub fn JS_Invoke(ctx: NonNull<JSContext>, this_val: JSValue, atom: JSAtom, argc: c_int, argv: *mut JSValue) -> JSValue;
    pub fn JS_CallConstructor(ctx: NonNull<JSContext>, func_obj: JSValue, argc: c_int, argv: *mut JSValue) -> JSValue;
    pub fn JS_CallConstructor2(ctx: NonNull<JSContext>, func_obj: JSValue, new_target: JSValue, argc: c_int, argv: *mut JSValue) -> JSValue;
    pub fn JS_DetectModule(input: *const c_char, input_len: usize) -> JSBool;
    pub fn JS_Eval(ctx: NonNull<JSContext>, input: *const c_char, input_len: usize, filename: *const c_char, eval_flags: JSEvalType) -> JSValue;
    pub fn JS_EvalThis(ctx: NonNull<JSContext>, this_obj: JSValue, input: *const c_char, input_len: usize, filename: *const c_char, eval_flags: JSEvalType) -> JSValue;
    pub fn JS_GetGlobalObject(ctx: NonNull<JSContext>) -> JSValue;
    pub fn JS_IsInstanceOf(ctx: NonNull<JSContext>, val: JSValue, obj: JSValue) -> c_int;
    pub fn JS_DefineProperty(ctx: NonNull<JSContext>, this_obj: JSValue, prop: JSAtom, val: JSValue, getter: JSValue, setter: JSValue, flags: c_uint) -> c_int;
    pub fn JS_DefinePropertyValue(ctx: NonNull<JSContext>, this_obj: JSValue, prop: JSAtom, val: JSValue, flags: c_uint) -> c_int;
    pub fn JS_DefinePropertyValueUint32(ctx: NonNull<JSContext>, this_obj: JSValue, idx: u32, val: JSValue, flags: c_uint) -> c_int;
    pub fn JS_DefinePropertyValueStr(ctx: NonNull<JSContext>, this_obj: JSValue, prop: *const c_char, val: JSValue, flags: c_uint) -> c_int;
    pub fn JS_DefinePropertyGetSet(ctx: NonNull<JSContext>, this_obj: JSValue, prop: JSAtom, getter: JSValue, setter: JSValue, flags: c_uint) -> c_int;
    pub fn JS_SetOpaque(obj: JSValue, opaque: *mut c_void);
    pub fn JS_GetOpaque(obj: JSValue, class_id: JSClassID) -> *mut c_void;
    pub fn JS_GetOpaque2(ctx: NonNull<JSContext>, obj: JSValue, class_id: JSClassID) -> *mut c_void;
    pub fn JS_GetAnyOpaque(obj: JSValue, class_id: *mut Option<JSClassID>) -> *mut c_void;

    pub fn JS_ParseJSON(ctx: NonNull<JSContext>, buf: *const c_char, buf_len: usize, filename: *const c_char) -> JSValue;
    pub fn JS_JSONStringify(ctx: NonNull<JSContext>, obj: JSValue, replacer: JSValue, space0: JSValue) -> JSValue;

    pub fn JS_NewArrayBuffer(ctx: NonNull<JSContext>, buf: *mut u8, len: usize, free_func: JSFreeArrayBufferDataFn, opaque: *mut c_void, is_shared: JSBool) -> JSValue;
    pub fn JS_NewArrayBufferCopy(ctx: NonNull<JSContext>, buf: *const u8, len: usize) -> JSValue;
    pub fn JS_DetachArrayBuffer(ctx: NonNull<JSContext>, obj: JSValue);
    pub fn JS_GetArrayBuffer(ctx: NonNull<JSContext>, psize: *mut usize, obj: JSValue) -> *mut u8;
    pub fn JS_IsArrayBuffer(obj: JSValue) -> JSBool;
    pub fn JS_GetUint8Array(ctx: NonNull<JSContext>, psize: *mut usize, obj: JSValue) -> *mut u8;
    pub fn JS_GetTypedArrayBuffer(ctx: NonNull<JSContext>, obj: JSValue, pbyte_offset: *mut usize, pbyte_length: *mut usize, pbytes_per_element: *mut usize) -> JSValue;
    pub fn JS_NewUint8Array(ctx: NonNull<JSContext>, buf: *mut u8, len: usize, free_func: JSFreeArrayBufferDataFn, opaque: *mut c_void, is_shared: JSBool) -> JSValue;
    pub fn JS_IsUint8Array(obj: JSValue) -> JSBool;
    pub fn JS_NewUint8ArrayCopy(ctx: NonNull<JSContext>, buf: *const u8, len: usize) -> JSValue;

    pub fn JS_SetSharedArrayBufferFunctions(rt: NonNull<JSRuntime>, sf: *const JSSharedArrayBufferFunctions);

    pub fn JS_NewPromiseCapability(ctx: NonNull<JSContext>, resolving_funcs: *mut JSValue) -> JSValue;
    pub fn JS_PromiseState(ctx: NonNull<JSContext>, promise: JSValue) -> JSPromiseStateEnum;
    pub fn JS_PromiseResult(ctx: NonNull<JSContext>, promise: JSValue) -> JSValue;

    pub fn JS_NewSymbol(ctx: NonNull<JSContext>, description: *const c_char, is_global: JSBool) -> JSValue;

    pub fn JS_SetHostPromiseRejectionTracker(rt: NonNull<JSRuntime>, cb: *mut JSHostPromiseRejectionTrackerFn, opaque: *mut c_void);

    pub fn JS_SetInterruptHandler(rt: NonNull<JSRuntime>, cb: *mut JSInterruptHandlerFn, opaque: *mut c_void);
    pub fn JS_SetCanBlock(rt: NonNull<JSRuntime>, can_block: JSBool);
    pub fn JS_SetIsHTMLDDA(ctx: NonNull<JSContext>, obj: JSValue);

    pub fn JS_SetModuleLoaderFunc(rt: NonNull<JSRuntime>, module_normalize: JSModuleNormalizeFn, module_loader: JSModuleLoaderFn, opaque: *mut c_void);
    pub fn JS_GetImportMeta(ctx: NonNull<JSContext>, m: NonNull<JSModuleDef>) -> JSValue;
    pub fn JS_GetModuleName(ctx: NonNull<JSContext>, m: NonNull<JSModuleDef>) -> Option<JSAtom>;
    pub fn JS_GetModuleNamespace(ctx: NonNull<JSContext>, m: NonNull<JSModuleDef>) -> JSValue; // wasn't marked extern?

    pub fn JS_EnqueueJob(ctx: NonNull<JSContext>, job_func: JSJobFn, argc: c_int, argv: *mut JSValue) -> c_int;

    pub fn JS_IsJobPending(rt: NonNull<JSRuntime>) -> JSBool;
    pub fn JS_ExecutePendingJob(rt: NonNull<JSRuntime>, pctx: *mut *mut JSContext) -> c_int;

    pub fn JS_WriteObject(ctx: NonNull<JSContext>, psize: *mut usize, obj: JSValue, flags: c_uint) -> *mut u8;
    pub fn JS_WriteObject2(ctx: NonNull<JSContext>, psize: *mut usize, obj: JSValue, flags: c_uint, psab_tab: *mut *mut *mut u8, psab_tab_len: *mut usize) -> *mut u8;

    pub fn JS_ReadObject(ctx: NonNull<JSContext>, buf: *mut u8, buf_len: usize, flags: c_int) -> JSValue;
    // pub fn JS_ReadObject2(ctx: NonNull<JSContext>, buf: *mut u8, buf_len: usize, flags: c_int, psab_tab: *mut JSSABTab) -> JSValue;
    pub fn JS_EvalFunction(ctx: NonNull<JSContext>, fun_obj: JSValue) -> JSValue;
    pub fn JS_ResolveModule(ctx: NonNull<JSContext>, obj: JSValue) -> c_int;

    pub fn JS_GetScriptOrModuleName(ctx: NonNull<JSContext>, n_stack_levels: c_int) -> Option<JSAtom>;
    pub fn JS_LoadModule(ctx: NonNull<JSContext>, basename: *const c_char, filename: *const c_char) -> JSValue;

    pub fn JS_NewCFunction2(ctx: NonNull<JSContext>, func: JSCFunctionFn, name: *const c_char, length: c_int, cproto: JSCFunctionEnum, magic: c_int) -> JSValue;
    pub fn JS_NewCFunctionData(ctx: NonNull<JSContext>, func: JSCFunctionDataFn, length: c_int, magic: c_int, data_len: c_int, data: *mut JSValue) -> JSValue;

    pub fn JS_SetConstructor(ctx: NonNull<JSContext>, func_obj: JSValue, proto: JSValue);

    pub fn JS_SetPropertyFunctionList(ctx: NonNull<JSContext>, obj: JSValue, tab: *const JSCFunctionListEntry, len: c_int);

    pub fn JS_NewCModule(ctx: NonNull<JSContext>, name_str: *const c_char, func: JSModuleInitFn) -> Option<NonNull<JSModuleDef>>;
    pub fn JS_AddModuleExport(ctx: NonNull<JSContext>, m: NonNull<JSModuleDef>, name_str: *const c_char) -> c_int;
    pub fn JS_AddModuleExportList(ctx: NonNull<JSContext>, m: NonNull<JSModuleDef>, tab: *const JSCFunctionListEntry, len: c_int) -> c_int;
    pub fn JS_SetModuleExport(ctx: NonNull<JSContext>, m: NonNull<JSModuleDef>, export_name: *const c_char, val: JSValue) -> c_int;
    pub fn JS_SetModuleExportList(ctx: NonNull<JSContext>, m: NonNull<JSModuleDef>, tab: *const JSCFunctionListEntry, len: c_int) -> c_int;

    pub fn JS_GetVersion() -> *const c_char;
}
