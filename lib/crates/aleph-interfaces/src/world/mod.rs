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
use std::ops::{Deref, DerefMut};
use std::sync::Arc;

pub use aleph_ecs::world::*;

use crate::any::IAny;

///
/// This trait is used to provide the engine with a central [World] object to store game objects
/// and implement gameplay logic with.
///
pub trait IWorldProvider: IAny {
    /// Provides a ref-counted pointer to an [IWorldCell] which is used to gate ownership of the
    /// schedule
    fn get(&self) -> Arc<dyn IWorldCell>;
}

/// A thread safe cell that is used to pass ownership of a [World] around to different users.
///
/// # Implementor Note
///
/// It is assumed that an implementation of this interface *does not* use a lock
/// (Mutex, RwLock, etc). This interface was designed with an implementation backed by an AtomicCell
/// which can hand a pointer sized object between threads in a thread safe way without locks.
///
/// It would be best to respect this expectation for performance reasons.
pub trait IWorldCell: Send + Sync + 'static {
    /// Take ownership of the world and remove it from the cell, leaving the cell empty.
    ///
    /// # Panics
    ///
    /// Will panic if [IWorldCell::take] is called while the cell is empty.
    fn take(&self) -> Box<World>;

    /// Return ownership of the world to the cell, placing the given world back into the cell.
    ///
    /// # Warning
    ///
    /// While technically a different world can be placed back into an empty cell, it is likely a
    /// very bad idea to do so.
    ///
    /// # Panics
    ///
    /// Will panic if the cell is not empty.
    fn store(&self, world: Box<World>);

    /// A wrapper function that yields a scoped wrapper object that provides a [Deref] and
    /// [DerefMut] implementation for [World].
    ///
    /// The [WorldScope] object will handle taking the world from the cell, and returning it to
    /// the cell it was taken from with a custom [Drop] impl.
    ///
    /// Prefer using this interface over manually using [IWorldCell::take] and [IWorldCell::store].
    fn get(&self) -> WorldScope;
}

/// A scoping wrapper that provides an easier to use interface over the top of an [IWorldCell].
pub struct WorldScope<'a> {
    cell: &'a dyn IWorldCell,
    world: ManuallyDrop<Box<World>>,
}

impl<'a> WorldScope<'a> {
    /// Constructs a [WorldScope] by taking from the given cell.
    ///
    /// Will automatically return the world to the [IWorldCell] when `Self` is dropped.
    #[inline]
    pub fn take_from(cell: &'a dyn IWorldCell) -> Self {
        let world = cell.take();
        Self {
            cell,
            world: ManuallyDrop::new(world),
        }
    }
}

impl<'a> Deref for WorldScope<'a> {
    type Target = World;

    #[inline]
    fn deref(&self) -> &Self::Target {
        self.world.as_ref()
    }
}

impl<'a> DerefMut for WorldScope<'a> {
    #[inline]
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.world.as_mut()
    }
}

impl<'a> Drop for WorldScope<'a> {
    #[inline]
    fn drop(&mut self) {
        unsafe {
            // SAFETY: It is impossible to construct a WorldScope that does not contain a valid
            //         Box<World> and the ManuallyDrop wrapper can't be used after this as this is
            //         a drop handler.
            let world = ManuallyDrop::take(&mut self.world);
            self.cell.store(world)
        }
    }
}
