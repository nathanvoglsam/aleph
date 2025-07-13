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

use core::str;
use std::ffi::c_int;
use std::ptr::NonNull;

use raw::{JSTag, JSValue};

use crate::{Atom, Context, CtxString, OwnPropertyNames};

/// Enumeration of the internal number representations of QuickJS.
///
/// # Background
///
/// QuickJS stores 'number' values as either an integer (i32) or double (f64) as distinct JSTag
/// types as an optimization. Only one of the two values is active at once. We provide this wrapper
/// to expose this to users. Retaining pure integer semantics can be useful.
///
/// If your specific case doesn't care then call [`NumberVariant::normalize`] to cast away the info
/// and get a plain f64.
#[derive(Clone, Copy, PartialEq, PartialOrd, Debug)]
pub enum NumberVariant {
    Double(f64),
    Integer(c_int),
}

impl NumberVariant {
    /// Collapse both [`NumberVariant::Integer`] and [`NumberVariant::Double`] paths to f64.
    ///
    /// An i32 is guaranteed to fit in an f64 so this won't destroy information. If you don't care
    /// about integer semantics then you can use this function to simplify handling number values.
    pub const fn normalize(self) -> f64 {
        match self {
            NumberVariant::Double(v) => v,
            NumberVariant::Integer(v) => v as f64,
        }
    }

    pub const fn get_double(self) -> Option<f64> {
        match self {
            NumberVariant::Double(v) => Some(v),
            NumberVariant::Integer(_) => None,
        }
    }

    pub const fn get_int(self) -> Option<c_int> {
        match self {
            NumberVariant::Double(_) => None,
            NumberVariant::Integer(v) => Some(v),
        }
    }
}

/// Represents a wrapper of [`JSValue`] that represents only the pure value type variants of
/// a JS value.
///
/// That means that this will never contain an 'object', 'string' or any other kind of pointer based
/// [`raw::JSValue`]. This means the object is safe to pass as a pure value type with no kind of
/// ref-count semantics.
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct Value(JSValue);

impl Value {
    /// Represents a null reference JS value. In some sense this is a 'pointer' value, but it never
    /// actually points to anything (it's a null reference) so we can treat it like a pure value
    /// type.
    pub const NULL: Self = Self(JSValue::NULL);

    /// Represents an 'undefined' JS value.
    pub const UNDEFINED: Self = Self(JSValue::UNDEFINED);

    /// The false 'boolean' JS value.
    pub const FALSE: Self = Self(JSValue::FALSE);

    /// The true 'boolean' JS value.
    pub const TRUE: Self = Self(JSValue::TRUE);

    /// A special value that signals an exception has been thrown. This may be returned by QuickJS
    /// functions that yield JS values to signify an error has occured (i.e, an exception) and that
    /// we don't have a valid result.
    ///
    /// This does not contain the actual exception value. The exception object must be fetched
    /// from the context with [`crate::Context::get_exception`]. Because this contains nothing but
    /// a tag signalling an exception being thrown, this is a pure value type too.
    pub const EXCEPTION: Self = Self(JSValue::EXCEPTION);

    /// A special QuickJS value for 'uninitialized'. I don't actually know what this is for.
    pub const UNINITIALIZED: Self = Self(JSValue::UNINITIALIZED);

    /// A JS 'number' value that contains a NaN.
    ///
    /// This is guaranteed to be a JS 'number' containing some NaN bit pattern. This is needed as
    /// NaN-boxing means that not all float NaN bit patterns will be treated as 'number' by the
    /// JS runtime.
    pub const NAN: Self = Self(JSValue::NAN);

