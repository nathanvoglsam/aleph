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

pub extern crate aleph_nstr as nstr;
pub extern crate ctor;
pub extern crate uuid;

use std::collections::HashMap;
use std::mem::{ManuallyDrop, needs_drop};
use std::ops::{Deref, DerefMut};
use std::ptr::NonNull;
use std::sync::atomic::{AtomicPtr, Ordering};
use std::sync::{Arc, LazyLock};

use aleph_nstr::NStr;

/// This trait represents the capability of a type to interact with the 'object system'
pub unsafe trait IObject: Sized {
    /// A stable, globally unique UUID. This must forever be stable and must always uniquely
    /// identify the type implementing [`IObject`]. It is the implementors responsibility to uphold
    /// the uniqueness and stability guarantees of this interface.
    const ID: uuid::Uuid;

    /// The size, in bytes, of the type implementing [`IObject`].
    const SIZE: usize;

    /// The alignment, in bytes, of the type implementing [`IObject`].
    const ALIGN: usize;

    /// A name that can be used to identify the type implementing [`IObject`]. This name is not
    /// guaranteed to uniquely identify the type, only the ID may do that. This name should only
    /// be used for logging or other human visible use cases.
    const NAME: &'static NStr;

    /// A static reference to an [`ObjectDescription`] instance that describes the [`IObject`].
    const DESC: &'static ObjectDescription;
}

/// A type-erased destructor function that can be called on a pointer that is expected to be
/// pointing to a non-aliased, owned instance of the type implementing [`IObject`].
///
/// It is the _caller's_ (of [`IObject::destructor`], not the implementor of this trait)
/// responsibility to ensure the pointer given to this function points to a valid, live 'Self',
/// and that the access to this object is correctly synchronized.
///
/// This also takes a 'count' parameter, for denoting the number of objects to destroy. This
/// function will assume that the given pointer points to an array of 'count' objects and drop
/// all of them. It is the caller's responsibility to ensure this is correct.
pub unsafe extern "C" fn object_destructor<T: Sized>(this: NonNull<()>, count: u64) {
    unsafe {
        let mut base = this.cast::<T>();
        let mut count = count;
        while count != 0 {
            base.drop_in_place();
            base = base.add(1);
            count -= 1;
        }
    }
}

/// This is a 'const' wrapper function that will return a destructor function pointer if the given
/// 'T' returns true for [`needs_drop`].
pub const fn get_object_destructor_for<T: Sized>()
-> Option<unsafe extern "C" fn(NonNull<()>, count: u64)> {
    if needs_drop::<T>() {
        Some(object_destructor::<T>)
    } else {
        None
    }
}

/// Shorthand for getting the ID of an object of type `T`
pub const fn object_id<T: IObject>() -> uuid::Uuid {
    T::ID
}

/// Shorthand for getting the name of an object of type `T`
pub const fn object_name<T: IObject>() -> &'static NStr {
    T::NAME
}

/// FFI portable object description table. Contains all the information exposed by [`IObject`]
/// wrapped in a neat little struct that can be safely sent across FFI boundaries.
#[repr(C)]
#[derive(Clone)]
pub struct ObjectDescription {
    /// Type UUID that uniquely, globally identifies the underlying type of the object
    pub id: uuid::Uuid,

    /// Size, in bytes, of the underlying object type
    pub size: usize,

    /// Alignment, in bytes, of the underlying object type
    pub align: usize,

    /// Human-readable name of the underlying type. Not guaranteed to be unique.
    pub name: &'static NStr,

    /// Opaque fn-ptr to a drop wrapper that will drop a packed array of 'count' objects. This can
    /// be set to 'None' if the underlying type returns false for [`needs_drop`].
    pub destructor: Option<unsafe extern "C" fn(NonNull<()>, count: u64)>,

