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

use std::alloc::{handle_alloc_error, Layout};
use std::collections::HashMap;
use std::marker::PhantomData;
use std::ptr::NonNull;

use aleph_atomic_borrow::AtomicBorrow;
use aleph_math::{DVec3, Rotor3, Vec3};
use aleph_object_system::uuid::Uuid;
use aleph_object_system::{IObject, ObjectDescription};
use allocator_api2::alloc::{Allocator, Global};

pub struct RenderScene {
    components: HashMap<Uuid, Storage>,
}

impl RenderScene {
    pub fn new() -> Self {
        Self {
            components: HashMap::new(),
        }
    }

    pub fn push<T: IObject>(&mut self, transform: RenderTransform, object: T) {
        let table = self
            .components
            .entry(T::ID)
            .or_insert_with(|| Storage::new::<T>());
        unsafe {
            let ptr = NonNull::from(&object);
            let ptr = ptr.cast::<u8>();
            table.push(transform, ptr);
            std::mem::forget(object);
        }
    }

    pub fn reserve_storage<T: IObject>(&mut self) {
        let _ = self
            .components
            .entry(T::ID)
            .or_insert_with(|| Storage::new::<T>());
    }

    pub fn reserve_storage_with_capacity<T: IObject>(&mut self, capacity: usize) {
        let _ = self
            .components
            .entry(T::ID)
            .or_insert_with(|| Storage::with_capacity::<T>(capacity));
    }

    pub fn clear(&mut self) {
        for (_, s) in self.components.iter_mut() {
            s.clear();
        }
    }

    pub fn get_storage_ref<T: IObject>(&self) -> Option<StorageRef<T>> {
        if let Some(storage) = self.components.get(&T::ID) {
            if !storage.borrow.borrow() {
                return None;
            }
            let v = StorageRef {
                s: NonNull::from(storage),
                _phantom: PhantomData::default(),
                _phantom_t: PhantomData::default(),
            };
            Some(v)
        } else {
            None
        }
    }

    pub fn get_storage_mut<T: IObject>(&self) -> Option<StorageMut<T>> {
        if let Some(storage) = self.components.get(&T::ID) {
            if !storage.borrow.borrow_mut() {
                return None;
            }
            let v = StorageMut {
                s: NonNull::from(storage),
                _phantom: PhantomData::default(),
                _phantom_t: PhantomData::default(),
            };
            Some(v)
        } else {
            None
        }
    }
}

pub struct StorageRef<'a, T> {
    s: NonNull<Storage>,
    _phantom: PhantomData<&'a RenderScene>,
    _phantom_t: PhantomData<&'a T>,
}

impl<'a, T: IObject> StorageRef<'a, T> {
    #[inline]
    pub fn as_slice_ref(&self) -> (&[RenderTransform], &[T]) {
        unsafe { self.s.as_ref().as_slice_ref() }
    }
}

impl<'a, T> Drop for StorageRef<'a, T> {
    fn drop(&mut self) {
        unsafe {
            self.s.as_ref().borrow.release();
        }
    }
}

pub struct StorageMut<'a, T> {
    s: NonNull<Storage>,
    _phantom: PhantomData<&'a RenderScene>,
    _phantom_t: PhantomData<&'a T>,
}

impl<'a, T: IObject> StorageMut<'a, T> {
    #[inline]
    pub fn as_slice_ref(&self) -> (&[RenderTransform], &[T]) {
        unsafe { self.s.as_ref().as_slice_ref() }
    }

    #[inline]
    pub fn as_slice_mut(&mut self) -> (&mut [RenderTransform], &mut [T]) {
        unsafe { self.s.as_mut().as_slice_mut() }
    }

    #[inline]
    pub fn push(&mut self, transform: RenderTransform, object: T) {
        unsafe {
            let ptr = NonNull::from(&object);
            let ptr = ptr.cast::<u8>();
            self.s.as_mut().push(transform, ptr);
            std::mem::forget(object);
        }
    }

    #[inline]
    pub fn extend_from_storage(&mut self, other: &mut StorageMut<T>) {
        unsafe {
            let dst = self.s.as_mut();
            let source = other.s.as_mut();
            dst.extend_from_storage(source);
        }
    }

    #[inline]
    pub fn extend_from_iter(&mut self, iter: impl ExactSizeIterator<Item = (RenderTransform, T)>) {
        unsafe {
            let dst = self.s.as_mut();
            dst.extend_from_iter(iter);
        }
    }
}