    /// Converts a raw [`JSValue`] to a wrapped value type [`Value`]. Will return [`None`] for any
    /// JS value that isn't:
    /// - INT
    /// - BOOL
    /// - NULL
    /// - UNDEFINED
    /// - UNINITIALIZED
    /// - EXCEPTION
    /// - FLOAT64
    ///
    /// # Info
    ///
    /// Strictly speaking 'CATCH_OFFSET' is also a value type, but seems to be a value type used
    /// internally by the runtime that we shouldn't use. We make the choice to not map that to our
    /// [`Value`] wrapper.
    ///
    /// # Safety
    ///
    /// This is safe, while the [`RefValue::from_raw`] function is not. Why?
    ///
    /// [`RefValue`] may contain pointer based JS values, like 'object' or 'string'. These are heap
    /// allocated and reference counted. It would be possible to construct and wrap a JSValue that
    /// holds an invalid pointer. There's no way to check at runtime and so the call must be unsafe.
    ///
    /// The [`Value::from_raw`] function on the other hand will only yield a value for pure value
    /// JS types like number, checked at runtime These do not contain pointers and so as long as you
    /// check the tag they are valid to use. If you want an unchecked conversion just use
    /// [`Option::unwrap_unchecked`].
    pub const fn from_raw(v: JSValue) -> Option<Self> {
        match v.get_norm_tag() {
            JSTag::BIG_INT
            | JSTag::SYMBOL
            | JSTag::STRING
            | JSTag::MODULE
            | JSTag::FUNCTION_BYTECODE
            | JSTag::OBJECT
            | JSTag::CATCH_OFFSET => None,
            JSTag::INT
            | JSTag::BOOL
            | JSTag::NULL
            | JSTag::UNDEFINED
            | JSTag::UNINITIALIZED
            | JSTag::EXCEPTION
            | JSTag::FLOAT64 => Some(Self(v)),
            _ => unreachable!(),
        }
    }

    /// Get the inner [`JSValue`] for raw access to the QuickJS API.
    pub const fn to_raw(&self) -> JSValue {
        self.0
    }

    /// Creates a new JS 'boolean' from the given bool.
    pub const fn new_bool(val: bool) -> Self {
        Self(JSValue::new_bool(val))
    }

    /// Creates a new JS 'number' from the given f64.
    pub const fn new_f64(d: f64) -> Self {
        Self(JSValue::new_f64(d))
    }

    /// Creates a new JS 'number' from the given i32.
    ///
    /// # Info
    ///
    /// Strictly speaking JS only has a single 'number' type, but internally QuickJS has two. Int
    /// and Double. These are distinct 'JSTag' cases. The only difference is storage. Int stores an
    /// integer value as an i32 rather than a f64.
    ///
    /// This function creates an Int number.
    pub const fn new_i32(val: i32) -> Self {
        Self(JSValue::new_i32(val))
    }

    /// Creates a new JS 'number' from the given i64.
    ///
    /// This will attempt to pack an i64 into a JS 'number' value using the most compact
    /// representation possible. Values that fit into an i32 will be stored in an INT JSValue while
    /// any other value will be stored as a double.
    ///
    /// This, of course, means that in reality we can only store integer values within the
    /// safe integer range of a double (`2^53 - 1`) rather than the full range if an i64. It is the
    /// callers responsibility to check this range if it's important the exact value is retained.
    pub const fn new_i64(val: i64) -> Self {
        let min = i32::MIN as i64;
        let max = i32::MAX as i64;
        if val < min || val > max {
            Self::new_f64(val as f64)
        } else {
            Self::new_i32(val as i32)
        }
    }

    /// Creates a new JS 'number' from the given u32.
    ///
    /// # Info
    ///
    /// This function is similar to [`Self::new_i64`] but takes a u32. A u32 is guaranteed to fit
    /// inside the safe integer range of a double so this won't have the same pitfalls as the i64
    /// conversion.
    pub const fn new_u32(val: u32) -> Self {
        let vi64 = val as i64;
        let min = i32::MIN as i64;
        let max = i32::MAX as i64;
        if vi64 < min || vi64 > max {
            Self::new_f64(val as f64)
        } else {
            Self::new_i32(val as i32)
        }
    }

    /// Returns the [`JSTag`] that identifies the type of this value.
    ///
    /// # Info
    ///
    /// This uses what QuickJS calls the 'normalized' tag. A NaN-boxed value stores the tag as
    /// different NaN bit patterns. If we have a NaN value that stores something that isn't a JSTag
    /// then it's just treted as 'number'. The problem is if we try and match on our enumerated tag
    /// values then we may get values that mean 'number' that we can't match on.
    ///
    /// QuickJS provides the 'normalized tag' function that normalizes NaN-boxed tags into the
    /// enumerated JSTag values.
    pub const fn get_tag(&self) -> JSTag {
        // Only get the normalized tag as without normalized tags we may get JSTag values outside
        // the enumerated range. Principle of least surprise, give out the normalized tag here.
        self.0.get_norm_tag()
    }

