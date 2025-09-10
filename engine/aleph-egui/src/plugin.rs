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

use std::ops::Deref;

use aleph_alloc::instrumentation::IAllocationCategory;
use egui::ClippedPrimitive;
use interfaces::any::AnyArc;
use interfaces::label::make_label;
use interfaces::make_plugin_description_for_crate;
use interfaces::platform::{IClipboard, IEvents, IFrameTimer, IKeyboard, IMouse, IWindow};
use interfaces::plugin::{
    IPlugin, IPluginRegistrar, IRegistryAccessor, InitOrder, PluginDescription, Provides,
};
use interfaces::schedule::CoreStage;

use crate::traits::{EguiContextProvider, EguiRenderData};
use crate::{Egui, IEguiContextProvider, IEguiRenderData, RenderData};

pub struct PluginEgui();

impl PluginEgui {
    pub fn new() -> Self {
        Self()
    }
}

impl IPlugin for PluginEgui {
    fn get_description(&self) -> PluginDescription {
        make_plugin_description_for_crate!()
    }

    fn register(&mut self, registrar: &mut dyn IPluginRegistrar) {
        // We export two interfaces for interacting with egui
        registrar.provides::<dyn IEguiContextProvider>(Provides::Always);
        registrar.provides::<dyn IEguiRenderData>(Provides::Always);

        // We need to get handles to all these when we initialize to save querying them every frame
        registrar.requires::<dyn IWindow>(InitOrder::After);
        registrar.requires::<dyn IMouse>(InitOrder::After);
        registrar.requires::<dyn IKeyboard>(InitOrder::After);
        registrar.requires::<dyn IFrameTimer>(InitOrder::After);
        registrar.requires::<dyn IEvents>(InitOrder::After);
        registrar.requires::<dyn IClipboard>(InitOrder::After);
    }

    fn on_init(&mut self, registry: &mut dyn IRegistryAccessor) {
        let render_data: AnyArc<EguiRenderData> = AnyArc::default();
        let context_provider: AnyArc<EguiContextProvider> = AnyArc::default();

        let window = registry.get_interface::<dyn IWindow>().unwrap();
        let mouse = registry.get_interface::<dyn IMouse>().unwrap();
        let keyboard = registry.get_interface::<dyn IKeyboard>().unwrap();
        let frame_timer = registry.get_interface::<dyn IFrameTimer>().unwrap();
        let events = registry.get_interface::<dyn IEvents>().unwrap();
        let clipboard = registry.get_interface::<dyn IClipboard>().unwrap();

        let pre_update_keyboard = keyboard.clone();
        let pre_update_frame_timer = frame_timer.clone();
        let pre_update_ctx = context_provider.clone();
        let pre_update_window = window.clone();
        let pre_update_events = events.clone();
        registry
            .core()
            .schedule
            .add_exclusive_at_start_system_to_stage(
                CoreStage::PreUpdate.into(),
                make_label!("egui::pre_update"),
                move || {
                    let context_provider = pre_update_ctx.deref();
                    let window = pre_update_window.deref();
                    let keyboard = pre_update_keyboard.deref();
                    let frame_timer = pre_update_frame_timer.deref();
                    let events = pre_update_events.deref();

                    let input = Egui::with(|| {
                        crate::utils::get_egui_input(window, keyboard, frame_timer, events)
                    });
                    context_provider.begin_frame(input);
                },
            );

        let post_update_mouse = mouse.clone();
        let post_update_rnd = render_data.clone();
        let post_update_ctx = context_provider.clone();
        registry
            .core()
            .schedule
            .add_exclusive_at_end_system_to_stage(
                CoreStage::PostUpdate.into(),
                make_label!("egui::post_update"),
                move || {
                    let render_data = post_update_rnd.deref();
                    let context_provider = post_update_ctx.deref();

                    let mouse = post_update_mouse.deref();
                    let clipboard = clipboard.deref();

                    let output = context_provider.end_frame();
                    crate::utils::process_egui_output(output.platform_output, mouse, clipboard);

                    let egui_ctx = context_provider.get_context();
                    let jobs: Vec<ClippedPrimitive> =
                        Egui::with(|| egui_ctx.tessellate(output.shapes, output.pixels_per_point));

                    render_data.put(RenderData {
                        primitives: jobs,
                        textures_delta: output.textures_delta,
                    });
                },
            );

        registry.provide::<dyn IEguiContextProvider, _>(context_provider);
        registry.provide::<dyn IEguiRenderData, _>(render_data);
    }
}

impl Default for PluginEgui {
    fn default() -> Self {
        Self::new()
    }
}

interfaces::any::declare_interfaces!(PluginEgui, [IPlugin]);
