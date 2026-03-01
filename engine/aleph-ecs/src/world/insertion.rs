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

use std::mem::ManuallyDrop;
use std::ptr::NonNull;

use aleph_alloc::BVec;
use aleph_alloc::alloc::Allocator;

use crate::component::{Component, ComponentId};
use crate::world::World;

/// Abstract interface for copying data into an archetype's columns. This is used for bulk insertion
/// operations.
///
/// This trait expects some type `T` to declare a list of
pub trait EntityInsertionInfo {
    /// Returns an iterator that yields the component type of each data column.
    ///
    /// The list of types does not need to be sorted, but should not contain duplicates. The
    /// [`World`] will check for duplicates.
    fn types(&self) -> impl ExactSizeIterator<Item = ComponentId>;

    /// Performs the copy operation into the archetype.
    ///
    /// # Note
    ///
    /// This requires access to internal `World` and `Archetype` fields so in practice can only be
    /// implemented inside the ECS crate.
    ///
    /// # Safety
    ///
    /// Catastrophically unsafe. The _caller_ needs to guarantee that target archetype and all of
    /// its columns are valid to write for `size_of(component) * count` bytes.
    ///
    /// This may also inherit safety requirements from the implementation too. Typically around
    /// assuming their source data columns are valid `size_of(component) * count` bytes too.
    unsafe fn copy_into_columns(
        self,
        world: &mut World,
        archetype_index: usize,
        base_row: usize,
        count: usize,
    );
}

/// A marker trait that extends [`EntityInsertionInfo`] that has additional safety implications.
///
/// This trait is implemented on [`EntityInsertionInfo`] types where it is possible to expose bulk
/// insertion as a safe operation. The raw API, if marked as safe, would allow reinterpreting
/// arbitrary bytes as any Rust component type `T`. This is not safe in the general case.
///
/// Implement this trait for [`EntityInsertionInfo`] types where it's impossible to insert garbage
/// data in for Rust types.
pub unsafe trait RustEntityInsertionInfo: EntityInsertionInfo {
    fn count(&self) -> usize;
}

/// A marker trait that extends [`EntityInsertionInfo`] with the extra requirement that the
/// entity `count` must be exactly equal to 1.
pub unsafe trait SingleEntityInsertionInfo: EntityInsertionInfo {}

/// An `id -> ptr` pair. Contextually interpreted to declare the type of data that `data` points at.
///
/// Used as part of an [`EntityInsertionInfo`] implementation to declare the data source for one
/// particular type of component.
///
/// This is FFI friendly, and intended for use with dynamic component types.
#[repr(C)]
pub struct ComponentInsertionInfo {
    /// The 'id' of the component type that is stored in this pair.
    pub id: ComponentId,

    /// An array of the component type identified by `id`. In context of [`BulkInsertionInfo`] this
    /// must be valid for a length of at least `count * id -> type.size`.
    pub data: NonNull<u8>,
}

impl<'a> EntityInsertionInfo for &'a [ComponentInsertionInfo] {
    fn types(&self) -> impl ExactSizeIterator<Item = ComponentId> {
        self.iter().map(|v| v.id)
    }

    unsafe fn copy_into_columns(
        self,
        world: &mut World,
        archetype_index: usize,
        base_row: usize,
        count: usize,
    ) {
        for t in self {
            let type_info = &world.components[t.id];

            let column_index = type_info.archetypes.get(&archetype_index).unwrap().column;
            let column = &mut world.archetypes[archetype_index].columns[column_index];

            unsafe {
                let dst_size = type_info.desc.size * count;
                let dst_ptr = column.get_at_index(base_row).unwrap_unchecked();
                dst_ptr.copy_from_nonoverlapping(t.data, dst_size);
            }
        }
    }
}

impl<T: ComponentColumn> EntityInsertionInfo for T {
    fn types(&self) -> impl ExactSizeIterator<Item = ComponentId> {
        std::iter::once(T::id())
    }

