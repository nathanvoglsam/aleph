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

use crate::traits::{EguiContextProvider, EguiRenderData};
use crate::{IEguiContextProvider, IEguiRenderData};
use egui::PaintJobs;
use interfaces::any::AnyArc;
use interfaces::platform::{
    IClipboard, IClipboardProvider, IEvents, IEventsProvider, IFrameTimer, IFrameTimerProvider,
    IKeyboard, IKeyboardProvider, IMouse, IMouseProvider, IWindow, IWindowProvider,
};
use interfaces::plugin::{
    IInitResponse, IPlugin, IPluginRegistrar, IRegistryAccessor, PluginDescription, UpdateStage,
};
use std::any::TypeId;
use std::ops::Deref;

pub struct EguiPlugin {
    window: Option<AnyArc<dyn IWindow>>,
    mouse: Option<AnyArc<dyn IMouse>>,
    keyboard: Option<AnyArc<dyn IKeyboard>>,
    frame_timer: Option<AnyArc<dyn IFrameTimer>>,
    events: Option<AnyArc<dyn IEvents>>,
    clipboard: Option<AnyArc<dyn IClipboard>>,
    render_data: AnyArc<EguiRenderData>,
    context_provider: AnyArc<EguiContextProvider>,
}

impl EguiPlugin {
    pub fn new() -> Self {
        let render_data: AnyArc<EguiRenderData> = AnyArc::default();
        let context_provider: AnyArc<EguiContextProvider> = AnyArc::default();
        Self {
            window: None,
            mouse: None,
            keyboard: None,
            frame_timer: None,
            events: None,
            clipboard: None,
            render_data,
            context_provider,
        }
    }
}

impl IPlugin for EguiPlugin {
    fn get_description(&self) -> PluginDescription {
        PluginDescription {
            name: "Egui Plugin".to_string(),
            description: "Plugin that integrates egui".to_string(),
            major_version: 0,
            minor_version: 1,
            patch_version: 0,
        }
    }

    fn register(&mut self, registrar: &mut dyn IPluginRegistrar) {
        // We want to update in the pre update stage and post update stage
        registrar.update_stage(UpdateStage::PreUpdate);
        registrar.update_stage(UpdateStage::PostUpdate);

        // We export two interfaces for interacting with egui
        registrar.provides_interface::<dyn IEguiContextProvider>();
        registrar.provides_interface::<dyn IEguiRenderData>();

        // We need to get handles to all these when we initialize to save querying them every frame
        registrar.must_init_after::<dyn IWindowProvider>();
        registrar.must_init_after::<dyn IMouseProvider>();
        registrar.must_init_after::<dyn IKeyboardProvider>();
        registrar.must_init_after::<dyn IFrameTimerProvider>();
        registrar.must_init_after::<dyn IEventsProvider>();
        registrar.must_init_after::<dyn IClipboardProvider>();

        registrar.depends_on::<dyn IWindowProvider>();
        registrar.depends_on::<dyn IMouseProvider>();
        registrar.depends_on::<dyn IKeyboardProvider>();
        registrar.depends_on::<dyn IFrameTimerProvider>();
        registrar.depends_on::<dyn IEventsProvider>();
        registrar.depends_on::<dyn IClipboardProvider>();
    }

    fn on_init(&mut self, registry: &dyn IRegistryAccessor) -> Box<dyn IInitResponse> {
        self.window = registry
            .get_interface::<dyn IWindowProvider>()
            .unwrap()
            .get_window();
        self.mouse = registry
            .get_interface::<dyn IMouseProvider>()
            .unwrap()
            .get_mouse();
        self.keyboard = registry
            .get_interface::<dyn IKeyboardProvider>()
            .unwrap()
            .get_keyboard();
        self.frame_timer = registry
            .get_interface::<dyn IFrameTimerProvider>()
            .unwrap()
            .get_frame_timer();
        self.events = registry
            .get_interface::<dyn IEventsProvider>()
            .unwrap()
            .get_events();
        self.clipboard = registry
            .get_interface::<dyn IClipboardProvider>()
            .unwrap()
            .get_clipboard();

        assert!(self.window.is_some());
        assert!(self.mouse.is_some());
        assert!(self.keyboard.is_some());
        assert!(self.frame_timer.is_some());
        assert!(self.events.is_some());
        assert!(self.clipboard.is_some());

        let response = vec![
            (
                TypeId::of::<dyn IEguiContextProvider>(),
                AnyArc::into_any(self.context_provider.clone()),
            ),
            (
                TypeId::of::<dyn IEguiRenderData>(),
                AnyArc::into_any(self.render_data.clone()),
            ),
        ];
        Box::new(response)
    }

    fn on_pre_update(&mut self, _registry: &dyn IRegistryAccessor) {
        let window = self.window.as_ref().unwrap().deref();
        let mouse = self.mouse.as_ref().unwrap().deref();
        let keyboard = self.keyboard.as_ref().unwrap().deref();
        let frame_timer = self.frame_timer.as_ref().unwrap().deref();
        let events = self.events.as_ref().unwrap().deref();

        let input = crate::utils::get_egui_input(window, mouse, keyboard, frame_timer, events);
        self.context_provider.begin_frame(input);
    }

    fn on_post_update(&mut self, _registry: &dyn IRegistryAccessor) {
        let mouse = self.mouse.as_ref().unwrap().deref();
        let clipboard = self.clipboard.as_ref().unwrap().deref();

        let (output, shapes) = self.context_provider.end_frame();
        let egui_ctx = self.context_provider.get_context();
        let jobs: PaintJobs = egui_ctx.tessellate(shapes);
        crate::utils::process_egui_output(output, mouse, clipboard);

        self.render_data.put(jobs)
    }
}

interfaces::any::declare_interfaces!(EguiPlugin, [IPlugin]);
