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

pub use aleph_ecs::scheduler::*;

use crate::any::IAny;
use std::mem::ManuallyDrop;
use std::ops::{Deref, DerefMut};
use std::sync::Arc;

///
/// This trait is used to provide the engine with a central [Schedule] object to schedule work onto.
///
pub trait IScheduleProvider: IAny {
    /// Provides a ref-counted pointer to an [IScheduleCell] which is used to gate ownership of the
    /// schedule
    fn get(&self) -> Arc<dyn IScheduleCell>;
}

/// A thread safe cell that is used to pass ownership of a [Schedule] around to different users.
///
/// # Implementor Note
///
/// It is assumed that an implementation of this interface *does not* use a lock
/// (Mutex, RwLock, etc). This interface was designed with an implementation backed by an AtomicCell
/// which can hand a pointer sized object between threads in a thread safe way without locks.
///
/// It would be best to respect this expectation for performance reasons.
pub trait IScheduleCell: 'static {
    /// Take ownership of the schedule and remove it from the cell, leaving the cell empty.
    ///
    /// # Panics
    ///
    /// Will panic if [IScheduleCell::take] is called while the cell is empty.
    fn take(&self) -> Box<Schedule>;

    /// Return ownership of the schedule to the cell, placing the given schedule back into the cell.
    ///
    /// # Warning
    ///
    /// While technically a different schedule can be placed back into an empty cell, it is likely a
    /// very bad idea to do so.
    ///
    /// # Panics
    ///
    /// Will panic if the cell is not empty.
    fn store(&self, schedule: Box<Schedule>);

    /// A wrapper function that yields a scoped wrapper object that provides a [Deref] and
    /// [DerefMut] implementation for [Schedule].
    ///
    /// The [ScheduleScope] object will handle taking the schedule from the cell, and returning it
    /// to the cell it was taken from with a custom [Drop] impl.
    ///
    /// Prefer using this interface over manually using [IScheduleCell::take] and
    /// [IScheduleCell::store].
    fn get(&self) -> ScheduleScope;
}

/// A scoping wrapper that provides an easier to use interface over the top of an [IScheduleCell].
pub struct ScheduleScope<'a> {
    cell: &'a dyn IScheduleCell,
    schedule: ManuallyDrop<Box<Schedule>>,
}

impl<'a> ScheduleScope<'a> {
    /// Constructs a [ScheduleScope] by taking from the given cell.
    ///
    /// Will automatically return the schedule to the [IScheduleCell] when `Self` is dropped.
    #[inline]
    pub fn take_from(cell: &'a dyn IScheduleCell) -> Self {
        let schedule = cell.take();
        Self {
            cell,
            schedule: ManuallyDrop::new(schedule),
        }
    }
}

impl<'a> Deref for ScheduleScope<'a> {
    type Target = Schedule;

    #[inline]
    fn deref(&self) -> &Self::Target {
        self.schedule.as_ref()
    }
}

impl<'a> DerefMut for ScheduleScope<'a> {
    #[inline]
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.schedule.as_mut()
    }
}

impl<'a> Drop for ScheduleScope<'a> {
    #[inline]
    fn drop(&mut self) {
        unsafe {
            // SAFETY: It is impossible to construct a ScheduleScope that does not contain a valid
            //         Box<Schedule> and the ManuallyDrop wrapper can't be used after this as this
            //         is a drop handler.
            let schedule = ManuallyDrop::take(&mut self.schedule);
            self.cell.store(schedule)
        }
    }
}