impl<'a, T: IObject + Clone> StorageMut<'a, T> {
    #[inline]
    pub fn extend_from_slice(&mut self, transforms: &[RenderTransform], source: &[T]) {
        unsafe {
            let dst = self.s.as_mut();
            dst.extend_from_slice(transforms, source);
        }
    }
}

impl<'a, T> Drop for StorageMut<'a, T> {
    #[inline]
    fn drop(&mut self) {
        unsafe {
            self.s.as_ref().borrow.release_mut();
        }
    }
}

struct Storage {
    transforms: Vec<RenderTransform>,
    bytes: RawBytes,
    borrow: AtomicBorrow,
    t_id: Uuid,
    t_size: usize,
    t_destructor: unsafe extern "C" fn(NonNull<()>, count: u64),
}

impl Storage {
    fn new<T: IObject>() -> Self {
        Self::new_for(ObjectDescription::get::<T>())
    }

    fn new_for(desc: ObjectDescription) -> Self {
        Self {
            transforms: Vec::new(),
            bytes: RawBytes::new(desc.align),
            borrow: AtomicBorrow::new(),
            t_id: desc.id,
            t_size: desc.size,
            t_destructor: desc.destructor,
        }
    }

    fn with_capacity<T: IObject>(capacity: usize) -> Self {
        Self::with_capacity_for(ObjectDescription::get::<T>(), capacity)
    }

    fn with_capacity_for(desc: ObjectDescription, capacity: usize) -> Self {
        Self {
            transforms: Vec::with_capacity(capacity * desc.size),
            bytes: RawBytes::with_capacity(desc.align, capacity * desc.size),
            borrow: AtomicBorrow::new(),
            t_id: desc.id,
            t_size: desc.size,
            t_destructor: desc.destructor,
        }
    }

    #[inline(always)]
    fn is_empty(&self) -> bool {
        self.transforms.is_empty()
    }

    #[inline(always)]
    fn len(&self) -> usize {
        self.transforms.len()
    }

    /// It is the caller's responsibility to ensure the 'src' points to an object of the type this
    /// storage is supposed to be storing.
    unsafe fn push(&mut self, transform: RenderTransform, src: NonNull<u8>) {
        // Grow the buffer if there's not enough space
        if self.bytes.len() <= self.len() * self.t_size {
            // Take max(self.len, 1) to handle the len=0 case which would never grow the buffer
            let new_len = (self.len() * 2).max(1) * self.t_size;
            self.bytes.resize(new_len);
        }

        let dst = self.bytes.as_ptr();
        let dst = dst.add(self.t_size * self.len());
        dst.copy_from_nonoverlapping(src.cast(), self.t_size);

        self.transforms.push(transform);
    }

    fn clear(&mut self) {
        if !self.is_empty() {
            // Safety:
            // - It's impossible for the byte buffer to be zero sized if len > 0
            // - It's impossible for the items in the buffer to be the wrong type without _other_
            //   unsafe code.
            // - The byte buffer is guaranteed to contain 'self.len' objects of the correct type for
            //   self.desc.destructor.
            unsafe {
                let ptr = self.bytes.as_ptr();
                (self.t_destructor)(ptr.cast(), self.len() as u64);
                self.transforms.clear();
            }
        }
    }

    fn extend_from_storage(&mut self, source: &mut Storage) {
        assert_eq!(self.t_id, source.t_id);
        assert_eq!(self.t_size, source.t_size);
        assert_eq!(self.bytes.align, source.bytes.align);

        // Grow the buffer geometryically so it will fit the existing items and the items we're
        // pulling from the other storage
        let target_len = self.len() + source.len();
        if (target_len * self.t_size) > self.bytes.len() {
            let mut new_len = (self.len() * 2).max(1);
            while new_len < target_len {
                new_len *= 2;
            }
            unsafe {
                self.bytes.resize(new_len * self.t_size);
            }
        }

        // Perform the actual copy of the components from the source storage.
        unsafe {
            let src = source.bytes.as_ptr();
            let dst = self.bytes.as_ptr();
            let dst = dst.add(self.len() * self.t_size);
            dst.copy_from_nonoverlapping(src, source.len() * self.t_size);

            self.transforms.extend_from_slice(&source.transforms);
        }

        // Reset the source storage _without_ calling any drop functions as the components inside
        // were moved and not copied.
        source.transforms.clear();
    }

