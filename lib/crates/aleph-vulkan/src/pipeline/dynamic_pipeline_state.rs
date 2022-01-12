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

use aleph_vulkan_core::erupt::vk1_0::{DynamicState, PipelineDynamicStateCreateInfoBuilder};

///
/// Namespace struct for dynamic pipeline state
///
pub struct DynamicPipelineState {}

impl DynamicPipelineState {
    ///
    /// No dynamic pipeline state
    ///
    pub fn none() -> PipelineDynamicStateCreateInfoBuilder<'static> {
        PipelineDynamicStateCreateInfoBuilder::new()
    }

    ///
    /// Creates a state description with the given list of dynamic states
    ///
    pub fn states(
        states: &[aleph_vulkan_core::erupt::vk1_0::DynamicState],
    ) -> PipelineDynamicStateCreateInfoBuilder {
        PipelineDynamicStateCreateInfoBuilder::new().dynamic_states(states)
    }

    ///
    /// Creates a dynamic state description that specifies dynamic viewport and scissor state
    ///
    pub fn viewport_scissor() -> PipelineDynamicStateCreateInfoBuilder<'static> {
        static STATES: [DynamicState; 2] = [DynamicState::VIEWPORT, DynamicState::SCISSOR];
        PipelineDynamicStateCreateInfoBuilder::new().dynamic_states(&STATES)
    }
}