    /// Opaque fn-ptr to a drop wrapper that will drop a packed array of 'count'
    /// `Arc<Object<T>>` objects. That is: the type is `[Arc<Object<T>>]`. This function
    /// is always populated as even if the underlying type returns false for [`needs_drop`], we
    /// still need to call the [`Arc`] drop function.
    pub destructor_arc: unsafe extern "C" fn(NonNull<()>, count: u64),
    // /// Opaque fn-ptr to a drop wrapper that will drop a packed array of 'count'
    // /// `Box<Object<T>>` objects. That is: the type is `[Box<Object<T>>]`. This function
    // /// is always populated as even if the underlying type returns false for [`needs_drop`], we
    // /// still need to call the [`Box`] drop function.
    // pub destructor_box: unsafe extern "C" fn(NonNull<()>, count: u64),
}

impl ObjectDescription {
    /// Constructs a [`ObjectDescription`] for a given type `T`
    pub const fn get<T: IObject>() -> &'static Self {
        T::DESC
    }
}

#[repr(transparent)]
#[derive(Clone)]
pub struct ArcObject {
    inner: ManuallyDrop<Arc<ObjectHeader>>,
}

impl ArcObject {
    #[inline]
    pub fn from_object<T: IObject>(v: Arc<Object<T>>) -> Self {
        unsafe {
            let ptr = Arc::into_raw(v);
            let ptr = ptr as *const ObjectHeader;
            ArcObject {
                inner: ManuallyDrop::new(Arc::from_raw(ptr)),
            }
        }
    }

    ///
    /// Gets the number of strong ([`ArcObject`]) pointers to this allocation.
    ///
    /// # Safety
    ///
    /// This method by itself is safe, but using it correctly requires extra care.
    /// Another thread can change the strong count at any time,
    /// including potentially between calling this method and acting on the result.
    ///
    /// # Info
    ///
    /// This is just a wrapper around [`Arc::strong_count`]
    ///
    #[inline]
    #[must_use]
    pub fn strong_count(&self) -> usize {
        Arc::strong_count(&self.inner)
    }

    #[inline]
    pub fn downcast<U: IObject>(&self) -> Option<Arc<Object<U>>> {
        unsafe {
            if U::ID == self.inner.desc.id {
                let ptr = Arc::into_raw(self.inner.deref().clone());
                let ptr = ptr as *const Object<U>;
                let arc = Arc::from_raw(ptr);
                Some(arc)
            } else {
                None
            }
        }
    }

    #[inline]
    pub fn downcast_ref<U: IObject>(&self) -> Option<&U> {
        unsafe {
            if U::ID == self.inner.desc.id {
                let ptr: NonNull<ObjectHeader> = NonNull::from(self.inner.as_ref());
                Some(&ptr.cast::<Object<U>>().as_ref().object)
            } else {
                None
            }
        }
    }
}

impl Drop for ArcObject {
    fn drop(&mut self) {
        unsafe {
            let ptr = NonNull::from(&self.inner);
            let ptr = ptr.cast::<()>();
            let f = self.inner.desc.destructor_arc;
            f(ptr, 1)
        }
    }
}

/// FFI portable header struct that must be placed as the first field to use our opaque IObject
/// containers.
#[repr(C)]
#[derive(Clone)]
pub struct ObjectHeader {
    /// Reference to the object type's description table.
    pub desc: &'static ObjectDescription,

    /// Optional pointer to the trait object vtable for the primary trait the object has declared.
    ///
    /// This is used to enable thin trait objects.
    pub primary_trait: Option<NonNull<()>>,
}

unsafe impl Send for ObjectHeader {}
unsafe impl Sync for ObjectHeader {}

/// Container used for bundling an object with a header that can be used to interact with the type
/// behind type-erased interfaces.
///
/// This container is `repr(C)` (and so is the header itself) in order to guarantee the layout. We
/// must place the header before the object in memory so that a pointer to `Object<T>` is also a
/// valid pointer to `ObjectHeader`, such that you can freely view one as the other. This is what
/// enables our type-erasure. We can always use the information in the header to determine which
/// `Object<T>` a bare `ObjectHeader` can be interpreted as.
#[repr(C)]
pub struct Object<T: IObject> {
    header: ObjectHeader,
    object: T,
}

