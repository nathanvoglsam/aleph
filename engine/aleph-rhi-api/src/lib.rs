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

mod adapter;
mod buffer;
mod command_encoder;
mod command_list;
mod common_data;
mod common_interfaces;
mod context;
mod descriptor_arena;
mod descriptor_pool;
mod descriptor_set;
mod descriptor_set_layout;
mod device;
mod fence;
mod format;
mod pipeline_layout;
mod pipeline_state;
mod queue;
mod sampler;
mod semaphore;
mod surface;
mod swap_chain;
mod sync_data;
mod texture;
mod texture_view;

pub use adapter::*;
pub use buffer::*;
pub use command_encoder::*;
pub use command_list::*;
pub use common_data::*;
pub use common_interfaces::*;
pub use context::*;
pub use descriptor_arena::*;
pub use descriptor_pool::*;
pub use descriptor_set::*;
pub use descriptor_set_layout::*;
pub use device::*;
pub use fence::*;
pub use format::*;
pub use pipeline_layout::*;
pub use pipeline_state::*;
pub use queue::*;
pub use sampler::*;
pub use semaphore::*;
pub use surface::*;
pub use swap_chain::*;
pub use sync_data::*;
pub use texture::*;
pub use texture_view::*;

pub const API_VERSION_MAJOR: &str = env!("CARGO_PKG_VERSION_MAJOR");
pub const API_VERSION_MINOR: &str = env!("CARGO_PKG_VERSION_MINOR");
pub const API_VERSION_PATCH: &str = env!("CARGO_PKG_VERSION_PATCH");

//
// =================================================================================================
// PUBLIC MACROS
// =================================================================================================
//

/// Utility macro that can be (optionally) used to construct a name for an RHI object. This macro
/// will yield a bare string literal and is intended to be used with the `with_name` builder
/// functions.
///
/// For use with raw object descriptions [`obj_name_opt`] is available.
///
/// This macro will concatenate the given name with the callsite's [`module_path`]. This is simply
/// nice convention and we provide this macro to encourage using this convention where possible.
#[macro_export]
macro_rules! obj_name {
    ($v: expr) => {
        concat!(module_path!(), "::", $v)
    };
}

/// Utility macro that can be (optionally) used to construct a name for an RHI object.
///
/// This is an extension over [`obj_name`] that yields `Some(obj_name!(name))` for use when creating
/// a bare object description instead of via the `with_name` builder utilities.
///
/// This macro will concatenate the given name with the callsite's [`module_path`]. This is simply
/// nice convention and we provide this macro to encourage using this convention where possible.
#[macro_export]
macro_rules! obj_name_opt {
    ($v: expr) => {
        ::core::option::Option::Some($crate::obj_name!($v))
    };
}

//
// =================================================================================================
// UTILITY MACROS
// =================================================================================================
//

#[doc(hidden)]
#[macro_export]
macro_rules! any_arc_trait_utils_decl {
    ($x: path) => {
        /// Returns an `AnyArc` that points to `self`. This is similar to upgrading a weak
        /// reference. We take a non-owning reference `&dyn SomeTrait` and upgrade it to an owning
        /// `AnyArc<dyn SomeTrait>` handle.
        fn upgrade(&self) -> aleph_any::AnyArc<dyn $x>;

        /// Returns the number of strong references to the object.
        ///
        /// A strong reference is an owning handle to the object (`AnyArc`). The object will remain
        /// alive as long as this remains > 0. The object will be dropped when this reaches 0.
        ///
        /// It is only possible to observe a 0 value for `strong_count` through an `AnyWeak`.
        fn strong_count(&self) -> usize;

        /// Returns the number of weak references to the object.
        ///
        /// A weak reference is a non-owning handle to the object (`AnyWeak`). Weak references do
        /// not extend the lifetime of the object itself, only the ref-count block and the memory
        /// allocation that backs it.
        ///
        /// If `strong_count` is 0 and `weak_count` is >0 then the object is no longer accessible as
        /// it will have been dropped.
        ///
        /// It is only possible to observe a 0 value for `weak_count` through an `AnyArc`.
        fn weak_count(&self) -> usize;
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! error_enum_from_unit_type {
    ($x: ident) => {
        impl From<()> for $x {
            #[inline(always)]
            fn from(_value: ()) -> Self {
                $x::Platform
            }
        }
    };
}
