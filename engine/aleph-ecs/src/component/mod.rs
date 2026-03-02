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

pub(crate) mod internal;

use std::alloc::{Layout, LayoutError};
use std::mem::needs_drop;
use std::ptr::NonNull;
use std::sync::LazyLock;
use std::sync::atomic::{AtomicU32, Ordering};

use aleph_alloc::nstr::NStr;

/// This trait is implemented for all component types that can be used with a
/// [`crate::world::World`].
///
/// The trait exposes the interface need for __Rust__ types to be used with an ECS world using the
/// _safe_ APIs. Dynamic component types _do not_ need to implement this trait, they couldn't
/// implement it anyway.
///
/// You aren't expected to use or implement `Component` directly. The interface exposed on this
/// trait is intended only for use by the ECS implementation. It is also unsafe to implement as the
/// ECS world assumes that the trait was implemented exactly as the [`register_component`] macro
/// implements it. Using that macro is safe, implementing this trait by hand is not.
///
/// # Safety
///
/// - The `DESC` must correctly describe the Rust type this trait is implemented on.
///   - `size` must be correct for `T`.
///   - `align` must be correct for `T`.
///   - `destructor` must be a function that calls `drop_in_place` for a type-erased array of `T`s.
///   - `id` must be globally unique for all `T`s that implement `Component`.
/// - The `DESC` __MUST__ be registered using `register_component_type` _before `fn main()`_ is
///   called.
///
/// `register_component` handles all of this.
pub unsafe trait Component: Sized + Send + Sync + 'static {
    /// A name that can be used to identify the type implementing [`Component`]. This name is not
    /// guaranteed to uniquely identify the type, only the ID may do that. This name should only
    /// be used for logging or other human visible use cases.
    const NAME: &'static NStr;

    /// A static reference to an [`ComponentDescription`] instance that describes the [`Component`].
    const DESC: &'static LazyLock<ComponentDescription>;
}

/// A [`u32`] new-type that represents the ID of a component type within an ECS world.
#[repr(transparent)]
#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Debug)]
pub struct ComponentId(pub(crate) u32);

/// FFI portable type description table. Contains all the information exposed by [`Component`]
/// wrapped in a neat little struct that can be safely sent across FFI boundaries.
#[derive(Clone, Hash, Debug)]
#[repr(C)]
pub struct ComponentDescription {
    /// The auto-increment assigned ID.
    pub id: ComponentId,

    /// Size, in bytes, of the underlying object type.
    pub size: usize,

    /// Alignment, in bytes, of the underlying object type.
    pub align: usize,

    /// Human-readable name of the underlying type. Not guaranteed to be unique.
    pub name: &'static NStr,

    /// Opaque fn-ptr to a drop wrapper that will drop a packed array of 'count' objects. This can
    /// be set to 'None' if the underlying type returns false for [`needs_drop`].
    pub destructor: Option<unsafe extern "C" fn(NonNull<()>, count: u64)>,
}

