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

pub use crate::archetype::archetype_get_capacity;
pub use crate::archetype::archetype_get_component_descriptions;
pub use crate::archetype::archetype_get_component_index;
pub use crate::archetype::archetype_get_entity_layout;
pub use crate::archetype::archetype_get_len;
pub use crate::archetype::archetype_get_storage_by_index;
pub use crate::archetype_filter::archetype_filter_current;
pub use crate::archetype_filter::archetype_filter_destroy;
pub use crate::archetype_filter::archetype_filter_new;
pub use crate::archetype_filter::archetype_filter_next;
pub use crate::world::world_add_component;
pub use crate::world::world_get_component_ptr;
pub use crate::world::world_has_component;
pub use crate::world::world_register;
pub use crate::world::world_remove_component;