    /// Returns the number value for this value, if it is a JS 'number' value.
    pub const fn get_number(&self) -> Option<NumberVariant> {
        let v = match self.0.get_norm_tag() {
            JSTag::INT => NumberVariant::Integer(self.0.get_int()),
            JSTag::FLOAT64 => NumberVariant::Double(self.0.get_float64()),
            _ => return None,
        };
        Some(v)
    }

    /// Returns the bool value for this value, if it is a JS 'boolean' value.
    pub const fn get_bool(&self) -> Option<bool> {
        if self.is_bool() {
            Some(self.0.get_bool())
        } else {
            None
        }
    }

    pub const fn is_number(&self) -> bool {
        self.0.is_number()
    }

    pub const fn is_bool(&self) -> bool {
        self.0.is_bool()
    }

    pub const fn is_null(&self) -> bool {
        self.0.is_null()
    }

    pub const fn is_undefined(&self) -> bool {
        self.0.is_undefined()
    }

    pub const fn is_exception(&self) -> bool {
        self.0.is_exception()
    }

    pub const fn is_uninitialized(&self) -> bool {
        self.0.is_uninitialized()
    }

    #[inline]
    pub fn to_string(&self, ctx: &Context) -> RefValue {
        unsafe {
            let string = raw::JS_ToString(ctx.0.ctx, self.0);
            RefValue::from_raw(ctx, string)
        }
    }

    #[inline]
    pub fn to_c_str(&self, ctx: &Context) -> Option<CtxString> {
        unsafe {
            let mut len = 0;
            let cstr = raw::JS_ToCStringLen2(ctx.0.ctx, &mut len, self.0, raw::JSBool::FALSE);

            if len == 0 || cstr.is_null() {
                None
            } else {
                let bytes = std::slice::from_raw_parts(cstr as *const u8, len);
                let string = str::from_utf8(bytes).unwrap_unchecked();
                Some(CtxString::from_ctx_and_str(ctx.clone(), string))
            }
        }
    }
}

/// A wrapper over [`JSValue`] that can contain reference JS values like 'object' or 'string'. This
/// may also contain pure reference types too.
pub struct RefValue {
    /// The wrapped JS value
    v: JSValue,

    /// Attach this value to the context it was acquired from.
    c: Context,
}

impl RefValue {
    /// Destroys the [`RefValue`], without decrementing the JSValue's ref-count but while still
    /// decrementing the internal [`Context`] ref count.
    #[inline]
    pub fn detatch(self) -> JSValue {
        let v = self.v;
        unsafe { drop(std::ptr::read(&self.c)) };
        std::mem::forget(self);
        v
    }

    /// An explicit destructor for JS value wrappers. This is similar to the [`Drop`]
    /// implementations on our wrapper functions but will _immediately_ free the value if the ref
    /// count reaches zero. This will not trigger a GC cycle, rather the individual value will be
    /// explicitly freed.
    ///
    /// This isn't _required_ to be called. The [`Drop`] implementation will still decrement the
    /// reference count and the object will eventually be freed if you manually run a GC cycle or
    /// wait for the runtime to do it internally. This simply performs the free immediately if that
    /// is important to your use case.
    #[inline]
    pub fn free_value(self) {
        unsafe {
            // Performs the actual ref-count decrement and free call
            self.v.free_value(self.c.0.ctx);
        }
        // Avoid decrementing the refcount a second time by calling detach
        self.detatch();
    }

    /// Get the inner [`JSValue`] for raw access to the QuickJS API.
    pub const fn to_raw(&self) -> JSValue {
        self.v
    }

    /// Get a [`Value`] wrapper over this JS value, if this value contains a pure value type.
    pub const fn to_value(&self) -> Option<Value> {
        Value::from_raw(self.v)
    }

    /// Convert the [`RefValue`] into an [`Object`] if this value contains a JS 'object' value.
    pub const fn to_object(self) -> Result<Object, Self> {
        Object::from_value(self)
    }