    unsafe fn copy_into_columns(
        self,
        world: &mut World,
        archetype_index: usize,
        base_row: usize,
        count: usize,
    ) {
        let type_info = &world.components[T::id()];

        let column_index = type_info.archetypes.get(&archetype_index).unwrap().column;
        let column = &mut world.archetypes[archetype_index].columns[column_index];

        unsafe {
            let dst_size = type_info.desc.size * count;
            let dst_ptr = column.get_at_index(base_row).unwrap_unchecked();
            self.copy_into_column(dst_ptr, dst_size, count);
        }
    }
}

unsafe impl<T: ComponentColumn> RustEntityInsertionInfo for T {
    fn count(&self) -> usize {
        self.len()
    }
}

unsafe impl<T: SingleComponentColumn> SingleEntityInsertionInfo for T {}

/// This interface is implemented on types which are suitable for use as a single component column.
///
/// Types that implement this trait will be assembled into a group to form a [`EntityInsertionInfo`]
/// object that then defines the number and shape of a collection of entities to insert into the
/// ECS world.
pub trait ComponentColumn {
    /// Returns the ID for the component the column contains
    fn id() -> ComponentId;

    /// Returns the number of components the column contains
    fn len(&self) -> usize;

    /// Moves the `count` components the column contains into the buffer referred to be `dst_ptr`.
    ///
    /// # Safety
    ///
    /// The implementation is not expected to ensure that `self` actually contains `count`
    /// components, nor can the implementation guarantee `dst_ptr` is valid for
    /// `size_of(component) * count` bytes.
    ///
    /// The implementation also assumes that `dst_ptr` is correctly aligned for the component type.
    unsafe fn copy_into_column(self, dst_ptr: NonNull<u8>, component_size: usize, count: usize);
}

/// Extension marker trait that should be implemented on component column types that only contain a
/// single component.
pub unsafe trait SingleComponentColumn: ComponentColumn {}

impl<T: Component> ComponentColumn for T {
    fn id() -> ComponentId {
        T::DESC.id
    }

    fn len(&self) -> usize {
        1
    }

    unsafe fn copy_into_column(self, dst_ptr: NonNull<u8>, _component_size: usize, _count: usize) {
        debug_assert_eq!(_count, 1);
        unsafe {
            dst_ptr.cast::<T>().write(self);
        }
    }
}

unsafe impl<T: Component> SingleComponentColumn for T {}

impl<T: Component, const LEN: usize> ComponentColumn for [T; LEN] {
    fn id() -> ComponentId {
        T::DESC.id
    }

    fn len(&self) -> usize {
        LEN
    }

    unsafe fn copy_into_column(self, dst_ptr: NonNull<u8>, _component_size: usize, count: usize) {
        debug_assert_eq!(count, LEN);
        unsafe {
            let this = ManuallyDrop::new(self);
            let this_ptr = NonNull::from_ref(this.as_ref());
            if !this_ptr.is_empty() {
                let this_ptr = this_ptr.cast::<T>();
                let dst_ptr = dst_ptr.cast::<T>();
                dst_ptr.copy_from_nonoverlapping(this_ptr, count);
            }
        }
    }
}

impl<T: Component> ComponentColumn for Vec<T> {
    fn id() -> ComponentId {
        T::DESC.id
    }

    fn len(&self) -> usize {
        self.len()
    }

    unsafe fn copy_into_column(self, dst_ptr: NonNull<u8>, _component_size: usize, count: usize) {
        debug_assert_eq!(count, self.len());
        unsafe {
            // Transform the vec so it contains ManuallyDrop<T> so it won't drop the components
            let (ptr, length, capacity) = Vec::into_raw_parts(self);
            let this = Vec::from_raw_parts(ptr as *mut ManuallyDrop<T>, length, capacity);

            // And do the copy. The vec still gets dropped to clean up the memory, but won't drop
            // the components
            let this_ptr = NonNull::from_ref(this.as_slice());
            if !this_ptr.is_empty() {
                let this_ptr = this_ptr.cast::<T>();
                let dst_ptr = dst_ptr.cast::<T>();
                dst_ptr.copy_from_nonoverlapping(this_ptr, count);
            }
        }
    }
}

