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

use egui::paint::ClippedShape;
use egui::{ClippedMesh, Output, RawInput};
use interfaces::any::IAny;
use std::borrow::Cow;
use std::collections::BTreeMap;
use std::ops::DerefMut;
use std::sync::{Mutex, RwLock};

///
/// This interface is used for getting an `egui::CtxRef`.
///
pub trait IEguiContextProvider: IAny + Send + Sync {
    /// Gets a `egui::CtxRef` handle.
    fn get_context(&self) -> egui::CtxRef;
}

/// Concrete implementation of `IEguiContextProvider`
pub struct EguiContextProvider {
    ctx: RwLock<egui::CtxRef>,
}

impl EguiContextProvider {
    pub fn begin_frame(&self, new_input: RawInput) {
        self.ctx.write().unwrap().begin_frame(new_input);
    }

    pub fn end_frame(&self) -> (Output, Vec<ClippedShape>) {
        self.ctx.read().unwrap().end_frame()
    }
}

impl Default for EguiContextProvider {
    fn default() -> Self {
        let fonts = egui_font_definitions(true);
        let ctx = egui::CtxRef::default();
        ctx.set_fonts(fonts);
        EguiContextProvider {
            ctx: RwLock::new(ctx),
        }
    }
}

impl IEguiContextProvider for EguiContextProvider {
    fn get_context(&self) -> egui::CtxRef {
        self.ctx.read().unwrap().clone()
    }
}

interfaces::any::declare_interfaces!(EguiContextProvider, [IEguiContextProvider]);

///
/// This interface is used as a slot for storing/passing the egui rendering data off to a renderer.
///
/// A renderer can choose to support egui by checking for this interface, and using it to get the
/// current frame's paint jobs.
///
pub trait IEguiRenderData: IAny {
    /// Replace the old paint job data with the newly provided data.
    fn put(&self, data: Vec<ClippedMesh>);

    /// Take the current paint job data, leaving an empty job list in its place.
    fn take(&self) -> Vec<ClippedMesh>;
}

/// Concrete implementation of `IEguiRenderData`
pub struct EguiRenderData {
    slot: Mutex<Vec<ClippedMesh>>,
}

impl Default for EguiRenderData {
    fn default() -> Self {
        Self {
            slot: Mutex::new(vec![]),
        }
    }
}

impl IEguiRenderData for EguiRenderData {
    fn put(&self, data: Vec<ClippedMesh>) {
        let mut slot_lock = self.slot.lock().unwrap();
        let slot = slot_lock.deref_mut();
        *slot = data;
    }

    fn take(&self) -> Vec<ClippedMesh> {
        std::mem::take(self.slot.lock().unwrap().deref_mut())
    }
}

interfaces::any::declare_interfaces!(EguiRenderData, [IEguiRenderData]);

fn egui_font_definitions(jetbrains: bool) -> egui::FontDefinitions {
    let mut font_data = BTreeMap::new();
    let mut fonts_for_family = BTreeMap::new();

    let jetbrains_mono_name = "JetbrainsMono";
    let jetbrains_mono = crate::fonts::jetbrains_mono_regular();
    let cascadia_code_name = "CascadiaCode";
    let cascadia_code = crate::fonts::cascadia_code();
    let noto_sans_name = "NotoSans-Regular";
    let noto_sans = crate::fonts::noto_sans_regular();
    let noto_emoji_name = "NotoEmoji-Regular";
    let noto_emoji = crate::fonts::noto_emoji_regular();
    let emoji_icons_name = "emoji-icon-font";
    let emoji_icons = crate::fonts::emoji_icon_font();

    let monospace_name = if jetbrains {
        font_data.insert(
            jetbrains_mono_name.to_owned(),
            Cow::Borrowed(jetbrains_mono),
        );
        jetbrains_mono_name
    } else {
        font_data.insert(cascadia_code_name.to_owned(), Cow::Borrowed(cascadia_code));
        cascadia_code_name
    };
    font_data.insert(noto_sans_name.to_owned(), Cow::Borrowed(noto_sans));
    font_data.insert(noto_emoji_name.to_owned(), Cow::Borrowed(noto_emoji));
    font_data.insert(emoji_icons_name.to_owned(), Cow::Borrowed(emoji_icons));

    fonts_for_family.insert(
        egui::FontFamily::Monospace,
        vec![
            monospace_name.to_owned(),
            noto_sans_name.to_owned(),
            noto_emoji_name.to_owned(),
            emoji_icons_name.to_owned(),
        ],
    );
    fonts_for_family.insert(
        egui::FontFamily::Proportional,
        vec![
            noto_sans_name.to_owned(),
            noto_emoji_name.to_owned(),
            emoji_icons_name.to_owned(),
        ],
    );

    let mut family_and_size = BTreeMap::new();
    family_and_size.insert(
        egui::TextStyle::Small,
        (egui::FontFamily::Proportional, 14.0),
    );
    family_and_size.insert(
        egui::TextStyle::Body,
        (egui::FontFamily::Proportional, 17.0),
    );
    family_and_size.insert(
        egui::TextStyle::Button,
        (egui::FontFamily::Proportional, 18.0),
    );
    family_and_size.insert(
        egui::TextStyle::Heading,
        (egui::FontFamily::Proportional, 22.0),
    );
    family_and_size.insert(
        egui::TextStyle::Monospace,
        (egui::FontFamily::Monospace, 14.0),
    );

    egui::FontDefinitions {
        font_data,
        fonts_for_family,
        family_and_size,
    }
}