    /// Returns whether 'self' is an array
    pub fn is_array(&self) -> bool {
        // Safety: This wrapper type is guaranteed to contain a live JS object
        unsafe { raw::JS_IsArray(self.c.0.ctx, self.v) != 0 }
    }

    #[inline(always)]
    pub fn get_ref_count(&self) -> Option<c_int> {
        // Safety: This wrapper type is guaranteed to contain a live JS object
        unsafe { self.v.get_ref_count() }
    }

    #[inline]
    pub fn to_string(&self) -> RefValue {
        unsafe {
            let string = raw::JS_ToString(self.c.0.ctx, self.v);
            RefValue::from_raw(&self.c, string)
        }
    }

    #[inline]
    pub fn to_c_str(&self) -> Option<CtxString> {
        unsafe {
            let mut len = 0;
            let cstr = raw::JS_ToCStringLen2(self.c.0.ctx, &mut len, self.v, raw::JSBool::FALSE);

            if len == 0 || cstr.is_null() {
                None
            } else {
                let bytes = std::slice::from_raw_parts(cstr as *const u8, len);
                let string = str::from_utf8(bytes).unwrap_unchecked();
                Some(CtxString::from_ctx_and_str(self.c.clone(), string))
            }
        }
    }

    #[inline]
    pub fn get_property_str(&self, prop: &str) -> RefValue {
        match self.c.new_atom(prop) {
            Some(a) => {
                let v = self.get_property(&a);
                v
            }
            _ => {
                panic!()
            }
        }
    }

    #[inline]
    pub fn get_property(&self, prop: &Atom) -> RefValue {
        unsafe {
            assert!(self.c.same_rt(&prop.c));
            let v = raw::JS_GetProperty(self.c.0.ctx, self.v, prop.v);
            RefValue::from_raw(&self.c, v)
        }
    }

    #[inline]
    pub fn set_property_str(&self, prop: &str, v: &impl DupRawValue) -> c_int {
        match self.c.new_atom(prop) {
            Some(a) => self.set_property(&a, v),
            _ => {
                panic!()
            }
        }
    }

    #[inline]
    pub fn set_property(&self, prop: &Atom, v: &impl DupRawValue) -> c_int {
        unsafe {
            assert!(self.c.same_rt(&prop.c));
            if let Some(c) = v.ctx() {
                assert!(self.c.same_rt(c));
            }

            let v = v.dup_raw_value();
            raw::JS_SetProperty(self.c.0.ctx, self.v, prop.v, v)
        }
    }

    #[inline]
    pub fn delete_property_str(&self, prop: &str) -> c_int {
        match self.c.new_atom(prop) {
            Some(a) => self.delete_property(&a),
            _ => {
                panic!()
            }
        }
    }

    #[inline]
    pub fn delete_property(&self, prop: &Atom) -> c_int {
        unsafe {
            assert!(self.c.same_rt(&prop.c));
            raw::JS_DeleteProperty(self.c.0.ctx, self.v, prop.v, 0) // TODO: flags
        }
    }

    #[inline]
    pub fn call(&self, this: &impl GetRawValue, args: &[&RefValue]) -> RefValue {
        unsafe {
            if let Some(c) = this.ctx() {
                assert!(self.c.same_rt(c));
            }
            for arg in args {
                assert!(self.c.same_rt(&arg.c));
            }

            let args = Vec::from_iter(args.iter().map(|v| v.v));

            let argc: c_int = args.len().try_into().unwrap();
            let argv = if !args.is_empty() {
                args.as_ptr() as *mut RefValue as *mut raw::JSValue
            } else {
                std::ptr::null_mut()
            };

            let v = raw::JS_Call(self.c.0.ctx, self.v, this.get_raw_value(), argc, argv);
            RefValue::from_raw(&self.c, v)
        }
    }

