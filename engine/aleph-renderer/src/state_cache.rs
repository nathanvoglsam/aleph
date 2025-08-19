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

use std::any::{Any, TypeId};
use std::cell::UnsafeCell;
use std::collections::HashMap;
use std::hash::{DefaultHasher, Hasher};
use std::ptr::NonNull;
use std::sync::Arc;

use crate::IShaderAccessor;

pub struct StateCache {
    table: CacheTable,
    shader_db: Box<dyn IShaderAccessor + Send + Sync + 'static>,
}

impl StateCache {
    pub fn new(shader_db: Box<dyn IShaderAccessor + Send + Sync + 'static>) -> Self {
        Self {
            table: CacheTable::new(),
            shader_db,
        }
    }

    pub fn shader_db(&self) -> &(dyn IShaderAccessor + Send + Sync + 'static) {
        self.shader_db.as_ref()
    }

    pub fn insert<K: IStateCacheKey>(&mut self, k: &K, v: K::Storage) -> Arc<K::Storage> {
        self.table.insert(k, v)
    }

    pub fn get<K: IStateCacheKey>(&self, k: &K) -> Option<Arc<K::Storage>> {
        self.table.get(k)
    }

    pub fn get_or_insert_with<K: IStateCacheKey>(
        &mut self,
        k: &K,
        f: impl FnOnce(&mut StateCache, &K) -> K::Storage,
    ) -> Arc<K::Storage> {
        if let Some(v) = self.get(k) {
            v
        } else {
            let v = f(self, k);
            self.insert(k, v)
        }
    }

    pub fn take<K: IStateCacheKey>(&mut self, k: &K) -> Option<Arc<K::Storage>> {
        self.table.take(k)
    }

    pub fn clear(&mut self) {
        self.table.clear();
    }
}

pub trait IStateCacheKey: std::hash::Hash + Eq + Send + Sync + 'static {
    type Storage: Any + Send + Sync;
}

#[derive(PartialEq, Eq, Hash)]
struct CacheKey {
    uuid: TypeId,
    key: u64,
}

struct CacheTable {
    pub(crate) resources: HashMap<CacheKey, UnsafeCell<CacheArc>>,
}

impl CacheTable {
    pub fn new() -> Self {
        Self {
            resources: HashMap::new(),
        }
    }

    pub fn insert<K: IStateCacheKey>(&mut self, k: &K, v: K::Storage) -> Arc<K::Storage> {
        let key = Self::key_to_cache_key(k);

        // Create the arc to store into the table, as well as a copy to send out to the caller
        let v = CacheArc::new(v);
        let out = unsafe { v.get_unchecked::<K::Storage>() };
        let v = UnsafeCell::new(v);

        let _ = self.resources.insert(key, v);

        out
    }

    pub fn get<K: IStateCacheKey>(&self, k: &K) -> Option<Arc<K::Storage>> {
        let key = Self::key_to_cache_key(k);

        let cell = self.resources.get(&key)?;
        unsafe {
            let b = &*cell.get();
            Some(b.get_unchecked::<K::Storage>())
        }
    }

    pub fn take<K: IStateCacheKey>(&mut self, k: &K) -> Option<Arc<K::Storage>> {
        let key = Self::key_to_cache_key(k);

        let cell = self.resources.remove(&key)?;
        unsafe { Some(cell.into_inner().into_inner_unchecked::<K::Storage>()) }
    }

    pub fn clear(&mut self) {
        self.resources.clear();
    }

    fn key_to_cache_key<K: IStateCacheKey>(k: &K) -> CacheKey {
        let mut hasher = DefaultHasher::new();
        k.hash(&mut hasher);
        let key = hasher.finish();

        CacheKey {
            uuid: TypeId::of::<K::Storage>(),
            key,
        }
    }
}

unsafe impl Send for CacheTable {}
unsafe impl Sync for CacheTable {}

pub(crate) struct CacheArc {
    ptr: NonNull<()>,
    free_fn: unsafe fn(NonNull<()>),
}

unsafe impl Send for CacheArc {}
unsafe impl Sync for CacheArc {}

impl CacheArc {
    fn new<T: Send + Sync + 'static>(v: T) -> Self {
        let v = Arc::new(v);
        let v = Arc::into_raw(v);
        unsafe {
            let v: NonNull<T> = NonNull::new(v as *mut _).unwrap_unchecked();
            Self {
                ptr: v.cast(),
                free_fn: Self::free_fn::<T>,
            }
        }
    }

    pub(crate) unsafe fn get_unchecked<T: Send + Sync + 'static>(&self) -> Arc<T> {
        let v = self.ptr.cast::<T>();
        let v = unsafe { Arc::from_raw(v.as_ptr()) };

        // Take an owned copy of the object then leak the old one we materialized from the pointer
        // so we don't decrement the refcount incorrectly and trigger use after free.
        let out = v.clone();
        let _ = Arc::into_raw(v);

        out
    }

    pub(crate) unsafe fn into_inner_unchecked<T: Send + Sync + 'static>(self) -> Arc<T> {
        let v = self.ptr.cast::<T>();
        let v = unsafe { Arc::from_raw(v.as_ptr()) };

        // Make sure we don't drop the TypedTableArc and drop the Arc we just gave out incorrectly
        std::mem::forget(self);

        v
    }

    unsafe fn free_fn<T: Send + Sync + 'static>(v: NonNull<()>) {
        let v = v.cast::<T>().as_ptr();
        let v = unsafe { Arc::from_raw(v) };
        drop(v);
    }
}

impl Drop for CacheArc {
    fn drop(&mut self) {
        unsafe {
            (self.free_fn)(self.ptr);
        }
    }
}