impl<T: IObject> Object<T> {
    /// Constructs a new [`Object`] wrapping the given object.
    pub const fn new(object: T) -> Self {
        Self {
            header: ObjectHeader {
                desc: T::DESC,
                primary_trait: None,
            },
            object,
        }
    }

    /// Constructs a new `Arc<Object<T>>` wrapping the given object.
    #[inline]
    pub fn new_arc(object: T) -> Arc<Object<T>> {
        Arc::new(Self::new(object))
    }

    /// Constructs a new `Arc<Object<T>>` wrapping the given object that is then converted into
    /// an opaque [`ArcObject`].
    #[inline]
    pub fn new_arc_opaque(object: T) -> ArcObject {
        let arc = Self::new_arc(object);
        ArcObject::from_object(arc)
    }
}

impl<T: IObject> Deref for Object<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.object
    }
}

impl<T: IObject> DerefMut for Object<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.object
    }
}

/// This macro can be used to implement [`IObject`] on an object as a shorthand compared to manually
/// implementing it directly. This will correctly generate a (mostly) safe implementation of
/// [`IObject`] for the given type.
///
/// # Safety
///
/// This macro is still, strictly speaking, unsafe. We can't protect the caller from duplicate UUIDs
/// or if the caller changes the UUID.
#[macro_export]
macro_rules! unsafe_impl_iobject {
    ($t: path, $id: literal) => {
        impl $t {
            #[doc(hidden)]
            const fn __internal_vtable() -> &'static $crate::ObjectDescription {
                static VTABLE: $crate::ObjectDescription = $crate::ObjectDescription {
                    id: $crate::uuid::uuid!($id),
                    size: ::std::mem::size_of::<$t>(),
                    align: ::std::mem::align_of::<$t>(),
                    name: $crate::nstr::nstr!(concat!(module_path!(), "::", stringify!($t))),
                    destructor: $crate::get_object_destructor_for::<$t>(),
                    destructor_arc: $crate::object_destructor::<::std::sync::Arc<$crate::Object<$t>>>,
                    // destructor_box: $crate::object_destructor::<::std::boxed::Box<$t>>,
                };
                &VTABLE
            }
        }
        impl $t {
            #[doc(hidden)]
            const fn __internal_node() -> &'static $crate::ObjectTypeListNode {
                static VTABLE: $crate::ObjectTypeListNode = $crate::ObjectTypeListNode::new(<$t>::__internal_vtable());
                &VTABLE
            }

            #[doc(hidden)]
            const fn __internal_register_node_scope() -> bool {
                #[$crate::ctor::ctor(crate_path = $crate::ctor)]
                fn internal_register_t() {
                    unsafe {
                        $crate::register_type(<$t>::__internal_node());
                    }
                }
                true
            }
        }
        unsafe impl $crate::IObject for $t {
            const ID: $crate::uuid::Uuid = <$t>::__internal_vtable().id;
            const SIZE: usize = <$t>::__internal_vtable().size;
            const ALIGN: usize = <$t>::__internal_vtable().align;
            const NAME: &'static $crate::nstr::NStr = <$t>::__internal_vtable().name;
            const DESC: &'static $crate::ObjectDescription = <$t>::__internal_vtable();
        }
    };
}

/// A lazily initialized table of all types registered into the object system.
///
/// This is dervied from [`ObjectTypeIter`] by using it to walk the internal type list. This is the
/// preferred API of [`ObjectTypeIter`] as after the initial setup the hash map is much more
/// efficient to query.
pub static TYPES: LazyLock<HashMap<uuid::Uuid, &'static ObjectDescription>> = LazyLock::new(|| {
    assert_no_duplicate_ids_registered();
    HashMap::from_iter(ObjectTypeIter::new().map(|v| (v.id, v)))
});