    #[inline]
    pub fn get_own_property_names(&self, opts: raw::JSGetPropertyNameOption) -> OwnPropertyNames {
        unsafe {
            let mut tab = std::ptr::null_mut();
            let mut len = 0;
            let result =
                raw::JS_GetOwnPropertyNames(self.c.0.ctx, &mut tab, &mut len, self.v, opts);
            if result < 0 {
                panic!("Don't know how to handle exceptions yet");
            }

            let slice = if len > 0 {
                let tab = NonNull::new(tab).unwrap();
                NonNull::slice_from_raw_parts(tab, len as usize)
            } else {
                NonNull::slice_from_raw_parts(NonNull::dangling(), 0)
            };

            OwnPropertyNames {
                ctx: self.c.clone(),
                props: slice,
            }
        }
    }

    pub fn to_json(&self) -> Option<serde_json::Value> {
        let v = match self.get_tag() {
            raw::JSTag::BIG_INT => unimplemented!(),
            raw::JSTag::SYMBOL => unimplemented!(),
            raw::JSTag::STRING => {
                let string = self.to_c_str()?;
                let string = string.to_owned();
                serde_json::Value::String(string)
            }
            raw::JSTag::MODULE => unimplemented!(),
            raw::JSTag::FUNCTION_BYTECODE => unimplemented!(),
            raw::JSTag::OBJECT => unsafe {
                let v = self.clone().to_object().ok().unwrap_unchecked();
                v.to_json()?
            },
            raw::JSTag::BOOL => unsafe {
                let boolean = self.get_bool().unwrap_unchecked();
                serde_json::Value::Bool(boolean)
            },
            raw::JSTag::NULL => serde_json::Value::Null,
            raw::JSTag::UNDEFINED => return None,
            raw::JSTag::UNINITIALIZED => unimplemented!(),
            raw::JSTag::CATCH_OFFSET => unimplemented!(),
            raw::JSTag::EXCEPTION => return None,
            raw::JSTag::INT => unsafe {
                let number = self
                    .get_number()
                    .unwrap_unchecked()
                    .get_int()
                    .unwrap_unchecked();
                let number = serde_json::to_value(number).unwrap();
                assert!(number.is_i64() || number.is_u64());
                number
            },
            raw::JSTag::FLOAT64 => unsafe {
                let number = self
                    .get_number()
                    .unwrap_unchecked()
                    .get_double()
                    .unwrap_unchecked();
                let number = serde_json::to_value(number).unwrap();
                number
            },
            _ => unimplemented!(),
        };
        Some(v)
    }

    /// Returns the [`JSTag`] that identifies the type of this value.
    ///
    /// # Info
    ///
    /// This uses what QuickJS calls the 'normalized' tag. A NaN-boxed value stores the tag as
    /// different NaN bit patterns. If we have a NaN value that stores something that isn't a JSTag
    /// then it's just treted as 'number'. The problem is if we try and match on our enumerated tag
    /// values then we may get values that mean 'number' that we can't match on.
    ///
    /// QuickJS provides the 'normalized tag' function that normalizes NaN-boxed tags into the
    /// enumerated JSTag values.
    pub const fn get_tag(&self) -> JSTag {
        // Only get the normalized tag as without normalized tags we may get JSTag values outside
        // the enumerated range. Principle of least surprise, give out the normalized tag here.
        self.v.get_norm_tag()
    }

    /// Returns the number value for this value, if it is a JS 'number' value.
    pub const fn get_number(&self) -> Option<NumberVariant> {
        let v = match self.v.get_norm_tag() {
            JSTag::INT => NumberVariant::Integer(self.v.get_int()),
            JSTag::FLOAT64 => NumberVariant::Double(self.v.get_float64()),
            _ => return None,
        };
        Some(v)
    }

    /// Returns the bool value for this value, if it is a JS 'boolean' value.
    pub const fn get_bool(&self) -> Option<bool> {
        if self.is_bool() {
            Some(self.v.get_bool())
        } else {
            None
        }
    }

    pub const fn is_number(&self) -> bool {
        self.v.is_number()
    }

    pub const fn is_bool(&self) -> bool {
        self.v.is_bool()
    }

    pub const fn is_null(&self) -> bool {
        self.v.is_null()
    }

    pub const fn is_undefined(&self) -> bool {
        self.v.is_undefined()
    }

    pub const fn is_exception(&self) -> bool {
        self.v.is_exception()
    }

    pub const fn is_uninitialized(&self) -> bool {
        self.v.is_uninitialized()
    }

    pub const fn is_big_int(&self) -> bool {
        self.v.is_big_int()
    }

