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

use aleph_object_system::ArcObject;

use crate::*;

#[derive(Clone)]
pub struct PipelineLayoutHandle {
    inner: ArcObject,
}

impl PipelineLayoutHandle {
    /// # Safety
    ///
    /// It is the caller's responsibility to ensure that the given object refers to an object that
    /// the inner RHI implementation considers a semaphore objec.
    pub const unsafe fn new(inner: ArcObject) -> Self {
        Self { inner }
    }

    ///
    /// Gets the number of strong ([`PipelineLayoutHandle`]) pointers to this allocation.
    ///
    /// # Safety
    ///
    /// This method by itself is safe, but using it correctly requires extra care.
    /// Another thread can change the strong count at any time,
    /// including potentially between calling this method and acting on the result.
    ///
    /// # Info
    ///
    /// This is just a wrapper around [`std::sync::Arc::strong_count`]
    ///
    #[inline]
    #[must_use]
    pub fn strong_count(&self) -> usize {
        self.inner.strong_count()
    }

    /// Unwrap the [`PipelineLayoutHandle`] and get the inner [`ArcObject`]
    #[inline]
    pub fn into_inner(self) -> ArcObject {
        self.inner
    }

    /// Get the inner [`ArcObject`]
    pub const fn get(&self) -> &ArcObject {
        &self.inner
    }
}

#[derive(Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Debug, Default)]
pub struct PushConstantBlock {
    /// Specifies the binding index that the push constant range will be attached to in the shader.
    ///
    /// # Warning
    ///
    /// This is ignored on Vulkan. Vulkan has a dedicated 'push constant' location specifier. D3D12
    /// maps its 'root constants' (D3D12's analogue of push constants) to a register index.
    ///
    /// There is no robust way to automatically choose a register index, so we leave the choice as
    /// an exercise for the user.
    pub binding: u32,

    /// Specifies which shader stages the push constant range will be
    pub visibility: DescriptorShaderVisibility,

    /// Specifies the size, in bytes, of the push constant range.
    pub size: u16,
}

#[derive(Clone, Default)]
pub struct PipelineLayoutDesc<'a> {
    /// Specifies the layouts of all descriptor sets that will be combined into this pipeline
    /// layout. The order of this array is meaningful: the `n`th element will define the layout for
    /// the `n`th descriptor set.
    pub set_layouts: &'a [&'a DescriptorSetLayoutHandle],

    /// Specifies the set of push constant ranges that the pipeline layout will hold.
    pub push_constant_blocks: &'a [PushConstantBlock],

    /// The name of the object
    pub name: Option<&'a str>,
}

impl<'a> PipelineLayoutDesc<'a> {
    /// Returns a new, empty [PipelineLayoutDesc].
    pub const fn new() -> Self {
        Self {
            set_layouts: &[],
            push_constant_blocks: &[],
            name: None,
        }
    }

    /// Takes the given desc and returns it with [PipelineLayoutDesc::set_layouts] set to the given
    /// parameter
    pub const fn with_set_layouts(
        mut self,
        set_layouts: &'a [&'a DescriptorSetLayoutHandle],
    ) -> Self {
        self.set_layouts = set_layouts;
        self
    }

    /// Takes the given desc and returns it with [PipelineLayoutDesc::push_constant_blocks] set to
    /// the given parameter
    pub const fn with_push_constant_blocks(
        mut self,
        push_constant_blocks: &'a [PushConstantBlock],
    ) -> Self {
        self.push_constant_blocks = push_constant_blocks;
        self
    }

    /// Takes the given desc and returns it with [PipelineLayoutDesc::name] set to the given
    /// parameter
    pub const fn with_name(mut self, name: &'a str) -> Self {
        self.name = Some(name);
        self
    }
}
