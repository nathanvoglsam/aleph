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

#[doc(hidden)]
pub extern crate aleph_nstr as nstr;

#[doc(hidden)]
pub extern crate uuid;

#[doc(hidden)]
pub extern crate ctor;

#[doc(hidden)]
pub extern crate const_format;

pub extern crate aleph_mimalloc as mimalloc;

pub type Root = cfg_select! {
    feature = "force-system-alloc" => std::alloc::System,
    _ => mimalloc::MiMalloc,
};

pub const fn root() -> Root {
    cfg_select! {
        feature = "force-system-alloc" => std::alloc::System,
        _ => mimalloc::MiMalloc,
    }
}

pub mod allocator_global_handle;
pub mod instrumentation;
pub mod mallocator;
pub mod offset_allocator;

pub use allocator_api2::boxed::Box as BBox;
pub use allocator_api2::vec::Vec as BVec;
pub use allocator_api2::*;
pub use blink_alloc::*;
pub use hashbrown::*;

pub type BHashMap<K, V, A = alloc::Global> = HashMap<K, V, DefaultHashBuilder, A>;
pub type BHashSet<T, A = alloc::Global> = HashSet<T, DefaultHashBuilder, A>;

/// Allows turning a [`BBox<T: Sized, A>`][BBox] into a [`BBox<U: ?Sized, A>`][BBox] where `T` can
/// be unsizing-coerced into a `U`.
///
/// This is the only way to create an `BBox` of an unsized type on stable.
///
/// With the standard library's `alloc::boxed::Box`, this is done automatically using the unstable
/// unsize traits, but this crate's Box can't take advantage of that machinery on stable. So, we
/// need to use type inference and the fact that you *can* still coerce the inner pointer of a box
/// to get the compiler to help us unsize it using this macro.
///
/// # Example
///
/// ```
/// use aleph_alloc::unsize_bbox;
/// use aleph_alloc::BBox;
/// use core::any::Any;
///
/// let sized_box: BBox<u64> = BBox::new(0);
/// let unsized_box: BBox<dyn Any> = unsize_bbox!(sized_box);
/// ```
#[macro_export]
macro_rules! unsize_bbox {( $boxed:expr $(,)? ) => ({
    let (ptr, allocator) = $crate::BBox::into_raw_with_allocator($boxed);
    // we don't want to allow casting to arbitrary type U, but we do want to allow unsize coercion to happen.
    // that's exactly what's happening here -- this is *not* a pointer cast ptr as *mut _, but the compiler
    // *will* allow an unsizing coercion to happen into the `ptr` place, if one is available. And we use _ so that the user can
    // fill in what they want the unsized type to be by annotating the type of the variable this macro will
    // assign its result to.
    let ptr: *mut _ = ptr;
    // SAFETY: see above for why ptr's type can only be something that can be safely coerced.
    // also, ptr just came from a properly allocated box in the same allocator.
    unsafe {
        $crate::BBox::from_raw_in(ptr, allocator)
    }
})}