    pub const fn is_string(&self) -> bool {
        self.v.is_string()
    }

    pub const fn is_symbol(&self) -> bool {
        self.v.is_symbol()
    }

    pub const fn is_object(&self) -> bool {
        self.v.is_object()
    }
}

impl RefValue {
    pub(crate) unsafe fn from_raw(c: &Context, v: JSValue) -> Self {
        Self { v, c: c.clone() }
    }
}

impl<'a> Clone for RefValue {
    #[inline]
    fn clone(&self) -> Self {
        unsafe {
            let _ = self.v.increment_ref_count();
            Self {
                v: self.v,
                c: self.c.clone(),
            }
        }
    }
}

impl<'a> Drop for RefValue {
    #[inline]
    fn drop(&mut self) {
        unsafe {
            self.v.decrement_ref_count();
        }
    }
}

/// A wrapper over [`JSValue`] that can contain a JS 'object'.
#[derive(Clone)]
pub struct Object(pub(crate) RefValue);

impl Object {
    /// Destroys the [`Object`], without decrementing the JSValue's ref-count but while still
    /// decrementing the internal [`Context`] ref count.
    #[inline]
    pub fn detatch(self) -> JSValue {
        self.0.detatch()
    }

    /// Convert the [`RefValue`] into an [`Object`] if this value contains a JS 'object' value.
    pub const fn from_value(v: RefValue) -> Result<Self, RefValue> {
        if v.is_object() { Ok(Self(v)) } else { Err(v) }
    }

    /// Get the inner [`RefValue`].
    #[inline(always)]
    pub fn to_value(self) -> RefValue {
        self.0
    }

    /// Get the inner [`JSValue`] for raw access to the QuickJS API.
    pub const fn to_raw(&self) -> JSValue {
        self.0.to_raw()
    }

    /// Returns whether 'self' is an array
    #[inline]
    pub fn is_array(&self) -> bool {
        // Safety: This wrapper type is guaranteed to contain a live JS object
        unsafe { raw::JS_IsArray(self.0.c.0.ctx, self.0.v) != 0 }
    }

    #[inline]
    pub fn get_ref_count(&self) -> c_int {
        // Safety: This wrapper type is guaranteed to contain a live ref-counted JS object
        unsafe { self.0.get_ref_count().unwrap_unchecked() }
    }

    #[inline]
    pub fn to_string(&self) -> RefValue {
        self.0.to_string()
    }

    #[inline]
    pub fn to_c_str(&self) -> Option<CtxString> {
        self.0.to_c_str()
    }

    #[inline]
    pub fn get_property_str(&self, prop: &str) -> RefValue {
        self.0.get_property_str(prop)
    }

    #[inline]
    pub fn get_property(&self, prop: &Atom) -> RefValue {
        self.0.get_property(prop)
    }

    #[inline]
    pub fn set_property_str(&self, prop: &str, v: &impl DupRawValue) -> c_int {
        self.0.set_property_str(prop, v)
    }

    #[inline]
    pub fn set_property(&self, prop: &Atom, v: &impl DupRawValue) -> c_int {
        self.0.set_property(prop, v)
    }

    #[inline]
    pub fn delete_property_str(&self, prop: &str) -> c_int {
        self.0.delete_property_str(prop)
    }

    #[inline]
    pub fn delete_property(&self, prop: &Atom) -> c_int {
        self.0.delete_property(prop)
    }

    #[inline]
    pub fn call(&self, this: &impl GetRawValue, args: &[&RefValue]) -> RefValue {
        self.0.call(this, args)
    }

    #[inline]
    pub fn get_own_property_names(&self, opts: raw::JSGetPropertyNameOption) -> OwnPropertyNames {
        self.0.get_own_property_names(opts)
    }