impl<T: Component, A: Allocator> ComponentColumn for BVec<T, A> {
    fn id() -> ComponentId {
        T::DESC.id
    }

    fn len(&self) -> usize {
        self.len()
    }

    unsafe fn copy_into_column(self, dst_ptr: NonNull<u8>, _component_size: usize, count: usize) {
        debug_assert_eq!(count, self.len());
        unsafe {
            // Transform the vec so it contains ManuallyDrop<T> so it won't drop the components
            let (ptr, length, capacity, alloc) = BVec::into_raw_parts_with_alloc(self);
            let this =
                BVec::from_raw_parts_in(ptr as *mut ManuallyDrop<T>, length, capacity, alloc);

            // And do the copy. The vec still gets dropped to clean up the memory, but won't drop
            // the components
            let this_ptr = NonNull::from_ref(this.as_slice());
            if !this_ptr.is_empty() {
                let this_ptr = this_ptr.cast::<T>();
                let dst_ptr = dst_ptr.cast::<T>();
                dst_ptr.copy_from_nonoverlapping(this_ptr, count);
            }
        }
    }
}

macro_rules! impl_entity_insertion_info_for {
    ($($t: ident), *) => {
        impl<$($t: ComponentColumn),+> EntityInsertionInfo for ($($t,)+) {
            fn types(&self) -> impl ExactSizeIterator<Item = ComponentId> {
                [$(<$t as ComponentColumn>::id(),)+].into_iter()
            }

            #[allow(non_snake_case)]
            unsafe fn copy_into_columns(
                self,
                world: &mut World,
                archetype_index: usize,
                base_row: usize,
                count: usize,
            ) {
                let ($($t,)+) = self;

                $({
                    let type_info = &world.components[<$t as ComponentColumn>::id()];

                    let column_index = type_info.archetypes.get(&archetype_index).unwrap().column;
                    let column = &mut world.archetypes[archetype_index].columns[column_index];

                    unsafe {
                        let dst_size = type_info.desc.size * count;
                        let dst_ptr = column.get_at_index(base_row).unwrap_unchecked();
                        ComponentColumn::copy_into_column($t, dst_ptr, dst_size, count);
                    }
                })+
            }
        }

        unsafe impl<$($t: ComponentColumn),+> RustEntityInsertionInfo for ($($t,)+) {
            #[allow(non_snake_case)]
            fn count(&self) -> usize {
                let ($($t,)+) = self;
                let counts = [$(ComponentColumn::len($t),)+];
                let count = counts.iter().copied().min().unwrap();
                assert!(counts.iter().all(|&v| v == count), "All component channel item counts must match.");
                count
            }
        }

        unsafe impl<$($t: SingleComponentColumn),+> SingleEntityInsertionInfo for ($($t,)+) {}
    }
}

impl_entity_insertion_info_for!(A);
impl_entity_insertion_info_for!(A, B);
impl_entity_insertion_info_for!(A, B, C);
impl_entity_insertion_info_for!(A, B, C, D);
impl_entity_insertion_info_for!(A, B, C, D, E);
impl_entity_insertion_info_for!(A, B, C, D, E, F);
impl_entity_insertion_info_for!(A, B, C, D, E, F, G);
impl_entity_insertion_info_for!(A, B, C, D, E, F, G, H);
impl_entity_insertion_info_for!(A, B, C, D, E, F, G, H, I);
impl_entity_insertion_info_for!(A, B, C, D, E, F, G, H, I, J);
impl_entity_insertion_info_for!(A, B, C, D, E, F, G, H, I, J, K);
impl_entity_insertion_info_for!(A, B, C, D, E, F, G, H, I, J, K, L);
impl_entity_insertion_info_for!(A, B, C, D, E, F, G, H, I, J, K, L, M);