    fn extend_from_slice<T: IObject + Clone>(
        &mut self,
        transforms: &[RenderTransform],
        source: &[T],
    ) {
        assert_eq!(self.t_id, T::ID);
        assert_eq!(self.t_size, T::SIZE);
        assert_eq!(self.bytes.align, T::ALIGN);
        assert_eq!(transforms.len(), source.len());

        // Grow the buffer geometryically so it will fit the existing items and the items we're
        // pulling from the other storage
        let target_len = self.len() + source.len();
        self.grow_to_meet_target_len(target_len);

        // Perform the actual copy of the components from the source storage.
        unsafe {
            let mut src = source.as_ptr();
            let mut dst = self.bytes.as_ptr().cast::<T>().add(self.len());
            for _ in 0..source.len() {
                dst.write((*src).clone());
                dst = dst.add(1);
                src = src.add(1);
            }

            self.transforms.extend_from_slice(transforms);
        }
    }

    fn extend_from_iter<T: IObject>(
        &mut self,
        iter: impl ExactSizeIterator<Item = (RenderTransform, T)>,
    ) {
        assert_eq!(self.t_id, T::ID);
        assert_eq!(self.t_size, T::SIZE);
        assert_eq!(self.bytes.align, T::ALIGN);

        // Grow the buffer geometryically so it will fit the existing items and the items we're
        // pulling from the other storage
        let target_len = self.len() + iter.len();
        self.grow_to_meet_target_len(target_len);

        // Perform the actual copy of the components from the source storage.
        unsafe {
            let mut dst = self.bytes.as_ptr().cast::<T>().add(self.len());
            for (t, o) in iter {
                dst.write(o);
                dst = dst.add(1);
                self.transforms.push(t);
            }
        }
    }

    fn grow_to_meet_target_len(&mut self, target_len: usize) {
        // Grow the buffer geometryically so it will fit the existing items and the items we're
        // pulling from the other storage
        if (target_len * self.t_size) > self.bytes.len() {
            let mut new_len = (self.len() * 2).max(1);
            while new_len < target_len {
                new_len *= 2;
            }
            unsafe {
                self.bytes.resize(new_len * self.t_size);
            }
        }
    }

    fn as_slice_ref<T: IObject>(&self) -> (&[RenderTransform], &[T]) {
        assert_eq!(T::ID, self.t_id);
        if !self.is_empty() {
            unsafe {
                let ptr = self.bytes.as_ptr().cast::<T>();
                let out = std::slice::from_raw_parts(ptr.as_ptr(), self.len());
                (self.transforms.as_slice(), out)
            }
        } else {
            (&[], &[])
        }
    }

    fn as_slice_mut<T: IObject>(&mut self) -> (&mut [RenderTransform], &mut [T]) {
        assert_eq!(T::ID, self.t_id);
        if !self.is_empty() {
            unsafe {
                let ptr = self.bytes.as_ptr().cast::<T>();
                let out = std::slice::from_raw_parts_mut(ptr.as_ptr(), self.len());
                (self.transforms.as_mut_slice(), out)
            }
        } else {
            (&mut [], &mut [])
        }
    }
}

impl Drop for Storage {
    fn drop(&mut self) {
        self.clear();
    }
}

struct RawBytes<A: Allocator = Global> {
    slice: NonNull<[u8]>,
    align: usize,
    a: A,
}

impl RawBytes {
    fn new(align: usize) -> Self {
        Self::new_in(align, Global)
    }

    fn with_capacity(align: usize, capacity: usize) -> Self {
        Self::with_capacity_in(align, capacity, Global)
    }
}

impl<A: Allocator> RawBytes<A> {
    fn new_in(align: usize, a: A) -> Self {
        assert!(align != 0);
        assert!(align.is_power_of_two());
        let v = NonNull::slice_from_raw_parts(NonNull::<u8>::dangling(), 0);
        Self { slice: v, align, a }
    }

    fn with_capacity_in(align: usize, capacity: usize, a: A) -> Self {
        let layout = Layout::from_size_align(capacity, align).unwrap();
        match a.allocate(layout) {
            Ok(v) => Self { slice: v, align, a },
            Err(_) => handle_alloc_error(layout),
        }
    }

    unsafe fn resize(&mut self, new_len: usize) {
        let layout = Layout::from_size_align(new_len, self.align).unwrap();
        let mut v = match self.a.allocate(layout) {
            Ok(v) => v,
            Err(_) => handle_alloc_error(layout),
        };

        let src = self.slice.as_ref().as_ptr();
        let dst = v.as_mut().as_mut_ptr();
        dst.copy_from_nonoverlapping(src, self.slice.len());

        self.slice = v;
    }