/// An object that integrates with the object system to iterate over all declared IObject types.
///
/// The list is filled out using '__attribute__((constructor))' functions generated by the macros.
/// All the list setup is done before main and so once main is entered the list will always be
/// available.
///
/// # Performance
///
/// In general please use [`TYPES`]. This directly walks the linked list that gets built before
/// main. While this is safe, it's not particularly fast. [`TYPES`] is much more efficient to
/// iterate.
pub struct ObjectTypeIter {
    next: *mut ObjectTypeListNode,
}

impl Default for ObjectTypeIter {
    fn default() -> Self {
        Self::new()
    }
}

impl ObjectTypeIter {
    /// Constructs a new [`ObjectTypeIter`] instance.
    pub fn new() -> Self {
        // All accesses to the list pointer must leave it as null or a valid static pointer to a
        // 'ObjectTypeListNode' instance. All access to the head pointer is unsafe gated and so it's
        // impossible to break this expectation without other unsafe code.
        let next = unsafe { object_type_list_head().load(Ordering::Relaxed) };
        Self { next }
    }
}

impl Iterator for ObjectTypeIter {
    type Item = &'static ObjectDescription;

    fn next(&mut self) -> Option<Self::Item> {
        let ptr = self.next;
        if ptr.is_null() {
            None
        } else {
            unsafe {
                let out: &'static ObjectTypeListNode = ptr.as_ref().unwrap_unchecked();
                self.next = out.next.load(Ordering::Relaxed);
                Some(out.vtable)
            }
        }
    }
}

/// Utility function that will walk the list of registered types and assert if there are any
/// duplicate type IDs registered.
pub fn assert_no_duplicate_ids_registered() {
    let mut types = HashMap::new();
    for object in ObjectTypeIter::new() {
        let existing = types.insert(object.id, object);
        if let Some(existing) = existing {
            assert_eq!(object.id, existing.id); // Just being careful
            panic!(
                "Colliding IObject type IDs detected. '{}' and '{}' have the same ID of '{}'!",
                existing.name, object.name, existing.id
            );
        }
    }
}

/// Super-duper ultra unsafe do not access but needs to be public so macros can touch it.
///
/// Internal layout of the nodes that form the linked list of types.
///
/// Repr C so we can share it with foreign code if we need to.
#[doc(hidden)]
#[repr(C)]
pub struct ObjectTypeListNode {
    vtable: &'static ObjectDescription,
    next: AtomicPtr<ObjectTypeListNode>,
}

impl ObjectTypeListNode {
    /// Super duper ultra unsafe do not access but needs to be public so macros can touch it.
    #[doc(hidden)]
    pub const fn new(vtable: &'static ObjectDescription) -> Self {
        Self {
            vtable,
            next: AtomicPtr::new(core::ptr::null_mut()),
        }
    }
}

/// Super-duper ultra unsafe do not access but needs to be public so macros can touch it.
///
/// Forms the head of the linked list of all declared IObject types. Will be setup before main.
///
/// Call to get a reference to the head of the list. Unsafe because if you call this outside of this
/// library (it's implementation detail) I will beat you.
#[doc(hidden)]
pub const unsafe fn object_type_list_head() -> &'static AtomicPtr<ObjectTypeListNode> {
    static OBJECT_TYPE_LIST_HEAD: AtomicPtr<ObjectTypeListNode> =
        AtomicPtr::new(core::ptr::null_mut());
    &OBJECT_TYPE_LIST_HEAD
}

/// Super-duper ultra unsafe do not access but needs to be public so macros can touch it.
///
/// Utility function used by the unsafe_impl_iobject macro for registering the type into the global
/// linked list of all types.
#[doc(hidden)]
pub unsafe fn register_type(v: &'static ObjectTypeListNode) {
    unsafe {
        let ptr: *mut ObjectTypeListNode =
            v as *const ObjectTypeListNode as *mut ObjectTypeListNode;
        let next = object_type_list_head().swap(ptr, Ordering::SeqCst);
        v.next.store(next, Ordering::SeqCst);
    }
}

// struct Test();
// unsafe_impl_iobject!(Test, "019212d8-f424-7221-8d34-97b9f318bf0b");
