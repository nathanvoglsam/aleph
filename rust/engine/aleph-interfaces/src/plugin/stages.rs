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

macro_rules! implement_stage_plugin {
    ($ty:ident, $stage_interface:ident, [ $( $runs_after: ident ),* ]) => {
        #[allow(bare_trait_objects)]
        impl $crate::plugin::IPlugin for $ty {
            fn get_description(&self) -> $crate::plugin::PluginDescription {
                $crate::plugin::PluginDescription {
                    name: stringify!($ty).to_string(),
                    description: "A dummy plugin implementation used for ordering other plugins".to_string(),
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
pub struct RenderStage();

implement_stage_plugin!(
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
pub struct PostUpdateStage();

implement_stage_plugin!(
    PostUpdateStage,
    IPostUpdateStage,
    [IInputCollectionStage, IPreUpdateStage, IUpdateStage]
);

///
/// A dummy implementation of `IUpdateStage` that does nothing but can be inserted into the plugin
/// registry to allow for ordering other plugins
///
pub struct UpdateStage();

implement_stage_plugin!(
    UpdateStage,
    IUpdateStage,
    [IInputCollectionStage, IPreUpdateStage]
);

///
/// A dummy implementation of `IPreUpdateStage` that does nothing but can be inserted into the
/// plugin registry to allow for ordering other plugins
///
pub struct PreUpdateStage();

implement_stage_plugin!(PreUpdateStage, IPreUpdateStage, [IInputCollectionStage]);

///
/// A dummy implementation of `IInputCollectionStage` that does nothing but can be inserted into the
/// plugin registry to allow for ordering other plugins
///
pub struct InputCollectionStage();

implement_stage_plugin!(InputCollectionStage, IInputCollectionStage, []);
