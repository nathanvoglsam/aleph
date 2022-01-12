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

use crossbeam::atomic::AtomicCell;
use interfaces::schedule::{IScheduleCell, IScheduleProvider, Schedule, ScheduleScope};
use std::sync::Arc;

pub struct ScheduleProvider {
    schedule: Arc<dyn IScheduleCell>,
}

impl ScheduleProvider {
    pub fn new(schedule: Schedule) -> Self {
        let schedule = Box::new(schedule);
        let schedule = AtomicCell::new(Some(schedule));
        let schedule = ScheduleCell { cell: schedule };
        Self {
            schedule: Arc::new(schedule),
        }
    }
}

impl IScheduleProvider for ScheduleProvider {
    fn get(&self) -> Arc<dyn IScheduleCell> {
        self.schedule.clone()
    }
}

interfaces::any::declare_interfaces!(ScheduleProvider, [IScheduleProvider]);

/// Internal implementation of IScheduleCell
struct ScheduleCell {
    cell: AtomicCell<Option<Box<Schedule>>>,
}

impl IScheduleCell for ScheduleCell {
    fn take(&self) -> Box<Schedule> {
        self.cell.take().unwrap()
    }

    fn store(&self, schedule: Box<Schedule>) {
        assert!(self.cell.swap(Some(schedule)).is_none());
    }

    fn get(&self) -> ScheduleScope {
        ScheduleScope::take_from(self)
    }
}
