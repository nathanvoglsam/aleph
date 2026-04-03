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

use std::any::Any;
use std::ops::DerefMut;
use std::sync::Mutex;

use aleph_alloc::instrumentation::IAllocationCategory;
use egui::{ClippedPrimitive, FullOutput, RawInput};
use engine_api::make_interface_identifier;

use crate::Egui;

///
/// This interface is used for getting an `egui::CtxRef`.
///
pub trait IEguiContextProvider: Any + Send + Sync {
    /// Gets a `egui::CtxRef` handle.
    fn get_context(&self) -> egui::Context;
}

make_interface_identifier!(AEguiContextProvider, IEguiContextProvider);

/// Concrete implementation of `IEguiContextProvider`
pub struct EguiContextProvider {
    ctx: egui::Context,
}

impl EguiContextProvider {
    pub fn begin_frame(&self, new_input: RawInput) {
        Egui::with(|| self.ctx.begin_pass(new_input));
    }

    pub fn end_frame(&self) -> FullOutput {
        Egui::with(|| self.ctx.end_pass())
    }
}

impl Default for EguiContextProvider {
    fn default() -> Self {
        let ctx = egui::Context::default();
        EguiContextProvider { ctx }
    }
}

impl IEguiContextProvider for EguiContextProvider {
    fn get_context(&self) -> egui::Context {
        self.ctx.clone()
    }
}

///
/// This interface is used as a slot for storing/passing the egui rendering data off to a renderer.
///
/// A renderer can choose to support egui by checking for this interface, and using it to get the
/// current frame's paint jobs.
///
pub trait IEguiRenderData: Any + Send + Sync {
    /// Replace the old paint job data with the newly provided data.
    fn put(&self, data: RenderData);

    /// Take the current paint job data, leaving an empty job list in its place.
    fn take(&self) -> RenderData;
}

make_interface_identifier!(AEguiRenderData, IEguiRenderData);

pub struct RenderData {
    pub primitives: Vec<ClippedPrimitive>,
    pub textures_delta: egui::TexturesDelta,
}

/// Concrete implementation of `IEguiRenderData`
pub struct EguiRenderData {
    slot: Mutex<Option<RenderData>>,
}

impl Default for EguiRenderData {
    fn default() -> Self {
        Self {
            slot: Mutex::new(None),
        }
    }
}

impl IEguiRenderData for EguiRenderData {
    fn put(&self, data: RenderData) {
        let mut slot_lock = self.slot.lock().unwrap();
        let slot = slot_lock.deref_mut();
        *slot = Some(data);
    }

    fn take(&self) -> RenderData {
        let mut slot = None;
        std::mem::swap(self.slot.lock().unwrap().deref_mut(), &mut slot);
        slot.unwrap()
    }
}
