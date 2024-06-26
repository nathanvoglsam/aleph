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

use std::any::TypeId;
use std::ops::Deref;

use egui::ClippedPrimitive;
use interfaces::any::{AnyArc, IAny};
use interfaces::make_plugin_description_for_crate;
use interfaces::platform::{
    IClipboardProvider, IEventsProvider, IFrameTimerProvider, IKeyboardProvider, IMouseProvider,
    IWindowProvider,
};
use interfaces::plugin::{
    IInitResponse, IPlugin, IPluginRegistrar, IRegistryAccessor, PluginDescription,
};
use interfaces::schedule::{CoreStage, IScheduleProvider};

use crate::traits::{EguiContextProvider, EguiRenderData};
use crate::{IEguiContextProvider, IEguiRenderData, RenderData};

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
        registrar.provides_interface::<dyn IEguiContextProvider>();
        registrar.provides_interface::<dyn IEguiRenderData>();

        // We need to get handles to all these when we initialize to save querying them every frame
        registrar.must_init_after::<dyn IScheduleProvider>();
        registrar.must_init_after::<dyn IWindowProvider>();
        registrar.must_init_after::<dyn IMouseProvider>();
        registrar.must_init_after::<dyn IKeyboardProvider>();
        registrar.must_init_after::<dyn IFrameTimerProvider>();
        registrar.must_init_after::<dyn IEventsProvider>();
        registrar.must_init_after::<dyn IClipboardProvider>();

        registrar.depends_on::<dyn IScheduleProvider>();
        registrar.depends_on::<dyn IWindowProvider>();
        registrar.depends_on::<dyn IMouseProvider>();
        registrar.depends_on::<dyn IKeyboardProvider>();
        registrar.depends_on::<dyn IFrameTimerProvider>();
        registrar.depends_on::<dyn IEventsProvider>();
        registrar.depends_on::<dyn IClipboardProvider>();
    }

    fn on_init(&mut self, registry: &dyn IRegistryAccessor) -> Box<dyn IInitResponse> {
        let schedule_provider = registry.get_interface::<dyn IScheduleProvider>().unwrap();
        let schedule_cell = schedule_provider.get();
        let mut schedule = schedule_cell.get();

        let render_data: AnyArc<EguiRenderData> = AnyArc::default();
        let context_provider: AnyArc<EguiContextProvider> = AnyArc::default();

        let window = registry
            .get_interface::<dyn IWindowProvider>()
            .unwrap()
            .get_window()
            .unwrap();
        let mouse = registry
            .get_interface::<dyn IMouseProvider>()
            .unwrap()
            .get_mouse()
            .unwrap();
        let keyboard = registry
            .get_interface::<dyn IKeyboardProvider>()
            .unwrap()
            .get_keyboard()
            .unwrap();
        let frame_timer = registry
            .get_interface::<dyn IFrameTimerProvider>()
            .unwrap()
            .get_frame_timer()
            .unwrap();
        let events = registry
            .get_interface::<dyn IEventsProvider>()
            .unwrap()
            .get_events()
            .unwrap();
        let clipboard = registry
            .get_interface::<dyn IClipboardProvider>()
            .unwrap()
            .get_clipboard()
            .unwrap();

        let pre_update_keyboard = keyboard.clone();
        let pre_update_frame_timer = frame_timer.clone();
        let pre_update_ctx = context_provider.clone();
        let pre_update_window = window.clone();
        let pre_update_events = events.clone();
        schedule.add_exclusive_at_start_system_to_stage(
            &CoreStage::PreUpdate,
            "egui::pre_update",
            move || {
                let context_provider = pre_update_ctx.deref();
                let window = pre_update_window.deref();
                let keyboard = pre_update_keyboard.deref();
                let frame_timer = pre_update_frame_timer.deref();
                let events = pre_update_events.deref();

                let input = crate::utils::get_egui_input(window, keyboard, frame_timer, events);
                context_provider.begin_frame(input);
            },
        );

        let post_update_mouse = mouse.clone();
        let post_update_rnd = render_data.clone();
        let post_update_ctx = context_provider.clone();
        let post_update_window = window.clone();
        schedule.add_exclusive_at_start_system_to_stage(
            &CoreStage::PostUpdate,
            "egui::post_update",
            move || {
                let render_data = post_update_rnd.deref();
                let context_provider = post_update_ctx.deref();

                let mouse = post_update_mouse.deref();
                let clipboard = clipboard.deref();

                let output = context_provider.end_frame();
                let egui_ctx = context_provider.get_context();
                let pixels_per_point = post_update_window.current_display_scale();
                let jobs: Vec<ClippedPrimitive> =
                    egui_ctx.tessellate(output.shapes, pixels_per_point);
                crate::utils::process_egui_output(output.platform_output, mouse, clipboard);

                render_data.put(RenderData {
                    primitives: jobs,
                    textures_delta: output.textures_delta,
                });
            },
        );

        let response = vec![
            (
                TypeId::of::<dyn IEguiContextProvider>(),
                AnyArc::map::<dyn IAny, _>(context_provider, |v| v),
            ),
            (
                TypeId::of::<dyn IEguiRenderData>(),
                AnyArc::map::<dyn IAny, _>(render_data, |v| v),
            ),
        ];
        Box::new(response)
    }
}

impl Default for PluginEgui {
    fn default() -> Self {
        Self::new()
    }
}

interfaces::any::declare_interfaces!(PluginEgui, [IPlugin]);