    #[inline]
    pub fn to_json(&self) -> Option<serde_json::Value> {
        if self.is_array() {
            let opts =
                raw::JSGetPropertyNameOption::STRING_MASK | raw::JSGetPropertyNameOption::ENUM_ONLY;
            let props = self.get_own_property_names(opts);

            let mut array = Vec::new();
            for prop in props.iter() {
                let atom = prop.atom.as_ref()?;
                let value = self.get_property(atom);
                if let Some(value) = value.to_json() {
                    // let name = atom.to_c_str()?;
                    array.push(value);
                }
            }

            Some(serde_json::Value::Array(array))
        } else {
            let opts =
                raw::JSGetPropertyNameOption::STRING_MASK | raw::JSGetPropertyNameOption::ENUM_ONLY;
            let props = self.get_own_property_names(opts);

            let mut object = serde_json::Map::new();
            for prop in props.iter() {
                let atom = prop.atom.as_ref()?;
                let value = self.get_property(atom);
                if let Some(value) = value.to_json() {
                    let name = atom.to_c_str()?;
                    object.insert(name.to_string(), value);
                }
            }

            Some(serde_json::Value::Object(object))
        }
    }
}

impl Object {
    pub(crate) unsafe fn from_raw(c: &Context, v: JSValue) -> Option<Self> {
        unsafe {
            if v.is_object() {
                Some(Self(RefValue::from_raw(c, v)))
            } else {
                None
            }
        }
    }
}

impl Into<RefValue> for Object {
    #[inline]
    fn into(self) -> RefValue {
        self.to_value()
    }
}

/// Trait that abstracts over the various [`JSValue`] wrappers. This is generally intended to be used
/// as a generic parameter.
///
/// Very similar to [`GetRawValue`]. This trait will 'dup' the raw value instead of simply get it.
/// This is to say that the function on this trait will take an owned copy of the [`JSValue`] by
/// incrementing the reference count (if it has one)
pub trait DupRawValue {
    fn ctx(&self) -> Option<&Context>;
    fn dup_raw_value(&self) -> JSValue;
}

impl DupRawValue for Value {
    #[inline]
    fn ctx(&self) -> Option<&Context> {
        None
    }

    #[inline]
    fn dup_raw_value(&self) -> JSValue {
        self.0
    }
}

impl DupRawValue for RefValue {
    #[inline]
    fn ctx(&self) -> Option<&Context> {
        Some(&self.c)
    }

    #[inline]
    fn dup_raw_value(&self) -> JSValue {
        unsafe {
            self.v.increment_ref_count();
        }
        self.v
    }
}

impl DupRawValue for Object {
    #[inline]
    fn ctx(&self) -> Option<&Context> {
        DupRawValue::ctx(&self.0)
    }

    #[inline]
    fn dup_raw_value(&self) -> JSValue {
        DupRawValue::dup_raw_value(&self.0)
    }
}

/// Trait that abstracts over the various [`JSValue`] wrappers. This is generally intended to be used
/// as a generic parameter.
pub trait GetRawValue {
    fn ctx(&self) -> Option<&Context>;
    fn get_raw_value(&self) -> JSValue;
}

impl GetRawValue for Value {
    #[inline]
    fn ctx(&self) -> Option<&Context> {
        None
    }

    #[inline]
    fn get_raw_value(&self) -> JSValue {
        self.0
    }
}

impl GetRawValue for RefValue {
    #[inline]
    fn ctx(&self) -> Option<&Context> {
        Some(&self.c)
    }

    #[inline]
    fn get_raw_value(&self) -> JSValue {
        self.v
    }
}

impl GetRawValue for Object {
    #[inline]
    fn ctx(&self) -> Option<&Context> {
        GetRawValue::ctx(&self.0)
    }

    #[inline]
    fn get_raw_value(&self) -> JSValue {
        GetRawValue::get_raw_value(&self.0)
    }
}

/// Trait that abstracts over [`RefValue`] and [`RefValue`] accessories. Intended to be used as a
/// generic function parameter so you can pass a [`RefValue`], [`Object`] or other wrapper in the
/// same position.
///
/// This _should not_ be implemented for types that are known to be pure value types. While it's not
/// an error to do this it is still wrong to imply rc semantics for pure value types that _do not_
/// contain pointers.
pub trait ToRefValue {
    fn to_ref_value(self) -> RefValue;
}

impl ToRefValue for RefValue {
    #[inline]
    fn to_ref_value(self) -> RefValue {
        self
    }
}

impl ToRefValue for Object {
    #[inline]
    fn to_ref_value(self) -> RefValue {
        self.to_value()
    }
}
