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

extern crate aleph_interfaces as interfaces;
extern crate aleph_log as log;

mod adapter;
mod buffer;
mod command_list;
mod command_pool;
mod context;
mod descriptor_pool;
mod descriptor_set_layout;
mod device;
mod encoder;
mod internal;
mod pipeline;
mod pipeline_layout;
mod queue;
mod sampler;
mod shader;
mod surface;
mod swap_chain;
mod texture;

pub use buffer::Buffer;
pub use command_list::CommandList;
pub use context::Context;
pub use device::Device;
pub use pipeline::ComputePipeline;
pub use pipeline::GraphicsPipeline;
pub use pipeline_layout::PipelineLayout;
pub use shader::Shader;
pub use swap_chain::SwapChain;
pub use texture::Texture;

/// This is a manually unrolled form of [interfaces::any::declare_interfaces] that just punts to
/// the 'inner' field. We need to keep the validation object invisible. Users of the API may query
/// interfaces for the underlying device that we can't implement here.
///
/// Instead we give up the ability to query any extension interfaces on the validation device and
/// pass directly through to the inner object's `__query_interface`. This keeps the validation
/// object almost invisible. (Asking for the type id will give a different value, but it doesn't
/// matter because users shouldn't be able to reach the concrete implementation type to do anything
/// with).
#[macro_export]
macro_rules! validation_declare_interfaces (
    ( $typ: ident, [ $( $iface: ident ),* ]) => {
        impl $crate::interfaces::any::IAny for $typ {
            fn __query_interface(&self, target: ::core::any::TypeId) -> Option<$crate::interfaces::any::TraitObject> {
                unsafe {
                    $(
                    if target == ::core::any::TypeId::of::<dyn $iface>() {
                        return Some(::core::mem::transmute(self as &dyn $iface));
                    }
                    )*
                }

                unsafe {
                    if target == ::core::any::TypeId::of::<$typ>() {
                        return Some($crate::interfaces::any::TraitObject {
                            data: ::core::ptr::NonNull::new_unchecked(self as *const _ as *mut ()),
                            vtable: ::core::ptr::null_mut(),
                            phantom: ::std::default::Default::default(),
                        })
                    }
                }

                self.inner.__query_interface(target)
            }
        }
    }
);