    const fn len(&self) -> usize {
        self.slice.len()
    }

    fn as_ptr(&self) -> NonNull<u8> {
        unsafe { NonNull::new_unchecked(self.slice.as_ref().as_ptr() as *mut u8) }
    }
}

impl<A: Allocator> Drop for RawBytes<A> {
    fn drop(&mut self) {
        unsafe {
            let layout = Layout::from_size_align(self.slice.len(), self.align).unwrap();
            let ptr = NonNull::new_unchecked(self.slice.as_ptr() as *mut u8);
            self.a.deallocate(ptr, layout);
        }
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
pub struct RenderTransform {
    pub pos: DVec3,
    pub rot: Rotor3,
    pub scl: Vec3,
}

#[cfg(test)]
mod tests {
    use std::fmt::Debug;
    use std::sync::Arc;

    use aleph_math::{DVec3, Rotor3, Vec3};
    use aleph_object_system::unsafe_impl_iobject;

    use crate::{RenderScene, RenderTransform};

    #[derive(Clone, PartialEq, Debug)]
    struct A(u32);
    unsafe_impl_iobject!(A, "019228e4-3931-71e0-8624-c918dea2b2a3");

    #[derive(Clone, PartialEq, Debug)]
    struct B(f32);
    unsafe_impl_iobject!(B, "019228e4-e12a-7463-b3aa-644388d80a60");

    #[derive(Clone, PartialEq, Debug)]
    struct C(Arc<u32>);
    unsafe_impl_iobject!(C, "019228e4-ff0d-74f0-8d3c-891e38b7bb21");

    fn assert_slices<T: PartialEq + Debug>(a: &[T], b: &[T]) {
        assert_eq!(a.len(), b.len());
        a.iter()
            .zip(b.iter())
            .for_each(|(got, expected)| assert_eq!(got, expected));
    }

    fn a_expected() -> ([RenderTransform; 3], [A; 3]) {
        (
            [
                RenderTransform {
                    pos: DVec3::new(1., 2., 3.),
                    rot: Rotor3::identity(),
                    scl: Vec3::new(1.0, 2.0, 3.0),
                },
                RenderTransform {
                    pos: DVec3::new(4., 5., 6.),
                    rot: Rotor3::identity(),
                    scl: Vec3::new(1123.0, 762.0, 312.0),
                },
                RenderTransform {
                    pos: DVec3::new(1., 2., 3.),
                    rot: Rotor3::from_euler_angles(1., 2., 3.),
                    scl: Vec3::new(113.0, 62.0, 31.0),
                },
            ],
            [A(21), A(22), A(23)],
        )
    }

    fn b_expected() -> ([RenderTransform; 5], [B; 5]) {
        (
            [
                RenderTransform {
                    pos: DVec3::new(56., 21., 3.),
                    rot: Rotor3::from_euler_angles(1.8, 2.2, 3.6),
                    scl: Vec3::new(113.0, 62.0, 31.0),
                },
                RenderTransform {
                    pos: DVec3::new(41., 51., 65.),
                    rot: Rotor3::identity(),
                    scl: Vec3::new(1123.0, 762.0, 312.0),
                },
                RenderTransform {
                    pos: DVec3::new(14., 27., 32.),
                    rot: Rotor3::from_euler_angles(1.2, 2.1, 3.3),
                    scl: Vec3::new(1.0, 2.0, 3.0),
                },
                RenderTransform {
                    pos: DVec3::new(141., 51., 65.),
                    rot: Rotor3::identity(),
                    scl: Vec3::new(1.0, 2.0, 3.0),
                },
                RenderTransform {
                    pos: DVec3::new(514., 277., 32.),
                    rot: Rotor3::from_euler_angles(1.2, 2.5, 1.3),
                    scl: Vec3::new(1.0, 2.0, 3.0),
                },
            ],
            [B(569.0), B(12345.0), B(54321.0), B(1956.0), B(2024.21)],
        )
    }

    #[test]
    pub fn create_empty_scene() {
        let _scene = RenderScene::new();
    }

    #[test]
    pub fn create_scene() {
        let mut scene = RenderScene::new();

        let (expected_transforms, expected_objects) = a_expected();
        for (t, o) in expected_transforms.iter().zip(expected_objects.iter()) {
            scene.push(t.clone(), o.clone());
        }

        let storage = scene.get_storage_ref::<A>().unwrap();
        let (transforms, objects) = storage.as_slice_ref();

        assert_slices(transforms, &expected_transforms);
        assert_slices(objects, &expected_objects);
    }

    #[test]
    pub fn create_multi_type_scene() {
        let mut scene = RenderScene::new();

        let (a_expected_transforms, a_expected_objects) = a_expected();
        for (t, o) in a_expected_transforms.iter().zip(a_expected_objects.iter()) {
            scene.push(t.clone(), o.clone());
        }

        let (b_expected_transforms, b_expected_objects) = b_expected();
        for (t, o) in b_expected_transforms.iter().zip(b_expected_objects.iter()) {
            scene.push(t.clone(), o.clone());
        }

        let storage = scene.get_storage_ref::<A>().unwrap();
        let (a_transforms, a_objects) = storage.as_slice_ref();
        assert_slices(a_transforms, &a_expected_transforms);
        assert_slices(a_objects, &a_expected_objects);

        let storage = scene.get_storage_ref::<B>().unwrap();
        let (b_transforms, b_objects) = storage.as_slice_ref();
        assert_slices(b_transforms, &b_expected_transforms);
        assert_slices(b_objects, &b_expected_objects);
    }

    #[test]
    pub fn create_multi_type_scene_through_accessor() {
        let mut scene = RenderScene::new();
        scene.reserve_storage::<A>();
        scene.reserve_storage::<B>();

        let mut a_storage = scene.get_storage_mut::<A>().unwrap();
        let (a_expected_transforms, a_expected_objects) = a_expected();
        for (t, o) in a_expected_transforms.iter().zip(a_expected_objects.iter()) {
            a_storage.push(t.clone(), o.clone());
        }

        let (b_expected_transforms, b_expected_objects) = b_expected();
        let mut b_storage = scene.get_storage_mut::<B>().unwrap();
        for (t, o) in b_expected_transforms.iter().zip(b_expected_objects.iter()) {
            b_storage.push(t.clone(), o.clone());
        }

        let (a_transforms, a_objects) = a_storage.as_slice_ref();
        assert_slices(a_transforms, &a_expected_transforms);
        assert_slices(a_objects, &a_expected_objects);

        let (b_transforms, b_objects) = b_storage.as_slice_ref();
        assert_slices(b_transforms, &b_expected_transforms);
        assert_slices(b_objects, &b_expected_objects);
    }

    #[test]
    pub fn dynamic_borrow_checking() {
        let mut scene = RenderScene::new();
        scene.reserve_storage::<A>();
        scene.reserve_storage::<B>();

        let _a_storage = scene.get_storage_mut::<A>().unwrap();
        let _b_storage = scene.get_storage_mut::<B>().unwrap();
        assert!(scene.get_storage_ref::<A>().is_none());
        assert!(scene.get_storage_ref::<B>().is_none());
        assert!(scene.get_storage_mut::<A>().is_none());
        assert!(scene.get_storage_mut::<B>().is_none());
    }

    #[test]
    pub fn extend_from_other_storage() {
        let mut dst_scene = RenderScene::new();
        dst_scene.reserve_storage::<A>();
        dst_scene.reserve_storage::<B>();

        let mut a_storage = dst_scene.get_storage_mut::<A>().unwrap();
        let (a_expected_transforms, a_expected_objects) = a_expected();
        for (t, o) in a_expected_transforms.iter().zip(a_expected_objects.iter()) {
            a_storage.push(t.clone(), o.clone());
        }

        let mut b_scene = RenderScene::new();
        b_scene.reserve_storage::<B>();

        let (b_expected_transforms, b_expected_objects) = b_expected();
        for (t, o) in b_expected_transforms.iter().zip(b_expected_objects.iter()) {
            b_scene.push(t.clone(), o.clone());
        }

        let mut dst_b_storage = dst_scene.get_storage_mut::<B>().unwrap();
        let mut b_storage = b_scene.get_storage_mut::<B>().unwrap();
        dst_b_storage.extend_from_storage(&mut b_storage);

        drop(a_storage);
        drop(b_storage);
        drop(dst_b_storage);

        let a_storage = dst_scene.get_storage_ref::<A>().unwrap();
        let (a_transforms, a_objects) = a_storage.as_slice_ref();
        assert_slices(a_transforms, &a_expected_transforms);
        assert_slices(a_objects, &a_expected_objects);

        let b_storage = dst_scene.get_storage_ref::<B>().unwrap();
        let (b_transforms, b_objects) = b_storage.as_slice_ref();
        assert_slices(b_transforms, &b_expected_transforms);
        assert_slices(b_objects, &b_expected_objects);
    }

    #[test]
    pub fn extend_from_slices() {
        let mut dst_scene = RenderScene::new();
        dst_scene.reserve_storage::<A>();
        dst_scene.reserve_storage::<B>();

        let mut a_storage = dst_scene.get_storage_mut::<A>().unwrap();
        let (a_expected_transforms, a_expected_objects) = a_expected();
        for (t, o) in a_expected_transforms.iter().zip(a_expected_objects.iter()) {
            a_storage.push(t.clone(), o.clone());
        }

        let (b_expected_transforms, b_expected_objects) = b_expected();
        let mut dst_b_storage = dst_scene.get_storage_mut::<B>().unwrap();
        dst_b_storage.extend_from_slice(&b_expected_transforms, &b_expected_objects);

        drop(a_storage);
        drop(dst_b_storage);

        let a_storage = dst_scene.get_storage_ref::<A>().unwrap();
        let (a_transforms, a_objects) = a_storage.as_slice_ref();
        assert_slices(a_transforms, &a_expected_transforms);
        assert_slices(a_objects, &a_expected_objects);

        let b_storage = dst_scene.get_storage_ref::<B>().unwrap();
        let (b_transforms, b_objects) = b_storage.as_slice_ref();
        assert_slices(b_transforms, &b_expected_transforms);
        assert_slices(b_objects, &b_expected_objects);
    }

    #[test]
    pub fn extend_from_iter() {
        let mut dst_scene = RenderScene::new();
        dst_scene.reserve_storage::<A>();
        dst_scene.reserve_storage::<B>();

        let mut a_storage = dst_scene.get_storage_mut::<A>().unwrap();
        let (a_expected_transforms, a_expected_objects) = a_expected();
        for (t, o) in a_expected_transforms.iter().zip(a_expected_objects.iter()) {
            a_storage.push(t.clone(), o.clone());
        }

        let (b_expected_transforms, b_expected_objects) = b_expected();
        let mut dst_b_storage = dst_scene.get_storage_mut::<B>().unwrap();
        dst_b_storage.extend_from_iter(
            b_expected_transforms
                .into_iter()
                .zip(b_expected_objects.into_iter()),
        );

        drop(a_storage);
        drop(dst_b_storage);

        let a_storage = dst_scene.get_storage_ref::<A>().unwrap();
        let (a_transforms, a_objects) = a_storage.as_slice_ref();
        assert_slices(a_transforms, &a_expected_transforms);
        assert_slices(a_objects, &a_expected_objects);

        let (b_expected_transforms, b_expected_objects) = b_expected();
        let b_storage = dst_scene.get_storage_ref::<B>().unwrap();
        let (b_transforms, b_objects) = b_storage.as_slice_ref();
        assert_slices(b_transforms, &b_expected_transforms);
        assert_slices(b_objects, &b_expected_objects);
    }

    #[test]
    pub fn drop_check_1() {
        let mut scene = RenderScene::new();
        scene.reserve_storage::<C>();

        let a = C(Arc::new(56));
        let b = C(Arc::new(21));

        assert_eq!(Arc::strong_count(&a.0), 1);
        assert_eq!(Arc::strong_count(&b.0), 1);

        scene.push(RenderTransform::default(), a.clone());
        scene.push(RenderTransform::default(), b.clone());

        assert_eq!(Arc::strong_count(&a.0), 2);
        assert_eq!(Arc::strong_count(&b.0), 2);

        let mut dst_scene = RenderScene::new();
        dst_scene.reserve_storage::<C>();
        let mut src_storage = scene.get_storage_mut::<C>().unwrap();
        let mut dst_storage = dst_scene.get_storage_mut::<C>().unwrap();

        dst_storage.extend_from_storage(&mut src_storage);

        drop(src_storage);
        drop(dst_storage);

        assert_eq!(Arc::strong_count(&a.0), 2);
        assert_eq!(Arc::strong_count(&b.0), 2);

        scene.clear();

        assert_eq!(Arc::strong_count(&a.0), 2);
        assert_eq!(Arc::strong_count(&b.0), 2);

        dst_scene.clear();

        assert_eq!(Arc::strong_count(&a.0), 1);
        assert_eq!(Arc::strong_count(&b.0), 1);
    }
}
