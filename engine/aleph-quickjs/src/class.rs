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

use std::cell::OnceCell;
use std::ffi::c_void;
use std::mem::needs_drop;
use std::ptr::NonNull;
use std::rc::Rc;
use std::thread::LocalKey;

use aleph_alloc::BBox;
use aleph_alloc::allocator_global_handle::AllocatorGlobalHandle;
use aleph_nstr::NStr;

use crate::runtime::with_runtime;

/// Alias for [`raw::JSClassID`]. It's just an integer so there's no real purpose to doing an entire
/// new-type wrapper.
pub type ClassID = raw::JSClassID;

/// Trait implemented using [`crate::new_class!`] macro. Exports an interface on the declared type
/// to provide a rust identity to a class type inside a JS runtime with a safe interface.
///
/// This uses a combination of macros, thread_local and generics to lazy init a per-thread
/// [`ClassID`] and class definition for the runtime on the calling thread. Each thread gets its
/// own runtime, so setup must be done for each.
///
/// You shouldn't ever need to interact with this trait directly.
pub unsafe trait Class {
    /// The type of opaque data that will be attached to objects of this class.
    type Opaque: ClassOpaque;

    /// Get the [`ClassID`] for the calling thread of this class. The ID is only valid on the
    /// calling thread as it is derived from the thread-local runtime.
    ///
    /// This will lazy-init the ID if it has never been asked for on the calling thread before.
    fn get_thread_class_id() -> ClassID;
}

/// Base trait that must be implemented by class opaque containers. Provides the minimum facilities
/// to correctly handle dropping the container.
///
/// There are two sub-traits that are needed to make a useful [`ClaseOpaque`] type.
/// - [`ClassOpaqueContainer`]
/// - [`ClassOpaqueHandle`]
///
/// # 'Container'
///
/// Should be implemented on allocating containers like [`Box`] or [`Rc`]. The impl should provide
/// an associated type for the contained T, and functions are provided to enable conversion to/from
/// the opaque pointer stored on a JS object.
///
/// # 'Handle'
///
/// Should be implemented on handle types which are not pointers, but are instead smuggling a usize
/// sized handle in place of the opaque pointer. This can be implemented by types like
/// [`std::num::NonZeroUsize`], any type which can map itself into a [`NonNull`] address.
///
/// # Anyway...
///
/// This trait only provides the drop glue, which is common to both.
pub unsafe trait ClassOpaque: Sized + 'static {
    /// Takes an opaque pointer and correctly handles calling the drop implementation for the type
    /// erased container.
    ///
    /// For handle-like opaque types this should be a no-op
    unsafe fn drop_opaque(ptr: NonNull<c_void>);
}

/// Interface expected of types that can be used with [`raw::JS_SetOpaque`] and
/// [`raw::JS_GetOpaque`] to attach data to an object of some class.
///
/// See [`ClassOpaque`] for more info.
pub unsafe trait ClassOpaqueContainer: ClassOpaque {
    /// Type that 'Self' is a container of. Used by [`ClassOpaqueContainer::inner`] to 'deref' to
    /// the inner type. This doesn't necessarily have to be a different type as 'Self'.
    type Inner: Sized + 'static;

    /// Convert 'Self' into a pointer to be attached to a JS object.
    fn into_raw(self) -> NonNull<c_void>;

    /// Reconstitute the original 'Self' from the pointer fetched off of some JS object.
    unsafe fn from_raw(ptr: NonNull<c_void>) -> Self;

    /// Takes a pointer made by [`ClassOpaqueContainer::into_raw`] and returns whatever 'Self' has
    /// declared as the fetch type. This will typically be a shared reference to some inner type.
    unsafe fn ptr_to_inner(ptr: NonNull<c_void>) -> NonNull<Self::Inner>;
}

/// Interface expected of types that can be used with [`raw::JS_SetOpaque`] and
/// [`raw::JS_GetOpaque`] to attach data to an object of some class.
///
/// See [`ClassOpaque`] for more info.
pub unsafe trait ClassOpaqueHandle: ClassOpaque + Copy + Clone {
    /// Convert 'Self' into a pointer to be attached to a JS object.
    fn into_raw(self) -> NonNull<c_void>;

    /// Reconstitute the original 'Self' from the pointer fetched off of some JS object.
    fn from_raw(ptr: NonNull<c_void>) -> Self;
}

/// Declares a new native class type with the given '$name' that will use '$ty' as the opaque type
/// stored onto the object.
///
/// $ty must be a type that implements [`ClassOpaque`] like [`Box`] or [`Rc`].
///
/// $name must be an identifier for a new struct type.
///
/// This macro will declare a new struct `pub struct $name;` that provides a rust identity to the
/// class type being declared. The macro and trait intenerals handle registering the class
/// automatically with the JS runtime on the calling thread. The struct will implement the [`Class`]
/// trait to pair the $name identifier with all the associated and types and functions to make it
/// a 'class'.
///
/// $name will also be stringified to become the registered name of the class within the runtime.
/// The name must contain only ASCII characters, which is checked at compile time.
#[macro_export]
macro_rules! new_class {
    ($name: ident, $ty: path) => {
        pub struct $name;

        unsafe impl $crate::Class for $name {
            type Opaque = $ty;

            fn get_thread_class_id() -> $crate::ClassID {
                ::std::thread_local! {
                    static CLASS_ID: ::std::cell::OnceCell<$crate::ClassID> = const {
                        ::std::cell::OnceCell::new()
                    };
                    static CLASS_DEF: $crate::raw::JSClassDef = const {
                        $crate::class_def::<$name>($crate::nstr::nstr!(stringify!($name)))
                    };
                }
                unsafe { $crate::get_or_init_class_id_for(&CLASS_ID, &CLASS_DEF) }
            }
        }
    };
}

