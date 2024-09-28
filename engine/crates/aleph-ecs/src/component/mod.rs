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

pub mod component_query;
pub mod component_registry;
pub mod component_source;

use std::collections::HashMap;

use aleph_object_system::uuid::Uuid;
use aleph_object_system::IObject;

///
/// This trait needs to be implemented by any type that wishes to be used as a component
///
pub trait Component: IObject + Send + Sync + 'static {}

impl<T: IObject + Send + Sync + 'static> Component for T {}

/// A type alias for a configuration of `std::hash::HashMap` that efficiently uses `ComponentTypeId`
/// as a key. This alias is special as it skips hashing the `ComponentTypeId` and uses that id
/// directly as the key.
pub type ComponentIdMap<T> = HashMap<Uuid, T>;
