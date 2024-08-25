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

use std::ffi::c_int;
use std::marker::PhantomData;

use raw::{JSTag, JSValue};

use crate::Context;

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

    pub const fn to_ref_value(&self) -> RefValue<'static> {
        RefValue {
            v: self.0,
            _phantom: PhantomData,
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
}

/// A wrapper over [`JSValue`] that can contain reference JS values like 'object' or 'string'. This
/// may also contain pure reference types too.
#[repr(transparent)]
pub struct RefValue<'a> {
    /// The wrapped JS value
    v: JSValue,

    /// Attach this value to the lifetime of the context it was acquired from.
    _phantom: PhantomData<&'a Context<'a>>,
}

impl<'a> RefValue<'a> {
    /// Converts a raw [`JSValue`] to a wrapped value type [`RefValue`].
    ///
    /// # Safety
    ///
    /// It is the caller's responsibility to ensure that the value:
    /// - Contains a live 'JSValue'.
    /// - That the ref-count has been incremented so this wrapper can own this handle.
    ///
    /// Each [`RefValue`] wrapper should have exactly one increment of the ref-count associated with
    /// it so that if all [`RefValue`] wrappers referring to a given object are freed then we will
    /// get a single ref-count decrement for each increment that created a [`RefValue`].
    ///
    /// That is to say: the semantics of a [`RefValue`] are very similar to [`std::rc::Rc`], with
    /// the added complication that we have to free the value with the context it was created from
    /// so we can't provide nice Clone+Drop semantics like [`std::rc::Rc`] does. There's no safe way
    /// to free the value without accepting additional overhead.
    pub const unsafe fn from_raw(v: JSValue) -> Self {
        Self {
            v,
            _phantom: PhantomData,
        }
    }

    /// Get the inner [`JSValue`] for raw access to the QuickJS API.
    pub const fn to_raw(&self) -> JSValue {
        self.v
    }

    /// Detatches the [`RefValue`] lifetime from the context, giving it a free `'static` lifetime.
    ///
    /// This is very unsafe as it can easily allow use-after-free if you hold on to a RefValue after
    /// the runtime it was created in is destroyed. There are some use-cases for this though so we
    /// provide this function as a convenience.
    ///
    /// # Safety
    ///
    /// It is the caller's responsibility to ensure the resulting [`RefValue`] does not outlive the
    /// runtime it was created from.
    pub const unsafe fn detatch(self) -> RefValue<'static> {
        std::mem::transmute::<Self, RefValue<'static>>(self)
    }

    /// Get a [`Value`] wrapper over this JS value, if this value contains a pure value type.
    pub const fn to_value(&self) -> Option<Value> {
        Value::from_raw(self.v)
    }

    /// Convert the [`RefValue`] into an [`Object`] if this value contains a JS 'object' value.
    pub const fn to_object(self) -> Result<Object<'a>, Self> {
        Object::from_value(self)
    }

    pub fn get_ref_count(&self) -> Option<c_int> {
        // Safety: This wrapper type is guaranteed to contain a live JS object
        unsafe { self.v.get_ref_count() }
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

impl<'a> Clone for RefValue<'a> {
    #[inline]
    fn clone(&self) -> Self {
        unsafe {
            let _ = self.v.increment_ref_count();
            Self {
                v: self.v,
                _phantom: PhantomData,
            }
        }
    }
}

impl<'a> Drop for RefValue<'a> {
    #[inline]
    fn drop(&mut self) {
        unsafe {
            self.v.decrement_ref_count();
        }
    }
}

/// A wrapper over [`JSValue`] that can contains a JS 'object'.
#[repr(transparent)]
pub struct Object<'a> {
    /// The wrapped JS value
    v: JSValue,

    /// Attach this value to the lifetime of the context it was acquired from.
    _phantom: PhantomData<&'a Context<'a>>,
}