/// Internal support code used by [`crate::new_class!`] macro. Do not use.
#[doc(hidden)]
pub const fn class_def<T: Class>(name: &'static NStr) -> raw::JSClassDef {
    assert!(name.to_str().is_ascii());

    extern "C" fn gc_finalize<TT: Class>(_rt: NonNull<raw::JSRuntime>, val: raw::JSValueConst) {
        unsafe {
            let id = <TT as Class>::get_thread_class_id();
            let v = raw::JS_GetOpaque(val, id);
            if let Some(v) = v {
                <<TT as Class>::Opaque as ClassOpaque>::drop_opaque(v);
            }
        }
    }

    // We can skip providing a finalizer if the opaque type doesn't have a drop impl
    let needs_finalizer = needs_drop::<<T as Class>::Opaque>();

    raw::JSClassDef {
        class_name: name.to_cstr_ptr(),
        finalizer: if needs_finalizer {
            Some(gc_finalize::<T>)
        } else {
            None
        },
        gc_mark: None,
        call: None,
        exotic: std::ptr::null_mut(),
    }
}

/// Internal support code used by [`crate::new_class!`] macro. Do not use.
#[doc(hidden)]
pub unsafe fn get_or_init_class_id_for(
    class_id: &'static LocalKey<OnceCell<ClassID>>,
    class_def: &'static LocalKey<raw::JSClassDef>,
) -> ClassID {
    class_id.with(|class_id| {
        let id = class_id.get_or_init(|| {
            with_runtime(|r| unsafe {
                let mut dummy = None;
                let id = raw::JS_NewClassID(r.0, &mut dummy).unwrap();

                class_def.with(|class| {
                    let result = raw::JS_NewClass(r.0, id, class);
                    assert!(result >= 0);
                });

                id
            })
        });
        *id
    })
}

// ============================================================================================== //

unsafe impl<T: Sized + 'static> ClassOpaque for Box<T> {
    unsafe fn drop_opaque(ptr: NonNull<c_void>) {
        unsafe { drop(Box::<T>::from_raw(ptr.cast().as_ptr())) }
    }
}

unsafe impl<T: Sized + 'static> ClassOpaqueContainer for Box<T> {
    type Inner = T;

    fn into_raw(self) -> NonNull<c_void> {
        unsafe { NonNull::new(Box::into_raw(self)).unwrap_unchecked().cast() }
    }

    unsafe fn from_raw(ptr: NonNull<c_void>) -> Self {
        unsafe { Box::<T>::from_raw(ptr.cast().as_ptr()) }
    }

    unsafe fn ptr_to_inner(ptr: NonNull<c_void>) -> NonNull<Self::Inner> {
        ptr.cast()
    }
}

// ============================================================================================== //

unsafe impl<T: Sized + 'static, A: AllocatorGlobalHandle + 'static> ClassOpaque for BBox<T, A> {
    unsafe fn drop_opaque(ptr: NonNull<c_void>) {
        unsafe { drop(BBox::<T>::from_raw(ptr.cast().as_ptr())) }
    }
}

unsafe impl<T: Sized + 'static, A: AllocatorGlobalHandle + 'static> ClassOpaqueContainer
    for BBox<T, A>
{
    type Inner = T;

    fn into_raw(self) -> NonNull<c_void> {
        unsafe { NonNull::new(BBox::into_raw(self)).unwrap_unchecked().cast() }
    }

    unsafe fn from_raw(ptr: NonNull<c_void>) -> Self {
        unsafe { BBox::<T, A>::from_raw_in(ptr.cast().as_ptr(), A::make_handle()) }
    }

    unsafe fn ptr_to_inner(ptr: NonNull<c_void>) -> NonNull<Self::Inner> {
        ptr.cast()
    }
}

// ============================================================================================== //

unsafe impl<T: Sized + 'static> ClassOpaque for Rc<T> {
    unsafe fn drop_opaque(ptr: NonNull<c_void>) {
        unsafe { drop(Rc::<T>::from_raw(ptr.cast().as_ptr())) }
    }
}

unsafe impl<T: Sized + 'static> ClassOpaqueContainer for Rc<T> {
    type Inner = T;

    fn into_raw(self) -> NonNull<c_void> {
        unsafe {
            NonNull::new(Rc::into_raw(self) as *mut T)
                .unwrap_unchecked()
                .cast()
        }
    }

    unsafe fn from_raw(ptr: NonNull<c_void>) -> Self {
        unsafe { Rc::<T>::from_raw(ptr.cast().as_ptr()) }
    }

    unsafe fn ptr_to_inner(ptr: NonNull<c_void>) -> NonNull<Self::Inner> {
        ptr.cast()
    }
}
