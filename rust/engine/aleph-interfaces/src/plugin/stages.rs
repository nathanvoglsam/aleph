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

use crate::any::{AnyArc, IAny};
use crate::plugin::IPlugin;
use std::any::TypeId;
use std::sync::Arc;

///
/// Represents the update execution stage that occurs after `ICoreInitStage`
///
pub trait IMainInitStage: IAny {}

///
/// Represents the init execution stage that occurs first
///
pub trait ICoreInitStage: IAny {}

macro_rules! implement_init_stage_plugin {
    ($ty:ident, $stage_interface:ident, [ $( $runs_after: ident ),* ]) => {
        #[allow(bare_trait_objects)]
        impl $crate::plugin::IPlugin for $ty {
            fn get_description(&self) -> $crate::plugin::PluginDescription {
                $crate::plugin::PluginDescription {
                    name: stringify!($ty).to_string(),
                    description: "Execution ordering dummy plugin".to_string(),
                    major_version: 1,
                    minor_version: 0,
                    patch_version: 0
                }
            }

            fn register(&mut self, registrar: &mut dyn $crate::plugin::IPluginRegistrar) {
                $(
                    registrar.depends_on::<$runs_after>();
                    registrar.must_init_after::<$runs_after>();
                )*
                registrar.provides_interface::<$stage_interface>();
            }

            fn on_init(&mut self, _interfaces: &dyn $crate::plugin::IInterfaces) -> Box<dyn $crate::plugin::IInitResponse> {
                let id = TypeId::of::<dyn $stage_interface>();
                let object: Arc<dyn IAny + Send + Sync> = Arc::new(Self());
                let object = AnyArc::from_arc(object);

                let stages = vec![(id, object)];

                Box::new(stages)
            }

            fn on_update(&mut self, _interfaces: &dyn $crate::plugin::IInterfaces) {}

            fn on_exit(&mut self, _interfaces: &dyn $crate::plugin::IInterfaces) {}
        }

        impl $stage_interface for $ty {}

        any::declare_interfaces!($ty, [IPlugin, $stage_interface]);
    };
}

///
/// A dummy implementation of `IMainInitStage` that does nothing but can be inserted into the plugin
/// registry to allow for ordering other plugins
///
#[derive(Default)]
struct MainInitStage();

implement_init_stage_plugin!(MainInitStage, IMainInitStage, [ICoreInitStage]);

///
/// A dummy implementation of `ICoreInitStage` that does nothing but can be inserted into the plugin
/// registry to allow for ordering other plugins
///
#[derive(Default)]
struct CoreInitStage();

implement_init_stage_plugin!(CoreInitStage, ICoreInitStage, []);

///
/// Represents the update execution stage that occurs after `IPostUpdateStage`
///
pub trait IRenderStage: IAny {}

///
/// Represents the update execution stage that occurs after the `IUpdateStage`.
///
pub trait IPostUpdateStage: IAny {}

///
/// Represents the update execution stage that occurs after the `IPreUpdateStage`.
///
pub trait IUpdateStage: IAny {}

///
/// Represents the update execution stage that occurs after the `IInputCollectionStage`.
///
pub trait IPreUpdateStage: IAny {}

///
/// Represents the update execution stage that occurs at the beginning.
///
pub trait IInputCollectionStage: IAny {}

macro_rules! implement_update_stage_plugin {
    ($ty:ident, $stage_interface:ident, [ $( $runs_after: ident ),* ]) => {
        #[allow(bare_trait_objects)]
        impl $crate::plugin::IPlugin for $ty {
            fn get_description(&self) -> $crate::plugin::PluginDescription {
                $crate::plugin::PluginDescription {
                    name: stringify!($ty).to_string(),
                    description: "Execution ordering dummy plugin".to_string(),
                    major_version: 1,
                    minor_version: 0,
                    patch_version: 0
                }
            }

            fn register(&mut self, registrar: &mut dyn crate::plugin::IPluginRegistrar) {
                $(
                    registrar.depends_on::<$runs_after>();
                    registrar.must_update_after::<$runs_after>();
                )*
                registrar.provides_interface::<$stage_interface>();
            }

            fn on_init(&mut self, _interfaces: &dyn crate::plugin::IInterfaces) -> Box<dyn crate::plugin::IInitResponse> {
                let id = TypeId::of::<dyn $stage_interface>();
                let object: Arc<dyn IAny + Send + Sync> = Arc::new(Self());
                let object = AnyArc::from_arc(object);

                let stages = vec![(id, object)];

                Box::new(stages)
            }

            fn on_update(&mut self, _interfaces: &dyn crate::plugin::IInterfaces) {}

            fn on_exit(&mut self, _interfaces: &dyn crate::plugin::IInterfaces) {}
        }

        impl $stage_interface for $ty {}

        any::declare_interfaces!($ty, [IPlugin, $stage_interface]);
    };
}

///
/// A dummy implementation of `IRenderStage` that does nothing but can be inserted into the plugin
/// registry to allow for ordering other plugins
///
#[derive(Default)]
struct RenderStage();

implement_update_stage_plugin!(
    RenderStage,
    IRenderStage,
    [
        IInputCollectionStage,
        IPreUpdateStage,
        IUpdateStage,
        IPostUpdateStage
    ]
);

///
/// A dummy implementation of `IPostUpdateStage` that does nothing but can be inserted into the
/// plugin registry to allow for ordering other plugins
///
#[derive(Default)]
struct PostUpdateStage();

implement_update_stage_plugin!(
    PostUpdateStage,
    IPostUpdateStage,
    [IInputCollectionStage, IPreUpdateStage, IUpdateStage]
);

///
/// A dummy implementation of `IUpdateStage` that does nothing but can be inserted into the plugin
/// registry to allow for ordering other plugins
///
#[derive(Default)]
struct UpdateStage();

implement_update_stage_plugin!(
    UpdateStage,
    IUpdateStage,
    [IInputCollectionStage, IPreUpdateStage]
);

///
/// A dummy implementation of `IPreUpdateStage` that does nothing but can be inserted into the
/// plugin registry to allow for ordering other plugins
///
#[derive(Default)]
struct PreUpdateStage();

implement_update_stage_plugin!(PreUpdateStage, IPreUpdateStage, [IInputCollectionStage]);

///
/// A dummy implementation of `IInputCollectionStage` that does nothing but can be inserted into the
/// plugin registry to allow for ordering other plugins
///
#[derive(Default)]
struct InputCollectionStage();

implement_update_stage_plugin!(InputCollectionStage, IInputCollectionStage, []);

///
/// A function that provides an array prefilled with implementations of all the stages declared in
/// this module.
///
pub fn default_stages() -> Vec<Box<dyn IPlugin>> {
    vec![
        Box::new(InputCollectionStage::default()),
        Box::new(PreUpdateStage::default()),
        Box::new(UpdateStage::default()),
        Box::new(PostUpdateStage::default()),
        Box::new(RenderStage::default()),
        Box::new(CoreInitStage::default()),
        Box::new(MainInitStage::default()),
    ]
}
