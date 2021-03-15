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

use egui::PaintJobs;
use interfaces::any::ISendSyncAny;
use std::ops::DerefMut;
use std::sync::Mutex;

///
/// This interface is used for getting an `egui::CtxRef`.
///
pub trait IEguiContextProvider: ISendSyncAny {
    /// Gets a `egui::CtxRef` handle.
    fn get_context(&self) -> egui::CtxRef;
}

/// Concrete implementation of `IEguiContextProvider`
pub struct EguiContextProvider {
    ctx: egui::CtxRef,
}

impl Default for EguiContextProvider {
    fn default() -> Self {
        EguiContextProvider {
            ctx: egui::CtxRef::default(),
        }
    }
}

impl IEguiContextProvider for EguiContextProvider {
    fn get_context(&self) -> egui::CtxRef {
        self.ctx.clone()
    }
}

interfaces::any::declare_interfaces!(EguiContextProvider, [IEguiContextProvider]);

///
/// This interface is used as a slot for storing/passing the egui rendering data off to a renderer.
///
/// A renderer can choose to support egui by checking for this interface, and using it to get the
/// current frame's paint jobs.
///
pub trait IEguiRenderData: ISendSyncAny {
    /// Replace the old paint job data with the newly provided data.
    fn put(&self, data: PaintJobs);

    /// Take the current paint job data, leaving an empty job list in its place.
    fn take(&self) -> PaintJobs;
}

/// Concrete implementation of `IEguiRenderData`
pub struct EguiRenderData {
    slot: Mutex<PaintJobs>,
}

impl Default for EguiRenderData {
    fn default() -> Self {
        Self {
            slot: Mutex::new(vec![]),
        }
    }
}

impl IEguiRenderData for EguiRenderData {
    fn put(&self, data: PaintJobs) {
        let mut slot_lock = self.slot.lock().unwrap();
        let slot = slot_lock.deref_mut();
        *slot = data;
    }

    fn take(&self) -> PaintJobs {
        std::mem::take(self.slot.lock().unwrap().deref_mut())
    }
}

interfaces::any::declare_interfaces!(EguiRenderData, [IEguiRenderData]);
