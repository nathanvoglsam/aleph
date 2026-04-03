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

use std::any::Any;
use std::num::{NonZero, NonZeroU64};

use crate::*;

/// A binding signature represents the complete binding interface of a shader pipeline. This
/// includes the layout and position of all parameter blocks, as well as the size of any push
/// constant block.
///
/// A binding signature can be shared between many pipelines.
pub trait IBindingSignature: Any + Send + Sync {
    arc_trait_utils_decl!(IBindingSignature);

    /// Returns a globally unique ID that is guaranteed to not be shared by any other object
    /// allocated from the same [`IDevice`] instance.
    fn get_id(&self) -> NonZeroU64;
}

/// Description of a [`IBindingSignature`] object.
#[derive(Clone, Default)]
pub struct BindingSignatureDesc<'a> {
    /// Specifies the layouts of all parameter blocks that will be combined into this pipeline
    /// layout. The order of this array is meaningful: the `n`th element will define the layout for
    /// the `n`th block.
    pub parameter_block_layouts: &'a [&'a dyn IParameterBlockLayout],

    /// Specifies the set of push constant ranges that the pipeline layout will hold.
    pub push_constant_block: Option<PushConstantBlock>,

    /// The name of the object
    pub name: Option<&'a str>,
}

impl<'a> BindingSignatureDesc<'a> {
    /// Returns a new, empty [`BindingSignatureDesc`].
    pub const fn new() -> Self {
        Self {
            parameter_block_layouts: &[],
            push_constant_block: None,
            name: None,
        }
    }

    /// Takes the given desc and returns it with [BindingSignatureDesc::parameter_block_layouts] set
    /// to the given parameter
    pub const fn with_parameter_block_layouts(
        mut self,
        parameter_block_layouts: &'a [&'a dyn IParameterBlockLayout],
    ) -> Self {
        self.parameter_block_layouts = parameter_block_layouts;
        self
    }

    /// Takes the given desc and returns it with [BindingSignatureDesc::push_constant_block] set to
    /// the given parameter
    pub const fn with_push_constant_blocks(
        mut self,
        push_constant_block: PushConstantBlock,
    ) -> Self {
        self.push_constant_block = Some(push_constant_block);
        self
    }

    /// Takes the given desc and returns it with [BindingSignatureDesc::name] set to the given
    /// parameter
    pub const fn with_name(mut self, name: &'a str) -> Self {
        self.name = Some(name);
        self
    }
}

/// Description of a push constant block
#[derive(Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Debug)]
pub struct PushConstantBlock {
    /// Specifies the size, in bytes, of the push constant range.
    pub size: NonZero<u8>,

    /// Specifies what shader stages the push constant block made visible to.
    ///
    /// All shader stages in a pipeline must share the same push constant block under our RHI
    /// design. However some pipelines may combine shaders where only a subset of the shaders access
    /// the shared constant block.
    ///
    /// It may be benefitial to restrict this field to specify only the shader using the push
    /// constant block. Some hardware/drivers may be able to take advantage of this to more
    /// efficiently hoist parameters for individual stages.
    pub visibility: DescriptorShaderVisibility,
}

impl PushConstantBlock {
    /// Constructs a new `Option<PushConstantBlock>` from the given size. 0 is coerced to `None`,
    /// which can be used in a [`BindingSignatureDesc`] to encode that no push constant block is
    /// used.
    pub const fn new(size: u8) -> Option<PushConstantBlock> {
        Self::new_with_visibility(size, DescriptorShaderVisibility::All)
    }

    /// Constructs a new `Option<PushConstantBlock>` from the given size. 0 is coerced to `None`,
    /// which can be used in a [`BindingSignatureDesc`] to encode that no push constant block is
    /// used. Also sets 'visibility' to the given value.
    pub const fn new_with_visibility(
        size: u8,
        visibility: DescriptorShaderVisibility,
    ) -> Option<PushConstantBlock> {
        match NonZero::new(size) {
            Some(size) => Some(Self { size, visibility }),
            None => None,
        }
    }

    /// Takes self and returns it with [`PushConstantBlock::visibility`] set to the given
    /// parameter.
    pub const fn with_visibility(mut self, v: DescriptorShaderVisibility) -> Self {
        self.visibility = v;
        self
    }

    /// Returns the union of the given pust constant block descriptions. This is comparable to a
    /// 'max' operation where the resulting block will be large enough to cover the needs of both
    /// the input blocks.
    pub const fn merge(
        a: Option<PushConstantBlock>,
        b: Option<PushConstantBlock>,
    ) -> Option<PushConstantBlock> {
        match (a, b) {
            // If we do have two blocks then we have to perform merge/max operations to pick a size
            // and visibility that covers both input block descriptions.
            (Some(a), Some(b)) => {
                let size = if a.size.get() > b.size.get() {
                    a.size
                } else {
                    b.size
                };
                let visibility = DescriptorShaderVisibility::merge(a.visibility, b.visibility);
                Some(PushConstantBlock { size, visibility })
            }
            // Only a single block from a or b? Just return a or b!
            (None, Some(block)) | (Some(block), None) => Some(block),
            // No block in either? Just return None!
            (None, None) => None,
        }
    }
}