impl ComponentDescription {
    /// Constructs a new [`ComponentDescription`] for the given type `T`.
    ///
    /// A globally unique [`ComponentId`] will be assigned to the new instance using an internal,
    /// global counter.
    pub fn new<T: Sized>(name: &'static NStr) -> Self {
        unsafe extern "C" fn object_destructor<T: Sized>(this: NonNull<()>, count: u64) {
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

        static __COMPONENT_COUNTER: AtomicU32 = AtomicU32::new(0);

        // If ID == u32::MAX then we just overflowed the counter and we're in for sadness. Panic
        // instead.
        let id = __COMPONENT_COUNTER.fetch_add(1, Ordering::SeqCst);
        assert!(id < u32::MAX);

        Self {
            id: ComponentId(id),
            size: size_of::<T>(),
            align: align_of::<T>(),
            name,
            destructor: if needs_drop::<T>() {
                Some(object_destructor::<T>)
            } else {
                None
            },
        }
    }

    /// Will (attempt to) make a [`Layout`] from the stored `size` and `align`.
    pub const fn type_layout(&self) -> Result<Layout, LayoutError> {
        Layout::from_size_align(self.size, self.align)
    }
}

/// This macro is _the_ way to implement [`Component`] for a type, and is required to be able to use
/// any Rust type as a component. This is the only safe way to implement `Component`.
///
/// This macro handles:
///
/// 1. Correctly constructing the `DESC` so it correctly describes `T`.
/// 2. Assigns a globally unique `ComponentId` that is never shared with any other Rust type in the
///   same program.
/// 3. Ensures the component description is registered into a global list before `fn main()` is
///   called.
///
/// Pieces 1 and 2 are fairly mechanical, but 3 is critical.
///
/// Rust types are automatically registered with any ECS world. The global registry maintained by
/// this macro is how that is done. The reason this registration _must_ be performed is that an
/// unregistered __Rust__ component type could have an ID collision with a __dynamic__ component
/// type. An unregistered Rust component could lead to reading the data of a dynamic type as the
/// data for some Rust type `T`.
///
/// This will almost certainly lead to UB.
///
/// ### The 10,000 ton caveat
///
/// Technically.
///
/// ...Technically... We're still not entirely safe.
///
/// We use the `ctor` crate which provides a mechanism to run Rust functions using
/// `__attribute__((destructor))` or the equivalent for the target platform. This is the same
/// mechanism C++ static initializers use. If someone were to, say, construct an ECS world _inside
/// an `__attribute__((destructor))`_ function they are able to invoke the above problem.
///
/// There's no guarantee that the ECS world isn't constructed after all the Rust component types
/// have finished registering themselves. In which case, it's entirely possible for types to be
/// unregistered with that world and lead to UB.
///
/// However, I don't care.
///
/// The path to invoking this is so contrived I don't see it as a serious flaw. The ECS world is
/// something that lives inside `fn main()`. There is no good use for an ECS world in a pre-main
/// constructor. I think the trade-offs with how our auto-registration works is worth it, and that
/// making the API unsafe for such a ridiculous niche use case isn't worth it.
///
/// There's also likely issues around dynamic linking too, if two different Rust .dll files get
/// loaded into the same program, and they both use this ECS crate, and they share their ECS worlds
/// with each other. I don't think there's a way to write a purely safe Rust program where this is
/// possible to cause though because dlopen is unsafe, and Rust doesn't support dynamic linking
/// in a way that could invoke this edge case.
#[macro_export]
macro_rules! register_component {
    ($t: path) => {
        impl $t {
            #[doc(hidden)]
            const fn __internal_component_type_desc()
            -> &'static ::std::sync::LazyLock<$crate::component::ComponentDescription> {
                fn make() -> $crate::component::ComponentDescription {
                    $crate::component::ComponentDescription::new::<$t>($crate::nstr::nstr!(
                        concat!(module_path!(), "::", stringify!($t))
                    ))
                }
                static TYPE_DESC: ::std::sync::LazyLock<$crate::component::ComponentDescription> =
                    ::std::sync::LazyLock::new(make);
                &TYPE_DESC
            }
        }
        impl $t {
            #[doc(hidden)]
            const fn __internal_component_node() -> &'static $crate::init_list::ListItem<
                &'static ::std::sync::LazyLock<$crate::component::ComponentDescription>,
            > {
                static ENTRY: $crate::init_list::ListItem<
                    &'static ::std::sync::LazyLock<$crate::component::ComponentDescription>,
                > = $crate::init_list::ListItem::new(<$t>::__internal_component_type_desc());
                &ENTRY
            }

            #[doc(hidden)]
            const fn __internal_register_component_node_scope() -> bool {
                #[$crate::ctor::ctor(crate_path = $crate::ctor)]
                fn internal_register_t() {
                    unsafe {
                        $crate::register_component_type(<$t>::__internal_component_node());
                    }
                }
                true
            }
        }
        unsafe impl $crate::component::Component for $t {
            const NAME: &'static $crate::nstr::NStr =
                $crate::nstr::nstr!(concat!(module_path!(), "::", stringify!($t)));
            const DESC: &'static ::std::sync::LazyLock<$crate::component::ComponentDescription> =
                <$t>::__internal_component_type_desc();
        }
    };
}