impl<'a> Object<'a> {
    /// Convert the [`RefValue`] into an [`Object`] if this value contains a JS 'object' value.
    pub const fn from_value(v: RefValue<'a>) -> Result<Self, RefValue<'a>> {
        if v.is_object() {
            let val = v.v;

            // Prevent 'drop' from being called and decrementing the ref-count
            std::mem::forget(v);

            Ok(Self {
                v: val,
                _phantom: PhantomData,
            })
        } else {
            Err(v)
        }
    }

    pub const unsafe fn from_raw(v: JSValue) -> Option<Self> {
        if v.is_object() {
            Some(Self {
                v,
                _phantom: PhantomData,
            })
        } else {
            None
        }
    }

    pub const fn to_value(self) -> RefValue<'a> {
        let v = self.v;

        // Prevent 'drop' from being called and decrementing the ref-count
        std::mem::forget(self);

        RefValue {
            v,
            _phantom: PhantomData,
        }
    }

    /// Get the inner [`JSValue`] for raw access to the QuickJS API.
    pub const fn to_raw(&self) -> JSValue {
        self.v
    }

    pub fn get_ref_count(&self) -> c_int {
        // Safety: This wrapper type is guaranteed to contain a live ref-counted JS object
        unsafe { self.v.get_ref_count().unwrap_unchecked() }
    }
}

impl<'a> Into<RefValue<'a>> for Object<'a> {
    #[inline]
    fn into(self) -> RefValue<'a> {
        self.to_value()
    }
}

impl<'a> Clone for Object<'a> {
    #[inline]
    fn clone(&self) -> Self {
        unsafe {
            // We do an unwrap-unchecked here because we _know_ we have an object and we want to
            // encourage the optimizer to drop the branches inside 'increment_ref_count' as they're
            // dead code.
            let _ = self.v.increment_ref_count().unwrap_unchecked();
            Self {
                v: self.v,
                _phantom: PhantomData,
            }
        }
    }
}

impl<'a> Drop for Object<'a> {
    #[inline]
    fn drop(&mut self) {
        unsafe {
            // For the same reasons as the [Object::clone] impl we do an unwrap-unchecked here.
            let _ = self.v.decrement_ref_count().unwrap_unchecked();
        }
    }
}

/// Trait that abstracts over the various [`JSValue`] wrappers. This is generally intended to be used
/// as a generic parameter.
///
/// Very similar to [`GetRawValue`]. This trait will 'dup' the raw value instead of simply get it.
/// This is to say that the function on this trait will take an owned copy of the [`JSValue`] by
/// incrementing the reference count (if it has one)
pub trait DupRawValue {
    fn dup_raw_value(&self) -> JSValue;
}

impl DupRawValue for Value {
    fn dup_raw_value(&self) -> JSValue {
        self.0
    }
}

impl<'a> DupRawValue for RefValue<'a> {
    fn dup_raw_value(&self) -> JSValue {
        unsafe {
            self.v.increment_ref_count();
        }
        self.v
    }
}

impl<'a> DupRawValue for Object<'a> {
    fn dup_raw_value(&self) -> JSValue {
        unsafe {
            self.v.increment_ref_count().unwrap_unchecked();
        }
        self.v
    }
}

/// Trait that abstracts over the various [`JSValue`] wrappers. This is generally intended to be used
/// as a generic parameter.
pub trait GetRawValue {
    fn get_raw_value(&self) -> JSValue;
}

impl GetRawValue for Value {
    #[inline]
    fn get_raw_value(&self) -> JSValue {
        self.0
    }
}

impl<'a> GetRawValue for RefValue<'a> {
    #[inline]
    fn get_raw_value(&self) -> JSValue {
        self.v
    }
}

impl<'a> GetRawValue for Object<'a> {
    #[inline]
    fn get_raw_value(&self) -> JSValue {
        self.v
    }
}

/// Trait that abstracts over [`RefValue`] and [`RefValue`] accessories. Intended to be used as a
/// generic function parameter so you can pass a [`RefValue`], [`Object`] or other wrapper in the
/// same position.
///
/// This _should not_ be implemented for types that are known to be pure value types. While it's not
/// an error to do this it is still wrong to imply rc semantics for pure value types that _do not_
/// contain pointers.
pub trait ToRefValue<'a> {
    fn to_ref_value(self) -> RefValue<'a>;
}

impl<'a> ToRefValue<'a> for RefValue<'a> {
    #[inline]
    fn to_ref_value(self) -> RefValue<'a> {
        self
    }
}

impl<'a> ToRefValue<'a> for Object<'a> {
    #[inline]
    fn to_ref_value(self) -> RefValue<'a> {
        self.to_value()
    }
}
